use crate::http_request::{HttpMethod, HttpRequest};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub trait Connection {
    fn listen<T: Fn(&str) -> Result<String, std::io::Error>>(&self, callback: T);
}

pub struct Server<T>
where
    T: Connection,
{
    http_status_codes: HashMap<u32, String>,
    connection: T,
}

fn load_html_file_from_uri(uri: &str) -> Result<String, std::io::Error> {
    let path = Path::new(uri);
    fs::read_to_string(path)
}

impl<T: Connection> Server<T> {
    pub fn new(connection: T) -> Server<T> {
        let mut http_status_codes: HashMap<u32, String> = HashMap::new();
        http_status_codes.insert(200, "OK".to_string());
        http_status_codes.insert(404, "Not Found".to_string());
        http_status_codes.insert(501, "Not Implemented".to_string());

        Server {
            http_status_codes,
            connection,
        }
    }

    pub fn run(&self) {
        self.connection
            .listen(|message| self.request_handler(message));
    }

    fn add_response(&self, status_code: &u32) -> Result<String, std::io::Error> {
        if !self.http_status_codes.contains_key(status_code) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown status code",
            ));
        }

        Ok(format!(
            "HTTP/1.1 {} {}\r\n",
            status_code, &self.http_status_codes[&status_code],
        ))
    }

    fn handle_get_request(&self, request: &HttpRequest) -> Result<String, std::io::Error> {
        let contents = load_html_file_from_uri(&request.line.uri[1..])?;
        Ok(format!("{}\r\n{}", self.add_response(&200)?, contents))
    }

    fn handle_not_implemented(&self) -> String {
        format!("{}\r\n", self.add_response(&501).unwrap())
    }

    pub fn request_handler(&self, request: &str) -> Result<String, std::io::Error> {
        match HttpRequest::from_str(request) {
            Ok(http_request) => match http_request.line.method {
                HttpMethod::Get => self.handle_get_request(&http_request),
            },
            Err(e) => {
                Ok(self.handle_not_implemented())
            }
        }
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
        fn listen<T: Fn(&str) -> Result<String, std::io::Error>>(&self, callback: T) {
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
