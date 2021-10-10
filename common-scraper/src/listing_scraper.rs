use crate::{Listing, PotentialListing};

pub trait ListingScraper<L>: Send
where
    L: Listing,
{
    fn scrape_listing(&self, _potential_listing: &PotentialListing) -> Option<L>;
}
