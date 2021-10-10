use crate::skelbiu_lt_listing::SkelbiuLtListing;
use common_scraper::{Listing, ListingScraper, PotentialListing};

pub struct SkelbiuLtListingScraper;

impl SkelbiuLtListingScraper {
    pub fn new() -> Self {
        Self
    }
}

impl ListingScraper<SkelbiuLtListing> for SkelbiuLtListingScraper {
    fn scrape_listing(&self, _potential_listing: &PotentialListing) -> Option<SkelbiuLtListing> {
        Some(SkelbiuLtListing::new())
    }
}
