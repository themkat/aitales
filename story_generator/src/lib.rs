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

    pub async fn generate(&self) {
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

        // some printing because openai api sometimes behaved weird...
        println!("genre: {}", &selected_genre);
        println!("theme: {}", &selected_theme);
        println!("detail: {}", &selected_detail);

        // generation of our text, title and cover image
        let story_prompt = create_story_prompt_string(&story_details);
        let story_text = openai::do_chat_request(&self.token, &[story_prompt.clone()])
            .await
            .expect("no story...");
        let story_title = openai::do_chat_request(
            &self.token,
            &[
                story_prompt.clone(),
                story_text.clone(),
                "Suggest a title for the story above".to_string(),
            ],
        )
        .await
        .expect("no title...");

        // process the genre as well. Sometimes something is generated that fits better in other genres
        let story_refined_genres = openai::do_chat_request(
            &self.token,
            &[
                story_prompt.clone(),
                story_text.clone(),
                "Give a comma seperated list of maximum 3 genres for the story above".to_string(),
            ],
        )
        .await
        .expect("Could not get genres")
        .to_lowercase()
        .replace('.', "");

        println!("Refined genres: {story_refined_genres}");

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
        let mut story_genre_file = File::create("story_genre.txt")
            .await
            .expect("Could not create genre file!");

        // TODO: should probably export the genre(s) selected to a file as well. That way we can use them when generating md files

        let (w1, w2, w3, w4) = join!(
            story_text_file.write_all(story_text.as_bytes()),
            story_title_file.write_all(story_title.as_bytes()),
            story_image_url_file.write_all(story_image_url.as_bytes()),
            story_genre_file.write_all(story_refined_genres.as_bytes())
        );
        w1.expect("Coult not write file!");
        w2.expect("Coult not write file!");
        w3.expect("Coult not write file!");
        w4.expect("Coult not write file!");
    }

    pub async fn sequelize(&self, story_file: &String) {
        // TODO: should we take into account that there might be a chain of prequels? Or just assume
        let story_text = tokio::fs::read_to_string(story_file)
            .await
            .expect("File not found!");

        let sequel_text = openai::do_chat_request(
            &self.token,
            &[
                story_text,
                String::new(),
                "Generate a sequel to the story above. Should be at least 1000 words".to_string(),
            ],
        )
        .await
        .expect("No sequel generated. Probably server issue");

        let sequel_title = openai::do_chat_request(
            &self.token,
            &[
                sequel_text.clone(),
                String::new(),
                "Suggest a title for the story above".to_string(),
            ],
        )
        .await
        .expect("no title...");

        let sequel_genres = openai::do_chat_request(
            &self.token,
            &[
                sequel_text.clone(),
                String::new(),
                "Give a comma seperated list of maximum 3 genres for the story above".to_string(),
            ],
        )
        .await
        .expect("Could not get genres")
        .to_lowercase()
        .replace('.', "");

        // TODO: what is the best way to generate the image?
        //       make chatgpt generate a prompt for us? how? "Describe the setting in 4 sentences"? or will that be too boring? sending in the title?
        let sequel_image_url = openai::do_image_generation_request(&self.token, &sequel_title)
            .await
            .expect("no image fetched...");

        // write the results to file so the user (github actions) can use the data
        // TODO: should the file writing above be extracted to a method to avoid duplication maybe :P
        let mut sequel_text_file = File::create("sequel_text.txt")
            .await
            .expect("Could not create sequel text file!");
        let mut sequel_title_file = File::create("sequel_title.txt")
            .await
            .expect("Could not create sequel title file!");
        let mut sequel_image_url_file = File::create("sequel_image_url.txt")
            .await
            .expect("Could not create image url file!");
        let mut sequel_genre_file = File::create("sequel_genre.txt")
            .await
            .expect("Could not create genre file!");

        // TODO: should probably export the genre(s) selected to a file as well. That way we can use them when generating md files

        let (w1, w2, w3, w4) = join!(
            sequel_text_file.write_all(sequel_text.as_bytes()),
            sequel_title_file.write_all(sequel_title.as_bytes()),
            sequel_image_url_file.write_all(sequel_image_url.as_bytes()),
            sequel_genre_file.write_all(sequel_genres.as_bytes())
        );
        w1.expect("Coult not write file!");
        w2.expect("Coult not write file!");
        w3.expect("Coult not write file!");
        w4.expect("Coult not write file!");
    }
}

fn create_story_prompt_string(story_details: &StoryDetails) -> String {
    format!(
        "Write a {} 800 to 1400 word story about {}. {}",
        story_details.genre, story_details.theme, story_details.extra_detail
    )
}

#[deprecated]
#[allow(dead_code)]
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
            "Write a thriller 800 to 1400 word story about a secret agent tracking down a smuggler. ",
            "Write a drama 800 to 1400 word story about someone inheriting wealth. People should smile",
            "Write a diddly doo 800 to 1400 word story about doodely diddely. Okayley dokeley",
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
