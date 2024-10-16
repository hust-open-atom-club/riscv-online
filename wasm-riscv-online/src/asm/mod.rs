pub use rv32i::RV32I;
pub use rv64i::RV64I;
pub use rvc::RVC;
pub use rvf::RVF;
pub use rvzicsr::RVZicsr;

use crate::riscv::imm::{Imm, Uimm};

pub mod rv32i;
pub mod rv64i;
pub mod rvc;
pub mod rvf;
pub mod rvzicsr;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    RV32I(RV32I),
    RV64I(RV64I),
    RVC(RVC),
    RVZicsr(RVZicsr),
    RVF(RVF),
}

impl Instruction {
    pub fn disassembly(&self) -> String {
        match self {
            Self::RV32I(rv32i) => rv32i.to_string(),
            Self::RV64I(rv64i) => rv64i.to_string(),
            Self::RVC(rvc) => rvc.to_string(),
            Self::RVZicsr(rvzicsr) => rvzicsr.to_string(),
            Self::RVF(rvf) => rvf.to_string(),
        }
    }
}

impl From<RV32I> for Instruction {
    fn from(src: RV32I) -> Instruction {
        Instruction::RV32I(src)
    }
}

impl From<RV64I> for Instruction {
    fn from(src: RV64I) -> Instruction {
        Instruction::RV64I(src)
    }
}

impl From<RVC> for Instruction {
    fn from(src: RVC) -> Instruction {
        Instruction::RVC(src)
    }
}

impl From<RVZicsr> for Instruction {
    fn from(src: RVZicsr) -> Instruction {
        Instruction::RVZicsr(src)
    }
}

impl From<RVF> for Instruction {
    fn from(src: RVF) -> Instruction {
        Instruction::RVF(src)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UType {
    pub rd: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct JType {
    pub rd: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct IType {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct SType {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct BType {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct RType {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct CRType {
    pub rdrs1: u8,
    pub rs2: u8,
    pub funct4: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct CIType {
    pub rdrs1: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CSSType {
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CIWType {
    pub rd: u8,
    pub funct3: u8,
    pub uimm: Uimm,
}

#[derive(Debug, Clone, Copy)]
pub struct CLType {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CSType {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CAType {
    pub rdrs1: u8,
    pub rs2: u8,
    pub funct2: u8,
    pub funct6: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct CBType {
    pub rs1: u8,
    pub funct3: u8,
    pub off: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CJType {
    pub funct3: u8,
    pub target: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CsrRType {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub csr: u16,
}
#[derive(Debug, Clone, Copy)]
pub struct CsrIType {
    pub rd: u8,
    pub uimm: Uimm,
    pub funct3: u8,
    pub csr: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct R4Type {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub rs3: u8,
    pub funct3: u8,
    pub funct2: u8,
}

fn to_register(ins: u8) -> String {
    match ins {
        0 => "zero".to_string(),
        1 => "ra".to_string(),
        2 => "sp".to_string(),
        3 => "gp".to_string(),
        4 => "tp".to_string(),
        5 => "t0".to_string(),
        6 => "t1".to_string(),
        7 => "t2".to_string(),
        8 => "s0".to_string(),
        9 => "s1".to_string(),
        10 => "a0".to_string(),
        11 => "a1".to_string(),
        12 => "a2".to_string(),
        13 => "a3".to_string(),
        14 => "a4".to_string(),
        15 => "a5".to_string(),
        16 => "a6".to_string(),
        17 => "a7".to_string(),
        18 => "s2".to_string(),
        19 => "s3".to_string(),
        20 => "s4".to_string(),
        21 => "s5".to_string(),
        22 => "s6".to_string(),
        23 => "s7".to_string(),
        24 => "s8".to_string(),
        25 => "s9".to_string(),
        26 => "s10".to_string(),
        27 => "s11".to_string(),
        28 => "t3".to_string(),
        29 => "t4".to_string(),
        30 => "t5".to_string(),
        31 => "t6".to_string(),
        _ => "unknown".to_string(), // For values outside the valid register range
    }
}
