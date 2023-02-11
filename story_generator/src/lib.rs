use rand::Rng;
use serde::{Deserialize, Serialize};

// helper objects to avoid having to fiddle with strings
struct StoryDetails {
    genre: String,
    theme: String,
    extra_detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorConfig {
    genres: Vec<String>,
    themes: Vec<String>,
    extra_details: Vec<String>,
}

pub struct GeneratorApp {
    token: String,
    config: GeneratorConfig,
}

impl GeneratorApp {
    // TODO: a bit ugly with these two params. might fix
    pub fn new(token: String, config: GeneratorConfig) -> Self {
        Self { token, config }
    }

    pub async fn run(&self) {
        // choose randomly the information we want for each of the config options
        let genres = &self.config.genres;
        let themes = &self.config.themes;
        let extra_details = &self.config.extra_details;

        let selected_genre = genres
            .get(rand::thread_rng().gen_range(0..genres.len()))
            .expect("Could not select random genre!");
        let selected_theme = themes
            .get(rand::thread_rng().gen_range(0..themes.len()))
            .expect("Could not select random genre!");
        let selected_detail = extra_details
            .get(rand::thread_rng().gen_range(0..extra_details.len()))
            .expect("Could not select random genre!");
        let story_details = StoryDetails {
            genre: selected_genre.to_string(),
            theme: selected_theme.to_string(),
            extra_detail: selected_detail.to_string(),
        };

        // basic algorithm:
        // - fetch a story based upon parameters.
        // - fetch a descriptive image
        // - fetch a descriptive title somehow

        // TODO: what should be returned here? should we write to file?
        // separate export options struct for these settings?

        let story_text = fetch_story(&self.token, &story_details)
            .await
            .expect("no story...");

        println!("Text: {}", story_text)
    }
}

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
    // TODO: do we need the other fields?
    choices: Vec<OpenAiCompletionChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiCompletionChoice {
    text: String,
}

// TODO: should we have this as a separate function or inside generator?
async fn fetch_story(
    api_token: &String,
    story_details: &StoryDetails,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let request = OpenAiCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: create_prompt_string(story_details),
        temperature: (rand::thread_rng().gen_range(0..9) as f32) / 10.0,
        max_tokens: 3500,
    };

    let result = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_token))
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

// TODO: separate methods outside for generating story query etc?
fn create_prompt_string(story_details: &StoryDetails) -> String {
    format!(
        "Write a page long {} about {}. {}",
        story_details.genre, story_details.theme, story_details.extra_detail
    )
}

#[cfg(test)]
mod tests {
    use crate::{create_prompt_string, StoryDetails};

    #[test]
    fn test_create_prompt_string() {
        let story_details = vec![
            StoryDetails {
                genre: "thriller".to_string(),
                theme: "a secret agent tracking down a smuggler".to_string(),
                extra_detail: String::new(),
            },
            StoryDetails {
                genre: "drama".to_string(),
                theme: "someone inheriting wealth".to_string(),
                extra_detail: "People should smile".to_string(),
            },
            StoryDetails {
                genre: "diddly doo".to_string(),
                theme: "doodely diddely".to_string(),
                extra_detail: "Okayley dokeley".to_string(),
            },
        ];

        let expected_prompts = vec![
            "Write a thriller about a secret agent tracking down a smuggler. ",
            "Write a drama about someone inheriting wealth. People should smile",
            "Write a diddly doo about doodely diddely. Okayley dokeley",
        ];

        let results: Vec<String> = story_details
            .iter()
            .map(|elem| create_prompt_string(elem))
            .collect();

        assert_eq!(results, expected_prompts);
    }
}
