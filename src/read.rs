use crate::parse::{findattr, getcfgbase};
use failure::Fail;
use rnix::{SyntaxKind, SyntaxNode};

#[derive(Fail, Debug)]
pub enum ReadError {
    #[fail(display = "Read Error: Error while parsing.")]
    ParseError,
    #[fail(display = "Read Error: No attributes.")]
    NoAttr,
    #[fail(display = "Read Error: Error with array.")]
    ArrayError,
}

pub fn readvalue(f: &str, query: &str) -> Result<String, ReadError> {
    let ast = rnix::parse(f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    let outnode = match findattr(&configbase, query) {
        Some(x) => match findvalue(&x) {
            Some(y) => y.to_string(),
            None => return Err(ReadError::NoAttr),
        },
        None => return Err(ReadError::NoAttr),
    };
    Ok(outnode)
}

fn findvalue(node: &SyntaxNode) -> Option<SyntaxNode> {
    // First find the IDENT node
    for child in node.children() {
        if child.kind() != SyntaxKind::NODE_KEY {
            return Some(child);
        }
    }
    None
}

pub fn getarrvals(f: &str, query: &str) -> Result<Vec<String>, ReadError> {
    let ast = rnix::parse(f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    let output = match findattr(&configbase, query) {
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
    None
}

pub fn getwithvalue(f: &str, query: &str) -> Result<Vec<String>, ReadError> {
    let ast = rnix::parse(f);
    let configbase = match getcfgbase(&ast.node()) {
        Some(x) => x,
        None => {
            return Err(ReadError::ParseError);
        }
    };
    let output = match findattr(&configbase, query) {
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
    mut withvals: Vec<String>,
) -> Option<Vec<String>> {
    for child in node.children() {
        if child.kind() == rnix::SyntaxKind::NODE_WITH {
            for c in child.children() {
                if c.kind() == rnix::SyntaxKind::NODE_IDENT {
                    let mut newvals = vec![];
                    newvals.append(withvals.as_mut());
                    newvals.push(c.to_string());
                    match getwithval_aux(&child, newvals.clone()) {
                        Some(x) => return Some(x),
                        None => return Some(newvals),
                    }
                }
            }
        }
    }
    None
}