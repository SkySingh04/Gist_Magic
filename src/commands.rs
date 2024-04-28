use clap::{Parser, Subcommand};
// use serde::de;
use crate::requests::{fetch_gists , view_gist ,delete_gist , star_gist , unstar_gist , create_gist , edit_gist};
use textwrap::fill;
use logger_rust::*;
use crate::models::{ GistFile , GistPayload};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List all gists")]
    List {
    #[arg(long, short = 'o', help = "Username of the owner of gists to list")]
    username: Option<String>
    },
    #[command(about = "Create a new gist")]
    Create{
    #[arg(long, short = 'f', help = "File to create a gist from")]
    filepath: Option<String>,
    #[arg(long, short = 's', help = "Starting line of the file to create a gist from")]
    start: Option<usize>,
    #[arg(long, short = 'e', help = "Ending line of the file to create a gist from")]
    end: Option<usize>,
    },
    #[command(about = "Edit a gist")]
    Edit{
    #[arg(long, short = 'g' , help = "ID of the gist to list stargazers for")]
    gistid: String,
},
    #[command(about = "Delete a gist")]
    Delete{
    #[arg(long, short = 'g' , help = "ID of the gist to be deleted")]
    gistid: String,
    },
    #[command(about = "View a gist")]
    View{
    #[arg(long, short = 'g' , help = "ID of the gist to view")]
    gistid: String,
},
    #[command(about = "Star a gist")]
    Star{
    #[arg(long, short = 'g' , help = "ID of the gist to star")]
    gistid: String,
    },
    #[command(about = "Unstar a gist")]
    Unstar{
    #[arg(long, short = 'g' , help = "ID of the gist to unstar")]
    gistid: String,
    },
}

pub async fn  parse_cmd(args: Args , github_token: &str) {
    let mut request_url:  String = "https://api.github.com/gists".to_string();

    match args.cmd{
        Commands::List{username} => {
            if let Some(owner) = username {
                log_info!("Listing gists for user: {}", owner);
                let formatted_url = format!("https://api.github.com/users/{}/gists", owner);
                request_url = formatted_url;
            }
            match fetch_gists(&request_url  , &github_token).await {
                Ok(gists) => {
                    for gist in gists {
                      println!("ID: {}", gist.id);
                      println!("Description: {:?}", gist.description);
                      println!("Owner: {}", gist.owner.login);
                      println!("------------------------");
                    }
                    log_info!("To view a gist, use the view command with the --gistid flag.")
                },
                Err(e) => {
                    log_error!("Error: {}", e);
                }
          }
        },
        Commands::Create{filepath ,start, end} => {
            //fix this one where file is read, returning 422 error
            let mut filename = String::new();
            let mut content = String::new();
            if let Some(file) = filepath {
                filename = file;
                content = std::fs::read_to_string(&filename).expect("Could not read file");
                if start.is_some() && end.is_some() {
                    let start = start.unwrap();
                    let end = end.unwrap();
                    content = content.lines().skip(start - 1).take(end - start + 1).collect::<Vec<&str>>().join("\n");
                }
                if content.is_empty() {
                    log_error!("File is empty");
                    return;
                }
                println!("Enter the description of the gist: ");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).expect("Could not read line");
                let description = description.trim();
                filename = filename.split("/").last().unwrap().to_string();
                match create_gist(&request_url , &github_token , description , &filename , &content).await {
                    Ok(gist) => {
                        log_info!("Gist created successfully with id: {}", gist.id);
                    },
                    Err(e) => {
                        log_error!("Error: {}", e);
                    }
                }


            }
            else{
                log_info!("Enter the filename:");
                std::io::stdin().read_line(&mut filename).expect("Could not read line");
                let filename = filename.trim();
                if filename.is_empty() {
                    log_error!("Filename cannot be empty");
                    return;
                }
                let template = "Enter the content of the gist:\n";
                let editedtemplate = edit::edit(template);
                match editedtemplate {
                    Ok(editedtemplate) => {
                        content = editedtemplate;
                    },
                    Err(e) => {
                        log_error!("Error: {}", e);
                    }
                }
                //get the description of the gist from user
                println!("Enter the description of the gist: ");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).expect("Could not read line");
                let description = description.trim();

                match create_gist(&request_url , &github_token , description , filename , &content).await {
                    Ok(gist) => {
                        log_info!("Gist created successfully  with id: {}" , gist.id);
                    },
                    Err(e) => {
                        log_error!("Error: {}", e);
                    }
                }
            }
        },
        Commands::Edit{gistid} => {
            log_info!("Editing a gist with id: {}", gistid);
            let formatted_url = format!("https://api.github.com/gists/{}", gistid);
            request_url = formatted_url;
            match view_gist(&request_url , &github_token).await {
                Ok(gist) => {
                    let mut files = std::collections::HashMap::new();
                    for (filename, file) in gist.files.as_object().unwrap() {
                        let mut content = file["content"].as_str().unwrap().to_string();
                        let template = format!("Editing file: {}\n{}", filename, content);
                        let editedtemplate = edit::edit(&template);
                        match editedtemplate {
                            Ok(editedtemplate) => {
                                content = editedtemplate;
                            },
                            Err(e) => {
                                log_error!("Error: {}", e);
                            }
                        }
                        files.insert(filename.to_string(), GistFile { content });
                    }
                    let payload = GistPayload {
                        description: gist.description.unwrap_or("".to_string()),
                        public: true,
                        files,
                    };
                    let filename = gist.files.as_object().unwrap().keys().next().unwrap();
                    match edit_gist(&request_url , &github_token , &payload.description , filename , &payload.files[filename].content).await {
                        Ok(gist) => {
                            log_info!("Gist edited successfully with id: {}", gist.id);
                        },
                        Err(e) => {
                            log_error!("Error: {}", e);
                        }
                    }
                },
                Err(e) => {
                    log_error!("Error: {}", e);
                }
            }
        },
        Commands::Delete{gistid} => {
            log_warn!("Deleting a gist with id: {}", gistid);
            println!("Enter 'y' to confirm deletion: ");
            let mut confirm = String::new();
            std::io::stdin().read_line(&mut confirm).expect("Could not read line");
            let confirm = confirm.trim();
            if confirm == "y" {
                request_url = format!("https://api.github.com/gists/{}", gistid);
                match delete_gist(&request_url , &github_token ).await {
                    Ok(_) => {
                        log_info!("Gist deleted successfully");
                    },
                    Err(e) => {
                        log_error!("Error while deleting gist: {}", e);
                    }
                }
            } else {
                println!("Deletion cancelled");
            }

            
        },
        Commands::View{gistid  }  => {
            let formatted_url : String = format!("https://api.github.com/gists/{}", gistid);
            request_url = formatted_url;
            match view_gist(&request_url , &github_token).await {
                Ok(gist) => {
                    println!("ID: {}", gist.id);
                    println!("Description: {:?}", gist.description);
                    println!("Owner: {}", gist.owner.login);
                    println!("Gist Files: ");
                    if let Some(files) = gist.files.as_object() {
                        for (filename, file) in files {
                            println!("Filename: {}", filename);
                            println!("Language: {}", file["language"]);
                            println!("Size: {} bytes", file["size"]);
                            println!("Raw URL: {}", file["raw_url"]);
                            println!("-------------------------");
                            println!("Content:");
                            let wrapped_content = fill(file["content"].as_str().unwrap_or(""), 80);
                            println!("{}", wrapped_content);
                            println!("-------------------------");
                        }
                    }
                },
                Err(e) => {
                    log_error!("Error: {}", e);
                    ()
                }
            }
        },
        Commands::Star{gistid} => {
            log_info!("Starring gist with id: {}", gistid);
            request_url = format!("https://api.github.com/gists/{}/star", gistid);
            match star_gist(&request_url , &github_token).await {
                Ok(_) => {
                    log_info!("Gist starred successfully");
                },
                Err(e) => {
                    log_error!("Error while starring gist: {}", e);
                }
            }
        },
        Commands::Unstar{gistid} => {
            log_info!("Unstarring gist with id: {}", gistid);
            request_url = format!("https://api.github.com/gists/{}/star", gistid);
            match unstar_gist(&request_url , &github_token).await {
                Ok(_) => {
                    log_info!("Gist unstarred successfully");
                },
                Err(e) => {
                    log_error!("Error while unstarring gist: {}", e);
                }
        }
    }
}
}