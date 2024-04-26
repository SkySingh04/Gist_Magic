use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::models::Config;
use logger_rust::*;




pub fn get_config(github_token: &mut String) -> Result<String, Box<dyn std::error::Error>> {
    // Define the path to the configuration file in the user's home directory
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            log_error!("Failed to get user's home directory.");
            return Err("Failed to get user's home directory.".into());
        }
    };
    let config_path = home_dir.join(".gist_magic.json");

    // Check if the configuration file exists
    let config = if let Ok(contents) = fs::read_to_string(&config_path) {
        // If the file exists, deserialize the configuration
        match serde_json::from_str::<Config>(&contents) {
            Ok(config) => config,
            Err(err) => {
                log_error!("Error parsing configuration file: {}", err);
                return Err(format!("Error parsing configuration file: {}", err).into());
            }
        }
    } else {
        // If the file doesn't exist, create a new empty configuration
        Config { github_token: None }
    };

    // Check if the GitHub token is present in the configuration
     *github_token = match config.github_token {
        Some(token) => token,
        None => {
            // If the token is not present, prompt the user to input it
            log_warn!("GitHub token is not set.");
            log_info!("Please enter your GitHub token: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            *github_token = input.trim().to_string();

            // Save the GitHub token to the configuration file
            let new_config = Config {
                github_token: Some(github_token.clone()),
            };
            if let Err(err) = save_config(&config_path, &new_config) {
                log_error!("Error saving configuration: {}", err);
                return Err(format!("Error saving configuration: {}", err).into());
            }

            github_token.to_string()
        }
    };

    Ok(github_token.to_string())
}

// Function to save the configuration to the file
fn save_config(path: &Path, config: &Config) -> io::Result<()> {
    let config_str = serde_json::to_string_pretty(config)?;
    fs::write(path, config_str)?;
    Ok(())
}
