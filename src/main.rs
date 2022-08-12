mod fetcher;
mod output;
mod response;

use clap::{ArgEnum, CommandFactory, Parser};
use output::cricket_output::{CricketFormatter, OutputFormatter};
use output::influxdb_output::InfluxdbFormatter;

#[derive(ArgEnum, Debug, Clone)]
pub enum ModemTypes {
    Cgm4331com,
    Mb8600,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(arg_enum)]
    modem_type: ModemTypes,

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

    #[clap(short, long, help = "Don't use HTTPS")]
    no_ssl: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    let modem_type = args.modem_type;
    let mut cmd = Args::command();
    let use_ssl = !args.no_ssl;

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
            )
            .unwrap()
        }
        ModemTypes::Mb8600 => {
            if args.username.is_some() || args.password.is_some() {
                cmd.error(
                    clap::ErrorKind::ArgumentConflict,
                    "Username and password are not used for this modem type",
                )
                .print()?;
            }
            fetcher::fetch(&fetcher::mb8600_fetcher::MB8600::new(), use_ssl).unwrap()
        }
    };

    let channel_info = response::parse(modem_type, &body)?;

    // let output = CricketFormatter::format(&channel_info).unwrap();
    let influx = InfluxdbFormatter::new(
        "http://pine64.int.ods.org:8086".to_string(),
        "cm_stats".to_string(),
        "home".to_string(),
        "MUrewE1sAd3Ncqa9M2rEoRwuLM5PT6yX8zZb5WGBxAhB6EY3pCsDiY2LaaDFWOgDHBd2SBzN_ySCSIlT5S9jNw==".to_string(),
        false,
    );
    let output = influx.format(&channel_info).await.unwrap();
    println!("{}", output);

    Ok(())
}
