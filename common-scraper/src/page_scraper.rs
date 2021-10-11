use crate::PotentialListing;
use url::Url;

pub trait PageScraper: Send + Sync {
    fn scrape_page(&self, page_url: Url) -> (Vec<PotentialListing>, Option<Url>);
}
