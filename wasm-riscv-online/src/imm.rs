use crate::size::{Isize, Usize};

#[derive(Clone, Copy)]
pub struct Imm {
    data: u32,
    valid_bits: u8,
}

#[derive(Clone, Copy)]
pub struct Uimm {
    data: u32,
    valid_bits: u8,
}

impl Imm {
    pub(crate) fn new(data: u32, valid_bits: u8) -> Imm {
        assert!(valid_bits >= 1);
        Imm { data, valid_bits }
    }
}

impl Uimm {
    pub(crate) fn new(data: u32, valid_bits: u8) -> Uimm {
        assert!(valid_bits >= 1);
        Uimm { data, valid_bits }
    }

    pub fn low32(&self) -> u32 {
        self.data & MASK32[self.valid_bits as usize]
    }
}

static MASK32: [u32; 33] = [
    0x00000000, 0x00000001, 0x00000003, 0x00000007, 0x0000000F, 0x0000001F, 0x0000003F, 0x0000007F,
    0x000000FF, 0x000001FF, 0x000003FF, 0x000007FF, 0x00000FFF, 0x00001FFF, 0x00003FFF, 0x00007FFF,
    0x0000FFFF, 0x0001FFFF, 0x0003FFFF, 0x0007FFFF, 0x000FFFFF, 0x001FFFFF, 0x003FFFFF, 0x007FFFFF,
    0x00FFFFFF, 0x01FFFFFF, 0x03FFFFFF, 0x07FFFFFF, 0x0FFFFFFF, 0x1FFFFFFF, 0x3FFFFFFF, 0x7FFFFFFF,
    0xFFFFFFFF,
];

impl Imm {
    pub fn sext(self, xlen: Xlen) -> Isize {
        match xlen {
            Xlen::X32 => {
                let mut ans = self.data & MASK32[self.valid_bits as usize];
                if self.data & (1 << (self.valid_bits - 1)) != 0 {
                    ans |= !MASK32[self.valid_bits as usize];
                }
                Isize::I32(i32::from_ne_bytes(u32::to_ne_bytes(ans)))
            }
            Xlen::X64 => {
                let mut ans = (self.data & MASK32[self.valid_bits as usize]) as u64;
                if self.data & (1 << (self.valid_bits - 1)) != 0 {
                    ans |= !MASK32[self.valid_bits as usize] as u64 | 0xFFFFFFFF_00000000;
                }
                Isize::I64(i64::from_ne_bytes(u64::to_ne_bytes(ans)))
            }
            Xlen::X128 => panic!("Unsupported"),
        }
    }

    pub fn low_u32(&self) -> u32 {
        self.data & MASK32[self.valid_bits as usize]
    }

    pub fn low_i32(&self) -> i32 {
        i32::from_ne_bytes(u32::to_ne_bytes(self.low_u32()))
    }
}

impl Uimm {
    pub fn zext(self, xlen: Xlen) -> Usize {
        match xlen {
            Xlen::X32 => Usize::U32(self.data & MASK32[self.valid_bits as usize]),
            Xlen::X64 => Usize::U64((self.data & MASK32[self.valid_bits as usize]) as u64),
            Xlen::X128 => panic!("Unsupported"),
        }
    }
}

impl core::fmt::Debug for Imm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let num = self.data & MASK32[self.valid_bits as usize];
        f.write_fmt(format_args!(
            "{}",
            i32::from_ne_bytes(u32::to_ne_bytes(num))
        ))
    }
}

impl core::fmt::Debug for Uimm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let num = self.data & MASK32[self.valid_bits as usize];
        f.write_fmt(format_args!("{}", num))
    }
}

impl core::cmp::PartialEq<u32> for Uimm {
    fn eq(&self, rhs: &u32) -> bool {
        self.low32() == *rhs
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Xlen {
    X32,
    X64,
    X128,
}
