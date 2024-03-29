mod skelbiu_lt_listing;
mod skelbiu_lt_listing_scraper;
mod skelbiu_lt_scraper;

pub use skelbiu_lt_listing::SkelbiuLtListing;
pub use skelbiu_lt_listing_scraper::SkelbiuLtListingScraper;
pub use skelbiu_lt_scraper::SkelbiuLtScraper;

#[cfg(test)]
mod tests {
    use super::*;
    use common_scraper::{CommonScrapper, ScraperSettings};
    use url::Url;

    #[test]
    fn it_works() {
        println!("Initializing test");
        let scraper = SkelbiuLtScraper::new(ScraperSettings::new(3));

        let result = scraper.scrape_page(
            Url::parse("https://www.skelbiu.lt/skelbimai/?keywords=nvidia+gtx+1060").unwrap(),
        );

        while let Ok(listing) = result.0.recv() {
            println!("{:?}", listing)
        }
    }
}
