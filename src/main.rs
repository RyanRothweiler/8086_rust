use std::env;

#[cfg(test)]
#[allow(unused_variables, dead_code)]
mod tests;

#[allow(unused_variables, dead_code)]
mod parser;

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
enum Instruction {
    None,
    Mov,
    Add,
    Sub,
    Cmp,
}

#[derive(Debug, PartialEq, Eq)]
enum Register {
    None,
    Ax,
    Cx,
    Dx,
    Bx,
    Sp,
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
struct EffectiveAddress {
    first_operand: Register,
    second_operand: Register,
    offset: u16,
}

#[derive(Debug, PartialEq, Eq)]
enum Address {
    //None,
    Register(Register),
    EffectiveAddress(EffectiveAddress),
}

#[derive(Debug, PartialEq, Eq)]
struct RegMemToRegMem {
    source: Address,
    dest: Address,
}

impl RegMemToRegMem {
    fn new() -> RegMemToRegMem {
        RegMemToRegMem {
            source: Address::Register(Register::None),
            dest: Address::Register(Register::None),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ImmediateToRegMem {
    immediate: u16,
    dest: Address,
}

impl ImmediateToRegMem {
    fn new() -> ImmediateToRegMem {
        ImmediateToRegMem {
            immediate: 0,
            dest: Address::Register(Register::None),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ImmediateToReg {
    immediate: u16,
    dest: Register,
}

impl ImmediateToReg {
    fn new() -> ImmediateToReg {
        ImmediateToReg {
            immediate: 0,
            dest: Register::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ImmediateToAccumulator {
    dest: Register,
    immediate: u16,
}

impl ImmediateToAccumulator {
    fn new() -> ImmediateToAccumulator {
        ImmediateToAccumulator {
            dest: Register::None,
            immediate: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Encoding {
    //None,
    RegMemToRegMem(RegMemToRegMem),
    ImmediateToReg(ImmediateToReg),
    ImmediateToRegMem(ImmediateToRegMem),
    ImmediateToAccumulator(ImmediateToAccumulator),
}

impl Encoding {
    fn new_reg_mem_to_reg_mem() -> Encoding {
        Encoding::RegMemToRegMem(RegMemToRegMem::new())
    }

    fn new_immediate_to_reg() -> Encoding {
        Encoding::ImmediateToReg(ImmediateToReg::new())
    }

    fn new_immediate_to_reg_mem() -> Encoding {
        Encoding::ImmediateToRegMem(ImmediateToRegMem::new())
    }

    fn new_immediate_to_accumulator() -> Encoding {
        Encoding::ImmediateToAccumulator(ImmediateToAccumulator::new())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    instruction: Instruction,
    encoding: Encoding,
}

impl Command {
    fn print(&self) {
        let instruction = format!("{:?}", self.instruction).to_lowercase();
        print!("{instruction}");

        match &self.encoding {
            Encoding::RegMemToRegMem(data) => {
                let dest = format!("{:?}", data.dest).to_lowercase();
                let source = format!("{:?}", data.source).to_lowercase();
                println!("{dest}, {source}");
            }
            _ => {
                panic!("Unknown encoding");
            }
        }
    }
}

struct Asm {
    list: Vec<u8>,
    index: usize,
}

impl Asm {
    fn new(file_path: &str) -> Asm {
        let mut asm = Asm {
            list: vec![],
            index: 0,
        };

        asm.list = std::fs::read(file_path).expect(&format!("Error reading file {file_path}"));

        asm
    }

    fn pull_byte(&mut self) -> Option<&u8> {
        let ret = self.list.get(self.index);
        self.index += 1;

        ret
    }
}
