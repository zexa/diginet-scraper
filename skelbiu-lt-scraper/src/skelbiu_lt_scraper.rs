use common_scraper::{CommonPageScraper, PageScraper, ListingScraper, Scraper, Listing, PotentialListing};
use crate::skelbiu_lt_query::SkelbiuLtQuery;
use crate::skelbiu_lt_listing::SkelbiuLtListing;
use futures::Stream;
use crate::skelbiu_lt_listing_scraper::SkelbiuLtListingScraper;

struct SkelbiuLtScraper {
    page_scraper: Box<dyn PageScraper>,
    listing_scraper: Box<dyn ListingScraper<SkelbiuLtListing>>,
}

impl SkelbiuLtScraper {
    pub fn new(page_scraper: Box<dyn PageScraper>, listing_scraper: Box<dyn ListingScraper>) -> Self {
        Self {
            page_scraper,
            listing_scraper
        }
    }
}

impl Default for SkelbiuLtScraper {
    fn default() -> Self {
        Self::new(
            Box::new(CommonPageScraper::new(
                "",
                ""
            )),
            Box::new(SkelbiuLtListingScraper::new())
        )
    }
}

impl Scraper<SkelbiuLtListing> for SkelbiuLtScraper {
    type Query = SkelbiuLtQuery;

    fn get_page_scraper(&self) -> Box<dyn PageScraper> {
        self.page_scraper.clone()
    }

    fn get_listing_scraper(&self) -> Box<dyn ListingScraper<SkelbiuLtListing>> {
        self.page_scraper.clone()
    }
}
