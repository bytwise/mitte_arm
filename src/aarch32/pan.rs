use super::encoding;

use super::macros::functions;


functions! {
    setpan(pan: bool) => SETPAN_A1 {
        imm1: pan.into(),
    };
}
