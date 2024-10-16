use super::{CsrIType, CsrRType};

#[derive(Debug, Clone, Copy)]
pub enum RVZicsr {
    Csrrw(CsrRType),
    Csrrs(CsrRType),
    Csrrc(CsrRType),
    Csrrwi(CsrIType),
    Csrrsi(CsrIType),
    Csrrci(CsrIType),
}

impl RVZicsr {
    pub fn to_string(&self) -> String {
        // TODO
        format!("RVZicsr Instruction")
    }
}
