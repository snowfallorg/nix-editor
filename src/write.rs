use crate::parse::{findattr, getcfgbase, getkey};
use rnix::{self, SyntaxKind, SyntaxNode};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Error while parsing.")]
    ParseError,
    #[error("No attributes.")]
    NoAttr,
    #[error("Error with array.")]
    ArrayError,
    #[error("Writing value to attribute set.")]
    WriteValueToSet,
}

pub fn write(f: &str, query: &str, val: &str) -> Result<String, WriteError> {
    let ast = rnix::Root::parse(f);
    let configbase = match getcfgbase(&ast.syntax()) {
        Some(x) => x,
        None => {
            return Err(WriteError::ParseError);
        }
    };
    if val.trim_start().starts_with('{') && val.trim_end().ends_with('}') {
        if let Some(x) = getcfgbase(&rnix::Root::parse(val).syntax()) {
            if x.kind() == SyntaxKind::NODE_ATTR_SET {
                return addattrval(f, &configbase, query, &x);
            }
        }
    }

    let outnode = match findattr(&configbase, query) {
        Some(x) => {
            if let Some(n) = x.children().last() {
                if n.kind() == SyntaxKind::NODE_ATTR_SET {
                    return Err(WriteError::WriteValueToSet);
                }
            }
            modvalue(&x, val).unwrap()
        }
        None => {
            let mut y = query.split('.').collect::<Vec<_>>();
            y.pop();
            let x = findattrset(&configbase, &y.join("."), 0);
            match x {
                Some((base, attr_prefix, spaces)) => {
                    if let Some(stripped) = query.strip_prefix(&format!("{}.", attr_prefix)) {
                        addvalue(&base, &format!("{}{}", " ".repeat(spaces), stripped), val)
                    } else {
                        addvalue(&configbase, query, val)
                    }
                }
                None => {
                    addvalue(&configbase, query, val)
                }
            }
        }
    };
    Ok(outnode.to_string())
}

fn addvalue(configbase: &SyntaxNode, query: &str, val: &str) -> SyntaxNode {
    let mut index = configbase.green().children().len() - 2;
    // To find a better index for insertion, first find a matching node, then find the next newline token, after that, insert
    if let Some(x) = matchval(configbase, query, query.split('.').count()) {
        let i = configbase
            .green()
            .children()
            .position(|y| match y.into_node() {
                Some(y) => y.to_owned() == x.green().into_owned(),
                None => false,
            })
            .unwrap();
        let configgreen = configbase.green().clone();
        let configafter = &configgreen.children().collect::<Vec<_>>()[i..];
        for child in configafter {
            if let Some(x) = child.as_token() {
                if x.text().contains('\n') {
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
        }
    }
    let input = rnix::Root::parse(format!("\n  {} = {};", &query, &val).as_str()).syntax();
    let input = input.green().clone();
    if index == 0 {
        index += 1;
    };
    let new = configbase
        .green()
        .insert_child(index, rnix::NodeOrToken::Node(input.into_owned()));
    let replace = configbase.replace_with(new);
    rnix::Root::parse(&replace.to_string()).syntax()
}

// Currently indentation is badly done by inserting spaces, it should check the spaces of the previous attr instead
fn findattrset(
    configbase: &SyntaxNode,
    name: &str,
    spaces: usize,
) -> Option<(SyntaxNode, String, usize)> {
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            for subchild in child.children() {
                if subchild.kind() == SyntaxKind::NODE_ATTRPATH {
                    let key = getkey(&subchild);
                    let qkey = name
                        .split('.')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    if qkey == key {
                        match findnestedattrset(&child) {
                            Some(x) => {
                                return Some((x, name.to_string(), spaces + 2));
                            }
                            None => {
                                return None;
                            }
                        }
                    } else if qkey.len() > key.len() && qkey[0..key.len()] == key {
                        let subkey = qkey[key.len()..].join(".");
                        let newbase = getcfgbase(&child).unwrap();
                        let subattr = findattrset(&newbase, &subkey, spaces + 2);
                        match subattr {
                            Some((node, _, spaces)) => {
                                return Some((node, name.to_string(), spaces));
                            }
                            None => match findnestedattrset(&child) {
                                Some(x) => {
                                    return Some((x, key.join("."), spaces + 2));
                                }
                                None => {
                                    return None;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

// Recursively check children of NODE_APPLY and NODE_LAMBDA for NODE_ATTR_SET
fn findnestedattrset(configbase: &SyntaxNode) -> Option<SyntaxNode> {
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_APPLY
            || child.kind() == SyntaxKind::NODE_LAMBDA
            || child.kind() == SyntaxKind::NODE_WITH
        {
            return findnestedattrset(&child);
        } else if child.kind() == SyntaxKind::NODE_ATTR_SET {
            return Some(child);
        }
    }
    None
}

fn matchval(configbase: &SyntaxNode, query: &str, acc: usize) -> Option<SyntaxNode> {
    let qvec = &query
        .split('.')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let q = &qvec[..acc];
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            for subchild in child.children() {
                if subchild.kind() == SyntaxKind::NODE_ATTRPATH {
                    let key = getkey(&subchild);
                    if key.len() >= q.len() && &key[..q.len()] == q {
                        return Some(child);
                    }
                }
            }
        }
    }
    if acc == 1 {
        None
    } else {
        matchval(configbase, query, acc - 1)
    }
}

fn modvalue(node: &SyntaxNode, val: &str) -> Option<SyntaxNode> {
    // First find the IDENT node
    for child in node.children() {
        if child.kind() != SyntaxKind::NODE_ATTRPATH {
            let c = &child;
            let input = val.to_string();
            let rep = &rnix::Root::parse(&input)
                .syntax()
                .children()
                .collect::<Vec<SyntaxNode>>()[0];
            let index = node
                .green()
                .children()
                .position(|y| match y.into_node() {
                    Some(y) => y.to_owned() == c.green().into_owned(),
                    None => false,
                })
                .unwrap();
            let replaced = node
                .green()
                .replace_child(index, rnix::NodeOrToken::Node(rep.green().into_owned()));
            let out = node.replace_with(replaced);
            let rnode = rnix::Root::parse(&out.to_string()).syntax();
            return Some(rnode);
        }
    }
    None
}

// Add an attribute to the config
fn addattrval(
    f: &str,
    configbase: &SyntaxNode,
    query: &str,
    val: &SyntaxNode,
) -> Result<String, WriteError> {
    let mut attrmap = HashMap::new();
    buildattrvec(val, vec![], &mut attrmap);
    let mut file = f.to_string();

    if attrmap
        .iter()
        .any(|(key, _)| findattr(configbase, &format!("{}.{}", query, key)).is_some())
    {
        for (key, val) in attrmap {
            match write(&file, &format!("{}.{}", query, key), &val) {
                Ok(x) => file = x,
                Err(e) => return Err(e),
            }
        }
    } else if let Some(c) = getcfgbase(&rnix::Root::parse(&file).syntax()) {
        file = addvalue(&c, query, &val.to_string()).to_string();
    }
    Ok(file)
}

fn buildattrvec(val: &SyntaxNode, prefix: Vec<String>, map: &mut HashMap<String, String>) {
    for child in val.children() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            if let Some(subchild) = child.children().last() {
                if subchild.kind() == SyntaxKind::NODE_ATTR_SET {
                    for c in child.children() {
                        if c.kind() == SyntaxKind::NODE_ATTRPATH {
                            let key = getkey(&c);
                            let mut newprefix = prefix.clone();
                            newprefix.append(&mut key.clone());
                            buildattrvec(&subchild, newprefix, map);
                            break;
                        }
                    }
                } else {
                    for c in child.children() {
                        if c.kind() == SyntaxKind::NODE_ATTRPATH {
                            let key = getkey(&c);
                            let mut newprefix = prefix.clone();
                            newprefix.append(&mut key.clone());
                            map.insert(newprefix.join("."), subchild.to_string());
                        }
                    }
                }
            }
        }
    }
}

pub fn addtoarr(f: &str, query: &str, items: Vec<String>) -> Result<String, WriteError> {
    let ast = rnix::Root::parse(f);
    let configbase = match getcfgbase(&ast.syntax()) {
        Some(x) => x,
        None => return Err(WriteError::ParseError),
    };
    let outnode = match findattr(&configbase, query) {
        Some(x) => match addtoarr_aux(&x, items) {
            Some(x) => x,
            None => return Err(WriteError::ArrayError),
        },
        // If no arrtibute is found, create a new one
        None => {
            let newval = addvalue(&configbase, query, "[\n  ]");
            return addtoarr(&newval.to_string(), query, items);
        }
    };
    Ok(outnode.to_string())
}

fn addtoarr_aux(node: &SyntaxNode, items: Vec<String>) -> Option<SyntaxNode> {
    for child in node.children() {
        if child.kind() == rnix::SyntaxKind::NODE_WITH {
            return addtoarr_aux(&child, items);
        }
        if child.kind() == SyntaxKind::NODE_LIST {
            let mut green = child.green().into_owned();

            for elem in items {
                let mut i = 0;
                for c in green.children() {
                    if c.to_string() == "]" {
                        if green.children().collect::<Vec<_>>()[i - 1]
                            .as_token()
                            .unwrap()
                            .to_string()
                            .contains('\n')
                        {
                            i -= 1;
                        }
                        green = green.insert_child(
                            i,
                            rnix::NodeOrToken::Node(
                                rnix::Root::parse(&format!("\n{}{}", " ".repeat(4), elem))
                                    .syntax()
                                    .green()
                                    .into_owned(),
                            ),
                        );
                        break;
                    }
                    i += 1;
                }
            }

            let index = match node.green().children().position(|x| match x.into_node() {
                Some(x) => x.to_owned() == child.green().into_owned(),
                None => false,
            }) {
                Some(x) => x,
                None => return None,
            };

            let replace = node
                .green()
                .replace_child(index, rnix::NodeOrToken::Node(green));
            let out = node.replace_with(replace);
            let output = rnix::Root::parse(&out.to_string()).syntax();
            return Some(output);
        }
    }
    None
}

pub fn rmarr(f: &str, query: &str, items: Vec<String>) -> Result<String, WriteError> {
    let ast = rnix::Root::parse(f);
    let configbase = match getcfgbase(&ast.syntax()) {
        Some(x) => x,
        None => return Err(WriteError::ParseError),
    };
    let outnode = match findattr(&configbase, query) {
        Some(x) => match rmarr_aux(&x, items) {
            Some(x) => x,
            None => return Err(WriteError::ArrayError),
        },
        None => return Err(WriteError::NoAttr),
    };
    Ok(outnode.to_string())
}

fn rmarr_aux(node: &SyntaxNode, items: Vec<String>) -> Option<SyntaxNode> {
    for child in node.children() {
        if child.kind() == rnix::SyntaxKind::NODE_WITH {
            return rmarr_aux(&child, items);
        }
        if child.kind() == SyntaxKind::NODE_LIST {
            let green = child.green().into_owned();
            let mut idx = vec![];
            for elem in green.children() {
                if elem.as_node().is_some() && items.contains(&elem.to_string()) {
                    let index = match green.children().position(|x| match x.into_node() {
                        Some(x) => {
                            if let Some(y) = elem.as_node() {
                                x.eq(y)
                            } else {
                                false
                            }
                        }
                        None => false,
                    }) {
                        Some(x) => x,
                        None => return None,
                    };
                    idx.push(index)
                }
            }
            let mut acc = 0;
            let mut replace = green;

            for i in idx {
                replace = replace.remove_child(i - acc);
                let mut v = vec![];
                for c in replace.children() {
                    v.push(c);
                }
                if let Some(x) = v.get(i - acc - 1).unwrap().as_token() {
                    if x.to_string().contains('\n') {
                        replace = replace.remove_child(i - acc - 1);
                        acc += 1;
                    }
                }
                acc += 1;
            }
            let out = child.replace_with(replace);

            let output = rnix::Root::parse(&out.to_string()).syntax();
            return Some(output);
        }
    }
    None
}

pub fn deref(f: &str, query: &str) -> Result<String, WriteError> {
    let ast = rnix::Root::parse(f);
    let configbase = match getcfgbase(&ast.syntax()) {
        Some(x) => x,
        None => return Err(WriteError::ParseError),
    };
    let outnode = match deref_aux(&configbase, query) {
        Some(x) => x,
        None => return Err(WriteError::NoAttr),
    };
    Ok(outnode.to_string())
}

fn deref_aux(configbase: &SyntaxNode, name: &str) -> Option<SyntaxNode> {
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            // Now we have to read all the indent values from the key
            for subchild in child.children() {
                if subchild.kind() == SyntaxKind::NODE_ATTRPATH {
                    // We have a key, now we need to check if it's the one we're looking for
                    let key = getkey(&subchild);
                    let qkey = name
                        .split('.')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    if qkey == key {
                        let index =
                            match configbase
                                .green()
                                .children()
                                .position(|x| match x.into_node() {
                                    Some(x) => x.to_owned() == child.green().into_owned(),
                                    None => false,
                                }) {
                                Some(x) => x,
                                None => return None,
                            };
                        let mut del = configbase.green().remove_child(index);

                        // Remove leading newline if it still exists
                        if del.children().collect::<Vec<_>>()[index]
                            .to_string()
                            .contains('\n')
                        {
                            del = del.remove_child(index);
                        }
                        let out = configbase.replace_with(del);
                        return Some(rnix::Root::parse(&out.to_string()).syntax());
                    } else if qkey.len() > key.len() {
                        // We have a subkey, so we need to recurse
                        if key == qkey[0..key.len()] {
                            // We have a subkey, so we need to recurse
                            let subkey = &qkey[key.len()..].join(".").to_string();
                            let newbase = getcfgbase(&child).unwrap();
                            let subattr = deref_aux(&newbase, subkey);
                            if let Some(s) = subattr {
                                return Some(s);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
