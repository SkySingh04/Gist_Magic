use reqwest::Error;




use gist_magic_lib::requests; 




//TODO: Add Logger to handle different log levels
//TODO: Connect to CLI to read flags for owner and repo
//TODO: Add different commands for different operations





#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let request_url: &str = "https://api.github.com/gists";



  match requests::fetch_gists(&request_url).await {
      Ok(gists) => {
          for gist in gists {
            println!("ID: {}", gist.id);
            println!("Description: {:?}", gist.description);
            println!("Owner: {}", gist.owner.login);
            println!("------------------------");
          }
      },
      Err(e) => {
          eprintln!("Error: {}", e);
      }
}
  Ok(())
}
