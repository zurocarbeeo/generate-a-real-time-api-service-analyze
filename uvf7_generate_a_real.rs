use config::{Config, Environment};
use serde::{Deserialize, Serialize};
use tokio::{runtime, spawn};
use tracing::{info, instrument};

// Configuration for the API Service Analyzer
#[derive(Deserialize, Serialize)]
struct Config {
    api_url: String,
    api_key: String,
    analysis_interval: u64,
}

impl Config {
    fn from_env() -> Result<Self, config::ConfigError> {
        Config::fromEnvironment()
    }
}

// Real-time API Service Analyzer
#[instrument]
async fn analyze_api(config: Config) {
    let mut interval = tokio::time::interval(config.analysis_interval);
    let client = reqwest::Client::new();

    loop {
        interval.tick().await;
        let response = client.get(&config.api_url).header("Authorization", &format!("Bearer {}", config.api_key)).send().await?;
        let status_code = response.status().as_u16();
        info!("Received API response with status code {}", status_code);
        // Perform analysis on the response here
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    info!("Starting API Service Analyzer with config: {:?}", config);
    spawn(analyze_api(config));
    Ok(())
}