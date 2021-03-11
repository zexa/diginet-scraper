use crate::error::DiginetError;
use anyhow::Context;
use log::info;
use url::Url;

pub struct CvBankasListing {
    name: String,
    body: String,
}

struct ScanPageResult {
    next_page_url: Option<Url>,
    listing_urls: Vec<Url>,
}

pub struct CvBankasScraper;

impl CvBankasScraper {
    pub async fn scrape(
        &self,
        page_url: Url,
        limit: Option<usize>,
    ) -> Result<Vec<CvBankasListing>, DiginetError> {
        info!(target: "cvbankas", "Initializing scanning with url {} and limit {:?}", page_url, limit);

        let mut result = Vec::<CvBankasListing>::new();
        let mut page_url = page_url;

        loop {
            let scan_page_result = self.scan_page(page_url.clone()).await?;
            for listing_url in scan_page_result.listing_urls {
                result.push(self.scan_listing(listing_url)?);

                if let Some(limit) = limit {
                    if limit == result.len() {
                        return Ok(result);
                    }
                }
            }

            page_url = match scan_page_result.next_page_url {
                Some(next_page_url) => next_page_url,
                None => break,
            }
        }

        Ok(result)
    }

    // Where page is a page consisting of multiple links to listings
    async fn scan_page(&self, page_url: Url) -> anyhow::Result<ScanPageResult> {
        info!(target: "cvbankas", "Scanning page: {}", page_url);
        let mut next_page_url: Option<Url> = None;
        let mut listing_urls = Vec::<Url>::new();

        let listing_selector =
            scraper::Selector::parse("#js_id_id_job_ad_list > article > a").unwrap();
        let next_page_selector = scraper::Selector::parse(".prev_next").unwrap();
        let body = reqwest::get(page_url.clone())
            .await
            .context("Could not get cvbankas page")?
            .text()
            .await
            .context("Could not get body of cvbankas page")?;
        let document = scraper::Html::parse_document(body.as_str());
        let mut next_page_select = document.select(&next_page_selector);
        while let Some(element) = next_page_select.next() {
            if let Some(url) = element.value().attr("href") {
                info!(target: "cvbankas", "Found next page url {}", url);
                next_page_url = Some(
                    page_url
                        .clone()
                        .join(url)
                        .context("Could not build next page url")?,
                );
            }
        }
        let mut listing_select = document.select(&listing_selector);
        while let Some(element) = listing_select.next() {
            if let Some(url) = element.value().attr("href") {
                info!(target: "cvbankas", "Found listing url {}", url);
                listing_urls.push(
                    page_url
                        .clone()
                        .join(url)
                        .context("Could not build listing url")?,
                );
            }
        }

        Ok(ScanPageResult {
            next_page_url,
            listing_urls,
        })
    }

    fn scan_listing(&self, listing_url: Url) -> Result<CvBankasListing, ()> {
        Ok(CvBankasListing {
            name: "Dummy name".to_string(),
            body: "Dummy body".to_string(),
        })
    }

    pub fn output_listings(&self) {}
}
