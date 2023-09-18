use std::ops::Range;

use mitte_core::Error;

use crate::encoding::*;


pub enum FixupKind {
    Branch24,
}

impl FixupKind {
    #[inline]
    pub fn apply_fixup(&self, instruction: u32, offset: i64) -> u32 {
        match *self {
            FixupKind::Branch24 => {
                assert!(is_signed_nbit_integer(26, offset - 8));
                let mask = encode_imm24(u32::MAX);
                let imm24 = encode_imm24((offset - 8) as u32 >> 2);
                (instruction & !mask) | imm24
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
fn encode_imm24(imm24: u32) -> u32 {
    encode!(i8(0), i24(imm24))
}
