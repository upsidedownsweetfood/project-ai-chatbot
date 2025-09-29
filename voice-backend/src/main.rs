mod gemini;
use std::error::Error;
use gemini::{ create_client, send_message_to_gemini, GeminiRequest };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = create_client().await?;

    let prompt = "You are a helpful assistant.";
    let message = "Hello, I am Francesco";

    let mut gemini_request = GeminiRequest::new(prompt);
    gemini_request.set_message(message);

    let response = send_message_to_gemini(&client, &mut gemini_request).await?;
    println!("\nResponse from Gemini: {}", response);

    let message = "Do you remember my name?";
    println!("{}", message);
    gemini_request.set_message(message);
    let response = send_message_to_gemini(&client, &mut gemini_request).await?;
    println!("\nResponse from Gemini: {}", response);

    Ok(())
}
