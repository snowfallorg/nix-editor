use crate::write::write;
use expect_test::expect_file;
use std::{ffi::OsStr, fs, path::PathBuf};

fn dir_tests<F>(dir: &str, get_actual: F)
where
    F: Fn(String, Vec<String>) -> String,
{
    let base_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_data", dir]
        .iter()
        .collect();
    let tests = base_path.read_dir().unwrap();
    for test_case in tests {
        let case_folder = test_case.unwrap().path();
        let entries = base_path.join(&case_folder).read_dir().unwrap();

        for entry in entries {
            let path = entry.unwrap().path();

            if path.extension() != Some(OsStr::new("nix")) {
                continue;
            }

            println!("testing: {}", path.display());

            let mut code = fs::read_to_string(&path).unwrap();
            if code.ends_with('\n') {
                code.truncate(code.len() - 1);
            }
            let args_path = path.with_extension("args");
            let args_raw = fs::read_to_string(&args_path).unwrap();
            let args: Vec<String> = serde_json::from_str(&args_raw).unwrap();

            let actual = get_actual(code, args);
            expect_file![path.with_extension("expect")].assert_eq(&actual);
        }
    }
}

#[test]
fn write_dir_tests() {
    dir_tests("write", |code: String, args| {
        let actual = match write(&code, &args[0], &args[1]) {
            Ok(s) => s,
            Err(_) => panic!("Failed to write to file"),
        };

        actual
    })
}
