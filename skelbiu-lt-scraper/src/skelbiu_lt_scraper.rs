use crate::skelbiu_lt_listing::SkelbiuLtListing;
use crate::skelbiu_lt_listing_scraper::SkelbiuLtListingScraper;
use common_scraper::{
    CommonPageScraper, CommonScrapper, ListingScraper, PageScraper, ScraperSettings,
};

pub struct SkelbiuLtScraper {
    scraper_settings: ScraperSettings,
}

impl SkelbiuLtScraper {
    pub fn new(scraper_settings: ScraperSettings) -> Self {
        Self { scraper_settings }
    }
}

impl CommonScrapper<SkelbiuLtListing> for SkelbiuLtScraper {
    fn get_page_scraper(&self) -> Box<dyn PageScraper> {
        // TODO: Refactor this to use DI & clone
        Box::new(CommonPageScraper::new(
            "#itemsList > ul > li > a",
            ".pagination_link:last-child",
        ))
    }

    fn get_listing_scraper(&self) -> Box<dyn ListingScraper<SkelbiuLtListing>> {
        // TODO: Refactor this to use DI & clone
        Box::new(SkelbiuLtListingScraper::new(
            ".id",
            "h1[itemprop=name]",
            ".description",
            ".showed",
            "div[class='block']",
            ".bookmarks",
            ".cities",
            ".value:not(.js-monthly-payment)",
            ".price",
        ))
    }

    fn get_scraper_settings(&self) -> &ScraperSettings {
        &self.scraper_settings
    }
}
