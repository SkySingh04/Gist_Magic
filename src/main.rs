use clap::Parser;
use  reqwest::Error;
use gist_magic_lib::commands::Args;
use logger_rust::*;


use gist_magic_lib::config::get_config;
use gist_magic_lib::commands::parse_cmd;


#[tokio::main]
async fn main() -> Result<(), Error> {

    let github_token = match get_config(&mut String::new()) {
        Ok(token) => token,
        Err(e) => {
            log_error!("Error: {}", e);
            String::new()
        }
    };

    let args = Args::parse();
    parse_cmd(args, &github_token).await; // Await the parse_cmd function call.


  Ok(())
}
