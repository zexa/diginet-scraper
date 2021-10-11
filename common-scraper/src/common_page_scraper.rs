use crate::{PageScraper, PotentialListing};
use scraper::Selector;
use url::Url;

#[derive(Clone)]
pub struct CommonPageScraper {
    listing_selector: Selector,
    next_page_selector: Selector,
}

impl CommonPageScraper {
    pub fn new(listing_selector: String, next_page_selector: String) -> Self {
        let listing_selector = scraper::Selector::parse(listing_selector.as_str()).unwrap();
        let next_page_selector = scraper::Selector::parse(next_page_selector.as_str()).unwrap();

        Self {
            listing_selector,
            next_page_selector,
        }
    }
}

impl PageScraper for CommonPageScraper {
    fn scrape_page(&self, page_url: Url) -> (Vec<PotentialListing>, Option<Url>) {
        // TODO: Should not unwrap if possible
        let result = reqwest::blocking::get(page_url.to_string()).unwrap();
        let html = scraper::Html::parse_document(result.text().unwrap().as_str());

        let listings: Vec<PotentialListing> = html
            .select(&self.listing_selector)
            .map(|elem| elem.value().attr("href").unwrap())
            .map(|elem| PotentialListing::new(page_url.join(elem).unwrap(), page_url.clone()))
            .collect();

        let next_page = html
            .select(&self.next_page_selector)
            .map(|elem| elem.value().attr("href").unwrap())
            .map(|elem| page_url.join(elem).unwrap())
            .next();

        (listings, next_page)
    }
}
