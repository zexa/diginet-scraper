use crate::{PageScraper, PotentialListing};
use url::Url;

#[derive(Clone)]
pub struct CommonPageScraper {
    listing_selector: String,
    next_listing_page_selector: String,
}

impl CommonPageScraper {
    pub fn new(listing_selector: String, next_listing_page_selector: String) -> Self {
        Self {
            listing_selector,
            next_listing_page_selector,
        }
    }
}

impl PageScraper for CommonPageScraper {
    fn scrape_page(&self, _page_url: &Url) -> (Vec<PotentialListing>, Option<Url>) {
        todo!()
    }
}
