# diginet-lt-scraper
Is a collection of scrapers for services provided by the company that owns autoplius.lt, aruodas.lt, skelbiu.lt, 
cvbankas.lt, paslaugos.lt, kainos.lt - diginet.lt.

I wrote diginet-lt-scraper because:
* I wanted to practice writing rust;
* Diginet services do not expose their api's;
* I wanted to write my own highly responsive notifications to discord;
* I wanted more control over the sorting provided by aruodas.lt;
* I wanted more control over the filtering provided by skelbiu.lt;
* I wanted to write open source code that others would find useful.

## Disclaimer
* diginet-lt-scraper is in its early development stages. Breaking API changes are to be expected.
* Not all diginet services are currently available for scraping. We are still working on a proper abstraction. 
* versioning is currently not a concert - if you want to use the scraper - fork it or use a specific commit hash.

## Build
```
cargo build --release
```

## Usage
```
$ ./target/release/diginet-lt-scraper
diginet-scraper 0.1.1
Scrapes diginet.lt (autoplius.lt, aruodas.lt, skelbiu.lt, cvbankas.lt, paslaugos.lt, kainos.lt) listings

USAGE:
    diginet-scraper [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    aruodas     Scrapes aruodas listings
    cvbankas    Scrapes cvbankas listings
    help        Prints this message or the help of the given subcommand(s)
```
