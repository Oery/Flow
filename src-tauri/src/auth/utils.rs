use std::{error::Error, io};
use url::Url;

pub fn parse_url(url_str: String) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(&url_str)?;
    match url.query_pairs().find(|(key, _)| key == "code") {
        Some((_, value)) => Ok(value.into_owned()),
        None => Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "No code found"))),
    }
}
