#![allow(dead_code)]
use std::fmt::Display;
use crate::asm::{IType, RType, BType, to_register};

#[derive(Debug, Clone, Copy)]
pub enum RVB {
    // Basic Bit Manipulation
    // Shifts with Variable Shift Amount
    Sllw(RType),     // Shift Left Logical Word
    Srlw(RType),     // Shift Right Logical Word
    Sraw(RType),     // Shift Right Arithmetic Word
    Ror(RType),      // Rotate Right
    Rori(IType),     // Rotate Right Immediate
    Rorw(RType),     // Rotate Right Word
    Roriw(IType),    // Rotate Right Immediate Word
    
    // Bitwise Operations
    Andn(RType),     // AND NOT
    Orn(RType),      // OR NOT
    Xnor(RType),     // XNOR
    
    // Single-Bit Instructions
    Bset(RType),     // Bit Set
    Bseti(IType),    // Bit Set Immediate
    Bclr(RType),     // Bit Clear
    Bclri(IType),    // Bit Clear Immediate
    Binv(RType),     // Bit Invert
    Binvi(IType),    // Bit Invert Immediate
    Bext(RType),     // Bit Extract
    Bexti(IType),    // Bit Extract Immediate
    
    // Bit Field Instructions
    BEXT(RType),     // Bit Field Extract
    BEXTI(IType),    // Bit Field Extract Immediate
    BCLR(RType),     // Bit Field Clear
    BCLRI(IType),    // Bit Field Clear Immediate
    BSET(RType),     // Bit Field Set
    BSETI(IType),    // Bit Field Set Immediate
    BINVERT(RType),  // Bit Field Invert
    BINVERTI(IType), // Bit Field Invert Immediate
    
    // Bit Count Instructions
    Cpop(RType),     // Count Population
    Cpopw(RType),    // Count Population Word
    Ctz(RType),      // Count Trailing Zeros
    CtzW(RType),     // Count Trailing Zeros Word
    Clz(RType),      // Count Leading Zeros
    ClzW(RType),     // Count Leading Zeros Word
    FFS(RType),      // Find First Set
    FFSW(RType),     // Find First Set Word
    
    // Single-Bit Instructions (Word variants)
    BsetW(RType),    // Bit Set Word
    BsetiW(IType),   // Bit Set Immediate Word
    BclrW(RType),    // Bit Clear Word
    BclriW(IType),   // Bit Clear Immediate Word
    BinvW(RType),    // Bit Invert Word
    BinviW(IType),   // Bit Invert Immediate Word
    BextW(RType),    // Bit Extract Word
    BextiW(IType),   // Bit Extract Immediate Word
    
    // Deposit Instructions
    Pack(RType),     // Pack
    PackW(RType),    // Pack Word
    
    // Bit Manipulation - Advanced
    Grevi(RType),    // Generalized Reverse Immediate
    GreviW(RType),   // Generalized Reverse Immediate Word
    Gorb(RType),     // Generalized OR
    Gorhi(RType),    // Generalized OR High Immediate
    GorbW(RType),    // Generalized OR Word
    GorhiW(RType),   // Generalized OR High Immediate Word
    Gxori(RType),    // Generalized XOR Immediate
    GxoriW(RType),   // Generalized XOR Immediate Word
}

impl RVB {
    pub fn to_string(&self) -> String {
        match self {
            Self::Bset(r) => format!("bset {}, {}, {}", to_register(r.rd), to_register(r.rs1), r.rs2),
            Self::Bext(r) => format!("bext {}, {}, {}", to_register(r.rd), to_register(r.rs1), r.rs2),
            _ => String::from("RVB instruction"),
        }
    }
}

impl Display for RVB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}