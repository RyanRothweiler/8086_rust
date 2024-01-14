use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EffectiveAddress {
    pub first_operand: Register,
    pub second_operand: Register,
    pub offset: u16,
    pub direct: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Address {
    //None,
    Register(Register),
    EffectiveAddress(EffectiveAddress),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RegMemToRegMem {
    pub source: Address,
    pub dest: Address,
}

#[allow(dead_code)]
impl RegMemToRegMem {
    pub fn new() -> RegMemToRegMem {
        RegMemToRegMem {
            source: Address::Register(Register::None),
            dest: Address::Register(Register::None),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImmediateToRegMem {
    pub immediate: u16,
    pub dest: Address,
}

impl ImmediateToRegMem {
    pub fn new() -> ImmediateToRegMem {
        ImmediateToRegMem {
            immediate: 0,
            dest: Address::Register(Register::None),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImmediateToReg {
    pub immediate: u16,
    pub dest: Register,
}

impl ImmediateToReg {
    fn new() -> ImmediateToReg {
        ImmediateToReg {
            immediate: 0,
            dest: Register::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImmediateToAccumulator {
    pub dest: Register,
    pub immediate: u16,
}

impl ImmediateToAccumulator {
    pub fn new() -> ImmediateToAccumulator {
        ImmediateToAccumulator {
            dest: Register::None,
            immediate: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Encoding {
    //None,
    RegMemToRegMem(RegMemToRegMem),
    ImmediateToReg(ImmediateToReg),
    ImmediateToRegMem(ImmediateToRegMem),
    ImmediateToAccumulator(ImmediateToAccumulator),
    Jump(i8),
}

#[allow(dead_code)]
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
