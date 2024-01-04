use std::env;

mod asm;
mod command;
mod encoding;
mod parser;
mod simulator;

use asm::*;
use command::*;
use encoding::*;

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Expected a file argument.");
        return;
    }

    // load file
    let file_path = &args[1];
    let mut asm = Asm::new(file_path);

    // parse instructions
    loop {
        let command = parser::pull_command(&mut asm);
        match command {
            Some(v) => v.print(),
            None => break,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
enum Instruction {
    None,
    Mov,
    Add,
    Sub,
    Cmp,
    Jnz,
    Je,
    Jl,
    Jle,
    Jb,
    Jbe,
    Jp,
    Jo,
    Js,
    Jnl,
    Jg,
    Jnb,
    Ja,
    Jnp,
    Jno,
    Jns,
    Loop,
    Loopz,
    Loopnz,
    Jcxz,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Register {
    None,

    Ax,
    Bx,
    Cx,
    Dx,

    Sp,
    Bp,
    Si,
    Di,

    Al,
    Bl,
    Cl,
    Dl,

    Ah,
    Bh,
    Ch,
    Dh,
}
