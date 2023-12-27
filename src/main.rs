use std::env;

const MOV_REG_MEM: u8 = 0b_1000_1000;

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
        let first_byte: u8 = match asm.pull_byte() {
            Some(v) => *v,
            None => break,
        };
        let second_byte: u8 = match asm.pull_byte() {
            Some(v) => *v,
            None => break,
        };

        let instruction_mask: u8 = 0b_1111_1100;

        let w_mask: u8 = 0b_0000_0001;
        let mod_mask: u8 = 0b_1100_0000;
        let first_reg_mask: u8 = 0b_0011_1000;
        let second_reg_mask: u8 = 0b_0000_0111;

        let instruction_val: u8 = first_byte & instruction_mask;
        let w_val: bool = (first_byte & w_mask) == 1;
        let mod_val: u8 = second_byte & mod_mask;
        let first_reg: u8 = (second_byte & first_reg_mask) << 2;
        let second_reg: u8 = (second_byte & second_reg_mask) << 5;

        match instruction_val {
            MOV_REG_MEM => {

                if mod_val != 0b1100_0000 {
                    panic!("Unknown mod");
                }

                let first_reg = decode_register(first_reg, w_val);
                let second_reg = decode_register(second_reg, w_val);

                print!("mov {second_reg}, {first_reg}");
                println!("");
            }
            _ => {
                panic!("Unknown instruction");
            }
        }
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

fn decode_register(input: u8, w_set: bool) -> String {
    if w_set {
        match input {
            0b0000_0000 => "ax".to_string(),
            0b0010_0000 => "cx".to_string(),
            0b0100_0000 => "dx".to_string(),
            0b0110_0000 => "bx".to_string(),

            0b1000_0000 => "ap".to_string(),
            0b1010_0000 => "bp".to_string(),
            0b1100_0000 => "si".to_string(),
            0b1110_0000 => "di".to_string(),

            _ => panic!("Unknown register encoding"),
        }
    } else {
        match input {
            0b0000_0000 => "al".to_string(),
            0b0001_0000 => "cl".to_string(),
            0b0010_0000 => "dl".to_string(),
            0b0011_0000 => "bl".to_string(),

            0b0100_0000 => "ah".to_string(),
            0b0101_0000 => "ch".to_string(),
            0b0110_0000 => "dh".to_string(),
            0b0111_0000 => "bh".to_string(),

            _ => panic!("Unknown register encoding"),
        }
    }
}
