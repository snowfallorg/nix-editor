pub mod read;
pub mod write;

pub fn printread(file: String, query: String) {
    let outval = read::readvalue(&file, &query);
    match outval {
        Some(serde_json::Value::Bool(b)) => println!("bool: {}", b),
        Some(serde_json::Value::Number(n)) => println!("number: {}", n),
        Some(serde_json::Value::String(s)) => println!("string: {}", s),
        Some(serde_json::Value::Array(a)) => {
            println!("array: {}", serde_json::to_string(&a).unwrap())
        }
        Some(serde_json::Value::Object(o)) => {
            println!("object: {}", serde_json::to_string(&o).unwrap())
        }

        Some(serde_json::Value::Null) => println!("null"),
        None => println!("Attribute \"{}\" is not set", query),
    }
}
