use colored::*;
use std::io::Write;
pub mod read;
pub mod write;

pub fn printread(file: &str, attr: &str) -> Result<String, read::ReadError> {
    let outval = match read::readvalue(file, attr) {
        Ok(x) => x,
        Err(e) => return Err(e), /*{
                                     let msg = format!(
                                         "failed to parse '{}' as a nix configuration file",
                                         file.purple()
                                     );
                                     printerror(&msg);
                                     std::process::exit(1);
                                 }*/
    };
    Ok(match outval {
        serde_json::Value::Bool(b) => format!("bool: {}", b),
        serde_json::Value::Number(n) => format!("number: {}", n),
        serde_json::Value::String(s) => format!("string: {}", s),
        serde_json::Value::Array(a) => {
            format!("array: {}", serde_json::to_string(&a).unwrap())
        }
        serde_json::Value::Object(o) => {
            format!("object: {}", serde_json::to_string(&o).unwrap())
        }
        serde_json::Value::Null => format!("null"),
    })
}

pub fn writetofile(file: &str, out: &str) {
    let mut outfile = std::fs::File::create(file).expect("create failed");
    outfile.write_all(out.as_bytes()).expect("write failed");
}

pub fn writeerr(e: write::WriteError, file: &str, attr: &str) {
    let msg;
    match e {
        write::WriteError::ParseError => {
            msg = format!(
                "failed to parse '{}' as a nix configuration file",
                file.purple()
            );
            printerror(&msg);
        }
        write::WriteError::NoAttr => {
            msg = format!(
                "cannot dereference '{}' : {}",
                attr.purple(),
                "No such attribute".purple()
            );
            printerror(&msg);
        }
    }
}

pub fn readerr(e: read::ReadError, file: &str, attr: &str) {
    let msg;
    match e {
        read::ReadError::ParseError => {
            msg = format!(
                "failed to parse '{}' as a nix configuration file",
                file.purple()
            );
            printerror(&msg);
        }
        read::ReadError::NoAttr => {
            msg = format!(
                "cannot read attribute '{}' in '{}' : {}",
                attr.purple(),
                file.purple(),
                "No attribute found".purple()
            );
            printerror(&msg);
        }
    }
}

pub fn nofileerr(file: &str) {
    let msg = format!("reading '{}': {}", file.purple(), "No such file".purple());
    printerror(&msg);
}

fn printerror(msg: &str) {
    println!("{} {}", "error:".red(), msg);
}
