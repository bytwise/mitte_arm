use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    autda(rd: Register64, rn: Register64OrSp) => AUTDA_64P_dp_1src;
    autdb(rd: Register64, rn: Register64OrSp) => AUTDB_64P_dp_1src;
    autdza(rd: Register64) => AUTDZA_64Z_dp_1src;
    autdzb(rd: Register64) => AUTDZB_64Z_dp_1src;
    autia(rd: Register64, rn: Register64OrSp) => AUTIA_64P_dp_1src;
    autia1716() => AUTIA1716_HI_hints;
    autiasp() => AUTIASP_HI_hints;
    autiaz() => AUTIAZ_HI_hints;
    autib(rd: Register64, rn: Register64OrSp) => AUTIB_64P_dp_1src;
    autib1716() => AUTIB1716_HI_hints;
    autibsp() => AUTIBSP_HI_hints;
    autibz() => AUTIBZ_HI_hints;
    autiza(rd: Register64) => AUTIZA_64Z_dp_1src;
    autizb(rd: Register64) => AUTIZB_64Z_dp_1src;
    blraa(rn: Register64, rm: Register64OrSp) => BLRAA_64P_branch_reg;
    blraaz(rn: Register64) => BLRAAZ_64_branch_reg;
    blrab(rn: Register64, rm: Register64OrSp) => BLRAB_64P_branch_reg;
    blrabz(rn: Register64) => BLRABZ_64_branch_reg;
    braa(rn: Register64, rm: Register64OrSp) => BRAA_64P_branch_reg;
    braaz(rn: Register64) => BRAAZ_64_branch_reg;
    brab(rn: Register64, rm: Register64OrSp) => BRAB_64P_branch_reg;
    brabz(rn: Register64) => BRABZ_64_branch_reg;
    eretaa() => ERETAA_64E_branch_reg;
    eretab() => ERETAB_64E_branch_reg;
    ldraa_offset(rt: Register64, rn: Register64OrSp, index: Index<u64>) => LDRAA_64_ldst_pac {
        rt: rt.into(),
        rn: rn.into(),
        s: (index.index >> 9) as u32 & 1,
        imm9: index.index as u32 & 0x1ff,
    };
    ldraa_pre(rt: Register64, rn: Register64OrSp, index: Index<u64>) => LDRAA_64W_ldst_pac {
        rt: rt.into(),
        rn: rn.into(),
        s: (index.index >> 9) as u32 & 1,
        imm9: index.index as u32 & 0x1ff,
    };
    ldrab_offset(rt: Register64, rn: Register64OrSp, index: Index<u64>) => LDRAB_64_ldst_pac {
        rt: rt.into(),
        rn: rn.into(),
        s: (index.index >> 9) as u32 & 1,
        imm9: index.index as u32 & 0x1ff,
    };
    ldrab_pre(rt: Register64, rn: Register64OrSp, index: Index<u64>) => LDRAB_64W_ldst_pac {
        rt: rt.into(),
        rn: rn.into(),
        s: (index.index >> 9) as u32 & 1,
        imm9: index.index as u32 & 0x1ff,
    };
    pacda(rd: Register64, rn: Register64OrSp) => PACDA_64P_dp_1src;
    pacdb(rd: Register64, rn: Register64OrSp) => PACDB_64P_dp_1src;
    pacdza(rd: Register64) => PACDZA_64Z_dp_1src;
    pacdzb(rd: Register64) => PACDZB_64Z_dp_1src;
    pacga(rd: Register64, rn: Register64, rm: Register64OrSp) => PACGA_64P_dp_2src;
    pacia(rd: Register64, rn: Register64OrSp) => PACIA_64P_dp_1src;
    pacia1716() => PACIA1716_HI_hints;
    paciasp() => PACIASP_HI_hints;
    paciaz() => PACIAZ_HI_hints;
    pacib(rd: Register64, rn: Register64OrSp) => PACIB_64P_dp_1src;
    pacib1716() => PACIB1716_HI_hints;
    pacibsp() => PACIBSP_HI_hints;
    pacibz() => PACIBZ_HI_hints;
    paciza(rd: Register64) => PACIZA_64Z_dp_1src;
    pacizb(rd: Register64) => PACIZB_64Z_dp_1src;
    retaa() => RETAA_64E_branch_reg;
    retab() => RETAB_64E_branch_reg;
    xpacd(rd: Register64) => XPACD_64Z_dp_1src;
    xpaci(rd: Register64) => XPACI_64Z_dp_1src;
    xpaclri() => XPACLRI_HI_hints;
}
