/// Represents a trading pair in a cryptocurrency exchange.
///
/// A `Ticker` consists of a base currency and a quote currency.
/// For example, in the trading pair "BTC_USDT", BTC is the base currency,
/// and USDT is the quote currency.
pub struct Ticker {
    pub base: String,
    pub quote: String,
}

impl Ticker {
    /// Creates a new `Ticker` from a symbol string.
    ///
    /// The symbol should be in the format "BASE_QUOTE", where BASE is the base currency
    /// and QUOTE is the quote currency. For example, "BTC_USDT".
    ///
    /// Returns `Some(Ticker)` if the symbol is valid, or `None` if the symbol format is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let ticker = Ticker::new("BTC_USDT").unwrap();
    /// assert_eq!(ticker.base, "BTC");
    /// assert_eq!(ticker.quote, "USDT");
    /// ```
    pub fn new(symbol: &str) -> Option<Self> {
        if let Some(index) = symbol.find('_') {
            let base = &symbol[..index];
            let quote = &symbol[index+1..];
            Some(Ticker {
                base: base.to_string(),
                quote: quote.to_string()
            })
        } else {
            None
        }
    }

    /// Returns a string representation of the `Ticker`.
    ///
    /// The format of the string is "BASE_QUOTE".
    ///
    /// # Examples
    ///
    /// ```
    /// let ticker = Ticker::new("BTC_USDT").unwrap();
    /// assert_eq!(ticker.to_string(), "BTC_USDT");
    /// ```
    pub fn to_string(&self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

#[cfg(test)]
mod tests {
    use super::Ticker;

    #[test]
    fn test_ticker_new_valid() {
        let symbol = "BTC_USDT";
        let ticker = Ticker::new(symbol).unwrap();

        assert_eq!(ticker.base, "BTC");
        assert_eq!(ticker.quote, "USDT");
    }

    #[test]
    fn test_ticker_new_invalid() {
        let symbol = "InvalidSymbol";
        assert!(Ticker::new(symbol).is_none());
    }

    #[test]
    fn test_ticker_to_string() {
        let ticker = Ticker {
            base: "BTC".to_string(),
            quote: "USDT".to_string(),
        };

        assert_eq!(ticker.to_string(), "BTC_USDT");
    }
}
