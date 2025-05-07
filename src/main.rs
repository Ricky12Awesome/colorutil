use clap::{Arg, ValueHint};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Generate completions for your shell")]
    Completions,
    #[clap(about = "Parse text or file with color info")]
    Parse {
        #[clap(
            index = 1,
            required_unless_present = "text",
            value_hint = ValueHint::FilePath
        )]
        src: Option<PathBuf>,

        #[clap(
            index = 2,
            value_hint = ValueHint::FilePath
        )]
        dst: Option<PathBuf>,

        #[clap(
            long,
            conflicts_with_all = ["src", "dst"],
            required_unless_present_all = ["src", "dst"],
            value_hint = ValueHint::FilePath
        )]
        text: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
}
