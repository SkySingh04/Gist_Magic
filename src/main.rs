use clap::Parser;
use  reqwest::Error;
use gist_magic_lib::commands::Args;

use gist_magic_lib::requests::fetch_gists; 
use gist_magic_lib::config::get_config;
use gist_magic_lib::commands::parse_cmd;

//TODO: Add Logger to handle different log levels
//TODO: Connect to CLI to read flags for owner and repo
//TODO: Add different commands for different operations

#[tokio::main]
async fn main() -> Result<(), Error> {

    let github_token = match get_config(&mut String::new()) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Error: {}", e);
            String::new()
        }
    };

    let args = Args::parse();
    parse_cmd(args);

    print!("GitHub Token: {}", github_token);
    
  
    let request_url: &str = "https://api.github.com/gists";



  match fetch_gists(&request_url  , &github_token).await {
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
