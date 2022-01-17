use crate::parse::{findattr, getcfgbase, getkey};
use rnix::{self, SyntaxKind, SyntaxNode};
pub enum WriteError {
    ParseError,
    NoAttr,
}

pub fn write(f: &str, query: &str, val: &str) -> Result<String, WriteError> {
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(WriteError::ParseError);
        }
    };
    let outnode = match findattr(&configbase, &query) {
        Some(x) => modvalue(&x, &val).unwrap(),
        None => addvalue(&configbase, &query, &val),
    };
    Ok(outnode.to_string())
}

fn addvalue(configbase: &SyntaxNode, query: &str, val: &str) -> SyntaxNode {
    let mut index = configbase.green().children().len() - 2;
    // To find a better index for insertion, first find a matching node, then find the next newline token, after that, insert
    match matchval(&configbase, &query, query.split(".").count()) {
        Some(x) => {
            let i = configbase
                .green()
                .children()
                .position(|y| match y.into_node() {
                    Some(y) => y.to_owned() == x.green().to_owned(),
                    None => false,
                })
                .unwrap();
            let configafter = &configbase.green().children().collect::<Vec<_>>()[i..];
            for child in configafter {
                match child.as_token() {
                    Some(x) => {
                        if x.text().contains("\n") {
                            let cas = configafter.to_vec();
                            index = i + cas
                                .iter()
                                .position(|y| match y.as_token() {
                                    Some(t) => t == x,
                                    None => false,
                                })
                                .unwrap();
                            break;
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }
    let input = rnix::parse(format!("\n  {} = {};", &query, &val).as_str())
        .node()
        .green()
        .to_owned();
    let new = configbase
        .green()
        .insert_child(index, rnix::NodeOrToken::Node(input));
    let replace = configbase.replace_with(new.clone());
    rnix::parse(&replace.to_string()).node()
}

fn matchval(configbase: &SyntaxNode, query: &str, acc: usize) -> Option<SyntaxNode> {
    let qvec = &query
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let q = &qvec[..acc];
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_KEY_VALUE {
            for subchild in child.children() {
                if subchild.kind() == SyntaxKind::NODE_KEY {
                    let key = getkey(&subchild);
                    if key.len() >= q.len() {
                        if &key[..q.len()] == q {
                            return Some(child.clone());
                        }
                    }
                }
            }
        }
    }
    if acc == 1 {
        return None;
    } else {
        matchval(configbase, query, acc - 1)
    }
}

fn modvalue(node: &SyntaxNode, val: &str) -> Option<SyntaxNode> {
    // First find the IDENT node
    for child in node.children() {
        if child.kind() != SyntaxKind::NODE_KEY {
            let c = &child.clone();
            let input = val.to_string();
            /* if child.kind() == SyntaxKind::NODE_STRING
            /* && check if quotes are already passed */
            {
                input = format!("\"{}\"", input);
            }
            // Add a check for valid lists */
            let rep = &rnix::parse(&input)
                .node()
                .children()
                .collect::<Vec<SyntaxNode>>()[0];
            let index = node
                .green()
                .children()
                .position(|y| match y.into_node() {
                    Some(y) => y.to_owned() == c.green().to_owned(),
                    None => false,
                })
                .unwrap();
            let replaced = node
                .green()
                .replace_child(index, rnix::NodeOrToken::Node(rep.green().to_owned()));
            let out = node.replace_with(replaced);
            let rnode = rnix::parse(&out.to_string()).node();
            return Some(rnode);
        }
    }
    return None;
}

pub fn deref(f: &str, query: &str) -> Result<String, WriteError> {
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => return Err(WriteError::ParseError),
    };
    let outnode = match findattr(&configbase, &query) {
        Some(x) => deref_aux(&configbase, &x).unwrap(),
        None => return Err(WriteError::NoAttr),
    };
    Ok(outnode.to_string())
}

fn deref_aux(configbase: &SyntaxNode, node: &SyntaxNode) -> Option<SyntaxNode> {
    let index = match configbase
        .green()
        .children()
        .position(|x| match x.into_node() {
            Some(x) => x.to_owned() == node.green().to_owned(),
            None => false,
        }) {
        Some(x) => x,
        None => return None,
    };
    let del = configbase.green().remove_child(index);
    let out = configbase.replace_with(del);
    return Some(rnix::parse(&out.to_string()).node());
}
