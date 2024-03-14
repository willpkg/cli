use std::io::{Result, stdout};
use clap::Command;
use clap_complete::{generate, Generator, Shell};

extern crate serde_json;
extern crate clap;
extern crate clap_complete;

pub mod record;
pub mod request;
pub mod cli;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
}

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = cli::build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }
    
    // let iterator = matches.values_of("something");
    // TODO fix

    Ok(())
}

