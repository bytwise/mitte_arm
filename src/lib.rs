#[macro_use]
mod encoding;

use encoding::{ToBits, u4, u8, u12, i24};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

pub struct RegisterList(u16);

impl ToBits for RegisterList {
    #[inline(always)]
    fn size(&self) -> u8 { 16 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { self.0 as u32 }
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

impl_from_array! { 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 }

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

#[derive(Clone, Copy)]
pub enum Condition {
    Eq = 0,
    Ne = 1,
    Cs = 2,
    Cc = 3,
    Mi = 4,
    Pl = 5,
    Vs = 6,
    Vc = 7,
    Hi = 8,
    Ls = 9,
    Ge = 10,
    Lt = 11,
    Gt = 12,
    Le = 13,
    Al = 14,
}


pub enum Shift<T> {
    Lsl(T),
    Lsr(T),
    Asr(T),
    Ror(T),
}

impl<T> Shift<T> {
    fn ty(&self) -> u8 {
        match *self {
            Shift::Lsl(_) => 0,
            Shift::Lsr(_) => 1,
            Shift::Asr(_) => 2,
            Shift::Ror(_) => 3,
        }
    }

    fn amount(&self) -> &T {
        match *self {
            Shift::Lsl(ref amount) |
            Shift::Lsr(ref amount) |
            Shift::Asr(ref amount) |
            Shift::Ror(ref amount)
                => amount,
        }
    }
}

impl ToBits for Shift<u8> {
    #[inline(always)]
    fn size(&self) -> u8 { 8 }
    #[inline(always)]
    fn to_bits(&self) -> u32 {
        ((*self.amount() as u32 & 0x1f) << 3) | ((self.ty() as u32 & 3) << 1)
    }
}

impl ToBits for Shift<Register> {
    #[inline(always)]
    fn size(&self) -> u8 { 8 }
    #[inline(always)]
    fn to_bits(&self) -> u32 {
        (self.amount().to_bits() << 4) | ((self.ty() as u32 & 3) << 1) | 1
    }
}


pub struct Const {
    imm8: u8,
    rotation: u8,
}

impl ToBits for Const {
    #[inline(always)]
    fn size(&self) -> u8 { 12 }
    #[inline(always)]
    fn to_bits(&self) -> u32 { ((self.rotation as u32 & 0xf) << 8) | self.imm8 as u32 }
}

impl From<u8> for Const {
    #[inline(always)]
    fn from(x: u8) -> Const {
        Const {
            imm8: x,
            rotation: 0,
        }
    }
}

impl From<u32> for Const {
    #[inline]
    fn from(x: u32) -> Const {
        let rot_imm = (0..0x10).into_iter()
            .map(|r| (r, x.rotate_left(2 * r)))
            .find(|&(_, c)| c <= 0xff);
        if let Some((rotation, imm8)) = rot_imm {
            Const {
                imm8: imm8 as u8,
                rotation: rotation as u8,
            }
        } else {
            #[cold]
            fn const_from_fail(x: u32) -> ! {
                panic!("constant 0x{:x} cannot be encoded", x);
            }
            const_from_fail(x)
        }
    }
}

impl From<i32> for Const {
    #[inline]
    fn from(x: i32) -> Const {
        (x as u32).into()
    }
}


pub fn add_imm<C>(rd: Register, rn: Register, imm: C) -> [u8; 4]
    where C: Into<Const>
{
    encoding!(Condition::Al, u4(0b0010), u4(0b1000), rn, rd, imm.into())
}

#[inline]
pub fn add_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0000), u4(0b1000), rn, rd, shift, rm)
}

#[inline]
pub fn orr_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1000), rn, rd, shift, rm)
}

#[inline]
pub fn orr_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>)
    -> [u8; 4]
{
    encoding!(Condition::Al, u4(0b0001), u4(0b1000), rn, rd, shift, rm)
}

#[inline]
pub fn and_imm(rd: Register, rn: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0010), u4(0b0000), rn, rd, u4(0), u8(imm8))
}

#[inline]
pub fn and_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0000), u4(0b0000), rn, rd, shift, rm)
}

#[inline]
pub fn and_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>)
    -> [u8; 4]
{
    encoding!(Condition::Al, u4(0b0000), u4(0b0000), rn, rd, shift, rm)
}

#[inline]
pub fn eor_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0000), u4(0b0010), rn, rd, shift, rm)
}

#[inline]
pub fn sub_imm(rd: Register, rn: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0010), u4(0b0100), rn, rd, u4(0), u8(imm8))
}

#[inline]
pub fn sub_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0000), u4(0b0100), rn, rd, shift, rm)
}

#[inline]
pub fn rsb_imm(rd: Register, rn: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0010), u4(0b0110), rn, rd, u4(0), u8(imm8))
}

#[inline]
pub fn bic_imm(rd: Register, rn: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0011), u4(0b1100), rn, rd, u4(0), u8(imm8))
}

#[inline]
pub fn bic_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>)
    -> [u8; 4]
{
    encoding!(Condition::Al, u4(0b0001), u4(0b1100), rn, rd, shift, rm)
}

#[inline]
pub fn lsl_imm(rd: Register, rm: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, Shift::Lsl(imm8), rm)
}

#[inline]
pub fn lsl_reg(rd: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, rm, u4(0b0001), rn)
}

#[inline]
pub fn lsr_imm(rd: Register, rm: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, Shift::Lsr(imm8), rm)
}

#[inline]
pub fn lsr_reg(rd: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, rm, u4(0b0011), rn)
}

#[inline]
pub fn asr_imm(rd: Register, rm: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, Shift::Asr(imm8), rm)
}

#[inline]
pub fn asr_reg(rd: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, rm, u4(0b0101), rn)
}

#[inline]
pub fn ror_imm(rd: Register, rm: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, Shift::Ror(imm8), rm)
}

#[inline]
pub fn ror_reg(rd: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, rm, u4(0b0111), rn)
}

#[inline]
pub fn ldr_imm(rt: Register, rn: Register, imm12: u16) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0101), u4(0b1001), rn, rt, u12(imm12))
}

#[inline]
pub fn ldr_reg(rt: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0111), u4(0b1001), rn, rt, Shift::Lsl(0), rm)
}

#[inline]
pub fn ldrh_imm(rt: Register, rn: Register, imm8: u8) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1101), rn, rt, u4(imm8 >> 4), u4(0b1011), u4(imm8 & 0xf))
}

#[inline]
pub fn ldrh(rt: Register, rn: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1101), rn, rt, u4(0), u4(0b1011), u4(0))
}

#[inline]
pub fn str_imm(rt: Register, rn: Register, imm12: u16) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0101), u4(0b1000), rn, rt, u12(imm12))
}

#[inline]
pub fn str_reg(rt: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0111), u4(0b1000), rn, rt, Shift::Lsl(0), rm)
}

#[inline]
pub fn strh(rt: Register, rn: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1100), rn, rt, u4(0), u4(0b1011), u4(0))
}

#[inline]
pub fn strh_reg(rt: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1000), rn, rt, u4(0), u4(0b1011), rm)
}

pub fn mvn_imm<C>(rd: Register, imm: C) -> [u8; 4]
    where C: Into<Const>
{
    encoding!(Condition::Al, u4(0b0011), u4(0b1110), u4(0), rd, imm.into())
}

#[inline]
pub fn mvn_reg(rd: Register, rm: Register, shift: Shift<u8>) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1110), u4(0), rd, shift, rm)
}

pub fn mov_imm<C>(rd: Register, imm: C) -> [u8; 4]
    where C: Into<Const>
{
    encoding!(Condition::Al, u4(0b0011), u4(0b1010), u4(0), rd, imm.into())
}

#[inline]
pub fn mov_reg(rd: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b1010), u4(0), rd, u8(0), rm)
}

#[inline]
pub fn movt_cc(cond: Condition, rd: Register, imm16: u16) -> [u8; 4] {
    let imm4 = (imm16 >> 12) as u8 & 0xf;
    let imm12 = imm16 & 0xfff;
    encoding!(cond, u4(0b0011), u4(0b0100), u4(imm4), rd, u12(imm12))
}

#[inline]
pub fn movt(rd: Register, imm16: u16) -> [u8; 4] {
    movt_cc(Condition::Al, rd, imm16)
}

#[inline]
pub fn movw_cc(cond: Condition, rd: Register, imm16: u16) -> [u8; 4] {
    let imm4 = (imm16 >> 12) as u8 & 0xf;
    let imm12 = imm16 & 0xfff;
    encoding!(cond, u4(0b0011), u4(0b0000), u4(imm4), rd, u12(imm12))
}

#[inline]
pub fn movw(rd: Register, imm16: u16) -> [u8; 4] {
    movw_cc(Condition::Al, rd, imm16)
}

#[inline]
pub fn sxtb(rd: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0110), u4(0b1010), u4(0b1111), rd, u4(0b0000), u4(0b0111), rm)
}

#[inline]
pub fn sxth(rd: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0110), u4(0b1011), u4(0b1111), rd, u4(0b0000), u4(0b0111), rm)
}

#[inline]
pub fn uxth(rd: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0110), u4(0b1111), u4(0b1111), rd, u4(0b0000), u4(0b0111), rm)
}

pub fn cmp_imm<C>(rn: Register, imm: C) -> [u8; 4]
    where C: Into<Const>
{
    encoding!(Condition::Al, u4(0b0011), u4(0b0101), rn, u4(0), imm.into())
}

#[inline]
pub fn cmp_reg(rn: Register, rm: Register, shift: Shift<u8>) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0001), u4(0b0101), rn, u4(0), shift, rm)
}

#[inline]
pub fn cmn_imm_cc(cond: Condition, rn: Register, imm8: u8) -> [u8; 4] {
    encoding!(cond, u4(0b0011), u4(0b0111), rn, u4(0), u4(0), u8(imm8))
}

#[inline]
pub fn blx(rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u8(0b0001_0010), u4(0b1111), u4(0b1111), u4(0b1111), u4(0b0011), rm)
}

#[inline]
pub fn b_cc(cond: Condition, imm24: i32) -> [u8; 4] {
    encoding!(cond, u4(0b1010), i24(imm24))
}

#[inline]
pub fn b(imm24: i32) -> [u8; 4] {
    b_cc(Condition::Al, imm24)
}

#[inline]
pub fn beq(imm24: i32) -> [u8; 4] {
    b_cc(Condition::Eq, imm24)
}

#[inline]
pub fn bne(imm24: i32) -> [u8; 4] {
    b_cc(Condition::Ne, imm24)
}

#[inline]
pub fn bgt(imm24: i32) -> [u8; 4] {
    b_cc(Condition::Gt, imm24)
}

#[inline]
pub fn bl(imm24: i32) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b1011), i24(imm24))
}

pub fn push<L>(regs: L) -> [u8; 4]
    where L: Into<RegisterList>
{
    encoding!(Condition::Al, u4(0b1001), u4(0b0010), u4(0b1101), regs.into())
}

pub fn pop<L>(regs: L) -> [u8; 4]
    where L: Into<RegisterList>
{
    encoding!(Condition::Al, u4(0b1000), u4(0b1011), u4(0b1101), regs.into())
}

#[inline]
pub fn smull(rdlo: Register, rdhi: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0000), u4(0b1100), rdhi, rdlo, rm, u4(0b1001), rn)
}

#[inline]
pub fn umull(rdlo: Register, rdhi: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0000), u4(0b1000), rdhi, rdlo, rm, u4(0b1001), rn)
}

#[inline]
pub fn sdiv(rd: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0111), u4(0b0001), rd, u4(0b1111), rm, u4(0b0001), rn)
}

#[inline]
pub fn udiv(rd: Register, rn: Register, rm: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0111), u4(0b0011), rd, u4(0b1111), rm, u4(0b0001), rn)
}

#[inline]
pub fn mls(rd: Register, rn: Register, rm: Register, ra: Register) -> [u8; 4] {
    encoding!(Condition::Al, u4(0b0000), u4(0b0110), rd, ra, rm, u4(0b1001), rn)
}
