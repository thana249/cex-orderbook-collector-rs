use async_trait::async_trait;
use std::error::Error;
use crate::cex_api::CexApi;
use crate::ticker;
use ticker::Ticker;

/// Represents the Bitkub API for fetching order book data.
pub struct BitkubApi;

#[async_trait]
impl CexApi for BitkubApi {
    /// Returns the name of the exchange.
    ///
    /// # Returns
    /// A static string slice representing the name of the exchange.
    fn name(&self) -> &'static str {
        "BITKUB"
    }

    /// Asynchronously fetches the order book for a given ticker symbol up to a specified depth.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A reference to a `Ticker` struct containing the base and quote currencies.
    /// * `depth` - The depth of the order book to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` with the order book data as a `String` if successful, or an `Err` with an error message.
    async fn get_order_book(&self, ticker: &Ticker, depth: u32) -> Result<String, Box<dyn Error>> {
        // Construct the symbol by combining the quote and base currencies.
        let symbol = format!("{}_{}", ticker.quote, ticker.base);

        // Perform the HTTP GET request to fetch the order book data.
        let response_text = reqwest::get(&format!(
            "https://api.bitkub.com/api/market/depth?sym={}&lmt={}",
            symbol, depth
        ))
            .await?
            .text()
            .await?;

        // Check if the response contains a specific error message indicating a null result.
        if response_text.contains(r#""result":null"#) {
            Err("Received null result in response".into())
        } else {
            Ok(response_text)
        }
    }

    /// Returns the interval at which the order book should be fetched.
    ///
    /// # Returns
    /// A `u64` representing the interval in seconds.
    fn get_order_book_interval(&self) -> u64 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    /// Test to ensure the API name is correct.
    #[test]
    fn test_bitkub_api_name() {
        assert_eq!(BitkubApi.name(), "BITKUB");
    }

    /// Asynchronous test to check the functionality of the `get_order_book` method.
    #[tokio::test]
    async fn test_get_order_book() {
        // Mock HTTP requests setup (if applicable)

        let ticker = Ticker::new("BTC_THB").unwrap();
        let result = BitkubApi.get_order_book(&ticker, 10).await;

        // Assert that the result is Ok and contains the expected "asks" and "bids" data.
        assert!(result.is_ok());
        if let Ok(response_text) = result {
            let json: Value = serde_json::from_str(&response_text).unwrap();
            assert_eq!(json["asks"].as_array().unwrap().len(), 10);
            assert_eq!(json["bids"].as_array().unwrap().len(), 10);
        }
    }

    /// Test to ensure the order book fetch interval is correct.
    #[test]
    fn test_get_order_book_interval() {
        assert_eq!(BitkubApi.get_order_book_interval(), 2);
    }
}
