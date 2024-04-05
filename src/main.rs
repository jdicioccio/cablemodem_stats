mod fetcher;
mod output;
mod response;

use clap::{ArgEnum, CommandFactory, Parser};
use output::cricket_output::{CricketFormatter, OutputFormatter};
use output::influxdb_output::InfluxdbFormatter;

#[derive(ArgEnum, Debug, Clone)]
pub enum ModemTypes {
    Cgm4331com,
    Cgm4981com,
    Mb8600,
}

#[derive(ArgEnum, Debug, Clone, PartialEq)]
pub enum Output {
    Cricket,
    Influxdbv2,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(arg_enum)]
    modem_type: ModemTypes,

    #[clap(short, long, arg_enum, default_value_t = Output::Cricket)]
    output: Output,

    #[clap(
        short,
        long,
        value_parser,
        value_name = "USERNAME",
        help = "Only used with some cable modems"
    )]
    username: Option<String>,

    #[clap(
        short,
        long,
        value_parser,
        value_name = "PASSWORD",
        help = "Only used with some cable modems"
    )]
    password: Option<String>,

    #[clap(long, value_parser, value_name = "URL", help = "InfluxDB URL")]
    influxdb_url: Option<String>,

    #[clap(long, value_parser, value_name = "BUCKET", help = "InfluxDB bucket")]
    influxdb_bucket: Option<String>,

    #[clap(long, value_parser, value_name = "ORG", help = "InfluxDB organization")]
    influxdb_org: Option<String>,

    #[clap(long, value_parser, value_name = "TOKEN", help = "InfluxDB token")]
    influxdb_token: Option<String>,

    #[clap(short, long, help = "Don't use SSL when connecting to cable modem")]
    no_ssl: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    let modem_type = args.modem_type;
    let mut cmd = Args::command();
    let use_ssl = !args.no_ssl;

    if args.output == Output::Influxdbv2
        && (args.influxdb_url.is_none()
            || args.influxdb_bucket.is_none()
            || args.influxdb_org.is_none()
            || args.influxdb_token.is_none())
    {
        cmd.error(clap::ErrorKind::MissingRequiredArgument,
            "--influxdb-url, --influxdb-bucket, --influxdb-org, and --influxdb-token are required when outputting to InfluxDB")
            .exit();
    }

    let body = match modem_type {
        ModemTypes::Cgm4331com => {
            if args.username.is_none() || args.password.is_none() {
                cmd.error(
                    clap::ErrorKind::MissingRequiredArgument,
                    "Username and password are required for this modem type",
                )
                .exit();
            }
            fetcher::fetch(
                &fetcher::cgm4331com_fetcher::CGM4331COM::new(
                    &args.username.unwrap(),
                    &args.password.unwrap(),
                ),
                use_ssl,
            )?
        }
        ModemTypes::Cgm4981com => {
            if args.username.is_none() || args.password.is_none() {
                cmd.error(
                    clap::ErrorKind::MissingRequiredArgument,
                    "Username and password are required for this modem type",
                )
                .exit();
            }
            fetcher::fetch(
                &fetcher::cgm4331com_fetcher::CGM4331COM::new(
                    &args.username.unwrap(),
                    &args.password.unwrap(),
                ),
                use_ssl,
            )?
        }
        ModemTypes::Mb8600 => {
            if args.username.is_some() || args.password.is_some() {
                cmd.error(
                    clap::ErrorKind::ArgumentConflict,
                    "Username and password are not used for this modem type",
                )
                .print()?;
            }
            fetcher::fetch(&fetcher::mb8600_fetcher::MB8600::new(), use_ssl)?
        }
    };

    let channel_info = response::parse(modem_type, &body)?;

    match args.output {
        Output::Cricket => {
            let output = CricketFormatter::format(&channel_info)?;
            println!("{}", output);
        }
        Output::Influxdbv2 => {
            let formatter = InfluxdbFormatter::new(
                args.influxdb_url.unwrap(),
                args.influxdb_bucket.unwrap(),
                args.influxdb_org.unwrap(),
                args.influxdb_token.unwrap()
            );
            formatter.format(&channel_info).await?;
        }
    }

    Ok(())
}
