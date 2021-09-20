use crate::{PotentialListing, PageScraper, PotentialListingStream};
use futures::Stream;
use url::Url;

pub struct CommonPageScraper<'a> {
    listing_selector: &'a str,
    next_listing_page_selector: &'a str,
}

impl<'a> CommonPageScraper<'a> {
    pub fn new(listing_selector: &'a str, next_listing_page_selector: &'a str) -> Self {
        Self {
            listing_selector,
            next_listing_page_selector,
        }
    }
}

impl PageScraper for CommonPageScraper<'_> {
    fn scrape_page(&self, page_url: &Url) -> (Vec<PotentialListing>, Option<Url>) {
        todo!()
    }
}
