use rnix::{self, SyntaxKind, SyntaxNode};
use std::io::Write;

pub fn write(f: &str, query: &str, val: &str) -> SyntaxNode {
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            println!("No config base found");
            std::process::exit(1);
        }
    };
    match findattr(&configbase, &query) {
        Some(x) => modvalue(&x, &val).unwrap(),
        None => {
            println!("No config key found, adding value");
            addvalue(&configbase, &query, &val)
        }
    }
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
            println!("i is equal to: {}", i);
            let configafter = &configbase.green().children().collect::<Vec<_>>()[i..];
            println!(
                "Configafter: {:?}\n Length: {}",
                configafter,
                configafter.len()
            );
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
                            println!("Found key: {:?}", &key);
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
            let mut input = val.to_string();
            if child.kind() == SyntaxKind::NODE_STRING
            /* && check if quotes are already passed */
            {
                input = format!("\"{}\"", input);
            }
            // Add a check for valid lists
            let rep = &rnix::parse(&input)
                .node()
                .children()
                .collect::<Vec<SyntaxNode>>()[0];
            let replaced = c.replace_with(rep.green().to_owned());
            let rnode = rnix::parse(&replaced.to_string()).node();
            return Some(rnode);
        }
    }
    return None;
}

fn findattr(configbase: &SyntaxNode, name: &str) -> Option<SyntaxNode> {
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_KEY_VALUE {
            // Now we have to read all the indent values from the key
            for subchild in child.children() {
                if subchild.kind() == SyntaxKind::NODE_KEY {
                    // We have a key, now we need to check if it's the one we're looking for
                    let key = getkey(&subchild);
                    let qkey = name
                        .split(".")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    if qkey == key {
                        return Some(child);
                    } else if qkey.len() > key.len() {
                        // We have a subkey, so we need to recurse
                        if key == qkey[0..key.len()] {
                            // We have a subkey, so we need to recurse
                            let subkey = &qkey[key.len()..].join(".").to_string();
                            let newbase = getcfgbase(&child).unwrap();
                            let subattr = findattr(&newbase, &subkey);
                            if subattr.is_some() {
                                return subattr;
                            }
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn getkey(node: &SyntaxNode) -> Vec<String> {
    let mut key = vec![];
    for child in node.children() {
        if child.kind() == SyntaxKind::NODE_IDENT {
            key.push(child.text().to_string());
        }
    }
    return key;
}

fn getcfgbase(node: &SyntaxNode) -> Option<SyntaxNode> {
    // First check if we're in a set
    if node.kind() == SyntaxKind::NODE_ATTR_SET {
        return Some(node.clone());
    }
    // Next check if any of our children the set
    for child in node.children() {
        if child.kind() == SyntaxKind::NODE_ATTR_SET {
            return Some(child.clone());
        }
    }
    for child in node.children() {
        match getcfgbase(&child) {
            Some(x) => {
                return Some(x);
            }
            None => {}
        }
    }
    return None;
}

pub fn deref(f: &str, query: &str) -> SyntaxNode {
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            println!("No config base found");
            std::process::exit(1);
        }
    };
    match findattr(&configbase, &query) {
        Some(x) => deref_aux(&configbase, &x).unwrap(),
        None => {
            println!("No config key found");
            std::process::exit(1);
        }
    }
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
