use gemini_rust::Gemini;
use std::{ env, error::Error };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv()?;

    // Retrieve the Gemini API key from environment variables
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set in .env file");

    // Initialize the Gemini client
    let client = Gemini::new(api_key)?;

    // Example usage: Generate text
    let prompt = "You are a helpful assistant.";
    let message = "Hello, how are you?";
    let response = client
        .generate_content()
        .with_system_prompt(prompt)
        .with_user_message(message)
        .execute().await?;

    println!("Response from Gemini: {}", response.text());

    Ok(())
}
