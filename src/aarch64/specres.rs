use super::encoding;
use super::Register64;

use super::macros::functions;


functions! {
    cfp_rctx(rt: Register64) => CFP_SYS_CR_systeminstrs;
    cpp_rctx(rt: Register64) => CPP_SYS_CR_systeminstrs;
    dvp_rctx(rt: Register64) => DVP_SYS_CR_systeminstrs;
}
