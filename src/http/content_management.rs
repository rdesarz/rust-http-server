use mime::Mime;
use std::fs;
use std::path::Path;

pub type Message = Vec<u8>;

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