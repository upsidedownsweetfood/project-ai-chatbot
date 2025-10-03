pub use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::gemini::{GeminiRequestArcMutex, send_message_to_gemini};

#[derive(Debug, Deserialize)]
pub struct FrontendMessage {
    pub message: String,
}

#[post("/message")]
pub async fn input_message(
    req: web::Json<FrontendMessage>,
    gemini_request: web::Data<GeminiRequestArcMutex>,
) -> HttpResponse {
    let message = req.message.as_str();

    println!("MESSAGE FROM USER:\n {}\n", message);

    let mut gemini_request = gemini_request.0.lock().unwrap();
    gemini_request.set_message(message);

    let response = send_message_to_gemini(&mut gemini_request)
        .await
        .unwrap_or_else(|e| format!("Error sending message to Gemini: {}\n", e.to_string()));

    println!("RESPONSE FROM GEMINI:\n{}\n", response);

    HttpResponse::Ok().body(response)
}
