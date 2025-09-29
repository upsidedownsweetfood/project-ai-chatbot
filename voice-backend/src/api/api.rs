pub use actix_web::{ web, HttpResponse, HttpRequest };

use crate::gemini::{send_message_to_gemini, GeminiRequest};

#[allow(clippy::async_yields_async)]
pub async fn input_message(
    req: HttpRequest,
    gemini_request: web::Data<GeminiRequest>
) -> HttpResponse {

    let message = req.match_info().get("message").unwrap_or("Hello, I am Francesco");

    gemini_request.set_message(message);

    let response = send_message_to_gemini(&mut gemini_request).await.unwrap_or_else(|e| {
            format!("Error sending message to Gemini: {}", e.to_string())
        });

    println!("\nResponse from Gemini: {}", response);

    HttpResponse::Ok().body(response)
}
