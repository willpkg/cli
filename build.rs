#[path = "src/cli.rs"]
pub mod cli;

extern crate clap;
extern crate clap_complete;

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from("man");

    let cmd = cli::build_cli();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("will.1"), buffer)?;

    Ok(())
}