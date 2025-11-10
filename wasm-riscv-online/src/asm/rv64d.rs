#![allow(dead_code)]
use std::fmt::Display;
use crate::asm::{IType, RType, SType, R4Type, to_register};

#[derive(Debug, Clone, Copy)]
pub enum RV64D {
    // Load/Store
    Fld(IType),
    Fsd(SType),
    
    // FMADD Group
    FmaddD(R4Type),
    FmsubD(R4Type),
    FnmsubD(R4Type),
    FnmaddD(R4Type),
    
    // Arithmetic
    FaddD(RType),
    FsubD(RType),
    FmulD(RType),
    FdivD(RType),
    FsqrtD(RType),
    
    // Comparisons
    FeqD(RType),
    FltD(RType),
    FleD(RType),
    
    // Min/Max
    FminD(RType),
    FmaxD(RType),
    
    // Sign-Injection
    FsgnjD(RType),
    FsgnjnD(RType),
    FsgnjxD(RType),
    
    // Conversions (FP <-> Integer)
    FcvtWD(RType),     // fcvt.w.d
    FcvtWUD(RType),    // fcvt.wu.d
    FcvtLD(RType),     // fcvt.l.d
    FcvtLUD(RType),    // fcvt.lu.d
    FcvtDW(RType),     // fcvt.d.w
    FcvtDWU(RType),    // fcvt.d.wu
    FcvtDL(RType),     // fcvt.d.l
    FcvtDLU(RType),    // fcvt.d.lu
    
    // Conversions (Single <-> Double)
    FcvtSD(RType),     // fcvt.s.d
    FcvtDS(RType),     // fcvt.d.s
    
    // Move and Classify
    FmvXD(RType),      // fmv.x.d
    FmvDX(RType),      // fmv.d.x
    FclassD(RType),    // fclass.d
}

impl RV64D {
    pub fn to_string(&self) -> String {
        match self {
            Self::Fld(i) => format!("fld f{}, {:?}({})", i.rd, i.imm, to_register(i.rs1)),
            Self::Fsd(s) => format!("fsd f{}, {:?}({})", s.rs2, s.imm, to_register(s.rs1)),
            _ => String::from("RV64D instruction"),
        }
    }
}

impl Display for RV64D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}