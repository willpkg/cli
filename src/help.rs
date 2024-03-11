// use chrono::{TimeZone, Utc};
use std::env;

pub fn help_prompt() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    /* let now = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => { Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap() }
        Err(_) => Utc::now(),
    }; */
    // let build_date = format!("{}", &now.format("%d/%m/%Y at %H:%M"));
    println!(
r#"will: A package manager that willingly installes packages.

Usage: will [-v | --version] [-h | --help] <command> [<args>]

You are running will v{VERSION}"#
    );
}

pub fn version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("You are running will version {VERSION}");
}