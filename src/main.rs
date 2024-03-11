use read_package_list::read;
use std::io::Result;

extern crate json;

pub mod read_package_list;

fn main() -> Result<()> {
    let args: Vec<String> = (&std::env::args().collect::<Vec<String>>())[1..].to_vec();
    if args.len() == 0 {
        println!("Help coming soon!!!");
        return Ok(())
    }
    // dbg!(&args);
    if &args[0] == "i" || &args[0] == "install" {
        println!("hi you want to install")
    }
    if &args[0] == "test" {
        read().expect("This should work.");
    }
    Ok(())
}