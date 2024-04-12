use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(u16),
    Reqwest(reqwest::Error),
    Parser(String),
    Selector(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Http(status_code) => write!(f, "TLDR Error: HTTP error: {}", status_code),
            Error::Reqwest(err) => write!(f, "TLDR Error: reqwest error: {}", err),
            Error::Parser(err) => write!(f, "TLDR Error: parser error: {}", err),
            Error::Selector(err) => write!(f, "TLDR Error: selector error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl<'a> From<scraper::error::SelectorErrorKind<'a>> for Error {
    fn from(err: scraper::error::SelectorErrorKind<'a>) -> Self {
        Error::Selector(err.to_string())
    }
}
