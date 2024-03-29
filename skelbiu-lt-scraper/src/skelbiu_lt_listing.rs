use common_scraper::Listing;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SkelbiuLtListing {
    url: String,
    id: String,
    title: String,
    description: Option<String>,
    views: String,
    updated_at: String,
    liked_amount: String,
    location: String,
    quality: Option<String>,
    price: Option<String>,
    price_change: Option<String>,
}

impl SkelbiuLtListing {
    pub fn new(
        url: String,
        id: String,
        title: String,
        description: Option<String>,
        views: String,
        updated_at: String,
        liked_amount: String,
        location: String,
        quality: Option<String>,
        price: Option<String>,
        price_change: Option<String>,
    ) -> Self {
        Self {
            url,
            id,
            title,
            description,
            views,
            updated_at,
            liked_amount,
            location,
            quality,
            price,
            price_change,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    pub fn get_views(&self) -> &str {
        &self.views
    }

    pub fn get_updated_at(&self) -> &str {
        &self.updated_at
    }

    pub fn get_liked_amount(&self) -> &str {
        &self.liked_amount
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn get_quality(&self) -> &Option<String> {
        &self.quality
    }

    pub fn get_price(&self) -> &Option<String> {
        &self.price
    }

    pub fn get_price_change(&self) -> &Option<String> {
        &self.price_change
    }
}

impl Listing for SkelbiuLtListing {
    fn get_url(&self) -> &str {
        &self.url
    }
}
