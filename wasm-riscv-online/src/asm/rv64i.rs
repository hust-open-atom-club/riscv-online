use super::{to_register, IType, RType, SType};

#[derive(Debug, Clone, Copy)]
pub enum RV64I {
    Lwu(IType),
    Ld(IType),
    Sd(SType),

    Sll(RType),
    Srl(RType),
    Sra(RType),

    Slli(IType),
    Srli(IType),
    Srai(IType),

    Addiw(IType),
    Slliw(IType),
    Srliw(IType),
    Sraiw(IType),

    Addw(RType),
    Subw(RType),
    Sllw(RType),
    Srlw(RType),
    Sraw(RType),
}

impl RV64I {
    pub fn to_string(&self) -> String {
        match self {
            Self::Lwu(i) => format!(
                "lwu {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Ld(i) => format!(
                "ld {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Sd(s) => format!(
                "sd {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),

            Self::Sll(r) => format!(
                "sll {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Srl(r) => format!(
                "srl {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sra(r) => format!(
                "sra {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Slli(i) => format!(
                "slli {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Srli(i) => format!(
                "srli {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Srai(i) => format!(
                "srai {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),

            Self::Addiw(i) => format!(
                "addiw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Slliw(i) => format!(
                "slliw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Srliw(i) => format!(
                "srliw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Sraiw(i) => format!(
                "sraiw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),

            Self::Addw(r) => format!(
                "addw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Subw(r) => format!(
                "subw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sllw(r) => format!(
                "sllw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Srlw(r) => format!(
                "srlw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sraw(r) => format!(
                "sraw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
        }
    }
}
