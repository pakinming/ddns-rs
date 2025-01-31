use rand::Rng;
use reqwest;
use tokio::time::{self, Duration};
use tracing::{error, info};
use tracing_appender::rolling;
use tracing_subscriber::filter;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time as logtime;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    let file_appender = rolling::daily("logs", "ip_changes.log");
    let file_layer = fmt::layer()
        .with_timer(logtime::SystemTime::default()) // Use the custom time formatter
        .with_writer(file_appender)
        .with_target(false) // Hide module paths in file
        .with_level(true) // Show log levels in file
        .with_ansi(false);

    let console_layer = fmt::layer()
        .with_timer(logtime::SystemTime::default()) // Same custom time formatter for console
        .with_ansi(true) // Enable ANSI color codes in console logs
        .with_level(true) // Show log levels in console
        .with_target(true); // Show module paths in console

    tracing_subscriber::registry()
        .with(filter::LevelFilter::INFO)
        .with(file_layer)
        .with(console_layer)
        .init();

    info!("Starting IP change detection...");

    let mut previous_ip = String::new();
    let check_interval = Duration::from_secs(3);
    let mut interval = time::interval(check_interval);

    loop {
        interval.tick().await;

        match get_public_ip().await {
            Ok(current_ip) => {
                if current_ip != previous_ip {
                    info!(
                        "IP Changed: Old IP = {}, New IP = {}",
                        previous_ip, current_ip
                    );
                    previous_ip = current_ip;
                } else {
                    info!("IP Unchanged: {}", current_ip);
                }
            }
            Err(e) => {
                error!("Error fetching public IP: {}", e);
            }
        }
    }
}

// Function to fetch the public IP using a public API
async fn get_public_ip() -> Result<String, reqwest::Error> {
    let mut rng = rand::thread_rng();

    let url = if rng.gen_bool(0.5) {
        "https://api.ipify.org"
    } else {
        "https://ipinfo.io/ip"
    };
    info!("url: {}", url);

    let response = reqwest::get(url).await?.text().await?;
    info!("response: {}", response);
    Ok(response.trim().to_string())
}
