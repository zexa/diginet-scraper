use crate::skelbiu_lt_listing::SkelbiuLtListing;
use common_scraper::{Listing, ListingScraper, PotentialListing};
use scraper::Selector;

pub struct SkelbiuLtListingScraper {
    id_selector: Selector,
    title_selector: Selector,
    description_selector: Selector,
}

impl SkelbiuLtListingScraper {
    pub fn new(id_selector: String, title_selector: String, description_selector: String) -> Self {
        let id_selector = Selector::parse(id_selector.as_str()).unwrap();
        let title_selector = Selector::parse(title_selector.as_str()).unwrap();
        let description_selector = Selector::parse(description_selector.as_str()).unwrap();

        Self {
            id_selector,
            title_selector,
            description_selector,
        }
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
                .trim()
                .to_string();

            let description = html
                .select(&self.description_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            let id = html
                .select(&self.id_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .replace("ID: ", "")
                .trim()
                .to_string();

            return Some(SkelbiuLtListing::new(listing_url, id, title, description));
        }

        None
    }
}
