use crate::skelbiu_lt_listing::SkelbiuLtListing;
use common_scraper::{ListingScraper, PotentialListing};
use scraper::Selector;
use std::ops::Index;

pub struct SkelbiuLtListingScraper {
    id_selector: Selector,
    title_selector: Selector,
    description_selector: Selector,
    view_selector: Selector,
    updated_at_selector: Selector,
    liked_amount_selector: Selector,
    location_selector: Selector,
    quality_selector: Selector,
    price_selector: Selector,
}

impl SkelbiuLtListingScraper {
    pub fn new(
        id_selector: &str,
        title_selector: &str,
        description_selector: &str,
        view_selector: &str,
        updated_at_selector: &str,
        liked_amount_selector: &str,
        location_selector: &str,
        quality_selector: &str,
        price_selector: &str,
    ) -> Self {
        let id_selector = Selector::parse(id_selector).unwrap();
        let title_selector = Selector::parse(title_selector).unwrap();
        let description_selector = Selector::parse(description_selector).unwrap();
        let view_selector = Selector::parse(view_selector).unwrap();
        let updated_at_selector = Selector::parse(updated_at_selector).unwrap();
        let liked_amount_selector = Selector::parse(liked_amount_selector).unwrap();
        let location_selector = Selector::parse(location_selector).unwrap();
        let quality_selector = Selector::parse(quality_selector).unwrap();
        let price_selector = Selector::parse(price_selector).unwrap();

        Self {
            id_selector,
            title_selector,
            description_selector,
            view_selector,
            updated_at_selector,
            liked_amount_selector,
            location_selector,
            quality_selector,
            price_selector,
        }
    }
}

impl ListingScraper<SkelbiuLtListing> for SkelbiuLtListingScraper {
    fn scrape_listing(&self, potential_listing: &PotentialListing) -> Option<SkelbiuLtListing> {
        let listing_url = potential_listing.listing_url().to_string();
        if let Ok(response) = reqwest::blocking::get(&listing_url) {
            let html = scraper::Html::parse_document(response.text().unwrap().as_str());

            let title = html
                .select(&self.title_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find title for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            let description = html
                .select(&self.description_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find description for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            let id = html
                .select(&self.id_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find id for {}", &listing_url))
                .text()
                .collect::<String>()
                .replace("ID: ", "")
                .trim()
                .to_string();

            let views = html
                .select(&self.view_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find views for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            let updated_at = html
                .select(&self.updated_at_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find updated_at for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .replace("Atnaujintas ", "");

            let liked_amount = html
                .select(&self.liked_amount_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find liked_amount for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .replace("Įsimintas ", "");

            let mut location = html
                .select(&self.location_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find location for {}", &listing_url))
                .text()
                .collect::<String>();
            location.truncate(location.find("Siųsti siuntą vos nuo").unwrap());
            location = location.trim().to_string();

            let quality = if let Some(quality) = html.select(&self.quality_selector).next() {
                Some(quality.text().collect::<String>().trim().to_string())
            } else {
                None
            };

            let price = if let Some(price) = html.select(&self.price_selector).next() {
                Some(price.text().collect::<String>().trim().to_string())
            } else {
                None
            };

            return Some(SkelbiuLtListing::new(
                listing_url,
                id,
                title,
                description,
                views,
                updated_at,
                liked_amount,
                location,
                quality,
                price,
            ));
        }

        None
    }
}
