use crate::error;
use crate::program;
use crate::parser;

/// Read bytes to code
pub fn convert (bytes: &mut Vec<u8>) -> String {
    if !bytes.starts_with(&[0xFF, 0x10, 0xFA, 0xF8]) {
        panic!("{}", error::error_file_format());
    }
    bytes.drain(0..4);
    bytes.chunks_exact(2)
        .map(|chunk| {
            let num = u16::from_le_bytes([ chunk[0], chunk[1] ]);
            char::from_u32(num as u32).unwrap_or(' ')
        })
        .collect()
}

/// Read code text file
pub fn text(data: &String) -> Vec<(u8, i64)> {
    let mut local: Vec<(u8, i64)> = vec![];
    for line in data.lines() {
        let items: Vec<&str> = line.split(',').collect();
        let name: u8 = parser::encode::parse(items[0]);
        let value: i64 = match items.len() {
            1 => 0,
            2 => match items[1] {
                "'\\n'" => 10,
                i if i.len() >= 3 && i.starts_with('\'') && i.ends_with('\'') => {
                    let ch = i.chars().nth(1).unwrap_or('?');
                    ch as i64
                },
                _ => items[1].parse().unwrap_or(0)
            },
            _ => panic!("{}: {:?}", error::error_line_empty(), local.len())
        };
        local.push((name, value));
    }

    local
}

/// Run .prog
pub fn run (bytes: &mut Vec<u8>) {
    let code = convert(bytes);
    let prog = text(&code);
    program::run(&prog);
}
