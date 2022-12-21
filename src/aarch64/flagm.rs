use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    cfinv() => CFINV_M_pstate;
    rmif(rn: Register64, shift: u8, mask: u8) => RMIF_only_rmif {
        rn: rn.into(),
        imm6: shift.into(),
        mask: mask.into(),
    };
    setf8(rn: Register32) => SETF8_only_setf;
    setf16(rn: Register32) => SETF16_only_setf;
}
