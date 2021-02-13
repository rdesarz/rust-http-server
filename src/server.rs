use crate::http_request::{HttpMethod, HttpRequest};
use http::StatusCode;
use mime::Mime;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type Message = Vec<u8>;

pub trait Connection {
    fn listen<T: 'static + Copy + Fn(&[u8]) -> Result<Message, std::io::Error> + Send + Sync>(
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

fn load_content_from_uri(uri: &str) -> Result<Message, std::io::Error> {
    let path = Path::new(uri);
    fs::read(path)
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

    res
}

fn build_content_type(mime: &Mime) -> String {
    format!("Content-Type: {}/{}\r\n", mime.type_(), mime.subtype())
}

impl<T: Connection> Server<T> {
    pub fn new(connection: T) -> Server<T> {
        Server { connection }
    }

    pub fn run(&self) {
        self.connection
            .listen(|message| Self::request_handler(message));
    }

    pub fn request_handler(request: &[u8]) -> Result<Message, std::io::Error> {
        match HttpRequest::from_str(std::str::from_utf8(request).unwrap()) {
            Ok(http_request) => match http_request.line.method {
                HttpMethod::Get => Self::handle_get_request(&http_request),
            },
            Err(_e) => Ok(Self::build_not_implemented_response()),
        }
    }

    fn handle_get_request(request: &HttpRequest) -> Result<Message, std::io::Error> {
        let mime = find_mimetype(&request.line.uri[1..]);

        match load_content_from_uri(&request.line.uri[1..]) {
            Ok(content) => {
                let response = Self::build_http_response(200).unwrap();
                let content_type = build_content_type(&mime);
                let blank_line = "\r\n";
                let mut message = Vec::new();
                message.extend_from_slice(response.as_bytes());
                message.extend_from_slice(content_type.as_bytes());
                message.extend_from_slice(blank_line.as_bytes());
                message.extend_from_slice(&content);
                Ok(message)
            }
            Err(_e) => Ok(Self::build_not_found_response()),
        }
    }

    fn build_not_implemented_response() -> Message {
        format!("{}\r\n", Self::build_http_response(501).unwrap()).into_bytes()
    }

    fn build_not_found_response() -> Message {
        match load_content_from_uri("404.html") {
            Ok(content) => {
                let response = Self::build_http_response(404).unwrap();
                let blank_line = "\r\n";
                let mut message = Vec::new();
                message.extend_from_slice(response.as_bytes());
                message.extend_from_slice(blank_line.as_bytes());
                message.extend_from_slice(&content);
                message
            }
            Err(_e) => format!(
                "{}\r\n404 - Page not found",
                Self::build_http_response(404).unwrap()
            )
            .into_bytes(),
        }
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
        pull_message: Vec<Vec<u8>>,
        push_message: RefCell<Vec<Vec<u8>>>,
    }

    impl TestConnection {
        fn new() -> TestConnection {
            TestConnection {
                pull_message: vec![
                    String::from("1").as_bytes().to_vec(),
                    String::from("2").as_bytes().to_vec(),
                    String::from("3").as_bytes().to_vec(),
                ],
                push_message: RefCell::new(vec![]),
            }
        }
    }

    impl Connection for TestConnection {
        fn listen<
            T: 'static + Copy + Fn(&[u8]) -> Result<Message, std::io::Error> + Send + Sync,
        >(
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
        let test_connection = TestConnection::new();
        test_connection.listen(|_| Ok(String::from("Test").as_bytes().to_vec()));
        assert_eq!(
            String::from("Test"),
            test_connection.push_message.borrow()[0]
        );
    }

    #[test]
    fn test_load_non_existing_png_file() {
        let uri = "non_existing.png";

        let result = load_content_from_uri(&uri);

        assert!(result.is_err());
    }
}
