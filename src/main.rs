extern crate clap;
extern crate reqwest;

#[macro_use]
extern crate tokio;

mod aruodas;

use clap::{Arg, App, AppSettings};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Skelbiu-scraper-rs")
        .subcommand(App::new("aruodas")
            .arg(Arg::with_name("url")
                .takes_value(true))
            .setting(AppSettings::ArgRequiredElseHelp))
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("aruodas") {
        // I.e. https://www.aruodas.lt/butu-nuoma/vilniuje/?FPriceMin=200&FPriceMax=250
        let url = matches.value_of("url").unwrap().to_string();
        let scraper = crate::aruodas::Scraper::new();
        scraper.scrape(url).await;
    }

    Ok(())
}

