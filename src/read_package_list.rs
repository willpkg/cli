use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::Result;
use std::fs;

use json::object;
use json::parse;
use json::JsonValue;

pub fn read() -> Result<JsonValue> {
    fs::create_dir_all("/opt/will")?;
    let file_path = "/opt/will/packages.json";
    let packages_result = File::open(file_path);
    let mut packages = match packages_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc,
                Err(_e) => panic!("Uh oh! /opt/will/packages.json could not be created")
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let mut contents = String::new();
    packages.read_to_string(&mut contents)?;
    dbg!(&contents == "");

    if &contents == "" {
        let empty_packages = object!{
            version: "0.1.0",
            packages: {
                will: "0.1.0"
            },
        };
        packages.write_all(empty_packages.dump().as_bytes())?;
        packages.read_to_string(&mut contents)?;
        dbg!(&contents);
    }
    let packages_json_result = parse(&contents);
    let packages_json = match packages_json_result {
        Ok(json) => json,
        Err(_e) => panic!("Uh oh! JSON-ing failed.")
    };
    Ok(packages_json)
}