use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use crate::Article;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Newsletter {
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub date: NaiveDate,
    pub articles: Vec<Article>,
}

impl Newsletter {
    pub async fn load(category: &str, date: NaiveDate) -> Result<Self> {
        let html = Self::download(category, date).await?;
        Self::from_html(category, date, html)
    }

    pub fn from_html(category: &str, date: NaiveDate, html: String) -> Result<Self> {
        crate::scrape::scrape(html, category.to_owned(), date)
    }

    async fn download(category: &str, date: NaiveDate) -> Result<String> {
        let url = format!("https://tldr.tech/{}/{}", category, date);
        let response = reqwest::get(&url).await?;
        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(Error::Http(response.status().as_u16()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load() {
        let date = NaiveDate::from_ymd_opt(2024, 4, 11).unwrap();
        let newsletter = Newsletter::load("ai", date).await.unwrap();

        assert_eq!(newsletter.category, "ai");
        assert_eq!(newsletter.date, date);
        assert_eq!(newsletter.title, "TLDR AI 2024-04-11");
        assert!(newsletter.subtitle.contains("Google’s Gemini Pro 1.5"));

        assert_eq!(newsletter.articles.len(), 14);
        assert!(newsletter.articles[0].title.contains("Microsoft could update Copilot"));
        assert!(newsletter.articles[0].description.contains("Microsoft is collaborating with OpenAI"));
        assert!(newsletter.articles[0].url.contains("https://windowsreport.com"));
    }
}
