use wikipedia::{Wikipedia, http, Iter};
use wikipedia::http::default::Client;
use wikipedia::iter::{Category, IterItem};
use crate::genie::GenieError;

const CLIENT_NAME: String = String::from("Genie Client");

struct SearchForContent {
    wikipedia: Wikipedia<Client>
}

struct WikiArticle<'a, A: 'a + http::HttpClient, B: IterItem> {
    categories: Iter<'a, A, B>,
    summary: String,
    content: String
}

impl SearchForContent {
    pub fn new() -> Self {
        SearchForContent { wikipedia: Default::default() }
    }

    pub fn wiki_search(&self, phrase: &str) -> Result<Vec<String>, GenieError> {
        Ok(self.wikipedia.search(phrase)?)
    }

    pub fn get_wiki_article(&self, title: String) -> Result<WikiArticle<Client, Category>, GenieError> {
        let page_content = self.wikipedia.page_from_title(title);
        Ok(WikiArticle {
            categories: page_content.get_categories()?,
            summary: page_content.get_summary()?,
            content: page_content.get_content()?
        })
    }

}