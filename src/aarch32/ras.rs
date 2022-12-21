use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    esb() => ESB_A1 {
        cond: Condition::Al,
    };
}
