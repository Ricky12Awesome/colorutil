use clap::{CommandFactory, ValueHint, value_parser};
use clap::{Parser, Subcommand};
use clap_complete::{Shell, generate};
use colorutil::Error;
use colorutil::config::{ConfigBase, load_config, override_config_dir};
use colorutil::parse::replace_colors;
use std::borrow::Cow;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand,

    #[clap(short, long, help = "Override config for this instance")]
    config: Option<PathBuf>,

    #[clap(short, long, help = "Override palette for this instance")]
    palette: Option<Cow<'static, str>>,
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
            required_unless_present_any = ["src", "dst"],
            value_hint = ValueHint::Other
        )]
        text: Option<Cow<'static, str>>,

        #[clap(
            long,
            value_hint = ValueHint::Other
        )]
        prefix: Option<Cow<'static, str>>,

        #[clap(
            long,
            value_hint = ValueHint::Other
        )]
        suffix: Option<Cow<'static, str>>,

        #[clap(
            long,
            action,
            default_value_t = false,
            help = "Overrides destination file if it exists"
        )]
        force: bool,
    },
}

fn main() -> colorutil::Result<()> {
    let args = Cli::parse();

    if let Some(path) = &args.config {
        override_config_dir(path);
    }

    let config = load_config::<ConfigBase>("config")?;
    let config = config.parse()?;
    let palette = &config.palettes[&config.palette];

    match args.command {
        CliCommand::Completions { shell } => {
            let mut command = Cli::command();
            let name = command.get_name().to_string();

            generate(shell, &mut command, name, &mut std::io::stdout());
        }
        CliCommand::Parse {
            src: Some(src),
            dst,
            force,
            ..
        } => {
            let text = std::fs::read_to_string(&src)?;
            let text = replace_colors(text, config.prefix, config.suffix, palette)?;

            match dst {
                None => {
                    println!("{}", text);
                }
                Some(path) if force && !path.is_file() => {
                    return Err(Error::NotFile(path));
                }
                Some(path) if force && path.is_file() || !path.exists() => {
                    std::fs::write(&path, text)?;
                }
                Some(path) if !force && path.is_file() => {
                    print!(
                        "Do you want to overwrite '{}'? [y/n]: ",
                        path.to_string_lossy()
                    );

                    std::io::stdout().flush()?;

                    let Some(Ok(line)) = std::io::stdin().lines().next() else {
                        return Ok(());
                    };

                    let line = line.to_lowercase();
                    let line = line.trim();

                    match line {
                        "y" | "yes" => {
                            std::fs::write(&path, text)?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        CliCommand::Parse {
            text: Some(text),
            prefix,
            suffix,
            ..
        } => {
            let prefix = prefix.unwrap_or(config.prefix);
            let suffix = suffix.unwrap_or(config.suffix);

            let text = replace_colors(text, prefix, suffix, palette)?;

            println!("{}", text);
        }
        _ => {}
    }

    Ok(())
}
