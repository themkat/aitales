use serde::{Deserialize, Serialize};

// helper objects to avoid having to fiddle with strings
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorConfig {
    genres: Vec<String>,
    themes: Vec<String>,
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
        // TODO: assert the correct input we need somehow

        // TODO: do the steps.

        // choose randomly the information we want for each of the config options

        // basic algorithm:
        // - fetch a story based upon parameters.
    }
}

#[cfg(test)]
mod tests {}
