use crate::scraper_settings::ScraperSettings;
use crate::semaphore_share::SemaphoreShare;
use crate::semaphore_share_result::SemaphoreShareResult;
use crate::{Listing, ListingScraper, PageScraper, PotentialListing};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use url::Url;

pub trait CommonScrapper<L>
where
    L: 'static + Listing + std::marker::Send + Clone,
{
    fn scrape(&self, initial_url: Url) -> (Receiver<L>, Vec<JoinHandle<()>>)
    where
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
                    let mut scrape_result = page_scraper.scrape_page(page);
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

                loop {
                    if let Ok(mut potential_listing_share) = potential_listing_share_mutex.lock() {
                        potential_listing_share.set_has_reported_over(true);
                        break;
                    }
                }
            }));
        }

        let (sender, receiver) = channel();

        for thread_index in 0u64..settings.get_threads() {
            let sender = sender.clone();
            let potential_listing_share_mutex = potential_listing_share_mutex.clone();
            let listing_scraper = self.get_listing_scraper();
            handles.push(thread::spawn(move || loop {
                let mut potential_listing_option = None;
                if let Ok(mut potential_listing_share) = potential_listing_share_mutex.lock() {
                    match potential_listing_share.get() {
                        SemaphoreShareResult::Red => {
                            println!("Found RED on thread {}", thread_index);
                            break;
                        }
                        SemaphoreShareResult::Green(potential_listing) => {
                            println!("Found GREEN on thread {}", thread_index);
                            potential_listing_option = Some(potential_listing);
                        }
                        SemaphoreShareResult::Yellow => {
                            println!("Found YELLOW on thread {}", thread_index);
                            continue;
                        }
                    }
                }

                if let Some(potential_listing) = potential_listing_option {
                    if let Some(listing) = listing_scraper.scrape_listing(&potential_listing) {
                        sender.send(listing).unwrap();
                    }
                }
            }));
        }

        (receiver, handles)
    }

    fn get_page_scraper(&self) -> Box<dyn PageScraper>;

    fn get_listing_scraper(&self) -> Box<dyn ListingScraper<L>>;

    fn get_scraper_settings(&self) -> &ScraperSettings;
}
