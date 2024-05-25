// rcli csv -i input.csv -o output.json --header -d ','

use clap::{Parser};
use rcli::{Opts, process_csv, process_genpass, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}",opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output{
                output.clone()
            } else{
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        },
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            );
        }
    }
    Ok(())
}
