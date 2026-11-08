use crate::parser::decode;
use crate::error;
use std::io::Write;

/// Run program stack
pub fn run(program: &Vec<(u8, i64)>) -> () {
    let mut pc: usize = 0;
    let mut stack: Vec<i64> = Vec::new();
    let mut mem = [0i64; 256];
    let mut jump_stack: Vec<usize> = Vec::new();
    let mut jump_limit: usize = 0;
    loop {
        if pc >= program.len() {
            panic!("{}", error::error_exit());
        }
        if jump_limit > 1_000_000 {
            panic!("{}", error::error_jump_limit());
        }
        let code: u8 = program[pc].0;
        let value = program[pc].1;
        pc += 1;
        match code {
            0 => break,
            1 => stack.push(value),
            2 => { stack.pop(); },
            3 => stack.push(*stack.last().expect(error::error_stack_empty())),
            4 => {
                let a = stack.len() - 2;
                let b = stack.len() - 1;
                stack.swap(a, b);
            },
            5 => {
                let a: i64 = *stack.get(stack.len() - 2).unwrap();
                stack.push(a);
            },
            6 => {
                let a: i64 = stack.remove(stack.len() - 3);
                stack.push(a);
            },
            7 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let b: i64 = stack.pop().expect(error::error_stack_empty());
                stack.push(b.wrapping_add(a));
            },
            8 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let b: i64 = stack.pop().expect(error::error_stack_empty());
                stack.push(b.wrapping_sub(a));
            },
            9 => {
                pc = value as usize;
                jump_limit += 1;
            },
            10 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                if a == 0 {
                    pc = value as usize;
                }
            },
            11 => {
                jump_stack.push(pc);
                pc = value as usize;
                jump_limit += 1;
            },
            12 => {
                let pos: usize = jump_stack.pop().expect(error::error_jump_empty());
                pc = pos;
                jump_limit -= 1;
            },
            13 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                print!("{} ", a);
            },
            14 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let b: i64 = stack.pop().expect(error::error_stack_empty());
                stack.push(if a > b { 1 } else { 0 });
            },
            15 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let b: i64 = stack.pop().expect(error::error_stack_empty());
                stack.push(b & a);
            },
            16 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                print!("{}", char::from_u32(a as u32).unwrap_or('?'));
            },
            17 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let b: i64 = stack.pop().expect(error::error_stack_empty());
                stack.push(b.wrapping_mul(a));
            },
            18 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let b: i64 = stack.pop().expect(error::error_stack_empty());

                if a == 0 {
                    panic!("{}", error::error_zero());
                }
                stack.push(b.wrapping_div(a));
            },
            19 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let b: i64 = stack.pop().expect(error::error_stack_empty());
                stack.push(b % a);
            },
            20 => {
                std::io::stdout().flush().unwrap();
                let mut line = String::new();
                if std::io::stdin().read_line(&mut line).unwrap() == 0 {
                    break;
                }
                let value: i64 = line.trim().parse().unwrap_or(0);
                stack.push(value);
            },
            21 => {
                let a = mem[value as usize];
                stack.push(a);
            },
            22 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                mem[value as usize] = a;
            },
            23 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                stack.push(mem[a as usize]);
            },
            24 => {
                let a: i64 = stack.pop().expect(error::error_stack_empty());
                let value: i64 = stack.pop().expect(error::error_stack_empty());
                mem[a as usize] = value;
            },
            25 => {
                std::io::stdout().flush().unwrap();
                let mut line = String::new();
                if std::io::stdin().read_line(&mut line).unwrap() == 0 {
                    break;
                }
                let value = line.trim().chars().next().unwrap_or(' ');
                stack.push(value as i64);
            },
            _ => panic!("{}: {}", error::error_unknown_command(), decode::parse(&code))
        }
    }
    print!("\n");
}
