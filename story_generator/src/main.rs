use std::{env, fs::File};

use story_generator::{GeneratorApp, GeneratorConfig};

#[tokio::main]
async fn main() {
    let openai_token =
        env::var("OPENAI_TOKEN").expect("OpenAI token needs to be defined to be able to use APIs!");
    let config_file = File::open("generator_conf.yml")
        .expect("generator_conf.yml should be present in the same directory as the executable!");
    let app_config: GeneratorConfig = serde_yaml::from_reader(config_file)
        .expect("Could not parse yaml config file! Make sure it is correctly formatted");

    let app = GeneratorApp::new(openai_token, app_config);
    app.run().await;
}

// TODO: maybe stupid? very unconventional probably
// I only use one default yaml at the moment. that one should be correct. Basically veryfying production setup :P
#[cfg(test)]
mod tests {
    use std::fs::File;

    use story_generator::GeneratorConfig;

    #[test]
    fn default_yaml_correctness() -> Result<(), serde_yaml::Error> {
        let config_file =
            File::open("generator_conf.yml").expect("generator_conf.yml does not exist");
        let _app_config: GeneratorConfig = serde_yaml::from_reader(config_file)?;

        Ok(())
    }
}
