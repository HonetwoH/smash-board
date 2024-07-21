// this modules will read and interpret config which is
// as of now just the number of available buffers
#[cfg(feature = "read-config")]
use serde::Deserialize;
#[cfg(feature = "read-config")]
use std::{env, fs};

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "read-config", derive(Deserialize))]
pub enum Base {
    Hexa = 6,
    #[default]
    Octal = 8,
    Decimal = 10,
    HexaDecimal = 16,
}

#[cfg_attr(feature = "read-config", derive(Deserialize))]
pub struct Config {
    base: Base,
    // in seconds
    polling_rate: u16,
}

impl Config {
    fn new(base: Base) -> Self {
        Self {
            base,
            polling_rate: 2,
        }
    }

    pub fn base(self) -> Base {
        self.base
    }
    pub fn polling_rate(self) -> u16 {
        self.polling_rate
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(Base::Octal)
    }
}

#[cfg(feature = "read-config")]
pub fn read_config() -> Config {
    let config_filename = {
        let mut config_dir = env::var("XDG_CONFIG_HOME").expect(
            "Could not find XDG_CONFIG_HOME variable in environment, or maybe it is overloaded",
        );
        config_dir.push_str("/smashboard.toml");
        config_dir
    };
    let config_string = fs::read_to_string(config_filename)
        .expect("Could not find the config file \"smashboard.toml\" in config directory.");
    let config: Config = toml::from_str(&config_string).unwrap_or(Config {
        base: Base::Octal,
        polling_rate: 2,
    });
    config
}

#[test]
#[cfg(feature = "read-config")]
fn t() {
    let toml_str = r#"
    available_buffers = "Hexa"
    polling_rate = 3
    "#;

    let decoded: Config = toml::from_str(toml_str).unwrap();
    println!("{:#?}", decoded.base);
}
