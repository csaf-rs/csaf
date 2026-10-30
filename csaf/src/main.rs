use clap::{Parser, Subcommand};
use csaf_converter::cli::{ConverterArgs, run_converter};
use csaf_validator::cli::{ValidatorArgs, run_validator};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Validate(ValidatorArgs),
    Convert(ConverterArgs),
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        Commands::Convert(args) => run_converter(args),
        Commands::Validate(args) => run_validator(args),
    }
}
