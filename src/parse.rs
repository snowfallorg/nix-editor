use rnix::{self, SyntaxKind, SyntaxNode};

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

pub fn getkey(node: &SyntaxNode) -> Vec<String> {
    let mut key = vec![];
    for child in node.children() {
        if child.kind() == SyntaxKind::NODE_IDENT {
            key.push(child.text().to_string());
        }
    }
    return key;
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
    return None;
}
