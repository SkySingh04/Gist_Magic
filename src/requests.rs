// src/requests.rs
use reqwest::Error;
use crate::models::Gist;



pub async fn fetch_gists(request_url: &str , github_token:&str) -> Result<Vec<Gist>, Error> {

    let response = reqwest::Client::new()
        .get(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?;

        response.error_for_status_ref()?;

        let gists: Vec<Gist> = response.json().await?;
        Ok(gists)
}

pub async fn view_gist(request_url: &str , github_token:&str) -> Result<Gist, Error> {
    let response = reqwest::Client::new()
        .get(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?;

        response.error_for_status_ref()?;
    
        let gist: Gist = response.json().await
        .map_err(|e| Error::from(e))?;

    Ok(gist)

}

