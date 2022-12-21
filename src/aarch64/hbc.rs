use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    bc(cond: Condition, label: Label) => BC_only_condbranch {
        cond,
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
}
