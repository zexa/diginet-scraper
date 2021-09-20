use url::Url;

pub struct PotentialListing<'a> {
    listing_url: &'a Url,
    listing_page_url: &'a Url,
}

impl<'a> PotentialListing<'a> {
    pub fn new(url: &'a Url, from_url: &'a Url) -> Self {
        Self {
            listing_url: url,
            listing_page_url: from_url,
        }
    }

    pub fn listing_url(&self) -> &Url {
        self.listing_url
    }

    pub fn listing_page_url(&self) -> &Url {
        self.listing_page_url
    }
}
