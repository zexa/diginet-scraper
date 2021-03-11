use scraper::{ElementRef, Selector};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone)]
pub struct AruodasListing {
    url: String,
    price: String,
    area: String,
    location: String,
    price_per_area: f32,
}

pub struct AruodasScraper;

impl AruodasScraper {
    pub fn new() -> Self {
        Self
    }

    pub async fn scrape(&self, url: String, limit: Option<usize>) -> Vec<AruodasListing> {
        let mut next_listing_urls = Vec::<String>::new();
        let mut next_page_url = Some(Url::parse(&url).unwrap());
        let mut scrape_page: (Option<String>, Vec<String>);
        while let Some(page_url) = next_page_url {
            scrape_page = self.scrape_page(page_url.to_string()).await;
            next_page_url = match scrape_page.0 {
                None => None,
                Some(url) => Some(page_url.join(&url).unwrap()),
            };
            next_listing_urls.append(&mut scrape_page.1);
            match limit {
                None => {}
                Some(lim) => {
                    if next_listing_urls.len() >= lim {
                        println!("Stopping page scraping due to reached limit: {}", lim);
                        break;
                    }
                }
            };
        }

        let mut listings = Vec::<AruodasListing>::new();
        let mut parsed = 0;
        while let Some(next_listing_url) = next_listing_urls.pop() {
            listings.push(self.scrape_listing(next_listing_url).await);
            parsed += 1;
            match limit {
                None => {}
                Some(lim) => {
                    if parsed >= lim {
                        println!("Stopping listing scraping due to reached limit: {}", lim);
                        break;
                    }
                }
            }
        }

        listings
    }

    // Scrapes a page containing a table of listings
    // Returns a next page and urls of listings
    pub async fn scrape_page(&self, url: String) -> (Option<String>, Vec<String>) {
        // TODO: Keep these for as long as the struct lives, don't reinitialize it every time
        // scrape_page is called
        let page_selector = scraper::Selector::parse("body > div.main.filter-form > div.content > div.main-content > div.pagination > a:last-child")
            .unwrap();
        // Aruodas DOM is weird - non existing listings sometimes appear.
        // We bypass this with tr:nth-child(1n+4)
        // Above skips the first 4 tr elements in DOM as they do not contain useful info.
        let listing_selector = scraper::Selector::parse("body > div.main.filter-form > div.content > div.main-content > table > tbody > tr:nth-child(1n+4) > td.list-adress > h3 > a")
            .unwrap();

        println!("Parsing page {}", url);

        let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let document = scraper::Html::parse_document(body.as_str());

        let next_page_url = match document.select(&page_selector).next() {
            None => None,
            Some(next_page) => match next_page.value().attr("href") {
                None => None,
                Some(href) => Some(href.to_string()),
            },
        };

        let mut listing_urls = Vec::<String>::new();
        for listing_url_element in document.select(&listing_selector) {
            match listing_url_element.value().attr("href") {
                None => {
                    continue;
                }
                Some(listing_url) => {
                    listing_urls.push(listing_url.to_string());
                }
            }
        }

        (next_page_url, listing_urls)
    }

    // Scrapes an individual listing
    // Returns a listing
    pub async fn scrape_listing(&self, url: String) -> AruodasListing {
        // TODO: Keep these for as long as the struct lives, don't reinitialize it every time
        // scrape_listing is called
        let table_selector = scraper::Selector::parse(
            "body > div.main > div.content > div.main-content > div.obj-cont > dl",
        )
        .unwrap();

        println!("Parsing url {}", url);

        let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let document = scraper::Html::parse_document(body.as_str());

        let obj_details = self.parse_obj_details(document.select(&table_selector).next().unwrap());

        let price = obj_details
            .get("Kaina mėn.:")
            .unwrap()
            .replace(" €", "")
            .replace(",", ".");
        let area = obj_details
            .get("Plotas:")
            .unwrap()
            .replace(" m²", "")
            .replace(",", ".");
        let price_per_area = price.parse::<f32>().expect(&format!(
            "Could not parse price to floating point: {}",
            price
        )) / area
            .parse::<f32>()
            .expect(&format!("Could not parse area to floating point: {}", area));

        AruodasListing {
            url: url.clone(),
            price,
            area,
            location: "".to_string(),
            price_per_area,
        }
    }

    // Parses commonly found detail tables inside individual orders
    fn parse_obj_details(&self, obj_details: ElementRef) -> HashMap<String, String> {
        // TODO: Keep these for as long as the struct lives, don't reinitialize it every time
        // parse_obj_details is called
        let dt_selector = scraper::Selector::parse("dt").unwrap();
        let dd_selector = scraper::Selector::parse("dd").unwrap();

        let dts = self.get_obj_detail_elements(obj_details, &dt_selector);
        let dds = self.get_obj_detail_elements(obj_details, &dd_selector);
        let mut obj_hash = HashMap::<String, String>::new();
        for dt in dts.iter().enumerate() {
            match dds.iter().nth(dt.0) {
                None => continue,
                Some(dd) => obj_hash.insert(dt.1.clone(), dd.clone()),
            };
        }

        obj_hash
    }

    fn get_obj_detail_elements(&self, element: ElementRef, selector: &Selector) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for el in element.select(selector) {
            result.push(
                el.text()
                    .collect::<String>()
                    .replace("\n", "")
                    .trim()
                    .to_string(),
            );
        }

        result
    }
}

pub fn sort_by_price_per_area(mut listings: Vec<AruodasListing>) -> Vec<AruodasListing> {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..listings.len() {
            if listings[i].price_per_area > listings[i - 1].price_per_area {
                swapped = true;
                listings.swap(i - 1, i);
            }
        }
    }

    listings
}
