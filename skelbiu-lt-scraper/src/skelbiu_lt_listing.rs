use common_scraper::Listing;
use url::Url;

#[derive(Clone, Debug)]
pub struct SkelbiuLtListing;

impl SkelbiuLtListing {
    pub fn new() -> Self {
        Self
    }
}

impl Listing for SkelbiuLtListing {
    fn get_url(&self) -> &Url {
        todo!()
    }
}
