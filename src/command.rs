#![allow(dead_code)]

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Command {
    pub instruction: Instruction,
    pub encoding: Encoding,
}

impl Command {
    pub fn print(&self) {
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
