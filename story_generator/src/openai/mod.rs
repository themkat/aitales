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
