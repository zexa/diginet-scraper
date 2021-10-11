use common_scraper::Listing;

#[derive(Clone, Debug)]
pub struct SkelbiuLtListing {
    url: String,
    title: String,
}

impl SkelbiuLtListing {
    pub fn new(url: String, title: String) -> Self {
        Self { url, title }
    }

    fn get_title(&self) -> &str {
        &self.title
    }
}

impl Listing for SkelbiuLtListing {
    fn get_url(&self) -> &str {
        &self.url
    }
}
