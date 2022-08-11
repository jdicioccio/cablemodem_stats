use isahc::prelude::*;
use isahc::Request;
use isahc::config::SslOption;
use crate::fetcher::Fetcher;

pub struct MB8600 {}

impl MB8600 {
    pub fn new() -> MB8600 {
        MB8600 {}
    }
}

impl Fetcher for MB8600 {
    fn fetch(&self) -> Result<String, isahc::Error> {
        let req = Request::post("https://192.168.100.1/HNAP1/")
        .ssl_options(SslOption::DANGER_ACCEPT_INVALID_CERTS|SslOption::DANGER_ACCEPT_INVALID_HOSTS)
        .header("Content-Type", "application/json")
//        .header("Cookie", format!("uid={}; PrivateKey={}", uid, private_key))
//        .header("HNAP_AUTH", "XXXX 1559973888764")
        .header("SOAPACTION", r#""http://purenetworks.com/HNAP1/GetMultipleHNAPs""#)
        .header("Expect", "")
        .body(r#"{"GetMultipleHNAPs":{"GetMotoStatusStartupSequence":"","GetMotoStatusConnectionInfo":"","GetMotoStatusDownstreamChannelInfo":"","GetMotoStatusUpstreamChannelInfo":"","GetMotoLagStatus":""}}"#)?;

        let mut resp = req.send()?;

        let body = resp.text()?;

        Ok(body)
    }
}