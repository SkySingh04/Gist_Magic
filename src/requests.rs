// src/requests.rs
use reqwest::Error;
use crate::models::User;
use crate::models::Gist;




pub async fn fetch_stargazers(request_url: &str , github_token:&str) -> Result<Vec<User>, Error> {
    
    let response = reqwest::Client::new()
        .get(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", github_token)
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    //print the users
    for user in &users {
        println!("ID: {}", user.id);
        println!("Login: {}", user.login);
        println!("------------------------");
    }
    Ok(users)
}

pub async fn fetch_gists(request_url: &str , github_token:&str) -> Result<Vec<Gist>, Error> {

    let response = reqwest::Client::new()
        .get(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?;

        let gists: Vec<Gist> = response.json().await?;
        Ok(gists)
       
    
}