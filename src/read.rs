use serde_json::{self, Value};
use std::process::Command;

pub enum ReadError {
    ParseError,
    NoAttr,
}

pub fn readvalue(file: &str, query: &str) -> Result<Value, ReadError> {
    let output = Command::new("nix-instantiate")
        .arg("-E")
        .arg(format!(
            "import {} {}",
            &file, "{ config = {}; pkgs = import <nixpkgs> {}; lib = import <nixpkgs/lib>;}"
        ))
        .arg("--eval")
        .arg("--json")
        .arg("--strict")
        .output()
        .expect("nix-instantiate failed");
    /*if !&output.status.success() {
        return Err(ReadError::ParseError);
    }*/
    let outstr = String::from_utf8_lossy(&output.stdout);
    let res: serde_json::Value = match serde_json::from_str(&outstr) {
        Ok(x) => x,
        Err(_) => return Err(ReadError::ParseError),
    };
    let qlist = query.split(".");
    let vec = qlist.collect::<Vec<&str>>();
    match valfromjson(vec, &res) {
        Some(x) => Ok(x),
        None => return Err(ReadError::NoAttr),
    }
}

fn valfromjson<'a>(lst: Vec<&str>, j: &'a serde_json::Value) -> Option<serde_json::Value> {
    match lst[..] {
        [] => None,
        [_] => match j.get(lst[0]) {
            Some(x) => Some(x.to_owned()),
            None => None,
        },
        _ => valfromjson(lst[1..].to_vec(), &j[lst[0]]),
    }
}
