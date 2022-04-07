use clap::{self, ArgGroup, Parser};
use nix_editor::{write::deref, write::write, write::addtoarr};
use std::{fs, path::Path, io::Write};
use owo_colors::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("write").args(&["val", "deref", "arr"])))]
struct Args {
    /// Configuration file to read
    file: String,

    /// Nix configuration option arribute
    attribute: String,

    /// Value to write
    #[clap(short, long)]
    val: Option<String>,

    /// Element to add
    #[clap(short, long)]
    arr: Option<String>,

    /// Dereference the value of the attribute
    #[clap(short, long)]
    deref: bool,

    /// Output file for modified config or read value
    #[clap(short, long)]
    output: Option<String>,
}

fn writetofile(file: &str, out: &str) {
    let mut outfile = std::fs::File::create(file).expect("create failed");
    outfile.write_all(out.as_bytes()).expect("write failed");
}

fn printread(f: &str, attr: &str) -> Result<String, nix_editor::read::ReadError> {
    match nix_editor::read::readvalue(f, attr) {
        Ok(x) => Ok(x),
        Err(e) => return Err(e),
    }
}

fn writeerr(e: nix_editor::write::WriteError, file: &str, attr: &str) {
    let msg;
    match e {
        nix_editor::write::WriteError::ParseError => {
            msg = format!(
                "failed to parse '{}' as a nix configuration file",
                file.purple()
            );
            printerror(&msg);
        }
        nix_editor::write::WriteError::NoAttr => {
            msg = format!(
                "cannot modify '{}' : {}",
                attr.purple(),
                "No such attribute".purple()
            );
            printerror(&msg);
        }
        nix_editor::write::WriteError::ArrayError => {
            msg = format!(
                "cannot add an element to '{}' : {}",
                attr.purple(),
                "Is this value an array?".purple()
            );
            printerror(&msg);
        }
    }
}

fn readerr(e: nix_editor::read::ReadError, file: &str, attr: &str) {
    let msg;
    match e {
        nix_editor::read::ReadError::ParseError => {
            msg = format!(
                "failed to parse '{}' as a nix configuration file",
                file.purple()
            );
            printerror(&msg);
        }
        nix_editor::read::ReadError::NoAttr => {
            msg = format!(
                "cannot read attribute '{}' in '{}' : {}",
                attr.purple(),
                file.purple(),
                "No attribute found".purple()
            );
            printerror(&msg);
        }
        nix_editor::read::ReadError::ArrayError => {
            msg = format!(
                "cannot read array '{}' : {}",
                attr.purple(),
                "Is this value an array?".purple()
            );
            printerror(&msg);
        }
    }
}

fn nofileerr(file: &str) {
    let msg = format!("reading '{}': {}", file.purple(), "No such file".purple());
    printerror(&msg);
}

fn printerror(msg: &str) {
    println!("{} {}", "error:".red(), msg);
}


fn main() {
    let args = Args::parse();
    let output;
    if !Path::is_file(Path::new(&args.file)) {
        nofileerr(&args.file);
        std::process::exit(1);
    }
    let f = fs::read_to_string(&args.file).expect("Failed to read file");

    if args.arr.is_some() {
        output = match addtoarr(&f, &args.attribute, vec![args.arr.unwrap().to_string()]) {
            Ok(x) => x,
            Err(e) => {
                writeerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    }

    else if args.val.is_some() {
        output = match write(&f, &args.attribute, &args.val.unwrap()) {
            Ok(x) => x,
            Err(e) => {
                writeerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    } else if args.deref {
        output = match deref(&f, &args.attribute) {
            Ok(x) => x,
            Err(e) => {
                writeerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    } else {
        output = match printread(&f, &args.attribute) {
            Ok(x) => x,
            Err(e) => {
                readerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    }

    if args.output.is_some() {
        writetofile(&args.output.unwrap(), &output)
    } else {
        println!("{}", output);
    }
}
