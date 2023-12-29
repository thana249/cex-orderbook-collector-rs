# CEX Order Book Collector

## Overview
This Rust project is designed to collect order book data from various Cryptocurrency Exchanges (CEXs) such as Binance and Bitkub. It operates by spawning separate threads for each asset and saves the order book data in JSON format to the `data` folder.

## Features
- Supports multiple CEXs (Binance, Bitkub).
- Concurrent data collection through multi-threading.
- Real-time monitoring of `config.json` for dynamic symbol management.
- Data persistence in JSON format.

## Configuration
The service is configured using a `config.json` file, which specifies the CEX and the ticker symbols to track. Here are two example configurations:

**For Binance:**
```json
{
  "cex": "BINANCE",
  "tickers": [
    "BTC_USDT",
    "ETH_USDT"
  ]
}
```

**For Bitkub:**
```json
{
  "cex": "BITKUB",
  "tickers": [
    "BTC_THB",
    "ETH_THB"
  ]
}
```

Note: While the service is running, you can add or remove ticker symbols from `config.json`. The service will update its running threads accordingly. However, changing the CEX in the configuration is not currently supported.

## Running the Service with Docker Compose
To run the service using Docker Compose, follow these steps:

1. Clone the repository:
   ```bash
   git clone [Your Repository URL]
   cd [Your Repository Name]
   ```
2. Start the service using Docker Compose:
   ```bash
   docker-compose up --build
   ```
3. To stop the service, use:
   ```bash
   docker-compose down
   ```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
