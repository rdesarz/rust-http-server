use regex::Regex;
use std::str::FromStr;

pub enum HttpMethod {
    Get,
}

impl FromStr for HttpMethod {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown http method",
            )),
        }
    }
}

pub enum HttpVersion {
    V11,
}

impl FromStr for HttpVersion {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(HttpVersion::V11),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown http version",
            )),
        }
    }
}

pub struct HttpRequestLine {
    pub method: HttpMethod,
    pub uri: String,
    pub version: HttpVersion,
}

impl FromStr for HttpRequestLine {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"([A-Z]*) (.*) (HTTP/[1-9.]*)"#).unwrap();
        let caps = re.captures(s).unwrap();

        let method = match caps.get(1) {
            Some(match_method) => match HttpMethod::from_str(match_method.as_str()) {
                Ok(method) => method,
                Err(e) => return Err(e),
            },
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Http method not found in request line",
                ))
            }
        };

        let uri = match caps.get(2) {
            Some(uri) => uri.as_str(),
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Http uri not found in request line",
                ))
            }
        };

        let version = match caps.get(3) {
            Some(match_method) => match HttpVersion::from_str(match_method.as_str()) {
                Ok(method) => method,
                Err(e) => return Err(e),
            },
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Http version not found in request line",
                ))
            }
        };

        Ok(HttpRequestLine {
            method,
            uri: String::from(uri),
            version,
        })
    }
}

// TODO: implement parsing of header and body
pub struct HttpRequestHeader {}
pub struct HttpRequestBody {}

pub struct HttpRequest {
    pub line: HttpRequestLine,
    pub header: Option<HttpRequestHeader>,
    pub body: Option<HttpRequestBody>,
}

impl FromStr for HttpRequest {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = match HttpRequestLine::from_str(s) {
            Ok(method) => method,
            Err(e) => return Err(e),
        };

        Ok(HttpRequest {
            line,
            header: None,
            body: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_method() {
        let method = "GET";

        let result = HttpMethod::from_str(method);

        assert!(matches!(result, Ok(HttpMethod::Get)));
    }

    #[test]
    fn parse_unknown_method() {
        let method = "UNKNOWN";

        let result = HttpMethod::from_str(method);

        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn parse_11_version() {
        let version = "HTTP/1.1";

        let result = HttpVersion::from_str(version);

        assert!(matches!(result, Ok(HttpVersion::V11)));
    }

    #[test]
    fn parse_unknown_version() {
        let version = "UNKNOWN";

        let result = HttpVersion::from_str(version);

        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn parse_request_line() {
        let request_line = "GET /index.html HTTP/1.1 \r\n";

        let result = HttpRequestLine::from_str(request_line).expect("");

        assert!(matches!(result.method, HttpMethod::Get));
        assert_eq!(result.uri, String::from("/index.html"));
        assert!(matches!(result.version, HttpVersion::V11));
    }

    #[test]
    fn parse_wrong_request_line() {
        let request_line = "GET /index.html HTTP/.1 \r\n";
        assert!(matches!(HttpRequestLine::from_str(request_line), Err(_)));
    }
}
