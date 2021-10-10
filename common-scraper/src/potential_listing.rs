use url::Url;

#[derive(Clone)]
pub struct PotentialListing {
    listing_url: Url,
    listing_page_url: Url,
}

impl<'a> PotentialListing {
    pub fn new(url: Url, from_url: Url) -> Self {
        Self {
            listing_url: url,
            listing_page_url: from_url,
        }
    }

    pub fn listing_url(&self) -> &Url {
        &self.listing_url
    }

    pub fn listing_page_url(&self) -> &Url {
        &self.listing_page_url
    }
}
