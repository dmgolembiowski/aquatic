use std::sync::{atomic::AtomicUsize, Arc};

use rand_distr::Gamma;

pub use aquatic_ws_protocol::*;

#[derive(Default)]
pub struct Statistics {
    pub requests: AtomicUsize,
    pub response_peers: AtomicUsize,
    pub responses_announce: AtomicUsize,
    pub responses_offer: AtomicUsize,
    pub responses_answer: AtomicUsize,
    pub responses_scrape: AtomicUsize,
    pub responses_error: AtomicUsize,
}

#[derive(Clone)]
pub struct LoadTestState {
    pub info_hashes: Arc<Vec<InfoHash>>,
    pub statistics: Arc<Statistics>,
    pub gamma: Arc<Gamma<f64>>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RequestType {
    Announce,
    Scrape,
}
