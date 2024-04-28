use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


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
    pub files: Value,
}

#[derive(Deserialize, Debug)]
pub struct Files {
    pub filename: String,
    pub file_type: String,
    pub language: String,
    pub raw_url: String,
    pub size: u32,
    pub truncated: bool,
    pub content: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFile {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistPayload {
    pub description: String,
    pub public: bool,
    pub files: std::collections::HashMap<String, GistFile>,
}