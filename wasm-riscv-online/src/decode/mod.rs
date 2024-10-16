mod process16;
mod process32;
pub use process16::resolve_u16;
pub use process32::resolve_u32;

fn c_reg(regid: u8) -> u8 {
    regid + 8
}
