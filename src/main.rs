use clap::{self, ArgGroup, Parser};
use nix_editor::{write::addtoarr, write::deref, write::write};
use owo_colors::*;
use std::{fs, io::Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("write").args(&["val", "deref", "arr"])))]
struct Args {
    /// Configuration file to read
    file: String,

    /// Nix configuration option arribute
    attribute: String,

    /// Value to write
    #[arg(short, long)]
    val: Option<String>,

    /// Element to add
    #[arg(short, long)]
    arr: Option<String>,

    /// Dereference the value of the attribute
    #[arg(short, long)]
    deref: bool,

    /// Edit the file in-place
    #[arg(short, long)]
    #[arg(requires("write"))]
    inplace: bool,

    /// Output file for modified config or read value
    #[arg(short, long)]
    output: Option<String>,

    /// Prints console output without newlines or trimmed output
    #[arg(short, long)]
    raw: bool,

    /// Formats output using nixpkgs-fmt. Helps when writing new values
    #[arg(short, long)]
    format: bool,
}

fn writetofile(file: &str, out: &str, format: bool) {
    let mut outfile = std::fs::File::create(file).expect("create failed");
    if format {
        outfile
            .write_all(nixpkgs_fmt::reformat_string(out).as_bytes())
            .expect("write failed");
    } else {
        outfile.write_all(out.as_bytes()).expect("write failed");
    }
}

fn printread(f: &str, attr: &str) -> Result<String, nix_editor::read::ReadError> {
    nix_editor::read::readvalue(f, attr)
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
        nix_editor::write::WriteError::WriteValueToSet => {
            msg = format!(
                "cannot modify '{}' : {}",
                attr.purple(),
                "Cannot set an attribute-set to a value".purple()
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
    let f = match fs::read_to_string(&args.file) {
        Ok(x) => x,
        Err(_) => {
            nofileerr(&args.file);
            std::process::exit(1);
        }
    };
    if args.arr.is_some() {
        output = match addtoarr(&f, &args.attribute, vec![args.arr.unwrap()]) {
            Ok(x) => x,
            Err(e) => {
                writeerr(e, &args.file, &args.attribute);
                std::process::exit(1)
            }
        };
    } else if args.val.is_some() {
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

    if args.inplace {
        writetofile(&args.file, &output, args.format);
    } else if args.output.is_some() {
        writetofile(&args.output.unwrap(), &output, args.format);
    } else if args.raw {
        print!(
            "{}",
            if args.format {
                nixpkgs_fmt::reformat_string(&output)
            } else {
                output
            }
        );
        if let Err(e) = std::io::stdout().flush() {
            panic!("{}", e);
        }
    } else {
        println!(
            "{}",
            if args.format {
                nixpkgs_fmt::reformat_string(&output).trim().to_string()
            } else {
                output.trim().to_string()
            }
        );
    }
}
