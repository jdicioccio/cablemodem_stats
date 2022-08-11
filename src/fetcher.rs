pub mod cgm4331com_fetcher;
pub mod mb8600_fetcher;
use isahc::config::SslOption;
use isahc::prelude::*;
use isahc::Request;

pub trait Fetcher {
    fn fetch(&self, use_ssl: bool) -> Result<String, isahc::Error>;
}

pub fn fetch<T: Fetcher>(t: &T, use_ssl: bool) -> Result<String, isahc::Error> {
    t.fetch(use_ssl)
}

pub fn make_request_builder(url: &str, use_ssl: bool) -> isahc::http::request::Builder {
    if use_ssl {
        Request::post(format!("https://{}", url)).ssl_options(
            SslOption::DANGER_ACCEPT_INVALID_CERTS | SslOption::DANGER_ACCEPT_INVALID_HOSTS,
        )
    } else {
        Request::post(format!("http://{}", url))
    }
}