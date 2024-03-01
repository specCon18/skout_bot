use dotenv::dotenv;
use reqwest;
use serde_json::Value;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = dotenv::var("YOUTUBE_API").expect("Expected a token in the environment");
    let binding = api_key.to_owned();
    let api_key = binding.as_str();
    let client = reqwest::Client::new();

    loop {
        let res = client.get("https://youtube.googleapis.com/youtube/v3/search")
            .query(&[
                ("part", "snippet"),
                ("channelId", "UC8ZFJetaO6NiRKERz6JXMcA"), // Replace with your channel ID
                ("eventType", "live"),
                ("maxResults", "5"),
                ("type", "video"),
                ("key", api_key),
            ])
            .header("Accept", "application/json")
            .send()
            .await?;
        let body = res.text().await?;
        let v: Value = serde_json::from_str(&body)?;

        if let Some(page_info) = v["pageInfo"].as_object(){
            if let Some(results_per_page) = page_info["resultsPerPage"].as_u64() {
                // Check if resultsPerPage is greater than 0
                if results_per_page > 0 {
                    let formatted_message = format!("[ThatGeodeGamer](https://www.youtube.com/@Gaming_Amethyst420) is Live! Come hang and chat!");
                    // Send the message to the Discord webhook
                    // TODO: Change to sending to skout
                    let _ = client.post(&webhook_url).json(&serde_json::json!({ "content": formatted_message })).send().await?;
                }
            }
        }

        // Sleep for 30 minutes
        sleep(Duration::from_secs(1800)).await;
    }
}
