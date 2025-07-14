use super::{to_register, CsrIType, CsrRType}; 

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
        match self {  
            Self::Csrrw(csr) => format!(  
                "csrrw {}, {:#x}, {}",  
                to_register(csr.rd),  
                csr.csr,  
                to_register(csr.rs1)  
            ),  
            Self::Csrrs(csr) => format!(  
                "csrrs {}, {:#x}, {}",  
                to_register(csr.rd),  
                csr.csr,  
                to_register(csr.rs1)  
            ),  
            Self::Csrrc(csr) => format!(  
                "csrrc {}, {:#x}, {}",  
                to_register(csr.rd),  
                csr.csr,  
                to_register(csr.rs1)  
            ),  
            Self::Csrrwi(csr) => format!(  
                "csrrwi {}, {:#x}, {:?}",  
                to_register(csr.rd),  
                csr.csr,  
                csr.uimm  
            ),  
            Self::Csrrsi(csr) => format!(  
                "csrrsi {}, {:#x}, {:?}",  
                to_register(csr.rd),  
                csr.csr,  
                csr.uimm  
            ),  
            Self::Csrrci(csr) => format!(  
                "csrrci {}, {:#x}, {:?}",  
                to_register(csr.rd),  
                csr.csr,  
                csr.uimm  
            ),  
        }  
    }
}
