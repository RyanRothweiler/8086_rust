use std::env;

const MOV_REG_MEM: u8 = 0b_1000_1000;

#[cfg(test)]
#[allow(unused_variables, dead_code)]
mod tests;

mod parser;

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Expected a file argument.");
        return;
    }

    let mut asm = Asm {
        list: vec![],
        index: 0,
    };

    // load file
    {
        let file_path = &args[1];
        asm.list = std::fs::read(file_path).expect(&format!("Error reading file {file_path}"));
    }

    // parse instructions
    loop {
        let command = parser::pull_command(&mut asm);
        match command {
            Some(v) => v.print(),
            None => break,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    None,
    Mov,
}

#[derive(Debug, PartialEq, Eq)]
enum Register {
    None,
    Ax,
    Cx,
    Dx,
    Bx,
    Ap,
    Bp,
    Si,
    Di,
    Al,
    Cl,
    Dl,
    Bl,
    Ah,
    Ch,
    Dh,
    Bh,
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    instruction: Instruction,
    source: Register,
    dest: Register,
}

impl Command {
    fn print(&self) {
        let instruction = format!("{:?}", self.instruction).to_lowercase();
        let dest = format!("{:?}", self.dest).to_lowercase();
        let source = format!("{:?}", self.source).to_lowercase();

        println!("{instruction} {dest}, {source}");
    }
}

struct Asm {
    list: Vec<u8>,
    index: usize,
}

impl Asm {
    fn pull_byte(&mut self) -> Option<&u8> {
        let ret = self.list.get(self.index);
        self.index += 1;

        ret
    }
}

