use crate::skelbiu_lt_listing::SkelbiuLtListing;
use crate::skelbiu_lt_listing_scraper::SkelbiuLtListingScraper;
use common_scraper::{
    CommonPageScraper, CommonScrapper, ListingScraper, PageScraper, PotentialListing,
    ScraperSettings,
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
            "#itemsList > ul > li > a".to_string(),
            ".pagination_link:last-child".to_string(),
        ))
    }

    fn get_listing_scraper(&self) -> Box<dyn ListingScraper<SkelbiuLtListing>> {
        // TODO: Refactor this to use DI & clone
        Box::new(SkelbiuLtListingScraper::new(
            ".id".to_string(),
            "h1[itemprop=name]".to_string(),
            ".description".to_string(),
            ".showed".to_string(),
            "div[class='block']".to_string(),
        ))
    }

    fn get_scraper_settings(&self) -> &ScraperSettings {
        &self.scraper_settings
    }
}
