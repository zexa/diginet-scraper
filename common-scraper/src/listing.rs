use url::Url;

pub trait Listing {
    fn get_url(&self) -> &Url;
}
