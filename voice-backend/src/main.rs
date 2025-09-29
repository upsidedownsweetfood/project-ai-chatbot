mod gemini;
mod api;
use actix_web::{ web, App, HttpServer };
use gemini::{ create_client, send_message_to_gemini, GeminiRequest };
use api::{ input_message };
use std::{ error::Error, net::TcpListener };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = format!("127.0.0.1:8080");
    let listener = TcpListener::bind(address)?;
    let client = create_client().await?;
    let prompt = "You are a helpful assistant.";
    let mut gemini_request = GeminiRequest::new(client, prompt);

    HttpServer::new(move || {
        App::new().app_data(gemini_request.clone_from(&gemini_request)).route("/message", web::post().to(input_message))
    })
        .listen(listener)?
        .run().await?;

    Ok(())
    /* 
    let message = "Hello, I am Francesco";

    gemini_request.set_message(message);

    let response = send_message_to_gemini(&client, &mut gemini_request).await?;
    println!("\nResponse from Gemini: {}", response);

    let message = "Do you remember my name?";
    println!("{}", message);
    gemini_request.set_message(message);
    let response = send_message_to_gemini(&client, &mut gemini_request).await?;
    println!("\nResponse from Gemini: {}", response);

    Ok(())
*/
}
