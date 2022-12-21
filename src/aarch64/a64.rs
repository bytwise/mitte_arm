use crate::encoding::RegisterIndex;

use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    adc_32(rd: Register32, rn: Register32, rm: Register32) => ADC_32_addsub_carry;
    adc_64(rd: Register64, rn: Register64, rm: Register64) => ADC_64_addsub_carry;
    adcs_32(rd: Register32, rn: Register32, rm: Register32) => ADCS_32_addsub_carry;
    adcs_64(rd: Register64, rn: Register64, rm: Register64) => ADCS_64_addsub_carry;
    add_ext_32(rd: Register32OrSp, rn: Register32OrSp, rm: Register32; extend: Extend) => ADD_32_addsub_ext;
    add_ext_64(rd: Register64OrSp, rn: Register64OrSp, rm: Register64; extend: Extend) => ADD_64_addsub_ext;
    add_imm_32(rd: Register32OrSp, rn: Register32OrSp; imm: impl Into<ShiftedImm12>) => ADD_32_addsub_imm;
    add_imm_64(rd: Register64OrSp, rn: Register64OrSp; imm: impl Into<ShiftedImm12>) => ADD_64_addsub_imm;
    add_shift_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => ADD_32_addsub_shift;
    add_shift_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => ADD_64_addsub_shift;
    adds_ext_32(rd: Register32, rn: Register32OrSp, rm: Register32; extend: Extend) => ADDS_32S_addsub_ext;
    adds_ext_64(rd: Register64, rn: Register64OrSp, rm: Register64; extend: Extend) => ADDS_64S_addsub_ext;
    adds_imm_32(rd: Register32, rn: Register32OrSp; imm: impl Into<ShiftedImm12>) => ADDS_32S_addsub_imm;
    adds_imm_64(rd: Register64, rn: Register64OrSp; imm: impl Into<ShiftedImm12>) => ADDS_64S_addsub_imm;
    adds_shift_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => ADDS_32_addsub_shift;
    adds_shift_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => ADDS_64_addsub_shift;
    adr(rd: Register64, label: Label) => ADR_only_pcreladdr {
        rd: rd.into(),
        immhi: (label.offset >> 2) as u32,
        immlo: (label.offset & 3) as u32,
    };
    adrp(rd: Register64, label: Label) => ADRP_only_pcreladdr {
        rd: rd.into(),
        immhi: (label.offset >> 14) as u32,
        immlo: ((label.offset >> 12) & 3) as u32,
    };
    and_imm_32(rd: Register32OrSp, rn: Register32; imm: Bitmask32) => AND_32_log_imm;
    and_imm_64(rd: Register64OrSp, rn: Register64; imm: Bitmask64) => AND_64_log_imm;
    and_reg_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => AND_32_log_shift;
    and_reg_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => AND_64_log_shift;
    ands_imm_32(rd: Register32, rn: Register32; imm: Bitmask32) => ANDS_32S_log_imm;
    ands_imm_64(rd: Register64, rn: Register64; imm: Bitmask64) => ANDS_64S_log_imm;
    ands_reg_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => ANDS_32_log_shift;
    ands_reg_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => ANDS_64_log_shift;
    asr_imm_32(rd: Register32, rn: Register32, shift: u8) => ASR_SBFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: shift as u32,
    };
    asr_imm_64(rd: Register64, rn: Register64, shift: u8) => ASR_SBFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: shift as u32,
    };
    asr_reg_32(rd: Register32, rn: Register32, rm: Register32) => ASR_ASRV_32_dp_2src;
    asr_reg_64(rd: Register64, rn: Register64, rm: Register64) => ASR_ASRV_64_dp_2src;
    asrv_32(rd: Register32, rn: Register32, rm: Register32) => ASRV_32_dp_2src;
    asrv_64(rd: Register64, rn: Register64, rm: Register64) => ASRV_64_dp_2src;
    at(op1: u8, crm: u8, op2: u8, rt: Register64) => AT_SYS_CR_systeminstrs;
    b(label: Label) => B_only_branch_imm {
        imm26: (label.offset >> 2) as u32 & 0x3ff_ffff,
    };
    b_cond(cond: Condition, label: Label) => B_only_condbranch {
        cond,
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    bfc_32(rd: Register32, lsb: u8, width: u8) => BFC_BFM_32M_bitfield {
        rd: rd.into(),
        immr: (lsb as u32).wrapping_neg() & 31,
        imms: width as u32 - 1,
    };
    bfc_64(rd: Register64, lsb: u8, width: u8) => BFC_BFM_64M_bitfield {
        rd: rd.into(),
        immr: (lsb as u32).wrapping_neg() & 63,
        imms: width as u32 - 1,
    };
    bfi_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => BFI_BFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (lsb as u32).wrapping_neg() & 31,
        imms: width as u32 - 1,
    };
    bfi_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => BFI_BFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (lsb as u32).wrapping_neg() & 63,
        imms: width as u32 - 1,
    };
    bfm_32(rd: Register32, rn: Register32, immr: u8, imms: u8) => BFM_32M_bitfield;
    bfm_64(rd: Register64, rn: Register64, immr: u8, imms: u8) => BFM_64M_bitfield;
    bfxil_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => BFXIL_BFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: lsb as u32,
        imms: lsb as u32 + width as u32 - 1,
    };
    bfxil_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => BFXIL_BFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: lsb as u32,
        imms: lsb as u32 + width as u32 - 1,
    };
    bic_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => BIC_32_log_shift;
    bic_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => BIC_64_log_shift;
    bics_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => BICS_32_log_shift;
    bics_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => BICS_64_log_shift;
    bl(label: Label) => BL_only_branch_imm {
        imm26: (label.offset >> 2) as u32 & 0x3ff_ffff,
    };
    blr(rn: Register64) => BLR_64_branch_reg;
    br(rn: Register64) => BR_64_branch_reg;
    brk(imm16: u16) => BRK_EX_exception;
    cbnz_32(rt: Register32, label: Label) => CBNZ_32_compbranch {
        rt: rt.into(),
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    cbnz_64(rt: Register64, label: Label) => CBNZ_64_compbranch {
        rt: rt.into(),
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    cbz_32(rt: Register32, label: Label) => CBZ_32_compbranch {
        rt: rt.into(),
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    cbz_64(rt: Register64, label: Label) => CBZ_64_compbranch {
        rt: rt.into(),
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    ccmn_imm_32(rn: Register32, imm5: u8, nzcv: u8, cond: Condition) => CCMN_32_condcmp_imm;
    ccmn_imm_64(rn: Register64, imm5: u8, nzcv: u8, cond: Condition) => CCMN_64_condcmp_imm;
    ccmn_reg_32(rn: Register32, rm: Register32, nzcv: u8, cond: Condition) => CCMN_32_condcmp_reg;
    ccmn_reg_64(rn: Register64, rm: Register64, nzcv: u8, cond: Condition) => CCMN_64_condcmp_reg;
    ccmp_imm_32(rn: Register32, imm5: u8, nzcv: u8, cond: Condition) => CCMP_32_condcmp_imm;
    ccmp_imm_64(rn: Register64, imm5: u8, nzcv: u8, cond: Condition) => CCMP_64_condcmp_imm;
    ccmp_reg_32(rn: Register32, rm: Register32, nzcv: u8, cond: Condition) => CCMP_32_condcmp_reg;
    ccmp_reg_64(rn: Register64, rm: Register64, nzcv: u8, cond: Condition) => CCMP_64_condcmp_reg;
    cinc_32(rd: Register32, rn: Register32, cond: Condition) => CINC_CSINC_32_condsel {
        rd: rd.into(),
        rn: rn.into(),
        rm: rn.into(),
        cond: cond.invert().unwrap(),
    };
    cinc_64(rd: Register64, rn: Register64, cond: Condition) => CINC_CSINC_64_condsel {
        rd: rd.into(),
        rn: rn.into(),
        rm: rn.into(),
        cond: cond.invert().unwrap(),
    };
    cinv_32(rd: Register32, rn: Register32, cond: Condition) => CINV_CSINV_32_condsel {
        rd: rd.into(),
        rn: rn.into(),
        rm: rn.into(),
        cond: cond.invert().unwrap(),
    };
    cinv_64(rd: Register64, rn: Register64, cond: Condition) => CINV_CSINV_64_condsel {
        rd: rd.into(),
        rn: rn.into(),
        rm: rn.into(),
        cond: cond.invert().unwrap(),
    };
    clrex(crm: u8) => CLREX_BN_barriers;
    cls_32(rd: Register32, rn: Register32) => CLS_32_dp_1src;
    cls_64(rd: Register64, rn: Register64) => CLS_64_dp_1src;
    clz_32(rd: Register32, rn: Register32) => CLZ_32_dp_1src;
    clz_64(rd: Register64, rn: Register64) => CLZ_64_dp_1src;
    cmn_ext_32(rn: Register32OrSp, rm: Register32; extend: Extend) => CMN_ADDS_32S_addsub_ext;
    cmn_ext_64(rn: Register64OrSp, rm: Register64; extend: Extend) => CMN_ADDS_64S_addsub_ext;
    cmn_imm_32(rn: Register32OrSp; imm: impl Into<ShiftedImm12>) => CMN_ADDS_32S_addsub_imm;
    cmn_imm_64(rn: Register64OrSp; imm: impl Into<ShiftedImm12>) => CMN_ADDS_64S_addsub_imm;
    cmn_shift_32(rn: Register32, rm: Register32; shift: Shift<u8>) => CMN_ADDS_32_addsub_shift;
    cmn_shift_64(rn: Register64, rm: Register64; shift: Shift<u8>) => CMN_ADDS_64_addsub_shift;
    cmp_ext_32(rn: Register32OrSp, rm: Register32; extend: Extend) => CMP_SUBS_32S_addsub_ext;
    cmp_ext_64(rn: Register64OrSp, rm: Register64; extend: Extend) => CMP_SUBS_64S_addsub_ext;
    cmp_imm_32(rn: Register32OrSp; imm: impl Into<ShiftedImm12>) => CMP_SUBS_32S_addsub_imm;
    cmp_imm_64(rn: Register64OrSp; imm: impl Into<ShiftedImm12>) => CMP_SUBS_64S_addsub_imm;
    cmp_shift_32(rn: Register32, rm: Register32; shift: Shift<u8>) => CMP_SUBS_32_addsub_shift;
    cmp_shift_64(rn: Register64, rm: Register64; shift: Shift<u8>) => CMP_SUBS_64_addsub_shift;
    cneg_32(rd: Register32, rn: Register32, cond: Condition) => CNEG_CSNEG_32_condsel {
        rd: rd.into(),
        rn: rn.into(),
        rm: rn.into(),
        cond: cond.invert().unwrap(),
    };
    cneg_64(rd: Register64, rn: Register64, cond: Condition) => CNEG_CSNEG_64_condsel {
        rd: rd.into(),
        rn: rn.into(),
        rm: rn.into(),
        cond: cond.invert().unwrap(),
    };
    csdb() => CSDB_HI_hints;
    csel_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => CSEL_32_condsel;
    csel_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => CSEL_64_condsel;
    cset_32(rd: Register32, cond: Condition) => CSET_CSINC_32_condsel {
        rd: rd.into(),
        cond: cond.invert().unwrap(),
    };
    cset_64(rd: Register64, cond: Condition) => CSET_CSINC_64_condsel {
        rd: rd.into(),
        cond: cond.invert().unwrap(),
    };
    csetm_32(rd: Register32, cond: Condition) => CSETM_CSINV_32_condsel {
        rd: rd.into(),
        cond: cond.invert().unwrap(),
    };
    csetm_64(rd: Register64, cond: Condition) => CSETM_CSINV_64_condsel {
        rd: rd.into(),
        cond: cond.invert().unwrap(),
    };
    csinc_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => CSINC_32_condsel;
    csinc_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => CSINC_64_condsel;
    csinv_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => CSINV_32_condsel;
    csinv_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => CSINV_64_condsel;
    csneg_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => CSNEG_32_condsel;
    csneg_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => CSNEG_64_condsel;
    dc(op1: u8, crm: u8, op2: u8, rt: Register64) => DC_SYS_CR_systeminstrs;
    dcps1(imm16: u16) => DCPS1_DC_exception;
    dcps2(imm16: u16) => DCPS2_DC_exception;
    dcps3(imm16: u16) => DCPS3_DC_exception;
    dmb(crm: u8) => DMB_BO_barriers;
    drps() => DRPS_64E_branch_reg;
    dsb(crm: u8) => DSB_BO_barriers;
    eon_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => EON_32_log_shift;
    eon_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => EON_64_log_shift;
    eor_imm_32(rd: Register32OrSp, rn: Register32; imm: Bitmask32) => EOR_32_log_imm;
    eor_imm_64(rd: Register64OrSp, rn: Register64; imm: Bitmask64) => EOR_64_log_imm;
    eor_reg_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => EOR_32_log_shift;
    eor_reg_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => EOR_64_log_shift;
    eret() => ERET_64E_branch_reg;
    extr_32(rd: Register32, rn: Register32, rm: Register32, lsb: u8) => EXTR_32_extract {
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
        imms: lsb.into(),
    };
    extr_64(rd: Register64, rn: Register64, rm: Register64, lsb: u8) => EXTR_64_extract {
        rd: rd.into(),
        rn: rn.into(),
        rm: rm.into(),
        imms: lsb.into(),
    };
    hint(imm: u8) => HINT_HM_hints {
        crm: (imm >> 3).into(),
        op2: (imm & 7).into(),
    };
    hlt(imm16: u16) => HLT_EX_exception;
    hvc(imm16: u16) => HVC_EX_exception;
    ic(op1: u8, crm: u8, op2: u8, rt: Register64) => IC_SYS_CR_systeminstrs;
    isb(crm: u8) => ISB_BI_barriers;
    ldar_32(rt: Register32, rn: Register64OrSp) => LDAR_LR32_ldstord;
    ldar_64(rt: Register64, rn: Register64OrSp) => LDAR_LR64_ldstord;
    ldarb(rt: Register32, rn: Register64OrSp) => LDARB_LR32_ldstord;
    ldarh(rt: Register32, rn: Register64OrSp) => LDARH_LR32_ldstord;
    ldaxp_32(rt: Register32, rt2: Register32, rn: Register64OrSp) => LDAXP_LP32_ldstexclp;
    ldaxp_64(rt: Register64, rt2: Register64, rn: Register64OrSp) => LDAXP_LP64_ldstexclp;
    ldaxr_32(rt: Register32, rn: Register64OrSp) => LDAXR_LR32_ldstexclr;
    ldaxr_64(rt: Register64, rn: Register64OrSp) => LDAXR_LR64_ldstexclr;
    ldaxrb(rt: Register32, rn: Register64OrSp) => LDAXRB_LR32_ldstexclr;
    ldaxrh(rt: Register32, rn: Register64OrSp) => LDAXRH_LR32_ldstexclr;
    ldnp_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => LDNP_32_ldstnapair_offs {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldnp_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => LDNP_64_ldstnapair_offs {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldp_offset_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => LDP_32_ldstpair_off {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldp_offset_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => LDP_64_ldstpair_off {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldp_post_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => LDP_32_ldstpair_post {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldp_post_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => LDP_64_ldstpair_post {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldp_pre_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => LDP_32_ldstpair_pre {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldp_pre_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => LDP_64_ldstpair_pre {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldpsw_offset(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u32>) => LDPSW_64_ldstpair_off {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldpsw_post(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u32>) => LDPSW_64_ldstpair_post {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldpsw_pre(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u32>) => LDPSW_64_ldstpair_pre {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    ldr_immpost_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDR_32_ldst_immpost;
    ldr_immpost_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDR_64_ldst_immpost;
    ldr_immpre_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDR_32_ldst_immpre;
    ldr_immpre_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDR_64_ldst_immpre;
    ldr_lit_32(rt: Register32, label: Label) => LDR_32_loadlit {
        rt: rt.into(),
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    ldr_lit_64(rt: Register64, label: Label) => LDR_64_loadlit {
        rt: rt.into(),
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    ldr_pos_32(rt: Register32, rn: Register64OrSp, index: Index<u32>) => LDR_32_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    ldr_pos_64(rt: Register64, rn: Register64OrSp, index: Index<u64>) => LDR_64_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    ldr_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDR_32_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldr_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDR_64_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldrb_immpost(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRB_32_ldst_immpost;
    ldrb_immpre(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRB_32_ldst_immpre;
    ldrb_pos(rt: Register32, rn: Register64OrSp, imm12: u16) => LDRB_32_ldst_pos;
    ldrb_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDRB_32B_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldrh_immpost(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRH_32_ldst_immpost;
    ldrh_immpre(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRH_32_ldst_immpre;
    ldrh_pos(rt: Register32, rn: Register64OrSp, index: Index<u16>) => LDRH_32_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    ldrh_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDRH_32_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldrsb_immpost_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRSB_32_ldst_immpost;
    ldrsb_immpost_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDRSB_64_ldst_immpost;
    ldrsb_immpre_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRSB_32_ldst_immpre;
    ldrsb_immpre_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDRSB_64_ldst_immpre;
    ldrsb_pos_32(rt: Register32, rn: Register64OrSp, imm12: u16) => LDRSB_32_ldst_pos;
    ldrsb_pos_64(rt: Register64, rn: Register64OrSp, imm12: u16) => LDRSB_64_ldst_pos;
    ldrsb_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDRSB_32B_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldrsb_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDRSB_64B_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldrsh_immpost_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRSH_32_ldst_immpost;
    ldrsh_immpost_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDRSH_64_ldst_immpost;
    ldrsh_immpre_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDRSH_32_ldst_immpre;
    ldrsh_immpre_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDRSH_64_ldst_immpre;
    ldrsh_pos_32(rt: Register32, rn: Register64OrSp, index: Index<u16>) => LDRSH_32_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    ldrsh_pos_64(rt: Register64, rn: Register64OrSp, index: Index<u16>) => LDRSH_64_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    ldrsh_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDRSH_32_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldrsh_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDRSH_64_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldrsw_immpost(rt: Register64, rn: Register64OrSp; imm9: i16) => LDRSW_64_ldst_immpost;
    ldrsw_immpre(rt: Register64, rn: Register64OrSp; imm9: i16) => LDRSW_64_ldst_immpre;
    ldrsw_lit(rt: Register64, label: Label) => LDRSW_64_loadlit {
        rt: rt.into(),
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    ldrsw_pos(rt: Register64, rn: Register64OrSp, index: Index<u32>) => LDRSW_64_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    ldrsw_regoff(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => LDRSW_64_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    ldtr_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDTR_32_ldst_unpriv;
    ldtr_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDTR_64_ldst_unpriv;
    ldtrb(rt: Register32, rn: Register64OrSp; imm9: i16) => LDTRB_32_ldst_unpriv;
    ldtrh(rt: Register32, rn: Register64OrSp; imm9: i16) => LDTRH_32_ldst_unpriv;
    ldtrsb_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDTRSB_32_ldst_unpriv;
    ldtrsb_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDTRSB_64_ldst_unpriv;
    ldtrsh_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDTRSH_32_ldst_unpriv;
    ldtrsh_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDTRSH_64_ldst_unpriv;
    ldtrsw(rt: Register64, rn: Register64OrSp; imm9: i16) => LDTRSW_64_ldst_unpriv;
    ldur_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDUR_32_ldst_unscaled;
    ldur_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDUR_64_ldst_unscaled;
    ldurb(rt: Register32, rn: Register64OrSp; imm9: i16) => LDURB_32_ldst_unscaled;
    ldurh(rt: Register32, rn: Register64OrSp; imm9: i16) => LDURH_32_ldst_unscaled;
    ldursb_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDURSB_32_ldst_unscaled;
    ldursb_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDURSB_64_ldst_unscaled;
    ldursh_32(rt: Register32, rn: Register64OrSp; imm9: i16) => LDURSH_32_ldst_unscaled;
    ldursh_64(rt: Register64, rn: Register64OrSp; imm9: i16) => LDURSH_64_ldst_unscaled;
    ldursw(rt: Register64, rn: Register64OrSp; imm9: i16) => LDURSW_64_ldst_unscaled;
    ldxp_32(rt: Register32, rt2: Register32, rn: Register64OrSp) => LDXP_LP32_ldstexclp;
    ldxp_64(rt: Register64, rt2: Register64, rn: Register64OrSp) => LDXP_LP64_ldstexclp;
    ldxr_32(rt: Register32, rn: Register64OrSp) => LDXR_LR32_ldstexclr;
    ldxr_64(rt: Register64, rn: Register64OrSp) => LDXR_LR64_ldstexclr;
    ldxrb(rt: Register32, rn: Register64OrSp) => LDXRB_LR32_ldstexclr;
    ldxrh(rt: Register32, rn: Register64OrSp) => LDXRH_LR32_ldstexclr;
    lsl_imm_32(rd: Register32, rn: Register32, shift: u8) => LSL_UBFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (shift as u32).wrapping_neg() & 31,
        imms: 31 - shift as u32,
    };
    lsl_imm_64(rd: Register64, rn: Register64, shift: u8) => LSL_UBFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (shift as u32).wrapping_neg() & 63,
        imms: 63 - shift as u32,
    };
    lsl_reg_32(rd: Register32, rn: Register32, rm: Register32) => LSL_LSLV_32_dp_2src;
    lsl_reg_64(rd: Register64, rn: Register64, rm: Register64) => LSL_LSLV_64_dp_2src;
    lslv_32(rd: Register32, rn: Register32, rm: Register32) => LSLV_32_dp_2src;
    lslv_64(rd: Register64, rn: Register64, rm: Register64) => LSLV_64_dp_2src;
    lsr_imm_32(rd: Register32, rn: Register32, shift: u8) => LSR_UBFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: shift as u32,
    };
    lsr_imm_64(rd: Register64, rn: Register64, shift: u8) => LSR_UBFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: shift as u32,
    };
    lsr_reg_32(rd: Register32, rn: Register32, rm: Register32) => LSR_LSRV_32_dp_2src;
    lsr_reg_64(rd: Register64, rn: Register64, rm: Register64) => LSR_LSRV_64_dp_2src;
    lsrv_32(rd: Register32, rn: Register32, rm: Register32) => LSRV_32_dp_2src;
    lsrv_64(rd: Register64, rn: Register64, rm: Register64) => LSRV_64_dp_2src;
    madd_32(rd: Register32, rn: Register32, rm: Register32, ra: Register32) => MADD_32A_dp_3src;
    madd_64(rd: Register64, rn: Register64, rm: Register64, ra: Register64) => MADD_64A_dp_3src;
    mneg_32(rd: Register32, rn: Register32, rm: Register32) => MNEG_MSUB_32A_dp_3src;
    mneg_64(rd: Register64, rn: Register64, rm: Register64) => MNEG_MSUB_64A_dp_3src;
    mov_bitmask_32(rd: Register32OrSp; imm: Bitmask32) => MOV_ORR_32_log_imm;
    mov_bitmask_64(rd: Register64OrSp; imm: Bitmask64) => MOV_ORR_64_log_imm;
    mov_reg_32(rd: Register32, rm: Register32) => MOV_ORR_32_log_shift;
    mov_reg_64(rd: Register64, rm: Register64) => MOV_ORR_64_log_shift;
    mov_sp_32(rd: Register32OrSp, rn: Register32OrSp) => MOV_ADD_32_addsub_imm;
    mov_sp_64(rd: Register64OrSp, rn: Register64OrSp) => MOV_ADD_64_addsub_imm;
    movk_32(rd: Register32; imm: impl Into<MovImm32>) => MOVK_32_movewide;
    movk_64(rd: Register64; imm: impl Into<MovImm64>) => MOVK_64_movewide;
    movn_32(rd: Register32; imm: impl Into<MovImm32>) => MOVN_32_movewide;
    movn_64(rd: Register64; imm: impl Into<MovImm64>) => MOVN_64_movewide;
    movz_32(rd: Register32; imm: impl Into<MovImm32>) => MOVZ_32_movewide;
    movz_64(rd: Register64; imm: impl Into<MovImm64>) => MOVZ_64_movewide;
    mrs(rt: Register64, o0: u8, op1: u8, crn: u8, crm: u8, op2: u8) => MRS_RS_systemmove;
    msr_imm(op1: u8, op2: u8, imm: u8) => MSR_SI_pstate {
        op1: op1.into(),
        op2: op2.into(),
        crm: imm.into(),
    };
    msr_reg(o0: u8, op1: u8, crn: u8, crm: u8, op2: u8, rt: Register64) => MSR_SR_systemmove;
    msub_32(rd: Register32, rn: Register32, rm: Register32, ra: Register32) => MSUB_32A_dp_3src;
    msub_64(rd: Register64, rn: Register64, rm: Register64, ra: Register64) => MSUB_64A_dp_3src;
    mul_32(rd: Register32, rn: Register32, rm: Register32) => MUL_MADD_32A_dp_3src;
    mul_64(rd: Register64, rn: Register64, rm: Register64) => MUL_MADD_64A_dp_3src;
    mvn_32(rd: Register32, rm: Register32; shift: Shift<u8>) => MVN_ORN_32_log_shift;
    mvn_64(rd: Register64, rm: Register64; shift: Shift<u8>) => MVN_ORN_64_log_shift;
    neg_32(rd: Register32, rm: Register32; shift: Shift<u8>) => NEG_SUB_32_addsub_shift;
    neg_64(rd: Register64, rm: Register64; shift: Shift<u8>) => NEG_SUB_64_addsub_shift;
    negs_32(rd: Register32, rm: Register32; shift: Shift<u8>) => NEGS_SUBS_32_addsub_shift;
    negs_64(rd: Register64, rm: Register64; shift: Shift<u8>) => NEGS_SUBS_64_addsub_shift;
    ngc_32(rd: Register32, rm: Register32) => NGC_SBC_32_addsub_carry;
    ngc_64(rd: Register64, rm: Register64) => NGC_SBC_64_addsub_carry;
    ngcs_32(rd: Register32, rm: Register32) => NGCS_SBCS_32_addsub_carry;
    ngcs_64(rd: Register64, rm: Register64) => NGCS_SBCS_64_addsub_carry;
    nop() => NOP_HI_hints;
    orn_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => ORN_32_log_shift;
    orn_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => ORN_64_log_shift;
    orr_imm_32(rd: Register32OrSp, rn: Register32; imm: Bitmask32) => ORR_32_log_imm;
    orr_imm_64(rd: Register64OrSp, rn: Register64; imm: Bitmask64) => ORR_64_log_imm;
    orr_reg_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => ORR_32_log_shift;
    orr_reg_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => ORR_64_log_shift;
    prfm_lit(imm5: u8, label: Label) => PRFM_P_loadlit {
        rt: RegisterIndex { index: imm5.into() },
        imm19: (label.offset >> 2) as u32 & 0x7_ffff,
    };
    prfm_pos(imm5: u8, rn: Register64OrSp, imm12: u16) => PRFM_P_ldst_pos {
        rt: RegisterIndex { index: imm5.into() },
        rn: rn.into(),
        imm12: imm12.into(),
    };
    prfm_regoff(imm5: u8, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => PRFM_P_ldst_regoff {
        rt: RegisterIndex { index: imm5.into() },
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    prfum(imm5: u8, rn: Register64OrSp, imm9: i16) => PRFUM_P_ldst_unscaled {
        rt: RegisterIndex { index: imm5.into() },
        rn: rn.into(),
        imm9: imm9 as u32 & 0x1ff,
    };
    pssbb() => PSSBB_DSB_BO_barriers;
    rbit_32(rd: Register32, rn: Register32) => RBIT_32_dp_1src;
    rbit_64(rd: Register64, rn: Register64) => RBIT_64_dp_1src;
    ret(rn: Register64) => RET_64R_branch_reg;
    rev_32(rd: Register32, rn: Register32) => REV_32_dp_1src;
    rev_64(rd: Register64, rn: Register64) => REV_64_dp_1src;
    rev16_32(rd: Register32, rn: Register32) => REV16_32_dp_1src;
    rev16_64(rd: Register64, rn: Register64) => REV16_64_dp_1src;
    rev32(rd: Register64, rn: Register64) => REV32_64_dp_1src;
    rev64(rd: Register64, rn: Register64) => REV64_REV_64_dp_1src;
    ror_imm_32(rd: Register32, rs: Register32, shift: u8) => ROR_EXTR_32_extract {
        rd: rd.into(),
        rn: rs.into(),
        rm: rs.into(),
        imms: shift.into(),
    };
    ror_imm_64(rd: Register64, rs: Register64, shift: u8) => ROR_EXTR_64_extract {
        rd: rd.into(),
        rn: rs.into(),
        rm: rs.into(),
        imms: shift.into(),
    };
    ror_reg_32(rd: Register32, rn: Register32, rm: Register32) => ROR_RORV_32_dp_2src;
    ror_reg_64(rd: Register64, rn: Register64, rm: Register64) => ROR_RORV_64_dp_2src;
    rorv_32(rd: Register32, rn: Register32, rm: Register32) => RORV_32_dp_2src;
    rorv_64(rd: Register64, rn: Register64, rm: Register64) => RORV_64_dp_2src;
    sbc_32(rd: Register32, rn: Register32, rm: Register32) => SBC_32_addsub_carry;
    sbc_64(rd: Register64, rn: Register64, rm: Register64) => SBC_64_addsub_carry;
    sbcs_32(rd: Register32, rn: Register32, rm: Register32) => SBCS_32_addsub_carry;
    sbcs_64(rd: Register64, rn: Register64, rm: Register64) => SBCS_64_addsub_carry;
    sbfiz_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => SBFIZ_SBFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (lsb as u32).wrapping_neg() & 31,
        imms: width as u32 - 1,
    };
    sbfiz_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => SBFIZ_SBFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (lsb as u32).wrapping_neg() & 63,
        imms: width as u32 - 1,
    };
    sbfm_32(rd: Register32, rn: Register32, immr: u8, imms: u8) => SBFM_32M_bitfield;
    sbfm_64(rd: Register64, rn: Register64, immr: u8, imms: u8) => SBFM_64M_bitfield;
    sbfx_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => SBFX_SBFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: lsb as u32,
        imms: lsb as u32 + width as u32 - 1,
    };
    sbfx_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => SBFX_SBFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: lsb as u32,
        imms: lsb as u32 + width as u32 - 1,
    };
    sdiv_32(rd: Register32, rn: Register32, rm: Register32) => SDIV_32_dp_2src;
    sdiv_64(rd: Register64, rn: Register64, rm: Register64) => SDIV_64_dp_2src;
    sev() => SEV_HI_hints;
    sevl() => SEVL_HI_hints;
    smaddl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => SMADDL_64WA_dp_3src;
    smc(imm16: u16) => SMC_EX_exception;
    smnegl(rd: Register64, rn: Register32, rm: Register32) => SMNEGL_SMSUBL_64WA_dp_3src;
    smsubl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => SMSUBL_64WA_dp_3src;
    smulh(rd: Register64, rn: Register64, rm: Register64) => SMULH_64_dp_3src;
    smull(rd: Register64, rn: Register32, rm: Register32) => SMULL_SMADDL_64WA_dp_3src;
    ssbb() => SSBB_DSB_BO_barriers;
    stlr_32(rt: Register32, rn: Register64OrSp) => STLR_SL32_ldstord;
    stlr_64(rt: Register64, rn: Register64OrSp) => STLR_SL64_ldstord;
    stlrb(rt: Register32, rn: Register64OrSp) => STLRB_SL32_ldstord;
    stlrh(rt: Register32, rn: Register64OrSp) => STLRH_SL32_ldstord;
    stlxp_32(rs: Register32, rt: Register32, rt2: Register32, rn: Register64OrSp) => STLXP_SP32_ldstexclp;
    stlxp_64(rs: Register32, rt: Register64, rt2: Register64, rn: Register64OrSp) => STLXP_SP64_ldstexclp;
    stlxr_32(rs: Register32, rt: Register32, rn: Register64OrSp) => STLXR_SR32_ldstexclr;
    stlxr_64(rs: Register32, rt: Register64, rn: Register64OrSp) => STLXR_SR64_ldstexclr;
    stlxrb(rs: Register32, rt: Register32, rn: Register64OrSp) => STLXRB_SR32_ldstexclr;
    stlxrh(rs: Register32, rt: Register32, rn: Register64OrSp) => STLXRH_SR32_ldstexclr;
    stnp_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => STNP_32_ldstnapair_offs {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    stnp_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => STNP_64_ldstnapair_offs {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    stp_offset_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => STP_32_ldstpair_off {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    stp_offset_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => STP_64_ldstpair_off {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    stp_post_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => STP_32_ldstpair_post {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    stp_post_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => STP_64_ldstpair_post {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    stp_pre_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => STP_32_ldstpair_pre {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    stp_pre_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => STP_64_ldstpair_pre {
        rt: rt.into(),
        rt2: rt2.into(),
        rn: rn.into(),
        imm7: index.index as u32 & 0x7f,
    };
    str_immpost_32(rt: Register32, rn: Register64OrSp; imm9: i16) => STR_32_ldst_immpost;
    str_immpost_64(rt: Register64, rn: Register64OrSp; imm9: i16) => STR_64_ldst_immpost;
    str_immpre_32(rt: Register32, rn: Register64OrSp; imm9: i16) => STR_32_ldst_immpre;
    str_immpre_64(rt: Register64, rn: Register64OrSp; imm9: i16) => STR_64_ldst_immpre;
    str_pos_32(rt: Register32, rn: Register64OrSp, index: Index<u32>) => STR_32_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    str_pos_64(rt: Register64, rn: Register64OrSp, index: Index<u64>) => STR_64_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    str_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => STR_32_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    str_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => STR_64_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    strb_immpost(rt: Register32, rn: Register64OrSp; imm9: i16) => STRB_32_ldst_immpost;
    strb_immpre(rt: Register32, rn: Register64OrSp; imm9: i16) => STRB_32_ldst_immpre;
    strb_pos(rt: Register32, rn: Register64OrSp, imm12: u16) => STRB_32_ldst_pos;
    strb_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => STRB_32B_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    strh_immpost(rt: Register32, rn: Register64OrSp; imm9: i16) => STRH_32_ldst_immpost;
    strh_immpre(rt: Register32, rn: Register64OrSp; imm9: i16) => STRH_32_ldst_immpre;
    strh_pos(rt: Register32, rn: Register64OrSp, index: Index<u16>) => STRH_32_ldst_pos {
        rt: rt.into(),
        rn: rn.into(),
        imm12: index.index as u32,
    };
    strh_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => STRH_32_ldst_regoff {
        rt: rt.into(),
        rn: rn.into(),
        rm: rm.into(),
        option: extend,
        s: amount,
    };
    sttr_32(rt: Register32, rn: Register64OrSp; imm9: i16) => STTR_32_ldst_unpriv;
    sttr_64(rt: Register64, rn: Register64OrSp; imm9: i16) => STTR_64_ldst_unpriv;
    sttrb(rt: Register32, rn: Register64OrSp; imm9: i16) => STTRB_32_ldst_unpriv;
    sttrh(rt: Register32, rn: Register64OrSp; imm9: i16) => STTRH_32_ldst_unpriv;
    stur_32(rt: Register32, rn: Register64OrSp; imm9: i16) => STUR_32_ldst_unscaled;
    stur_64(rt: Register64, rn: Register64OrSp; imm9: i16) => STUR_64_ldst_unscaled;
    sturb(rt: Register32, rn: Register64OrSp; imm9: i16) => STURB_32_ldst_unscaled;
    sturh(rt: Register32, rn: Register64OrSp; imm9: i16) => STURH_32_ldst_unscaled;
    stxp_32(rs: Register32, rt: Register32, rt2: Register32, rn: Register64OrSp) => STXP_SP32_ldstexclp;
    stxp_64(rs: Register32, rt: Register64, rt2: Register64, rn: Register64OrSp) => STXP_SP64_ldstexclp;
    stxr_32(rs: Register32, rt: Register32, rn: Register64OrSp) => STXR_SR32_ldstexclr;
    stxr_64(rs: Register32, rt: Register64, rn: Register64OrSp) => STXR_SR64_ldstexclr;
    stxrb(rs: Register32, rt: Register32, rn: Register64OrSp) => STXRB_SR32_ldstexclr;
    stxrh(rs: Register32, rt: Register32, rn: Register64OrSp) => STXRH_SR32_ldstexclr;
    sub_ext_32(rd: Register32OrSp, rn: Register32OrSp, rm: Register32; extend: Extend) => SUB_32_addsub_ext;
    sub_ext_64(rd: Register64OrSp, rn: Register64OrSp, rm: Register64; extend: Extend) => SUB_64_addsub_ext;
    sub_imm_32(rd: Register32OrSp, rn: Register32OrSp; imm: impl Into<ShiftedImm12>) => SUB_32_addsub_imm;
    sub_imm_64(rd: Register64OrSp, rn: Register64OrSp; imm: impl Into<ShiftedImm12>) => SUB_64_addsub_imm;
    sub_shift_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => SUB_32_addsub_shift;
    sub_shift_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => SUB_64_addsub_shift;
    subs_ext_32(rd: Register32, rn: Register32OrSp, rm: Register32; extend: Extend) => SUBS_32S_addsub_ext;
    subs_ext_64(rd: Register64, rn: Register64OrSp, rm: Register64; extend: Extend) => SUBS_64S_addsub_ext;
    subs_imm_32(rd: Register32, rn: Register32OrSp; imm: impl Into<ShiftedImm12>) => SUBS_32S_addsub_imm;
    subs_imm_64(rd: Register64, rn: Register64OrSp; imm: impl Into<ShiftedImm12>) => SUBS_64S_addsub_imm;
    subs_shift_32(rd: Register32, rn: Register32, rm: Register32; shift: Shift<u8>) => SUBS_32_addsub_shift;
    subs_shift_64(rd: Register64, rn: Register64, rm: Register64; shift: Shift<u8>) => SUBS_64_addsub_shift;
    svc(imm16: u16) => SVC_EX_exception;
    sxtb_32(rd: Register32, rn: Register32) => SXTB_SBFM_32M_bitfield;
    sxtb_64(rd: Register64, rn: Register32) => SXTB_SBFM_64M_bitfield;
    sxth_32(rd: Register32, rn: Register32) => SXTH_SBFM_32M_bitfield;
    sxth_64(rd: Register64, rn: Register32) => SXTH_SBFM_64M_bitfield;
    sxtw(rd: Register64, rn: Register32) => SXTW_SBFM_64M_bitfield;
    sys(op1: u8, crn: u8, crm: u8, op2: u8, rt: Register64) => SYS_CR_systeminstrs;
    sysl(rt: Register64, op1: u8, crn: u8, crm: u8, op2: u8) => SYSL_RC_systeminstrs;
    tbnz(rt: Register64, imm: u8, label: Label) => TBNZ_only_testbranch {
        rt: rt.into(),
        b5: (imm >> 5).into(),
        b40: (imm & 0x1f).into(),
        imm14: (label.offset >> 2) as u32,
    };
    tbz(rt: Register64, imm: u8, label: Label) => TBZ_only_testbranch {
        rt: rt.into(),
        b5: (imm >> 5).into(),
        b40: (imm & 0x1f).into(),
        imm14: (label.offset >> 2) as u32,
    };
    tlbi(op1: u8, crm: u8, op2: u8, rt: Register64) => TLBI_SYS_CR_systeminstrs;
    tst_imm_32(rn: Register32; imm: Bitmask32) => TST_ANDS_32S_log_imm;
    tst_imm_64(rn: Register64; imm: Bitmask64) => TST_ANDS_64S_log_imm;
    tst_reg_32(rn: Register32, rm: Register32; shift: Shift<u8>) => TST_ANDS_32_log_shift;
    tst_reg_64(rn: Register64, rm: Register64; shift: Shift<u8>) => TST_ANDS_64_log_shift;
    ubfiz_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => UBFIZ_UBFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (lsb as u32).wrapping_neg() & 31,
        imms: width as u32 - 1,
    };
    ubfiz_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => UBFIZ_UBFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: (lsb as u32).wrapping_neg() & 63,
        imms: width as u32 - 1,
    };
    ubfm_32(rd: Register32, rn: Register32, immr: u8, imms: u8) => UBFM_32M_bitfield;
    ubfm_64(rd: Register64, rn: Register64, immr: u8, imms: u8) => UBFM_64M_bitfield;
    ubfx_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => UBFX_UBFM_32M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: lsb as u32,
        imms: lsb as u32 + width as u32 - 1,
    };
    ubfx_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => UBFX_UBFM_64M_bitfield {
        rd: rd.into(),
        rn: rn.into(),
        immr: lsb as u32,
        imms: lsb as u32 + width as u32 - 1,
    };
    udf(imm16: u16) => UDF_only_perm_undef;
    udiv_32(rd: Register32, rn: Register32, rm: Register32) => UDIV_32_dp_2src;
    udiv_64(rd: Register64, rn: Register64, rm: Register64) => UDIV_64_dp_2src;
    umaddl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => UMADDL_64WA_dp_3src;
    umnegl(rd: Register64, rn: Register32, rm: Register32) => UMNEGL_UMSUBL_64WA_dp_3src;
    umsubl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => UMSUBL_64WA_dp_3src;
    umulh(rd: Register64, rn: Register64, rm: Register64) => UMULH_64_dp_3src;
    umull(rd: Register64, rn: Register32, rm: Register32) => UMULL_UMADDL_64WA_dp_3src;
    uxtb(rd: Register32, rn: Register32) => UXTB_UBFM_32M_bitfield;
    uxth(rd: Register32, rn: Register32) => UXTH_UBFM_32M_bitfield;
    wfe() => WFE_HI_hints;
    wfi() => WFI_HI_hints;
    yield_() => YIELD_HI_hints;
}
