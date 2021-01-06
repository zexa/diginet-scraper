extern crate clap;
extern crate reqwest;
extern crate tokio;
extern crate url;
extern crate scraper;

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
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(aruodas_matches) = matches.subcommand_matches("aruodas") {
        // I.e. https://www.aruodas.lt/butu-nuoma/vilniuje/?FPriceMin=200&FPriceMax=250
        let url = aruodas_matches.value_of("url").unwrap().to_string();
        println!("Initial page provided: {}", url);

        let scraper = crate::aruodas::Scraper::new();

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

    Ok(())
}
