use crate::response::ChannelInfo;
use influxdb_rs::{Client, Point, Points, Precision, Value};
use url::Url;
use chrono::prelude::*;

pub struct InfluxdbFormatter {
    pub url: String,
    pub bucket: String,
    pub org: String,
    pub token: String,
}

impl InfluxdbFormatter {
    pub fn new(url: String, bucket: String, org: String, token: String) -> InfluxdbFormatter {
        InfluxdbFormatter {
            url,
            bucket,
            org,
            token,
        }
    }

    pub async fn format(&self, info: &ChannelInfo) -> Result<(), String> {
        let client = Client::new(Url::parse(self.url.as_str()).unwrap(), &self.bucket, &self.org, &self.token).await.unwrap();

        let now = Utc::now();

        // Submit downstream points
        let mut ds_points = Points::create_new(Vec::new());

        for (channel_id, channel) in info.downstream_info.iter() {
            let ds_point = Point::new(format!("{}_ds", channel_id))
            .add_field("freq", Value::Float(channel.freq_mhz as f64))
            .add_field("power", Value::Float(channel.power_dbmv as f64))
            .add_field("snr", Value::Float(channel.snr_db as f64))
            .add_field("no_errs", Value::Integer(channel.no_errs as i64))
            .add_field("corrected_errs", Value::Integer(channel.corrected_errs as i64))
            .add_field("uncorrected_errs", Value::Integer(channel.uncorrected_errs as i64))
            .add_timestamp(now.timestamp());

            ds_points = ds_points.push(ds_point);
        }
        let ds_result = client.write_points(ds_points, Some(Precision::Seconds), None).await;

        if ds_result.is_err()
        {
            return Err(ds_result.unwrap_err().to_string());
        }

        // Submit upstream points
        let mut us_points = Points::create_new(Vec::new());

        for (channel_id, channel) in info.upstream_info.iter() {
            let us_point = Point::new(&format!("{}_us", channel_id))
            .add_field("sym_rate", Value::Float(channel.sym_rate as f64))
            .add_field("freq", Value::Float(channel.freq_mhz as f64))
            .add_field("power", Value::Float(channel.power_dbmv as f64))
            .add_timestamp(now.timestamp());
            us_points = us_points.push(us_point);
        }
        let us_result = client.write_points(us_points, Some(Precision::Seconds), None).await;

        if us_result.is_err()
        {
            return Err(us_result.unwrap_err().to_string());
        }

        let uptime_point = Point::new("cm_system")
        .add_field("uptime", Value::Integer(info.system_uptime as i64));

        let uptime_result = client.write_point(uptime_point, Some(Precision::Seconds), None).await;

        if uptime_result.is_err()
        {
            return Err(uptime_result.unwrap_err().to_string());
        }
        
        Ok(())
    }
}