use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    crc32b(rd: Register, rn: Register, rm: Register) => CRC32B_A1 {
        cond: Condition::Al,
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
    };
    crc32cb(rd: Register, rn: Register, rm: Register) => CRC32CB_A1 {
        cond: Condition::Al,
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
    };
    crc32ch(rd: Register, rn: Register, rm: Register) => CRC32CH_A1 {
        cond: Condition::Al,
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
    };
    crc32cw(rd: Register, rn: Register, rm: Register) => CRC32CW_A1 {
        cond: Condition::Al,
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
    };
    crc32h(rd: Register, rn: Register, rm: Register) => CRC32H_A1 {
        cond: Condition::Al,
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
    };
    crc32w(rd: Register, rn: Register, rm: Register) => CRC32W_A1 {
        cond: Condition::Al,
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
    };
}
