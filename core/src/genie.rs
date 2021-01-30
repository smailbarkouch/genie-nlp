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
        println!("Looking for relevant wiki pages.");
        let searcher = SearchForContent::new();
        let search_results = searcher.wiki_search(question).unwrap();
        let mut answer_results = Vec::<RelevantAnswer>::new();

        println!("Looking through each summary for the best answer.");
        for (index, result) in search_results.iter().enumerate() {
            let article = searcher.get_wiki_article(result.clone()).unwrap();
            let answer_relevance = NLPHelp::is_relevant(question, article.summary, index + 1).unwrap();
            if answer_relevance.is_some() {
                answer_results.push(answer_relevance.unwrap());
            }
        }

        println!("Taking the best answers and finding the highest scoring one.");
        let mut best_answer = String::from("Genie couldn't find an accurate enough answer.");
        let mut best_score = 0f64;
        answer_results.iter().for_each(|result| {
            if result.score > best_score {
                best_answer = format!("The answer, with a score of {}, should be: {}", result.score, result.answer.clone());
                best_score = result.score;
            }
        });

        println!("{}", best_answer);
    }

}
