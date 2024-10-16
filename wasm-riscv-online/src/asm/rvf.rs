use super::{IType, R4Type, RType, SType};

#[derive(Debug, Clone, Copy)]
pub enum RVF {
    // RV32F
    Flw(IType),
    Fsw(SType),
    Fmadds(R4Type),
    Fmsubs(R4Type),
    Fnmadds(R4Type),
    Fnmsubs(R4Type),
    Fadds(RType),
    Fsubs(RType),
    Fmuls(RType),
    Fdivs(RType),
    Fsqrts(RType),
    Fsgnjs(RType),
    Fsgnjns(RType),
    Fsgnjxs(RType),
    Fmins(RType),
    Fmaxs(RType),
    Fcvtws(RType),
    Fcvtwus(RType),
    Fmvxw(RType),
    Feqs(RType),
    Flts(RType),
    Fles(RType),
    Fclasss(RType),
    Fcvtsw(RType),
    Fcvtswu(RType),
    Fmvwx(RType),
    // RV64F
    Fcvtls(RType),
    Fcvtlus(RType),
    Fcvtsl(RType),
    Fcvtslu(RType),
}

impl RVF {
    pub fn to_string(&self) -> String {
        // TODO
        format!("RVF Instruction")
    }
}
