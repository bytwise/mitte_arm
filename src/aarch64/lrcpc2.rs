use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    ldapur_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDAPUR_32_ldapstl_unscaled;
    ldapur_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDAPUR_64_ldapstl_unscaled;
    ldapurb(rt: Register32, rn: Register64OrSp; imm9: i16) => LDAPURB_32_ldapstl_unscaled;
    ldapurh(rt: Register32, rn: Register64OrSp; imm9: i16) => LDAPURH_32_ldapstl_unscaled;
    ldapursb_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDAPURSB_32_ldapstl_unscaled;
    ldapursb_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDAPURSB_64_ldapstl_unscaled;
    ldapursh_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDAPURSH_32_ldapstl_unscaled;
    ldapursh_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDAPURSH_64_ldapstl_unscaled;
    ldapursw(rt: Register64, rn: Register64OrSp; imm9: i16) => LDAPURSW_64_ldapstl_unscaled;
    stlur_32(rt: Register32, rn: Register64OrSp; imm9: i16) => STLUR_32_ldapstl_unscaled;
    stlur_64(rt: Register64, rn: Register64OrSp; imm9: i16) => STLUR_64_ldapstl_unscaled;
    stlurb(rt: Register32, rn: Register64OrSp; imm9: i16) => STLURB_32_ldapstl_unscaled;
    stlurh(rt: Register32, rn: Register64OrSp; imm9: i16) => STLURH_32_ldapstl_unscaled;
}
