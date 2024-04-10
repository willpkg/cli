use std::env;
use sysinfo::System;
use std::io::Result;
use ansi_term::{Colour, Style};


pub fn get_triple() -> Result<String> {
    let os = env::consts::OS;
    let arch = System::cpu_arch().unwrap();
    let gcc_result = which::which("gcc");
    let llvm_result = which::which("clang");
    let compiler = match gcc_result {
        Ok(_path) => "gcc",
        Err(_err) => match llvm_result {
            Ok(_path) => "llvm",
            Err(_err) => {
                eprintln!("{} No compiler found.", Colour::Red.paint(Style::new().bold().paint("Error:").to_string()));
                std::process::exit(1)
            }
        }
    };
    Ok(format!("{os}-{arch}-{compiler}"))
}