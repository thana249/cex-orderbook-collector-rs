use async_trait::async_trait;
use std::error::Error;
use crate::ticker::Ticker;

/// `CexApi` is a trait defining the common interface for interacting with different cryptocurrency exchanges (CEX).
/// It provides methods for fetching order book data and other exchange-specific information.
#[async_trait]
pub trait CexApi {
    /// Returns the name of the cryptocurrency exchange.
    /// This is typically a static string representing the exchange, like "BINANCE" or "BITKUB".
    fn name(&self) -> &'static str;

    /// Asynchronously fetches the order book for a given symbol up to a specified depth.
    ///
    /// # Arguments
    /// * `symbol` - A `Ticker` representing the trading pair (e.g., BTC_USDT).
    /// * `depth` - The depth of the order book to fetch. This usually represents the number of buy/sell orders to retrieve.
    ///
    /// # Returns
    /// A `Result` which is `Ok` with the order book data as a JSON string if the fetch is successful,
    /// or an `Err` with an error message boxed as a `dyn Error` if the fetch fails.
    async fn get_order_book(&self, symbol: &Ticker, depth: u32) -> Result<String, Box<dyn Error>>;

    /// Returns the interval in seconds at which the order book should be fetched.
    /// This can be used to rate limit the requests to the exchange's API.
    fn get_order_book_interval(&self) -> u64;
}
