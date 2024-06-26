[![Rust](https://github.com/jdicioccio/cablemodem_stats/actions/workflows/rust.yml/badge.svg)](https://github.com/jdicioccio/cablemodem_stats/actions/workflows/rust.yml)
# Cable Modem Stats gatherer (formerly only for Motorola MB8600)
## What is this?
In a nutshell, what it does is it:

1. Gathers stats from a Motorola, Comcast XB7 (Technicolor), or XB8 (Technicolor) cable modem. These stats include SnR, power level, corrected errors, and uncorrected errors for each channel.
1. It also will graph this data using the ancient RRDTool or the much newer InfluxDB (version 2)

I wrote this tool as I was having issues with my cable internet and wanted to have solid documentation to back it up. The issues have since been resolved. Hopefully this comes in handy for others.

**This, so far, is only known to work on the Motorola MB8600, the Comcast XB7 (CGM4331COM), and XB8 (CGM4981COM) modems.** If you have a different cable modem, feel free to give it a shot, but know that it's a roll of the dice. I suspect that other flavors of recent Comcast-provided modems have a decent chance of working.

## Prerequisites
- Rust compiler (I'm using 1.62.1)
- RRDTool or InfluxDBv2
- And for RRDTool, a unix/linux shell environment would let you run some of the convenience scripts

## Usage
### Build it
`cargo build`
### Run it
If you choose to go the `rrdtool` route (referred to as `cricket` output in `cablemodem_stats`), I've provided a convenience script to generate the RRD file. Run, for example `./target/build/cablemodem_stats mb8600 | ./rrd_create_command.sh`, to generate the appropriate RRD commands for creating the RRD file. You can copy and paste the output, or you can pipe it to `sh` if it looks sane.

After creating the RRD file, test the tool to make sure you're getting stats from it. For example, with an MB8600: ` ./target/debug/cablemodem_stats mb8600`

You should see a lot of output, such as: `1_ds.freq:495 1_ds.power:1.4 1_ds.snr:41.4 1_ds.corr_e:1895 1_ds.uncorr_e:2955 1_ds.lock_st:1 1_ds.uncorr_e_dlt:0 1_ds.corr_e_dlt:0 2_ds.freq:447 2_ds.power:2 2_ds.snr:41.9 ...`

If you're getting that, then we're ready to graph the data. For that, pipe the output to `rrd_update.sh`, like so: `./target/debug/cablemodem_stats mb8600 | ./rrd_update.sh`

Running that from a cron job, or in a sleep loop will populate that RRD file with data. Once you have a good amount of data and you want to see a graph, you can use/modify `rrd_graph.sh` as you like. It will spit out png files which you can view or make accessible however you wish.

**InfluxDB is likely the superior and less cumbersome option**, so that's what I'd recommend. Whichever output you use, you'll need to run `cablemodem_stats` periodically (I run it every 5 minutes) in order to populate your graphs.

### Options
```
USAGE:
    cablemodem_stats [OPTIONS] <MODEM_TYPE>

ARGS:
    <MODEM_TYPE>    [possible values: cgm4331com, cgm4981com, mb8600]

OPTIONS:
    -h, --help                        Print help information
        --influxdb-bucket <BUCKET>    InfluxDB bucket
        --influxdb-org <ORG>          InfluxDB organization
        --influxdb-token <TOKEN>      InfluxDB token
        --influxdb-url <URL>          InfluxDB URL
    -n, --no-ssl                      Don't use SSL when connecting to cable modem
    -o, --output <OUTPUT>             [default: cricket] [possible values: cricket, influxdbv2]
    -p, --password <PASSWORD>         Only used with some cable modems
    -u, --username <USERNAME>         Only used with some cable modems
    -V, --version                     Print version information
```
