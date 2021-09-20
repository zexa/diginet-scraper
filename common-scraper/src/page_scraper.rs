use crate::PotentialListing;
use futures::Stream;
use crate::potential_listing_stream::PotentialListingStream;
use url::Url;

pub trait PageScraper {
    fn scrape_page(&self, page_url: &Url) -> (Vec<PotentialListing>, Option<Url>);
}
