use async_trait::async_trait;
use std::error::Error;
use reqwest;
use crate::cex_api::CexApi;
use crate::ticker::Ticker;

/// Represents the Binance API for fetching order book data.
pub struct BinanceApi;

#[async_trait]
impl CexApi for BinanceApi {
    /// Returns the name of the exchange.
    ///
    /// # Returns
    /// A static string slice representing the name of the exchange.
    fn name(&self) -> &'static str {
        "BINANCE"
    }

    /// Asynchronously fetches the order book for a given ticker and depth from Binance.
    ///
    /// # Arguments
    /// * `ticker` - A reference to a `Ticker` struct containing the base and quote currencies.
    /// * `depth` - The depth of the order book to fetch.
    ///
    /// # Returns
    /// A `Result` which is either a string containing the order book data or an error.
    async fn get_order_book(&self, ticker: &Ticker, depth: u32) -> Result<String, Box<dyn Error>> {
        let symbol = format!("{}{}", ticker.base, ticker.quote);
        let response_text = reqwest::get(&format!(
            "https://api.binance.com/api/v3/depth?symbol={}&limit={}",
            symbol, depth
        )).await?
            .text()
            .await?;

        if response_text.contains(r#""code":-"#) {
            Err("Invalid symbol in response from Binance".into())
        } else {
            Ok(response_text)
        }
    }

    /// Returns the interval at which the order book should be fetched.
    ///
    /// # Returns
    /// A `u64` representing the interval in seconds.
    fn get_order_book_interval(&self) -> u64 {
        1
    }
}

// Unit tests for the BinanceApi implementation
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_binance_api_name() {
        assert_eq!(BinanceApi.name(), "BINANCE");
    }

    #[tokio::test]
    async fn test_get_order_book() {
        // Mock HTTP requests setup would go here

        let ticker = Ticker::new("BTC_USDT").unwrap();
        let result = BinanceApi.get_order_book(&ticker, 10).await;

        assert!(result.is_ok());
        if let Ok(response_text) = result {
            let json: Value = serde_json::from_str(&response_text).unwrap();
            assert_eq!(json["asks"].as_array().unwrap().len(), 10);
            assert_eq!(json["bids"].as_array().unwrap().len(), 10);
        }
    }

    #[test]
    fn test_get_order_book_interval() {
        assert_eq!(BinanceApi.get_order_book_interval(), 1);
    }
}
