use gemini_rust::{Content, Gemini, GenerationResponse, Message, Part, Role};
use std::{
    env,
    error::Error,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct GeminiRequest {
    pub client: Gemini,
    pub prompt: String,
    pub message: String,
    pub context: Vec<Message>,
}

impl GeminiRequest {
    pub fn new(client: Gemini, prompt: &str) -> Self {
        Self {
            client: client,
            prompt: prompt.to_string(),
            message: String::new(),
            context: vec![],
        }
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }
}

#[derive(Clone)]
pub struct GeminiRequestArcMutex(pub Arc<Mutex<GeminiRequest>>);

pub async fn create_client() -> Result<Gemini, Box<dyn Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect(r#"GEMINI_API_KEY must be set in config file"#);
    let client = Gemini::new(api_key)?;
    Ok(client)
}

pub async fn send_message_to_gemini(req: &mut GeminiRequest) -> Result<String, Box<dyn Error>> {
    let response = req
        .client
        .generate_content()
        .with_system_prompt(&req.prompt)
        .with_messages(req.context.clone())
        .with_user_message(&req.message)
        .execute()
        .await?;

    update_context_with_gemini_response(&response, req).await?;

    Ok(response.text().to_string())
}

async fn update_context_with_gemini_response(
    response: &GenerationResponse,
    req: &mut GeminiRequest,
) -> Result<(), Box<dyn Error>> {
    req.context.push(Message {
        role: Role::User,
        content: Content {
            role: Some(Role::User),
            parts: Some(vec![Part::Text {
                text: req.message.clone(),
                thought: None,
                thought_signature: None,
            }]),
        },
    });

    req.context.push(Message {
        role: Role::Model,
        content: Content {
            role: Some(Role::Model),
            parts: Some(vec![Part::Text {
                text: response.text().to_string(),
                thought: None,
                thought_signature: None,
            }]),
        },
    });

    Ok(())
}
