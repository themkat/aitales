use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let openAiToken =
        env::var("OPENAI_TOKEN").expect("OpenAI token needs to be defined to be able to use APIs!");

    Ok(())
}
