use std::ops::Range;

use mitte_core::Error;

use crate::encoding::*;


pub enum FixupKind {
    PcRel14,
    PcRel19,
    PcRel26,
}

impl FixupKind {
    #[inline]
    pub fn apply_fixup(&self, instruction: u32, offset: i64) -> u32 {
        match *self {
            FixupKind::PcRel14 => {
                assert!(is_signed_nbit_integer(16, offset));
                let mask = encode_imm14(u32::MAX);
                let imm14 = encode_imm14(offset as u32 >> 2);
                (instruction & !mask) | imm14
            }
            FixupKind::PcRel19 => {
                assert!(is_signed_nbit_integer(21, offset));
                let mask = encode_imm19(u32::MAX);
                let imm19 = encode_imm19(offset as u32 >> 2);
                (instruction & !mask) | imm19
            }
            FixupKind::PcRel26 => {
                assert!(is_signed_nbit_integer(28, offset));
                let mask = encode_imm26(u32::MAX);
                let imm26 = encode_imm26(offset as u32 >> 2);
                (instruction & !mask) | imm26
            }
        }
    }
}

impl<Emit> mitte_core::FixupKind<Emit> for FixupKind
    where Emit: mitte_core::Emit + ?Sized
{
    #[inline]
    fn apply_fixup(&self, emit: &mut Emit, range: Range<u64>, offset: i64) -> Result<(), Error> {
        let buffer = emit.get_mut_array::<4>(range.start)?;
        let instruction = u32::from_le_bytes(*buffer);
        buffer.copy_from_slice(&self.apply_fixup(instruction, offset).to_le_bytes());
        Ok(())
    }
}

#[inline]
fn encode_imm14(imm14: u32) -> u32 {
    encode!(i13(0), i14(imm14), i5(0))
}

#[inline]
fn encode_imm19(imm19: u32) -> u32 {
    encode!(i8(0), i19(imm19), i5(0))
}

#[inline]
fn encode_imm26(imm26: u32) -> u32 {
    encode!(i6(0), i26(imm26))
}
