use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use tokio::time::{sleep, Duration};
use chrono::prelude::Utc;
use std::io::Write;
use std::fmt::Write as FmtWrite;
use crate::cex_api::CexApi;
use crate::ticker::Ticker;

/// A collector for order book data from cryptocurrency exchanges (CEXs).
pub struct OrderBookCollector {
    handles: HashMap<String, thread::JoinHandle<()>>,
    alive: HashMap<String, Arc<AtomicBool>>,
}

impl OrderBookCollector {
    /// Creates a new `OrderBookCollector`.
    pub fn new() -> OrderBookCollector {
        OrderBookCollector {
            handles: HashMap::new(),
            alive: HashMap::new(),
        }
    }

    /// Starts collecting order book data for a given symbol using a specified API.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A string slice that holds the symbol to collect data for.
    /// * `api` - An `Arc` pointing to an object that implements the `CexApi` trait.
    pub fn start<T>(&mut self, symbol: &str, api: Arc<T>)
        where
            T: 'static + Send + Sync + CexApi,
    {
        if let Some(ticker) = Ticker::new(symbol) {
            println!("Start {}", symbol);
            let alive_flag = self.alive.entry(symbol.to_string())
                .or_insert_with(|| Arc::new(AtomicBool::new(true)));
            alive_flag.store(true, Ordering::SeqCst);
            let alive_clone = alive_flag.clone();

            let api_clone = api.clone(); // Clone the API object

            let handle = thread::spawn(move || {
                let runtime = tokio::runtime::Runtime::new().unwrap(); // Create a new Tokio runtime
                runtime.block_on(async move {
                    OrderBookCollector::worker(ticker, api_clone, alive_clone).await;
                });
            });

            self.handles.insert(symbol.to_string(), handle);
        } else {
            eprintln!("Invalid symbol format: {}", symbol);
        }
    }

    /// Stops collecting order book data for a given symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A string slice that holds the symbol to stop collecting data for.
    pub fn stop(&mut self, symbol: &str) {
        if let Some(alive) = self.alive.get(symbol) {
            println!("Stop {}", symbol);
            alive.store(false, Ordering::SeqCst);
            if let Some(handle) = self.handles.remove(symbol) {
                handle.join().expect("Could not join spawned thread");
            }
        }
    }

    /// Starts collecting order book data for multiple symbols.
    ///
    /// # Arguments
    ///
    /// * `symbols` - A slice of strings that holds the symbols to collect data for.
    /// * `api` - An `Arc` pointing to an object that implements the `CexApi` trait.
    pub fn start_multiple<T>(&mut self, symbols: &[String], api: Arc<T>)
        where
            T: 'static + Send + Sync + CexApi,
    {
        let symbol_set: std::collections::HashSet<_> = symbols.iter().cloned().collect();

        for existing_symbol in self.handles.keys().cloned().collect::<Vec<_>>() {
            if !symbol_set.contains(&existing_symbol) {
                self.stop(&existing_symbol);
            }
        }

        for symbol in symbols {
            if !self.handles.contains_key(symbol) {
                self.start(symbol, api.clone());
            }
        }
    }

    /// Stops all collecting threads.
    #[allow(dead_code)]
    pub fn stop_all(&mut self) {
        for alive in self.alive.values() {
            alive.store(false, Ordering::SeqCst);
        }

        for handle in self.handles.drain().map(|(_, h)| h) {
            handle.join().expect("Could not join spawned thread");
        }
    }

    /// The worker function for collecting order book data.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A `Ticker` object representing the asset pair.
    /// * `api` - An `Arc` pointing to an object that implements the `CexApi` trait.
    /// * `alive` - An `Arc` pointing to an `AtomicBool` that indicates whether the thread should continue running.
    pub async fn worker(ticker: Ticker, api: Arc<dyn CexApi>, alive: Arc<AtomicBool>) {
        let interval_in_milliseconds = api.get_order_book_interval() * 1000;
        let remainder = Utc::now().timestamp_millis() as u64 % interval_in_milliseconds;
        if remainder > 0 {
            sleep(Duration::from_millis(interval_in_milliseconds - remainder)).await;
        }

        let dir = format!("data/{}/{}", api.name(), ticker.to_string());
        OrderBookCollector::create_directory(dir.as_str());

        let mut file_path = dir.clone();
        let mut last_saved_hour_timestamp = 0;

        while alive.load(Ordering::SeqCst) {
            let response_result = api.get_order_book(&ticker, 10).await;

            match response_result {
                Ok(response_text) => {
                    let timestamp = Utc::now().timestamp();
                    let response_text = response_text.trim_end_matches('\n');

                    let json_data = format!(
                        r#"{{"time": {}, "response": {}}}"#,
                        timestamp, response_text
                    );

                    let hour_timestamp = timestamp / 3600i64 * 3600;
                    if hour_timestamp > last_saved_hour_timestamp {
                        file_path.truncate(dir.len());
                        write!(file_path, "/{}.json", hour_timestamp).unwrap();
                        println!("{}", file_path);

                        last_saved_hour_timestamp = hour_timestamp;
                    }

                    OrderBookCollector::save_to_file(&file_path, &json_data);
                }
                Err(error) => {
                    eprintln!("Error fetching order book: {:?}", error);
                }
            }
            let remainder = Utc::now().timestamp_millis() as u64 % interval_in_milliseconds;
            if remainder > 0 {
                sleep(Duration::from_millis(interval_in_milliseconds - remainder)).await;
            }
        }
        println!("Worker for {} is stopped", ticker.base);
    }

    /// Creates a directory if it does not exist.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path of the directory to create.
    fn create_directory(path: &str) {
        create_dir_all(path).expect(&format!("Cannot create dir {}", path));
        println!("Directory {} created or already exists", path);
    }

    /// Saves data to a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path of the file to write to.
    /// * `data` - A string slice containing the data to be written.
    fn save_to_file(file_path: &String, data: &String) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Unable to open file");

        writeln!(file, "{}", data).expect("Unable to write data");
    }
}
