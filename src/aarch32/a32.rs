use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    adc_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => ADC_i_A1;
    adc_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => ADC_r_A1;
    adc_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => ADC_r_A1_RRX;
    adc_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => ADC_rr_A1;
    adcs_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => ADCS_i_A1;
    adcs_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => ADCS_r_A1;
    adcs_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => ADCS_r_A1_RRX;
    adcs_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => ADCS_rr_A1;
    add_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => ADD_i_A1;
    add_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => ADD_r_A1;
    add_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => ADD_r_A1_RRX;
    add_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => ADD_rr_A1;
    adds_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => ADDS_i_A1;
    adds_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => ADDS_r_A1;
    adds_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => ADDS_r_A1_RRX;
    adds_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => ADDS_rr_A1;
    adr_add#cond(rd: Register; imm: impl Into<Const>) => ADR_A1;
    adr_sub#cond(rd: Register; imm: impl Into<Const>) => ADR_A2;
    and_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => AND_i_A1;
    and_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => AND_r_A1;
    and_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => AND_r_A1_RRX;
    and_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => AND_rr_A1;
    ands_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => ANDS_i_A1;
    ands_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => ANDS_r_A1;
    ands_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => ANDS_r_A1_RRX;
    ands_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => ANDS_rr_A1;
    asr_imm#cond(rd: Register, rm: Register, imm5: u8) => ASR_MOV_r_A1;
    asr_reg#cond(rd: Register, rm: Register, rs: Register) => ASR_MOV_rr_A1;
    asrs_imm#cond(rd: Register, rm: Register, imm5: u8) => ASRS_MOVS_r_A1;
    asrs_reg#cond(rd: Register, rm: Register, rs: Register) => ASRS_MOVS_rr_A1;
    b#cond(label: Label) => B_A1 {
        imm24: ((label.offset - 8) >> 2) as u32 & 0xff_ffff,
    };
    bfc#cond(rd: Register, lsb: u8, width: u8) => BFC_A1 {
        rd: rd.into(),
        lsb: lsb.into(),
        msb: (lsb + width - 1).into(),
    };
    bfi#cond(rd: Register, rn: Register, lsb: u8, width: u8) => BFI_A1 {
        rd: rd.into(),
        rn: rn.into(),
        lsb: lsb.into(),
        msb: (lsb + width - 1).into(),
    };
    bic_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => BIC_i_A1;
    bic_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => BIC_r_A1;
    bic_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => BIC_r_A1_RRX;
    bic_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => BIC_rr_A1;
    bics_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => BICS_i_A1;
    bics_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => BICS_r_A1;
    bics_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => BICS_r_A1_RRX;
    bics_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => BICS_rr_A1;
    bkpt(imm: u16) => BKPT_A1 {
        cond: Condition::Al,
        imm12: (imm >> 4).into(),
        imm4: (imm & 0xf).into(),
    };
    bl#cond(label: Label) => BL_i_A1 {
        imm24: ((label.offset - 8) >> 2) as u32 & 0xff_ffff,
    };
    blx_imm(label: Label) => BL_i_A2 {
        imm24: ((label.offset - 8) >> 2) as u32 & 0xff_ffff,
        h: (((label.offset - 8) >> 1) & 1) as u32,
    };
    blx_reg#cond(rm: Register) => BLX_r_A1;
    bx#cond(rm: Register) => BX_A1;
    bxj#cond(rm: Register) => BXJ_A1;
    clrex() => CLREX_A1;
    clz#cond(rd: Register, rm: Register) => CLZ_A1;
    cmn_imm#cond(rn: Register; imm: impl Into<Const>) => CMN_i_A1;
    cmn_reg#cond(rn: Register, rm: Register; shift: Shift<u8>) => CMN_r_A1;
    cmn_reg_rrx#cond(rn: Register, rm: Register) => CMN_r_A1_RRX;
    cmn_reg_shifted_reg#cond(rn: Register, rm: Register; shift: Shift<Register>) => CMN_rr_A1;
    cmp_imm#cond(rn: Register; imm: impl Into<Const>) => CMP_i_A1;
    cmp_reg#cond(rn: Register, rm: Register; shift: Shift<u8>) => CMP_r_A1;
    cmp_reg_rrx#cond(rn: Register, rm: Register) => CMP_r_A1_RRX;
    cmp_reg_shifted_reg#cond(rn: Register, rm: Register; shift: Shift<Register>) => CMP_rr_A1;
    cps(mode: u8) => CPS_A1_AS {
        a: 0,
        i: 0,
        f: 0,
        mode: mode.into(),
    };
    cpsid(a: bool, i: bool, f: bool) => CPSID_A1_AS {
        a: a.into(),
        i: i.into(),
        f: f.into(),
        mode: 0,
    };
    cpsid_mode(a: bool, i: bool, f: bool, mode: u8) => CPSID_A1_ASM {
        a: a.into(),
        i: i.into(),
        f: f.into(),
        mode: mode.into(),
    };
    cpsie(a: bool, i: bool, f: bool) => CPSIE_A1_AS {
        a: a.into(),
        i: i.into(),
        f: f.into(),
        mode: 0,
    };
    cpsie_mode(a: bool, i: bool, f: bool, mode: u8) => CPSIE_A1_ASM {
        a: a.into(),
        i: i.into(),
        f: f.into(),
        mode: mode.into(),
    };
    csdb#cond() => CSDB_A1;
    dbg#cond(option: u8) => DBG_A1;
    dmb(option: u8) => DMB_A1;
    dsb(option: u8) => DSB_A1;
    eor_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => EOR_i_A1;
    eor_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => EOR_r_A1;
    eor_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => EOR_r_A1_RRX;
    eor_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => EOR_rr_A1;
    eors_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => EORS_i_A1;
    eors_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => EORS_r_A1;
    eors_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => EORS_r_A1_RRX;
    eors_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => EORS_rr_A1;
    eret#cond() => ERET_A1;
    hvc(imm: u16) => HVC_A1 {
        cond: Condition::Al,
        imm12: (imm >> 4).into(),
        imm4: (imm & 0xf).into(),
    };
    isb(option: u8) => ISB_A1;
    ldc_imm_offset#cond(rn: Register, offset: Offset<u8>) => LDC_i_A1_off {
        rn: rn.into(),
        u: offset.is_add().into(),
        imm8: offset.abs_offset().into(),
    };
    ldc_imm_post#cond(rn: Register, offset: Offset<u8>) => LDC_i_A1_post {
        rn: rn.into(),
        u: offset.is_add().into(),
        imm8: offset.abs_offset().into(),
    };
    ldc_imm_pre#cond(rn: Register, offset: Offset<u8>) => LDC_i_A1_pre {
        rn: rn.into(),
        u: offset.is_add().into(),
        imm8: offset.abs_offset().into(),
    };
    ldc_imm_unindexed#cond(rn: Register, imm8: u8) => LDC_i_A1_unind;
    ldc_lit_offset#cond(offset: Offset<u8>) => LDC_l_A1 {
        p: 1,
        u: offset.is_add().into(),
        w: 0,
        imm8: offset.abs_offset().into(),
    };
    ldc_lit_unindexed#cond(imm8: u8) => LDC_l_A1 {
        p: 0,
        u: 1,
        w: 0,
        imm8: imm8.into(),
    };
    ldm#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => LDM_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    ldm_exception_return#cond<Rn, L>(amode: AddressMode, rn: Rn, regs: L)
        where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => LDM_e_A1_AS {
        rn: rn.reg.into(),
        p: amode.is_before().into(),
        u: amode.is_increment().into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits() & 0x7fff,
    };
    ldm_user_registers#cond<L>(amode: AddressMode, rn: Register, regs: L)
        where L: Into<RegisterList>
    => LDM_u_A1_AS {
        rn: rn.into(),
        p: amode.is_before().into(),
        u: amode.is_increment().into(),
        register_list: regs.into().to_bits(),
    };
    ldmda#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => LDMDA_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    ldmdb#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => LDMDB_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    ldmib#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => LDMIB_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    ldr_imm_offset#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDR_i_A1_off;
    ldr_imm_post#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDR_i_A1_post;
    ldr_imm_pre#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDR_i_A1_pre;
    ldr_lit#cond(rt: Register, offset12: Offset<u16>) => LDR_l_A1 {
        rt: rt.into(),
        p: 1,
        u: offset12.is_add().into(),
        w: 0,
        imm12: offset12.abs_offset().into(),
    };
    ldr_reg_offset#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDR_r_A1_off;
    ldr_reg_post#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDR_r_A1_post;
    ldr_reg_pre#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDR_r_A1_pre;
    ldrb_imm_offset#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDRB_i_A1_off;
    ldrb_imm_post#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDRB_i_A1_post;
    ldrb_imm_pre#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDRB_i_A1_pre;
    ldrb_lit#cond(rt: Register, offset12: Offset<u16>) => LDRB_l_A1 {
        rt: rt.into(),
        p: 1,
        u: offset12.is_add().into(),
        w: 0,
        imm12: offset12.abs_offset().into(),
    };
    ldrb_reg_offset#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDRB_r_A1_off;
    ldrb_reg_post#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDRB_r_A1_post;
    ldrb_reg_pre#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDRB_r_A1_pre;
    ldrbt_imm#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDRBT_A1;
    ldrbt_reg#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDRBT_A2;
    ldrd_imm_offset#cond(rt: RegisterPair, rn: Register; offset: Offset<u8>) => LDRD_i_A1_off;
    ldrd_imm_post#cond(rt: RegisterPair, rn: Register; offset: Offset<u8>) => LDRD_i_A1_post;
    ldrd_imm_pre#cond(rt: RegisterPair, rn: Register; offset: Offset<u8>) => LDRD_i_A1_pre;
    ldrd_lit#cond(rt: RegisterPair; offset: Offset<u8>) => LDRD_l_A1;
    ldrd_reg_offset#cond(rt: RegisterPair, rn: Register; offset: Offset<Register>) => LDRD_r_A1_off;
    ldrd_reg_post#cond(rt: RegisterPair, rn: Register; offset: Offset<Register>) => LDRD_r_A1_post;
    ldrd_reg_pre#cond(rt: RegisterPair, rn: Register; offset: Offset<Register>) => LDRD_r_A1_pre;
    ldrex#cond(rt: Register, rn: Register) => LDREX_A1;
    ldrexb#cond(rt: Register, rn: Register) => LDREXB_A1;
    ldrexd#cond(rt: RegisterPair, rn: Register) => LDREXD_A1;
    ldrexh#cond(rt: Register, rn: Register) => LDREXH_A1;
    ldrh_imm_offset#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRH_i_A1_off;
    ldrh_imm_post#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRH_i_A1_post;
    ldrh_imm_pre#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRH_i_A1_pre;
    ldrh_lit#cond(rt: Register, offset: Offset<u8>) => LDRH_l_A1 {
        rt: rt.into(),
        p: 1,
        u: offset.is_add().into(),
        w: 0,
        imm4h: (offset.abs_offset() >> 4).into(),
        imm4l: (offset.abs_offset() & 0xf).into(),
    };
    ldrh_reg_offset#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRH_r_A1_off;
    ldrh_reg_post#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRH_r_A1_post;
    ldrh_reg_pre#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRH_r_A1_pre;
    ldrht_imm#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRHT_A1;
    ldrht_reg#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRHT_A2;
    ldrsb_imm_offset#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSB_i_A1_off;
    ldrsb_imm_post#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSB_i_A1_post;
    ldrsb_imm_pre#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSB_i_A1_pre;
    ldrsb_lit#cond(rt: Register, offset: Offset<u8>) => LDRSB_l_A1 {
        rt: rt.into(),
        p: 1,
        u: offset.is_add().into(),
        w: 0,
        imm4h: (offset.abs_offset() >> 4).into(),
        imm4l: (offset.abs_offset() & 0xf).into(),
    };
    ldrsb_reg_offset#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSB_r_A1_off;
    ldrsb_reg_post#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSB_r_A1_post;
    ldrsb_reg_pre#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSB_r_A1_pre;
    ldrsbt_imm#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSBT_A1;
    ldrsbt_reg#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSBT_A2;
    ldrsh_imm_offset#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSH_i_A1_off;
    ldrsh_imm_post#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSH_i_A1_post;
    ldrsh_imm_pre#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSH_i_A1_pre;
    ldrsh_lit#cond(rt: Register, offset: Offset<u8>) => LDRSH_l_A1 {
        rt: rt.into(),
        p: 1,
        u: offset.is_add().into(),
        w: 0,
        imm4h: (offset.abs_offset() >> 4).into(),
        imm4l: (offset.abs_offset() & 0xf).into(),
    };
    ldrsh_reg_offset#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSH_r_A1_off;
    ldrsh_reg_post#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSH_r_A1_post;
    ldrsh_reg_pre#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSH_r_A1_pre;
    ldrsht_imm#cond(rt: Register, rn: Register; offset: Offset<u8>) => LDRSHT_A1;
    ldrsht_reg#cond(rt: Register, rn: Register; offset: Offset<Register>) => LDRSHT_A2;
    ldrt_imm#cond(rt: Register, rn: Register; offset12: Offset<u16>) => LDRT_A1;
    ldrt_reg#cond(rt: Register, rn: Register; index: impl Into<Index>) => LDRT_A2;
    lsl_imm#cond(rd: Register, rm: Register, imm5: u8) => LSL_MOV_r_A1;
    lsl_reg#cond(rd: Register, rm: Register, rs: Register) => LSL_MOV_rr_A1;
    lsls_imm#cond(rd: Register, rm: Register, imm5: u8) => LSLS_MOVS_r_A1;
    lsls_reg#cond(rd: Register, rm: Register, rs: Register) => LSLS_MOVS_rr_A1;
    lsr_imm#cond(rd: Register, rm: Register, imm5: u8) => LSR_MOV_r_A1;
    lsr_reg#cond(rd: Register, rm: Register, rs: Register) => LSR_MOV_rr_A1;
    lsrs_imm#cond(rd: Register, rm: Register, imm5: u8) => LSRS_MOVS_r_A1;
    lsrs_reg#cond(rd: Register, rm: Register, rs: Register) => LSRS_MOVS_rr_A1;
    mcr#cond(coproc: u8, opc1: u8, rt: Register, crn: u8, crm: u8, opc2: u8) => MCR_A1;
    mcrr#cond(coproc: u8, opc1: u8, rt: Register, rt2: Register, crm: u8) => MCRR_A1;
    mla#cond(rd: Register, rn: Register, rm: Register, ra: Register) => MLA_A1;
    mlas#cond(rd: Register, rn: Register, rm: Register, ra: Register) => MLAS_A1;
    mls#cond(rd: Register, rn: Register, rm: Register, ra: Register) => MLS_A1;
    mov_imm#cond(rd: Register; imm: impl Into<Const>) => MOV_i_A1;
    mov_reg#cond(rd: Register, rm: Register; shift: Shift<u8>) => MOV_r_A1;
    mov_reg_rrx#cond(rd: Register, rm: Register) => MOV_r_A1_RRX;
    mov_reg_shifted_reg#cond(rd: Register, rm: Register; shift: Shift<Register>) => MOV_rr_A1;
    movs_imm#cond(rd: Register; imm: impl Into<Const>) => MOVS_i_A1;
    movs_reg#cond(rd: Register, rm: Register; shift: Shift<u8>) => MOVS_r_A1;
    movs_reg_rrx#cond(rd: Register, rm: Register) => MOVS_r_A1_RRX;
    movs_reg_shifted_reg#cond(rd: Register, rm: Register; shift: Shift<Register>) => MOVS_rr_A1;
    movt#cond(rd: Register, imm16: u16) => MOVT_A1 {
        rd: rd.into(),
        imm4: (imm16 >> 12).into(),
        imm12: (imm16 & 0xfff).into(),
    };
    movw#cond(rd: Register, imm16: u16) => MOV_i_A2 {
        rd: rd.into(),
        imm4: (imm16 >> 12).into(),
        imm12: (imm16 & 0xfff).into(),
    };
    mrc#cond(coproc: u8, opc1: u8, rt: Register, crn: u8, crm: u8, opc2: u8) => MRC_A1;
    mrrc#cond(coproc: u8, opc1: u8, rt: Register, rt2: Register, crm: u8) => MRRC_A1;
    mrs#cond(rd: Register, special_register: u8) => MRS_A1_AS {
        rd: rd.into(),
        r: special_register.into(),
    };
    mrs_banked#cond(rd: Register, banked_register: BankedRegister) => MRS_br_A1_AS {
        rd: rd.into(),
        r: banked_register.accesses_spsr().into(),
        m: ((banked_register as u32) >> 4) & 1,
        m1: (banked_register as u32) & 0xf,
    };
    msr_banked#cond(banked_register: BankedRegister, rn: Register) => MSR_br_A1_AS {
        r: banked_register.accesses_spsr().into(),
        m: ((banked_register as u32) >> 4) & 1,
        m1: (banked_register as u32) & 0xf,
        rn: rn.into(),
    };
    msr_imm#cond<C>(special_register: u8, mask: u8, imm: C)
        where C: Into<Const>
    => MSR_i_A1_AS {
        r: special_register.into(),
        mask: mask.into(),
        imm12: imm.into().to_bits(),
    };
    msr_reg#cond(special_register: u8, mask: u8, rn: Register) => MSR_r_A1_AS {
        r: special_register.into(),
        mask: mask.into(),
        rn: rn.into(),
    };
    mul#cond(rd: Register, rn: Register, rm: Register) => MUL_A1;
    muls#cond(rd: Register, rn: Register, rm: Register) => MULS_A1;
    mvn_imm#cond(rd: Register; imm: impl Into<Const>) => MVN_i_A1;
    mvn_reg#cond(rd: Register, rm: Register; shift: Shift<u8>) => MVN_r_A1;
    mvn_reg_rrx#cond(rd: Register, rm: Register) => MVN_r_A1_RRX;
    mvn_reg_shifted_reg#cond(rd: Register, rm: Register; shift: Shift<Register>) => MVN_rr_A1;
    mvns_imm#cond(rd: Register; imm: impl Into<Const>) => MVNS_i_A1;
    mvns_reg#cond(rd: Register, rm: Register; shift: Shift<u8>) => MVNS_r_A1;
    mvns_reg_rrx#cond(rd: Register, rm: Register) => MVNS_r_A1_RRX;
    mvns_reg_shifted_reg#cond(rd: Register, rm: Register; shift: Shift<Register>) => MVNS_rr_A1;
    nop#cond() => NOP_A1;
    orr_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => ORR_i_A1;
    orr_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => ORR_r_A1;
    orr_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => ORR_r_A1_RRX;
    orr_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => ORR_rr_A1;
    orrs_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => ORRS_i_A1;
    orrs_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => ORRS_r_A1;
    orrs_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => ORRS_r_A1_RRX;
    orrs_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => ORRS_rr_A1;
    pkhbt#cond(rd: Register, rn: Register, rm: Register, imm5: u8) => PKHBT_A1;
    pkhtb#cond(rd: Register, rn: Register, rm: Register, imm5: u8) => PKHTB_A1;
    pld_imm(rn: Register; offset12: Offset<u16>) => PLD_i_A1;
    pld_lit(; offset12: Offset<u16>) => PLD_l_A1;
    pld_reg(rn: Register; index: impl Into<Index>) => PLD_r_A1;
    pld_reg_rrx(rn: Register, u: u32, rm: Register) => PLD_r_A1_RRX;
    pldw_imm(rn: Register; offset12: Offset<u16>) => PLDW_i_A1;
    pldw_reg(rn: Register; index: impl Into<Index>) => PLDW_r_A1;
    pldw_reg_rrx(rn: Register, u: u32, rm: Register) => PLDW_r_A1_RRX;
    pli_imm(rn: Register; offset12: Offset<u16>) => PLI_i_A1;
    pli_reg(rn: Register; index: impl Into<Index>) => PLI_r_A1;
    pli_reg_rrx(rn: Register, u: u32, rm: Register) => PLI_r_A1_RRX;
    pop_list#cond(; regs: impl Into<RegisterList>) => POP_LDM_A1;
    pop_reg#cond(rt: Register) => POP_LDR_i_A1_post;
    push_list#cond(; regs: impl Into<RegisterList>) => PUSH_STMDB_A1;
    push_reg#cond(rt: Register) => PUSH_STR_i_A1_pre;
    qadd#cond(rd: Register, rm: Register, rn: Register) => QADD_A1;
    qadd8#cond(rd: Register, rn: Register, rm: Register) => QADD8_A1;
    qadd16#cond(rd: Register, rn: Register, rm: Register) => QADD16_A1;
    qasx#cond(rd: Register, rn: Register, rm: Register) => QASX_A1;
    qdadd#cond(rd: Register, rm: Register, rn: Register) => QDADD_A1;
    qdsub#cond(rd: Register, rm: Register, rn: Register) => QDSUB_A1;
    qsax#cond(rd: Register, rn: Register, rm: Register) => QSAX_A1;
    qsub#cond(rd: Register, rm: Register, rn: Register) => QSUB_A1;
    qsub8#cond(rd: Register, rn: Register, rm: Register) => QSUB8_A1;
    qsub16#cond(rd: Register, rn: Register, rm: Register) => QSUB16_A1;
    rbit#cond(rd: Register, rm: Register) => RBIT_A1;
    rev#cond(rd: Register, rm: Register) => REV_A1;
    rev16#cond(rd: Register, rm: Register) => REV16_A1;
    revsh#cond(rd: Register, rm: Register) => REVSH_A1;
    rfeda(rn: Register) => RFEDA_A1_AS {
        w: 0,
        rn: rn.into(),
    };
    rfedb(rn: Register) => RFEDB_A1_AS {
        w: 0,
        rn: rn.into(),
    };
    rfeia(rn: Register) => RFEIA_A1_AS {
        w: 0,
        rn: rn.into(),
    };
    rfeib(rn: Register) => RFEIB_A1_AS {
        w: 0,
        rn: rn.into(),
    };
    ror_imm#cond(rd: Register, rm: Register, imm5: u8) => ROR_MOV_r_A1;
    ror_reg#cond(rd: Register, rm: Register, rs: Register) => ROR_MOV_rr_A1;
    rors_imm#cond(rd: Register, rm: Register, imm5: u8) => RORS_MOVS_r_A1;
    rors_reg#cond(rd: Register, rm: Register, rs: Register) => RORS_MOVS_rr_A1;
    rrx#cond(rd: Register, rm: Register) => RRX_MOV_r_A1_RRX;
    rrxs#cond(rd: Register, rm: Register) => RRXS_MOVS_r_A1_RRX;
    rsb_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => RSB_i_A1;
    rsb_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => RSB_r_A1;
    rsb_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => RSB_r_A1_RRX;
    rsb_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => RSB_rr_A1;
    rsbs_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => RSBS_i_A1;
    rsbs_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => RSBS_r_A1;
    rsbs_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => RSBS_r_A1_RRX;
    rsbs_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => RSBS_rr_A1;
    rsc_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => RSC_i_A1;
    rsc_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => RSC_r_A1;
    rsc_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => RSC_r_A1_RRX;
    rsc_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => RSC_rr_A1;
    rscs_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => RSCS_i_A1;
    rscs_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => RSCS_r_A1;
    rscs_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => RSCS_r_A1_RRX;
    rscs_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => RSCS_rr_A1;
    sadd8#cond(rd: Register, rn: Register, rm: Register) => SADD8_A1;
    sadd16#cond(rd: Register, rn: Register, rm: Register) => SADD16_A1;
    sasx#cond(rd: Register, rn: Register, rm: Register) => SASX_A1;
    sb() => SB_A1;
    sbc_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => SBC_i_A1;
    sbc_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => SBC_r_A1;
    sbc_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => SBC_r_A1_RRX;
    sbc_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => SBC_rr_A1;
    sbcs_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => SBCS_i_A1;
    sbcs_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => SBCS_r_A1;
    sbcs_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => SBCS_r_A1_RRX;
    sbcs_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => SBCS_rr_A1;
    sbfx#cond(rd: Register, rn: Register, lsb: u8, width: u8) => SBFX_A1 {
        widthm1: (width - 1).into(),
        rd: rd.into(),
        lsb: lsb.into(),
        rn: rn.into(),
    };
    sdiv#cond(rd: Register, rn: Register, rm: Register) => SDIV_A1;
    sel#cond(rd: Register, rn: Register, rm: Register) => SEL_A1;
    setend(endianness: Endianness) => SETEND_A1 { e: endianness as _ };
    sev#cond() => SEV_A1;
    shadd8#cond(rd: Register, rn: Register, rm: Register) => SHADD8_A1;
    shadd16#cond(rd: Register, rn: Register, rm: Register) => SHADD16_A1;
    shasx#cond(rd: Register, rn: Register, rm: Register) => SHASX_A1;
    shsax#cond(rd: Register, rn: Register, rm: Register) => SHSAX_A1;
    shsub8#cond(rd: Register, rn: Register, rm: Register) => SHSUB8_A1;
    shsub16#cond(rd: Register, rn: Register, rm: Register) => SHSUB16_A1;
    smc#cond(imm4: u8) => SMC_A1_AS;
    smlabb#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLABB_A1;
    smlabt#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLABT_A1;
    smlad#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLAD_A1;
    smladx#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLADX_A1;
    smlal#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLAL_A1;
    smlalbb#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLALBB_A1;
    smlalbt#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLALBT_A1;
    smlald#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLALD_A1;
    smlaldx#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLALDX_A1;
    smlals#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLALS_A1;
    smlaltb#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLALTB_A1;
    smlaltt#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLALTT_A1;
    smlatb#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLATB_A1;
    smlatt#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLATT_A1;
    smlawb#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLAWB_A1;
    smlawt#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLAWT_A1;
    smlsd#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLSD_A1;
    smlsdx#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMLSDX_A1;
    smlsld#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLSLD_A1;
    smlsldx#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMLSLDX_A1;
    smmla#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMMLA_A1;
    smmlar#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMMLAR_A1;
    smmls#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMMLS_A1;
    smmlsr#cond(rd: Register, rn: Register, rm: Register, ra: Register) => SMMLSR_A1;
    smmul#cond(rd: Register, rn: Register, rm: Register) => SMMUL_A1;
    smmulr#cond(rd: Register, rn: Register, rm: Register) => SMMULR_A1;
    smuad#cond(rd: Register, rn: Register, rm: Register) => SMUAD_A1;
    smuadx#cond(rd: Register, rn: Register, rm: Register) => SMUADX_A1;
    smulbb#cond(rd: Register, rn: Register, rm: Register) => SMULBB_A1;
    smulbt#cond(rd: Register, rn: Register, rm: Register) => SMULBT_A1;
    smull#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMULL_A1;
    smulls#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => SMULLS_A1;
    smultb#cond(rd: Register, rn: Register, rm: Register) => SMULTB_A1;
    smultt#cond(rd: Register, rn: Register, rm: Register) => SMULTT_A1;
    smulwb#cond(rd: Register, rn: Register, rm: Register) => SMULWB_A1;
    smulwt#cond(rd: Register, rn: Register, rm: Register) => SMULWT_A1;
    smusd#cond(rd: Register, rn: Register, rm: Register) => SMUSD_A1;
    smusdx#cond(rd: Register, rn: Register, rm: Register) => SMUSDX_A1;
    srsda(writeback: bool, mode: u8) => SRSDA_A1_AS {
        w: writeback.into(),
        mode: mode.into(),
    };
    srsdb(writeback: bool, mode: u8) => SRSDB_A1_AS {
        w: writeback.into(),
        mode: mode.into(),
    };
    srsia(writeback: bool, mode: u8) => SRSIA_A1_AS {
        w: writeback.into(),
        mode: mode.into(),
    };
    srsib(writeback: bool, mode: u8) => SRSIB_A1_AS {
        w: writeback.into(),
        mode: mode.into(),
    };
    ssat_asr#cond(rd: Register, saturate_to: u8, rn: Register, amount: u8) => SSAT_A1_ASR {
        sat_imm: (saturate_to - 1).into(),
        rd: rd.into(),
        imm5: amount.into(),
        rn: rn.into(),
    };
    ssat_lsl#cond(rd: Register, saturate_to: u8, rn: Register, amount: u8) => SSAT_A1_LSL {
        sat_imm: (saturate_to - 1).into(),
        rd: rd.into(),
        imm5: amount.into(),
        rn: rn.into(),
    };
    ssat16#cond(rd: Register, saturate_to: u8, rn: Register) => SSAT16_A1 {
        sat_imm: (saturate_to - 1).into(),
        rd: rd.into(),
        rn: rn.into(),
    };
    ssax#cond(rd: Register, rn: Register, rm: Register) => SSAX_A1;
    ssub8#cond(rd: Register, rn: Register, rm: Register) => SSUB8_A1;
    ssub16#cond(rd: Register, rn: Register, rm: Register) => SSUB16_A1;
    stc_imm_offset#cond(rn: Register, offset: Offset<u8>) => STC_A1_off {
        rn: rn.into(),
        u: offset.is_add().into(),
        imm8: offset.abs_offset().into(),
    };
    stc_imm_post#cond(rn: Register, offset: Offset<u8>) => STC_A1_post {
        rn: rn.into(),
        u: offset.is_add().into(),
        imm8: offset.abs_offset().into(),
    };
    stc_imm_pre#cond(rn: Register, offset: Offset<u8>) => STC_A1_pre {
        rn: rn.into(),
        u: offset.is_add().into(),
        imm8: offset.abs_offset().into(),
    };
    stc_imm_unindexed#cond(rn: Register, imm8: u8) => STC_A1_unind;
    stm#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => STM_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    stm_user_registers#cond<L>(rn: Register, regs: L)
        where L: Into<RegisterList>
    => STM_u_A1_AS {
        rn: rn.into(),
        p: 0,
        u: 1,
        register_list: regs.into().to_bits(),
    };
    stmda#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => STMDA_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    stmda_user_registers#cond<L>(rn: Register, regs: L)
        where L: Into<RegisterList>
    => STM_u_A1_AS {
        rn: rn.into(),
        p: 0,
        u: 0,
        register_list: regs.into().to_bits(),
    };
    stmdb#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => STMDB_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    stmdb_user_registers#cond<L>(rn: Register, regs: L)
        where L: Into<RegisterList>
    => STM_u_A1_AS {
        rn: rn.into(),
        p: 1,
        u: 0,
        register_list: regs.into().to_bits(),
    };
    stmib#cond<Rn, L>(rn: Rn, regs: L) where Rn: Into<Mem>, L: Into<RegisterList>
    ; let rn = rn.into() => STMIB_A1 {
        rn: rn.reg.into(),
        w: rn.writeback.into(),
        register_list: regs.into().to_bits(),
    };
    stmib_user_registers#cond<L>(rn: Register, regs: L)
        where L: Into<RegisterList>
    => STM_u_A1_AS {
        rn: rn.into(),
        p: 1,
        u: 1,
        register_list: regs.into().to_bits(),
    };
    str_imm_offset#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STR_i_A1_off;
    str_imm_post#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STR_i_A1_post;
    str_imm_pre#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STR_i_A1_pre;
    str_reg_offset#cond(rt: Register, rn: Register; index: impl Into<Index>) => STR_r_A1_off;
    str_reg_post#cond(rt: Register, rn: Register; index: impl Into<Index>) => STR_r_A1_post;
    str_reg_pre#cond(rt: Register, rn: Register; index: impl Into<Index>) => STR_r_A1_pre;
    strb_imm_offset#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STRB_i_A1_off;
    strb_imm_post#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STRB_i_A1_post;
    strb_imm_pre#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STRB_i_A1_pre;
    strb_reg_offset#cond(rt: Register, rn: Register; index: impl Into<Index>) => STRB_r_A1_off;
    strb_reg_post#cond(rt: Register, rn: Register; index: impl Into<Index>) => STRB_r_A1_post;
    strb_reg_pre#cond(rt: Register, rn: Register; index: impl Into<Index>) => STRB_r_A1_pre;
    strbt_imm_post#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STRBT_A1;
    strbt_reg_post#cond(rt: Register, rn: Register; index: impl Into<Index>) => STRBT_A2;
    strd_imm_offset#cond(rt: RegisterPair, rn: Register; offset: Offset<u8>) => STRD_i_A1_off;
    strd_imm_post#cond(rt: RegisterPair, rn: Register; offset: Offset<u8>) => STRD_i_A1_post;
    strd_imm_pre#cond(rt: RegisterPair, rn: Register; offset: Offset<u8>) => STRD_i_A1_pre;
    strd_reg_offset#cond(rt: RegisterPair, rn: Register; offset: Offset<Register>) => STRD_r_A1_off;
    strd_reg_post#cond(rt: RegisterPair, rn: Register; offset: Offset<Register>) => STRD_r_A1_post;
    strd_reg_pre#cond(rt: RegisterPair, rn: Register; offset: Offset<Register>) => STRD_r_A1_pre;
    strex#cond(rd: Register, rt: Register, rn: Register) => STREX_A1;
    strexb#cond(rd: Register, rt: Register, rn: Register) => STREXB_A1;
    strexd#cond(rd: Register, rt: RegisterPair, rn: Register) => STREXD_A1;
    strexh#cond(rd: Register, rt: Register, rn: Register) => STREXH_A1;
    strh_imm_offset#cond(rt: Register, rn: Register; offset: Offset<u8>) => STRH_i_A1_off;
    strh_imm_post#cond(rt: Register, rn: Register; offset: Offset<u8>) => STRH_i_A1_post;
    strh_imm_pre#cond(rt: Register, rn: Register; offset: Offset<u8>) => STRH_i_A1_pre;
    strh_reg_offset#cond(rt: Register, rn: Register; offset: Offset<Register>) => STRH_r_A1_off;
    strh_reg_post#cond(rt: Register, rn: Register; offset: Offset<Register>) => STRH_r_A1_post;
    strh_reg_pre#cond(rt: Register, rn: Register; offset: Offset<Register>) => STRH_r_A1_pre;
    strht_imm#cond(rt: Register, rn: Register; offset: Offset<u8>) => STRHT_A1;
    strht_reg#cond(rt: Register, rn: Register; offset: Offset<Register>) => STRHT_A2;
    strt_imm#cond(rt: Register, rn: Register; offset12: Offset<u16>) => STRT_A1;
    strt_reg#cond(rt: Register, rn: Register; index: impl Into<Index>) => STRT_A2;
    sub_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => SUB_i_A1;
    sub_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => SUB_r_A1;
    sub_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => SUB_r_A1_RRX;
    sub_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => SUB_rr_A1;
    subs_imm#cond(rd: Register, rn: Register; imm: impl Into<Const>) => SUBS_i_A1;
    subs_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<u8>) => SUBS_r_A1;
    subs_reg_rrx#cond(rd: Register, rn: Register, rm: Register) => SUBS_r_A1_RRX;
    subs_reg_shifted_reg#cond(rd: Register, rn: Register, rm: Register; shift: Shift<Register>) => SUBS_rr_A1;
    svc#cond(imm24: u32) => SVC_A1;
    sxtab#cond(rd: Register, rn: Register, rm: Register; ror: Ror) => SXTAB_A1;
    sxtab16#cond(rd: Register, rn: Register, rm: Register; ror: Ror) => SXTAB16_A1;
    sxtah#cond(rd: Register, rn: Register, rm: Register; ror: Ror) => SXTAH_A1;
    sxtb#cond(rd: Register, rm: Register; ror: Ror) => SXTB_A1;
    sxtb16#cond(rd: Register, rm: Register; ror: Ror) => SXTB16_A1;
    sxth#cond(rd: Register, rm: Register; ror: Ror) => SXTH_A1;
    teq_imm#cond(rn: Register; imm: impl Into<Const>) => TEQ_i_A1;
    teq_reg#cond(rn: Register, rm: Register; shift: Shift<u8>) => TEQ_r_A1;
    teq_reg_rrx#cond(rn: Register, rm: Register) => TEQ_r_A1_RRX;
    teq_reg_shifted_reg#cond(rn: Register, rm: Register; shift: Shift<Register>) => TEQ_rr_A1;
    tst_imm#cond(rn: Register; imm: impl Into<Const>) => TST_i_A1;
    tst_reg#cond(rn: Register, rm: Register; shift: Shift<u8>) => TST_r_A1;
    tst_reg_rrx#cond(rn: Register, rm: Register) => TST_r_A1_RRX;
    tst_reg_shifted_reg#cond(rn: Register, rm: Register; shift: Shift<Register>) => TST_rr_A1;
    uadd8#cond(rd: Register, rn: Register, rm: Register) => UADD8_A1;
    uadd16#cond(rd: Register, rn: Register, rm: Register) => UADD16_A1;
    uasx#cond(rd: Register, rn: Register, rm: Register) => UASX_A1;
    ubfx#cond(rd: Register, rn: Register, lsb: u8, width: u8) => UBFX_A1 {
        widthm1: (width - 1).into(),
        rd: rd.into(),
        lsb: lsb.into(),
        rn: rn.into(),
    };
    udf(imm16: u16) => UDF_A1 {
        imm12: (imm16 >> 4).into(),
        imm4: (imm16 & 0xf).into(),
    };
    udiv#cond(rd: Register, rn: Register, rm: Register) => UDIV_A1;
    uhadd8#cond(rd: Register, rn: Register, rm: Register) => UHADD8_A1;
    uhadd16#cond(rd: Register, rn: Register, rm: Register) => UHADD16_A1;
    uhasx#cond(rd: Register, rn: Register, rm: Register) => UHASX_A1;
    uhsax#cond(rd: Register, rn: Register, rm: Register) => UHSAX_A1;
    uhsub8#cond(rd: Register, rn: Register, rm: Register) => UHSUB8_A1;
    uhsub16#cond(rd: Register, rn: Register, rm: Register) => UHSUB16_A1;
    umaal#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => UMAAL_A1;
    umlal#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => UMLAL_A1;
    umlals#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => UMLALS_A1;
    umull#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => UMULL_A1;
    umulls#cond(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => UMULLS_A1;
    uqadd8#cond(rd: Register, rn: Register, rm: Register) => UQADD8_A1;
    uqadd16#cond(rd: Register, rn: Register, rm: Register) => UQADD16_A1;
    uqasx#cond(rd: Register, rn: Register, rm: Register) => UQASX_A1;
    uqsax#cond(rd: Register, rn: Register, rm: Register) => UQSAX_A1;
    uqsub8#cond(rd: Register, rn: Register, rm: Register) => UQSUB8_A1;
    uqsub16#cond(rd: Register, rn: Register, rm: Register) => UQSUB16_A1;
    usad8#cond(rd: Register, rn: Register, rm: Register) => USAD8_A1;
    usada8#cond(rd: Register, rn: Register, rm: Register, ra: Register) => USADA8_A1;
    usat_asr#cond(rd: Register, sat_imm: u8, rn: Register, imm5: u8) => USAT_A1_ASR;
    usat_lsl#cond(rd: Register, sat_imm: u8, rn: Register, imm5: u8) => USAT_A1_LSL;
    usat16#cond(rd: Register, sat_imm: u8, rn: Register) => USAT16_A1;
    usax#cond(rd: Register, rn: Register, rm: Register) => USAX_A1;
    usub8#cond(rd: Register, rn: Register, rm: Register) => USUB8_A1;
    usub16#cond(rd: Register, rn: Register, rm: Register) => USUB16_A1;
    uxtab#cond(rd: Register, rn: Register, rm: Register; ror: Ror) => UXTAB_A1;
    uxtab16#cond(rd: Register, rn: Register, rm: Register; ror: Ror) => UXTAB16_A1;
    uxtah#cond(rd: Register, rn: Register, rm: Register; ror: Ror) => UXTAH_A1;
    uxtb#cond(rd: Register, rm: Register; ror: Ror) => UXTB_A1;
    uxtb16#cond(rd: Register, rm: Register; ror: Ror) => UXTB16_A1;
    uxth#cond(rd: Register, rm: Register; ror: Ror) => UXTH_A1;
    wfe#cond() => WFE_A1;
    wfi#cond() => WFI_A1;
    yield_#cond() => YIELD_A1;
}
