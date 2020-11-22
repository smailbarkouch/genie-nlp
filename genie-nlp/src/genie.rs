use rust_bert::RustBertError;
use crate::search::SearchForContent;
use crate::choice::{NLPHelp, RelevantAnswer};

#[derive(Debug)]
pub enum GenieError {
    WikiError(wikipedia::Error),
    NLPError(RustBertError),
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
        let mut answer_results = Vec::<RelevantAnswer>::new();

        println!("Found wiki pages.");
        search_results.iter().for_each(|result| {
            let article = searcher.get_wiki_article(result.clone()).unwrap();
            answer_results.append(NLPHelp::is_relevant(question, article.summary).unwrap().as_mut());
        });
        println!("Found the most relevant pages.");

        let mut best_answer = String::from("Genie couldn't figure out the answer.");
        let mut best_score = 0f64;
        answer_results.iter().for_each(|result| {
            if result.score > best_score {
                best_answer = format!("The answer should be: {}", result.answer.clone());
                best_score = result.score;
            }
        });

        println!("{}", best_answer);
    }

}
