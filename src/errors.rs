pub fn printread(f: &str, attr: &str) -> Result<String, read::ReadError> {
    match read::readvalue(f, attr) {
        Ok(x) => Ok(x),
        Err(e) => return Err(e),
    }
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
                "cannot modify '{}' : {}",
                attr.purple(),
                "No such attribute".purple()
            );
            printerror(&msg);
        }
        write::WriteError::ArrayError => {
            msg = format!(
                "cannot add an element to '{}' : {}",
                attr.purple(),
                "Is this value an array?".purple()
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
