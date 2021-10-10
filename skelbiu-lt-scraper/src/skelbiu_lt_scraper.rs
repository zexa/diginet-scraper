use crate::skelbiu_lt_listing::SkelbiuLtListing;
use crate::skelbiu_lt_listing_scraper::SkelbiuLtListingScraper;
use crate::skelbiu_lt_query::SkelbiuLtQuery;
use common_scraper::{
    CommonPageScraper, Listing, ListingScraper, PageScraper, PotentialListing, Scraper,
    ScraperSettings,
};
use futures::Stream;

struct SkelbiuLtScraper {
    scraper_settings: ScraperSettings,
}

impl SkelbiuLtScraper {
    pub fn new(scraper_settings: ScraperSettings) -> Self {
        Self { scraper_settings }
    }
}

impl Scraper<SkelbiuLtListing> for SkelbiuLtScraper {
    fn get_page_scraper(&self) -> Box<dyn PageScraper> {
        Box::new(CommonPageScraper::new("".to_string(), "".to_string()))
    }

    fn get_listing_scraper(&self) -> Box<dyn ListingScraper<SkelbiuLtListing>> {
        Box::new(SkelbiuLtListingScraper::new())
    }

    fn get_scraper_settings(&self) -> &ScraperSettings {
        &self.scraper_settings
    }
}
