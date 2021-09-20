use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::PotentialListing;
use std::sync::{Arc, Mutex};

pub struct PotentialListingStream<'a> {
    potential_listing_mutex: Arc<Mutex<Vec::<PotentialListing<'a>>>>,
}

impl<'a> PotentialListingStream<'a> {
    pub fn new(potential_listing_mutex: Arc<Mutex<Vec::<PotentialListing<'a>>>>) -> Self {
        Self {
            potential_listing_mutex,
        }
    }
}

impl<'a> Stream for PotentialListingStream<'a> {
    type Item= PotentialListing<'a>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
