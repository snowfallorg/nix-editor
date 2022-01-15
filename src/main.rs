use clap::{self, ArgGroup, Parser};
use nix_editor::{printread, write::deref, write::write};
use std::{fs, path::Path};
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("write")
        .args(&["val", "deref"]),
))]
struct Args {
    /// Configuration file to read
    file: String,

    /// Nix configuration option arribute
    attribute: String,

    /// Value to write
    #[clap(short, long)]
    val: Option<String>,

    /// Dereference the value of the attribute
    #[clap(short, long)]
    deref: bool,

    /// Output file for modified config or read value
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let output;
    if !Path::is_file(Path::new(&args.file)) {
        nix_editor::nofileerr(&args.file);
        std::process::exit(1);
    }
    let f = fs::read_to_string(&args.file).expect("Failed to read file");
    if args.val.is_some() {
        output = match write(&f, &args.attribute, &args.val.unwrap()) {
            Ok(x) => x,
            Err(e) => {
                nix_editor::writeerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    } else if args.deref {
        output = match deref(&f, &args.attribute) {
            Ok(x) => x,
            Err(e) => {
                nix_editor::writeerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    } else {
        output = match printread(&args.file, &args.attribute) {
            Ok(x) => x,
            Err(e) => {
                nix_editor::readerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    }

    if args.output.is_some() {
        nix_editor::writetofile(&args.output.unwrap(), &output)
    } else {
        println!("{}", output);
    }
}
