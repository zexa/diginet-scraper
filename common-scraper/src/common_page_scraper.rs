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
    fn scrape_page(&self, page_url: Url) -> (Vec<PotentialListing>, Option<Url>) {
        // TODO: Should not unwrap if possible
        let result = reqwest::blocking::get(page_url.to_string()).unwrap();
        let html = scraper::Html::parse_document(result.text().unwrap().as_str());
        let listing_selector = scraper::Selector::parse(self.listing_selector.as_str()).unwrap();
        let next_page_selector =
            scraper::Selector::parse(self.next_listing_page_selector.as_str()).unwrap();

        let listings: Vec<PotentialListing> = html
            .select(&listing_selector)
            .map(|elem| elem.value().attr("href").unwrap())
            .map(|elem| PotentialListing::new(page_url.join(elem).unwrap(), page_url.clone()))
            .collect();

        let next_page = html
            .select(&next_page_selector)
            .map(|elem| elem.value().attr("href").unwrap())
            .map(|elem| page_url.join(elem).unwrap())
            .next();

        (listings, next_page)
    }
}
