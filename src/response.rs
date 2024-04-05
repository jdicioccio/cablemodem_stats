pub mod cgm4331com_response;
pub mod mb8600_response;

use std::collections::BTreeMap;
use crate::ModemTypes;

#[derive(Debug)]
pub struct DownstreamChannelInfo {
    pub channel: i32,
    pub lock_status: String,
    pub modulation: String,
    pub channel_id: i32,
    pub freq_mhz: f32,
    pub power_dbmv: f32,
    pub snr_db: f32,
    pub no_errs: u32,
    pub corrected_errs: u32,
    pub uncorrected_errs: u32,
}

impl DownstreamChannelInfo {
    pub fn new() -> DownstreamChannelInfo {
        DownstreamChannelInfo {
            channel: 0,
            lock_status: String::from(""),
            modulation: String::from(""),
            channel_id: 0,
            freq_mhz: 0.0,
            power_dbmv: 0.0,
            snr_db: 0.0,
            no_errs: 0,
            corrected_errs: 0,
            uncorrected_errs: 0,
        }
    }
}

#[derive(Debug)]
pub struct UpstreamChannelInfo {
    pub channel: i32,
    pub lock_status: String,
    pub modulation: String,
    pub channel_id: i32,
    pub sym_rate: i32,
    pub freq_mhz: f32,
    pub power_dbmv: f32,
}

impl UpstreamChannelInfo {
    pub fn new() -> UpstreamChannelInfo {
        UpstreamChannelInfo {
            channel: 0,
            lock_status: String::from(""),
            modulation: String::from(""),
            channel_id: 0,
            sym_rate: 0,
            freq_mhz: 0.0,
            power_dbmv: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct ChannelInfo {
    pub timestamp: std::time::SystemTime,
    pub system_uptime: u64,
    pub downstream_info: BTreeMap<i32, DownstreamChannelInfo>,
    pub upstream_info: BTreeMap<i32, UpstreamChannelInfo>,
}

pub trait Parser {
    fn parse(body: &str) -> Result<ChannelInfo, String>;
}

pub fn parse(modem_type: ModemTypes, body: &str) -> Result<ChannelInfo, String> {
    match modem_type {
        ModemTypes::Cgm4331com => ChannelInfoModemModelCGM4331COM::parse(body),
        ModemTypes::Cgm4981com => ChannelInfoModemModelCGM4331COM::parse(body),
        ModemTypes::Mb8600 => ChannelInfoModemModelMB8600::parse(body),
    }
}

pub struct ChannelInfoModemModelMB8600;
pub struct ChannelInfoModemModelCGM4331COM;