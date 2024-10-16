use super::{
    to_register, CAType, CBType, CIType, CIWType, CJType, CLType, CRType, CSSType, CSType,
};

#[derive(Debug, Clone, Copy)]
pub enum RVC {
    Caddi4spn(CIWType),
    Cfld(CLType),
    Clq(CLType),
    Clw(CLType),
    Cflw(CLType),
    Cld(CLType),
    Cfsd(CSType),
    Csq(CSType),
    Csw(CSType),
    Cfsw(CSType),
    Csd(CSType),

    Cnop(CIType),
    Caddi(CIType),
    Cjal(CJType),
    Caddiw(CIType),
    Cli(CIType),
    Caddi16sp(CIType),
    Clui(CIType),
    Csrli(CIType),
    Csrli64(CIType),
    Csrai(CIType),
    Csrai64(CIType),
    Candi(CIType),
    Csub(CAType),
    Cxor(CAType),
    Cor(CAType),
    Cand(CAType),
    Csubw(CAType),
    Caddw(CAType),
    Cj(CJType),
    Cbeqz(CBType),
    Cbnez(CBType),

    Cslli(CIType),
    Cslli64(CIType),
    Cfldsp(CIType),
    Clqsp(CIType),
    Clwsp(CIType),
    Cflwsp(CIType),
    Cldsp(CIType),
    Cjr(CRType),
    Cmv(CRType),
    Cebreak(CRType),
    Cjalr(CRType),
    Cadd(CRType),
    Cfsdsp(CSSType),
    Csqsp(CSSType),
    Cswsp(CSSType),
    Cfswsp(CSSType),
    Csdsp(CSSType),
}

impl RVC {
    pub fn to_string(&self) -> String {
        match self {
            Self::Caddi4spn(ciw) => format!(
                "c.addi {}, {}, {:?}",
                to_register(ciw.rd),
                to_register(2),
                ciw.uimm
            ),
            Self::Cfld(cl) => format!(
                "c.fld {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Clq(cl) => format!(
                "c.lq {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Clw(cl) => format!(
                "c.lw {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Cflw(cl) => format!(
                "c.flw {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Cld(cl) => format!(
                "c.ld {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Cfsd(cs) => format!(
                "c.fsd {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Csq(cs) => format!(
                "c.sq {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Csw(cs) => format!(
                "c.sw {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Cfsw(cs) => format!(
                "c.fsw {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Csd(cs) => format!(
                "c.sd {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),

            Self::Cnop(ci) => format!("c.nop"),
            Self::Caddi(ci) => format!(
                "c.addi {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cjal(cj) => format!("c.jal {},{:?}", to_register(0), cj.target),
            Self::Caddiw(ci) => format!(
                "c.addiw {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cli(ci) => format!(
                "c.li {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(0),
                ci.imm
            ),
            Self::Caddi16sp(ci) => format!(
                "c.addi16sp {}, {}, {:?}",
                to_register(2),
                to_register(2),
                ci.imm
            ),
            Self::Clui(ci) => format!("c.lui {}, {:?}", to_register(ci.rdrs1), ci.imm),
            Self::Csrli(ci) => format!(
                "c.srli {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Csrli64(ci) => format!(
                "c.srli64 {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Csrai(ci) => format!(
                "c.srai {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm,
            ),
            Self::Csrai64(ci) => format!(
                "c.srai64 {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm,
            ),
            Self::Candi(ci) => format!(
                "c.andi {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm,
            ),
            Self::Csub(ca) => format!(
                "c.sub {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cxor(ca) => format!(
                "c.xor {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cor(ca) => format!(
                "c.or {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cand(ca) => format!(
                "c.and {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Csubw(ca) => format!(
                "c.subw {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Caddw(ca) => format!(
                "c.addw {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cj(cj) => format!("c.j {}, {:?}", to_register(0), cj.target),
            Self::Cbeqz(cb) => format!(
                "c.beqz {}, {}, {:?}",
                to_register(cb.rs1),
                to_register(0),
                cb.off
            ),
            Self::Cbnez(cb) => format!(
                "c.bnez {}, {}, {:?}",
                to_register(cb.rs1),
                to_register(0),
                cb.off
            ),

            Self::Cslli(ci) => format!(
                "c.slli {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cslli64(ci) => format!(
                "c.slli64 {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cfldsp(ci) => format!(
                "c.fldsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Clqsp(ci) => format!(
                "c.lqsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Clwsp(ci) => format!(
                "c.lwsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Cflwsp(ci) => format!(
                "c.flwsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Cldsp(ci) => format!(
                "c.ldsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Cjr(cr) => format!("c.jr {}, 0({})", to_register(0), to_register(cr.rdrs1)),
            Self::Cmv(cr) => format!(
                "c.mv {}, {}, {}",
                to_register(cr.rdrs1),
                to_register(0),
                to_register(cr.rs2)
            ),
            Self::Cebreak(cr) => format!("c.ebreak"),
            Self::Cjalr(cr) => format!("c.jalr {}, 0({})", to_register(1), to_register(cr.rdrs1)),
            Self::Cadd(cr) => format!(
                "c.add {}, {}, {}",
                to_register(cr.rdrs1),
                to_register(cr.rdrs1),
                to_register(cr.rs2)
            ),
            Self::Cfsdsp(css) => format!(
                "c.fsdsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Csqsp(css) => format!(
                "c.sqsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Cswsp(css) => format!(
                "c.swsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Cfswsp(css) => format!(
                "c.fswsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Csdsp(css) => format!(
                "c.sdsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
        }
    }
}
