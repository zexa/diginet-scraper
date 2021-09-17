extern crate clap;
extern crate reqwest;
extern crate scraper;
extern crate tokio;
extern crate url;
#[macro_use]
extern crate async_stream;
#[macro_use]
extern crate log;

mod aruodas;
mod cvbankas;
mod error;

use crate::error::DiginetError;
use clap::{App, AppSettings, Arg};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::str::FromStr;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), DiginetError> {
    let version = env!("CARGO_PKG_VERSION");
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about("Scrapes diginet.lt (autoplius.lt, aruodas.lt, skelbiu.lt, cvbankas.lt, paslaugos.lt, kainos.lt) listings")
        .version(version)
        .subcommand(App::new("aruodas")
            .about("Scrapes aruodas listings")
            .arg(Arg::with_name("url")
                .takes_value(true))
            .arg(Arg::with_name("sort_by_ratio")
                .short("sbr")
                .long("sort-by-ratio")
                .help("Sorts by price per area ratio")
                .required(false)
                .takes_value(false))
            .arg(Arg::with_name("limit")
                .short("l")
                .long("limit")
                .help("Limits the amount of listings the scraper will scan for")
                .takes_value(true)
                .required(false))
            .setting(AppSettings::ArgRequiredElseHelp))
        .subcommand(App::new("cvbankas")
            .about("Scrapes cvbankas listings")
            .arg(Arg::with_name("url")
                .takes_value(true))
            .arg(Arg::with_name("limit")
                .short("l")
                .long("limit")
                .help("Limits the amount of listings the scraper will scan for")
                .takes_value(true)
                .required(false))
            .setting(AppSettings::ArgRequiredElseHelp))
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    SimpleLogger::new().with_level(LevelFilter::Info).init()?;

    if let Some(aruodas_matches) = matches.subcommand_matches("aruodas") {
        // I.e. https://www.aruodas.lt/butu-nuoma/vilniuje/?FPriceMin=200&FPriceMax=250
        let url = aruodas_matches.value_of("url").unwrap().to_string();
        println!("Initial page provided: {}", url);
        let scraper = crate::aruodas::AruodasScraper::new();
        let limit: Option<usize> = match aruodas_matches.value_of("limit") {
            None => None,
            Some(lim) => Some(lim.parse::<usize>().unwrap()),
        };
        println!("Order limit specified: {:?}", limit);
        let mut listings = scraper.scrape(url, limit).await;
        if aruodas_matches.is_present("sort_by_ratio") {
            println!("Sorting by price per area");
            listings = crate::aruodas::sort_by_price_per_area(listings);
        };
        for listing in listings {
            println!("{:?}", listing)
        }
    }

    if let Some(cvbankas_matches) = matches.subcommand_matches("cvbankas") {
        let url = Url::from_str(cvbankas_matches.value_of("url").ok_or(DiginetError{})?)?;
        println!("Initial page provided: {}", url);
        let limit: Option<usize> = match cvbankas_matches.value_of("limit") {
            None => None,
            Some(lim) => Some(lim.parse::<usize>().unwrap()),
        };
        let scraper = cvbankas::CvBankasScraper {};
        let result = scraper.scrape(url, limit).await?;
        scraper.output_listings(result);
    }

    Ok(())
}
