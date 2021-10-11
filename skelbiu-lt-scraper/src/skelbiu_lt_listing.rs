use common_scraper::Listing;

#[derive(Clone, Debug)]
pub struct SkelbiuLtListing {
    url: String,
    id: String,
    title: String,
    description: String,
    views: String,
    updated_at: String,
}

impl SkelbiuLtListing {
    pub fn new(
        url: String,
        id: String,
        title: String,
        description: String,
        views: String,
        updated_at: String,
    ) -> Self {
        Self {
            url,
            id,
            title,
            description,
            views,
            updated_at,
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

    pub fn get_views(&self) -> &str {
        &self.views
    }

    pub fn get_updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl Listing for SkelbiuLtListing {
    fn get_url(&self) -> &str {
        &self.url
    }
}
