use std::io::prelude::*;
use std::fs::File;
use std::env;
use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use toml;
use error::*;

#[derive(Default, Debug, StructOpt, Serialize, Deserialize)]
/// Configuration options
///
/// These are both the CLI and TOML options
pub struct Config {
    #[structopt(short = "p", long = "port", help = "Port to run on", default_value = "3000")]
    pub port: u16,

    #[structopt(short = "b", long = "bind", help = "Bind to IP address",
                default_value = "127.0.0.1")]
    pub bind: String,

    #[serde(skip)]
    #[structopt(short = "c", long = "config", help = "Path to configuration file",
                default_value = "./config.toml")]
    pub config_path: String,

    #[structopt(short = "d", long = "database", help = "Database URL to use")]
    pub database_url: Option<String>,
}

impl Config {
    pub fn is_complete(&self) -> bool {
        if self.database_url == None {
            env::var("DATABASE_URL").is_ok()
        } else {
            true
        }
    }

    /// Merges two configurations together with the left hand side being dominant
    ///
    /// A value is pulled from the right hand side when the left hand side's value is
    /// the default value
    ///
    /// e.g.
    /// `lhs.port = u16::Default()`
    /// `rhs.port = 2000`
    /// `final.port = 2000`
    ///
    /// e.g.
    /// `lhs.port = 1000`
    /// `rhs.port = 2000`
    /// `final.port = 1000`
    pub fn merge(self, other: Config) -> Self {
        let port = if self.port == 3000 {
            other.port
        } else {
            self.port
        };

        let bind = if self.bind == "127.0.0.1".to_owned() {
            other.bind
        } else {
            self.bind
        };

        let config_path = if self.config_path == String::default() {
            other.config_path
        } else {
            self.config_path
        };

        let database_url = if self.database_url == None {
            other.database_url
        } else {
            self.database_url
        };


        Config {
            port: port,
            bind: bind,
            config_path: config_path,
            database_url: database_url,
        }
    }
}

/// Merges the CLI and configuration file settings together
///
/// > Note: If the CLI options are complete then the configuration file
/// > is not read.
pub fn get_config() -> Result<Config> {
    let options = Config::from_args();

    // no need to merge if CLI is complete
    if !options.is_complete() {
        let config = read_config_file(&options.config_path);
        config.map(|config| options.merge(config))
    } else {
        Ok(options)
    }
}

fn read_config_file(path: &str) -> Result<Config> {
    let mut config_file = File::open(path)?;

    let mut buffer = vec![];
    config_file.read_to_end(&mut buffer)?;

    toml::from_slice::<Config>(&buffer).map_err(|err| err.into())
}
