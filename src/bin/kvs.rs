use clap::*;
use kvs::{KvStoreErrorKind, KvStore};
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

    let database_path = "bar.db";
    let store = KvStore::open(database_path);

    let mut store = match store {
        Ok(a) => a,
        Err(e) => exit(&format!("{}", e)),
    };

    let result = match matches.subcommand() {
        sub!(get) => {
            let key = get.value_of("KEY").unwrap();

            match store.get(key.to_owned()) {
                Ok(Some(val)) => {
                    println!("{}", val);
                    process::exit(0);
                },
                Ok(None) => Err(KvStoreErrorKind::KeyDoesNotExist.into()),
                Err(e) => Err(e)
            }
        },
        sub!(set) => {
            let key = set.value_of("KEY").unwrap();
            let val = set.value_of("VALUE").unwrap();

            store.set(key.to_owned(), val.to_owned())
        }
        sub!(rm) => {
            let key = rm.value_of("KEY").unwrap();

            store.remove(key.to_owned())
        }

        _ => exit("Invalid command"),
    };

    match result {
        Ok(_) => process::exit(0),
        Err(e) => exit(&format!("{}", e)),
    }
}

fn exit(error: &str) -> ! {
    println!("{}", error);
    process::exit(1)
}
