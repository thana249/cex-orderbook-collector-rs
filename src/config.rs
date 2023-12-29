use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};

/// Represents the configuration for the order book collector.
///
/// This struct is used to deserialize the configuration from a JSON file.
/// It includes the name of the cryptocurrency exchange (CEX) and a list of tickers to collect order book data for.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// The name of the cryptocurrency exchange (e.g., "BINANCE", "BITKUB").
    pub cex: String,
    /// A list of asset tickers (e.g., "BTC_USDT", "ETH_USDT") for which to collect order book data.
    pub tickers: Vec<String>,
}

impl Config {
    /// Returns the path to the configuration file.
    ///
    /// This is a static method that provides the path to the `config.json` file.
    /// The path is hardcoded and points to the file in the current working directory.
    pub fn path() -> &'static str {
        "config.json"
    }

    /// Loads the configuration from a JSON file.
    ///
    /// This method reads the configuration file located at the path returned by `Config::path()`,
    /// deserializes it into a `Config` object, and returns it.
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if reading from the file fails or if the file content is not a valid JSON format for `Config`.
    pub fn load() -> Result<Config, io::Error> {
        let file_path = Config::path();
        let mut file = fs::File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(config)
    }
}
