use mitte_core::EmitSlice;

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
            FixupKind::Branch24,
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
            FixupKind::Branch24,
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
            FixupKind::Branch24,
            |e, offset| {
                e.emit_bl(self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_bl_cond_label<Label>(&mut self, cond: Condition, label: &mut Label)
        -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch24,
            |e, offset| {
                e.emit_bl_cond(cond, self::Label::from_byte_offset(offset))
            },
        )
    }

    fn emit_blx_label<Label>(&mut self, label: &mut Label) -> Result<(), Self::Error>
        where Self: mitte_core::Emit,
              Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch24,
            |e, offset| {
                e.emit_blx_imm(self::Label::from_byte_offset(offset))
            },
        )
    }

    forward! {
        emit_adc_imm(rd: Register, rn: Register, imm: impl Into<Const>) => adc_imm;
        emit_adc_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => adc_imm::cond;
        emit_adc_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => adc_reg;
        emit_adc_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => adc_reg::cond;
        emit_adc_reg_rrx(rd: Register, rn: Register, rm: Register) => adc_reg_rrx;
        emit_adc_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => adc_reg_rrx::cond;
        emit_adc_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => adc_reg_shifted_reg;
        emit_adc_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => adc_reg_shifted_reg::cond;
        emit_adcs_imm(rd: Register, rn: Register, imm: impl Into<Const>) => adcs_imm;
        emit_adcs_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => adcs_imm::cond;
        emit_adcs_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => adcs_reg;
        emit_adcs_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => adcs_reg::cond;
        emit_adcs_reg_rrx(rd: Register, rn: Register, rm: Register) => adcs_reg_rrx;
        emit_adcs_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => adcs_reg_rrx::cond;
        emit_adcs_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => adcs_reg_shifted_reg;
        emit_adcs_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => adcs_reg_shifted_reg::cond;
        emit_add_imm(rd: Register, rn: Register, imm: impl Into<Const>) => add_imm;
        emit_add_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => add_imm::cond;
        emit_add_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => add_reg;
        emit_add_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => add_reg::cond;
        emit_add_reg_rrx(rd: Register, rn: Register, rm: Register) => add_reg_rrx;
        emit_add_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => add_reg_rrx::cond;
        emit_add_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => add_reg_shifted_reg;
        emit_add_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => add_reg_shifted_reg::cond;
        emit_adds_imm(rd: Register, rn: Register, imm: impl Into<Const>) => adds_imm;
        emit_adds_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => adds_imm::cond;
        emit_adds_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => adds_reg;
        emit_adds_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => adds_reg::cond;
        emit_adds_reg_rrx(rd: Register, rn: Register, rm: Register) => adds_reg_rrx;
        emit_adds_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => adds_reg_rrx::cond;
        emit_adds_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => adds_reg_shifted_reg;
        emit_adds_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => adds_reg_shifted_reg::cond;
        emit_adr_add(rd: Register, imm: impl Into<Const>) => adr_add;
        emit_adr_add_cond(cond: Condition, rd: Register, imm: impl Into<Const>) => adr_add::cond;
        emit_adr_sub(rd: Register, imm: impl Into<Const>) => adr_sub;
        emit_adr_sub_cond(cond: Condition, rd: Register, imm: impl Into<Const>) => adr_sub::cond;
        emit_and_imm(rd: Register, rn: Register, imm: impl Into<Const>) => and_imm;
        emit_and_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => and_imm::cond;
        emit_and_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => and_reg;
        emit_and_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => and_reg::cond;
        emit_and_reg_rrx(rd: Register, rn: Register, rm: Register) => and_reg_rrx;
        emit_and_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => and_reg_rrx::cond;
        emit_and_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => and_reg_shifted_reg;
        emit_and_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => and_reg_shifted_reg::cond;
        emit_ands_imm(rd: Register, rn: Register, imm: impl Into<Const>) => ands_imm;
        emit_ands_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => ands_imm::cond;
        emit_ands_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => ands_reg;
        emit_ands_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => ands_reg::cond;
        emit_ands_reg_rrx(rd: Register, rn: Register, rm: Register) => ands_reg_rrx;
        emit_ands_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => ands_reg_rrx::cond;
        emit_ands_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => ands_reg_shifted_reg;
        emit_ands_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => ands_reg_shifted_reg::cond;
        emit_asr_imm(rd: Register, rm: Register, imm5: u8) => asr_imm;
        emit_asr_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => asr_imm::cond;
        emit_asr_reg(rd: Register, rm: Register, rs: Register) => asr_reg;
        emit_asr_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => asr_reg::cond;
        emit_asrs_imm(rd: Register, rm: Register, imm5: u8) => asrs_imm;
        emit_asrs_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => asrs_imm::cond;
        emit_asrs_reg(rd: Register, rm: Register, rs: Register) => asrs_reg;
        emit_asrs_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => asrs_reg::cond;
        emit_b(label: Label) => b;
        emit_b_cond(cond: Condition, label: Label) => b::cond;
        emit_bfc(rd: Register, lsb: u8, width: u8) => bfc;
        emit_bfc_cond(cond: Condition, rd: Register, lsb: u8, width: u8) => bfc::cond;
        emit_bfi(rd: Register, rn: Register, lsb: u8, width: u8) => bfi;
        emit_bfi_cond(cond: Condition, rd: Register, rn: Register, lsb: u8, width: u8) => bfi::cond;
        emit_bic_imm(rd: Register, rn: Register, imm: impl Into<Const>) => bic_imm;
        emit_bic_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => bic_imm::cond;
        emit_bic_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => bic_reg;
        emit_bic_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => bic_reg::cond;
        emit_bic_reg_rrx(rd: Register, rn: Register, rm: Register) => bic_reg_rrx;
        emit_bic_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => bic_reg_rrx::cond;
        emit_bic_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => bic_reg_shifted_reg;
        emit_bic_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => bic_reg_shifted_reg::cond;
        emit_bics_imm(rd: Register, rn: Register, imm: impl Into<Const>) => bics_imm;
        emit_bics_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => bics_imm::cond;
        emit_bics_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => bics_reg;
        emit_bics_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => bics_reg::cond;
        emit_bics_reg_rrx(rd: Register, rn: Register, rm: Register) => bics_reg_rrx;
        emit_bics_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => bics_reg_rrx::cond;
        emit_bics_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => bics_reg_shifted_reg;
        emit_bics_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => bics_reg_shifted_reg::cond;
        emit_bkpt(imm: u16) => bkpt;
        emit_bl(label: Label) => bl;
        emit_bl_cond(cond: Condition, label: Label) => bl::cond;
        emit_blx_imm(label: Label) => blx_imm;
        emit_blx_reg(rm: Register) => blx_reg;
        emit_blx_reg_cond(cond: Condition, rm: Register) => blx_reg::cond;
        emit_bx(rm: Register) => bx;
        emit_bx_cond(cond: Condition, rm: Register) => bx::cond;
        emit_bxj(rm: Register) => bxj;
        emit_bxj_cond(cond: Condition, rm: Register) => bxj::cond;
        emit_clrex() => clrex;
        emit_clz(rd: Register, rm: Register) => clz;
        emit_clz_cond(cond: Condition, rd: Register, rm: Register) => clz::cond;
        emit_cmn_imm(rn: Register, imm: impl Into<Const>) => cmn_imm;
        emit_cmn_imm_cond(cond: Condition, rn: Register, imm: impl Into<Const>) => cmn_imm::cond;
        emit_cmn_reg(rn: Register, rm: Register, shift: Shift<u8>) => cmn_reg;
        emit_cmn_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<u8>) => cmn_reg::cond;
        emit_cmn_reg_rrx(rn: Register, rm: Register) => cmn_reg_rrx;
        emit_cmn_reg_rrx_cond(cond: Condition, rn: Register, rm: Register) => cmn_reg_rrx::cond;
        emit_cmn_reg_shifted_reg(rn: Register, rm: Register, shift: Shift<Register>) => cmn_reg_shifted_reg;
        emit_cmn_reg_shifted_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<Register>) => cmn_reg_shifted_reg::cond;
        emit_cmp_imm(rn: Register, imm: impl Into<Const>) => cmp_imm;
        emit_cmp_imm_cond(cond: Condition, rn: Register, imm: impl Into<Const>) => cmp_imm::cond;
        emit_cmp_reg(rn: Register, rm: Register, shift: Shift<u8>) => cmp_reg;
        emit_cmp_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<u8>) => cmp_reg::cond;
        emit_cmp_reg_rrx(rn: Register, rm: Register) => cmp_reg_rrx;
        emit_cmp_reg_rrx_cond(cond: Condition, rn: Register, rm: Register) => cmp_reg_rrx::cond;
        emit_cmp_reg_shifted_reg(rn: Register, rm: Register, shift: Shift<Register>) => cmp_reg_shifted_reg;
        emit_cmp_reg_shifted_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<Register>) => cmp_reg_shifted_reg::cond;
        emit_cps(mode: u8) => cps;
        emit_cpsid(a: bool, i: bool, f: bool) => cpsid;
        emit_cpsid_mode(a: bool, i: bool, f: bool, mode: u8) => cpsid_mode;
        emit_cpsie(a: bool, i: bool, f: bool) => cpsie;
        emit_cpsie_mode(a: bool, i: bool, f: bool, mode: u8) => cpsie_mode;
        emit_csdb() => csdb;
        emit_csdb_cond(cond: Condition) => csdb::cond;
        emit_dbg(option: u8) => dbg;
        emit_dbg_cond(cond: Condition, option: u8) => dbg::cond;
        emit_dmb(option: u8) => dmb;
        emit_dsb(option: u8) => dsb;
        emit_eor_imm(rd: Register, rn: Register, imm: impl Into<Const>) => eor_imm;
        emit_eor_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => eor_imm::cond;
        emit_eor_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => eor_reg;
        emit_eor_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => eor_reg::cond;
        emit_eor_reg_rrx(rd: Register, rn: Register, rm: Register) => eor_reg_rrx;
        emit_eor_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => eor_reg_rrx::cond;
        emit_eor_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => eor_reg_shifted_reg;
        emit_eor_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => eor_reg_shifted_reg::cond;
        emit_eors_imm(rd: Register, rn: Register, imm: impl Into<Const>) => eors_imm;
        emit_eors_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => eors_imm::cond;
        emit_eors_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => eors_reg;
        emit_eors_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => eors_reg::cond;
        emit_eors_reg_rrx(rd: Register, rn: Register, rm: Register) => eors_reg_rrx;
        emit_eors_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => eors_reg_rrx::cond;
        emit_eors_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => eors_reg_shifted_reg;
        emit_eors_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => eors_reg_shifted_reg::cond;
        emit_eret() => eret;
        emit_eret_cond(cond: Condition) => eret::cond;
        emit_hvc(imm: u16) => hvc;
        emit_isb(option: u8) => isb;
        emit_ldc_imm_offset(rn: Register, offset: Offset<u8>) => ldc_imm_offset;
        emit_ldc_imm_offset_cond(cond: Condition, rn: Register, offset: Offset<u8>) => ldc_imm_offset::cond;
        emit_ldc_imm_post(rn: Register, offset: Offset<u8>) => ldc_imm_post;
        emit_ldc_imm_post_cond(cond: Condition, rn: Register, offset: Offset<u8>) => ldc_imm_post::cond;
        emit_ldc_imm_pre(rn: Register, offset: Offset<u8>) => ldc_imm_pre;
        emit_ldc_imm_pre_cond(cond: Condition, rn: Register, offset: Offset<u8>) => ldc_imm_pre::cond;
        emit_ldc_imm_unindexed(rn: Register, imm8: u8) => ldc_imm_unindexed;
        emit_ldc_imm_unindexed_cond(cond: Condition, rn: Register, imm8: u8) => ldc_imm_unindexed::cond;
        emit_ldc_lit_offset(offset: Offset<u8>) => ldc_lit_offset;
        emit_ldc_lit_offset_cond(cond: Condition, offset: Offset<u8>) => ldc_lit_offset::cond;
        emit_ldc_lit_unindexed(imm8: u8) => ldc_lit_unindexed;
        emit_ldc_lit_unindexed_cond(cond: Condition, imm8: u8) => ldc_lit_unindexed::cond;
        emit_ldm(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldm;
        emit_ldm_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldm::cond;
        emit_ldm_exception_return(amode: AddressMode, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldm_exception_return;
        emit_ldm_exception_return_cond(cond: Condition, amode: AddressMode, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldm_exception_return::cond;
        emit_ldm_user_registers(amode: AddressMode, rn: Register, regs: impl Into<RegisterList>) => ldm_user_registers;
        emit_ldm_user_registers_cond(cond: Condition, amode: AddressMode, rn: Register, regs: impl Into<RegisterList>) => ldm_user_registers::cond;
        emit_ldmda(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldmda;
        emit_ldmda_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldmda::cond;
        emit_ldmdb(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldmdb;
        emit_ldmdb_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldmdb::cond;
        emit_ldmib(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldmib;
        emit_ldmib_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => ldmib::cond;
        emit_ldr_imm_offset(rt: Register, rn: Register, offset12: Offset<u16>) => ldr_imm_offset;
        emit_ldr_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldr_imm_offset::cond;
        emit_ldr_imm_post(rt: Register, rn: Register, offset12: Offset<u16>) => ldr_imm_post;
        emit_ldr_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldr_imm_post::cond;
        emit_ldr_imm_pre(rt: Register, rn: Register, offset12: Offset<u16>) => ldr_imm_pre;
        emit_ldr_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldr_imm_pre::cond;
        emit_ldr_lit(rt: Register, offset12: Offset<u16>) => ldr_lit;
        emit_ldr_lit_cond(cond: Condition, rt: Register, offset12: Offset<u16>) => ldr_lit::cond;
        emit_ldr_reg_offset(rt: Register, rn: Register, index: impl Into<Index>) => ldr_reg_offset;
        emit_ldr_reg_offset_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldr_reg_offset::cond;
        emit_ldr_reg_post(rt: Register, rn: Register, index: impl Into<Index>) => ldr_reg_post;
        emit_ldr_reg_post_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldr_reg_post::cond;
        emit_ldr_reg_pre(rt: Register, rn: Register, index: impl Into<Index>) => ldr_reg_pre;
        emit_ldr_reg_pre_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldr_reg_pre::cond;
        emit_ldrb_imm_offset(rt: Register, rn: Register, offset12: Offset<u16>) => ldrb_imm_offset;
        emit_ldrb_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldrb_imm_offset::cond;
        emit_ldrb_imm_post(rt: Register, rn: Register, offset12: Offset<u16>) => ldrb_imm_post;
        emit_ldrb_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldrb_imm_post::cond;
        emit_ldrb_imm_pre(rt: Register, rn: Register, offset12: Offset<u16>) => ldrb_imm_pre;
        emit_ldrb_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldrb_imm_pre::cond;
        emit_ldrb_lit(rt: Register, offset12: Offset<u16>) => ldrb_lit;
        emit_ldrb_lit_cond(cond: Condition, rt: Register, offset12: Offset<u16>) => ldrb_lit::cond;
        emit_ldrb_reg_offset(rt: Register, rn: Register, index: impl Into<Index>) => ldrb_reg_offset;
        emit_ldrb_reg_offset_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldrb_reg_offset::cond;
        emit_ldrb_reg_post(rt: Register, rn: Register, index: impl Into<Index>) => ldrb_reg_post;
        emit_ldrb_reg_post_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldrb_reg_post::cond;
        emit_ldrb_reg_pre(rt: Register, rn: Register, index: impl Into<Index>) => ldrb_reg_pre;
        emit_ldrb_reg_pre_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldrb_reg_pre::cond;
        emit_ldrbt_imm(rt: Register, rn: Register, offset12: Offset<u16>) => ldrbt_imm;
        emit_ldrbt_imm_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldrbt_imm::cond;
        emit_ldrbt_reg(rt: Register, rn: Register, index: impl Into<Index>) => ldrbt_reg;
        emit_ldrbt_reg_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldrbt_reg::cond;
        emit_ldrd_imm_offset(rt: RegisterPair, rn: Register, offset: Offset<u8>) => ldrd_imm_offset;
        emit_ldrd_imm_offset_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<u8>) => ldrd_imm_offset::cond;
        emit_ldrd_imm_post(rt: RegisterPair, rn: Register, offset: Offset<u8>) => ldrd_imm_post;
        emit_ldrd_imm_post_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<u8>) => ldrd_imm_post::cond;
        emit_ldrd_imm_pre(rt: RegisterPair, rn: Register, offset: Offset<u8>) => ldrd_imm_pre;
        emit_ldrd_imm_pre_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<u8>) => ldrd_imm_pre::cond;
        emit_ldrd_lit(rt: RegisterPair, offset: Offset<u8>) => ldrd_lit;
        emit_ldrd_lit_cond(cond: Condition, rt: RegisterPair, offset: Offset<u8>) => ldrd_lit::cond;
        emit_ldrd_reg_offset(rt: RegisterPair, rn: Register, offset: Offset<Register>) => ldrd_reg_offset;
        emit_ldrd_reg_offset_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<Register>) => ldrd_reg_offset::cond;
        emit_ldrd_reg_post(rt: RegisterPair, rn: Register, offset: Offset<Register>) => ldrd_reg_post;
        emit_ldrd_reg_post_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<Register>) => ldrd_reg_post::cond;
        emit_ldrd_reg_pre(rt: RegisterPair, rn: Register, offset: Offset<Register>) => ldrd_reg_pre;
        emit_ldrd_reg_pre_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<Register>) => ldrd_reg_pre::cond;
        emit_ldrex(rt: Register, rn: Register) => ldrex;
        emit_ldrex_cond(cond: Condition, rt: Register, rn: Register) => ldrex::cond;
        emit_ldrexb(rt: Register, rn: Register) => ldrexb;
        emit_ldrexb_cond(cond: Condition, rt: Register, rn: Register) => ldrexb::cond;
        emit_ldrexd(rt: RegisterPair, rn: Register) => ldrexd;
        emit_ldrexd_cond(cond: Condition, rt: RegisterPair, rn: Register) => ldrexd::cond;
        emit_ldrexh(rt: Register, rn: Register) => ldrexh;
        emit_ldrexh_cond(cond: Condition, rt: Register, rn: Register) => ldrexh::cond;
        emit_ldrh_imm_offset(rt: Register, rn: Register, offset: Offset<u8>) => ldrh_imm_offset;
        emit_ldrh_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrh_imm_offset::cond;
        emit_ldrh_imm_post(rt: Register, rn: Register, offset: Offset<u8>) => ldrh_imm_post;
        emit_ldrh_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrh_imm_post::cond;
        emit_ldrh_imm_pre(rt: Register, rn: Register, offset: Offset<u8>) => ldrh_imm_pre;
        emit_ldrh_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrh_imm_pre::cond;
        emit_ldrh_lit(rt: Register, offset: Offset<u8>) => ldrh_lit;
        emit_ldrh_lit_cond(cond: Condition, rt: Register, offset: Offset<u8>) => ldrh_lit::cond;
        emit_ldrh_reg_offset(rt: Register, rn: Register, offset: Offset<Register>) => ldrh_reg_offset;
        emit_ldrh_reg_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrh_reg_offset::cond;
        emit_ldrh_reg_post(rt: Register, rn: Register, offset: Offset<Register>) => ldrh_reg_post;
        emit_ldrh_reg_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrh_reg_post::cond;
        emit_ldrh_reg_pre(rt: Register, rn: Register, offset: Offset<Register>) => ldrh_reg_pre;
        emit_ldrh_reg_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrh_reg_pre::cond;
        emit_ldrht_imm(rt: Register, rn: Register, offset: Offset<u8>) => ldrht_imm;
        emit_ldrht_imm_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrht_imm::cond;
        emit_ldrht_reg(rt: Register, rn: Register, offset: Offset<Register>) => ldrht_reg;
        emit_ldrht_reg_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrht_reg::cond;
        emit_ldrsb_imm_offset(rt: Register, rn: Register, offset: Offset<u8>) => ldrsb_imm_offset;
        emit_ldrsb_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsb_imm_offset::cond;
        emit_ldrsb_imm_post(rt: Register, rn: Register, offset: Offset<u8>) => ldrsb_imm_post;
        emit_ldrsb_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsb_imm_post::cond;
        emit_ldrsb_imm_pre(rt: Register, rn: Register, offset: Offset<u8>) => ldrsb_imm_pre;
        emit_ldrsb_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsb_imm_pre::cond;
        emit_ldrsb_lit(rt: Register, offset: Offset<u8>) => ldrsb_lit;
        emit_ldrsb_lit_cond(cond: Condition, rt: Register, offset: Offset<u8>) => ldrsb_lit::cond;
        emit_ldrsb_reg_offset(rt: Register, rn: Register, offset: Offset<Register>) => ldrsb_reg_offset;
        emit_ldrsb_reg_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsb_reg_offset::cond;
        emit_ldrsb_reg_post(rt: Register, rn: Register, offset: Offset<Register>) => ldrsb_reg_post;
        emit_ldrsb_reg_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsb_reg_post::cond;
        emit_ldrsb_reg_pre(rt: Register, rn: Register, offset: Offset<Register>) => ldrsb_reg_pre;
        emit_ldrsb_reg_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsb_reg_pre::cond;
        emit_ldrsbt_imm(rt: Register, rn: Register, offset: Offset<u8>) => ldrsbt_imm;
        emit_ldrsbt_imm_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsbt_imm::cond;
        emit_ldrsbt_reg(rt: Register, rn: Register, offset: Offset<Register>) => ldrsbt_reg;
        emit_ldrsbt_reg_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsbt_reg::cond;
        emit_ldrsh_imm_offset(rt: Register, rn: Register, offset: Offset<u8>) => ldrsh_imm_offset;
        emit_ldrsh_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsh_imm_offset::cond;
        emit_ldrsh_imm_post(rt: Register, rn: Register, offset: Offset<u8>) => ldrsh_imm_post;
        emit_ldrsh_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsh_imm_post::cond;
        emit_ldrsh_imm_pre(rt: Register, rn: Register, offset: Offset<u8>) => ldrsh_imm_pre;
        emit_ldrsh_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsh_imm_pre::cond;
        emit_ldrsh_lit(rt: Register, offset: Offset<u8>) => ldrsh_lit;
        emit_ldrsh_lit_cond(cond: Condition, rt: Register, offset: Offset<u8>) => ldrsh_lit::cond;
        emit_ldrsh_reg_offset(rt: Register, rn: Register, offset: Offset<Register>) => ldrsh_reg_offset;
        emit_ldrsh_reg_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsh_reg_offset::cond;
        emit_ldrsh_reg_post(rt: Register, rn: Register, offset: Offset<Register>) => ldrsh_reg_post;
        emit_ldrsh_reg_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsh_reg_post::cond;
        emit_ldrsh_reg_pre(rt: Register, rn: Register, offset: Offset<Register>) => ldrsh_reg_pre;
        emit_ldrsh_reg_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsh_reg_pre::cond;
        emit_ldrsht_imm(rt: Register, rn: Register, offset: Offset<u8>) => ldrsht_imm;
        emit_ldrsht_imm_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => ldrsht_imm::cond;
        emit_ldrsht_reg(rt: Register, rn: Register, offset: Offset<Register>) => ldrsht_reg;
        emit_ldrsht_reg_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => ldrsht_reg::cond;
        emit_ldrt_imm(rt: Register, rn: Register, offset12: Offset<u16>) => ldrt_imm;
        emit_ldrt_imm_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => ldrt_imm::cond;
        emit_ldrt_reg(rt: Register, rn: Register, index: impl Into<Index>) => ldrt_reg;
        emit_ldrt_reg_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => ldrt_reg::cond;
        emit_lsl_imm(rd: Register, rm: Register, imm5: u8) => lsl_imm;
        emit_lsl_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => lsl_imm::cond;
        emit_lsl_reg(rd: Register, rm: Register, rs: Register) => lsl_reg;
        emit_lsl_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => lsl_reg::cond;
        emit_lsls_imm(rd: Register, rm: Register, imm5: u8) => lsls_imm;
        emit_lsls_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => lsls_imm::cond;
        emit_lsls_reg(rd: Register, rm: Register, rs: Register) => lsls_reg;
        emit_lsls_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => lsls_reg::cond;
        emit_lsr_imm(rd: Register, rm: Register, imm5: u8) => lsr_imm;
        emit_lsr_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => lsr_imm::cond;
        emit_lsr_reg(rd: Register, rm: Register, rs: Register) => lsr_reg;
        emit_lsr_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => lsr_reg::cond;
        emit_lsrs_imm(rd: Register, rm: Register, imm5: u8) => lsrs_imm;
        emit_lsrs_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => lsrs_imm::cond;
        emit_lsrs_reg(rd: Register, rm: Register, rs: Register) => lsrs_reg;
        emit_lsrs_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => lsrs_reg::cond;
        emit_mcr(coproc: u8, opc1: u8, rt: Register, crn: u8, crm: u8, opc2: u8) => mcr;
        emit_mcr_cond(cond: Condition, coproc: u8, opc1: u8, rt: Register, crn: u8, crm: u8, opc2: u8) => mcr::cond;
        emit_mcrr(coproc: u8, opc1: u8, rt: Register, rt2: Register, crm: u8) => mcrr;
        emit_mcrr_cond(cond: Condition, coproc: u8, opc1: u8, rt: Register, rt2: Register, crm: u8) => mcrr::cond;
        emit_mla(rd: Register, rn: Register, rm: Register, ra: Register) => mla;
        emit_mla_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => mla::cond;
        emit_mlas(rd: Register, rn: Register, rm: Register, ra: Register) => mlas;
        emit_mlas_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => mlas::cond;
        emit_mls(rd: Register, rn: Register, rm: Register, ra: Register) => mls;
        emit_mls_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => mls::cond;
        emit_mov_imm(rd: Register, imm: impl Into<Const>) => mov_imm;
        emit_mov_imm_cond(cond: Condition, rd: Register, imm: impl Into<Const>) => mov_imm::cond;
        emit_mov_reg(rd: Register, rm: Register, shift: Shift<u8>) => mov_reg;
        emit_mov_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<u8>) => mov_reg::cond;
        emit_mov_reg_rrx(rd: Register, rm: Register) => mov_reg_rrx;
        emit_mov_reg_rrx_cond(cond: Condition, rd: Register, rm: Register) => mov_reg_rrx::cond;
        emit_mov_reg_shifted_reg(rd: Register, rm: Register, shift: Shift<Register>) => mov_reg_shifted_reg;
        emit_mov_reg_shifted_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<Register>) => mov_reg_shifted_reg::cond;
        emit_movs_imm(rd: Register, imm: impl Into<Const>) => movs_imm;
        emit_movs_imm_cond(cond: Condition, rd: Register, imm: impl Into<Const>) => movs_imm::cond;
        emit_movs_reg(rd: Register, rm: Register, shift: Shift<u8>) => movs_reg;
        emit_movs_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<u8>) => movs_reg::cond;
        emit_movs_reg_rrx(rd: Register, rm: Register) => movs_reg_rrx;
        emit_movs_reg_rrx_cond(cond: Condition, rd: Register, rm: Register) => movs_reg_rrx::cond;
        emit_movs_reg_shifted_reg(rd: Register, rm: Register, shift: Shift<Register>) => movs_reg_shifted_reg;
        emit_movs_reg_shifted_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<Register>) => movs_reg_shifted_reg::cond;
        emit_movt(rd: Register, imm16: u16) => movt;
        emit_movt_cond(cond: Condition, rd: Register, imm16: u16) => movt::cond;
        emit_movw(rd: Register, imm16: u16) => movw;
        emit_movw_cond(cond: Condition, rd: Register, imm16: u16) => movw::cond;
        emit_mrc(coproc: u8, opc1: u8, rt: Register, crn: u8, crm: u8, opc2: u8) => mrc;
        emit_mrc_cond(cond: Condition, coproc: u8, opc1: u8, rt: Register, crn: u8, crm: u8, opc2: u8) => mrc::cond;
        emit_mrrc(coproc: u8, opc1: u8, rt: Register, rt2: Register, crm: u8) => mrrc;
        emit_mrrc_cond(cond: Condition, coproc: u8, opc1: u8, rt: Register, rt2: Register, crm: u8) => mrrc::cond;
        emit_mrs(rd: Register, special_register: u8) => mrs;
        emit_mrs_cond(cond: Condition, rd: Register, special_register: u8) => mrs::cond;
        emit_mrs_banked(rd: Register, banked_register: BankedRegister) => mrs_banked;
        emit_mrs_banked_cond(cond: Condition, rd: Register, banked_register: BankedRegister) => mrs_banked::cond;
        emit_msr_banked(banked_register: BankedRegister, rn: Register) => msr_banked;
        emit_msr_banked_cond(cond: Condition, banked_register: BankedRegister, rn: Register) => msr_banked::cond;
        emit_msr_imm(special_register: u8, mask: u8, imm: impl Into<Const>) => msr_imm;
        emit_msr_imm_cond(cond: Condition, special_register: u8, mask: u8, imm: impl Into<Const>) => msr_imm::cond;
        emit_msr_reg(special_register: u8, mask: u8, rn: Register) => msr_reg;
        emit_msr_reg_cond(cond: Condition, special_register: u8, mask: u8, rn: Register) => msr_reg::cond;
        emit_mul(rd: Register, rn: Register, rm: Register) => mul;
        emit_mul_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => mul::cond;
        emit_muls(rd: Register, rn: Register, rm: Register) => muls;
        emit_muls_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => muls::cond;
        emit_mvn_imm(rd: Register, imm: impl Into<Const>) => mvn_imm;
        emit_mvn_imm_cond(cond: Condition, rd: Register, imm: impl Into<Const>) => mvn_imm::cond;
        emit_mvn_reg(rd: Register, rm: Register, shift: Shift<u8>) => mvn_reg;
        emit_mvn_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<u8>) => mvn_reg::cond;
        emit_mvn_reg_rrx(rd: Register, rm: Register) => mvn_reg_rrx;
        emit_mvn_reg_rrx_cond(cond: Condition, rd: Register, rm: Register) => mvn_reg_rrx::cond;
        emit_mvn_reg_shifted_reg(rd: Register, rm: Register, shift: Shift<Register>) => mvn_reg_shifted_reg;
        emit_mvn_reg_shifted_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<Register>) => mvn_reg_shifted_reg::cond;
        emit_mvns_imm(rd: Register, imm: impl Into<Const>) => mvns_imm;
        emit_mvns_imm_cond(cond: Condition, rd: Register, imm: impl Into<Const>) => mvns_imm::cond;
        emit_mvns_reg(rd: Register, rm: Register, shift: Shift<u8>) => mvns_reg;
        emit_mvns_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<u8>) => mvns_reg::cond;
        emit_mvns_reg_rrx(rd: Register, rm: Register) => mvns_reg_rrx;
        emit_mvns_reg_rrx_cond(cond: Condition, rd: Register, rm: Register) => mvns_reg_rrx::cond;
        emit_mvns_reg_shifted_reg(rd: Register, rm: Register, shift: Shift<Register>) => mvns_reg_shifted_reg;
        emit_mvns_reg_shifted_reg_cond(cond: Condition, rd: Register, rm: Register, shift: Shift<Register>) => mvns_reg_shifted_reg::cond;
        emit_nop() => nop;
        emit_nop_cond(cond: Condition) => nop::cond;
        emit_orr_imm(rd: Register, rn: Register, imm: impl Into<Const>) => orr_imm;
        emit_orr_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => orr_imm::cond;
        emit_orr_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => orr_reg;
        emit_orr_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => orr_reg::cond;
        emit_orr_reg_rrx(rd: Register, rn: Register, rm: Register) => orr_reg_rrx;
        emit_orr_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => orr_reg_rrx::cond;
        emit_orr_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => orr_reg_shifted_reg;
        emit_orr_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => orr_reg_shifted_reg::cond;
        emit_orrs_imm(rd: Register, rn: Register, imm: impl Into<Const>) => orrs_imm;
        emit_orrs_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => orrs_imm::cond;
        emit_orrs_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => orrs_reg;
        emit_orrs_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => orrs_reg::cond;
        emit_orrs_reg_rrx(rd: Register, rn: Register, rm: Register) => orrs_reg_rrx;
        emit_orrs_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => orrs_reg_rrx::cond;
        emit_orrs_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => orrs_reg_shifted_reg;
        emit_orrs_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => orrs_reg_shifted_reg::cond;
        emit_pkhbt(rd: Register, rn: Register, rm: Register, imm5: u8) => pkhbt;
        emit_pkhbt_cond(cond: Condition, rd: Register, rn: Register, rm: Register, imm5: u8) => pkhbt::cond;
        emit_pkhtb(rd: Register, rn: Register, rm: Register, imm5: u8) => pkhtb;
        emit_pkhtb_cond(cond: Condition, rd: Register, rn: Register, rm: Register, imm5: u8) => pkhtb::cond;
        emit_pld_imm(rn: Register, offset12: Offset<u16>) => pld_imm;
        emit_pld_lit(offset12: Offset<u16>) => pld_lit;
        emit_pld_reg(rn: Register, index: impl Into<Index>) => pld_reg;
        emit_pld_reg_rrx(rn: Register, u: u32, rm: Register) => pld_reg_rrx;
        emit_pldw_imm(rn: Register, offset12: Offset<u16>) => pldw_imm;
        emit_pldw_reg(rn: Register, index: impl Into<Index>) => pldw_reg;
        emit_pldw_reg_rrx(rn: Register, u: u32, rm: Register) => pldw_reg_rrx;
        emit_pli_imm(rn: Register, offset12: Offset<u16>) => pli_imm;
        emit_pli_reg(rn: Register, index: impl Into<Index>) => pli_reg;
        emit_pli_reg_rrx(rn: Register, u: u32, rm: Register) => pli_reg_rrx;
        emit_pop_list(regs: impl Into<RegisterList>) => pop_list;
        emit_pop_list_cond(cond: Condition, regs: impl Into<RegisterList>) => pop_list::cond;
        emit_pop_reg(rt: Register) => pop_reg;
        emit_pop_reg_cond(cond: Condition, rt: Register) => pop_reg::cond;
        emit_push_list(regs: impl Into<RegisterList>) => push_list;
        emit_push_list_cond(cond: Condition, regs: impl Into<RegisterList>) => push_list::cond;
        emit_push_reg(rt: Register) => push_reg;
        emit_push_reg_cond(cond: Condition, rt: Register) => push_reg::cond;
        emit_qadd(rd: Register, rm: Register, rn: Register) => qadd;
        emit_qadd_cond(cond: Condition, rd: Register, rm: Register, rn: Register) => qadd::cond;
        emit_qadd8(rd: Register, rn: Register, rm: Register) => qadd8;
        emit_qadd8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => qadd8::cond;
        emit_qadd16(rd: Register, rn: Register, rm: Register) => qadd16;
        emit_qadd16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => qadd16::cond;
        emit_qasx(rd: Register, rn: Register, rm: Register) => qasx;
        emit_qasx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => qasx::cond;
        emit_qdadd(rd: Register, rm: Register, rn: Register) => qdadd;
        emit_qdadd_cond(cond: Condition, rd: Register, rm: Register, rn: Register) => qdadd::cond;
        emit_qdsub(rd: Register, rm: Register, rn: Register) => qdsub;
        emit_qdsub_cond(cond: Condition, rd: Register, rm: Register, rn: Register) => qdsub::cond;
        emit_qsax(rd: Register, rn: Register, rm: Register) => qsax;
        emit_qsax_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => qsax::cond;
        emit_qsub(rd: Register, rm: Register, rn: Register) => qsub;
        emit_qsub_cond(cond: Condition, rd: Register, rm: Register, rn: Register) => qsub::cond;
        emit_qsub8(rd: Register, rn: Register, rm: Register) => qsub8;
        emit_qsub8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => qsub8::cond;
        emit_qsub16(rd: Register, rn: Register, rm: Register) => qsub16;
        emit_qsub16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => qsub16::cond;
        emit_rbit(rd: Register, rm: Register) => rbit;
        emit_rbit_cond(cond: Condition, rd: Register, rm: Register) => rbit::cond;
        emit_rev(rd: Register, rm: Register) => rev;
        emit_rev_cond(cond: Condition, rd: Register, rm: Register) => rev::cond;
        emit_rev16(rd: Register, rm: Register) => rev16;
        emit_rev16_cond(cond: Condition, rd: Register, rm: Register) => rev16::cond;
        emit_revsh(rd: Register, rm: Register) => revsh;
        emit_revsh_cond(cond: Condition, rd: Register, rm: Register) => revsh::cond;
        emit_rfeda(rn: Register) => rfeda;
        emit_rfedb(rn: Register) => rfedb;
        emit_rfeia(rn: Register) => rfeia;
        emit_rfeib(rn: Register) => rfeib;
        emit_ror_imm(rd: Register, rm: Register, imm5: u8) => ror_imm;
        emit_ror_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => ror_imm::cond;
        emit_ror_reg(rd: Register, rm: Register, rs: Register) => ror_reg;
        emit_ror_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => ror_reg::cond;
        emit_rors_imm(rd: Register, rm: Register, imm5: u8) => rors_imm;
        emit_rors_imm_cond(cond: Condition, rd: Register, rm: Register, imm5: u8) => rors_imm::cond;
        emit_rors_reg(rd: Register, rm: Register, rs: Register) => rors_reg;
        emit_rors_reg_cond(cond: Condition, rd: Register, rm: Register, rs: Register) => rors_reg::cond;
        emit_rrx(rd: Register, rm: Register) => rrx;
        emit_rrx_cond(cond: Condition, rd: Register, rm: Register) => rrx::cond;
        emit_rrxs(rd: Register, rm: Register) => rrxs;
        emit_rrxs_cond(cond: Condition, rd: Register, rm: Register) => rrxs::cond;
        emit_rsb_imm(rd: Register, rn: Register, imm: impl Into<Const>) => rsb_imm;
        emit_rsb_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => rsb_imm::cond;
        emit_rsb_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rsb_reg;
        emit_rsb_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rsb_reg::cond;
        emit_rsb_reg_rrx(rd: Register, rn: Register, rm: Register) => rsb_reg_rrx;
        emit_rsb_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => rsb_reg_rrx::cond;
        emit_rsb_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rsb_reg_shifted_reg;
        emit_rsb_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rsb_reg_shifted_reg::cond;
        emit_rsbs_imm(rd: Register, rn: Register, imm: impl Into<Const>) => rsbs_imm;
        emit_rsbs_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => rsbs_imm::cond;
        emit_rsbs_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rsbs_reg;
        emit_rsbs_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rsbs_reg::cond;
        emit_rsbs_reg_rrx(rd: Register, rn: Register, rm: Register) => rsbs_reg_rrx;
        emit_rsbs_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => rsbs_reg_rrx::cond;
        emit_rsbs_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rsbs_reg_shifted_reg;
        emit_rsbs_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rsbs_reg_shifted_reg::cond;
        emit_rsc_imm(rd: Register, rn: Register, imm: impl Into<Const>) => rsc_imm;
        emit_rsc_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => rsc_imm::cond;
        emit_rsc_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rsc_reg;
        emit_rsc_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rsc_reg::cond;
        emit_rsc_reg_rrx(rd: Register, rn: Register, rm: Register) => rsc_reg_rrx;
        emit_rsc_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => rsc_reg_rrx::cond;
        emit_rsc_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rsc_reg_shifted_reg;
        emit_rsc_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rsc_reg_shifted_reg::cond;
        emit_rscs_imm(rd: Register, rn: Register, imm: impl Into<Const>) => rscs_imm;
        emit_rscs_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => rscs_imm::cond;
        emit_rscs_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rscs_reg;
        emit_rscs_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => rscs_reg::cond;
        emit_rscs_reg_rrx(rd: Register, rn: Register, rm: Register) => rscs_reg_rrx;
        emit_rscs_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => rscs_reg_rrx::cond;
        emit_rscs_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rscs_reg_shifted_reg;
        emit_rscs_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => rscs_reg_shifted_reg::cond;
        emit_sadd8(rd: Register, rn: Register, rm: Register) => sadd8;
        emit_sadd8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sadd8::cond;
        emit_sadd16(rd: Register, rn: Register, rm: Register) => sadd16;
        emit_sadd16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sadd16::cond;
        emit_sasx(rd: Register, rn: Register, rm: Register) => sasx;
        emit_sasx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sasx::cond;
        emit_sb() => sb;
        emit_sbc_imm(rd: Register, rn: Register, imm: impl Into<Const>) => sbc_imm;
        emit_sbc_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => sbc_imm::cond;
        emit_sbc_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => sbc_reg;
        emit_sbc_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => sbc_reg::cond;
        emit_sbc_reg_rrx(rd: Register, rn: Register, rm: Register) => sbc_reg_rrx;
        emit_sbc_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sbc_reg_rrx::cond;
        emit_sbc_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => sbc_reg_shifted_reg;
        emit_sbc_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => sbc_reg_shifted_reg::cond;
        emit_sbcs_imm(rd: Register, rn: Register, imm: impl Into<Const>) => sbcs_imm;
        emit_sbcs_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => sbcs_imm::cond;
        emit_sbcs_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => sbcs_reg;
        emit_sbcs_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => sbcs_reg::cond;
        emit_sbcs_reg_rrx(rd: Register, rn: Register, rm: Register) => sbcs_reg_rrx;
        emit_sbcs_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sbcs_reg_rrx::cond;
        emit_sbcs_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => sbcs_reg_shifted_reg;
        emit_sbcs_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => sbcs_reg_shifted_reg::cond;
        emit_sbfx(rd: Register, rn: Register, lsb: u8, width: u8) => sbfx;
        emit_sbfx_cond(cond: Condition, rd: Register, rn: Register, lsb: u8, width: u8) => sbfx::cond;
        emit_sdiv(rd: Register, rn: Register, rm: Register) => sdiv;
        emit_sdiv_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sdiv::cond;
        emit_sel(rd: Register, rn: Register, rm: Register) => sel;
        emit_sel_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sel::cond;
        emit_setend(endianness: Endianness) => setend;
        emit_sev() => sev;
        emit_sev_cond(cond: Condition) => sev::cond;
        emit_shadd8(rd: Register, rn: Register, rm: Register) => shadd8;
        emit_shadd8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => shadd8::cond;
        emit_shadd16(rd: Register, rn: Register, rm: Register) => shadd16;
        emit_shadd16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => shadd16::cond;
        emit_shasx(rd: Register, rn: Register, rm: Register) => shasx;
        emit_shasx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => shasx::cond;
        emit_shsax(rd: Register, rn: Register, rm: Register) => shsax;
        emit_shsax_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => shsax::cond;
        emit_shsub8(rd: Register, rn: Register, rm: Register) => shsub8;
        emit_shsub8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => shsub8::cond;
        emit_shsub16(rd: Register, rn: Register, rm: Register) => shsub16;
        emit_shsub16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => shsub16::cond;
        emit_smc(imm4: u8) => smc;
        emit_smc_cond(cond: Condition, imm4: u8) => smc::cond;
        emit_smlabb(rd: Register, rn: Register, rm: Register, ra: Register) => smlabb;
        emit_smlabb_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlabb::cond;
        emit_smlabt(rd: Register, rn: Register, rm: Register, ra: Register) => smlabt;
        emit_smlabt_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlabt::cond;
        emit_smlad(rd: Register, rn: Register, rm: Register, ra: Register) => smlad;
        emit_smlad_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlad::cond;
        emit_smladx(rd: Register, rn: Register, rm: Register, ra: Register) => smladx;
        emit_smladx_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smladx::cond;
        emit_smlal(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlal;
        emit_smlal_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlal::cond;
        emit_smlalbb(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlalbb;
        emit_smlalbb_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlalbb::cond;
        emit_smlalbt(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlalbt;
        emit_smlalbt_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlalbt::cond;
        emit_smlald(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlald;
        emit_smlald_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlald::cond;
        emit_smlaldx(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlaldx;
        emit_smlaldx_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlaldx::cond;
        emit_smlals(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlals;
        emit_smlals_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlals::cond;
        emit_smlaltb(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlaltb;
        emit_smlaltb_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlaltb::cond;
        emit_smlaltt(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlaltt;
        emit_smlaltt_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlaltt::cond;
        emit_smlatb(rd: Register, rn: Register, rm: Register, ra: Register) => smlatb;
        emit_smlatb_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlatb::cond;
        emit_smlatt(rd: Register, rn: Register, rm: Register, ra: Register) => smlatt;
        emit_smlatt_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlatt::cond;
        emit_smlawb(rd: Register, rn: Register, rm: Register, ra: Register) => smlawb;
        emit_smlawb_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlawb::cond;
        emit_smlawt(rd: Register, rn: Register, rm: Register, ra: Register) => smlawt;
        emit_smlawt_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlawt::cond;
        emit_smlsd(rd: Register, rn: Register, rm: Register, ra: Register) => smlsd;
        emit_smlsd_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlsd::cond;
        emit_smlsdx(rd: Register, rn: Register, rm: Register, ra: Register) => smlsdx;
        emit_smlsdx_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smlsdx::cond;
        emit_smlsld(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlsld;
        emit_smlsld_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlsld::cond;
        emit_smlsldx(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlsldx;
        emit_smlsldx_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smlsldx::cond;
        emit_smmla(rd: Register, rn: Register, rm: Register, ra: Register) => smmla;
        emit_smmla_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smmla::cond;
        emit_smmlar(rd: Register, rn: Register, rm: Register, ra: Register) => smmlar;
        emit_smmlar_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smmlar::cond;
        emit_smmls(rd: Register, rn: Register, rm: Register, ra: Register) => smmls;
        emit_smmls_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smmls::cond;
        emit_smmlsr(rd: Register, rn: Register, rm: Register, ra: Register) => smmlsr;
        emit_smmlsr_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => smmlsr::cond;
        emit_smmul(rd: Register, rn: Register, rm: Register) => smmul;
        emit_smmul_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smmul::cond;
        emit_smmulr(rd: Register, rn: Register, rm: Register) => smmulr;
        emit_smmulr_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smmulr::cond;
        emit_smuad(rd: Register, rn: Register, rm: Register) => smuad;
        emit_smuad_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smuad::cond;
        emit_smuadx(rd: Register, rn: Register, rm: Register) => smuadx;
        emit_smuadx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smuadx::cond;
        emit_smulbb(rd: Register, rn: Register, rm: Register) => smulbb;
        emit_smulbb_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smulbb::cond;
        emit_smulbt(rd: Register, rn: Register, rm: Register) => smulbt;
        emit_smulbt_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smulbt::cond;
        emit_smull(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smull;
        emit_smull_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smull::cond;
        emit_smulls(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smulls;
        emit_smulls_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => smulls::cond;
        emit_smultb(rd: Register, rn: Register, rm: Register) => smultb;
        emit_smultb_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smultb::cond;
        emit_smultt(rd: Register, rn: Register, rm: Register) => smultt;
        emit_smultt_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smultt::cond;
        emit_smulwb(rd: Register, rn: Register, rm: Register) => smulwb;
        emit_smulwb_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smulwb::cond;
        emit_smulwt(rd: Register, rn: Register, rm: Register) => smulwt;
        emit_smulwt_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smulwt::cond;
        emit_smusd(rd: Register, rn: Register, rm: Register) => smusd;
        emit_smusd_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smusd::cond;
        emit_smusdx(rd: Register, rn: Register, rm: Register) => smusdx;
        emit_smusdx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => smusdx::cond;
        emit_srsda(writeback: bool, mode: u8) => srsda;
        emit_srsdb(writeback: bool, mode: u8) => srsdb;
        emit_srsia(writeback: bool, mode: u8) => srsia;
        emit_srsib(writeback: bool, mode: u8) => srsib;
        emit_ssat_asr(rd: Register, saturate_to: u8, rn: Register, amount: u8) => ssat_asr;
        emit_ssat_asr_cond(cond: Condition, rd: Register, saturate_to: u8, rn: Register, amount: u8) => ssat_asr::cond;
        emit_ssat_lsl(rd: Register, saturate_to: u8, rn: Register, amount: u8) => ssat_lsl;
        emit_ssat_lsl_cond(cond: Condition, rd: Register, saturate_to: u8, rn: Register, amount: u8) => ssat_lsl::cond;
        emit_ssat16(rd: Register, saturate_to: u8, rn: Register) => ssat16;
        emit_ssat16_cond(cond: Condition, rd: Register, saturate_to: u8, rn: Register) => ssat16::cond;
        emit_ssax(rd: Register, rn: Register, rm: Register) => ssax;
        emit_ssax_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => ssax::cond;
        emit_ssub8(rd: Register, rn: Register, rm: Register) => ssub8;
        emit_ssub8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => ssub8::cond;
        emit_ssub16(rd: Register, rn: Register, rm: Register) => ssub16;
        emit_ssub16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => ssub16::cond;
        emit_stc_imm_offset(rn: Register, offset: Offset<u8>) => stc_imm_offset;
        emit_stc_imm_offset_cond(cond: Condition, rn: Register, offset: Offset<u8>) => stc_imm_offset::cond;
        emit_stc_imm_post(rn: Register, offset: Offset<u8>) => stc_imm_post;
        emit_stc_imm_post_cond(cond: Condition, rn: Register, offset: Offset<u8>) => stc_imm_post::cond;
        emit_stc_imm_pre(rn: Register, offset: Offset<u8>) => stc_imm_pre;
        emit_stc_imm_pre_cond(cond: Condition, rn: Register, offset: Offset<u8>) => stc_imm_pre::cond;
        emit_stc_imm_unindexed(rn: Register, imm8: u8) => stc_imm_unindexed;
        emit_stc_imm_unindexed_cond(cond: Condition, rn: Register, imm8: u8) => stc_imm_unindexed::cond;
        emit_stm(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stm;
        emit_stm_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stm::cond;
        emit_stm_user_registers(rn: Register, regs: impl Into<RegisterList>) => stm_user_registers;
        emit_stm_user_registers_cond(cond: Condition, rn: Register, regs: impl Into<RegisterList>) => stm_user_registers::cond;
        emit_stmda(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stmda;
        emit_stmda_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stmda::cond;
        emit_stmda_user_registers(rn: Register, regs: impl Into<RegisterList>) => stmda_user_registers;
        emit_stmda_user_registers_cond(cond: Condition, rn: Register, regs: impl Into<RegisterList>) => stmda_user_registers::cond;
        emit_stmdb(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stmdb;
        emit_stmdb_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stmdb::cond;
        emit_stmdb_user_registers(rn: Register, regs: impl Into<RegisterList>) => stmdb_user_registers;
        emit_stmdb_user_registers_cond(cond: Condition, rn: Register, regs: impl Into<RegisterList>) => stmdb_user_registers::cond;
        emit_stmib(rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stmib;
        emit_stmib_cond(cond: Condition, rn: impl Into<Mem>, regs: impl Into<RegisterList>) => stmib::cond;
        emit_stmib_user_registers(rn: Register, regs: impl Into<RegisterList>) => stmib_user_registers;
        emit_stmib_user_registers_cond(cond: Condition, rn: Register, regs: impl Into<RegisterList>) => stmib_user_registers::cond;
        emit_str_imm_offset(rt: Register, rn: Register, offset12: Offset<u16>) => str_imm_offset;
        emit_str_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => str_imm_offset::cond;
        emit_str_imm_post(rt: Register, rn: Register, offset12: Offset<u16>) => str_imm_post;
        emit_str_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => str_imm_post::cond;
        emit_str_imm_pre(rt: Register, rn: Register, offset12: Offset<u16>) => str_imm_pre;
        emit_str_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => str_imm_pre::cond;
        emit_str_reg_offset(rt: Register, rn: Register, index: impl Into<Index>) => str_reg_offset;
        emit_str_reg_offset_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => str_reg_offset::cond;
        emit_str_reg_post(rt: Register, rn: Register, index: impl Into<Index>) => str_reg_post;
        emit_str_reg_post_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => str_reg_post::cond;
        emit_str_reg_pre(rt: Register, rn: Register, index: impl Into<Index>) => str_reg_pre;
        emit_str_reg_pre_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => str_reg_pre::cond;
        emit_strb_imm_offset(rt: Register, rn: Register, offset12: Offset<u16>) => strb_imm_offset;
        emit_strb_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => strb_imm_offset::cond;
        emit_strb_imm_post(rt: Register, rn: Register, offset12: Offset<u16>) => strb_imm_post;
        emit_strb_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => strb_imm_post::cond;
        emit_strb_imm_pre(rt: Register, rn: Register, offset12: Offset<u16>) => strb_imm_pre;
        emit_strb_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => strb_imm_pre::cond;
        emit_strb_reg_offset(rt: Register, rn: Register, index: impl Into<Index>) => strb_reg_offset;
        emit_strb_reg_offset_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => strb_reg_offset::cond;
        emit_strb_reg_post(rt: Register, rn: Register, index: impl Into<Index>) => strb_reg_post;
        emit_strb_reg_post_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => strb_reg_post::cond;
        emit_strb_reg_pre(rt: Register, rn: Register, index: impl Into<Index>) => strb_reg_pre;
        emit_strb_reg_pre_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => strb_reg_pre::cond;
        emit_strbt_imm_post(rt: Register, rn: Register, offset12: Offset<u16>) => strbt_imm_post;
        emit_strbt_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => strbt_imm_post::cond;
        emit_strbt_reg_post(rt: Register, rn: Register, index: impl Into<Index>) => strbt_reg_post;
        emit_strbt_reg_post_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => strbt_reg_post::cond;
        emit_strd_imm_offset(rt: RegisterPair, rn: Register, offset: Offset<u8>) => strd_imm_offset;
        emit_strd_imm_offset_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<u8>) => strd_imm_offset::cond;
        emit_strd_imm_post(rt: RegisterPair, rn: Register, offset: Offset<u8>) => strd_imm_post;
        emit_strd_imm_post_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<u8>) => strd_imm_post::cond;
        emit_strd_imm_pre(rt: RegisterPair, rn: Register, offset: Offset<u8>) => strd_imm_pre;
        emit_strd_imm_pre_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<u8>) => strd_imm_pre::cond;
        emit_strd_reg_offset(rt: RegisterPair, rn: Register, offset: Offset<Register>) => strd_reg_offset;
        emit_strd_reg_offset_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<Register>) => strd_reg_offset::cond;
        emit_strd_reg_post(rt: RegisterPair, rn: Register, offset: Offset<Register>) => strd_reg_post;
        emit_strd_reg_post_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<Register>) => strd_reg_post::cond;
        emit_strd_reg_pre(rt: RegisterPair, rn: Register, offset: Offset<Register>) => strd_reg_pre;
        emit_strd_reg_pre_cond(cond: Condition, rt: RegisterPair, rn: Register, offset: Offset<Register>) => strd_reg_pre::cond;
        emit_strex(rd: Register, rt: Register, rn: Register) => strex;
        emit_strex_cond(cond: Condition, rd: Register, rt: Register, rn: Register) => strex::cond;
        emit_strexb(rd: Register, rt: Register, rn: Register) => strexb;
        emit_strexb_cond(cond: Condition, rd: Register, rt: Register, rn: Register) => strexb::cond;
        emit_strexd(rd: Register, rt: RegisterPair, rn: Register) => strexd;
        emit_strexd_cond(cond: Condition, rd: Register, rt: RegisterPair, rn: Register) => strexd::cond;
        emit_strexh(rd: Register, rt: Register, rn: Register) => strexh;
        emit_strexh_cond(cond: Condition, rd: Register, rt: Register, rn: Register) => strexh::cond;
        emit_strh_imm_offset(rt: Register, rn: Register, offset: Offset<u8>) => strh_imm_offset;
        emit_strh_imm_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => strh_imm_offset::cond;
        emit_strh_imm_post(rt: Register, rn: Register, offset: Offset<u8>) => strh_imm_post;
        emit_strh_imm_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => strh_imm_post::cond;
        emit_strh_imm_pre(rt: Register, rn: Register, offset: Offset<u8>) => strh_imm_pre;
        emit_strh_imm_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => strh_imm_pre::cond;
        emit_strh_reg_offset(rt: Register, rn: Register, offset: Offset<Register>) => strh_reg_offset;
        emit_strh_reg_offset_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => strh_reg_offset::cond;
        emit_strh_reg_post(rt: Register, rn: Register, offset: Offset<Register>) => strh_reg_post;
        emit_strh_reg_post_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => strh_reg_post::cond;
        emit_strh_reg_pre(rt: Register, rn: Register, offset: Offset<Register>) => strh_reg_pre;
        emit_strh_reg_pre_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => strh_reg_pre::cond;
        emit_strht_imm(rt: Register, rn: Register, offset: Offset<u8>) => strht_imm;
        emit_strht_imm_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<u8>) => strht_imm::cond;
        emit_strht_reg(rt: Register, rn: Register, offset: Offset<Register>) => strht_reg;
        emit_strht_reg_cond(cond: Condition, rt: Register, rn: Register, offset: Offset<Register>) => strht_reg::cond;
        emit_strt_imm(rt: Register, rn: Register, offset12: Offset<u16>) => strt_imm;
        emit_strt_imm_cond(cond: Condition, rt: Register, rn: Register, offset12: Offset<u16>) => strt_imm::cond;
        emit_strt_reg(rt: Register, rn: Register, index: impl Into<Index>) => strt_reg;
        emit_strt_reg_cond(cond: Condition, rt: Register, rn: Register, index: impl Into<Index>) => strt_reg::cond;
        emit_sub_imm(rd: Register, rn: Register, imm: impl Into<Const>) => sub_imm;
        emit_sub_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => sub_imm::cond;
        emit_sub_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => sub_reg;
        emit_sub_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => sub_reg::cond;
        emit_sub_reg_rrx(rd: Register, rn: Register, rm: Register) => sub_reg_rrx;
        emit_sub_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => sub_reg_rrx::cond;
        emit_sub_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => sub_reg_shifted_reg;
        emit_sub_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => sub_reg_shifted_reg::cond;
        emit_subs_imm(rd: Register, rn: Register, imm: impl Into<Const>) => subs_imm;
        emit_subs_imm_cond(cond: Condition, rd: Register, rn: Register, imm: impl Into<Const>) => subs_imm::cond;
        emit_subs_reg(rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => subs_reg;
        emit_subs_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<u8>) => subs_reg::cond;
        emit_subs_reg_rrx(rd: Register, rn: Register, rm: Register) => subs_reg_rrx;
        emit_subs_reg_rrx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => subs_reg_rrx::cond;
        emit_subs_reg_shifted_reg(rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => subs_reg_shifted_reg;
        emit_subs_reg_shifted_reg_cond(cond: Condition, rd: Register, rn: Register, rm: Register, shift: Shift<Register>) => subs_reg_shifted_reg::cond;
        emit_svc(imm24: u32) => svc;
        emit_svc_cond(cond: Condition, imm24: u32) => svc::cond;
        emit_sxtab(rd: Register, rn: Register, rm: Register, ror: Ror) => sxtab;
        emit_sxtab_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ror: Ror) => sxtab::cond;
        emit_sxtab16(rd: Register, rn: Register, rm: Register, ror: Ror) => sxtab16;
        emit_sxtab16_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ror: Ror) => sxtab16::cond;
        emit_sxtah(rd: Register, rn: Register, rm: Register, ror: Ror) => sxtah;
        emit_sxtah_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ror: Ror) => sxtah::cond;
        emit_sxtb(rd: Register, rm: Register, ror: Ror) => sxtb;
        emit_sxtb_cond(cond: Condition, rd: Register, rm: Register, ror: Ror) => sxtb::cond;
        emit_sxtb16(rd: Register, rm: Register, ror: Ror) => sxtb16;
        emit_sxtb16_cond(cond: Condition, rd: Register, rm: Register, ror: Ror) => sxtb16::cond;
        emit_sxth(rd: Register, rm: Register, ror: Ror) => sxth;
        emit_sxth_cond(cond: Condition, rd: Register, rm: Register, ror: Ror) => sxth::cond;
        emit_teq_imm(rn: Register, imm: impl Into<Const>) => teq_imm;
        emit_teq_imm_cond(cond: Condition, rn: Register, imm: impl Into<Const>) => teq_imm::cond;
        emit_teq_reg(rn: Register, rm: Register, shift: Shift<u8>) => teq_reg;
        emit_teq_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<u8>) => teq_reg::cond;
        emit_teq_reg_rrx(rn: Register, rm: Register) => teq_reg_rrx;
        emit_teq_reg_rrx_cond(cond: Condition, rn: Register, rm: Register) => teq_reg_rrx::cond;
        emit_teq_reg_shifted_reg(rn: Register, rm: Register, shift: Shift<Register>) => teq_reg_shifted_reg;
        emit_teq_reg_shifted_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<Register>) => teq_reg_shifted_reg::cond;
        emit_tst_imm(rn: Register, imm: impl Into<Const>) => tst_imm;
        emit_tst_imm_cond(cond: Condition, rn: Register, imm: impl Into<Const>) => tst_imm::cond;
        emit_tst_reg(rn: Register, rm: Register, shift: Shift<u8>) => tst_reg;
        emit_tst_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<u8>) => tst_reg::cond;
        emit_tst_reg_rrx(rn: Register, rm: Register) => tst_reg_rrx;
        emit_tst_reg_rrx_cond(cond: Condition, rn: Register, rm: Register) => tst_reg_rrx::cond;
        emit_tst_reg_shifted_reg(rn: Register, rm: Register, shift: Shift<Register>) => tst_reg_shifted_reg;
        emit_tst_reg_shifted_reg_cond(cond: Condition, rn: Register, rm: Register, shift: Shift<Register>) => tst_reg_shifted_reg::cond;
        emit_uadd8(rd: Register, rn: Register, rm: Register) => uadd8;
        emit_uadd8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uadd8::cond;
        emit_uadd16(rd: Register, rn: Register, rm: Register) => uadd16;
        emit_uadd16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uadd16::cond;
        emit_uasx(rd: Register, rn: Register, rm: Register) => uasx;
        emit_uasx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uasx::cond;
        emit_ubfx(rd: Register, rn: Register, lsb: u8, width: u8) => ubfx;
        emit_ubfx_cond(cond: Condition, rd: Register, rn: Register, lsb: u8, width: u8) => ubfx::cond;
        emit_udf(imm16: u16) => udf;
        emit_udiv(rd: Register, rn: Register, rm: Register) => udiv;
        emit_udiv_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => udiv::cond;
        emit_uhadd8(rd: Register, rn: Register, rm: Register) => uhadd8;
        emit_uhadd8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uhadd8::cond;
        emit_uhadd16(rd: Register, rn: Register, rm: Register) => uhadd16;
        emit_uhadd16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uhadd16::cond;
        emit_uhasx(rd: Register, rn: Register, rm: Register) => uhasx;
        emit_uhasx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uhasx::cond;
        emit_uhsax(rd: Register, rn: Register, rm: Register) => uhsax;
        emit_uhsax_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uhsax::cond;
        emit_uhsub8(rd: Register, rn: Register, rm: Register) => uhsub8;
        emit_uhsub8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uhsub8::cond;
        emit_uhsub16(rd: Register, rn: Register, rm: Register) => uhsub16;
        emit_uhsub16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uhsub16::cond;
        emit_umaal(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umaal;
        emit_umaal_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umaal::cond;
        emit_umlal(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umlal;
        emit_umlal_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umlal::cond;
        emit_umlals(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umlals;
        emit_umlals_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umlals::cond;
        emit_umull(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umull;
        emit_umull_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umull::cond;
        emit_umulls(rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umulls;
        emit_umulls_cond(cond: Condition, rdlo: Register, rdhi: Register, rn: Register, rm: Register) => umulls::cond;
        emit_uqadd8(rd: Register, rn: Register, rm: Register) => uqadd8;
        emit_uqadd8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uqadd8::cond;
        emit_uqadd16(rd: Register, rn: Register, rm: Register) => uqadd16;
        emit_uqadd16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uqadd16::cond;
        emit_uqasx(rd: Register, rn: Register, rm: Register) => uqasx;
        emit_uqasx_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uqasx::cond;
        emit_uqsax(rd: Register, rn: Register, rm: Register) => uqsax;
        emit_uqsax_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uqsax::cond;
        emit_uqsub8(rd: Register, rn: Register, rm: Register) => uqsub8;
        emit_uqsub8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uqsub8::cond;
        emit_uqsub16(rd: Register, rn: Register, rm: Register) => uqsub16;
        emit_uqsub16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => uqsub16::cond;
        emit_usad8(rd: Register, rn: Register, rm: Register) => usad8;
        emit_usad8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => usad8::cond;
        emit_usada8(rd: Register, rn: Register, rm: Register, ra: Register) => usada8;
        emit_usada8_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ra: Register) => usada8::cond;
        emit_usat_asr(rd: Register, sat_imm: u8, rn: Register, imm5: u8) => usat_asr;
        emit_usat_asr_cond(cond: Condition, rd: Register, sat_imm: u8, rn: Register, imm5: u8) => usat_asr::cond;
        emit_usat_lsl(rd: Register, sat_imm: u8, rn: Register, imm5: u8) => usat_lsl;
        emit_usat_lsl_cond(cond: Condition, rd: Register, sat_imm: u8, rn: Register, imm5: u8) => usat_lsl::cond;
        emit_usat16(rd: Register, sat_imm: u8, rn: Register) => usat16;
        emit_usat16_cond(cond: Condition, rd: Register, sat_imm: u8, rn: Register) => usat16::cond;
        emit_usax(rd: Register, rn: Register, rm: Register) => usax;
        emit_usax_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => usax::cond;
        emit_usub8(rd: Register, rn: Register, rm: Register) => usub8;
        emit_usub8_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => usub8::cond;
        emit_usub16(rd: Register, rn: Register, rm: Register) => usub16;
        emit_usub16_cond(cond: Condition, rd: Register, rn: Register, rm: Register) => usub16::cond;
        emit_uxtab(rd: Register, rn: Register, rm: Register, ror: Ror) => uxtab;
        emit_uxtab_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ror: Ror) => uxtab::cond;
        emit_uxtab16(rd: Register, rn: Register, rm: Register, ror: Ror) => uxtab16;
        emit_uxtab16_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ror: Ror) => uxtab16::cond;
        emit_uxtah(rd: Register, rn: Register, rm: Register, ror: Ror) => uxtah;
        emit_uxtah_cond(cond: Condition, rd: Register, rn: Register, rm: Register, ror: Ror) => uxtah::cond;
        emit_uxtb(rd: Register, rm: Register, ror: Ror) => uxtb;
        emit_uxtb_cond(cond: Condition, rd: Register, rm: Register, ror: Ror) => uxtb::cond;
        emit_uxtb16(rd: Register, rm: Register, ror: Ror) => uxtb16;
        emit_uxtb16_cond(cond: Condition, rd: Register, rm: Register, ror: Ror) => uxtb16::cond;
        emit_uxth(rd: Register, rm: Register, ror: Ror) => uxth;
        emit_uxth_cond(cond: Condition, rd: Register, rm: Register, ror: Ror) => uxth::cond;
        emit_wfe() => wfe;
        emit_wfe_cond(cond: Condition) => wfe::cond;
        emit_wfi() => wfi;
        emit_wfi_cond(cond: Condition) => wfi::cond;
        emit_yield_() => yield_;
        emit_yield_cond(cond: Condition) => yield_::cond;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


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
