use clap::{Parser, Subcommand};
use crate::requests::{fetch_gists , view_gist}; 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Args {
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
    Edit{
    #[arg(long, short = 'g' , help = "ID of the gist to list stargazers for")]
    gistid: String,
},
    #[command(about = "Delete a gist")]
    Delete,
    #[command(about = "View a gist")]
    View{
    #[arg(long, short = 'g' , help = "ID of the gist to list stargazers for")]
    gistid: String,
},
    #[command(about = "Star a gist")]
    Star,
    #[command(about = "Unstar a gist")]
    Unstar,
}

pub async fn  parse_cmd(args: Args , github_token: &str) {
    let mut request_url: &str = "https://api.github.com/gists";

    match args.cmd{
        Commands::List => {
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
        },
        Commands::Create => {
            println!("Create a new gist");
        },
        Commands::Edit{gistid} => {
            println!("Edit a gist {}" , gistid);
        },
        Commands::Delete => {
            println!("Delete a gist");
        },
        Commands::View{gistid}  => {
            let formatted_url = format!("https://api.github.com/gists/{}", gistid);
            request_url = &formatted_url;
            match view_gist(&request_url , &github_token).await {
                Ok(gist) => {
                    println!("ID: {}", gist.id);
                    println!("Description: {:?}", gist.description);
                    println!("Owner: {}", gist.owner.login);
                    println!("Gist Files: ");
                    if let Some(filename) = gist.files.get("filename").and_then(|v| v.as_str()) {
                        println!("Filename: {}", filename);
                    }
                    println!("File type: {}", gist.files.get("file_type").and_then(|v| v.as_str()).unwrap_or(""));
                    println!("Language: {}", gist.files.get("language").and_then(|v| v.as_str()).unwrap_or(""));
                    println!("Raw URL: {}", gist.files.get("raw_url").and_then(|v| v.as_str()).unwrap_or(""));
                    println!("Size: {}", gist.files.get("size").and_then(|v| v.as_str()).unwrap_or(""));
                    println!("Truncated: {}", gist.files.get("truncated").and_then(|v| v.as_bool()).unwrap_or(false));
                    println!("Content: {}", gist.files.get("content").and_then(|v| v.as_str()).unwrap_or(""));
                    println!("------------------------");
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        },
        Commands::Star => {
            println!("Star a gist");
        },
        Commands::Unstar => {
            println!("Unstar a gist");
        }
    }
}