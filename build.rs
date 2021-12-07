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
        let block: Vec<&str> = line.splitn(2, ':').collect();
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

    let data = match read_to_string("Cargo.lock") {
        Ok(value) => value,
        Err(err) => panic!("Cannot read Cargo.lock: {}", err),
    };

    for line in data.lines() {
        if let Some(data) = line.strip_prefix("name = ") {
            name = data.to_string();
            name_flag = true;
        }
        if name_flag {
            if let Some(data) = line.strip_prefix("version = ") {
                name_flag = false;
                // (name, version)
                packages.push_str(&format!("    ({}, {}),\n", name, data));
                counter += 1;
            }
        }
    }

    format!("pub static APP_PACKAGES: [(&str, &str); {}] = [\n{}];", counter, packages)
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
            let fmt_str = format!("pub static {}_{}: &str = \"{}\";\n", prefix, key, v);
            source_code.push_str(&fmt_str);
        }
    }

    // add project info
    let git_hash = include_str!(".git/ORIG_HEAD").trim();
    let git_branch = include_str!(".git/HEAD").rsplitn(2, '/').next().unwrap_or("-").trim();
    let project_info = format!(
        "// project info\n\
        pub static GIT_PROJECT_BRANCH: &str = \"{}\";\n\
        pub static GIT_PROJECT_HASH: &str = \"{}\";\n\
        pub static PROJECT_BUILD_DATE: &str = \"{}\"; // UTC+0\n\
        // packages\n",
        git_branch,
        git_hash,
        get_current_date()
    );
    source_code.push_str(&project_info);

    // add packages in Cargo.lock
    source_code.push_str(&app_packages());

    // and write to build.rs file
    match File::create("src/build.rs").and_then(|mut file| write!(file, "{}", source_code)) {
        Ok(_) => (),
        Err(err) => panic!("Cannot create `build.rs`: {}", err),
    }

    // set icon for windows binary
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("extra/icon.ico");
        match res.compile() {
            Ok(_) => (),
            Err(err) => panic!("Cannot compile winres: {}", err),
        }
    }
}
