use clap::{self, Parser};
use serde::{Deserialize, Deserializer};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_parser = file_exists)]
    config: Config,
}

#[derive(Debug, Clone)]
struct CLITool {
    cli_name: String,
    install_name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Config {
    cli_tools: Vec<CLITool>,
}

impl<'de> Deserialize<'de> for CLITool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ConfigVariants {
            String(String),
            Struct {
                cli_name: String,
                install_name: String,
            },
        }

        Ok(match ConfigVariants::deserialize(deserializer)? {
            ConfigVariants::String(name) => Self {
                cli_name: name.to_owned(),
                install_name: name.to_owned(),
            },
            ConfigVariants::Struct {
                cli_name,
                install_name,
            } => Self {
                cli_name,
                install_name,
            },
        })
    }
}

fn main() {
    let args = Args::parse();
    let cli_tools: &Vec<bool> = &args
        .config
        .cli_tools
        .clone()
        .into_iter()
        .map(|tool| is_program_installed(&tool))
        .collect();
    // let cli_tool = CLITool::from_config(&args.config["cli_tools"]["helix"]);

    dbg!(&cli_tools);
    // dbg!(&cli_tool);
}

fn is_program_installed(program: &CLITool) -> bool {
    // let mut cmd = Command::new("command");
    // let cmd = cmd.arg("-v");
    // let cmd = cmd.arg(&program.cli_name);

    // Command::new("source ~/.zshrc && command -v")
    //     // .stdout(Stdio::null())
    //     // .arg("-v")
    //     .arg(&program.cli_name)
    //     .stderr(Stdio::piped())
    //     .status()
    //     .map_or(false, |status| status.success())
    which::which(&program.cli_name).is_ok()
}

fn file_exists(file_path: &str) -> Result<Config, String> {
    let path = Path::new(&file_path);

    let file_text = fs::read_to_string(path).or(Err("could not read file".to_owned()))?;
    toml::from_str(&file_text).or(Err("Could not parse toml file".to_owned()))
}
