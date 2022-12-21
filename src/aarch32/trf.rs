use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    tsb_csync() => TSB_A1 {
        cond: Condition::Al,
    };
}
