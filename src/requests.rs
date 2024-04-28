// src/requests.rs
use reqwest::Error;
use crate::models::Gist;
use crate::models::GistFile;
use crate::models::GistPayload;


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

pub async fn edit_gist(request_url: &str , github_token:&str , description: &str , filename: &str , content: &str) -> Result<Gist, Error> {
    let mut files = std::collections::HashMap::new();

    files.insert(filename.to_owned(), GistFile { content: content.to_owned() });
    let payload = GistPayload {
        description: description.to_owned(),
        public: true,
        files,
    }; 

    let response = reqwest::Client::new()
        .patch(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .json(&payload)
        .send()
        .await?;

    response.error_for_status_ref()?;
    
    let gist: Gist = response.json().await
        .map_err(|e| Error::from(e))?;

    Ok(gist)
}

pub async fn delete_gist(request_url: &str , github_token:&str) -> Result<(), Error> {
    let response = reqwest::Client::new()
        .delete(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?;

        response.error_for_status_ref()?;
    
    Ok(())
}

pub async fn star_gist(request_url: &str , github_token:&str) -> Result<(), Error> {
    let response = reqwest::Client::new()
        .put(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?;

        response.error_for_status_ref()?;
    
    Ok(())
}

pub async fn unstar_gist(request_url: &str , github_token:&str) -> Result<(), Error> {
    let response = reqwest::Client::new()
        .delete(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .send()
        .await?;

        response.error_for_status_ref()?;
    
    Ok(())
}




pub async fn create_gist(request_url: &str , github_token:&str , description: &str , filename: &str , content: &str) -> Result<Gist, Error> {
    let mut files = std::collections::HashMap::new();

    files.insert(filename.to_owned(), GistFile { content: content.to_owned() });
    let payload = GistPayload {
        description: description.to_owned(),
        public: true,
        files,
    }; 

    let response = reqwest::Client::new()
        .post(request_url)
        .header("User-Agent", "Gist_Magic")
        .header("Authorization", format!("token {}", github_token))
        .json(&payload)
        .send()
        .await?;

    response.error_for_status_ref()?;
    
    let gist: Gist = response.json().await
        .map_err(|e| Error::from(e))?;

    Ok(gist)
}
