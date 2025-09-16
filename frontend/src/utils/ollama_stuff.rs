use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRoleMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequestBody {
    model: String,
    messages: Vec<ChatRoleMessage>,
    stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponseBody {
    pub model: String,
    pub message: ChatRoleMessage,
}

#[derive(Clone)]
pub struct OllamaClient {
    url: String,
    client: Client,
}

#[derive(Clone, Serialize)]
pub struct OllamaModelPull {
    model: String,
    stream: bool,
}

impl OllamaClient {
    pub fn new(client: Client, url: String) -> Self {
        Self { url, client }
    }

    pub async fn pull_model(&self, name: &str) -> Result<Response, reqwest::Error> {
        self.client
            .post(format!("{}/api/pull", self.url))
            .json(&OllamaModelPull {
                model: name.into(),
                stream: false,
            })
            .send()
            .await
    }

    pub async fn chat(
        self,
        text: String,
        model: &str,
        messages: &mut Vec<ChatRoleMessage>,
    ) -> Result<Response, reqwest::Error> {
        messages.push(ChatRoleMessage {
            role: "user".into(),
            content: text,
        });

        self.client
            .post(format!("{}/api/chat", self.url))
            .json(&ChatRequestBody {
                model: model.into(),
                messages: messages.clone(),
                stream: false,
            })
            .send()
            .await
    }
}
