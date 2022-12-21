use std::fmt;
use std::convert::TryFrom;
use std::error::Error;

use crate::encoding::RegisterIndex;

pub use crate::types::Condition;
pub use crate::types::Label;
pub use crate::types::Shift;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

impl From<Register> for RegisterIndex {
    #[inline]
    fn from(reg: Register) -> RegisterIndex {
        RegisterIndex {
            index: reg as u32,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RegisterPair {
    R0R1 = 0,
    R2R3 = 2,
    R4R5 = 4,
    R6R7 = 6,
    R8R9 = 8,
    R10R11 = 10,
    R12R13 = 12,
}

impl From<RegisterPair> for RegisterIndex {
    #[inline]
    fn from(reg_pair: RegisterPair) -> RegisterIndex {
        RegisterIndex {
            index: reg_pair as u32,
        }
    }
}


#[derive(Clone, Copy)]
pub struct RegisterList(u16);

impl RegisterList {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    pub(crate) fn to_bits(&self) -> u32 { self.0 as u32 }
}

impl From<&[Register]> for RegisterList {
    #[inline]
    fn from(slice: &[Register]) -> RegisterList {
        let mut list = 0;
        for &reg in slice {
            list |= 1u16 << reg as u8;
        }
        RegisterList(list)
    }
}

macro_rules! impl_from_array {
    ($($n:expr)*) => {
        $(
            impl From<[Register; $n]> for RegisterList {
                #[inline]
                fn from(array: [Register; $n]) -> RegisterList {
                    RegisterList::from_slice(&array)
                }
            }
        )*
    }
}

impl_from_array! { 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 }

impl RegisterList {
    #[inline]
    pub fn from_slice(slice: &[Register]) -> RegisterList {
        let mut list = 0;
        for &reg in slice {
            list |= 1u16 << reg as u8;
        }
        RegisterList(list)
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum BankedRegister {
    R8_usr = 0,
    R9_usr,
    R10_usr,
    R11_usr,
    R12_usr,
    SP_usr,
    LR_usr,
    R8_fiq = 0b1000,
    R9_fiq,
    R10_fiq,
    R11_fiq,
    R12_fiq,
    SP_fiq,
    LR_fiq,
    LR_irq = 0b01_0000,
    SP_irq,
    LR_svc,
    SP_svc,
    LR_abt,
    SP_abt,
    LR_und,
    SP_und,
    LR_mon = 0xb01_1100,
    SP_mon,
    ELR_hyp,
    SP_hyp,
    SPSR_fiq = 0b10_1110,
    SPSR_irq = 0b11_0000,
    SPSR_svc = 0b11_0010,
    SPSR_abt = 0b11_0100,
    SPSR_und = 0b11_0110,
    SPSR_mon = 0b11_1100,
    SPSR_hyp = 0b11_1110,
}

impl BankedRegister {
    #[inline]
    pub fn accesses_spsr(&self) -> bool {
        *self as u32 >= 0b10_0000
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Const {
    imm8: u8,
    rotation: u8,
}

impl Const {
    #[inline]
    pub(crate) fn to_bits(&self) -> u32 {
        ((self.rotation as u32 & 0xf) << 8) | self.imm8 as u32
    }
}

impl From<Const> for u32 {
    #[inline]
    fn from(c: Const) -> u32 {
        (c.imm8 as u32).rotate_right(2 * c.rotation as u32)
    }
}

impl From<u8> for Const {
    #[inline]
    fn from(x: u8) -> Const {
        Const {
            imm8: x,
            rotation: 0,
        }
    }
}

impl TryFrom<u32> for Const {
    type Error = ConstTryFromError;

    #[inline]
    fn try_from(x: u32) -> Result<Const, ConstTryFromError> {
        (0..0x10).into_iter()
            .map(|r| (r, x.rotate_left(2 * r)))
            .find(|&(_, c)| c <= 0xff)
            .map(|(rotation, imm8)| {
                Const {
                    imm8: imm8 as u8,
                    rotation: rotation as u8,
                }
            })
            .ok_or(ConstTryFromError(()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ConstTryFromError(());

impl fmt::Display for ConstTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "could not convert number to Const".fmt(f)
    }
}

impl Error for ConstTryFromError {}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ror {
    Ror0,
    Ror8,
    Ror16,
    Ror24,
}


#[derive(Clone, Copy)]
pub enum Offset<T> {
    Sub(T),
    Add(T),
}

impl<T: Copy> Offset<T> {
    #[inline]
    pub fn is_add(&self) -> bool {
        match *self {
            Offset::Sub(_) => false,
            Offset::Add(_) => true,
        }
    }

    #[inline]
    pub fn abs_offset(&self) -> T {
        match *self {
            Offset::Add(offset) | Offset::Sub(offset) => offset,
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub struct Mem {
    pub reg: Register,
    pub writeback: bool,
}

impl From<Register> for Mem {
    #[inline]
    fn from(reg: Register) -> Mem {
        Mem { reg, writeback: false }
    }
}

impl Mem {
    #[inline]
    pub fn writeback(reg: Register) -> Mem {
        Mem { reg, writeback: true }
    }
}


#[derive(Clone, Copy)]
pub enum Index {
    Sub(Register, Shift<u8>),
    Add(Register, Shift<u8>),
}

impl From<Register> for Index {
    #[inline]
    fn from(index: Register) -> Index {
        Index::Add(index, Shift::Lsl(0))
    }
}

impl Index {
    #[inline]
    pub fn is_add(&self) -> bool {
        match *self {
            Index::Sub(..) => false,
            Index::Add(..) => true,
        }
    }

    #[inline]
    pub fn index_reg(&self) -> Register {
        match *self {
            Index::Sub(reg, _) | Index::Add(reg, _) => reg,
        }
    }

    #[inline]
    pub fn shift(&self) -> Shift<u8> {
        match *self {
            Index::Sub(_, shift) | Index::Add(_, shift) => shift,
        }
    }
}


#[derive(Clone, Copy)]
pub enum AddressMode {
    DecrementAfter,
    DecrementBefore,
    IncrementAfter,
    IncrementBefore,
}

impl AddressMode {
    #[inline]
    pub fn is_increment(&self) -> bool {
        match *self {
            AddressMode::DecrementAfter | AddressMode::DecrementBefore => false,
            AddressMode::IncrementAfter | AddressMode::IncrementBefore => true,
        }
    }

    #[inline]
    pub fn is_before(&self) -> bool {
        match *self {
            AddressMode::DecrementAfter | AddressMode::IncrementAfter => false,
            AddressMode::DecrementBefore | AddressMode::IncrementBefore => true,
        }
    }
}


pub enum Endianness {
    Little = 0,
    Big = 1,
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Const;

    #[test]
    fn test_const_into_u32() {
        assert_eq!(0u32, Const::from(0u8).into());
        assert_eq!(2u32, Const { imm8: 2, rotation: 0 }.into());
        assert_eq!(0x8000_0000u32, Const { imm8: 2, rotation: 1 }.into());
        assert_eq!(0x200_0000u32, Const { imm8: 2, rotation: 4 }.into());
    }

    #[test]
    fn test_const_try_from() {
        for imm8 in [1, 2, 3, 7, 0xff] {
            for rotation in 0..16 {
                let imm = (imm8 as u32).rotate_right(2 * rotation);
                assert_eq!(Ok(imm), Const::try_from(imm).map(Into::into));
            }
        }
    }
}
