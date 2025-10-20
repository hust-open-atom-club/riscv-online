#![allow(dead_code)]
use super::{to_register, RType};

#[derive(Debug, Clone, Copy)]
pub enum RV32A {
    // Load-Reserved/Store-Conditional - Word (32-bit)
    Lrw(RType),
    Scw(RType),
    
    // Atomic Memory Operations - Word (32-bit)
    Amoswapw(RType),
    Amoaddw(RType),
    Amoxorw(RType),
    Amoandw(RType),
    Amoorw(RType),
    Amominw(RType),
    Amomaxw(RType),
    Amominuw(RType),
    Amomaxuw(RType),
}

#[derive(Debug, Clone, Copy)]
pub enum RV64A {
    // Load-Reserved/Store-Conditional - Double Word (64-bit)
    Lrd(RType),
    Scd(RType),
    
    // Atomic Memory Operations - Double Word (64-bit)
    Amoswapd(RType),
    Amoaddd(RType),
    Amoxord(RType),
    Amoandd(RType),
    Amoord(RType),
    Amomind(RType),
    Amomaxd(RType),
    Amominud(RType),
    Amomaxud(RType),
}

impl RV32A {
    pub fn to_string(&self) -> String {
        match self {
            Self::Lrw(r) => format!("lr.w {}, {}", to_register(r.rd), to_register(r.rs1)),
            Self::Scw(r) => format!("sc.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amoswapw(r) => format!("amoswap.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amoaddw(r) => format!("amoadd.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amoxorw(r) => format!("amoxor.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amoandw(r) => format!("amoand.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amoorw(r) => format!("amoor.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amominw(r) => format!("amomin.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amomaxw(r) => format!("amomax.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amominuw(r) => format!("amominu.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amomaxuw(r) => format!("amomaxu.w {}, {}", to_register(r.rs2), to_register(r.rs1)),
        }
    }
}

impl RV64A {
    pub fn to_string(&self) -> String {
        match self {
            Self::Lrd(r) => format!("lr.d {}, {}", to_register(r.rd), to_register(r.rs1)),
            Self::Scd(r) => format!("sc.d {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amoswapd(r) => format!("amoswap.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amoaddd(r) => format!("amoadd.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amoxord(r) => format!("amoxor.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amoandd(r) => format!("amoand.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amoord(r) => format!("amoor.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amomind(r) => format!("amomin.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amomaxd(r) => format!("amomax.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amominud(r) => format!("amominu.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
            Self::Amomaxud(r) => format!("amomaxu.d {} ,{}, {}", to_register(r.rd),to_register(r.rs2), to_register(r.rs1)),
        }
    }
}
 
#[derive(Debug, Clone, Copy)]
pub enum RV128A {
    // Load-Reserved/Store-Conditional - Quad Word (128-bit)
    Lrq(RType),
    Scq(RType),
    
    // Atomic Memory Operations - Quad Word (128-bit)
    Amoswapq(RType),
    Amoaddq(RType),
    Amoxorq(RType),
    Amoandq(RType),
    Amoorq(RType),
    Amominq(RType),
    Amomaxq(RType),
    Amominuq(RType),
    Amomaxuq(RType),
}

impl RV128A {
    pub fn to_string(&self) -> String {
        match self {
            Self::Lrq(r) => format!("lr.q {}, {}", to_register(r.rd), to_register(r.rs1)),
            Self::Scq(r) => format!("sc.q {}, {}", to_register(r.rs2), to_register(r.rs1)),
            Self::Amoswapq(r) => format!("amoswap.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amoaddq(r) => format!("amoadd.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amoxorq(r) => format!("amoxor.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amoandq(r) => format!("amoand.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amoorq(r) => format!("amoor.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amominq(r) => format!("amomin.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amomaxq(r) => format!("amomax.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amominuq(r) => format!("amominu.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
            Self::Amomaxuq(r) => format!("amomaxu.q {} ,{}, {}", to_register(r.rd), to_register(r.rs2), to_register(r.rs1)),
        }
    }
}
        