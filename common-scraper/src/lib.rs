#![feature(destructuring_assignment)]

mod common_page_scraper;
mod potential_listing;
mod listing_scraper;
mod listing;
mod page_scraper;
mod scraper;
mod potential_listing_stream;
mod listing_stream;

pub use common_page_scraper::CommonPageScraper;
pub use potential_listing::PotentialListing;
pub use listing::Listing;
pub use listing_scraper::ListingScraper;
pub use page_scraper::PageScraper;
pub use scraper::Scraper;
pub use potential_listing_stream::PotentialListingStream;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
