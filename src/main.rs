// Module imports
mod config;
mod cex_api;
mod binance_api;
mod bitkub_api;
mod ticker;
mod orderbook_collector;

// Use statements to bring types into scope
use orderbook_collector::OrderBookCollector;
use binance_api::BinanceApi;
use bitkub_api::BitkubApi;
use crate::config::Config;
use std::path::Path;
use notify::{Watcher, RecursiveMode};

/// Updates the tasks in the OrderBookCollector based on the current configuration.
/// It loads the configuration and starts collecting order books for the specified tickers.
fn update_tasks_based_on_config(collector: &mut OrderBookCollector) {
    match Config::load() {
        Ok(config) => {
            println!("CEX: {}", config.cex);
            // Start tasks based on the specified CEX in the configuration
            if config.cex == "BINANCE" {
                collector.start_multiple(&config.tickers, BinanceApi.into());
            } else if config.cex == "BITKUB" {
                collector.start_multiple(&config.tickers, BitkubApi.into());
            } else {
                eprintln!("Unsupported CEX: {}", config.cex);
                return;
            }
        }
        Err(e) => eprintln!("Failed to load config: {}", e),
    }
}

fn main() {
    // Initialize the OrderBookCollector
    let mut collector = OrderBookCollector::new();

    // Load and apply the initial configuration
    update_tasks_based_on_config(&mut collector);

    // Set up a filesystem watcher to monitor changes in the configuration file
    let mut watcher = notify::recommended_watcher(move |res| {
        match res {
            Ok(event) => {
                println!("Change detected: {:?}", event);
                // Reload the configuration and update tasks upon any change
                update_tasks_based_on_config(&mut collector);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();

    // Watch the configuration file for changes
    watcher.watch(Path::new(Config::path()), RecursiveMode::NonRecursive).unwrap();

    // Keep the main thread alive to continuously monitor for changes
    loop {}
}
