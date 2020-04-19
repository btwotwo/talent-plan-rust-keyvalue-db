use clap::*;
use kvs::KvStore;
use std::process;

macro_rules! sub {
    ($name:ident) => {
        (stringify!($name), Some($name))
    };
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .get_matches();

    let database_path = "foo.db";
    let mut store = KvStore::open(database_path).unwrap();

    match matches.subcommand() {
        sub!(get) => exit("unimplemented"),
        sub!(set) => {
            let key = set.value_of("KEY").unwrap();
            let val = set.value_of("VALUE").unwrap();

            store.set(key.to_owned(), val.to_owned()).unwrap();
        }
        sub!(rm) => exit("unimplemented"),

        _ => exit("Invalid command"),
    }
}

fn exit(error: &str) -> ! {
    eprintln!("{}", error);
    process::exit(1)
}
