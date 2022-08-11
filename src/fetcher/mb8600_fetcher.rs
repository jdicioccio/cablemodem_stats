use crate::fetcher::Fetcher;
use isahc::prelude::*;
use super::make_request_builder;

pub struct MB8600 {}

impl MB8600 {
    pub fn new() -> MB8600 {
        MB8600 {}
    }
}

impl Fetcher for MB8600 {
    fn fetch(&self, use_ssl: bool) -> Result<String, isahc::Error> {
        let req_builder = make_request_builder("192.168.100.1/HNAP1/", use_ssl);

        let req = req_builder
        .header("Content-Type", "application/json")
        .header("SOAPACTION", r#""http://purenetworks.com/HNAP1/GetMultipleHNAPs""#)
        .header("Expect", "")
        // .header("Cookie", format!("uid={}; PrivateKey={}", uid, private_key))
        // .header("HNAP_AUTH", "XXXX 1559973888764")
        .body(r#"{"GetMultipleHNAPs":{"GetMotoStatusStartupSequence":"","GetMotoStatusConnectionInfo":"","GetMotoStatusDownstreamChannelInfo":"","GetMotoStatusUpstreamChannelInfo":"","GetMotoLagStatus":""}}"#)?;

        let mut resp = req.send()?;

        let body = resp.text()?;

        Ok(body)
    }
}
