#[cfg(windows)]
extern crate winres;
use std::collections::HashMap;
use std::process::Command;
use std::{fs::File, io::Write};

fn execute(cmd: &str) -> String {
    match Command::new(cmd).arg("-vV").output() {
        Ok(value) => match String::from_utf8(value.stdout) {
            Ok(value) => value,
            Err(_) => "unknown".to_string()
        },
        Err(_) => "unknown".to_string()
    }
}

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

fn main() {
    // generate build info
    let mut source_code = "#![allow(dead_code)]\n".to_string();
    for (prefix, executable) in [("RUST", "rustc"), ("CARGO", "cargo")].iter() {
        let iterator = parse(execute(executable));
        for (k, v) in iterator {
            let key = k.to_uppercase().replace("-", "_").replace(" ", "_");
            let fmt_str = format!("pub static {}_{}: &'static str = \"{}\";\n", prefix, key, v);
            source_code.push_str(&fmt_str);
        }
    }
    File::create("src/build.rs")
        .and_then(|mut file| write!(file, "{}", source_code))
        .unwrap();

    // set icon for windows binary
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico");
        res.compile().unwrap();
    }
}