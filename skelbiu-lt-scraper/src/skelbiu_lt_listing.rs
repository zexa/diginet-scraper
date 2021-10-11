use common_scraper::Listing;

#[derive(Clone, Debug)]
pub struct SkelbiuLtListing {
    url: String,
    id: String,
    title: String,
    description: String,
}

impl SkelbiuLtListing {
    pub fn new(url: String, id: String, title: String, description: String) -> Self {
        Self {
            url,
            id,
            title,
            description,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

impl Listing for SkelbiuLtListing {
    fn get_url(&self) -> &str {
        &self.url
    }
}
