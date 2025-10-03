pub use actix_web::{HttpRequest, HttpResponse, web};

use crate::gemini::{GeminiRequestArcMutex, send_message_to_gemini};

#[allow(clippy::async_yields_async)]
pub async fn input_message(
    req: HttpRequest,
    gemini_request: web::Data<GeminiRequestArcMutex>,
) -> HttpResponse {
    println!("Received request: {:?}", req);

    let message = req
        .match_info()
        .get("message")
        .unwrap_or("Hello, I am Francesco");

    println!("Message: {}", message);

    let mut gemini_request = gemini_request.0.lock().unwrap();
    gemini_request.set_message(message);

    let response = send_message_to_gemini(&mut gemini_request)
        .await
        .unwrap_or_else(|e| format!("Error sending message to Gemini: {}", e.to_string()));

    println!("\nResponse from Gemini: {}", response);

    HttpResponse::Ok().body(response)
}
