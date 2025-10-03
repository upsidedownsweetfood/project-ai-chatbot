mod api;
mod gemini;
use actix_web::{App, HttpServer, web};
use api::input_message;
use gemini::{GeminiRequest, GeminiRequestArcMutex, create_client};
use std::{error::Error, sync::Arc};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = create_client().await?;
    let prompt = "You are a helpful assistant.";
    let gemini_request = GeminiRequestArcMutex(Arc::new(std::sync::Mutex::new(
        GeminiRequest::new(client, prompt),
    )));

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(gemini_request.clone()))
            .service(input_message)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
