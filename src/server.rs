use crate::http_request::{HttpMethod, HttpRequest};
use http::StatusCode;
use mime::Mime;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub trait Connection {
    fn listen<T: 'static + Copy + Fn(&str) -> Result<String, std::io::Error> + Send + Sync>(
        &self,
        callback: T,
    );
}

pub struct Server<T>
where
    T: Connection,
{
    connection: T,
}

fn load_html_file_from_uri(uri: &str) -> Result<String, std::io::Error> {
    let path = Path::new(uri);
    fs::read_to_string(path)
}

impl<T: Connection> Server<T> {
    pub fn new(connection: T) -> Server<T> {
        Server { connection }
    }

    pub fn run(&self) {
        self.connection
            .listen(|message| Self::request_handler(message));
    }

    pub fn request_handler(request: &str) -> Result<String, std::io::Error> {
        match HttpRequest::from_str(request) {
            Ok(http_request) => match http_request.line.method {
                HttpMethod::Get => Self::handle_get_request(&http_request),
            },
            Err(e) => Ok(Self::build_not_implemented_response()),
        }
    }

    fn find_mimetype(filename: &str) -> Mime {
        let parts: Vec<&str> = filename.split('.').collect();

        let res = match parts.last() {
            Some(v) => match *v {
                "html" => mime::TEXT_HTML,
                "png" => mime::IMAGE_PNG,
                "jpg" => mime::IMAGE_JPEG,
                "json" => mime::APPLICATION_JSON,
                &_ => mime::TEXT_PLAIN,
            },
            None => mime::TEXT_PLAIN,
        };
        return res;
    }

    fn handle_get_request(request: &HttpRequest) -> Result<String, std::io::Error> {
        let mime_type = Self::find_mimetype(&request.line.uri[1..]);
        let content_type = format!(
            "Content-Type: {}/{}\r\n",
            mime_type.type_(),
            mime_type.subtype()
        );
        match load_html_file_from_uri(&request.line.uri[1..]) {
            Ok(contents) => Ok(format!(
                "{}{}\r\n{}",
                Self::build_http_response(200).unwrap(),
                content_type,
                contents
            )),
            Err(e) => Ok(Self::build_not_found_response()),
        }
    }

    fn build_not_implemented_response() -> String {
        format!("{}\r\n", Self::build_http_response(501).unwrap())
    }

    fn build_not_found_response() -> String {
        format!(
            "{}\r\n404 - Page not found",
            Self::build_http_response(404).unwrap()
        )
    }

    fn build_http_response(status_code: u16) -> Result<String, std::io::Error> {
        StatusCode::from_u16(status_code).map_or(
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown status code",
            )),
            |code| Ok(format!("HTTP/1.1 {} {}\r\n", status_code, code.as_str())),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct TestConnection {
        pull_message: Vec<String>,
        push_message: RefCell<Vec<String>>,
    }

    impl TestConnection {
        fn new() -> TestConnection {
            TestConnection {
                pull_message: vec![String::from("1"), String::from("2"), String::from("3")],
                push_message: RefCell::new(vec![]),
            }
        }
    }

    impl Connection for TestConnection {
        fn listen<T: 'static + Copy + Fn(&str) -> Result<String, std::io::Error> + Send + Sync>(
            &self,
            callback: T,
        ) {
            self.push_message
                .borrow_mut()
                .push((callback)(&self.pull_message[0]).unwrap());
        }
    }

    #[test]
    fn pull_message() {
        let mut test_connection = TestConnection::new();
        test_connection.listen(|string| Ok(String::from("Test")));
        assert_eq!(
            String::from("Test"),
            test_connection.push_message.borrow()[0]
        );
    }
}
