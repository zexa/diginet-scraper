# diginet-scraper
Because diginet doesn't expose their API's...

Personally, I needed a bit more control while searching for aruodas listings
and so I decided to build my own scraping, which I'd then use to stuff
said listings onto my own database and filter them however I'd like.

## Usage
```
diginet-scraper-rs 0.1.0
Scrapes diginet.lt (autoplius.lt, aruodas.lt, skelbiu.lt, cvbankas.lt, paslaugos.lt, kainos.lt) listings

USAGE:
    diginet-scraper-rs [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    aruodas    Allows to scan aruodas listings
    help       Prints this message or the help of the given subcommand(s)
```

## Usage aruodas
1. Go to aruodas.lt
2. Use their search to filter the listings
3. Copy {url}
4. Run `digine-scraper aruodas {url}`

```
diginet-scraper aruodas https://www.aruodas.lt/butu-nuoma/vilniuje/\?FPriceMin\=200\&FPriceMax\=250
``` 