use std::{collections::HashMap, fmt::Write};

use rnix::{self, SyntaxKind, SyntaxNode};

use crate::read::ReadError;

pub fn findattr(configbase: &SyntaxNode, name: &str) -> Option<SyntaxNode> {
    let mut childvec: Vec<(String, String)> = Vec::new();
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            let qkey = name
                .split('.')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            // Now we have to read all the indent values from the key
            for subchild in child.children() {
                if subchild.kind() == SyntaxKind::NODE_ATTRPATH {
                    // We have a key, now we need to check if it's the one we're looking for
                    let key = getkey(&subchild);
                    if qkey == key {
                        if child
                            .children()
                            .any(|x| x.kind() == SyntaxKind::NODE_ATTR_SET)
                        {
                            if let Some(x) = child.children().last() {
                                if x.kind() == SyntaxKind::NODE_ATTR_SET {
                                    for n in x.children() {
                                        let i = n.children().count();
                                        if let (Some(k), Some(v)) =
                                            (n.children().nth(i - 2), n.last_child())
                                        {
                                            let f = n.to_string().find(&k.to_string()).unwrap()
                                                + k.to_string().len();
                                            childvec.push((
                                                n.to_string()[0..f].to_string(),
                                                v.to_string(),
                                            ));
                                        }
                                    }
                                }
                            }
                        } else {
                            return Some(child);
                        }
                    } else if qkey.len() > key.len() {
                        // We have a subkey, so we need to recurse
                        if key == qkey[0..key.len()] {
                            // We have a subkey, so we need to recurse
                            let subkey = &qkey[key.len()..].join(".").to_string();
                            if let Some(newbase) = getcfgbase(&child) {
                                if let Some(subattr) = findattr(&newbase, subkey) {
                                    return Some(subattr);
                                }
                            }
                        }
                    } else if qkey.len() < key.len() && qkey == key[0..qkey.len()] {
                        match child.last_child() {
                            Some(x) => {
                                childvec.push((key[qkey.len()..].join("."), x.to_string()));
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }
    if childvec.is_empty() {
        None
    } else {
        let s;
        if childvec.len() == 1 {
            s = format!("{{{} = {{ {} = {}; }}; }}", name, childvec[0].0, childvec[0].1);
        } else {
            let mut list = String::new();
            for (k, v) in childvec.iter() {
                let _ = writeln!(list, "  {} = {};", k, v);
            }
            list = list.strip_suffix('\n').unwrap_or(&list).to_string();
            s = format!("{{ {} = {{\n{}\n}}; }}", name, list);
        }
        let ast = rnix::Root::parse(&s);
        if let Some(x) = ast.syntax().children().next() {
            if x.kind() == SyntaxKind::NODE_ATTR_SET {
                if let Some(y) = x.children().next() {
                    if y.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
                        return Some(y);
                    }
                }
            }
        }
        None
    }
}

pub fn get_collection(f: String) -> Result<HashMap<String, String>, ReadError> {
    let mut map = HashMap::new();
    let ast = rnix::Root::parse(&f);
    let configbase = match getcfgbase(&ast.syntax()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    collectattrs(&configbase, &mut map);
    Ok(map)
}

pub fn collectattrs(configbase: &SyntaxNode, map: &mut HashMap<String, String>)
/*-> HashMap<String, String>*/
{
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            let children = child.children().collect::<Vec<SyntaxNode>>();
            let nodekey = children.get(0).unwrap();
            let value = children.get(1).unwrap();
            if nodekey.kind() == SyntaxKind::NODE_ATTRPATH {
                if value.kind() == SyntaxKind::NODE_ATTR_SET {
                    let mut childmap = HashMap::new();
                    collectattrs(value, &mut childmap);
                    for (nk, v) in &childmap {
                        map.insert(format!("{}.{}", nodekey, nk), v.clone());
                    }
                } else {
                    map.insert(nodekey.to_string(), value.to_string());
                }
            }
        }
    }
}

pub fn getkey(node: &SyntaxNode) -> Vec<String> {
    let mut key = vec![];
    for child in node.children() {
        if child.kind() == SyntaxKind::NODE_IDENT || child.kind() == SyntaxKind::NODE_STRING {
            key.push(child.text().to_string());
        }
    }
    key
}

pub fn getcfgbase(node: &SyntaxNode) -> Option<SyntaxNode> {
    // First check if we're in a set
    if node.kind() == SyntaxKind::NODE_ATTR_SET {
        return Some(node.clone());
    }
    // Next check if any of our children the set
    for child in node.children() {
        if child.kind() == SyntaxKind::NODE_ATTR_SET {
            return Some(child);
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
    None
}
