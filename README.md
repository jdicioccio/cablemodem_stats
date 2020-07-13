# Motorola / Arris Cable Modem Stats
## What is this?
As you can see, I don't have a clever name for this tool, but, in a nutshell, what it does is it:

1. Gathers stats from a Motorola/Arris cable modem. These stats include SnR, power level, corrected errors, and uncorrected errors for each channel.
1. It also will graph this data using the ancient RRDTool.

I wrote this tool as I was having issues with my cable internet and wanted to have solid documentation to back it up. The issues have since been resolved. Hopefully this comes in handy for others.

## Prerequisites
- Rust compiler (I'm using 1.44.1)
- RRDTool
- A unix/linux shell environment would let you run some of the convenience scripts

## Usage
### Build it
`cargo build`
### Run it
I've provided a convenience script to generate the RRD file. Run, for example `./target/build/moto_arris_cm_stats | ./rrd_create_command.sh`, to generate the appropriate RRD commands for creating the RRD file. You can copy and paste the output, or you can pipe it to `sh` if it looks sane.

After creating the RRD file, test the tool to make sure you're getting stats from it. For example: ` ./target/debug/moto_arris_cm_stats`

You should see a lot of output, such as: `1_ds.freq:495 1_ds.power:1.4 1_ds.snr:41.4 1_ds.corr_e:1895 1_ds.uncorr_e:2955 1_ds.lock_st:1 1_ds.uncorr_e_dlt:0 1_ds.corr_e_dlt:0 2_ds.freq:447 2_ds.power:2 2_ds.snr:41.9 ...`

If you're getting that, then we're ready to graph the data. For that, pipe the output to `rrd_update.sh`, like so: `./target/debug/moto_arris_cm_stats | ./rrd_update.sh`

Running that from a cron job, or in a sleep loop will populate that RRD file with data. Once you have a good amount of data and you want to see a graph, you can use/modify `rrd_graph.sh` as you like. It will spit out png files which you can view or make accessible however you wish.
