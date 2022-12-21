use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    crc32b(rd: Register32, rn: Register32, rm: Register32) => CRC32B_32C_dp_2src;
    crc32cb(rd: Register32, rn: Register32, rm: Register32) => CRC32CB_32C_dp_2src;
    crc32ch(rd: Register32, rn: Register32, rm: Register32) => CRC32CH_32C_dp_2src;
    crc32cw(rd: Register32, rn: Register32, rm: Register32) => CRC32CW_32C_dp_2src;
    crc32cx(rd: Register32, rn: Register32, rm: Register64) => CRC32CX_64C_dp_2src;
    crc32h(rd: Register32, rn: Register32, rm: Register32) => CRC32H_32C_dp_2src;
    crc32w(rd: Register32, rn: Register32, rm: Register32) => CRC32W_32C_dp_2src;
    crc32x(rd: Register32, rn: Register32, rm: Register64) => CRC32X_64C_dp_2src;
}
