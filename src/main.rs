use std::env;

use std::io::stdin;

mod command;
mod encoding;
mod parser;
mod simulator;

use command::*;
use encoding::*;

#[allow(dead_code)]
struct Computer {
    cpu: simulator::Cpu,
    program: Vec<u8>,
    memory: [i16; 2000],
}

impl Computer {
    fn new(prog_path: &str) -> Computer {
        Computer {
            cpu: simulator::Cpu::new(),
            program: std::fs::read(prog_path).expect(&format!("Error reading file {prog_path}")),
            memory: [0; 2000],
        }
    }

    pub fn pull_byte(&mut self) -> Option<&u8> {
        let i : usize = usize::try_from(self.cpu.instruction_pointer).expect("Cannot convert instruction pointer to usize.");

        let ret = self.program.get(i);
        self.cpu.instruction_pointer += 1;
        ret
    }

    pub fn simulate(&mut self, command: Command) { 
        self.cpu.simulate(command, &mut self.memory);
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

        computer.simulate(command);
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
