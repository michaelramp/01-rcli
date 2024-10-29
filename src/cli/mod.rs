mod base64;
mod csv;
mod genpass;
mod http;
mod jwt;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use regex::Regex;
use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, jwt::*, text::*};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
    #[command(subcommand, about = "Jwt sign/verify")]
    Jwt(JwtSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

fn parser_date(time: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"(\d+)([smhdw])")?;
    let mut total_seconds = 0;

    for cap in re.captures_iter(time) {
        let num = cap[1].parse::<u64>()?;
        let unit = &cap[2];
        let seconds = match unit {
            "s" => num,
            "m" => num * 60,
            "h" => num * 60 * 60,
            "d" => num * 60 * 60 * 24,
            "w" => num * 60 * 60 * 24 * 7,
            _ => return Err(anyhow::anyhow!("Invalid unit")),
        };
        total_seconds += seconds;
    }

    Ok(total_seconds.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
