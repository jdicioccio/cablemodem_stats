#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MotoStatusStartupSequenceResponse {
    MotoConnDSFreq: String,
    MotoConnDSComment: String,
    MotoConnConnectivityStatus: String,
    MotoConnConnectivityComment: String,
    MotoConnBootStatus: String,
    MotoConnBootComment: String,
    MotoConnConfigurationFileStatus: String,
    MotoConnConfigurationFileComment: String,
    MotoConnSecurityStatus: String,
    MotoConnSecurityComment: String,
    GetMotoStatusStartupSequenceResult: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MotoStatusConnectionInfoResponse {
    MotoConnSystemUpTime: String,
    MotoConnNetworkAccess: String,
    GetMotoStatusConnectionInfoResult: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MotoStatusDownstreamChannelInfoResponse {
    MotoConnDownstreamChannel: String,
    GetMotoStatusDownstreamChannelInfoResult: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MotoStatusUpstreamChannelInfoResponse {
    MotoConnUpstreamChannel: String,
    GetMotoStatusUpstreamChannelInfoResult: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MotoLagStatusResponse {
    MotoLagCurrentStatus: String,
    GetMotoLagStatusResult: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MultipleHNAPsResponse {
    GetMotoStatusStartupSequenceResponse: MotoStatusStartupSequenceResponse,
    GetMotoStatusConnectionInfoResponse: MotoStatusConnectionInfoResponse,
    GetMotoStatusDownstreamChannelInfoResponse: MotoStatusDownstreamChannelInfoResponse,
    GetMotoStatusUpstreamChannelInfoResponse: MotoStatusUpstreamChannelInfoResponse,
    GetMotoLagStatusResponse: MotoLagStatusResponse,
    GetMultipleHNAPsResult: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HNAPsResponse {
    GetMultipleHNAPsResponse: MultipleHNAPsResponse,
}

#[derive(Debug)]
pub struct DownstreamChannelInfo {
    channel: i32,
    lock_status: String,
    modulation: String,
    channel_id: i32,
    freq_mhz: f32,
    power_dbmv: f32,
    snr_db: f32,
    corrected_errs: i128,
    uncorrected_errs: i128,
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
            corrected_errs: 0,
            uncorrected_errs: 0,
        }
    }
}

#[derive(Debug)]
pub struct UpstreamChannelInfo {
    channel: i32,
    lock_status: String,
    modulation: String,
    channel_id: i32,
    sym_rate: i32,
    freq_mhz: f32,
    power_dbmv: f32,
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
    downstream_info: Vec<DownstreamChannelInfo>,
    upstream_info: Vec<UpstreamChannelInfo>,
}

fn parse_downstream_channel_info(data: &HNAPsResponse) -> Result<Vec<DownstreamChannelInfo>, String> {
    let channels_raw = data.GetMultipleHNAPsResponse.GetMotoStatusDownstreamChannelInfoResponse.MotoConnDownstreamChannel.split("|+|");
    let mut channels: Vec<DownstreamChannelInfo> = Vec::new();

    for channel_raw in channels_raw {
        let channel_data: Vec<&str> = channel_raw.split("^").collect();
        let mut channel = DownstreamChannelInfo::new();
        //1^Locked^QAM256^20^495.0^-4.2^40.1^494575281^18816965^
        channel.channel = channel_data[0].parse().unwrap();
        channel.lock_status = channel_data[1].to_string();
        channel.modulation = channel_data[2].to_string();
        channel.channel_id = channel_data[3].parse().unwrap();
        channel.freq_mhz = channel_data[4].parse().unwrap();
        channel.power_dbmv = channel_data[5].parse().unwrap();
        channel.snr_db = channel_data[6].parse().unwrap();
        channel.corrected_errs = channel_data[7].parse().unwrap();
        channel.uncorrected_errs = channel_data[8].parse().unwrap();

        channels.push(channel);
    }

    Ok(channels)
}

fn parse_upstream_channel_info(data: &HNAPsResponse) -> Result<Vec<UpstreamChannelInfo>, String> {
    let channels_raw = data.GetMultipleHNAPsResponse.GetMotoStatusUpstreamChannelInfoResponse.MotoConnUpstreamChannel.split("|+|");
    let mut channels: Vec<UpstreamChannelInfo> = Vec::new();

    for channel_raw in channels_raw {
        let channel_data: Vec<&str> = channel_raw.split("^").collect();
        let mut channel = UpstreamChannelInfo::new();
        //1^Locked^SC-QAM^77^5120^17.3^55.5^
        channel.channel = channel_data[0].parse().unwrap();
        channel.lock_status = channel_data[1].to_string();
        channel.modulation = channel_data[2].to_string();
        channel.channel_id = channel_data[3].parse().unwrap();
        channel.sym_rate = channel_data[4].parse().unwrap();
        channel.freq_mhz = channel_data[5].parse().unwrap();
        channel.power_dbmv = channel_data[6].parse().unwrap();

        channels.push(channel);
    }

    Ok(channels)
}

pub fn parse_channel_info(body: &str) -> Result<ChannelInfo, String> {
    let parsed: HNAPsResponse = serde_json::from_str(body).unwrap();
    let ds_channels = parse_downstream_channel_info(&parsed)?;
    let us_channels = parse_upstream_channel_info(&parsed)?;
    let result = ChannelInfo { downstream_info: ds_channels, upstream_info: us_channels };

    Ok(result)
}