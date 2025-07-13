use super::{to_register, IType, R4Type, RType, SType};  
  
// 添加浮点寄存器转换函数 
fn to_fp_register(ins: u8) -> String {  
    match ins {  
        0..=31 => format!("f{}", ins),  
        _ => "unknown".to_string(),  
    }  
}  
  
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
        match self {  
            Self::Flw(i) => format!(  
                "flw {}, {:?}({})",  
                to_fp_register(i.rd),  
                i.imm,  
                to_register(i.rs1)  
            ),  
            Self::Fsw(s) => format!(  
                "fsw {}, {:?}({})",  
                to_fp_register(s.rs2),  
                s.imm,  
                to_register(s.rs1)  
            ),  
              
            // Fused multiply-add - 四操作数格式  
            Self::Fmadds(r4) => format!(  
                "fmadd.s {}, {}, {}, {}",  
                to_fp_register(r4.rd),  
                to_fp_register(r4.rs1),  
                to_fp_register(r4.rs2),  
                to_fp_register(r4.rs3)  
            ),  
            Self::Fmsubs(r4) => format!(  
                "fmsub.s {}, {}, {}, {}",  
                to_fp_register(r4.rd),  
                to_fp_register(r4.rs1),  
                to_fp_register(r4.rs2),  
                to_fp_register(r4.rs3)  
            ),  
            Self::Fnmadds(r4) => format!(  
                "fnmadd.s {}, {}, {}, {}",  
                to_fp_register(r4.rd),  
                to_fp_register(r4.rs1),  
                to_fp_register(r4.rs2),  
                to_fp_register(r4.rs3)  
            ),  
            Self::Fnmsubs(r4) => format!(  
                "fnmsub.s {}, {}, {}, {}",  
                to_fp_register(r4.rd),  
                to_fp_register(r4.rs1),  
                to_fp_register(r4.rs2),  
                to_fp_register(r4.rs3)  
            ),  
              
            // 算术运算 - 三操作数格式  
            Self::Fadds(r) => format!(  
                "fadd.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fsubs(r) => format!(  
                "fsub.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fmuls(r) => format!(  
                "fmul.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fdivs(r) => format!(  
                "fdiv.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fsqrts(r) => format!(  
                "fsqrt.s {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1)  
            ),  
            Self::Fmins(r) => format!(  
                "fmin.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fmaxs(r) => format!(  
                "fmax.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
              
            // 符号注入  
            Self::Fsgnjs(r) => format!(  
                "fsgnj.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fsgnjns(r) => format!(  
                "fsgnjn.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fsgnjxs(r) => format!(  
                "fsgnjx.s {}, {}, {}",  
                to_fp_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
              
            // 比较 - 结果写入整数寄存器  
            Self::Feqs(r) => format!(  
                "feq.s {}, {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Flts(r) => format!(  
                "flt.s {}, {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
            Self::Fles(r) => format!(  
                "fle.s {}, {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1),  
                to_fp_register(r.rs2)  
            ),  
              
            // 转换指令  
            Self::Fcvtws(r) => format!(  
                "fcvt.w.s {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1)  
            ),  
            Self::Fcvtwus(r) => format!(  
                "fcvt.wu.s {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1)  
            ),  
            Self::Fcvtsw(r) => format!(  
                "fcvt.s.w {}, {}",  
                to_fp_register(r.rd),  
                to_register(r.rs1)  
            ),  
            Self::Fcvtswu(r) => format!(  
                "fcvt.s.wu {}, {}",  
                to_fp_register(r.rd),  
                to_register(r.rs1)  
            ),  
              
            // 搬移指令  
            Self::Fmvxw(r) => format!(  
                "fmv.x.w {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1)  
            ),  
            Self::Fmvwx(r) => format!(  
                "fmv.w.x {}, {}",  
                to_fp_register(r.rd),  
                to_register(r.rs1)  
            ),  
              
            // 分类指令  
            Self::Fclasss(r) => format!(  
                "fclass.s {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1)  
            ),  
              
            // RV64F 扩展  
            Self::Fcvtls(r) => format!(  
                "fcvt.l.s {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1)  
            ),  
            Self::Fcvtlus(r) => format!(  
                "fcvt.lu.s {}, {}",  
                to_register(r.rd),  
                to_fp_register(r.rs1)  
            ),  
            Self::Fcvtsl(r) => format!(  
                "fcvt.s.l {}, {}",  
                to_fp_register(r.rd),  
                to_register(r.rs1)  
            ),  
            Self::Fcvtslu(r) => format!(  
                "fcvt.s.lu {}, {}",  
                to_fp_register(r.rd),  
                to_register(r.rs1)  
            ),  
        }  
    }  
}