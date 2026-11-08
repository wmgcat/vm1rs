use crate::error;

/// Parse string codes to number codes
pub fn parse(command: &str) -> u8 {
    match command {
        "HALT"  => 0,
        "PUSH"  => 1,
        "POP"   => 2,
        "DUP"   => 3,
        "SWAP"  => 4,
        "OVER"  => 5,
        "ROT"   => 6,
        "ADD"   => 7,
        "SUB"   => 8,
        "JMP"   => 9,
        "JZ"    => 10,
        "CALL"  => 11,
        "RET"   => 12,
        "PRINT" => 13,
        "LT"    => 14,
        "AND"   => 15,
        "EMIT"  => 16,
        "MUL"   => 17,
        "DIV"   => 18,
        "MOD"   => 19,
        "READ"  => 20,
        "LOAD"  => 21,
        "STORE" => 22,
        "LOADI" => 23,
        "STOREI"=> 24,
        "KEY"   => 25,
        _       => panic!("{}: {}", error::error_unknown_command(), command)
    }
}
