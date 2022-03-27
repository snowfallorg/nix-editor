use crate::parse::{findattr, getcfgbase, collectattrs};
use rnix::{SyntaxKind, SyntaxNode};
use serde_json::{self, Value};
use std::{process::Command, collections::HashMap};

pub enum ReadError {
    ParseError,
    NoAttr,
    ArrayError,
}

pub fn readvalue(f: &str, query: &str) -> Result<String, ReadError> {
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    let outnode = match findattr(&configbase, &query) {
        Some(x) => match findvalue(&x) {
            Some(y) => y.to_string(),
            None => return Err(ReadError::NoAttr),
        },
        None => return Err(ReadError::NoAttr),
    };
    //let mut map = HashMap::new();
    //collectattrs(&configbase, &mut map);
    Ok(outnode)
}

fn findvalue(node: &SyntaxNode) -> Option<SyntaxNode> {
    // First find the IDENT node
    for child in node.children() {
        if child.kind() != SyntaxKind::NODE_KEY {
            return Some(child);
        }
    }
    return None;
}

pub fn readevalvalue(file: &str, query: &str) -> Result<Value, ReadError> {
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

pub fn getarrvals(f: &str, query: &str) -> Result<Vec<String>, ReadError> {
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    let output = match findattr(&configbase, &query) {
        Some(x) => match getarrvals_aux(&x) {
            Some(y) => y,
            None => return Err(ReadError::ArrayError),
        },
        None => return Err(ReadError::NoAttr),
    };
    Ok(output)
}

fn getarrvals_aux(
    node: &SyntaxNode,
) -> Option<Vec<String>> {
    for child in node.children() {
        if child.kind() == rnix::SyntaxKind::NODE_WITH {
            return getarrvals_aux(&child);

        }
        if child.kind() == SyntaxKind::NODE_LIST {
            let mut out = vec![];
            for elem in child.children() {
                out.push(elem.to_string());
            }
            return Some(out);
        }
    }
    return None;
}

pub fn getwithvalue(f: &str, query: &str) -> Result<Vec<String>, ReadError> {
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    let output = match findattr(&configbase, &query) {
        Some(x) => match getwithval_aux(&x, vec![]) {
            Some(y) => y,
            None => return Err(ReadError::NoAttr),
        },
        None => return Err(ReadError::NoAttr),
    };
    Ok(output)
}

fn getwithval_aux(
    node: &SyntaxNode,
    withvals: Vec<String>,
) -> Option<Vec<String>> {
    for child in node.children() {
        if child.kind() == rnix::SyntaxKind::NODE_WITH {
            for c in child.children() {
                if c.kind() == rnix::SyntaxKind::NODE_IDENT {
                    let mut newvals = vec![];
                    newvals.append(withvals.clone().as_mut());
                    newvals.push(c.to_string());
                    match getwithval_aux(&child, newvals.clone()) {
                        Some(x) => return Some(x),
                        None => return Some(newvals),
                    }
                }
            }
        }
    }
    return None;
}