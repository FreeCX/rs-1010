use std::panic;
use std::{fs::File, io::Write};

use crate::build;

// handle panic and write crash repot to file
pub fn panic_handler(panic_info: &panic::PanicInfo) {
    let mut buffer = String::new();

    buffer.push_str(&format!(
        "The application had a problem and crashed.\n\
         To help us diagnose the problem you can send us a crash report.\n\n\
         Author(s): {}\n\n\
         We take privacy seriously, and do not perform any automated error collection.\n\
         In order to improve the software, we rely on people to submit reports.\n\n\
         Thank you!\n\n\
         --- crash report start ---\n\
         name: {}\n\
         version: {} ({} {})\n\
         compiler: {}\n\
         package manager: {}\n\
         host: {}\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        build::GIT_PROJECT_HASH,
        build::PROJECT_BUILD_DATE,
        build::RUST_HEADER,
        build::CARGO_HEADER,
        build::RUST_HOST,
    ));

    buffer.push_str("packages:\n");
    for (name, version) in build::APP_PACKAGES.iter() {
        buffer.push_str(&format!("  {} {}\n", name, version));
    }
    buffer.push_str("\n");

    match panic_info.location() {
        Some(location) => {
            let info = format!("panic occurred in file '{}' at line {}\n", location.file(), location.line());
            buffer.push_str(&info);
        }
        None => buffer.push_str("panic occurred but can't get location information...\n"),
    }

    buffer.push_str("stack backtrace:\n");

    let mut index = 0;
    backtrace::trace(|frame| {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();
        backtrace::resolve(ip, |symbol| {
            if let Some(name) = symbol.name() {
                let symbol_info = format!("\t{}: {} @ {:?}\n", index, name, symbol_address);
                buffer.push_str(&symbol_info);
                index += 1;
            }
            match (symbol.filename(), symbol.lineno()) {
                (Some(filename), Some(line)) => {
                    let file_info = format!("\t\t\tat {}:{}\n", filename.display(), line);
                    buffer.push_str(&file_info);
                }
                _ => {}
            }
        });
        true
    });

    buffer.push_str("--- crash report end ---");

    File::create("crash.log")
        .and_then(|mut file| write!(file, "{}", buffer))
        .unwrap_or_else(|_| println!("{}", buffer));
}
