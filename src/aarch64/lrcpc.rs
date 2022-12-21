use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    ldapr_32(rt: Register32, rn: Register64OrSp) => LDAPR_32L_memop;
    ldapr_64(rt: Register64, rn: Register64OrSp) => LDAPR_64L_memop;
    ldaprb(rt: Register32, rn: Register64OrSp) => LDAPRB_32L_memop;
    ldaprh(rt: Register32, rn: Register64OrSp) => LDAPRH_32L_memop;
}
