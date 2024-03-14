use clap::{Arg, Command, value_parser, ArgAction};
use clap_complete::Shell;

pub fn build_cli() -> Command {
    Command::new("will")
        .arg_required_else_help(true)
        .version("0.1.0")
        .author("uimaxbai (https://gitlab.com/uimaxbai)")
        .about("A package manager that willingly installs packages.")
        
        .arg(Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)))
        .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Be verbose.")
                .action(ArgAction::SetTrue)) // No argument after flag
        .subcommand(Command::new("install")
                        .arg_required_else_help(true)
                        .alias("i")
                        .arg(Arg::new("package")
                                .index(1)
                                .help("The package you want to install.")
                                .required(true))
                        .arg(Arg::new("source")
                                .short('s')
                                .long("from-source")
                                .help("Install packages only from source (abort if no source)"))
                        .arg(Arg::new("binary")
                                .short('b')
                                .long("from-binary")
                                .help("Install packages only from binary (abort if no binary)")))
}