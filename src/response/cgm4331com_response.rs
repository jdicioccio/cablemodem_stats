use crate::response::*;
use crabquery::Document;

impl Parser for ChannelInfoModemModelCGM4331COM {
    fn parse(body: &str) -> Result<ChannelInfo, String> {
        let root = Document::from(body);
        let sections = root.select("tbody");
        let mut channelinfo = ChannelInfo {
            timestamp: std::time::SystemTime::now(),
            system_uptime: 0,
            downstream_info: BTreeMap::new(),
            upstream_info: BTreeMap::new(),
        };

        enum OutputSections {
            Downstream = 0,
            Upstream = 1,
            Errors = 2,
        }

        for (section_num, section) in sections.iter().enumerate() {
            let rows = section.select("tr");
            for row in rows {
                let name = row.select("th")[0].text().unwrap().trim().to_string();
                let values = row.select("td > div");

                for (i, value) in values.iter().enumerate() {
                    let value_str = value.text().unwrap().trim().to_string();
                    let mut channel_id: i32 = i.try_into().unwrap();
                    channel_id += 1;

                    if section_num == OutputSections::Downstream as usize {
                        if !channelinfo.downstream_info.contains_key(&channel_id) {
                            channelinfo
                                .downstream_info
                                .insert(channel_id, DownstreamChannelInfo::new());
                        }
                        match name.as_str() {
                            "Index" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .channel_id = value_str.parse::<i32>().unwrap();
                            }
                            "Lock Status" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .lock_status = value_str;
                            }
                            "Frequency" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .freq_mhz =
                                    value_str.split(' ').nth(0).unwrap().parse::<f32>().unwrap();
                                if !value_str.contains("MHz") {
                                    channelinfo
                                        .downstream_info
                                        .get_mut(&channel_id)
                                        .unwrap()
                                        .freq_mhz /= 1_000_000.0;
                                }
                            }
                            "SNR" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .snr_db =
                                    value_str.split(' ').nth(0).unwrap().parse::<f32>().unwrap();
                            }
                            "Power Level" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .power_dbmv =
                                    value_str.split(' ').nth(0).unwrap().parse::<f32>().unwrap();
                            }
                            "Modulation" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .modulation = value_str;
                            }
                            _ => {}
                        }
                    } else if section_num == OutputSections::Upstream as usize {
                        if !channelinfo.upstream_info.contains_key(&channel_id) {
                            channelinfo
                                .upstream_info
                                .insert(channel_id, UpstreamChannelInfo::new());
                        }
                        match name.as_str() {
                            "Index" => {
                                channelinfo
                                    .upstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .channel_id = value_str.parse::<i32>().unwrap();
                            }
                            "Lock Status" => {
                                channelinfo
                                    .upstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .lock_status = value_str;
                            }
                            "Frequency" => {
                                channelinfo
                                    .upstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .freq_mhz =
                                    value_str.split(' ').nth(0).unwrap().parse::<f32>().unwrap();
                                if !value_str.contains("MHz") {
                                    channelinfo
                                        .upstream_info
                                        .get_mut(&channel_id)
                                        .unwrap()
                                        .freq_mhz /= 1_000_000.0;
                                }
                            }
                            "Symbol Rate" => {
                                channelinfo
                                    .upstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .sym_rate = value_str.parse::<i32>().unwrap();
                            }
                            "Power Level" => {
                                channelinfo
                                    .upstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .power_dbmv =
                                    value_str.split(' ').nth(0).unwrap().parse::<f32>().unwrap();
                            }
                            "Modulation" => {
                                channelinfo
                                    .upstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .modulation = value_str;
                            }
                            _ => {}
                        }
                    } else if section_num == OutputSections::Errors as usize {
                        match name.as_str() {
                            "Unerrored Codewords" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .no_errs = value_str.parse::<u32>().unwrap();
                            }
                            "Correctable Codewords" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .corrected_errs = value_str.parse::<u32>().unwrap();
                            }
                            "Uncorrectable Codewords" => {
                                channelinfo
                                    .downstream_info
                                    .get_mut(&channel_id)
                                    .unwrap()
                                    .uncorrected_errs = value_str.parse::<u32>().unwrap();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(channelinfo)
    }
}
