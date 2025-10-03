mod api;
mod gemini;
use actix_web::{App, HttpServer, web};
use api::input_message;
use gemini::{GeminiRequest, GeminiRequestArcMutex, create_client};
use std::{error::Error, net::TcpListener, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = format!("localhost:0");
    let listener = TcpListener::bind(address)?;
    let client = create_client().await?;
    let prompt = "You are a helpful assistant.";
    let gemini_request = GeminiRequestArcMutex(Arc::new(std::sync::Mutex::new(
        GeminiRequest::new(client, prompt),
    )));

    println!("Server running at {}", listener.local_addr()?);
    HttpServer::new(move || {
        App::new()
            .app_data(gemini_request.clone())
            .route("/message", web::post().to(input_message))
    })
    .listen(listener)?
    .on_connect(|_conn, _data| {
        println!("New connection established");
    })
    .run()
    .await?;

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
