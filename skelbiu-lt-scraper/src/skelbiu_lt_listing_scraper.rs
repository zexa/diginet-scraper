use crate::skelbiu_lt_listing::SkelbiuLtListing;
use common_scraper::{Listing, ListingScraper, PotentialListing};
use scraper::Selector;

pub struct SkelbiuLtListingScraper {
    title_selector: Selector,
}

impl SkelbiuLtListingScraper {
    pub fn new(title_selector: String) -> Self {
        let title_selector = Selector::parse(title_selector.as_str()).unwrap();

        Self { title_selector }
    }
}

impl ListingScraper<SkelbiuLtListing> for SkelbiuLtListingScraper {
    fn scrape_listing(&self, potential_listing: &PotentialListing) -> Option<SkelbiuLtListing> {
        let listing_url = potential_listing.listing_url().to_string();
        if let Ok(response) = reqwest::blocking::get(&listing_url) {
            let html = scraper::Html::parse_document(response.text().unwrap().as_str());

            let title = html
                .select(&self.title_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .replace("\\n", "")
                .trim()
                .to_string();

            return Some(SkelbiuLtListing::new(listing_url, title));
        }

        None
    }
}
