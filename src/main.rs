use std::io::stdout;
use clap::Command;
use clap_complete::{generate, Generator, Shell};
use std::collections::HashMap;
use std::result::Result;
use ansi_term::{Colour, Style};

extern crate serde_json;
extern crate clap;
extern crate clap_complete;

mod record;
mod request;
mod cli;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::build_cli();
    let matches = cli.clone().get_matches();

    /* let binding = cli.clone();
    let positionals = binding.get_positionals(); */

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = cli::build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }

    if let Some(matches) = matches.subcommand_matches("install") {
        if let Some(package) = matches.get_one::<String>("package") {
            println!("You want to install, {}!", package);
            let url = format!("https://will.okit.works/package?p={package}");
            let resp = reqwest::get(url).await?;
            // dbg!("{resp:#?}");
            match resp.status() {
                reqwest::StatusCode::OK => {
                    let resp_json = &resp.json::<HashMap<String, String>>().await?;
                    println!("Success! {:?}", &resp_json);
                },
                reqwest::StatusCode::NOT_FOUND => {
                    let resp_json = &resp.json::<HashMap<String, String>>().await?;
                    if &resp_json["e"] == "Package not found." {
                        println!("{} Package not found.", Colour::Red.paint(Style::new().bold().paint("Error:").to_string()));
                    }
                },
                _ => {
                    panic!("Uh oh! Something unexpected happened.");
                },
            };
        }
    }
    
    // let iterator = matches.values_of("something");
    // TODO fix

    Ok(())
}

