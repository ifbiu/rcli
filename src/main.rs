// rcli csv -i input.csv -o output.json --header -d ','

use clap::{Parser};
use rcli::{Opts, process_csv, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}",opts);
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
