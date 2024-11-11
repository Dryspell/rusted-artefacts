use serde_json::Value;
use std::env;

// Function to make the API request and format JSON response
pub async fn pokemon_test_request() -> Result<String, Box<dyn std::error::Error>>{
    // Fetch environment variables
    let _api_key = env::var("API_KEY").expect("API_KEY must be set");
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let url = format!("{}pokemon/ditto", base_url);

    // Make the request
    let response = reqwest::get(&url).await?;

    // Check response status
    if response.status().is_success() {
        let body = response.text().await?;

        // Parse JSON and format it
        let json: Value = serde_json::from_str(&body)?;
        let formatted_json = serde_json::to_string_pretty(&json).unwrap_or_else(|_| "Invalid JSON format".to_string());

        Ok(formatted_json)
    } else {
        Ok(format!("Failed: {}", response.status()))
    }
}
