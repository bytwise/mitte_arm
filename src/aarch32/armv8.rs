use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    hlt(imm: u16) => HLT_A1 {
        cond: Condition::Al,
        imm12: (imm >> 4).into(),
        imm4: (imm & 0xf).into(),
    };
    lda#cond(rt: Register, rn: Register) => LDA_A1;
    ldab#cond(rt: Register, rn: Register) => LDAB_A1;
    ldaex#cond(rt: Register, rn: Register) => LDAEX_A1;
    ldaexb#cond(rt: Register, rn: Register) => LDAEXB_A1;
    ldaexd#cond(rt: RegisterPair, rn: Register) => LDAEXD_A1;
    ldaexh#cond(rt: Register, rn: Register) => LDAEXH_A1;
    ldah#cond(rt: Register, rn: Register) => LDAH_A1;
    pssbb() => PSSBB_A1;
    sevl#cond() => SEVL_A1;
    ssbb() => SSBB_A1;
    stl#cond(rt: Register, rn: Register) => STL_A1;
    stlb#cond(rt: Register, rn: Register) => STLB_A1;
    stlex#cond(rd: Register, rt: Register, rn: Register) => STLEX_A1;
    stlexb#cond(rd: Register, rt: Register, rn: Register) => STLEXB_A1;
    stlexd#cond(rd: Register, rt: RegisterPair, rn: Register) => STLEXD_A1;
    stlexh#cond(rd: Register, rt: Register, rn: Register) => STLEXH_A1;
    stlh#cond(rt: Register, rn: Register) => STLH_A1;
}
