use wikipedia::Wikipedia;
use wikipedia::http::default::Client;
use crate::genie::GenieError;

pub struct SearchForContent {
    pub wikipedia: Wikipedia<Client>
}

pub struct WikiArticle {
    pub summary: String,
    pub content: String
}

impl SearchForContent {
    pub fn new() -> Self {
        SearchForContent { wikipedia: Default::default() }
    }

    pub fn wiki_search(&self, phrase: &str) -> Result<Vec<String>, GenieError> {
        Ok(self.wikipedia.search(phrase)?)
    }

    pub fn get_wiki_article(&self, title: String) -> Result<WikiArticle, GenieError> {
        let page_content = self.wikipedia.page_from_title(title);
        Ok(WikiArticle {
            summary: page_content.get_summary()?.clone(),
            content: page_content.get_content()?.clone()
        })
    }

}