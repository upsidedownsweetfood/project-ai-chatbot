use gemini_rust::{ Gemini, Message, Part, Role, Content };
use std::{ env, error::Error };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set in .env file");
    let client = Gemini::new(api_key)?;

    let prompt = "You are a helpful assistant.";
    let message = "Hello, I am Francesco";

    let mut context: Vec<Message> = vec![];

    let response = client
        .generate_content()
        .with_system_prompt(prompt)
        .with_user_message(message)
        .execute().await?;

    println!("Response from Gemini: {}", response.text());

    context.push(Message {
        role: Role::User,
        content: Content {
            role: Some(Role::User),
            parts: Some(
                vec![Part::Text {
                    text: message.to_string(),
                    thought: None,
                    thought_signature: None,
                }]
            ),
        },
    });

    context.push(Message {
        role: Role::Model,
        content: Content {
            role: Some(Role::Model),
            parts: Some(
                vec![Part::Text {
                    text: response.text().to_string(),
                    thought: None,
                    thought_signature: None,
                }]
            ),
        },
    });

    let response = client
        .generate_content()
        .with_system_prompt(prompt)
        .with_messages(context.clone())
        .with_user_message("What's my name?")
        .execute().await?;

    println!("Do you remember my name?");
    println!("\nResponse from Gemini: {}", response.text());

    Ok(())
}
