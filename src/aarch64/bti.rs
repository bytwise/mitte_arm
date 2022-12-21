use super::encoding;
use super::BranchTargets;

use super::macros::functions;


functions! {
    bti(targets: BranchTargets) => BTI_HB_hints {
        op2: targets as u32,
    };
}
