pub struct Bits {
    pub bits: u32,
    pub shift: u8,
}

impl Bits {
    #[inline(always)]
    pub fn new() -> Bits {
        Bits { bits: 0, shift: 32, }
    }

    #[inline(always)]
    pub fn add(&mut self, x: impl ToBits) {
        self.shift -= x.size();
        self.bits |= x.to_bits() << self.shift;
    }
}

pub trait ToBits {
    fn size(&self) -> u8;
    fn to_bits(&self) -> u32;
}

impl ToBits for ::Condition {
    #[inline(always)]
    fn size(&self) -> u8 { 4 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { *self as u32 }
}

impl ToBits for ::Register {
    #[inline(always)]
    fn size(&self) -> u8 { 4 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { *self as u32 }
}

pub struct U4(u8);

#[inline(always)]
pub fn u4(x: u8) -> U4 {
    // TODO check range
    U4(x)
}

impl ToBits for U4 {
    #[inline(always)]
    fn size(&self) -> u8 { 4 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { self.0 as u32 }
}

pub struct U8(u8);

#[inline(always)]
pub fn u8(x: u8) -> U8 {
    U8(x)
}

impl ToBits for U8 {
    #[inline(always)]
    fn size(&self) -> u8 { 8 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { self.0 as u32 }
}

pub struct U12(u16);

#[inline(always)]
pub fn u12(x: u16) -> U12 {
    // TODO check range
    U12(x)
}

impl ToBits for U12 {
    #[inline(always)]
    fn size(&self) -> u8 { 12 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { self.0 as u32 }
}

pub struct I24(i32);

#[inline(always)]
pub fn i24(x: i32) -> I24 {
    // TODO check range
    I24(x)
}

impl ToBits for I24 {
    #[inline(always)]
    fn size(&self) -> u8 { 24 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { self.0 as u32 & 0xff_ffff }
}

macro_rules! encoding {
    ($($e:expr),*) => {
        {
            let mut bits = ::encoding::Bits::new();
            $(
            bits.add($e);
            )*
            bits.bits.to_le_bytes()
        }
    };
}
