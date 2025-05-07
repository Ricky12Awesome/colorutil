use clap::{value_parser, CommandFactory, ValueHint};
use clap::{Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::borrow::Cow;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    #[clap(about = "Generate completions for your shell")]
    Completions {
        #[clap(
            index = 1,
            value_parser = value_parser!(Shell)
        )]
        shell: Shell,
    },
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
            value_hint = ValueHint::Other
        )]
        text: Option<Cow<'static, str>>,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        CliCommand::Completions { shell } => {
            let mut command = Cli::command();
            let name = command.get_name().to_string();

            generate(*shell, &mut command, name, &mut std::io::stdout());
        }
        CliCommand::Parse {
            src: Some(src),
            dst,
            ..
        } => {}
        CliCommand::Parse {
            text: Some(text), ..
        } => {}
        _ => {}
    }
}
