use common_scraper::{ListingScraper, PotentialListing, Listing};
use crate::skelbiu_lt_listing::SkelbiuLtListing;

pub struct SkelbiuLtListingScraper;

impl SkelbiuLtListingScraper {
    pub fn new() -> Self {
        Self
    }
}

impl ListingScraper<SkelbiuLtListing> for SkelbiuLtListingScraper {
    fn scrape_listing(&self, _potential_listing: &PotentialListing) -> SkelbiuLtListing {
        todo!()
    }
}
