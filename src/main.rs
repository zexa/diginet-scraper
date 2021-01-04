extern crate clap;
extern crate reqwest;
extern crate tokio;

mod aruodas;

use clap::{Arg, App, AppSettings};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let version = env!("CARGO_PKG_VERSION");
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about("Scrapes diginet.lt (autoplius.lt, aruodas.lt, skelbiu.lt, cvbankas.lt, paslaugos.lt, kainos.lt) listings")
        .version(version)
        .subcommand(App::new("aruodas")
            .version(version)
            .about("Scrapes aruodas listings")
            .arg(Arg::with_name("url")
                .takes_value(true))
            .setting(AppSettings::ArgRequiredElseHelp))
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("aruodas") {
        // I.e. https://www.aruodas.lt/butu-nuoma/vilniuje/?FPriceMin=200&FPriceMax=250
        let url = matches.value_of("url").unwrap().to_string();
        let scraper = crate::aruodas::Scraper::new();
        for listing in scraper.scrape(url).await {
            println!("{:?}", listing)
        }
    }

    Ok(())
}

