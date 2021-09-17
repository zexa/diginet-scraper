use crate::error::DiginetError;
use anyhow::Context;
use log::info;
use scraper::ElementRef;
use url::Url;

#[derive(Debug)]
pub struct CvBankasListing {
    url: Url,
    name: String,
    description: String,
    compensation: CompensationRange,
}

struct ScanPageResult {
    next_page_url: Option<Url>,
    listing_urls: Vec<Url>,
}

#[derive(Debug)]
pub struct CompensationRange {
    from: String,
    to: String,
    compensation_type: CompensationType,
}

#[derive(Debug)]
pub enum CompensationType {
    Gross,
    Net,
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
                result.push(self.scan_listing(listing_url).await?);

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
                let next_url = page_url
                    .clone()
                    .join(url)
                    .context("Could not build next page url")?;

                if !self.is_next_page(page_url.clone(), next_url.clone())? {
                    continue;
                }

                info!(target: "cvbankas", "Found next page url {}", url);
                next_page_url = Some(next_url);
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
            };
        }

        Ok(ScanPageResult {
            next_page_url,
            listing_urls,
        })
    }

    fn get_page(&self, url: Url) -> anyhow::Result<Option<usize>> {
        match url.query_pairs().find(|pair| pair.0 == "page") {
            None => Ok(None),
            Some(pair) => Ok(Some(pair.1.parse::<usize>()?)),
        }
    }

    pub fn is_next_page(&self, current_url: Url, next_url: Url) -> anyhow::Result<bool> {
        let curr_page = self
            .get_page(current_url)
            .context("Could not parse curr page into a number")?;
        let next_page = self
            .get_page(next_url)
            .context("Could not parse next page into a number")?;

        match curr_page {
            None => match next_page {
                None => Ok(false),
                Some(_) => Ok(true),
            },
            Some(curr_page_nr) => match next_page {
                None => Ok(false),
                Some(next_page_nr) => Ok(next_page_nr > curr_page_nr),
            },
        }
    }

    async fn scan_listing(&self, listing_url: Url) -> Result<CvBankasListing, DiginetError> {
        info!(target: "cvbankas", "Scanning listing: {}", listing_url);
        let description_selector =
            scraper::Selector::parse("section[itemprop=\"description\"]").unwrap();
        // ".salary_amount" separate by "-". Left is from, right is To.
        let compensation_range_selector = scraper::Selector::parse(".salary_amount").unwrap();
        // ".salary_calculation" options are "Į rankas" and "Gross"
        let compensation_type_selector = scraper::Selector::parse(".salary_calculation").unwrap();
        let name_selector = scraper::Selector::parse("#jobad_heading1").unwrap();
        let body = reqwest::get(listing_url.clone())
            .await
            .context("Could not get cvbankas listing")?
            .text()
            .await
            .context("Could not get body of cvbankas listing")?;
        let document = scraper::Html::parse_document(body.as_str());
        let description = document
            .select(&description_selector)
            .next()
            .ok_or(DiginetError{})?
            .text()
            .collect();
        let name = document
            .select(&name_selector)
            .next()
            .ok_or(DiginetError{})?
            .text()
            .next()
            .ok_or(DiginetError{})?
            .to_string();
        let mut from = "0".to_string();
        let mut to = "0".to_string();
        let compensation_range = document
            .select(&compensation_range_selector)
            .next()
            .ok_or(DiginetError{})?
            .text()
            .next()
            .ok_or(DiginetError{})?;
        if compensation_range.contains('-') {
            let mut compensation_range = compensation_range.split('-');
            from = compensation_range.next().ok_or(DiginetError{})?.to_string();
            to = compensation_range.next().ok_or(DiginetError{})?.to_string();
        } else if compensation_range.contains("Up to ") {
            to = compensation_range.split("Up to ").next().ok_or(DiginetError{})?.to_string();
        }
        let compensation_type = match document
            .select(&compensation_type_selector)
            .next()
            .ok_or(DiginetError{})?
            .text()
            .next()
            .ok_or(DiginetError{})?
        {
            "Į rankas" => CompensationType::Net,
            "Net" => CompensationType::Net,
            "Gross" => CompensationType::Gross,
            "Neatskaičius mokesčių" => CompensationType::Gross,
            _ => return Err(DiginetError {}),
        };

        Ok(CvBankasListing {
            url: listing_url,
            name,
            description,
            compensation: CompensationRange {
                from,
                to,
                compensation_type,
            },
        })
    }

    pub fn output_listings(&self, listings: Vec<CvBankasListing>) {
        for listing in listings {
            println!("{:?}", listing);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_next_url_when_curr_page_no_query() {
        assert!(CvBankasScraper {}
            .is_next_page(
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76").unwrap(),
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76&page=2")
                    .unwrap()
            )
            .unwrap());
    }

    #[test]
    fn is_next_url_when_next_page_higher() {
        assert!(CvBankasScraper {}
            .is_next_page(
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76&page=2")
                    .unwrap(),
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76&page=3")
                    .unwrap()
            )
            .unwrap());
    }

    #[test]
    fn is_next_url_when_curr_page_higher() {
        assert!(!CvBankasScraper {}
            .is_next_page(
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76&page=3")
                    .unwrap(),
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76&page=2")
                    .unwrap()
            )
            .unwrap());
    }

    #[test]
    fn is_next_url_when_next_page_no_query() {
        assert!(!CvBankasScraper {}
            .is_next_page(
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76&page=2")
                    .unwrap(),
                Url::parse("https://www.cvbankas.lt/?miestas=Vilnius&padalinys%5B0%5D=76").unwrap()
            )
            .unwrap());
    }
}
