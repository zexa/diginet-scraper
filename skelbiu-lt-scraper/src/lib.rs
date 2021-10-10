mod skelbiu_lt_listing;
mod skelbiu_lt_listing_scraper;
mod skelbiu_lt_query;
mod skelbiu_lt_scraper;

pub use skelbiu_lt_listing::SkelbiuLtListing;
pub use skelbiu_lt_listing_scraper::SkelbiuLtListingScraper;
pub use skelbiu_lt_query::SkelbiuLtQuery;
pub use skelbiu_lt_scraper::SkelbiuLtScraper;

#[cfg(test)]
mod tests {
    use super::*;
    use common_scraper::{Scraper, ScraperSettings};
    use url::Url;

    #[test]
    fn it_works() {
        println!("Initializing test");
        let scraper = SkelbiuLtScraper::new(ScraperSettings::new(1));

        let result = scraper.scrape(
            Url::parse("https://www.skelbiu.lt/skelbimai/?keywords=nvidia+gtx+1060").unwrap(),
        );

        while let Ok(listing) = result.0.recv() {
            println!("{:?}", listing)
        }
    }
}
