use crate::{PotentialListing, Listing, PotentialListingStream};
use futures::Stream;

pub trait ListingScraper<L> where L: Listing {
    fn scrape_listing(&self, potential_listing: &PotentialListing) -> Option<L>;
}
