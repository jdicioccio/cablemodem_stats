pub mod cgm4331com_fetcher;
pub mod mb8600_fetcher;

pub trait Fetcher {
    fn fetch(&self) -> Result<String, isahc::Error>;
}

pub fn fetch<T: Fetcher>(t: &T) -> Result<String, isahc::Error> {
    t.fetch()
}