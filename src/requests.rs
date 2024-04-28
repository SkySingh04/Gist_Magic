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

// pub async fn create_gist(request_url: &str , github_token:&str , description: &str , filename: &str , content: String) -> Result<Gist, Error> {
//     // let mut files = std::collections::HashMap::new();
//     // files.insert(filename, reqwest::multipart::Part::text(content));
//     // let mut payload = std::collections::HashMap::new();
//     // payload.insert("description", description);
//     // payload.insert("public", "true");
//     // payload.insert("files", serde_json::to_value(&files).unwrap().as_object().unwrap());
//     // let response = reqwest::Client::new()
//     //     .post(request_url)
//     //     .header("User-Agent", "Gist_Magic")
//     //     .header("Authorization", format!("token {}", github_token))
//     //     .json(&payload)
//     //     .send()
//     //     .await?;

//     //     response.error_for_status_ref()?;
    
//     //     let gist: Gist = response.json().await
//     //     .map_err(|e| Error::from(e))?;

//     // Ok(gist)
// }