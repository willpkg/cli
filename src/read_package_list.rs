use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::fs;

use json::object;
use json::parse;
use json::JsonValue;

pub fn write(file_path: &str, content: String) -> Result<File> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(file)
}

pub fn read() -> Result<JsonValue> {
    let empty_packages = object!{
        version: "0.1.0",
        packages: {
            will: "0.1.0"
        },
    };

    fs::create_dir_all("/opt/will")?;
    let file_path = "/opt/will/packages.json";
    let packages_result = File::open(file_path);
    let packages = match packages_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match write(file_path, empty_packages.dump()) {
                Ok(_fc) => File::open(file_path)?,
                Err(_e) => panic!("Uh oh! /opt/will/packages.json could not be created")
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    // dbg!(&packages);

    let mut contents = String::new();
    let mut buf_reader = BufReader::new(packages);
    buf_reader.read_to_string(&mut contents)?;

    if &contents == "" {
        write(file_path, empty_packages.dump())?;
    }
    let packages_json_result = parse(&contents);

    let packages_json = match packages_json_result {
        Ok(json) => json,
        Err(_e) => match write(file_path, empty_packages.dump()) {
            Ok(_file) => empty_packages,
            Err(e) => panic!("Problem opening the file: {:?}", e),
        }
    };
    // dbg!(&packages_json_result);
    Ok(packages_json)
}