// src/requests.rs
use reqwest::Error;
// use std::str::FromStr;
use dotenv::dotenv;
use std::env;
use crate::models::User;
use crate::models::Gist;




pub async fn fetch_stargazers(request_url: &str) -> Result<Vec<User>, Error> {
    dotenv().ok();
    
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found in .env file");


    let response = reqwest::Client::new()
        .get(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", github_token)
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    Ok(users)
}

pub async fn fetch_gists(request_url: &str) -> Result<Vec<Gist>, Error> {
    dotenv().ok();
    
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found in .env file");

    let response = reqwest::Client::new()
        .get(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?;

        let gists: Vec<Gist> = response.json().await?;
        Ok(gists)
       
    
}