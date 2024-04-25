use reqwest::Error;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List all gists")]
    List,
    #[command(about = "Create a new gist")]
    Create,
    #[command(about = "Edit a gist")]
    Edit,
    #[command(about = "Delete a gist")]
    Delete,
    #[command(about = "View a gist")]
    View,
    #[command(about = "Star a gist")]
    Star,
    #[command(about = "Unstar a gist")]
    Unstar,
    #[command(about = "List all stargazers of a gist")]
    Stargazers,
}

use gist_magic_lib::requests; 




//TODO: Add Logger to handle different log levels
//TODO: Connect to CLI to read flags for owner and repo
//TODO: Add different commands for different operations





#[tokio::main]
async fn main() -> Result<(), Error> {
  let args = Args::parse();
  
  match args.cmd{
      Commands::List => {
        println!("List all gists");
      },
      Commands::Create => {
          println!("Create a new gist");
      },
      Commands::Edit => {
          println!("Edit a gist");
      },
      Commands::Delete => {
          println!("Delete a gist");
      },
      Commands::View => {
          println!("View a gist");
      },
      Commands::Star => {
          println!("Star a gist");
      },
      Commands::Unstar => {
          println!("Unstar a gist");
      },
      Commands::Stargazers => {
          println!("List all stargazers of a gist");
      }
  }
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
