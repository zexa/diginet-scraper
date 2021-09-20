use crate::Listing;
use std::sync::{Arc, Mutex};

pub struct ListingStream<L> where L: Listing {
    mutex: Arc<Mutex<Vec<L>>>
}

impl<L> ListingStream<L> where L: Listing {
    pub(crate) fn new(mutex: Arc<Mutex<Vec<L>>>) -> Self {
        Self {
            mutex,
        }
    }
}
