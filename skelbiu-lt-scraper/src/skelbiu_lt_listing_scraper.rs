use crate::skelbiu_lt_listing::SkelbiuLtListing;
use common_scraper::{ListingScraper, PotentialListing};
use scraper::Selector;
use slog::Logger;

pub struct SkelbiuLtListingScraper {
    logger: Logger,
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
        logger: Logger,
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
            logger,
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
        debug!(self.logger, "Started logging {:?}", &potential_listing);

        let listing_url = potential_listing.listing_url().to_string();
        if let Ok(response) = reqwest::blocking::get(&listing_url) {
            debug!(self.logger, "Got response from {}", &listing_url);

            let html = scraper::Html::parse_document(response.text().unwrap().as_str());
            debug!(self.logger, "Parsed html for {}", &listing_url);

            let title = html
                .select(&self.title_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find title for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .to_string();
            debug!(self.logger, "Found title for {}", &listing_url);

            let description = match html.select(&self.description_selector).next() {
                None => {
                    debug!(
                        self.logger,
                        "Could not find description for {}", &listing_url
                    );

                    None
                }
                Some(description) => {
                    debug!(self.logger, "Found description for {}", &listing_url);

                    Some(description.text().collect::<String>().trim().to_string())
                }
            };

            let mut id = html
                .select(&self.id_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find id for {}", &listing_url))
                .text()
                .collect::<String>();
            if let Some(id_pos) = id.find("ID: ") {
                id = (&id[id_pos..]).replace("ID: ", "");
            }
            id = id.trim().to_string();
            debug!(self.logger, "Found id for {}", &listing_url);

            let views = html
                .select(&self.view_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find views for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .to_string();
            debug!(self.logger, "Found views for {}", &listing_url);

            let updated_at = html
                .select(&self.updated_at_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find updated_at for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .replace("Atnaujintas ", "");
            debug!(self.logger, "Found updated_at for {}", &listing_url);

            let liked_amount = html
                .select(&self.liked_amount_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find liked_amount for {}", &listing_url))
                .text()
                .collect::<String>()
                .trim()
                .replace("Įsimintas ", "");
            debug!(self.logger, "Found liked_amount for {}", &listing_url);

            let mut location = html
                .select(&self.location_selector)
                .next()
                .unwrap_or_else(|| panic!("Could not find location for {}", &listing_url))
                .text()
                .collect::<String>();
            if let Some(send_index) = location.find("Siųsti siuntą vos nuo") {
                location.truncate(send_index);
            }
            location = location.trim().to_string();
            debug!(self.logger, "Found location for {}", &listing_url);

            let quality = if let Some(quality) = html.select(&self.quality_selector).next() {
                debug!(self.logger, "Found quality for {}", &listing_url);

                Some(quality.text().collect::<String>().trim().to_string())
            } else {
                debug!(self.logger, "Could not find quality for {}", &listing_url);

                None
            };

            let price = if let Some(price) = html.select(&self.price_selector).next() {
                debug!(self.logger, "Found price for {}", &listing_url);

                Some(price.text().collect::<String>().trim().to_string())
            } else {
                debug!(self.logger, "Could not find price for {}", &listing_url);

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
