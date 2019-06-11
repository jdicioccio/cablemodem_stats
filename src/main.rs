mod output_formatter;
mod response;
use chttp::http;
use output_formatter::*;
use response::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let req = http::Request::post("http://192.168.100.1/HNAP1/")
        .header("Content-Type", "application/json")
//        .header("Cookie", format!("uid={}; PrivateKey={}", uid, private_key))
//        .header("HNAP_AUTH", "XXXX 1559973888764")
        .header("SOAPACTION", r#""http://purenetworks.com/HNAP1/GetMultipleHNAPs""#)
        .header("Expect", "")
        .body(r#"{"GetMultipleHNAPs":{"GetMotoStatusStartupSequence":"","GetMotoStatusConnectionInfo":"","GetMotoStatusDownstreamChannelInfo":"","GetMotoStatusUpstreamChannelInfo":"","GetMotoLagStatus":""}}"#)?;

    let mut resp = chttp::send(req)?;

    //assert!(resp.status().is_success());

    let body = resp.body_mut().text()?;
    let channel_info = ChannelInfo::from(&body)?;

    let output = output_formatter::CricketFormatter::format(&channel_info).unwrap();
    println!("{}", output);

    Ok(())
}
