use clap::*;
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .get_matches();

    match matches.subcommand_name() {
        Some("get") => exit("unimplemented"),
        Some("set") => exit("unimplemented"),
        Some("rm") => exit("unimplemented"),

        _ => exit("Invalid command"),
    }
}

fn exit(error: &str) -> ! {
    eprintln!("{}", error);
    process::exit(1)
}
