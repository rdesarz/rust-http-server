use regex::Regex;
use std::fmt;
use std::str::FromStr;

/// Error returned when HTTP request parsing fails
#[derive(Debug, Clone)]
pub struct HttpRequestError {
    msg: String,
}

impl HttpRequestError {
    /// Creates a new [`HttpRequestError`]. An error message should be provided when building the error.
    fn new(msg: &str) -> HttpRequestError {
        HttpRequestError {
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// HTTP method (GET, POST, ETC)
pub enum HttpMethod {
    Get,
}

impl FromStr for HttpMethod {
    type Err = HttpRequestError;

    /// Creates an [`HttpMethod`] from a string containing the corresponding method.
    /// Returns [`HttpMethod`] if string corresponds to an implemented method, else returns HttpRequestError.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            _ => Err(HttpRequestError::new("Unknown http method")),
        }
    }
}

/// HTTP protocol version
pub enum HttpVersion {
    V11,
}

impl FromStr for HttpVersion {
    type Err = HttpRequestError;

    /// Creates an [`HttpVersion`] from a string containing the corresponding version.
    /// Returns [`HttpVersion`] if string corresponds to an implemented method, else returns [`HttpRequestError`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(HttpVersion::V11),
            _ => Err(HttpRequestError::new("Unknown http version")),
        }
    }
}

/// Stores HTTP request line information
pub struct HttpRequestLine {
    pub method: HttpMethod,
    pub uri: String,
    pub version: HttpVersion,
}

impl FromStr for HttpRequestLine {
    type Err = HttpRequestError;

    /// Create an [`HttpRequestLine`] from a string containing the complete request line.
    /// Returns [`HttpRequestLine`] if success, else return [`HttpRequestError`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"([A-Z]*) (.*) (HTTP/[1-9.]*)"#)
            .map_err(|_| HttpRequestError::new("Not able to parse input Http request"))?;

        let caps = match re.captures(s) {
            Some(caps) => caps,
            None => {
                return Err(HttpRequestError::new(
                    "No captures found when parsing Http request",
                ))
            }
        };

        let method = match caps.get(1) {
            Some(match_method) => HttpMethod::from_str(match_method.as_str())?,
            None => {
                return Err(HttpRequestError::new(
                    "Http method not found in request line",
                ))
            }
        };

        let uri = match caps.get(2) {
            Some(uri) => uri.as_str(),
            None => return Err(HttpRequestError::new("Http uri not found in request line")),
        };

        let version = match caps.get(3) {
            Some(match_method) => HttpVersion::from_str(match_method.as_str())?,
            None => {
                return Err(HttpRequestError::new(
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

/// Stores full HTTP request content
pub struct HttpRequest {
    pub line: HttpRequestLine,
}

impl FromStr for HttpRequest {
    type Err = HttpRequestError;

    /// Creates an [`HttpRequest`] from a string containing the complete HTTP request.
    /// Returns [`HttpRequest`] if success, else returns [`HttpRequestError`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = HttpRequestLine::from_str(s)?;

        Ok(HttpRequest { line })
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
