use crate::error;

/// Parse number codes to string codes
pub fn parse(command: &u8) -> &'static str {
    match command {
        0   => "HALT",
        1   => "PUSH",
        2   => "POP",
        3   => "DUP",
        4   => "SWAP",
        5   => "OVER",
        6   => "ROT",
        7   => "ADD",
        8   => "SUB",
        9   => "JMP",
        10  => "JZ",
        11  => "CALL",
        12  => "RET",
        13  => "PRINT",
        14  => "LT",
        15  => "AND",
        16  => "EMIT",
        17  => "MUL",
        18  => "DIV",
        19  => "MOD",
        20  => "READ",
        21  => "LOAD",
        22  => "STORE",
        23  => "LOADI",
        24  => "STOREI",
        25  => "KEY",
        _   => panic!("{}: {}", error::error_unknown_command(), command)
    }
}
