use super::parser_date;
use crate::{process_jwt_sign, process_jwt_verify, Claims, CmdExector};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "Sign a jwt with a private key and return a signature")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a jwt with a public key")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, value_parser = parser_date, default_value = "3600")]
    pub exp: String,
    #[arg(long)]
    pub sub: Option<String>,
    #[arg(long)]
    pub aud: Option<String>,
    #[arg(long)]
    pub iss: Option<String>,
    #[arg(long, value_parser = parser_date)]
    pub iat: Option<String>,
    #[arg(long, value_parser = parser_date)]
    pub nbf: Option<String>,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let claims = Claims {
            exp: calculate_time(current_time, self.exp.parse().unwrap())?,
            sub: self.sub,
            aud: self.aud,
            iss: self.iss,
            iat: self
                .iat
                .map(|x| calculate_time(current_time, x.parse().unwrap()).unwrap()),
            nbf: self
                .nbf
                .map(|x| calculate_time(current_time, x.parse().unwrap()).unwrap()),
        };
        let sig = process_jwt_sign(claims)?;
        println!("{}", sig);
        Ok(())
    }
}

fn calculate_time(current_time: u64, diff: u64) -> anyhow::Result<usize> {
    let total_time = current_time + diff;
    let result: usize = total_time
        .try_into()
        .map_err(|_| anyhow::anyhow!("Value out of range for usize"))?;
    Ok(result)
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = self.token;
        let ret = process_jwt_verify(&token)?;
        println!("Jwt verify result: {}", ret);
        Ok(())
    }
}
