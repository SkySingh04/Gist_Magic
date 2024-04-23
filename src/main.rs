use serde::Deserialize;
use reqwest::Error;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize , Debug)]
struct User {
    id: u32,
    login: String,
}

//TODO: Add Logger to handle different log levels
//TODO: Connect to CLI to read flags for owner and repo
//TODO: Add different commands for different operations





#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found in .env file");
    
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers", 
   owner ="Akash-Singh04" , repo="QuizQuest");

   println!( "Request URL: {}", request_url);

    let response = reqwest::Client::new()
        .get(&request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", &github_token)
        .send()
        .await?;

    if response.status().is_success() {
        let users: Vec<User> = response.json().await?;
        println!("{:?}", users);
    
    } else {
        println!("Request Failed with Status: {:?}", response.status());
    }

    Ok(())

    // println!("Hello, world!
}
