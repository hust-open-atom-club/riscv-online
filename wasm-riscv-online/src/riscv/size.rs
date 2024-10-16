#[derive(Clone, Copy, Ord, Eq, PartialEq)]
pub enum Usize {
    U32(u32),
    U64(u64),
}

impl Usize {
    pub fn low_u32(self) -> u32 {
        match self {
            Usize::U32(a) => a,
            Usize::U64(a) => (a & 0xFFFFFFFF) as u32,
        }
    }
}

impl core::fmt::Debug for Usize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Usize::U32(a) => f.write_fmt(format_args!("{}", a)),
            Usize::U64(a) => f.write_fmt(format_args!("{}", a)),
        }
    }
}

impl core::fmt::UpperHex for Usize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Usize::U32(a) => core::fmt::UpperHex::fmt(&a, f),
            Usize::U64(a) => core::fmt::UpperHex::fmt(&a, f),
        }
    }
}

impl core::fmt::LowerHex for Usize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Usize::U32(a) => core::fmt::LowerHex::fmt(&a, f),
            Usize::U64(a) => core::fmt::LowerHex::fmt(&a, f),
        }
    }
}

impl core::cmp::PartialOrd for Usize {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Usize::*;
        match (self, other) {
            (U32(a), U32(b)) => a.partial_cmp(b),
            (U64(a), U64(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl core::ops::Add<Usize> for Usize {
    type Output = Usize;
    fn add(self, rhs: Usize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Usize::U32(b)) => Usize::U32(a.wrapping_add(b)),
            (Usize::U64(a), Usize::U64(b)) => Usize::U64(a.wrapping_add(b)),
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::Sub<Usize> for Usize {
    type Output = Usize;
    fn sub(self, rhs: Usize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Usize::U32(b)) => Usize::U32(a.wrapping_sub(b)),
            (Usize::U64(a), Usize::U64(b)) => Usize::U64(a.wrapping_sub(b)),
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::Add<u32> for Usize {
    type Output = Usize;
    fn add(self, rhs: u32) -> Self::Output {
        match self {
            Usize::U32(a) => Usize::U32(a.wrapping_add(rhs)),
            Usize::U64(a) => Usize::U64(a.wrapping_add(rhs as u64)),
        }
    }
}

impl core::ops::Add<Isize> for Usize {
    type Output = Usize;
    fn add(self, rhs: Isize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Isize::I32(b)) => Usize::U32(if b > 0 {
                a.wrapping_add(b as u32)
            } else {
                a.wrapping_add(b.abs() as u32)
            }),
            (Usize::U64(a), Isize::I64(b)) => Usize::U64(if b > 0 {
                a.wrapping_add(b as u64)
            } else {
                a.wrapping_add(b.abs() as u64)
            }),
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::BitAnd<Usize> for Usize {
    type Output = Usize;
    fn bitand(self, rhs: Usize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Usize::U32(b)) => Usize::U32(a & b),
            (Usize::U64(a), Usize::U64(b)) => Usize::U64(a & b),
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::BitOr<Usize> for Usize {
    type Output = Usize;
    fn bitor(self, rhs: Usize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Usize::U32(b)) => Usize::U32(a | b),
            (Usize::U64(a), Usize::U64(b)) => Usize::U64(a | b),
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::BitXor<Usize> for Usize {
    type Output = Usize;
    fn bitxor(self, rhs: Usize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Usize::U32(b)) => Usize::U32(a ^ b),
            (Usize::U64(a), Usize::U64(b)) => Usize::U64(a ^ b),
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::BitAnd<Isize> for Usize {
    type Output = Usize;
    fn bitand(self, rhs: Isize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Isize::I32(b)) => {
                Usize::U32(a & u32::from_ne_bytes(i32::to_ne_bytes(b)))
            }
            (Usize::U64(a), Isize::I64(b)) => {
                Usize::U64(a & u64::from_ne_bytes(i64::to_ne_bytes(b)))
            }
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::BitOr<Isize> for Usize {
    type Output = Usize;
    fn bitor(self, rhs: Isize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Isize::I32(b)) => {
                Usize::U32(a | u32::from_ne_bytes(i32::to_ne_bytes(b)))
            }
            (Usize::U64(a), Isize::I64(b)) => {
                Usize::U64(a | u64::from_ne_bytes(i64::to_ne_bytes(b)))
            }
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::BitXor<Isize> for Usize {
    type Output = Usize;
    fn bitxor(self, rhs: Isize) -> Self::Output {
        match (self, rhs) {
            (Usize::U32(a), Isize::I32(b)) => {
                Usize::U32(a ^ u32::from_ne_bytes(i32::to_ne_bytes(b)))
            }
            (Usize::U64(a), Isize::I64(b)) => {
                Usize::U64(a ^ u64::from_ne_bytes(i64::to_ne_bytes(b)))
            }
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::Not for Usize {
    type Output = Usize;
    fn not(self) -> Self::Output {
        match self {
            Usize::U32(a) => Usize::U32(!a),
            Usize::U64(a) => Usize::U64(!a),
        }
    }
}

impl core::ops::Shl<u32> for Usize {
    type Output = Usize;
    fn shl(self, rhs: u32) -> Self::Output {
        match self {
            Usize::U32(a) => Usize::U32(a.checked_shl(rhs).unwrap_or(0)),
            Usize::U64(a) => Usize::U64(a.checked_shl(rhs).unwrap_or(0)),
        }
    }
}

impl core::ops::Shr<u32> for Usize {
    type Output = Usize;
    fn shr(self, rhs: u32) -> Self::Output {
        match self {
            Usize::U32(a) => Usize::U32(a.checked_shr(rhs).unwrap_or(0)),
            Usize::U64(a) => Usize::U64(a.checked_shr(rhs).unwrap_or(0)),
        }
    }
}

impl core::ops::AddAssign<Usize> for Usize {
    fn add_assign(&mut self, rhs: Usize) {
        match (self, rhs) {
            (Usize::U32(a), Usize::U32(b)) => *a = a.wrapping_add(b),
            (Usize::U64(a), Usize::U64(b)) => *a = a.wrapping_add(b),
            _ => panic!("Not the same type"),
        }
    }
}

impl core::ops::AddAssign<u32> for Usize {
    fn add_assign(&mut self, rhs: u32) {
        match self {
            Usize::U32(a) => *a = a.wrapping_add(rhs),
            Usize::U64(a) => *a = a.wrapping_add(rhs as u64),
        }
    }
}

#[derive(Clone, Copy, Ord, Eq, PartialEq)]
pub enum Isize {
    I32(i32),
    I64(i64),
}

impl core::fmt::Debug for Isize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Isize::I32(a) => f.write_fmt(format_args!("{}", a)),
            Isize::I64(a) => f.write_fmt(format_args!("{}", a)),
        }
    }
}

impl Isize {
    pub fn cast_to_usize(self) -> Usize {
        match self {
            Isize::I32(a) => Usize::U32(u32::from_ne_bytes(i32::to_ne_bytes(a))),
            Isize::I64(a) => Usize::U64(u64::from_ne_bytes(i64::to_ne_bytes(a))),
        }
    }
}

impl core::cmp::PartialOrd for Isize {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Isize::*;
        match (self, other) {
            (I32(a), I32(b)) => a.partial_cmp(b),
            (I64(a), I64(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl core::ops::Shr<u32> for Isize {
    type Output = Isize;
    fn shr(self, rhs: u32) -> Self::Output {
        match self {
            Isize::I32(a) => Isize::I32(a.checked_shr(rhs).unwrap_or(-1)), // 0xFFFFFFFF
            Isize::I64(a) => Isize::I64(a.checked_shr(rhs).unwrap_or(-1)), // 0xFFFF....FFFF
        }
    }
}
