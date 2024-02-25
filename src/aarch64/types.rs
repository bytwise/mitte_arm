use std::fmt;
use std::convert::TryFrom;
use std::error::Error;
use std::marker::PhantomData;

pub use crate::types::Condition;
pub use crate::types::Label;
pub use crate::types::Shift;

use crate::encoding::RegisterIndex;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ZR;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SP;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NoReg31 {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Register32Common<W31> {
    W0, W1, W2, W3, W4, W5, W6, W7,
    W8, W9, W10, W11, W12, W13, W14, W15,
    W16, W17, W18, W19, W20, W21, W22, W23,
    W24, W25, W26, W27, W28, W29, W30, W31(W31),
}

pub type Register32 = Register32Common<ZR>;
pub type Register32OrSp = Register32Common<SP>;
pub type Register32NoReg31 = Register32Common<NoReg31>;
pub const WZR: Register32 = Register32Common::W31(ZR);
pub const WSP: Register32OrSp = Register32Common::W31(SP);

impl<W31> Register32Common<W31> {
    #[inline]
    pub fn index(&self) -> usize {
        use Register32Common::*;
        match *self {
            W0 => 0,
            W1 => 1,
            W2 => 2,
            W3 => 3,
            W4 => 4,
            W5 => 5,
            W6 => 6,
            W7 => 7,
            W8 => 8,
            W9 => 9,
            W10 => 10,
            W11 => 11,
            W12 => 12,
            W13 => 13,
            W14 => 14,
            W15 => 15,
            W16 => 16,
            W17 => 17,
            W18 => 18,
            W19 => 19,
            W20 => 20,
            W21 => 21,
            W22 => 22,
            W23 => 23,
            W24 => 24,
            W25 => 25,
            W26 => 26,
            W27 => 27,
            W28 => 28,
            W29 => 29,
            W30 => 30,
            W31(_) => 31,
        }
    }

    #[inline]
    pub fn to_register64(self) -> Register64Common<W31> {
        use Register32Common::*;
        use Register64Common::*;
        match self {
            W0 => X0,
            W1 => X1,
            W2 => X2,
            W3 => X3,
            W4 => X4,
            W5 => X5,
            W6 => X6,
            W7 => X7,
            W8 => X8,
            W9 => X9,
            W10 => X10,
            W11 => X11,
            W12 => X12,
            W13 => X13,
            W14 => X14,
            W15 => X15,
            W16 => X16,
            W17 => X17,
            W18 => X18,
            W19 => X19,
            W20 => X20,
            W21 => X21,
            W22 => X22,
            W23 => X23,
            W24 => X24,
            W25 => X25,
            W26 => X26,
            W27 => X27,
            W28 => X28,
            W29 => X29,
            W30 => X30,
            W31(w31) => X31(w31),
        }
    }
}

impl Register32Common<NoReg31> {
    #[inline]
    pub fn from_index(index: usize) -> Option<Register32Common<NoReg31>> {
        use Register32Common::*;
        match index {
            0 => Some(W0),
            1 => Some(W1),
            2 => Some(W2),
            3 => Some(W3),
            4 => Some(W4),
            5 => Some(W5),
            6 => Some(W6),
            7 => Some(W7),
            8 => Some(W8),
            9 => Some(W9),
            10 => Some(W10),
            11 => Some(W11),
            12 => Some(W12),
            13 => Some(W13),
            14 => Some(W14),
            15 => Some(W15),
            16 => Some(W16),
            17 => Some(W17),
            18 => Some(W18),
            19 => Some(W19),
            20 => Some(W20),
            21 => Some(W21),
            22 => Some(W22),
            23 => Some(W23),
            24 => Some(W24),
            25 => Some(W25),
            26 => Some(W26),
            27 => Some(W27),
            28 => Some(W28),
            29 => Some(W29),
            30 => Some(W30),
            _ => None
        }
    }
}

impl From<Register32NoReg31> for Register32 {
    #[inline]
    fn from(reg: Register32NoReg31) -> Register32 {
        use Register32Common::*;
        match reg {
            W0 => W0,
            W1 => W1,
            W2 => W2,
            W3 => W3,
            W4 => W4,
            W5 => W5,
            W6 => W6,
            W7 => W7,
            W8 => W8,
            W9 => W9,
            W10 => W10,
            W11 => W11,
            W12 => W12,
            W13 => W13,
            W14 => W14,
            W15 => W15,
            W16 => W16,
            W17 => W17,
            W18 => W18,
            W19 => W19,
            W20 => W20,
            W21 => W21,
            W22 => W22,
            W23 => W23,
            W24 => W24,
            W25 => W25,
            W26 => W26,
            W27 => W27,
            W28 => W28,
            W29 => W29,
            W30 => W30,
            W31(w31) => match w31 {},
        }
    }
}

impl From<Register32NoReg31> for Register32OrSp {
    #[inline]
    fn from(reg: Register32NoReg31) -> Register32OrSp {
        use Register32Common::*;
        match reg {
            W0 => W0,
            W1 => W1,
            W2 => W2,
            W3 => W3,
            W4 => W4,
            W5 => W5,
            W6 => W6,
            W7 => W7,
            W8 => W8,
            W9 => W9,
            W10 => W10,
            W11 => W11,
            W12 => W12,
            W13 => W13,
            W14 => W14,
            W15 => W15,
            W16 => W16,
            W17 => W17,
            W18 => W18,
            W19 => W19,
            W20 => W20,
            W21 => W21,
            W22 => W22,
            W23 => W23,
            W24 => W24,
            W25 => W25,
            W26 => W26,
            W27 => W27,
            W28 => W28,
            W29 => W29,
            W30 => W30,
            W31(w31) => match w31 {},
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Register64Common<X31> {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30, X31(X31),
}

pub type Register64 = Register64Common<ZR>;
pub type Register64OrSp = Register64Common<SP>;
pub type Register64NoReg31 = Register64Common<NoReg31>;
pub const XZR: Register64 = Register64Common::X31(ZR);
pub const XSP: Register64OrSp = Register64Common::X31(SP);

impl<X31> Register64Common<X31> {
    #[inline]
    pub fn index(&self) -> usize {
        use Register64Common::*;
        match *self {
            X0 => 0,
            X1 => 1,
            X2 => 2,
            X3 => 3,
            X4 => 4,
            X5 => 5,
            X6 => 6,
            X7 => 7,
            X8 => 8,
            X9 => 9,
            X10 => 10,
            X11 => 11,
            X12 => 12,
            X13 => 13,
            X14 => 14,
            X15 => 15,
            X16 => 16,
            X17 => 17,
            X18 => 18,
            X19 => 19,
            X20 => 20,
            X21 => 21,
            X22 => 22,
            X23 => 23,
            X24 => 24,
            X25 => 25,
            X26 => 26,
            X27 => 27,
            X28 => 28,
            X29 => 29,
            X30 => 30,
            X31(_) => 31,
        }
    }

    #[inline]
    pub fn to_register32(self) -> Register32Common<X31> {
        use Register32Common::*;
        use Register64Common::*;
        match self {
            X0 => W0,
            X1 => W1,
            X2 => W2,
            X3 => W3,
            X4 => W4,
            X5 => W5,
            X6 => W6,
            X7 => W7,
            X8 => W8,
            X9 => W9,
            X10 => W10,
            X11 => W11,
            X12 => W12,
            X13 => W13,
            X14 => W14,
            X15 => W15,
            X16 => W16,
            X17 => W17,
            X18 => W18,
            X19 => W19,
            X20 => W20,
            X21 => W21,
            X22 => W22,
            X23 => W23,
            X24 => W24,
            X25 => W25,
            X26 => W26,
            X27 => W27,
            X28 => W28,
            X29 => W29,
            X30 => W30,
            X31(x31) => W31(x31),
        }
    }
}

impl Register64Common<NoReg31> {
    #[inline]
    pub fn from_index(index: usize) -> Option<Register64Common<NoReg31>> {
        use Register64Common::*;
        match index {
            0 => Some(X0),
            1 => Some(X1),
            2 => Some(X2),
            3 => Some(X3),
            4 => Some(X4),
            5 => Some(X5),
            6 => Some(X6),
            7 => Some(X7),
            8 => Some(X8),
            9 => Some(X9),
            10 => Some(X10),
            11 => Some(X11),
            12 => Some(X12),
            13 => Some(X13),
            14 => Some(X14),
            15 => Some(X15),
            16 => Some(X16),
            17 => Some(X17),
            18 => Some(X18),
            19 => Some(X19),
            20 => Some(X20),
            21 => Some(X21),
            22 => Some(X22),
            23 => Some(X23),
            24 => Some(X24),
            25 => Some(X25),
            26 => Some(X26),
            27 => Some(X27),
            28 => Some(X28),
            29 => Some(X29),
            30 => Some(X30),
            _ => None
        }
    }
}

impl From<Register64NoReg31> for Register64 {
    #[inline]
    fn from(reg: Register64NoReg31) -> Register64 {
        use Register64Common::*;
        match reg {
            X0 => X0,
            X1 => X1,
            X2 => X2,
            X3 => X3,
            X4 => X4,
            X5 => X5,
            X6 => X6,
            X7 => X7,
            X8 => X8,
            X9 => X9,
            X10 => X10,
            X11 => X11,
            X12 => X12,
            X13 => X13,
            X14 => X14,
            X15 => X15,
            X16 => X16,
            X17 => X17,
            X18 => X18,
            X19 => X19,
            X20 => X20,
            X21 => X21,
            X22 => X22,
            X23 => X23,
            X24 => X24,
            X25 => X25,
            X26 => X26,
            X27 => X27,
            X28 => X28,
            X29 => X29,
            X30 => X30,
            X31(w31) => match w31 {},
        }
    }
}

impl From<Register64NoReg31> for Register64OrSp {
    #[inline]
    fn from(reg: Register64NoReg31) -> Register64OrSp {
        use Register64Common::*;
        match reg {
            X0 => X0,
            X1 => X1,
            X2 => X2,
            X3 => X3,
            X4 => X4,
            X5 => X5,
            X6 => X6,
            X7 => X7,
            X8 => X8,
            X9 => X9,
            X10 => X10,
            X11 => X11,
            X12 => X12,
            X13 => X13,
            X14 => X14,
            X15 => X15,
            X16 => X16,
            X17 => X17,
            X18 => X18,
            X19 => X19,
            X20 => X20,
            X21 => X21,
            X22 => X22,
            X23 => X23,
            X24 => X24,
            X25 => X25,
            X26 => X26,
            X27 => X27,
            X28 => X28,
            X29 => X29,
            X30 => X30,
            X31(w31) => match w31 {},
        }
    }
}


impl<W31> From<Register32Common<W31>> for RegisterIndex {
    #[inline]
    fn from(reg: Register32Common<W31>) -> RegisterIndex {
        RegisterIndex { index: reg.index() as u32 }
    }
}

impl<X31> From<Register64Common<X31>> for RegisterIndex {
    #[inline]
    fn from(reg: Register64Common<X31>) -> RegisterIndex {
        RegisterIndex { index: reg.index() as u32 }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShiftedImm12 {
    pub(crate) sh: u8,
    pub(crate) imm12: u16,
}

impl ShiftedImm12 {
    #[inline]
    pub fn lsl_12(imm: u8) -> ShiftedImm12 {
        ShiftedImm12 {
            sh: 1,
            imm12: imm.into(),
        }
    }
}

impl From<u8> for ShiftedImm12 {
    #[inline]
    fn from(imm: u8) -> ShiftedImm12 {
        ShiftedImm12 {
            sh: 0,
            imm12: imm.into(),
        }
    }
}

impl From<ShiftedImm12> for u32 {
    #[inline]
    fn from(imm: ShiftedImm12) -> u32 {
        if imm.sh != 0 {
            u32::from(imm.imm12) << 12
        } else {
            u32::from(imm.imm12)
        }
    }
}

impl TryFrom<u32> for ShiftedImm12 {
    type Error = ShiftedImm12TryFromError;

    #[inline]
    fn try_from(imm: u32) -> Result<ShiftedImm12, ShiftedImm12TryFromError> {
        if imm < (1 << 12) {
            Ok(ShiftedImm12 {
                sh: 0,
                imm12: imm as u16,
            })
        } else if imm & 0xfff == 0 && imm < (1 << 24) {
            Ok(ShiftedImm12 {
                sh: 1,
                imm12: (imm >> 12) as u16,
            })
        } else {
            Err(ShiftedImm12TryFromError(()))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShiftedImm12TryFromError(());

impl fmt::Display for ShiftedImm12TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "could not convert number to ShiftedImm12".fmt(f)
    }
}

impl Error for ShiftedImm12TryFromError {}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MovImm32 {
    pub(crate) hw: u8,
    pub(crate) imm16: u16,
}

impl MovImm32 {
    #[inline]
    pub fn lsl_16(imm: u16) -> MovImm32 {
        MovImm32 {
            hw: 1,
            imm16: imm,
        }
    }
}

impl From<MovImm32> for u32 {
    #[inline]
    fn from(movimm: MovImm32) -> u32 {
        (movimm.imm16 as u32) << (16 * movimm.hw)
    }
}

impl From<u16> for MovImm32 {
    #[inline]
    fn from(imm: u16) -> MovImm32 {
        MovImm32 {
            hw: 0,
            imm16: imm,
        }
    }
}

impl TryFrom<u32> for MovImm32 {
    type Error = MovImm32TryFromError;

    #[inline]
    fn try_from(imm: u32) -> Result<MovImm32, MovImm32TryFromError> {
        if imm < (1 << 16) {
            Ok(MovImm32 {
                hw: 0,
                imm16: imm as u16,
            })
        } else if imm & 0xffff == 0 {
            Ok(MovImm32 {
                hw: 1,
                imm16: (imm >> 16) as u16,
            })
        } else {
            Err(MovImm32TryFromError(()))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MovImm32TryFromError(());

impl fmt::Display for MovImm32TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "could not convert number to MovImm32".fmt(f)
    }
}

impl Error for MovImm32TryFromError {}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MovImm64 {
    pub(crate) hw: u8,
    pub(crate) imm16: u16,
}

impl MovImm64 {
    #[inline]
    pub fn lsl_16(imm: u16) -> MovImm64 {
        MovImm64 {
            hw: 1,
            imm16: imm,
        }
    }

    #[inline]
    pub fn lsl_32(imm: u16) -> MovImm64 {
        MovImm64 {
            hw: 2,
            imm16: imm,
        }
    }

    #[inline]
    pub fn lsl_48(imm: u16) -> MovImm64 {
        MovImm64 {
            hw: 3,
            imm16: imm,
        }
    }
}

impl From<MovImm64> for u64 {
    #[inline]
    fn from(movimm: MovImm64) -> u64 {
        (movimm.imm16 as u64) << (16 * movimm.hw)
    }
}

impl From<u16> for MovImm64 {
    #[inline]
    fn from(imm: u16) -> MovImm64 {
        MovImm64 {
            hw: 0,
            imm16: imm,
        }
    }
}

impl TryFrom<u64> for MovImm64 {
    type Error = MovImm64TryFromError;

    #[inline]
    fn try_from(imm: u64) -> Result<MovImm64, MovImm64TryFromError> {
        let mut imm = imm;
        for hw in 0..4 {
            if imm < (1 << 16) {
                return Ok(MovImm64 {
                    hw,
                    imm16: imm as u16,
                });
            } else if imm & 0xffff == 0 {
                imm = imm >> 16;
            } else {
                return Err(MovImm64TryFromError(()));
            }
        }
        Err(MovImm64TryFromError(()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MovImm64TryFromError(());

impl fmt::Display for MovImm64TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "could not convert number to MovImm64".fmt(f)
    }
}

impl Error for MovImm64TryFromError {}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bitmask32 {
    pub(crate) imms: u8,
    pub(crate) immr: u8,
}

impl From<Bitmask32> for u32 {
    fn from(bitmask: Bitmask32) -> u32 {
        let len = 5 - (bitmask.imms << 2).leading_ones();
        let ones = 1 + (bitmask.imms & ((1 << len) - 1));
        let mask = (1 << ones) - 1;
        let mask: u32 = match len {
            1 => mask * 0x5555_5555,
            2 => mask * 0x1111_1111,
            3 => mask * 0x0101_0101,
            4 => mask * 0x0001_0001,
            5 => mask,
            _ => unreachable!(),
        };
        mask.rotate_right(bitmask.immr as u32)
    }
}

impl TryFrom<u32> for Bitmask32 {
    type Error = Bitmask32TryFromError;

    #[inline]
    fn try_from(mask: u32) -> Result<Bitmask32, Bitmask32TryFromError> {
        if mask == 0 || mask == !0 {
            return Err(Bitmask32TryFromError(()));
        }

        let rotation = (mask & (mask + 1)).trailing_zeros();
        let normalized = mask.rotate_right(rotation);

        let z = normalized.leading_zeros() as u8;
        let o = normalized.trailing_ones() as u8;
        let size = z + o;
        if mask.rotate_right(size as u32) == mask {
            Ok(Bitmask32 {
                imms: ((size << 1).wrapping_neg() | (o - 1)) & 0x3f,
                immr: (rotation as u8).wrapping_neg() & (size - 1),
            })
        } else {
            Err(Bitmask32TryFromError(()))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bitmask32TryFromError(());

impl fmt::Display for Bitmask32TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "could not convert number to Bitmask32".fmt(f)
    }
}

impl Error for Bitmask32TryFromError {}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bitmask64 {
    pub(crate) n: u8,
    pub(crate) imms: u8,
    pub(crate) immr: u8,
}

impl From<Bitmask64> for u64 {
    fn from(bitmask: Bitmask64) -> u64 {
        let len = 6 - ((!bitmask.n << 7) | (bitmask.imms << 1)).leading_ones();
        let ones = 1 + (bitmask.imms & ((1 << len) - 1));
        let mask = (1 << ones) - 1;
        let mask: u64 = match len {
            1 => mask * 0x5555_5555_5555_5555,
            2 => mask * 0x1111_1111_1111_1111,
            3 => mask * 0x0101_0101_0101_0101,
            4 => mask * 0x0001_0001_0001_0001,
            5 => mask * 0x0000_0001_0000_0001,
            6 => mask,
            _ => unreachable!(),
        };
        mask.rotate_right(bitmask.immr as u32)
    }
}

impl TryFrom<u64> for Bitmask64 {
    type Error = Bitmask64TryFromError;

    #[inline]
    fn try_from(mask: u64) -> Result<Bitmask64, Bitmask64TryFromError> {
        if mask == 0 || mask == !0 {
            return Err(Bitmask64TryFromError(()));
        }

        let rotation = (mask & (mask + 1)).trailing_zeros();
        let normalized = mask.rotate_right(rotation);

        let z = normalized.leading_zeros() as u8;
        let o = normalized.trailing_ones() as u8;
        let size = z + o;
        if mask.rotate_right(size as u32) == mask {
            Ok(Bitmask64 {
                n: size >> 6,
                imms: ((size << 1).wrapping_neg() | (o - 1)) & 0x3f,
                immr: (rotation as u8).wrapping_neg() & (size - 1),
            })
        } else {
            Err(Bitmask64TryFromError(()))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bitmask64TryFromError(());

impl fmt::Display for Bitmask64TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "could not convert number to Bitmask64".fmt(f)
    }
}

impl Error for Bitmask64TryFromError {}


#[derive(Clone, Copy)]
pub struct Index<T> {
    pub(crate) index: i16,
    _marker: PhantomData<*const T>,
}

impl<T> Index<T> {
    #[inline]
    pub fn new(index: i16) -> Index<T> {
        Index {
            index,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn from_byte_offset(offset: i16) -> Index<T> {
        Index {
            index: offset / std::mem::size_of::<T>() as i16,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn index(&self) -> i16 {
        self.index
    }

    #[inline]
    pub fn byte_offset(&self) -> i16 {
        self.index * std::mem::size_of::<T>() as i16
    }
}


#[derive(Clone, Copy)]
pub enum Extend {
    Uxtb(u8),
    Uxth(u8),
    Uxtw(u8),
    Uxtx(u8),
    Sxtb(u8),
    Sxth(u8),
    Sxtw(u8),
    Sxtx(u8),
}

impl Extend {
    pub(crate) fn ty(&self) -> u8 {
        match *self {
            Extend::Uxtb(_) => 0,
            Extend::Uxth(_) => 1,
            Extend::Uxtw(_) => 2,
            Extend::Uxtx(_) => 3,
            Extend::Sxtb(_) => 4,
            Extend::Sxth(_) => 5,
            Extend::Sxtw(_) => 6,
            Extend::Sxtx(_) => 7,
        }
    }

    pub(crate) fn amount(&self) -> u8 {
        match *self {
            Extend::Uxtb(amount) |
            Extend::Uxth(amount) |
            Extend::Uxtw(amount) |
            Extend::Uxtx(amount) |
            Extend::Sxtb(amount) |
            Extend::Sxth(amount) |
            Extend::Sxtw(amount) |
            Extend::Sxtx(amount)
                => amount,
        }
    }
}


pub enum BranchTargets {
    None = 0b000,
    C = 0b010,
    J = 0b100,
    Jc = 0b110,
}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use super::{ShiftedImm12, MovImm32, MovImm64, Bitmask32, Bitmask64};

    #[test]
    fn test_shifted_imm12_try_from_without_shift() {
        for i in 0..12 {
            let imm = 0xfffu32 >> i;
            assert_eq!(Ok(ShiftedImm12 { sh: 0, imm12: imm as u16 }), imm.try_into());
        }
    }

    #[test]
    fn test_shifted_imm12_try_from_with_shift() {
        for i in 0..12 {
            let imm = (0xff_f000u32 >> i) & 0xff_f000;
            assert_eq!(Ok(ShiftedImm12 { sh: 1, imm12: (imm >> 12) as u16 }), imm.try_into());
        }
    }

    #[test]
    fn test_shifted_imm12_try_from_invalid() {
        assert!(ShiftedImm12::try_from(0xffff_ffffu32).is_err());
        assert!(ShiftedImm12::try_from(0x0100_0000u32).is_err());
        assert!(ShiftedImm12::try_from(0x0000_1001u32).is_err());
        assert!(ShiftedImm12::try_from(0x0000_1800u32).is_err());
    }

    #[test]
    fn test_movimm32_try_from() {
        for hw in 0..2 {
            for imm16 in [1, 0x8000, 0xffff] {
                let movimm32 = MovImm32 { hw, imm16 };
                let imm: u32 = movimm32.into();
                assert_eq!(Ok(movimm32), imm.try_into());
            }
        }
    }

    #[test]
    fn test_movimm32_try_from_invalid() {
        assert!(MovImm32::try_from(0xffff_ffffu32).is_err());
        assert!(MovImm32::try_from(0x0001_0001u32).is_err());
        assert!(MovImm32::try_from(0x0001_8000u32).is_err());
    }

    #[test]
    fn test_movimm64_try_from() {
        for hw in 0..4 {
            for imm16 in [1, 0x8000, 0xffff] {
                let movimm64 = MovImm64 { hw, imm16 };
                let imm: u64 = movimm64.into();
                assert_eq!(Ok(movimm64), imm.try_into());
            }
        }
    }

    #[test]
    fn test_movimm64_try_from_invalid() {
        assert!(MovImm64::try_from(0xffff_ffff_ffff_ffffu64).is_err());
        assert!(MovImm64::try_from(0x0000_0000_0001_0001u64).is_err());
        assert!(MovImm64::try_from(0x0000_0001_0000_0001u64).is_err());
        assert!(MovImm64::try_from(0x0001_0000_0000_0001u64).is_err());
        assert!(MovImm64::try_from(0x0001_8000_0000_0000u64).is_err());
        assert!(MovImm64::try_from(0x0000_0001_8000_0000u64).is_err());
        assert!(MovImm64::try_from(0x0000_0000_0001_8000u64).is_err());
    }

    #[test]
    fn test_bitmask_32() {
        for len in 1..6 {
            for ones in 1..(1 << len) {
                for rotation in 0..(1 << len) {
                    let bitmask = Bitmask32 {
                        imms: ((0b111110 << len) & 0x3f) | (ones - 1),
                        immr: rotation,
                    };
                    let imm: u32 = bitmask.into();
                    assert_eq!(Ok(bitmask), imm.try_into());
                }
            }
        }
    }

    #[test]
    fn test_bitmask_64() {
        for len in 1..7 {
            for ones in 1..(1 << len) {
                for rotation in 0..(1 << len) {
                    let bitmask = Bitmask64 {
                        n: if len > 5 { 1 } else { 0 },
                        imms: ((0b111110 << len) & 0x3f) | (ones - 1),
                        immr: rotation,
                    };
                    let imm: u64 = bitmask.into();
                    assert_eq!(Ok(bitmask), imm.try_into());
                }
            }
        }
    }
}
