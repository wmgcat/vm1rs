pub fn error_unknown_command() -> &'static str {
    "[ERR00] Unknown command"
}

pub fn error_file() -> &'static str {
    "[ERR01] Read a file"
}

pub fn error_create_file() -> &'static str {
    "[ERR02] Create a file"
}

pub fn error_file_write() -> &'static str {
    "[ERR03] Write a file"
}

pub fn error_file_format() -> &'static str {
    "[ERR04] Support only .prog files"
}

pub fn error_stack_empty() -> &'static str {
    "[ERR05] Stack is empty"
}

pub fn error_jump_empty() -> &'static str {
    "[ERR06] Jump stack is empty"
}

pub fn error_jump_limit() -> &'static str {
    "[ERR07] Jump limit 1000000"
}

pub fn error_exit() -> &'static str {
    "[ERR08] Exit without HALT command"
}

pub fn error_zero() -> &'static str {
    "[ERR09] Imposible on this galaxy"
}

pub fn error_line_empty() -> &'static str {
    "[ERR10] Empty line"
}

pub fn error_unknown() -> &'static str {
    "[ERR11] Unknown"
}
