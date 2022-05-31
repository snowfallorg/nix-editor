use std::collections::HashMap;

use rnix::{self, SyntaxKind, SyntaxNode};

use crate::read::ReadError;

enum AttrTypes {
    String,
    Int,
    Bool,
    List,
    Map,
}

pub fn findattr(configbase: &SyntaxNode, name: &str) -> Option<SyntaxNode> {
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

pub fn get_collection(f: String) -> Result<HashMap<String, String>, ReadError> {
    let mut map = HashMap::new();
    let ast = rnix::parse(&f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    collectattrs(&configbase, &mut map);
    Ok(map)
}

pub fn collectattrs(configbase: &SyntaxNode, map: &mut HashMap<String, String>) /*-> HashMap<String, String>*/
{
    for child in configbase.children() {
        if child.kind() == SyntaxKind::NODE_KEY_VALUE {
            let children = child.children().collect::<Vec<SyntaxNode>>();
            let nodekey = children.get(0).unwrap();
            let value = children.get(1).unwrap();
            if nodekey.kind() == SyntaxKind::NODE_KEY {
                if value.kind() == SyntaxKind::NODE_ATTR_SET {
                    let mut childmap = HashMap::new();
                    collectattrs(&value, &mut childmap);
                    for (nk,v) in &childmap {
                        map.insert(format!("{}.{}", nodekey.to_string(), nk),v.clone());
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
        if child.kind() == SyntaxKind::NODE_IDENT {
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
    None
}
