mod fetcher;
mod output_formatter;
mod response;

use clap::{ArgEnum, CommandFactory, Parser};
use output_formatter::*;

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

    #[clap(short, long, value_parser, value_name = "USERNAME")]
    username: Option<String>,

    #[clap(short, long, value_parser, value_name = "PASSWORD")]
    password: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    let modem_type = args.modem_type;
    let mut cmd = Args::command();

    let body = match modem_type {
        ModemTypes::Cgm4331com => {
            if args.username.is_none() || args.password.is_none() {
                cmd.error(
                    clap::ErrorKind::MissingRequiredArgument,
                    "Username and password are required for this modem type",
                )
                .exit();
            }
            fetcher::fetch(&fetcher::cgm4331com_fetcher::CGM4331COM::new(
                &args.username.unwrap(),
                &args.password.unwrap(),
            ))
            .unwrap()
        }
        ModemTypes::Mb8600 => fetcher::fetch(&fetcher::mb8600_fetcher::MB8600::new()).unwrap(),
    };

    let channel_info = response::parse(modem_type, &body)?;

    let output = output_formatter::CricketFormatter::format(&channel_info).unwrap();
    println!("{}", output);

    Ok(())
}
