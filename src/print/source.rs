use crate::parser::decode;

/// Print source code
pub fn print(program: &Vec<(u8, i64)>) -> () {
    let mut line: i64 = 0;
    println!(" line\tcmd\tvalue");
    println!("{}", "-".repeat(20));
    for (command, value) in program {
        println!("{line:04}\t{:?}\t{value}", decode::parse(&command));
        line += 1;
    }
    println!("{}", "-".repeat(20));
    println!("lines: {}", program.len());
    print!("\n");
}
