use mitte_core::EmitSlice;

use crate::encoding::RegisterIndex;

use super::encoding;
use super::types::*;
use super::fixup::FixupKind;

use super::macros::{forward, functions};


pub trait Emit: EmitSlice {
    fn emit_b_label<Label>(&mut self, label: &mut Label) -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel26,
            |e, offset| {
                e.emit_b(self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_b_cond_label<Label>(&mut self, cond: Condition, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel19,
            |e, offset| {
                e.emit_b_cond(cond, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_bl_label<Label>(&mut self, label: &mut Label) -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel26,
            |e, offset| {
                e.emit_bl(self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_cbnz_label_32<Label>(&mut self, rt: Register32, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel19,
            |e, offset| {
                e.emit_cbnz_32(rt, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_cbnz_label_64<Label>(&mut self, rt: Register64, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel19,
            |e, offset| {
                e.emit_cbnz_64(rt, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_cbz_label_32<Label>(&mut self, rt: Register32, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel19,
            |e, offset| {
                e.emit_cbz_32(rt, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_cbz_label_64<Label>(&mut self, rt: Register64, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel19,
            |e, offset| {
                e.emit_cbz_64(rt, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_ldr_label_32<Label>(&mut self, rt: Register32, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel19,
            |e, offset| {
                e.emit_ldr_lit_32(rt, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_ldr_label_64<Label>(&mut self, rt: Register64, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel19,
            |e, offset| {
                e.emit_ldr_lit_64(rt, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_tbnz_label<Label>(&mut self, rt: Register64, imm: u8, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel14,
            |e, offset| {
                e.emit_tbnz(rt, imm, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_tbz_label<Label>(&mut self, rt: Register64, imm: u8, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::PcRel14,
            |e, offset| {
                e.emit_tbz(rt, imm, self::Label::from_byte_offset(offset))
            },
        )
    }

    forward! {
        emit_adc_32(rd: Register32, rn: Register32, rm: Register32) => adc_32;
        emit_adc_64(rd: Register64, rn: Register64, rm: Register64) => adc_64;
        emit_adcs_32(rd: Register32, rn: Register32, rm: Register32) => adcs_32;
        emit_adcs_64(rd: Register64, rn: Register64, rm: Register64) => adcs_64;
        emit_add_ext_32(rd: Register32OrSp, rn: Register32OrSp, rm: Register32, extend: Extend) => add_ext_32;
        emit_add_ext_64(rd: Register64OrSp, rn: Register64OrSp, rm: Register64, extend: Extend) => add_ext_64;
        emit_add_imm_32(rd: Register32OrSp, rn: Register32OrSp, imm: impl Into<ShiftedImm12>) => add_imm_32;
        emit_add_imm_64(rd: Register64OrSp, rn: Register64OrSp, imm: impl Into<ShiftedImm12>) => add_imm_64;
        emit_add_shift_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => add_shift_32;
        emit_add_shift_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => add_shift_64;
        emit_adds_ext_32(rd: Register32, rn: Register32OrSp, rm: Register32, extend: Extend) => adds_ext_32;
        emit_adds_ext_64(rd: Register64, rn: Register64OrSp, rm: Register64, extend: Extend) => adds_ext_64;
        emit_adds_imm_32(rd: Register32, rn: Register32OrSp, imm: impl Into<ShiftedImm12>) => adds_imm_32;
        emit_adds_imm_64(rd: Register64, rn: Register64OrSp, imm: impl Into<ShiftedImm12>) => adds_imm_64;
        emit_adds_shift_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => adds_shift_32;
        emit_adds_shift_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => adds_shift_64;
        emit_adr(rd: Register64, label: Label) => adr;
        emit_adrp(rd: Register64, label: Label) => adrp;
        emit_and_imm_32(rd: Register32OrSp, rn: Register32, imm: Bitmask32) => and_imm_32;
        emit_and_imm_64(rd: Register64OrSp, rn: Register64, imm: Bitmask64) => and_imm_64;
        emit_and_reg_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => and_reg_32;
        emit_and_reg_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => and_reg_64;
        emit_ands_imm_32(rd: Register32, rn: Register32, imm: Bitmask32) => ands_imm_32;
        emit_ands_imm_64(rd: Register64, rn: Register64, imm: Bitmask64) => ands_imm_64;
        emit_ands_reg_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => ands_reg_32;
        emit_ands_reg_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => ands_reg_64;
        emit_asr_imm_32(rd: Register32, rn: Register32, shift: u8) => asr_imm_32;
        emit_asr_imm_64(rd: Register64, rn: Register64, shift: u8) => asr_imm_64;
        emit_asr_reg_32(rd: Register32, rn: Register32, rm: Register32) => asr_reg_32;
        emit_asr_reg_64(rd: Register64, rn: Register64, rm: Register64) => asr_reg_64;
        emit_asrv_32(rd: Register32, rn: Register32, rm: Register32) => asrv_32;
        emit_asrv_64(rd: Register64, rn: Register64, rm: Register64) => asrv_64;
        emit_at(op1: u8, crm: u8, op2: u8, rt: Register64) => at;
        emit_b(label: Label) => b;
        emit_b_cond(cond: Condition, label: Label) => b_cond;
        emit_bfc_32(rd: Register32, lsb: u8, width: u8) => bfc_32;
        emit_bfc_64(rd: Register64, lsb: u8, width: u8) => bfc_64;
        emit_bfi_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => bfi_32;
        emit_bfi_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => bfi_64;
        emit_bfm_32(rd: Register32, rn: Register32, immr: u8, imms: u8) => bfm_32;
        emit_bfm_64(rd: Register64, rn: Register64, immr: u8, imms: u8) => bfm_64;
        emit_bfxil_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => bfxil_32;
        emit_bfxil_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => bfxil_64;
        emit_bic_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => bic_32;
        emit_bic_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => bic_64;
        emit_bics_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => bics_32;
        emit_bics_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => bics_64;
        emit_bl(label: Label) => bl;
        emit_blr(rn: Register64) => blr;
        emit_br(rn: Register64) => br;
        emit_brk(imm16: u16) => brk;
        emit_cbnz_32(rt: Register32, label: Label) => cbnz_32;
        emit_cbnz_64(rt: Register64, label: Label) => cbnz_64;
        emit_cbz_32(rt: Register32, label: Label) => cbz_32;
        emit_cbz_64(rt: Register64, label: Label) => cbz_64;
        emit_ccmn_imm_32(rn: Register32, imm5: u8, nzcv: u8, cond: Condition) => ccmn_imm_32;
        emit_ccmn_imm_64(rn: Register64, imm5: u8, nzcv: u8, cond: Condition) => ccmn_imm_64;
        emit_ccmn_reg_32(rn: Register32, rm: Register32, nzcv: u8, cond: Condition) => ccmn_reg_32;
        emit_ccmn_reg_64(rn: Register64, rm: Register64, nzcv: u8, cond: Condition) => ccmn_reg_64;
        emit_ccmp_imm_32(rn: Register32, imm5: u8, nzcv: u8, cond: Condition) => ccmp_imm_32;
        emit_ccmp_imm_64(rn: Register64, imm5: u8, nzcv: u8, cond: Condition) => ccmp_imm_64;
        emit_ccmp_reg_32(rn: Register32, rm: Register32, nzcv: u8, cond: Condition) => ccmp_reg_32;
        emit_ccmp_reg_64(rn: Register64, rm: Register64, nzcv: u8, cond: Condition) => ccmp_reg_64;
        emit_cinc_32(rd: Register32, rn: Register32, cond: Condition) => cinc_32;
        emit_cinc_64(rd: Register64, rn: Register64, cond: Condition) => cinc_64;
        emit_cinv_32(rd: Register32, rn: Register32, cond: Condition) => cinv_32;
        emit_cinv_64(rd: Register64, rn: Register64, cond: Condition) => cinv_64;
        emit_clrex(crm: u8) => clrex;
        emit_cls_32(rd: Register32, rn: Register32) => cls_32;
        emit_cls_64(rd: Register64, rn: Register64) => cls_64;
        emit_clz_32(rd: Register32, rn: Register32) => clz_32;
        emit_clz_64(rd: Register64, rn: Register64) => clz_64;
        emit_cmn_ext_32(rn: Register32OrSp, rm: Register32, extend: Extend) => cmn_ext_32;
        emit_cmn_ext_64(rn: Register64OrSp, rm: Register64, extend: Extend) => cmn_ext_64;
        emit_cmn_imm_32(rn: Register32OrSp, imm: impl Into<ShiftedImm12>) => cmn_imm_32;
        emit_cmn_imm_64(rn: Register64OrSp, imm: impl Into<ShiftedImm12>) => cmn_imm_64;
        emit_cmn_shift_32(rn: Register32, rm: Register32, shift: Shift<u8>) => cmn_shift_32;
        emit_cmn_shift_64(rn: Register64, rm: Register64, shift: Shift<u8>) => cmn_shift_64;
        emit_cmp_ext_32(rn: Register32OrSp, rm: Register32, extend: Extend) => cmp_ext_32;
        emit_cmp_ext_64(rn: Register64OrSp, rm: Register64, extend: Extend) => cmp_ext_64;
        emit_cmp_imm_32(rn: Register32OrSp, imm: impl Into<ShiftedImm12>) => cmp_imm_32;
        emit_cmp_imm_64(rn: Register64OrSp, imm: impl Into<ShiftedImm12>) => cmp_imm_64;
        emit_cmp_shift_32(rn: Register32, rm: Register32, shift: Shift<u8>) => cmp_shift_32;
        emit_cmp_shift_64(rn: Register64, rm: Register64, shift: Shift<u8>) => cmp_shift_64;
        emit_cneg_32(rd: Register32, rn: Register32, cond: Condition) => cneg_32;
        emit_cneg_64(rd: Register64, rn: Register64, cond: Condition) => cneg_64;
        emit_csdb() => csdb;
        emit_csel_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => csel_32;
        emit_csel_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => csel_64;
        emit_cset_32(rd: Register32, cond: Condition) => cset_32;
        emit_cset_64(rd: Register64, cond: Condition) => cset_64;
        emit_csetm_32(rd: Register32, cond: Condition) => csetm_32;
        emit_csetm_64(rd: Register64, cond: Condition) => csetm_64;
        emit_csinc_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => csinc_32;
        emit_csinc_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => csinc_64;
        emit_csinv_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => csinv_32;
        emit_csinv_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => csinv_64;
        emit_csneg_32(rd: Register32, rn: Register32, rm: Register32, cond: Condition) => csneg_32;
        emit_csneg_64(rd: Register64, rn: Register64, rm: Register64, cond: Condition) => csneg_64;
        emit_dc(op1: u8, crm: u8, op2: u8, rt: Register64) => dc;
        emit_dcps1(imm16: u16) => dcps1;
        emit_dcps2(imm16: u16) => dcps2;
        emit_dcps3(imm16: u16) => dcps3;
        emit_dmb(crm: u8) => dmb;
        emit_drps() => drps;
        emit_dsb(crm: u8) => dsb;
        emit_eon_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => eon_32;
        emit_eon_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => eon_64;
        emit_eor_imm_32(rd: Register32OrSp, rn: Register32, imm: Bitmask32) => eor_imm_32;
        emit_eor_imm_64(rd: Register64OrSp, rn: Register64, imm: Bitmask64) => eor_imm_64;
        emit_eor_reg_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => eor_reg_32;
        emit_eor_reg_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => eor_reg_64;
        emit_eret() => eret;
        emit_extr_32(rd: Register32, rn: Register32, rm: Register32, lsb: u8) => extr_32;
        emit_extr_64(rd: Register64, rn: Register64, rm: Register64, lsb: u8) => extr_64;
        emit_hint(imm: u8) => hint;
        emit_hlt(imm16: u16) => hlt;
        emit_hvc(imm16: u16) => hvc;
        emit_ic(op1: u8, crm: u8, op2: u8, rt: Register64) => ic;
        emit_isb(crm: u8) => isb;
        emit_ldar_32(rt: Register32, rn: Register64OrSp) => ldar_32;
        emit_ldar_64(rt: Register64, rn: Register64OrSp) => ldar_64;
        emit_ldarb(rt: Register32, rn: Register64OrSp) => ldarb;
        emit_ldarh(rt: Register32, rn: Register64OrSp) => ldarh;
        emit_ldaxp_32(rt: Register32, rt2: Register32, rn: Register64OrSp) => ldaxp_32;
        emit_ldaxp_64(rt: Register64, rt2: Register64, rn: Register64OrSp) => ldaxp_64;
        emit_ldaxr_32(rt: Register32, rn: Register64OrSp) => ldaxr_32;
        emit_ldaxr_64(rt: Register64, rn: Register64OrSp) => ldaxr_64;
        emit_ldaxrb(rt: Register32, rn: Register64OrSp) => ldaxrb;
        emit_ldaxrh(rt: Register32, rn: Register64OrSp) => ldaxrh;
        emit_ldnp_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => ldnp_32;
        emit_ldnp_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => ldnp_64;
        emit_ldp_offset_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => ldp_offset_32;
        emit_ldp_offset_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => ldp_offset_64;
        emit_ldp_post_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => ldp_post_32;
        emit_ldp_post_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => ldp_post_64;
        emit_ldp_pre_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => ldp_pre_32;
        emit_ldp_pre_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => ldp_pre_64;
        emit_ldpsw_offset(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u32>) => ldpsw_offset;
        emit_ldpsw_post(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u32>) => ldpsw_post;
        emit_ldpsw_pre(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u32>) => ldpsw_pre;
        emit_ldr_immpost_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldr_immpost_32;
        emit_ldr_immpost_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldr_immpost_64;
        emit_ldr_immpre_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldr_immpre_32;
        emit_ldr_immpre_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldr_immpre_64;
        emit_ldr_lit_32(rt: Register32, label: Label) => ldr_lit_32;
        emit_ldr_lit_64(rt: Register64, label: Label) => ldr_lit_64;
        emit_ldr_pos_32(rt: Register32, rn: Register64OrSp, index: Index<u32>) => ldr_pos_32;
        emit_ldr_pos_64(rt: Register64, rn: Register64OrSp, index: Index<u64>) => ldr_pos_64;
        emit_ldr_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldr_regoff_32;
        emit_ldr_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldr_regoff_64;
        emit_ldrb_immpost(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrb_immpost;
        emit_ldrb_immpre(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrb_immpre;
        emit_ldrb_pos(rt: Register32, rn: Register64OrSp, imm12: u16) => ldrb_pos;
        emit_ldrb_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldrb_regoff;
        emit_ldrh_immpost(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrh_immpost;
        emit_ldrh_immpre(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrh_immpre;
        emit_ldrh_pos(rt: Register32, rn: Register64OrSp, index: Index<u16>) => ldrh_pos;
        emit_ldrh_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldrh_regoff;
        emit_ldrsb_immpost_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrsb_immpost_32;
        emit_ldrsb_immpost_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldrsb_immpost_64;
        emit_ldrsb_immpre_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrsb_immpre_32;
        emit_ldrsb_immpre_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldrsb_immpre_64;
        emit_ldrsb_pos_32(rt: Register32, rn: Register64OrSp, imm12: u16) => ldrsb_pos_32;
        emit_ldrsb_pos_64(rt: Register64, rn: Register64OrSp, imm12: u16) => ldrsb_pos_64;
        emit_ldrsb_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldrsb_regoff_32;
        emit_ldrsb_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldrsb_regoff_64;
        emit_ldrsh_immpost_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrsh_immpost_32;
        emit_ldrsh_immpost_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldrsh_immpost_64;
        emit_ldrsh_immpre_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldrsh_immpre_32;
        emit_ldrsh_immpre_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldrsh_immpre_64;
        emit_ldrsh_pos_32(rt: Register32, rn: Register64OrSp, index: Index<u16>) => ldrsh_pos_32;
        emit_ldrsh_pos_64(rt: Register64, rn: Register64OrSp, index: Index<u16>) => ldrsh_pos_64;
        emit_ldrsh_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldrsh_regoff_32;
        emit_ldrsh_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldrsh_regoff_64;
        emit_ldrsw_immpost(rt: Register64, rn: Register64OrSp, imm9: i16) => ldrsw_immpost;
        emit_ldrsw_immpre(rt: Register64, rn: Register64OrSp, imm9: i16) => ldrsw_immpre;
        emit_ldrsw_lit(rt: Register64, label: Label) => ldrsw_lit;
        emit_ldrsw_pos(rt: Register64, rn: Register64OrSp, index: Index<u32>) => ldrsw_pos;
        emit_ldrsw_regoff(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => ldrsw_regoff;
        emit_ldtr_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldtr_32;
        emit_ldtr_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldtr_64;
        emit_ldtrb(rt: Register32, rn: Register64OrSp, imm9: i16) => ldtrb;
        emit_ldtrh(rt: Register32, rn: Register64OrSp, imm9: i16) => ldtrh;
        emit_ldtrsb_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldtrsb_32;
        emit_ldtrsb_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldtrsb_64;
        emit_ldtrsh_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldtrsh_32;
        emit_ldtrsh_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldtrsh_64;
        emit_ldtrsw(rt: Register64, rn: Register64OrSp, imm9: i16) => ldtrsw;
        emit_ldur_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldur_32;
        emit_ldur_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldur_64;
        emit_ldurb(rt: Register32, rn: Register64OrSp, imm9: i16) => ldurb;
        emit_ldurh(rt: Register32, rn: Register64OrSp, imm9: i16) => ldurh;
        emit_ldursb_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldursb_32;
        emit_ldursb_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldursb_64;
        emit_ldursh_32(rt: Register32, rn: Register64OrSp, imm9: i16) => ldursh_32;
        emit_ldursh_64(rt: Register64, rn: Register64OrSp, imm9: i16) => ldursh_64;
        emit_ldursw(rt: Register64, rn: Register64OrSp, imm9: i16) => ldursw;
        emit_ldxp_32(rt: Register32, rt2: Register32, rn: Register64OrSp) => ldxp_32;
        emit_ldxp_64(rt: Register64, rt2: Register64, rn: Register64OrSp) => ldxp_64;
        emit_ldxr_32(rt: Register32, rn: Register64OrSp) => ldxr_32;
        emit_ldxr_64(rt: Register64, rn: Register64OrSp) => ldxr_64;
        emit_ldxrb(rt: Register32, rn: Register64OrSp) => ldxrb;
        emit_ldxrh(rt: Register32, rn: Register64OrSp) => ldxrh;
        emit_lsl_imm_32(rd: Register32, rn: Register32, shift: u8) => lsl_imm_32;
        emit_lsl_imm_64(rd: Register64, rn: Register64, shift: u8) => lsl_imm_64;
        emit_lsl_reg_32(rd: Register32, rn: Register32, rm: Register32) => lsl_reg_32;
        emit_lsl_reg_64(rd: Register64, rn: Register64, rm: Register64) => lsl_reg_64;
        emit_lslv_32(rd: Register32, rn: Register32, rm: Register32) => lslv_32;
        emit_lslv_64(rd: Register64, rn: Register64, rm: Register64) => lslv_64;
        emit_lsr_imm_32(rd: Register32, rn: Register32, shift: u8) => lsr_imm_32;
        emit_lsr_imm_64(rd: Register64, rn: Register64, shift: u8) => lsr_imm_64;
        emit_lsr_reg_32(rd: Register32, rn: Register32, rm: Register32) => lsr_reg_32;
        emit_lsr_reg_64(rd: Register64, rn: Register64, rm: Register64) => lsr_reg_64;
        emit_lsrv_32(rd: Register32, rn: Register32, rm: Register32) => lsrv_32;
        emit_lsrv_64(rd: Register64, rn: Register64, rm: Register64) => lsrv_64;
        emit_madd_32(rd: Register32, rn: Register32, rm: Register32, ra: Register32) => madd_32;
        emit_madd_64(rd: Register64, rn: Register64, rm: Register64, ra: Register64) => madd_64;
        emit_mneg_32(rd: Register32, rn: Register32, rm: Register32) => mneg_32;
        emit_mneg_64(rd: Register64, rn: Register64, rm: Register64) => mneg_64;
        emit_mov_bitmask_32(rd: Register32OrSp, imm: Bitmask32) => mov_bitmask_32;
        emit_mov_bitmask_64(rd: Register64OrSp, imm: Bitmask64) => mov_bitmask_64;
        emit_mov_reg_32(rd: Register32, rm: Register32) => mov_reg_32;
        emit_mov_reg_64(rd: Register64, rm: Register64) => mov_reg_64;
        emit_mov_sp_32(rd: Register32OrSp, rn: Register32OrSp) => mov_sp_32;
        emit_mov_sp_64(rd: Register64OrSp, rn: Register64OrSp) => mov_sp_64;
        emit_movk_32(rd: Register32, imm: impl Into<MovImm32>) => movk_32;
        emit_movk_64(rd: Register64, imm: impl Into<MovImm64>) => movk_64;
        emit_movn_32(rd: Register32, imm: impl Into<MovImm32>) => movn_32;
        emit_movn_64(rd: Register64, imm: impl Into<MovImm64>) => movn_64;
        emit_movz_32(rd: Register32, imm: impl Into<MovImm32>) => movz_32;
        emit_movz_64(rd: Register64, imm: impl Into<MovImm64>) => movz_64;
        emit_mrs(rt: Register64, o0: u8, op1: u8, crn: u8, crm: u8, op2: u8) => mrs;
        emit_msr_imm(op1: u8, op2: u8, imm: u8) => msr_imm;
        emit_msr_reg(o0: u8, op1: u8, crn: u8, crm: u8, op2: u8, rt: Register64) => msr_reg;
        emit_msub_32(rd: Register32, rn: Register32, rm: Register32, ra: Register32) => msub_32;
        emit_msub_64(rd: Register64, rn: Register64, rm: Register64, ra: Register64) => msub_64;
        emit_mul_32(rd: Register32, rn: Register32, rm: Register32) => mul_32;
        emit_mul_64(rd: Register64, rn: Register64, rm: Register64) => mul_64;
        emit_mvn_32(rd: Register32, rm: Register32, shift: Shift<u8>) => mvn_32;
        emit_mvn_64(rd: Register64, rm: Register64, shift: Shift<u8>) => mvn_64;
        emit_neg_32(rd: Register32, rm: Register32, shift: Shift<u8>) => neg_32;
        emit_neg_64(rd: Register64, rm: Register64, shift: Shift<u8>) => neg_64;
        emit_negs_32(rd: Register32, rm: Register32, shift: Shift<u8>) => negs_32;
        emit_negs_64(rd: Register64, rm: Register64, shift: Shift<u8>) => negs_64;
        emit_ngc_32(rd: Register32, rm: Register32) => ngc_32;
        emit_ngc_64(rd: Register64, rm: Register64) => ngc_64;
        emit_ngcs_32(rd: Register32, rm: Register32) => ngcs_32;
        emit_ngcs_64(rd: Register64, rm: Register64) => ngcs_64;
        emit_nop() => nop;
        emit_orn_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => orn_32;
        emit_orn_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => orn_64;
        emit_orr_imm_32(rd: Register32OrSp, rn: Register32, imm: Bitmask32) => orr_imm_32;
        emit_orr_imm_64(rd: Register64OrSp, rn: Register64, imm: Bitmask64) => orr_imm_64;
        emit_orr_reg_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => orr_reg_32;
        emit_orr_reg_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => orr_reg_64;
        emit_prfm_lit(imm5: u8, label: Label) => prfm_lit;
        emit_prfm_pos(imm5: u8, rn: Register64OrSp, imm12: u16) => prfm_pos;
        emit_prfm_regoff(imm5: u8, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => prfm_regoff;
        emit_prfum(imm5: u8, rn: Register64OrSp, imm9: i16) => prfum;
        emit_pssbb() => pssbb;
        emit_rbit_32(rd: Register32, rn: Register32) => rbit_32;
        emit_rbit_64(rd: Register64, rn: Register64) => rbit_64;
        emit_ret(rn: Register64) => ret;
        emit_rev_32(rd: Register32, rn: Register32) => rev_32;
        emit_rev_64(rd: Register64, rn: Register64) => rev_64;
        emit_rev16_32(rd: Register32, rn: Register32) => rev16_32;
        emit_rev16_64(rd: Register64, rn: Register64) => rev16_64;
        emit_rev32(rd: Register64, rn: Register64) => rev32;
        emit_rev64(rd: Register64, rn: Register64) => rev64;
        emit_ror_imm_32(rd: Register32, rs: Register32, shift: u8) => ror_imm_32;
        emit_ror_imm_64(rd: Register64, rs: Register64, shift: u8) => ror_imm_64;
        emit_ror_reg_32(rd: Register32, rn: Register32, rm: Register32) => ror_reg_32;
        emit_ror_reg_64(rd: Register64, rn: Register64, rm: Register64) => ror_reg_64;
        emit_rorv_32(rd: Register32, rn: Register32, rm: Register32) => rorv_32;
        emit_rorv_64(rd: Register64, rn: Register64, rm: Register64) => rorv_64;
        emit_sbc_32(rd: Register32, rn: Register32, rm: Register32) => sbc_32;
        emit_sbc_64(rd: Register64, rn: Register64, rm: Register64) => sbc_64;
        emit_sbcs_32(rd: Register32, rn: Register32, rm: Register32) => sbcs_32;
        emit_sbcs_64(rd: Register64, rn: Register64, rm: Register64) => sbcs_64;
        emit_sbfiz_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => sbfiz_32;
        emit_sbfiz_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => sbfiz_64;
        emit_sbfm_32(rd: Register32, rn: Register32, immr: u8, imms: u8) => sbfm_32;
        emit_sbfm_64(rd: Register64, rn: Register64, immr: u8, imms: u8) => sbfm_64;
        emit_sbfx_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => sbfx_32;
        emit_sbfx_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => sbfx_64;
        emit_sdiv_32(rd: Register32, rn: Register32, rm: Register32) => sdiv_32;
        emit_sdiv_64(rd: Register64, rn: Register64, rm: Register64) => sdiv_64;
        emit_sev() => sev;
        emit_sevl() => sevl;
        emit_smaddl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => smaddl;
        emit_smc(imm16: u16) => smc;
        emit_smnegl(rd: Register64, rn: Register32, rm: Register32) => smnegl;
        emit_smsubl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => smsubl;
        emit_smulh(rd: Register64, rn: Register64, rm: Register64) => smulh;
        emit_smull(rd: Register64, rn: Register32, rm: Register32) => smull;
        emit_ssbb() => ssbb;
        emit_stlr_32(rt: Register32, rn: Register64OrSp) => stlr_32;
        emit_stlr_64(rt: Register64, rn: Register64OrSp) => stlr_64;
        emit_stlrb(rt: Register32, rn: Register64OrSp) => stlrb;
        emit_stlrh(rt: Register32, rn: Register64OrSp) => stlrh;
        emit_stlxp_32(rs: Register32, rt: Register32, rt2: Register32, rn: Register64OrSp) => stlxp_32;
        emit_stlxp_64(rs: Register32, rt: Register64, rt2: Register64, rn: Register64OrSp) => stlxp_64;
        emit_stlxr_32(rs: Register32, rt: Register32, rn: Register64OrSp) => stlxr_32;
        emit_stlxr_64(rs: Register32, rt: Register64, rn: Register64OrSp) => stlxr_64;
        emit_stlxrb(rs: Register32, rt: Register32, rn: Register64OrSp) => stlxrb;
        emit_stlxrh(rs: Register32, rt: Register32, rn: Register64OrSp) => stlxrh;
        emit_stnp_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => stnp_32;
        emit_stnp_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => stnp_64;
        emit_stp_offset_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => stp_offset_32;
        emit_stp_offset_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => stp_offset_64;
        emit_stp_post_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => stp_post_32;
        emit_stp_post_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => stp_post_64;
        emit_stp_pre_32(rt: Register32, rt2: Register32, rn: Register64OrSp, index: Index<u32>) => stp_pre_32;
        emit_stp_pre_64(rt: Register64, rt2: Register64, rn: Register64OrSp, index: Index<u64>) => stp_pre_64;
        emit_str_immpost_32(rt: Register32, rn: Register64OrSp, imm9: i16) => str_immpost_32;
        emit_str_immpost_64(rt: Register64, rn: Register64OrSp, imm9: i16) => str_immpost_64;
        emit_str_immpre_32(rt: Register32, rn: Register64OrSp, imm9: i16) => str_immpre_32;
        emit_str_immpre_64(rt: Register64, rn: Register64OrSp, imm9: i16) => str_immpre_64;
        emit_str_pos_32(rt: Register32, rn: Register64OrSp, index: Index<u32>) => str_pos_32;
        emit_str_pos_64(rt: Register64, rn: Register64OrSp, index: Index<u64>) => str_pos_64;
        emit_str_regoff_32(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => str_regoff_32;
        emit_str_regoff_64(rt: Register64, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => str_regoff_64;
        emit_strb_immpost(rt: Register32, rn: Register64OrSp, imm9: i16) => strb_immpost;
        emit_strb_immpre(rt: Register32, rn: Register64OrSp, imm9: i16) => strb_immpre;
        emit_strb_pos(rt: Register32, rn: Register64OrSp, imm12: u16) => strb_pos;
        emit_strb_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => strb_regoff;
        emit_strh_immpost(rt: Register32, rn: Register64OrSp, imm9: i16) => strh_immpost;
        emit_strh_immpre(rt: Register32, rn: Register64OrSp, imm9: i16) => strh_immpre;
        emit_strh_pos(rt: Register32, rn: Register64OrSp, index: Index<u16>) => strh_pos;
        emit_strh_regoff(rt: Register32, rn: Register64OrSp, rm: Register64, extend: u32, amount: u32) => strh_regoff;
        emit_sttr_32(rt: Register32, rn: Register64OrSp, imm9: i16) => sttr_32;
        emit_sttr_64(rt: Register64, rn: Register64OrSp, imm9: i16) => sttr_64;
        emit_sttrb(rt: Register32, rn: Register64OrSp, imm9: i16) => sttrb;
        emit_sttrh(rt: Register32, rn: Register64OrSp, imm9: i16) => sttrh;
        emit_stur_32(rt: Register32, rn: Register64OrSp, imm9: i16) => stur_32;
        emit_stur_64(rt: Register64, rn: Register64OrSp, imm9: i16) => stur_64;
        emit_sturb(rt: Register32, rn: Register64OrSp, imm9: i16) => sturb;
        emit_sturh(rt: Register32, rn: Register64OrSp, imm9: i16) => sturh;
        emit_stxp_32(rs: Register32, rt: Register32, rt2: Register32, rn: Register64OrSp) => stxp_32;
        emit_stxp_64(rs: Register32, rt: Register64, rt2: Register64, rn: Register64OrSp) => stxp_64;
        emit_stxr_32(rs: Register32, rt: Register32, rn: Register64OrSp) => stxr_32;
        emit_stxr_64(rs: Register32, rt: Register64, rn: Register64OrSp) => stxr_64;
        emit_stxrb(rs: Register32, rt: Register32, rn: Register64OrSp) => stxrb;
        emit_stxrh(rs: Register32, rt: Register32, rn: Register64OrSp) => stxrh;
        emit_sub_ext_32(rd: Register32OrSp, rn: Register32OrSp, rm: Register32, extend: Extend) => sub_ext_32;
        emit_sub_ext_64(rd: Register64OrSp, rn: Register64OrSp, rm: Register64, extend: Extend) => sub_ext_64;
        emit_sub_imm_32(rd: Register32OrSp, rn: Register32OrSp, imm: impl Into<ShiftedImm12>) => sub_imm_32;
        emit_sub_imm_64(rd: Register64OrSp, rn: Register64OrSp, imm: impl Into<ShiftedImm12>) => sub_imm_64;
        emit_sub_shift_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => sub_shift_32;
        emit_sub_shift_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => sub_shift_64;
        emit_subs_ext_32(rd: Register32, rn: Register32OrSp, rm: Register32, extend: Extend) => subs_ext_32;
        emit_subs_ext_64(rd: Register64, rn: Register64OrSp, rm: Register64, extend: Extend) => subs_ext_64;
        emit_subs_imm_32(rd: Register32, rn: Register32OrSp, imm: impl Into<ShiftedImm12>) => subs_imm_32;
        emit_subs_imm_64(rd: Register64, rn: Register64OrSp, imm: impl Into<ShiftedImm12>) => subs_imm_64;
        emit_subs_shift_32(rd: Register32, rn: Register32, rm: Register32, shift: Shift<u8>) => subs_shift_32;
        emit_subs_shift_64(rd: Register64, rn: Register64, rm: Register64, shift: Shift<u8>) => subs_shift_64;
        emit_svc(imm16: u16) => svc;
        emit_sxtb_32(rd: Register32, rn: Register32) => sxtb_32;
        emit_sxtb_64(rd: Register64, rn: Register32) => sxtb_64;
        emit_sxth_32(rd: Register32, rn: Register32) => sxth_32;
        emit_sxth_64(rd: Register64, rn: Register32) => sxth_64;
        emit_sxtw(rd: Register64, rn: Register32) => sxtw;
        emit_sys(op1: u8, crn: u8, crm: u8, op2: u8, rt: Register64) => sys;
        emit_sysl(rt: Register64, op1: u8, crn: u8, crm: u8, op2: u8) => sysl;
        emit_tbnz(rt: Register64, imm: u8, label: Label) => tbnz;
        emit_tbz(rt: Register64, imm: u8, label: Label) => tbz;
        emit_tlbi(op1: u8, crm: u8, op2: u8, rt: Register64) => tlbi;
        emit_tst_imm_32(rn: Register32, imm: Bitmask32) => tst_imm_32;
        emit_tst_imm_64(rn: Register64, imm: Bitmask64) => tst_imm_64;
        emit_tst_reg_32(rn: Register32, rm: Register32, shift: Shift<u8>) => tst_reg_32;
        emit_tst_reg_64(rn: Register64, rm: Register64, shift: Shift<u8>) => tst_reg_64;
        emit_ubfiz_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => ubfiz_32;
        emit_ubfiz_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => ubfiz_64;
        emit_ubfm_32(rd: Register32, rn: Register32, immr: u8, imms: u8) => ubfm_32;
        emit_ubfm_64(rd: Register64, rn: Register64, immr: u8, imms: u8) => ubfm_64;
        emit_ubfx_32(rd: Register32, rn: Register32, lsb: u8, width: u8) => ubfx_32;
        emit_ubfx_64(rd: Register64, rn: Register64, lsb: u8, width: u8) => ubfx_64;
        emit_udf(imm16: u16) => udf;
        emit_udiv_32(rd: Register32, rn: Register32, rm: Register32) => udiv_32;
        emit_udiv_64(rd: Register64, rn: Register64, rm: Register64) => udiv_64;
        emit_umaddl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => umaddl;
        emit_umnegl(rd: Register64, rn: Register32, rm: Register32) => umnegl;
        emit_umsubl(rd: Register64, rn: Register32, rm: Register32, ra: Register64) => umsubl;
        emit_umulh(rd: Register64, rn: Register64, rm: Register64) => umulh;
        emit_umull(rd: Register64, rn: Register32, rm: Register32) => umull;
        emit_uxtb(rd: Register32, rn: Register32) => uxtb;
        emit_uxth(rd: Register32, rn: Register32) => uxth;
        emit_wfe() => wfe;
        emit_wfi() => wfi;
        emit_yield() => yield_;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


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
