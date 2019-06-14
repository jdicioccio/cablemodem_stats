#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct DownstreamChannelInfo {
    pub channel: i32,
    pub lock_status: String,
    pub modulation: String,
    pub channel_id: i32,
    pub freq_mhz: f32,
    pub power_dbmv: f32,
    pub snr_db: f32,
    pub corrected_errs: i128,
    pub uncorrected_errs: i128,
    pub corrected_errs_delta: Option<i128>,
    pub uncorrected_errs_delta: Option<i128>,
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
            corrected_errs_delta: None,
            uncorrected_errs_delta: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelInfo {
    pub timestamp: std::time::SystemTime,
    pub downstream_info: BTreeMap<i32, DownstreamChannelInfo>,
    pub upstream_info: BTreeMap<i32, UpstreamChannelInfo>,
}

impl ChannelInfo {
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
            let channel_data: Vec<&str> = channel_raw.split("^").collect();
            let mut channel = DownstreamChannelInfo::new();
            //1^Locked^QAM256^20^495.0^-4.2^40.1^494575281^18816965^
            channel.channel = channel_data[0].parse().unwrap();
            channel.lock_status = channel_data[1].to_string();
            channel.modulation = channel_data[2].to_string();
            channel.channel_id = channel_data[3].parse().unwrap();
            channel.freq_mhz = channel_data[4].trim().parse().unwrap();
            channel.power_dbmv = channel_data[5].trim().parse().unwrap();
            channel.snr_db = channel_data[6].trim().parse().unwrap();
            channel.corrected_errs = channel_data[7].trim().parse().unwrap();
            channel.uncorrected_errs = channel_data[8].trim().parse().unwrap();

            channels.insert(channel.channel_id, channel);
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
            let channel_data: Vec<&str> = channel_raw.split("^").collect();
            let mut channel = UpstreamChannelInfo::new();
            //1^Locked^SC-QAM^77^5120^17.3^55.5^
            channel.channel = channel_data[0].parse().unwrap();
            channel.lock_status = channel_data[1].to_string();
            channel.modulation = channel_data[2].to_string();
            channel.channel_id = channel_data[3].parse().unwrap();
            channel.sym_rate = channel_data[4].parse().unwrap();
            channel.freq_mhz = channel_data[5].parse().unwrap();
            channel.power_dbmv = channel_data[6].trim().parse().unwrap();

            channels.insert(channel.channel_id, channel);
        }

        Ok(channels)
    }

    fn save_channel_info(&self) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("cm_state.json")?;

        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;

        Ok(())
    }

    fn load_deltas(&mut self) -> Result<(), std::io::Error> {
        let result: ChannelInfo;
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .open("cm_state.json")?;

        let reader = std::io::BufReader::new(file);
        result = serde_json::from_reader(reader).unwrap();

        for (channel_id, channel) in self.downstream_info.iter_mut() {
            channel.corrected_errs_delta =
                Some(channel.corrected_errs - result.downstream_info[&channel_id].corrected_errs);
            channel.uncorrected_errs_delta = Some(
                channel.uncorrected_errs - result.downstream_info[&channel_id].uncorrected_errs,
            );
        }

        Ok(())
    }

    pub fn from(body: &str) -> Result<ChannelInfo, String> {
        let parsed: HNAPsResponse = serde_json::from_str(body).unwrap();
        let ds_channels = ChannelInfo::parse_downstream_channel_info(&parsed)?;
        let us_channels = ChannelInfo::parse_upstream_channel_info(&parsed)?;
        let mut result = ChannelInfo {
            timestamp: std::time::SystemTime::now(),
            downstream_info: ds_channels,
            upstream_info: us_channels,
        };

        match result.load_deltas() {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        };

        result.save_channel_info().unwrap();

        // println!(
        //     "New is {:?} seconds newer than old",
        //     result
        //         .timestamp
        //         .duration_since(std::time::UNIX_EPOCH)
        //         .unwrap()
        //         - old_result
        //             .timestamp
        //             .duration_since(std::time::UNIX_EPOCH)
        //             .unwrap()
        // );

        Ok(result)
    }
}
