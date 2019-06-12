use crate::response::ChannelInfo;

pub trait OutputFormatter {
    fn format(info: &ChannelInfo) -> Result<String, String>;
}
pub struct CricketFormatter {}

// pub struct DownstreamChannelInfo {
//     channel: i32,
//     lock_status: String,
//     modulation: String,
//     channel_id: i32,
//     freq_mhz: f32,
//     power_dbmv: f32,
//     snr_db: f32,
//     corrected_errs: i128,
//     uncorrected_errs: i128,
//     corrected_errs_delta: Option<i128>,
//     uncorrected_errs_delta: Option<i128>,
// }
// pub struct UpstreamChannelInfo {
//     pub channel: i32,
//     pub lock_status: String,
//     pub modulation: String,
//     pub channel_id: i32,
//     pub sym_rate: i32,
//     pub freq_mhz: f32,
//     pub power_dbmv: f32,
// }

impl OutputFormatter for CricketFormatter {
    fn format(info: &ChannelInfo) -> Result<String, String> {
        let output: String;
        let mut iovec: Vec<String> = Vec::new();

        for (channel_id, channel) in info.downstream_info.iter() {
            iovec.push(format!("{}_ds.freq:{}", channel_id, channel.freq_mhz));
            iovec.push(format!(
                "{}_ds.power:{}",
                channel_id, channel.power_dbmv
            ));
            iovec.push(format!("{}_ds.snr:{}", channel_id, channel.snr_db));
            iovec.push(format!(
                "{}_ds.corr_e:{}",
                channel_id, channel.corrected_errs
            ));
            iovec.push(format!(
                "{}_ds.uncorr_e:{}",
                channel_id, channel.uncorrected_errs
            ));

            let locked = match channel.lock_status.as_str() {
                "Locked" => 1,
                _ => 0,
            };
            iovec.push(format!("{}_ds.lock_st:{}", channel_id, locked));

            match channel.uncorrected_errs_delta {
                Some(delta) => iovec.push(format!("{}_ds.uncorr_e_dlt:{}", channel_id, delta)),
                None => (),
            };
            match channel.corrected_errs_delta {
                Some(delta) => {
                    iovec.push(format!("{}_ds.corr_e_dlt:{}", channel_id, delta))
                }
                None => (),
            };
        }

        for (channel_id, channel) in info.upstream_info.iter() {
            iovec.push(format!("{}_us.sym_r:{}", channel_id, channel.sym_rate));
            iovec.push(format!("{}_us.freq:{}", channel_id, channel.freq_mhz));
            iovec.push(format!(
                "{}_us.power:{}",
                channel_id, channel.power_dbmv
            ));

            let locked = match channel.lock_status.as_str() {
                "Locked" => 1,
                _ => 0,
            };
            iovec.push(format!("{}_us.lock_st:{}", channel_id, locked));
        }

        output = iovec.join(" ");

        Ok(output)
    }
}
