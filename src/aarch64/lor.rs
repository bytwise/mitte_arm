use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    ldlar_32(rt: Register32, rn: Register64OrSp) => LDLAR_LR32_ldstord;
    ldlar_64(rt: Register64, rn: Register64OrSp) => LDLAR_LR64_ldstord;
    ldlarb(rt: Register32, rn: Register64OrSp) => LDLARB_LR32_ldstord;
    ldlarh(rt: Register32, rn: Register64OrSp) => LDLARH_LR32_ldstord;
    stllr_32(rt: Register32, rn: Register64OrSp) => STLLR_SL32_ldstord;
    stllr_64(rt: Register64, rn: Register64OrSp) => STLLR_SL64_ldstord;
    stllrb(rt: Register32, rn: Register64OrSp) => STLLRB_SL32_ldstord;
    stllrh(rt: Register32, rn: Register64OrSp) => STLLRH_SL32_ldstord;
}
