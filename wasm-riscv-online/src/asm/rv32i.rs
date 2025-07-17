use super::{to_register, BType, IType, JType, RType, SType, UType};

#[derive(Debug, Clone, Copy)]
pub enum RV32I {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),

    Beq(BType),
    Bne(BType),
    Blt(BType),
    Bge(BType),
    Bltu(BType),
    Bgeu(BType),

    Lb(IType),
    Lh(IType),
    Lw(IType),
    Lbu(IType),
    Lhu(IType),
    Sb(SType),
    Sh(SType),
    Sw(SType),

    Addi(IType),
    Slti(IType),
    Sltiu(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slli(IType),
    Srli(IType),
    Srai(IType),

    Add(RType),
    Sub(RType),
    Sll(RType),
    Slt(RType),
    Sltu(RType),
    Xor(RType),
    Srl(RType),
    Sra(RType),
    Or(RType),
    And(RType),

    // Multiplication and Division instructions
    Mul(RType),
    Mulh(RType),
    Mulhsu(RType),
    Mulhu(RType),
    Div(RType),
    Divu(RType),
    Rem(RType),
    Remu(RType),

    // System instructions with unit type
    Fence(()),
    FenceI(()),
    Ecall(()),
    Ebreak(()),
}

impl RV32I {
    pub fn to_string(&self) -> String {
        match self {
            Self::Lui(u) => format!("lui {}, {:?}", to_register(u.rd), u.imm),
            Self::Auipc(u) => format!("auipc {}, {:?}", to_register(u.rd), u.imm),
            Self::Jal(j) => format!("jal {}, {:?}", to_register(j.rd), j.imm),
            Self::Jalr(i) => format!(
                "jalr {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),

            Self::Beq(b) => format!(
                "beq {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bne(b) => format!(
                "bne {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Blt(b) => format!(
                "blt {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bge(b) => format!(
                "bge {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bltu(b) => format!(
                "bltu {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bgeu(b) => format!(
                "bgeu {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),

            Self::Lb(i) => format!(
                "lb {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lh(i) => format!(
                "lh {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lw(i) => format!(
                "lw {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lbu(i) => format!(
                "lbu {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lhu(i) => format!(
                "lhu {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),

            Self::Sb(s) => format!(
                "sb {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),
            Self::Sh(s) => format!(
                "sh {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),
            Self::Sw(s) => format!(
                "sw {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),

            Self::Add(r) => format!(
                "add {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sub(r) => format!(
                "sub {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sll(r) => format!(
                "sll {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Slt(r) => format!(
                "slt {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sltu(r) => format!(
                "sltu {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Xor(r) => format!(
                "xor {}, {}, {}",
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
            Self::Or(r) => format!(
                "or {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::And(r) => format!(
                "and {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),

            Self::Addi(i) => format!(
                "addi {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Slti(i) => format!(
                "slti {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Sltiu(i) => format!(
                "sltiu {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Xori(i) => format!(
                "xori {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Ori(i) => format!(
                "ori {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Andi(i) => format!(
                "andi {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Slli(i) => format!(  
                "slli {}, {}, {}",  
                to_register(i.rd),  
                to_register(i.rs1),  
                i.imm.low_u32() & 0x1f  
            ),  
            Self::Srli(i) => format!(  
                "srli {}, {}, {}",  
                to_register(i.rd),  
                to_register(i.rs1),  
                i.imm.low_u32() & 0x1f  
            ),  
            Self::Srai(i) => format!(  
                "srai {}, {}, {}",  
                to_register(i.rd),  
                to_register(i.rs1),  
                i.imm.low_u32() & 0x1f 
            ),

            Self::Mul(r) => format!(
                "mul {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Mulh(r) => format!(
                "mulh {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Mulhsu(r) => format!(
                "mulhsu {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Mulhu(r) => format!(
                "mulhu {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Div(r) => format!(
                "div {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Divu(r) => format!(
                "divu {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Rem(r) => format!(
                "rem {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Remu(r) => format!(
                "remu {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),

            Self::Fence(_) => format!("fence"),
            Self::FenceI(_) => format!("fence.i"),
            Self::Ecall(_) => format!("ecall"),
            Self::Ebreak(_) => format!("ebreak"),
        }
    }
}
