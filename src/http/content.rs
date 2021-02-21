use mime::Mime;
use std::fs;
use std::path::Path;

pub type Message = Vec<u8>;

/// Load content as a vector of bytes from the provided URI. Could be images, HTML file, etc.
/// Returns std::io::Error if loading failed
pub fn load_content_from_uri(uri: &str) -> Result<Message, std::io::Error> {
    let path = Path::new(uri);
    fs::read(path)
}

/// Returns a Mime type based on the filename. Returns text/plain by default.
pub fn find_mimetype(filename: &str) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();

    let result = match parts.last() {
        Some(v) => match *v {
            "html" => mime::TEXT_HTML,
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };

    result
}

/// Returns a string of a standard content type line based on the Mime type.
pub fn build_content_type(mime: &Mime) -> String {
    format!("Content-Type: {}/{}\r\n", mime.type_(), mime.subtype())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_existing_file() {
        let uri = "example/hello.html";

        let result = load_content_from_uri(&uri);

        assert!(result.is_ok());
    }

    #[test]
    fn test_load_non_existing_png_file() {
        let uri = "non_existing.png";

        let result = load_content_from_uri(&uri);

        assert!(result.is_err());
    }

    #[test]
    fn test_find_html_mime_type() {
        let filepath = "example/content.html";

        let result = find_mimetype(filepath);

        assert_eq!(result, mime::TEXT_HTML);
    }

    #[test]
    fn test_find_png_mime_type() {
        let filepath = "example/content.png";

        let result = find_mimetype(filepath);

        assert_eq!(result, mime::IMAGE_PNG);
    }

    #[test]
    fn test_find_non_existing_type() {
        let filepath = "example/content.pn";

        let result = find_mimetype(filepath);

        assert_eq!(result, mime::TEXT_PLAIN);
    }
}
