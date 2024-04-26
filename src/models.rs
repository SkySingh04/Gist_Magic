use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub login: String,
}
#[derive(Deserialize, Debug)]
pub struct Gist {
    pub id: String,
    pub description: Option<String>,
    pub owner: Owner,
}
#[derive(Deserialize, Debug)]
pub struct Owner {
    pub login: String,
}

// Define the structure of the configuration file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub github_token: Option<String>,
}
