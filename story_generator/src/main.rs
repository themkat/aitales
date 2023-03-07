use std::{env, fs::File};

use clap::{Parser, Subcommand};
use story_generator::{GeneratorApp, GeneratorConfig};

#[derive(Parser)]
#[command(author = "themkat")]
#[command(about = "Simple program for generating AI stories.")]
struct CliSettings {
    #[command(subcommand)]
    command: CliCommand,
    // TODO: possibility to override default configuration file?
}

#[derive(Subcommand)]
enum CliCommand {
    #[command(about = "Generate a new story from scratch, given the property file")]
    Generate,
    #[command(about = "Generate a sequel to an existing story")]
    Sequelize { story_file: Option<String> },
}

#[tokio::main]
async fn main() {
    let cli_settings = CliSettings::parse();

    let openai_token =
        env::var("OPENAI_TOKEN").expect("OpenAI token needs to be defined to be able to use APIs!");
    let config_file = File::open("generator_conf.yml")
        .expect("generator_conf.yml should be present in the same directory as the executable!");
    let app_config: GeneratorConfig = serde_yaml::from_reader(config_file)
        .expect("Could not parse yaml config file! Make sure it is correctly formatted");

    let app = GeneratorApp::new(openai_token, app_config);

    match &cli_settings.command {
        CliCommand::Generate => {
            app.generate().await;
        }
        CliCommand::Sequelize { story_file } => {
            // TODO: fix this. Probably quick and dirty with clone because I'm tired
            app.sequelize(&story_file.clone().expect("Story file should be present!"))
                .await;
        }
    }
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
