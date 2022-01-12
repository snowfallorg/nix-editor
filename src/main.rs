use argparse::{ArgumentParser, List, Store, StoreTrue}; //switch to clap
use std::fs;
use std::io::{stderr, stdout};
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Cmd {
    read,
    write,
}

impl FromStr for Cmd {
    type Err = ();
    fn from_str(src: &str) -> Result<Cmd, ()> {
        return match src {
            "read" => Ok(Cmd::read),
            "write" => Ok(Cmd::write),
            _ => Err(()),
        };
    }
}

fn write_command(file: String, query: String, args: Vec<String>) {
    let mut val = String::new();
    let mut out = String::new();
    let mut deref = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Write config option");
        ap.refer(&mut val)
            .add_option(&["-v", "--value"], Store, "Value to write");
        ap.refer(&mut deref)
            .add_option(&["-d", "--dereference"], StoreTrue, "Value to write");
        ap.refer(&mut out)
            .add_option(
                &["-o", "--output"],
                Store,
                "Output file for modified config file to write",
            )
            .required();
        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(()) => {}
            Err(x) => {
                std::process::exit(x);
            }
        }
    }
    let f = fs::read_to_string(&file).expect("Fail to read file");

    match (val.is_empty(), deref) {
        (true, false) => {
            println!("No value specified");
            std::process::exit(1);
        }
        (false, true) => {
            println!("Cannot write and dereference at the same time");
            std::process::exit(1);
        }
        (false, false) => {
            nix_editor::write::write(&f, &query, &val, &out);
        }
        (true, true) => {
            nix_editor::write::deref(&f, &query, &out);
        }
    }
}

fn read_command(file: String, query: String) {
    nix_editor::printread(file, query)
}

fn main() {
    let mut subcommand = Cmd::read;
    let mut verbose = false;
    let mut file = "".to_string();
    let mut query = "".to_string();
    let mut args = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Read and modify nixos configuration files");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut file)
            .required()
            .add_option(&["-f", "--file"], Store, "Config file");
        ap.refer(&mut subcommand).required().add_argument(
            "command",
            Store,
            r#"Command "read" or "write" required"#,
        );
        ap.set_description("Reads an option from a config file");
        ap.refer(&mut query)
            .required()
            .add_option(&["-q", "--query"], Store, r#"Option query"#);
        ap.refer(&mut args)
            .add_argument("arguments", List, r#"Arguments for command"#);
        ap.stop_on_first_argument(true);
        ap.parse_args_or_exit();
    }
    args.insert(0, format!("subcommand {:?}", subcommand));
    match subcommand {
        Cmd::read => read_command(file, query),
        Cmd::write => write_command(file, query, args),
    }
}
