use crate::response::*;
use crabquery::Document;

use std::str::FromStr;

fn parse_uptime(text: &str) -> Result<u64, String> {
    let mut days = 0;
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;

    let tokens = text.split(|c: char| c.is_whitespace() || c == ':').collect::<Vec<&str>>();

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            "days" => {
                days = match u64::from_str(tokens[i - 1]) {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Invalid format for days: {}", tokens[i - 1])),
                };
            }
            token if token.ends_with("h") => {
                hours = match u64::from_str(&token[..token.len() - 1]) {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Invalid format for hours: {}", token)),
                };
            }
            token if token.ends_with("m") => {
                minutes = match u64::from_str(&token[..token.len() - 1]) {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Invalid format for minutes: {}", token)),
                };
            }
            token if token.ends_with("s") => {
                seconds = match u64::from_str(&token[..token.len() - 1]) {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Invalid format for seconds: {}", token)),
                };
            }
            _ => {}
        }
        i += 1;
    }

    let total_seconds = days * 86400 + hours * 3600 + minutes * 60 + seconds;
    Ok(total_seconds)
}

impl Parser for ChannelInfoModemModelCGM4331COM {
    fn parse(body: &str) -> Result<ChannelInfo, String> {
        let root = Document::from(body);
        let sections = root.select("tbody");
        let form_rows = root.select("div.form-row");
        let uptime_row = form_rows.iter().filter(|row| {
            row.select("span")[0].text().unwrap_or("".to_string()) == "System Uptime:"
        }).next().unwrap();
        let uptime_value = uptime_row.select("span")[1].text().unwrap();

        let mut channelinfo = ChannelInfo {
            timestamp: std::time::SystemTime::now(),
            system_uptime: parse_uptime(&uptime_value).unwrap_or(0),
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
