use std::time::Duration;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiCompletionRequest {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: i16,
    // other fields?
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiCompletionResponse {
    choices: Vec<OpenAiCompletionChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiCompletionChoice {
    text: String,
}

#[deprecated]
#[allow(dead_code)]
pub async fn do_completion_request(
    api_token: &String,
    prompt: &String,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(720))
        .build()
        .expect("Could not create http client");
    let request = OpenAiCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: prompt.to_string(),
        temperature: (rand::thread_rng().gen_range(0..9) as f32) / 10.0,
        max_tokens: 3500,
    };

    let result = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {api_token}"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request).expect("Could not write json to string"))
        .send()
        .await?
        .json::<OpenAiCompletionResponse>()
        .await?;

    Ok(result
        .choices
        .first()
        .expect("OpenAPI completion API returned no text!")
        .text
        .clone())
}

// image requests
// TODO: should we have some sort of refine operation as well? Or variations? Just generate a set of images at once? An action to do this automagically for PRs would be awesome! Just click a button or something and generate new images!
#[derive(Debug, Serialize, Deserialize)]
struct OpenAiImageGenerationRequest {
    prompt: String,
    size: String,
    response_format: String,
}

// TODO: the response format so we can serialize that one
#[derive(Debug, Serialize, Deserialize)]
struct OpenAiImageGenerationResponse {
    data: Vec<OpenAiImageGenerationData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiImageGenerationData {
    url: String,
}

pub async fn do_image_generation_request(
    api_token: &String,
    prompt: &String,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(720))
        .build()
        .expect("Could not create http client");
    let request = OpenAiImageGenerationRequest {
        prompt: prompt.to_string(),
        size: "1024x1024".to_string(),
        response_format: "url".to_string(),
    };

    let result = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Authorization", format!("Bearer {api_token}"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request).expect("Could not write json to string"))
        .send()
        .await?
        .json::<OpenAiImageGenerationResponse>()
        .await?;

    Ok(result
        .data
        .first()
        .expect("OpenAPI completion API returned no images!")
        .url
        .clone())
}

// ChatGPT API requests
#[derive(Debug, Serialize, Deserialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<OpenAiChatMessage>,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChatChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiChatChoice {
    message: OpenAiChatMessage,
}

pub async fn do_chat_request(
    api_token: &String,
    earlier_messages: &[String],
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(720))
        .build()
        .expect("Could not create http client");

    let request = OpenAiChatRequest {
        model: "gpt-4".to_string(),
        messages: earlier_messages
            .iter()
            .enumerate()
            .map(|(ind, msg)| OpenAiChatMessage {
                // assume that even indices 0,2,4 etc. is the user. The others are the replies/assistant
                role: if ind % 2 == 0 {
                    "user".to_string()
                } else {
                    "assistant".to_string()
                },
                content: msg.to_string(),
            })
            .collect(),
        temperature: (rand::thread_rng().gen_range(0..9) as f32) / 10.0,
    };

    let result = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {api_token}"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request).expect("Could not write json to string"))
        .send()
        .await?
        .json::<OpenAiChatResponse>()
        .await?;

    Ok(result
        .choices
        .first()
        .expect("No chat message in result")
        .message
        .content
        .clone())
}
