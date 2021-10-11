use common_scraper::Listing;

#[derive(Clone, Debug)]
pub struct SkelbiuLtListing {
    url: String,
    title: String,
    description: String,
}

impl SkelbiuLtListing {
    pub fn new(url: String, title: String, description: String) -> Self {
        Self {
            url,
            title,
            description,
        }
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
