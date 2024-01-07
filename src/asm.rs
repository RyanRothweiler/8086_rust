pub struct Asm {
    list: Vec<u8>,
    index: usize,
}

impl Asm {
    pub fn new(file_path: &str) -> Asm {
        Asm {
            list: std::fs::read(file_path).expect(&format!("Error reading file {file_path}")),
            index: 0,
        }
    }

    pub fn pull_byte(&mut self) -> Option<&u8> {
        let ret = self.list.get(self.index);
        self.index += 1;
        ret
    }

    pub fn set_instruction_pointer(&mut self, ip: usize) {
        self.index = ip;
    }
}
