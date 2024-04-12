# TLDR Scraper

TLDR Scraper is a Rust-based web scraping tool to fetch and parse [TLDR newsletters](https://tldr.tech/).

## Dependencies

TLDR Scraper leverages several Rust crates to perform its tasks:

- `chrono`: For handling dates.
- `reqwest`: For making HTTP requests.
- `scraper`: For parsing and querying HTML documents.
- `serde`: For serializing and deserializing data structures.

## Usage

TLDR Scraper is designed as a library, which means it can be integrated into other Rust projects. Here's a basic example of how to use it:

```bash
cargo add tldr-scraper --git https://github.com/insprac/tldr-scraper
```

```rust
use tldr_scraper::{Newsletter, Article};
use chrono::NaiveDate;

#[tokio::main]
async fn main() -> Result<(), tldr_scraper::error::Error> {
    let category = "ai";
    let date = NaiveDate::from_ymd(2024, 4, 11);
    let newsletter = Newsletter::load(category, date).await?;
    println!("Newsletter: {} - {}", newsletter.title, newsletter.subtitle);
}
```
