use rust_bert::RustBertError;
use crate::search::SearchForContent;
use crate::choice::NLPHelp;

#[derive(Debug)]
pub enum GenieError {
    WikiError(wikipedia::Error),
    NLPError(RustBertError),
    RequestError(reqwest::Error),
    HtmlParseError(std::io::Error),
    HtmlSearchError(std::fmt::Error),
}

impl From<wikipedia::Error> for GenieError {
    fn from(error: wikipedia::Error) -> Self {
        GenieError::WikiError(error)
    }
}

impl From<RustBertError> for GenieError {
    fn from(error: RustBertError) -> Self {
        GenieError::NLPError(error)
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
    pub fn perform_search_query(question: &str) {
        let searcher = SearchForContent::new();
        let search_results = searcher.wiki_search(question).unwrap();

        search_results.iter().for_each(|result| {
            let article = searcher.get_wiki_article(result.clone()).unwrap();
            let summaries = NLPHelp::simplify(&article.summary).unwrap();
            println!("{:?}", NLPHelp::is_relevant(question, summaries).unwrap());
        });


    }

}
