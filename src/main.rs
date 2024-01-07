use std::env;

use std::io::stdin;

//mod asm;
mod command;
mod encoding;
mod parser;
mod simulator;

//use asm::*;
use command::*;
use encoding::*;

struct Computer {
    cpu: simulator::Cpu,
    program: Vec<u8>,
}

impl Computer {
    fn new(prog_path: &str) -> Computer {
        Computer {
            cpu: simulator::Cpu::new(),
            program: std::fs::read(prog_path).expect(&format!("Error reading file {prog_path}")),
        }
    }

    pub fn pull_byte(&mut self) -> Option<&u8> {
        let ret = self.program.get(self.cpu.instruction_pointer);
        self.cpu.instruction_pointer += 1;
        ret
    }
}

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
    let mut computer = Computer::new(file_path);

    // parse instructions
    loop {
        let command = parser::pull_command(&mut computer);
        let command = match command {
            Some(v) => v,
            None => break,
        };

        computer.cpu.simulate(command);
        computer.cpu.print();

        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
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
