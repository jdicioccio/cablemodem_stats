use crate::response::ChannelInfo;

pub trait OutputFormatter {
    fn format(info: &ChannelInfo) -> Result<String, String>;
}
pub struct CricketFormatter {}

impl OutputFormatter for CricketFormatter {
    fn format(info: &ChannelInfo) -> Result<String, String> {
        let output: String;
        let mut iovec: Vec<String> = Vec::new();

        iovec.push(format!("system_uptime:{}", info.system_uptime));
        for (channel_id, channel) in info.downstream_info.iter() {
            iovec.push(format!("{}_ds.freq:{}", channel_id, channel.freq_mhz));
            iovec.push(format!(
                "{}_ds.power:{}",
                channel_id, channel.power_dbmv
            ));
            iovec.push(format!("{}_ds.snr:{}", channel_id, channel.snr_db));
            iovec.push(format!(
                "{}_ds.no_errs:{}",
                channel_id, channel.no_errs
            ));
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
