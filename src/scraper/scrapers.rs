pub mod indeed;
use super::{Company, Job};

pub trait Scrape {
    fn build_url(&self) -> Url;
    fn scrape(&self) -> (Company, Job);
}

pub enum Scrapers {
    Indeed,
}
