mod openai;

use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt, join};

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

        // TODO: handle async better

        // generation of our text, title and cover image
        let story_text =
            openai::do_completion_request(&self.token, &create_story_prompt_string(&story_details))
                .await
                .expect("no story...");
        let story_title =
            openai::do_completion_request(&self.token, &create_title_prompt(&story_details))
                .await
                .expect("no title...");
        let story_image_url = openai::do_image_generation_request(
            &self.token,
            &create_image_generation_prompt(&story_details),
        )
        .await
        .expect("no image fetched...");

        // write the results to file so the user (github actions) can use the data
        // TODO: maybe have these as arguments?
        let mut story_text_file = File::create("story_text.txt")
            .await
            .expect("Could not create story text file!");
        let mut story_title_file = File::create("story_title.txt")
            .await
            .expect("Could not create story title file!");
        let mut story_image_url_file = File::create("story_image_url.txt")
            .await
            .expect("Could not create image url file!");

        let (w1, w2, w3) = join!(
            story_text_file.write_all(story_text.as_bytes()),
            story_title_file.write_all(story_title.as_bytes()),
            story_image_url_file.write_all(story_image_url.as_bytes())
        );
        w1.expect("Coult not write file!");
        w2.expect("Coult not write file!");
        w3.expect("Coult not write file!");
    }
}

fn create_story_prompt_string(story_details: &StoryDetails) -> String {
    format!(
        "Write a {} page long story about {}. {}",
        story_details.genre, story_details.theme, story_details.extra_detail
    )
}

fn create_title_prompt(story_details: &StoryDetails) -> String {
    format!(
        "Suggest a title for a {} about {}",
        story_details.genre, story_details.theme
    )
}

fn create_image_generation_prompt(story_details: &StoryDetails) -> String {
    format!(
        "{} in the style of {}",
        story_details.theme, story_details.genre
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        create_image_generation_prompt, create_story_prompt_string, create_title_prompt,
        StoryDetails,
    };

    #[test]
    fn test_create_story_prompt_string() {
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
            "Write a thriller page long story about a secret agent tracking down a smuggler. ",
            "Write a drama page long story about someone inheriting wealth. People should smile",
            "Write a diddly doo page long story about doodely diddely. Okayley dokeley",
        ];

        let results: Vec<String> = story_details
            .iter()
            .map(|elem| create_story_prompt_string(elem))
            .collect();

        assert_eq!(results, expected_prompts);
    }

    #[test]
    fn test_create_title_prompt() {
        let story_details = vec![
            StoryDetails {
                genre: "comedy".to_string(),
                theme: "a guy".to_string(),
                extra_detail: String::new(),
            },
            StoryDetails {
                genre: "drama".to_string(),
                theme: "family conflicts".to_string(),
                extra_detail: "plsdontincludeme".to_string(),
            },
            StoryDetails {
                genre: "diddly doo".to_string(),
                theme: "doodely diddely".to_string(),
                extra_detail: "Okayley dokeley".to_string(),
            },
        ];

        let expected_prompts = vec![
            "Suggest a title for a comedy about a guy",
            "Suggest a title for a drama about family conflicts",
            "Suggest a title for a diddly doo about doodely diddely",
        ];

        let results: Vec<String> = story_details
            .iter()
            .map(|elem| create_title_prompt(elem))
            .collect();

        assert_eq!(results, expected_prompts);
    }

    #[test]
    fn test_create_image_generation_prompt() {
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
            "a secret agent tracking down a smuggler in the style of thriller",
            "someone inheriting wealth in the style of drama",
            "doodely diddely in the style of diddly doo",
        ];

        let results: Vec<String> = story_details
            .iter()
            .map(|elem| create_image_generation_prompt(elem))
            .collect();

        assert_eq!(results, expected_prompts);
    }
}
