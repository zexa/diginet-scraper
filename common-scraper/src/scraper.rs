use crate::listing_stream::ListingStream;
use crate::scraper_settings::ScraperSettings;
use crate::semaphore_share::SemaphoreShare;
use crate::semaphore_share_result::SemaphoreShareResult;
use crate::{Listing, ListingScraper, PageScraper, PotentialListing};
use std::sync::{Arc, Mutex};
use std::thread;
use url::Url;

pub trait Scraper<L>
where
    L: 'static + Listing + std::marker::Send + Clone,
{
    fn scrape(&'static self, initial_url: Url) -> ListingStream<L>
    where
        Self: 'static,
        Self: Sync,
        Self: std::marker::Send,
    {
        let settings = self.get_scraper_settings();
        let potential_listing_share_mutex =
            Arc::new(Mutex::new(SemaphoreShare::<PotentialListing>::new()));
        let mut handles = vec![];

        {
            let potential_listing_share_mutex = potential_listing_share_mutex.clone();
            let page_scraper = self.get_page_scraper();
            handles.push(thread::spawn(move || {
                let mut next_page = Some(initial_url);
                while let Some(page) = next_page {
                    let mut scrape_result = page_scraper.scrape_page(&page);
                    next_page = scrape_result.1;
                    loop {
                        if let Ok(mut potential_listing_share) =
                            potential_listing_share_mutex.lock()
                        {
                            potential_listing_share.append(&mut scrape_result.0);
                            break;
                        }
                    }
                }
            }));
        }

        let listing_mutex = Arc::new(Mutex::new(Vec::<L>::new()));

        for _ in 0u64..settings.get_threads() {
            let listing_mutex = listing_mutex.clone();
            let potential_listing_share_mutex = potential_listing_share_mutex.clone();
            let listing_scraper = self.get_listing_scraper();
            handles.push(thread::spawn(move || loop {
                if let Ok(mut potential_listing_share) = potential_listing_share_mutex.lock() {
                    match potential_listing_share.get() {
                        SemaphoreShareResult::Red => break,
                        SemaphoreShareResult::Green(potential_listing) => {
                            if let Some(listing) =
                                listing_scraper.scrape_listing(&potential_listing)
                            {
                                loop {
                                    if let Ok(mut listings) = listing_mutex.lock() {
                                        listings.push(listing.clone());
                                        break;
                                    }
                                }
                            }
                        }
                        SemaphoreShareResult::Yellow => continue,
                    }
                }
            }));
        }

        ListingStream::new(listing_mutex)
    }

    fn get_page_scraper(&self) -> Box<dyn PageScraper>;

    fn get_listing_scraper(&self) -> Box<dyn ListingScraper<L>>;

    fn get_scraper_settings(&self) -> &ScraperSettings;
}
