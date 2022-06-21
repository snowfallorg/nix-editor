use crate::{
    read::{getarrvals, readvalue, ReadError, getwithvalue},
    write::{ addtoarr, deref, write, rmarr }, parse::{collectattrs, getcfgbase, get_collection},
};
use core::panic;
use std::{fs, path::Path, collections::HashMap};

#[test]
fn read_val1() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match readvalue(&config, "system.stateVersion") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is correct
    assert!(out == "\"22.05\"")
}

#[test]
fn read_val2() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match readvalue(&config, "programs.gnupg.agent.enable") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is correct
    assert!(out == "true")
}

#[test]
fn read_val3() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match readvalue(&config, "this.does.not.exist") {
        Ok(_) => panic!("Read value that does not exist"),
        Err(e) => e,
    };

    // Check if read error is correct
    match out {
        ReadError::NoAttr => (),
        _ => panic!("Incorrect error type"),
    }
}

#[test]
fn read_val4() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match readvalue(&config, "boot.loader") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is correct
    assert!(out == "{\n  systemd-boot.enable = true;\n  efi.canTouchEfiVariables = true;\n}")
}

#[test]
fn read_val5() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match readvalue(&config, "system") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is correct
    assert!(out == "{ stateVersion = \"22.05\"; }")
}

#[test]
fn readarr_val1() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match getarrvals(&config, "imports") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is correct
    assert!(out == vec!["./hardware-configuration.nix"])
}

#[test]
fn readarr_val2() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match getarrvals(&config, "environment.systemPackages") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is correct
    assert!(out == vec!["vim", "wget", "firefox"])
}

#[test]
fn write_val1() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file
    let out = match write(&config, "boot.loader.systemd-boot.enable", "false") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match readvalue(&out, "boot.loader.systemd-boot.enable") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is false
    assert!(r == "false")
}

#[test]
fn write_val2() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match write(&config, "this.doesnot.exist", "\"test\"") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match readvalue(&out, "this.doesnot.exist") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is "test"
    assert!(r == "\"test\"")
}

#[test]
fn write_val3() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match write(&config, "programs.gnupg.agent.enableExtraSocket", "true") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match readvalue(&out, "programs.gnupg.agent.enableExtraSocket") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is true
    assert!(r == "true")
}

#[test]
fn write_val4() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match write(&config, "boot.loader.systemd-boot.editor", "false") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match readvalue(&out, "boot.loader.systemd-boot.editor") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is false
    assert!(r == "false")
}

#[test]
fn write_format1() {
    let config =
        fs::read_to_string(Path::new("src/tests/format.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match write(&config, "a.c", "false") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match readvalue(&out, "a.c") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is false
    assert!(r == "false");
    // Check format
    let expectedout = r#"{
  a = {
    b = true;
    d = {
      e = false;
      f = "hello";
    };
    c = false;
  };
}"#;
    assert!(out.eq(expectedout))
}

#[test]
fn write_format2() {
    let config =
        fs::read_to_string(Path::new("src/tests/format.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match write(&config, "a.d.g", "\"test\"") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    println!("{out}");
    // Check if read value is correct
    let r = match readvalue(&out, "a.d.g") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is "test"
    assert!(r == "\"test\"");
    // Check format
    let expectedout = r#"{
  a = {
    b = true;
    d = {
      e = false;
      f = "hello";
      g = "test";
    };
  };
}"#;
    assert!(out.eq(expectedout))
}

#[test]
fn write_arr1() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match addtoarr(
        &config,
        "environment.systemPackages",
        vec!["nano".to_string(), "unzip".to_string()],
    ) {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match getarrvals(&out, "environment.systemPackages") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is "test"
    assert!(r == vec!["vim", "wget", "firefox", "nano", "unzip"])
}

#[test]
fn write_arr2() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match addtoarr(
        &config,
        "test.arr",
        vec!["one".to_string(), "two".to_string()],
    ) {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    let out2 = match addtoarr(
        &out,
        "test.arr",
        vec!["three".to_string(), "four".to_string()],
    ) {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    let out3 = match rmarr(
        &out2,
        "test.arr",
        vec!["four".to_string(), "two".to_string()],
    ) {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match getarrvals(&out3, "test.arr") {
        Ok(s) => s,
        Err(_) => panic!("Failed to read value"),
    };

    // Check if read value is "test"
    assert!(r == vec!["one", "three"])
}

#[test]
fn deref_val1() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match deref(&config, "programs.gnupg.agent.enableSSHSupport") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is correct
    let r = match readvalue(&out, "programs.gnupg.agent.enableSSHSupport") {
        Ok(_) => panic!("Read value that does not exist"),
        Err(e) => e,
    };

    // Check if read value is "test"
    match r {
        ReadError::NoAttr => (),
        _ => panic!("Incorrect error for no attribute"),
    }
}

#[test]
fn get_with1() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match getwithvalue(&config, "environment.systemPackages") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is "pkgs"
    assert!(out == vec!["pkgs"])
}

#[test]
fn get_with2() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out1 = match write(&config, "test2.test", "with x; with y; test") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    let out2 = match getwithvalue(&out1, "test2.test") {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read value is "pkgs"
    assert!(out2 == vec!["x", "y"])
}

#[test]
fn read_collect() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let mut map  = HashMap::new();
    let configbase = getcfgbase(&rnix::parse(&config).node()).unwrap();
    collectattrs(&configbase, &mut map);

    // Check if read value is "pkgs"
    assert!(map.len() == 11);
    println!("{:?}", map.get("imports").unwrap());
    assert!(map.get("imports").unwrap() == "[ # Include the results of the hardware scan.\n      ./hardware-configuration.nix\n    ]");
    assert!(map.get("boot.loader.systemd-boot.enable").unwrap() == "true");
    assert!(map.get("boot.loader.efi.canTouchEfiVariables").unwrap() == "true");
    assert!(map.get("programs.gnupg.agent.enable").unwrap() == "true");
    assert!(map.get("programs.gnupg.agent.enableSSHSupport").unwrap() == "true");
    assert!(map.get("system.stateVersion").unwrap() == "\"22.05\"");
}

#[test]
fn main_test() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let mut map  = HashMap::new();
    let configbase = getcfgbase(&rnix::parse(&config).node()).unwrap();
    collectattrs(&configbase, &mut map);

    // Check if read value is "pkgs"
    assert!(map.len() == 11);
    println!("{:?}", map.get("imports").unwrap());
    assert!(map.get("imports").unwrap() == "[ # Include the results of the hardware scan.\n      ./hardware-configuration.nix\n    ]");
    assert!(map.get("boot.loader.systemd-boot.enable").unwrap() == "true");
    assert!(map.get("boot.loader.efi.canTouchEfiVariables").unwrap() == "true");
    assert!(map.get("programs.gnupg.agent.enable").unwrap() == "true");
    assert!(map.get("programs.gnupg.agent.enableSSHSupport").unwrap() == "true");
    assert!(map.get("system.stateVersion").unwrap() == "\"22.05\"");
}

#[test]
fn collect1() {
    let config =
        fs::read_to_string(Path::new("src/tests/format.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match get_collection(config) {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read values are correct
    assert!(out.get("a.b") == Some(&String::from("true")));
}

#[test]
fn collect2() {
    let config =
        fs::read_to_string(Path::new("src/tests/configuration.nix")).expect("Failed to read file");

    // Write value to file that does not yet exist
    let out = match get_collection(config) {
        Ok(s) => s,
        Err(_) => panic!("Failed to write to file"),
    };

    // Check if read values are correct
    assert!(out.get("boot.loader.efi.canTouchEfiVariables") == Some(&String::from("true")));
    assert!(out.get("programs.gnupg.agent.enableSSHSupport") == Some(&String::from("true")));
    assert!(out.get("system.stateVersion") == Some(&String::from("\"22.05\"")));
}