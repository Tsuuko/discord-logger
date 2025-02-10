use std::env;
use std::io::{self, BufRead};
use serde::Serialize;

#[derive(Serialize)]
struct WebhookMessage {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get webhook URL from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <webhook-url>", args[0]);
        std::process::exit(1);
    }
    let webhook_url = &args[1];

    // Read from standard input
    let stdin = io::stdin();
    let mut content = String::new();
    for line in stdin.lock().lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    // Don't send empty content
    if content.trim().is_empty() {
        return Ok(());
    }

    // Send message
    let client = reqwest::Client::new();
    let message = WebhookMessage { content };

    client.post(webhook_url)
        .json(&message)
        .send()
        .await?;

    Ok(())
}
