use crate::{PageScraper, Listing, PotentialListing, ListingScraper, PotentialListingStream};
use futures::{Stream, StreamExt, FutureExt};
use std::sync::{Arc, Mutex};
use crate::listing_stream::ListingStream;
use url::Url;

pub trait Scraper<L> where L: Listing + std::marker::Send {
    fn scrape(&self, initial_url: Url) -> ListingStream<L> where Self: Sync {
        let potential_listing_mutex = Arc::new(Mutex::new(Vec::<PotentialListing>::new()));
        // tasks created using spawn can outlive the current function
        // so the solution would be to move only the pieces that are needed here.
        // might be for the best if we were to move
        tokio::task::spawn(async {
            let mut next_page: Option<Url> = Some(initial_url);
            while let Some(page) = next_page.clone() {
                let potential_listings: Vec<PotentialListing> = vec![];
                (potential_listings, next_page) = self.get_page_scraper().scrape_page(&page);
                potential_listing_mutex.lock().unwrap().append(&mut potential_listings);
            }
        });

        let mut potential_listing_stream = PotentialListingStream::new(potential_listing_mutex);
        let listing_mutex = Arc::new(Mutex::new(Vec::<L>::new()));
        tokio::task::spawn(async {
            potential_listing_stream.then(|potential_listing| async move {
                if let Some(listing) = self.get_listing_scraper().scrape_listing(&potential_listing) {
                    listing_mutex.lock().unwrap().push(listing)
                };
            });
        });

        ListingStream::new(listing_mutex)
    }

    fn get_page_scraper(&self) -> Box<dyn PageScraper>;

    fn get_listing_scraper(&self) -> Box<dyn ListingScraper<L>>;
}
