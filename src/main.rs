mod print;
mod parser;
mod program;
mod build;
mod error;
use std::io::Write;
use std::fs;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let msg = info.payload_as_str().unwrap_or(error::error_unknown());
        eprintln!("{}", msg);
    }));

    let mut arg = std::env::args();
    if arg.len() > 2 {
        let command: &str = &arg.nth(1).unwrap();
        match command {
            "test" => {
                let source: &str = &arg.nth(0).unwrap();
                let content = String::from(fs::read_to_string(&source).expect(error::error_file()));
                let code = build::text(&content);
                print::compile::print(&code);
                program::run(&code);
            },
            "build" => {
                let source: &str = &arg.nth(0).unwrap();
                let target: &str = &arg.nth(0).unwrap();
                let content = String::from(fs::read_to_string(&source).expect(error::error_file()));
                let code = build::text(&content);
                print::compile::print(&code);
                let bytes: Vec<u8> = content
                    .encode_utf16()
                    .flat_map(|ch| ch.to_le_bytes())
                    .collect();
                let file = std::fs::File::create(target).expect(error::error_create_file());
                let mut writer = std::io::BufWriter::new(file);
                writer.write_all(&[0xFF, 0x10, 0xFA, 0xF8]).expect(error::error_file_write());
                writer.write_all(&bytes).expect(error::error_file_write());
                println!("ok");
            },
            "run" => {
                let source: &str = &arg.nth(0).unwrap();
                let mut bytes = std::fs::read(source).expect(error::error_file());
                build::run(&mut bytes);
            },
            "view" => {
                let source: &str = &arg.nth(0).unwrap();
                let mut bytes = std::fs::read(source).expect(error::error_file());
                let code: String = build::convert(&mut bytes);
                let code: Vec<(u8, i64)> = build::text(&code);
                print::source::print(&code);
            },
            _ => panic!("{}: {}", error::error_unknown_command(), command)
        }
    }
    else {
        let source: &str = &arg.nth(1).unwrap();
        match source {
            "help" => {
                println!("Hello, world!");
                println!("Commands:");
                println!("run <filename> - Run .prog file");
                println!("view <filename> - View .prog file source code");
                println!("test <filename> - Test and run code");
                println!("build <source> <target> - Build code to .prog file");
                println!("");
            },
            _ => {
                let mut bytes = std::fs::read(source).expect(error::error_file());
                build::run(&mut bytes);
            }
        }
    }
}
