use clap::{Parser, Subcommand};

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

pub fn parse_cmd(args: Args) {
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
}