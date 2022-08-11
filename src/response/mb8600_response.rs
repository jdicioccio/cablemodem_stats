#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use regex::Regex;
use crate::response::*;

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

impl ChannelInfoModemModelMB8600 {
    fn neg_i32_to_u32(i: i32) -> Result<u32, String> {
        let u: u32;
        if i.is_negative() {
            u = u32::from_ne_bytes(i.to_ne_bytes());
        } else {
            u = i as u32;
        }
        Ok(u)
    }

    fn parse_downstream_channel_info(
        data: &HNAPsResponse,
    ) -> Result<BTreeMap<i32, DownstreamChannelInfo>, String> {
        let channels_raw = data
            .GetMultipleHNAPsResponse
            .GetMotoStatusDownstreamChannelInfoResponse
            .MotoConnDownstreamChannel
            .split("|+|");
        let mut channels: BTreeMap<i32, DownstreamChannelInfo> = BTreeMap::new();

        for channel_raw in channels_raw {
            let channel_data: Vec<&str> = channel_raw.split('^').collect();
            let mut channel = DownstreamChannelInfo::new();
            //1^Locked^QAM256^20^495.0^-4.2^40.1^494575281^18816965^
            channel.channel = channel_data[0].parse().unwrap();
            channel.lock_status = channel_data[1].to_string();
            channel.modulation = channel_data[2].to_string();
            channel.channel_id = channel_data[3].parse().unwrap();
            channel.freq_mhz = channel_data[4].trim().parse().unwrap();
            channel.power_dbmv = channel_data[5].trim().parse().unwrap();
            channel.snr_db = channel_data[6].trim().parse().unwrap();
            channel.corrected_errs = ChannelInfoModemModelMB8600::neg_i32_to_u32(channel_data[7].trim().parse::<i32>().unwrap()).unwrap();
            channel.uncorrected_errs = ChannelInfoModemModelMB8600::neg_i32_to_u32(channel_data[8].trim().parse::<i32>().unwrap()).unwrap();

            channels.insert(channel.channel, channel);
        }

        Ok(channels)
    }

    fn parse_upstream_channel_info(
        data: &HNAPsResponse,
    ) -> Result<BTreeMap<i32, UpstreamChannelInfo>, String> {
        let channels_raw = data
            .GetMultipleHNAPsResponse
            .GetMotoStatusUpstreamChannelInfoResponse
            .MotoConnUpstreamChannel
            .split("|+|");
        let mut channels: BTreeMap<i32, UpstreamChannelInfo> = BTreeMap::new();

        for channel_raw in channels_raw {
            let channel_data: Vec<&str> = channel_raw.split('^').collect();
            let mut channel = UpstreamChannelInfo::new();
            //1^Locked^SC-QAM^77^5120^17.3^55.5^
            channel.channel = channel_data[0].parse().unwrap();
            channel.lock_status = channel_data[1].to_string();
            channel.modulation = channel_data[2].to_string();
            channel.channel_id = channel_data[3].parse().unwrap();
            channel.sym_rate = channel_data[4].parse().unwrap();
            channel.freq_mhz = channel_data[5].parse().unwrap();
            channel.power_dbmv = channel_data[6].trim().parse().unwrap();

            channels.insert(channel.channel, channel);
        }

        Ok(channels)
    }

    fn uptime_to_seconds(uptime_str: &str) -> Result<u64, String> {
        let mut uptime: u64 = 0;
        let re = Regex::new(r"^(\d+) days (\d+)h:(\d+)m:(\d+)s").unwrap();
        for cap in re.captures_iter(uptime_str) {
            let days = &cap[1].parse::<u64>().unwrap();
            let hours = &cap[2].parse::<u64>().unwrap();
            let minutes = &cap[3].parse::<u64>().unwrap();
            let seconds = &cap[4].parse::<u64>().unwrap();

            uptime = (days * 86400) + (hours * 3600) + (minutes * 60) + seconds;
        }

        Ok(uptime)
    }
}

impl Parser for ChannelInfoModemModelMB8600 {
    fn parse(body: &str) -> Result<ChannelInfo, String> {
        let parsed: HNAPsResponse = serde_json::from_str(body).unwrap();
        let ds_channels = ChannelInfoModemModelMB8600::parse_downstream_channel_info(&parsed)?;
        let us_channels = ChannelInfoModemModelMB8600::parse_upstream_channel_info(&parsed)?;

        let result = ChannelInfo {
            timestamp: std::time::SystemTime::now(),
            system_uptime: ChannelInfoModemModelMB8600::uptime_to_seconds(&parsed.GetMultipleHNAPsResponse.GetMotoStatusConnectionInfoResponse.MotoConnSystemUpTime)?,
            downstream_info: ds_channels,
            upstream_info: us_channels,
        };

        Ok(result)
    }
}
