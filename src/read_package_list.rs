use std::fs::File;
use std::io::ErrorKind;
use std::io::BufReader;

fn read() {

    let file_path = "/opt/will/packages.json";
    let packages_result = File::open(file_path);
    let packages = match packages_result {
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

    let mut buf_reader = BufReader::new(packages);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
}