use crate::response::ChannelInfo;
use influxdb_rs::{Client, Point, Points, Precision, Value};
use url::Url;
use chrono::prelude::*;

pub struct InfluxdbFormatter {
    pub url: String,
    pub bucket: String,
    pub org: String,
    pub token: String,
    pub use_ssl: bool,
}

impl InfluxdbFormatter {
    pub fn new(url: String, bucket: String, org: String, token: String, use_ssl: bool) -> InfluxdbFormatter {
        InfluxdbFormatter {
            url,
            bucket,
            org,
            token,
            use_ssl,
        }
    }

    pub async fn format(&self, info: &ChannelInfo) -> Result<String, String> {
        let client = Client::new(Url::parse(self.url.as_str()).unwrap(), &self.bucket, &self.org, &self.token).await.unwrap();

        let now = Utc::now();

        // Submit downstream points
        let mut ds_points_vec = Vec::new();
        let mut ds_point: Point;

        for (channel_id, channel) in info.downstream_info.iter() {
            ds_point = Point::new(format!("{}_ds", channel_id))
            .add_field("freq", Value::Float(channel.freq_mhz as f64))
            .add_field("power", Value::Float(channel.power_dbmv as f64))
            .add_field("snr", Value::Float(channel.snr_db as f64))
            .add_field("no_errs", Value::Integer(channel.no_errs as i64))
            .add_field("corrected_errs", Value::Integer(channel.corrected_errs as i64))
            .add_field("uncorrected_errs", Value::Integer(channel.uncorrected_errs as i64))
            .add_timestamp(now.timestamp());

            ds_points_vec.push(ds_point);
        }
        let ds_points = Points::create_new(ds_points_vec);
        let ds_result = client.write_points(ds_points, Some(Precision::Seconds), None).await;

        if ds_result.is_err()
        {
            return Err(ds_result.unwrap_err().to_string());
        }

        // Submit upstream points
        let mut us_points_vec = Vec::new();
        let mut us_point: Point;

        for (channel_id, channel) in info.upstream_info.iter() {
            us_point = Point::new(format!("{}_us", channel_id))
            .add_field("sym_rate", Value::Float(channel.sym_rate as f64))
            .add_field("freq", Value::Float(channel.freq_mhz as f64))
            .add_field("power", Value::Float(channel.power_dbmv as f64))
            .add_timestamp(now.timestamp());
            us_points_vec.push(us_point);
        }
        let us_points = Points::create_new(us_points_vec);
        let us_result = client.write_points(us_points, Some(Precision::Seconds), None).await;

        if us_result.is_err()
        {
            return Err(us_result.unwrap_err().to_string());
        }
        
        Ok(String::from(""))
    }
}