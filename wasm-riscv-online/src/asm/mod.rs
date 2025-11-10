#![allow(dead_code)]
pub use rv32i::RV32I;
pub use rv64i::RV64I;
pub use rvc::RVC;
pub use rvf::RVF;
pub use rvzicsr::RVZicsr;
pub use rva::RV32A;
pub use rva::RV64A;
pub use rva::RV128A;
pub use rv64d::RV64D;
pub use rvb::RVB;
use crate::riscv::imm::{Imm, Uimm};

pub mod rv32i;
pub mod rv64i;
pub mod rvc;
pub mod rvf;
pub mod rvzicsr;
pub mod rva;
pub mod rv64d;
pub mod rvb;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    RV32I(RV32I),
    RV64I(RV64I),
    RVC(RVC),
    RVZicsr(RVZicsr),
    RVF(RVF),
    RV32A(RV32A),
    RV64A(RV64A),
    RV128A(RV128A),
    RV64D(RV64D),
    RVB(RVB),
}

impl Instruction {
    pub fn disassembly(&self) -> String {
        match self {
            Self::RV32I(rv32i) => rv32i.to_string(),
            Self::RV64I(rv64i) => rv64i.to_string(),
            Self::RVC(rvc) => rvc.to_string(),
            Self::RVZicsr(rvzicsr) => rvzicsr.to_string(),
            Self::RVF(rvf) => rvf.to_string(),
            Self::RV32A(rv32a) => rv32a.to_string(),
            Self::RV64A(rv64a) => rv64a.to_string(),
            Self::RV128A(rv128a) => rv128a.to_string(),
            Self::RV64D(rv64d) => rv64d.to_string(),
            Self::RVB(rvb) => rvb.to_string(),
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

impl From<RV32A> for Instruction {
    fn from(src: RV32A) -> Instruction {
        Instruction::RV32A(src)
    }
}

impl From<RV64A> for Instruction {
    fn from(src: RV64A) -> Instruction {
        Instruction::RV64A(src)
    }
}

impl From<RV128A> for Instruction {
    fn from(src: RV128A) -> Instruction {
        Instruction::RV128A(src)
    }
}

impl From<RV64D> for Instruction {
    fn from(src: RV64D) -> Instruction {
        Instruction::RV64D(src)
    }
}

impl From<RVB> for Instruction {
    fn from(src: RVB) -> Instruction {
        Instruction::RVB(src)
    }
}

impl Instruction {
    pub fn to_string(&self) -> String {
        match self {
            Self::RV64D(instr) => instr.to_string(),
            Self::RVB(instr) => instr.to_string(),
            _ => String::from("Unimplemented instruction"),
        }
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

pub fn from_register(name: &str) -> Option<u8> {
    let s = name.trim().to_lowercase();
    
    // 浮点寄存器 - fN 格式
    if let Some(num) = s.strip_prefix('f') {
        if let Ok(n) = num.parse::<u8>() {
            if n <= 31 { return Some(n); }
        }
    }
    
    // 整数寄存器 - xN 格式
    if let Some(num) = s.strip_prefix('x') {
        if let Ok(n) = num.parse::<u8>() {
            if n <= 31 { return Some(n); }
        }
    }
    
    // ABI names
    match s.as_str() {
        // 整数寄存器
        "zero" => Some(0),
        "ra" => Some(1),
        "sp" => Some(2),
        "gp" => Some(3),
        "tp" => Some(4),
        "t0" => Some(5),
        "t1" => Some(6),
        "t2" => Some(7),
        "s0" | "fp" => Some(8),
        "s1" => Some(9),
        "a0" => Some(10),
        "a1" => Some(11),
        "a2" => Some(12),
        "a3" => Some(13),
        "a4" => Some(14),
        "a5" => Some(15),
        "a6" => Some(16),
        "a7" => Some(17),
        "s2" => Some(18),
        "s3" => Some(19),
        "s4" => Some(20),
        "s5" => Some(21),
        "s6" => Some(22),
        "s7" => Some(23),
        "s8" => Some(24),
        "s9" => Some(25),
        "s10" => Some(26),
        "s11" => Some(27),
        "t3" => Some(28),
        "t4" => Some(29),
        "t5" => Some(30),
        "t6" => Some(31),
        
        // 浮点寄存器
        "ft0" => Some(0),
        "ft1" => Some(1),
        "ft2" => Some(2),
        "ft3" => Some(3),
        "ft4" => Some(4),
        "ft5" => Some(5),
        "ft6" => Some(6),
        "ft7" => Some(7),
        "fs0" => Some(8),
        "fs1" => Some(9),
        "fa0" => Some(10),
        "fa1" => Some(11),
        "fa2" => Some(12),
        "fa3" => Some(13),
        "fa4" => Some(14),
        "fa5" => Some(15),
        "fa6" => Some(16),
        "fa7" => Some(17),
        "fs2" => Some(18),
        "fs3" => Some(19),
        "fs4" => Some(20),
        "fs5" => Some(21),
        "fs6" => Some(22),
        "fs7" => Some(23),
        "fs8" => Some(24),
        "fs9" => Some(25),
        "fs10" => Some(26),
        "fs11" => Some(27),
        "ft8" => Some(28),
        "ft9" => Some(29),
        "ft10" => Some(30),
        "ft11" => Some(31),
        
        _ => None,
    }
}
