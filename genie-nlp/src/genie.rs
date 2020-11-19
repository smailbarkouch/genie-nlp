use rust_bert::RustBertError;

#[derive(Debug)]
pub enum GenieError {
    WikiError(wikipedia::Error),
    NLPError(RustBertError),
    RequestError(reqwest::Error),
    HtmlParseError(std::io::Error),
    HtmlSearchError(std::fmt::Error)
}

impl From<wikipedia::Error> for GenieError {
    fn from(error: wikipedia::Error) -> Self {
        GenieError::WikiError(error)
    }
}

impl From<reqwest::Error> for GenieError {
    fn from(error: reqwest::Error) -> Self {
        GenieError::RequestError(error)
    }
}

impl From<std::io::Error> for GenieError {
    fn from(error: std::io::Error) -> Self {
        GenieError::HtmlParseError(error)
    }
}

impl From<std::fmt::Error> for GenieError {
    fn from(error: std::fmt::Error) -> Self {
        GenieError::HtmlSearchError(error)
    }
}

pub struct Genie {}

impl Genie {
    pub fn get
}
