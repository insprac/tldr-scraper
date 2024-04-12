use crate::error::{Error, Result};
use crate::{Article, Newsletter};
use chrono::NaiveDate;
use scraper::{ElementRef, Html, Selector};

pub fn scrape(html: String, category: String, date: NaiveDate) -> Result<Newsletter> {
    let document = Html::parse_document(&html);
    let element = document.root_element();
    let content = select_single_element(&element, ".content-center")?;
    let title = extract_text(&content, "h1")?;
    let subtitle = extract_text(&content, "h2")?;
    let articles = scrape_articles(&content)?;

    Ok(Newsletter {
        title,
        subtitle,
        articles,
        category,
        date,
    })
}

fn scrape_articles(element: &ElementRef) -> Result<Vec<Article>> {
    let sections = select_elements(element, ":scope > div")?;
    let mut articles = Vec::new();

    for section in sections {
        let article_elements = select_elements(&section, ":scope > div")?;
        for article_element in article_elements {
            if has_child(&article_element, ":scope > a > h3")? {
                articles.push(scrape_article(&article_element)?);
            }
        }
    }
    Ok(articles)
}

fn scrape_article(element: &ElementRef) -> Result<Article> {
    let title = extract_text(element, ":scope > a > h3")?;
    let description = extract_text(element, ":scope > div")?;
    let url = element
        .select(&Selector::parse(":scope > a")?)
        .next()
        .ok_or_else(|| Error::Parser("Link not found".into()))?
        .value()
        .attr("href")
        .ok_or_else(|| Error::Parser("URL not found".into()))?
        .to_owned();

    Ok(Article {
        title,
        description,
        url,
    })
}

fn extract_text(element: &ElementRef, selector_str: &str) -> Result<String> {
    select_single_element(element, selector_str).map(|e| e.text().collect())
}

fn select_single_element<'a>(
    element: &'a ElementRef,
    selector_str: &str,
) -> Result<ElementRef<'a>> {
    let selector = Selector::parse(selector_str)?;
    element
        .select(&selector)
        .next()
        .ok_or_else(|| Error::Parser(format!("Element not found for selector: {}", selector_str)))
}

fn select_elements<'a>(element: &'a ElementRef, selector_str: &str) -> Result<Vec<ElementRef<'a>>> {
    let selector = Selector::parse(selector_str)?;
    Ok(element.select(&selector).collect())
}

fn has_child(element: &ElementRef, selector_str: &str) -> Result<bool> {
    let selector = Selector::parse(selector_str)?;
    Ok(element.select(&selector).next().is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use scraper::Html;

    fn create_document(html: &str) -> Html {
        Html::parse_fragment(html)
    }

    #[test]
    fn test_scrape_successful() {
        let html = r#"
            <div class="content-center">
                <h1>Title of Newsletter</h1>
                <h2>Subtitle of Newsletter</h2>
                <div>
                    <div>
                        <a href="http://example.com/article">
                            <h3>Article Title</h3>
                        </a>
                        <div>Description of Article</div>
                    </div>
                </div>
            </div>
        "#;
        let date = NaiveDate::from_ymd_opt(2023, 4, 12).unwrap();

        let result = scrape(html.to_string(), "News".to_string(), date).unwrap();
        assert_eq!(result.title, "Title of Newsletter");
        assert_eq!(result.subtitle, "Subtitle of Newsletter");
        assert_eq!(result.articles.len(), 1);
        assert_eq!(result.articles[0].title, "Article Title");
        assert_eq!(result.articles[0].description, "Description of Article");
        assert_eq!(result.articles[0].url, "http://example.com/article");
    }

    #[test]
    fn test_scrape_no_content() {
        let html = r#"<div class="wrong-class"></div>"#;
        let date = NaiveDate::from_ymd_opt(2023, 4, 12).unwrap();
        assert!(scrape(html.to_string(), "News".to_string(), date).is_err());
    }

    #[test]
    fn test_extract_text_success() {
        let html = r#"<div><h1>Hello World</h1></div>"#;
        let document = create_document(html);
        let element = document.root_element();
        assert_eq!(extract_text(&element, "h1").unwrap(), "Hello World");
    }

    #[test]
    fn test_extract_text_fail() {
        let html = r#"<div><h1>Hello World</h1></div>"#;
        let document = create_document(html);
        let element = document.root_element();
        assert!(extract_text(&element, "h2").is_err());
    }

    #[test]
    fn test_select_single_element_success() {
        let html = r#"<div><p>Paragraph</p></div>"#;
        let document = create_document(html);
        let element = document.root_element();
        assert!(select_single_element(&element, "p").is_ok());
    }

    #[test]
    fn test_select_single_element_not_found() {
        let html = r#"<div><p>Paragraph</p></div>"#;
        let document = create_document(html);
        let element = document.root_element();
        assert!(select_single_element(&element, "h1").is_err());
    }

    #[test]
    fn test_select_elements_success() {
        let html = r#"<div><span>One</span><span>Two</span></div>"#;
        let document = create_document(html);
        let element = document.root_element();
        let elements = select_elements(&element, "span").unwrap();
        assert_eq!(elements.len(), 2);
    }

    #[test]
    fn test_has_child_success() {
        let html = r#"<div><a><h3>Title</h3></a></div>"#;
        let document = create_document(html);
        let element = document.root_element();
        assert!(has_child(&element, "a > h3").unwrap());
    }

    #[test]
    fn test_has_child_failure() {
        let html = r#"<div><a><h3>Title</h3></a></div>"#;
        let document = create_document(html);
        let element = document.root_element();
        assert!(!has_child(&element, "a > h4").unwrap());
    }
}
