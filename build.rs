extern crate chrono;
#[cfg(windows)]
extern crate winres;

use std::collections::HashMap;
use std::process::Command;
use std::{fs::read_to_string, fs::File, io::Write};

// execute app and get stdout
fn execute(cmd: &str, args: &[&str]) -> String {
    match Command::new(cmd).args(args).output() {
        Ok(value) => match String::from_utf8(value.stdout) {
            Ok(value) => value.trim().to_string(),
            Err(_) => "unknown".to_string(),
        },
        Err(_) => "unknown".to_string(),
    }
}

// parse rustc and cargo stdout
fn parse(s: String) -> HashMap<String, String> {
    let mut res = HashMap::new();
    for line in s.lines() {
        let block: Vec<&str> = line.split(":").collect();
        if block.len() == 1 {
            res.insert("header".to_string(), block[0].trim().to_string());
        } else {
            res.insert(block[0].to_string(), block[1].trim().to_string());
        }
    }
    res
}

// get packages info from Cargo.lock
fn app_packages() -> String {
    let mut counter = 0;
    let mut name = String::new();
    let mut packages = String::new();
    let mut name_flag = false;

    let data = read_to_string("Cargo.lock").unwrap();

    for line in data.lines() {
        if line.starts_with("name = ") {
            name = line[7..].to_string();
            name_flag = true;
        }
        if line.starts_with("version =") && name_flag {
            name_flag = false;
            // (name, version)
            packages.push_str(&format!("    ({}, {}),\n", name, &line[10..]));
            counter += 1;
        }
    }

    format!("pub static APP_PACKAGES: [(&'static str, &'static str); {}] = [\n{}];", counter, packages)
}

fn get_current_date() -> String {
    use chrono::prelude::*;
    let utc: DateTime<Utc> = Utc::now();
    utc.format("%Y-%m-%d %H:%M:%S %z").to_string()
}

fn main() {
    // generate build info
    let mut source_code = include_str!("extra/build_rs_header.txt").to_string();
    // add comment line
    source_code.push_str("// builder info\n");

    // rust and cargo info
    for (prefix, executable) in [("RUST", "rustc"), ("CARGO", "cargo")].iter() {
        let iterator = parse(execute(executable, &["-vV"]));
        for (k, v) in iterator {
            let key = k.to_uppercase().replace("-", "_").replace(" ", "_");
            let fmt_str = format!("pub static {}_{}: &'static str = \"{}\";\n", prefix, key, v);
            source_code.push_str(&fmt_str);
        }
    }

    // and another comment line
    source_code.push_str("// project info\n");

    // add git project head hash
    let git_hash = include_str!(".git/ORIG_HEAD").trim();
    let git_project_hash = format!("pub static GIT_PROJECT_HASH: &'static str = \"{}\";\n", git_hash);
    source_code.push_str(&git_project_hash);

    // add build datetime
    let build_date = get_current_date();
    let project_build_date = format!("pub static PROJECT_BUILD_DATE: &'static str = \"{}\"; // UTC+0\n", build_date);
    source_code.push_str(&project_build_date);

    // add packages in Cargo.lock
    source_code.push_str(&app_packages());

    // and write to build.rs file
    File::create("src/build.rs").and_then(|mut file| write!(file, "{}", source_code)).unwrap();

    // set icon for windows binary
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("extra/icon.ico");
        res.compile().unwrap();
    }
}
