use serde_json::{self, Value};
use std::process::Command;

pub fn readvalue(file: &String, query: &String) -> Option<Value> {
    let output = Command::new("nix-instantiate")
        .arg("-E")
        .arg(format!(
            "import {} {}",
            &file, "{ config = {}; pkgs = import <nixpkgs> {}; lib = import <nixpkgs/lib>;}"
        ))
        //Temporary so that my config works lol
        .arg("-I")
        .arg("nixpkgs=/home/victor/Documents/nixpkgs")
        .arg("--eval")
        .arg("--json")
        .arg("--strict")
        .output()
        .expect("Failed");
    let outstr = String::from_utf8_lossy(&output.stdout);
    //println!("outstr {} ", outstr);
    //let contents = fs::read_to_string(config.filename)?;
    let res: serde_json::Value = serde_json::from_str(&outstr).expect("Unable to parse");
    let qlist = query.split(".");
    let vec = qlist.collect::<Vec<&str>>();
    valfromjson(vec, &res)
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
