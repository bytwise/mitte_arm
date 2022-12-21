extern crate mitte_arm;
extern crate capstone;

use std::convert::TryInto;

use mitte_arm::aarch64::*;
use mitte_arm::aarch64::Register32Common::*;
use mitte_arm::aarch64::Register64Common::*;

use capstone::Capstone;
use capstone::arch::BuildsCapstone;
use capstone::arch::arm64::ArchMode;


#[macro_use]
mod common;

use common::TestCases;


fn print_code(code: &[u32]) {
    print!("[");
    for instruction in code {
        print!("{:08x}", instruction);
    }
    println!("]");
}


#[track_caller]
fn test_disasm<S>(mnemonic: &str, expected: S, code: u32) where S: AsRef<str> {
    println!("code: {:02x?}", code.to_le_bytes());
    let capstone = Capstone::new().arm64().mode(ArchMode::Arm).build().unwrap();
    let disasm = match capstone.disasm_all(&code.to_le_bytes(), 0x0) {
        Ok(disasm) => disasm,
        Err(error) => {
            print_code(std::slice::from_ref(&code));
            panic!("failed to disassemble: {:?}", error);
        }
    };

    for (i, e) in disasm.iter().zip(std::iter::once(expected)) {
        println!("disasm: {:?} {:?}", i.mnemonic(), i.op_str());
        assert_eq!(mnemonic, i.mnemonic().unwrap(),
            "{} {:?} != {} {:?}",
            mnemonic, e.as_ref(),
            i.mnemonic().unwrap(), i.op_str().unwrap());
        assert_eq!(Some(e.as_ref()), i.op_str(),
            "{} {:?} != {0} {:?}", mnemonic,
            e.as_ref(), i.op_str().unwrap());
    }
    assert_eq!(disasm.len() as usize, 1);
}


impl TestCases for Register32 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (W0, "w0".into()),
            (W3, "w3".into()),
            (W4, "w4".into()),
            (W5, "w5".into()),
            (W29, "w29".into()),
            (W30, "w30".into()),
            (WZR, "wzr".into()),
        ]
    }
}

impl TestCases for Register32OrSp {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (W0, "w0".into()),
            (W3, "w3".into()),
            (W4, "w4".into()),
            (W5, "w5".into()),
            (W29, "w29".into()),
            (W30, "w30".into()),
            (WSP, "wsp".into()),
        ]
    }
}

impl TestCases for Register64 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (X0, "x0".into()),
            (X3, "x3".into()),
            (X4, "x4".into()),
            (X5, "x5".into()),
            (X29, "x29".into()),
            (X30, "x30".into()),
            (XZR, "xzr".into()),
        ]
    }
}

impl TestCases for Register64OrSp {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (X0, "x0".into()),
            (X3, "x3".into()),
            (X4, "x4".into()),
            (X5, "x5".into()),
            (X29, "x29".into()),
            (X30, "x30".into()),
            (XSP, "sp".into()),
        ]
    }
}

impl TestCases for Label {
    fn test_cases() -> Vec<(Self, String)> {
        let mut test_cases = vec![
            (Label::from_byte_offset(0), "#0".into()),
            (Label::from_byte_offset(1), "#1".into()),
            (Label::from_byte_offset(2), "#2".into()),
            (Label::from_byte_offset(4), "#4".into()),
        ];
        for n in 13..40 {
            let x = 1 << n;
            test_cases.extend_from_slice(&[
                (Label::from_byte_offset(x - 1), format!("#0x{:x}", x - 1)),
                (Label::from_byte_offset((x - 1) & !0xfff), format!("#0x{:x}", (x - 1) & !0xfff)),
                (Label::from_byte_offset(-x), format!("#0x{:x}", (-x) as u64)),
            ])
        }
        test_cases
    }
}

impl TestCases for ShiftedImm12 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (ShiftedImm12::from(0), "#0".into()),
            (ShiftedImm12::from(5), "#5".into()),
            (ShiftedImm12::from(6), "#6".into()),
            (ShiftedImm12::lsl_12(0), "#0, lsl #12".into()),
            (ShiftedImm12::lsl_12(5), "#5, lsl #12".into()),
            (ShiftedImm12::lsl_12(6), "#6, lsl #12".into()),
        ]
    }
}

impl TestCases for MovImm32 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (MovImm32::from(0), "#0".into()),
            (MovImm32::from(5), "#5".into()),
            (MovImm32::from(6), "#6".into()),
            (MovImm32::lsl_16(0), "#0, lsl #16".into()),
            (MovImm32::lsl_16(5), "#5, lsl #16".into()),
            (MovImm32::lsl_16(6), "#6, lsl #16".into()),
        ]
    }
}

impl TestCases for MovImm64 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (MovImm64::from(0), "#0".into()),
            (MovImm64::from(5), "#5".into()),
            (MovImm64::from(6), "#6".into()),
            (MovImm64::lsl_16(0), "#0, lsl #16".into()),
            (MovImm64::lsl_16(5), "#5, lsl #16".into()),
            (MovImm64::lsl_16(6), "#6, lsl #16".into()),
            (MovImm64::lsl_32(0), "#0, lsl #32".into()),
            (MovImm64::lsl_32(5), "#5, lsl #32".into()),
            (MovImm64::lsl_32(6), "#6, lsl #32".into()),
            (MovImm64::lsl_48(0), "#0, lsl #48".into()),
            (MovImm64::lsl_48(5), "#5, lsl #48".into()),
            (MovImm64::lsl_48(6), "#6, lsl #48".into()),
        ]
    }
}

impl TestCases for Bitmask32 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0x5555_5555.try_into().unwrap(), "#0x55555555".into()),
            (0x0100_0100.try_into().unwrap(), "#0x1000100".into()),
            (1.try_into().unwrap(), "#1".into()),
        ]
    }
}

impl TestCases for Bitmask64 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0x5555_5555_5555_5555.try_into().unwrap(), "#0x5555555555555555".into()),
            (0x0100_0100_0100_0100.try_into().unwrap(), "#0x100010001000100".into()),
            (1.try_into().unwrap(), "#1".into()),
        ]
    }
}

impl TestCases for Index<u16> {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Index::from_byte_offset(0), "#0".into()),
            (Index::from_byte_offset(6), "#6".into()),
            (Index::from_byte_offset(-6), "#-6".into()),
        ]
    }
}

impl TestCases for Index<u32> {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Index::from_byte_offset(0), "#0".into()),
            (Index::from_byte_offset(0xc), "#0xc".into()),
            (Index::from_byte_offset(-0xc), "#-0xc".into()),
        ]
    }
}

impl TestCases for Index<u64> {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Index::from_byte_offset(0), "#0".into()),
            (Index::from_byte_offset(0x18), "#0x18".into()),
            (Index::from_byte_offset(-0x18), "#-0x18".into()),
        ]
    }
}

impl TestCases for Extend {
    fn test_cases() -> Vec<(Self, String)> {
        vec![(Extend::Uxth(2), "uxth #2".into())]
    }
}


pub fn is_signed_nbit_integer(n: u8, value: impl Into<i64>) -> bool {
    let value = value.into();
    ((-1) << (n - 1)) <= value && value < (1 << (n - 1))
}


#[track_caller]
fn test1_filter<A1>(
    mnemonic: &str,
    f: fn(A1) -> u32,
    mut filter: impl FnMut(A1) -> bool
)
    where A1: TestCases
{
    for (a1, s1) in A1::test_cases() {
        if filter(a1) {
            test_disasm(mnemonic,
                        String::from(s1),
                        f(a1));
        }
    }
}

#[track_caller]
fn test1<A1>(mnemonic: &str, f: fn(A1) -> u32)
    where A1: TestCases
{
    test1_filter(mnemonic, f, |_| true);
}

#[track_caller]
fn test2_format_filter<A1, A2>(
    mnemonic: &str,
    f: fn(A1, A2) -> u32,
    s: impl Fn(&str, &str) -> String,
    mut filter: impl FnMut(A1, A2) -> bool
)
    where A1: TestCases, A2: TestCases
{
    for (a1, s1) in A1::test_cases() {
        for (a2, s2) in A2::test_cases() {
            if filter(a1, a2) {
                test_disasm(mnemonic, s(&s1, &s2), f(a1, a2));
            }
        }
    }
}

#[track_caller]
fn test2_filter<A1, A2>(
    mnemonic: &str,
    f: fn(A1, A2) -> u32,
    filter: impl FnMut(A1, A2) -> bool
)
    where A1: TestCases, A2: TestCases
{
    test2_format_filter(mnemonic, f,
        |s1, s2| format!("{}, {}", s1, s2),
        filter)
}

#[track_caller]
fn test2<A1, A2>(mnemonic: &str, f: fn(A1, A2) -> u32)
    where A1: TestCases, A2: TestCases
{
    test2_filter(mnemonic, f, |_, _| true);
}

#[track_caller]
fn test3_format_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    s: impl Fn(&str, &str, &str) -> String,
    mut filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    for (a1, s1) in A1::test_cases() {
        for (a2, s2) in A2::test_cases() {
            for (a3, s3) in A3::test_cases() {
                if filter(a1, a2, a3) {
                    test_disasm(mnemonic, s(&s1, &s2, &s3), f(a1, a2, a3));
                }
            }
        }
    }
}

#[track_caller]
fn test3_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(mnemonic, f,
        |s1, s2, s3| format!("{}, {}, {}", s1, s2, s3),
        filter)
}

#[track_caller]
fn test3<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_filter(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test4_format_filter<A1, A2, A3, A4>(
    mnemonic: &str,
    f: fn(A1, A2, A3, A4) -> u32,
    s: impl Fn(&str, &str, &str, &str) -> String,
    mut filter: impl FnMut(A1, A2, A3, A4) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    for (a1, s1) in A1::test_cases() {
        for (a2, s2) in A2::test_cases() {
            for (a3, s3) in A3::test_cases() {
                for (a4, s4) in A4::test_cases() {
                    if filter(a1, a2, a3, a4) {
                        test_disasm(mnemonic, s(&s1, &s2, &s3, &s4), f(a1, a2, a3, a4));
                    }
                }
            }
        }
    }
}

#[track_caller]
fn test4_filter<A1, A2, A3, A4>(
    mnemonic: &str,
    f: fn(A1, A2, A3, A4) -> u32,
    filter: impl FnMut(A1, A2, A3, A4) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter(mnemonic, f,
        |s1, s2, s3, s4| format!("{}, {}, {}, {}", s1, s2, s3, s4),
        filter)
}

#[track_caller]
fn test4<A1, A2, A3, A4>(mnemonic: &str, f: fn(A1, A2, A3, A4) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_filter(mnemonic, f, |_, _, _, _| true);
}


#[track_caller]
fn test_ldst_filter<A1, A2>(
    mnemonic: &str,
    f: fn(A1, A2) -> u32,
    filter: impl FnMut(A1, A2) -> bool
)
    where A1: TestCases, A2: TestCases
{
    test2_format_filter(mnemonic, f,
        |s1, s2| format!("{}, [{}]", s1, s2),
        filter);
}

#[track_caller]
fn test_ldst<A1, A2>(mnemonic: &str, f: fn(A1, A2) -> u32)
    where A1: TestCases, A2: TestCases
{
    test_ldst_filter(mnemonic, f, |_, _| true);
}

#[track_caller]
fn test_ldst_offset_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(mnemonic, f,
        |s1, s2, s3| format!("{}, [{}, {}]", s1, s2, s3),
        filter);
}

#[track_caller]
fn test_ldst_offset<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test_ldst_offset_filter(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test_ldst_post_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(mnemonic, f,
        |s1, s2, s3| format!("{}, [{}], {}", s1, s2, s3),
        filter);
}

#[track_caller]
fn test_ldst_pre_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(mnemonic, f,
        |s1, s2, s3| format!("{}, [{}, {}]!", s1, s2, s3),
        filter);
}

#[track_caller]
fn test_ldst_pre<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test_ldst_pre_filter(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test_ldstpair_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(mnemonic, f,
        |s1, s2, s3| format!("{}, {}, [{}]", s1, s2, s3),
        filter);
}

#[track_caller]
fn test_ldstpair<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test_ldstpair_filter(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test_ldstpair_offset_filter<A1, A2, A3, A4>(
    mnemonic: &str,
    f: fn(A1, A2, A3, A4) -> u32,
    filter: impl FnMut(A1, A2, A3, A4) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter(mnemonic, f,
        |s1, s2, s3, s4| format!("{}, {}, [{}, {}]", s1, s2, s3, s4),
        filter);
}

#[track_caller]
fn test_ldstpair_post_filter<A1, A2, A3, A4>(
    mnemonic: &str,
    f: fn(A1, A2, A3, A4) -> u32,
    filter: impl FnMut(A1, A2, A3, A4) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter(mnemonic, f,
        |s1, s2, s3, s4| format!("{}, {}, [{}], {}", s1, s2, s3, s4),
        filter);
}

#[track_caller]
fn test_ldstpair_pre_filter<A1, A2, A3, A4>(
    mnemonic: &str,
    f: fn(A1, A2, A3, A4) -> u32,
    filter: impl FnMut(A1, A2, A3, A4) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter(mnemonic, f,
        |s1, s2, s3, s4| format!("{}, {}, [{}, {}]!", s1, s2, s3, s4),
        filter);
}

fn test_xdsp_xnsp_xm_extend(mnemonic: &str, f: fn(Register64OrSp, Register64OrSp, Register64, Extend) -> u32) {
    test_disasm(mnemonic, "x3, x4, w5, uxth #2", f(X3, X4, X5, Extend::Uxth(2)));
    test_disasm(mnemonic, "sp, x4, w5, uxth #2", f(XSP, X4, X5, Extend::Uxth(2)));
    test_disasm(mnemonic, "x3, sp, w5, uxth #2", f(X3, XSP, X5, Extend::Uxth(2)));
    test_disasm(mnemonic, "x3, x4, wzr, uxth #2", f(X3, X4, XZR, Extend::Uxth(2)));
}

fn test_xnsp_xm_ext(mnemonic: &str, f: fn(Register64OrSp, Register64, Extend) -> u32) {
    test_disasm(mnemonic, "x3, w4, uxth #2", f(X3, X4, Extend::Uxth(2)));
    test_disasm(mnemonic, "sp, w4, uxth #2", f(XSP, X4, Extend::Uxth(2)));
    test_disasm(mnemonic, "x3, wzr, uxth #2", f(X3, XZR, Extend::Uxth(2)));
}


#[test]
fn test_adc_32() {
    test3("adc", a64::adc_32);
}

#[test]
fn test_adc_64() {
    test3("adc", a64::adc_64);
}

#[test]
fn test_adcs_32() {
    test3("adcs", a64::adcs_32);
}

#[test]
fn test_adcs_64() {
    test3("adcs", a64::adcs_64);
}

#[test]
fn test_add_ext_32() {
    test4("add", a64::add_ext_32);
}

#[test]
fn test_add_ext_64() {
    test_xdsp_xnsp_xm_extend("add", a64::add_ext_64);
}

#[test]
fn test_add_imm_32() {
    test3_filter("add", a64::add_imm_32::<u8>, |_, _, imm| imm != 0);
    test3_filter("add", a64::add_imm_32::<ShiftedImm12>, |_, _, imm| u32::from(imm) != 0);
}

#[test]
fn test_add_imm_64() {
    test3_filter("add", a64::add_imm_64::<u8>, |_, _, imm| imm != 0);
    test3_filter("add", a64::add_imm_64::<ShiftedImm12>, |_, _, imm| u32::from(imm) != 0);
}

#[test]
fn test_add_shift_32() {
    test4_filter("add", a64::add_shift_32, |_, _, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_add_shift_64() {
    test4_filter("add", a64::add_shift_64, |_, _, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_adds_ext_32() {
    test4_filter("adds", a64::adds_ext_32, |a1, _, _, _| a1 != WZR);
}

#[test]
fn test_adds_ext_64() {
    test_disasm("adds", "x3, x4, w5, uxth #2", a64::adds_ext_64(X3, X4, X5, Extend::Uxth(2)));
    test_disasm("cmn", "x4, w5, uxth #2", a64::adds_ext_64(XZR, X4, X5, Extend::Uxth(2)));
    test_disasm("adds", "x3, sp, w5, uxth #2", a64::adds_ext_64(X3, XSP, X5, Extend::Uxth(2)));
    test_disasm("adds", "x3, x4, wzr, uxth #2", a64::adds_ext_64(X3, X4, XZR, Extend::Uxth(2)));
}

#[test]
fn test_adds_imm_32() {
    test3_filter("adds", a64::adds_imm_32::<u8>, |a1, _, _| a1 != WZR);
    test3_filter("adds", a64::adds_imm_32::<ShiftedImm12>, |a1, _, _| a1 != WZR);
}

#[test]
fn test_adds_imm_64() {
    test3_filter("adds", a64::adds_imm_64::<u8>, |a1, _, _| a1 != XZR);
    test3_filter("adds", a64::adds_imm_64::<ShiftedImm12>, |a1, _, _| a1 != XZR);
}

#[test]
fn test_adds_shift_32() {
    test4_filter("adds", a64::adds_shift_32, |a1, _, _, shift| {
        a1 != WZR && !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_adds_shift_64() {
    test4_filter("adds", a64::adds_shift_64, |a1, _, _, shift| {
        a1 != XZR && !matches!(shift, Shift::Ror(_))
    });
    test3_filter("cmn", |rn, rm, shift| a64::adds_shift_64(XZR, rn, rm, shift), |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_adr() {
    test2_filter("adr", a64::adr, |_, label| {
        is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_adrp() {
    test2_filter("adrp", a64::adrp, |_, label| {
        label.byte_offset() & 0xfff == 0 && is_signed_nbit_integer(33, label.byte_offset())
    });
}

#[test]
fn test_and_imm_32() {
    test3("and", a64::and_imm_32);
}

#[test]
fn test_and_imm_64() {
    test3("and", a64::and_imm_64);
}

#[test]
fn test_and_reg_32() {
    test4("and", a64::and_reg_32);
}

#[test]
fn test_and_reg_64() {
    test4("and", a64::and_reg_64);
}

#[test]
fn test_ands_imm_32() {
    test3_filter("ands", a64::ands_imm_32, |a1, _, _| a1 != WZR);
}

#[test]
fn test_ands_imm_64() {
    test3_filter("ands", a64::ands_imm_64, |a1, _, _| a1 != XZR);
}

#[test]
fn test_ands_reg_32() {
    test4_filter("ands", a64::ands_reg_32, |a1, _, _, _| a1 != WZR);
}

#[test]
fn test_ands_reg_64() {
    test4_filter("ands", a64::ands_reg_64, |a1, _, _, _| a1 != XZR);
}

#[test]
fn test_asr_imm_32() {
    test3_filter("asr", a64::asr_imm_32, |_, _, shift| shift <= 31);
}

#[test]
fn test_asr_imm_64() {
    test3_filter("asr", a64::asr_imm_64, |_, _, shift| shift <= 63);
}

#[test]
fn test_asr_reg_32() {
    test3("asr", a64::asr_reg_32);
}

#[test]
fn test_asr_reg_64() {
    test3("asr", a64::asr_reg_64);
}

#[test]
fn test_asrv_32() {
    test3("asr", a64::asrv_32);
}

#[test]
fn test_asrv_64() {
    test3("asr", a64::asrv_64);
}

#[test]
fn test_at() {
    test_disasm("at", "s1e1r, x6", a64::at(0, 8, 0, X6));
    test_disasm("at", "s12e1w, x6", a64::at(4, 8, 5, X6));
}

#[test]
fn test_autda() {
    test2("autda", pauth::autda);
}

#[test]
fn test_autdb() {
    test2("autdb", pauth::autdb);
}

#[test]
fn test_autdza() {
    test1("autdza", pauth::autdza);
}

#[test]
fn test_autdzb() {
    test1("autdzb", pauth::autdzb);
}

#[test]
fn test_autia() {
    test2("autia", pauth::autia);
}

#[test]
fn test_autia1716() {
    test_disasm("autia1716", "", pauth::autia1716());
}

#[test]
fn test_autiasp() {
    test_disasm("autiasp", "", pauth::autiasp());
}

#[test]
fn test_autiaz() {
    test_disasm("autiaz", "", pauth::autiaz());
}

#[test]
fn test_autib() {
    test2("autib", pauth::autib);
}

#[test]
fn test_autib1716() {
    test_disasm("autib1716", "", pauth::autib1716());
}

#[test]
fn test_autibsp() {
    test_disasm("autibsp", "", pauth::autibsp());
}

#[test]
fn test_autibz() {
    test_disasm("autibz", "", pauth::autibz());
}

#[test]
fn test_autiza() {
    test1("autiza", pauth::autiza);
}

#[test]
fn test_autizb() {
    test1("autizb", pauth::autizb);
}

#[test]
#[ignore]
fn test_axflag() {
    test_disasm("axflag", "", flagm2::axflag());
}

#[test]
fn test_b() {
    test1_filter("b", a64::b, |label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(28, label.byte_offset())
    });
}

#[test]
fn test_b_cond() {
    test_disasm("b.al", "#0x5678", a64::b_cond(Condition::Al, Label::from_byte_offset(0x5678)));
    test_disasm("b.al", "#0xfffffffffffffff0", a64::b_cond(Condition::Al, Label::from_byte_offset(-0x10)));
    test_disasm("b.ge", "#0x5678", a64::b_cond(Condition::Ge, Label::from_byte_offset(0x5678)));
}

#[test]
#[ignore]
fn test_bc() {
    test_disasm("bc.al", "#0x5678", hbc::bc(Condition::Al, Label::from_byte_offset(0x5678)));
    test_disasm("bc.al", "#0xfffffffffffffff0", hbc::bc(Condition::Al, Label::from_byte_offset(-0x10)));
    test_disasm("bc.ge", "#0x5678", hbc::bc(Condition::Ge, Label::from_byte_offset(0x5678)));
}

#[test]
fn test_bfc_32() {
    test3_filter("bfc", a64::bfc_32, |_, lsb, width| {
        lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_bfc_64() {
    test3_filter("bfc", a64::bfc_64, |_, lsb, width| {
        lsb <= 63 && width > 0 && width <= 64 - lsb
    });
}

#[test]
fn test_bfi_32() {
    test4_filter("bfi", a64::bfi_32, |_, a1, lsb, width| {
        a1 != WZR && lsb > 0 && lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_bfi_64() {
    test4_filter("bfi", a64::bfi_64, |_, a1, lsb, width| {
        a1 != XZR && lsb > 0 && lsb <= 63 && width > 0 && width <= 64 - lsb
    });
}

#[test]
fn test_bfm_32() {
    test_disasm("bfc", "w3, #5, #6", a64::bfm_32(W3, WZR, 0x1b, 5));
    test_disasm("bfi", "w3, w4, #5, #6", a64::bfm_32(W3, W4, 0x1b, 5));
    test_disasm("bfxil", "w3, w4, #5, #6", a64::bfm_32(W3, W4, 5, 10));
}

#[test]
fn test_bfm_64() {
    test_disasm("bfc", "x3, #5, #6", a64::bfm_64(X3, XZR, 0x3b, 5));
    test_disasm("bfi", "x3, x4, #5, #6", a64::bfm_64(X3, X4, 0x3b, 5));
    test_disasm("bfxil", "x3, x4, #5, #6", a64::bfm_64(X3, X4, 5, 10));
}

#[test]
fn test_bfxil_32() {
    test4_filter("bfxil", a64::bfxil_32, |_, _, lsb, width| {
        lsb > 0 && lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_bfxil_64() {
    test4_filter("bfxil", a64::bfxil_64, |_, _, lsb, width| {
        lsb > 0 && lsb <= 63 && width > 0 && width <= 64 - lsb
    });
}

#[test]
fn test_bic_32() {
    test4("bic", a64::bic_32);
}

#[test]
fn test_bic_64() {
    test4("bic", a64::bic_64);
}

#[test]
fn test_bics_32() {
    test4("bics", a64::bics_32);
}

#[test]
fn test_bics_64() {
    test4("bics", a64::bics_64);
}

#[test]
fn test_bl() {
    test1_filter("bl", a64::bl, |label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(28, label.byte_offset())
    });
}

#[test]
fn test_blr() {
    test1("blr", a64::blr);
}

#[test]
fn test_blraa() {
    test2("blraa", pauth::blraa);
}

#[test]
fn test_blraaz() {
    test1("blraaz", pauth::blraaz);
}

#[test]
fn test_blrab() {
    test2("blrab", pauth::blrab);
}

#[test]
fn test_blrabz() {
    test1("blrabz", pauth::blrabz);
}

#[test]
fn test_br() {
    test1("br", a64::br);
}

#[test]
fn test_braa() {
    test2("braa", pauth::braa);
}

#[test]
fn test_braaz() {
    test1("braaz", pauth::braaz);
}

#[test]
fn test_brab() {
    test2("brab", pauth::brab);
}

#[test]
fn test_brabz() {
    test1("brabz", pauth::brabz);
}

#[test]
fn test_brk() {
    test1("brk", a64::brk);
}

#[test]
fn test_bti() {
    test_disasm("hint", "#0x20", bti::bti(BranchTargets::None));
    test_disasm("hint", "#0x22", bti::bti(BranchTargets::C));
    test_disasm("hint", "#0x24", bti::bti(BranchTargets::J));
    test_disasm("hint", "#0x26", bti::bti(BranchTargets::Jc));
}

#[test]
fn test_cas_32() {
    test_ldstpair("cas", lse::cas_32);
}

#[test]
fn test_cas_64() {
    test_ldstpair("cas", lse::cas_64);
}

#[test]
fn test_casa_32() {
    test_ldstpair("casa", lse::casa_32);
}

#[test]
fn test_casa_64() {
    test_ldstpair("casa", lse::casa_64);
}

#[test]
fn test_casab() {
    test_ldstpair("casab", lse::casab);
}

#[test]
fn test_casah() {
    test_ldstpair("casah", lse::casah);
}

#[test]
fn test_casal_32() {
    test_ldstpair("casal", lse::casal_32);
}

#[test]
fn test_casal_64() {
    test_ldstpair("casal", lse::casal_64);
}

#[test]
fn test_casalb() {
    test_ldstpair("casalb", lse::casalb);
}

#[test]
fn test_casalh() {
    test_ldstpair("casalh", lse::casalh);
}

#[test]
fn test_casb() {
    test_ldstpair("casb", lse::casb);
}

#[test]
fn test_cash() {
    test_ldstpair("cash", lse::cash);
}

#[test]
fn test_casl_32() {
    test_ldstpair("casl", lse::casl_32);
}

#[test]
fn test_casl_64() {
    test_ldstpair("casl", lse::casl_64);
}

#[test]
fn test_caslb() {
    test_ldstpair("caslb", lse::caslb);
}

#[test]
fn test_caslh() {
    test_ldstpair("caslh", lse::caslh);
}

#[test]
#[ignore]
fn test_casp_32() {
    test_ldstpair("casp", lse::casp_32);
}

#[test]
#[ignore]
fn test_casp_64() {
    test_ldstpair("casp", lse::casp_64);
}

#[test]
#[ignore]
fn test_caspa_32() {
    test_ldstpair("caspa", lse::caspa_32);
}

#[test]
#[ignore]
fn test_caspa_64() {
    test_ldstpair("caspa", lse::caspa_64);
}

#[test]
#[ignore]
fn test_caspal_32() {
    test_ldstpair("caspal", lse::caspal_32);
}

#[test]
#[ignore]
fn test_caspal_64() {
    test_ldstpair("caspal", lse::caspal_64);
}

#[test]
#[ignore]
fn test_caspl_32() {
    test_ldstpair("caspl", lse::caspl_32);
}

#[test]
#[ignore]
fn test_caspl_64() {
    test_ldstpair("caspl", lse::caspl_64);
}

#[test]
fn test_cbnz_32() {
    test2_filter("cbnz", a64::cbnz_32, |_, label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_cbnz_64() {
    test2_filter("cbnz", a64::cbnz_64, |_, label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_cbz_32() {
    test2_filter("cbz", a64::cbz_32, |_, label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_cbz_64() {
    test2_filter("cbz", a64::cbz_64, |_, label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_ccmn_imm_32() {
    test4_filter("ccmn", a64::ccmn_imm_32, |_, imm5, nzcv, _| imm5 < 0x20 && nzcv < 0x10);
}

#[test]
fn test_ccmn_imm_64() {
    test4_filter("ccmn", a64::ccmn_imm_64, |_, imm5, nzcv, _| imm5 < 0x20 && nzcv < 0x10);
}

#[test]
fn test_ccmn_reg_32() {
    test4_filter("ccmn", a64::ccmn_reg_32, |_, _, nzcv, _| nzcv < 0x10);
}

#[test]
fn test_ccmn_reg_64() {
    test4_filter("ccmn", a64::ccmn_reg_64, |_, _, nzcv, _| nzcv < 0x10);
}

#[test]
fn test_ccmp_imm_32() {
    test4_filter("ccmp", a64::ccmp_imm_32, |_, imm5, nzcv, _| imm5 < 0x20 && nzcv < 0x10);
}

#[test]
fn test_ccmp_imm_64() {
    test4_filter("ccmp", a64::ccmp_imm_64, |_, imm5, nzcv, _| imm5 < 0x20 && nzcv < 0x10);
}

#[test]
fn test_ccmp_reg_32() {
    test4_filter("ccmp", a64::ccmp_reg_32, |_, _, nzcv, _| nzcv < 0x10);
}

#[test]
fn test_ccmp_reg_64() {
    test4_filter("ccmp", a64::ccmp_reg_64, |_, _, nzcv, _| nzcv < 0x10);
}

#[test]
fn test_cfinv() {
    test_disasm("cfinv", "", flagm::cfinv());
}

#[test]
fn test_cfp() {
    test_disasm("sys", "#3, c7, c3, #4, x3", specres::cfp_rctx(X3));
}

#[test]
fn test_cinc_32() {
    test3_filter("cinc", a64::cinc_32, |_, a2, _| a2 != WZR);
}

#[test]
fn test_cinc_64() {
    test3_filter("cinc", a64::cinc_64, |_, a2, _| a2 != XZR);
}

#[test]
fn test_cinv_32() {
    test3_filter("cinv", a64::cinv_32, |_, a2, _| a2 != WZR);
}

#[test]
fn test_cinv_64() {
    test3_filter("cinv", a64::cinv_64, |_, a2, _| a2 != XZR);
}

#[test]
fn test_clrex() {
    test_disasm("clrex", "#5", a64::clrex(5));
}

#[test]
fn test_cls_32() {
    test2("cls", a64::cls_32);
}

#[test]
fn test_cls_64() {
    test2("cls", a64::cls_64);
}

#[test]
fn test_clz_32() {
    test2("clz", a64::clz_32);
}

#[test]
fn test_clz_64() {
    test2("clz", a64::clz_64);
}

#[test]
fn test_cmn_ext_32() {
    test3("cmn", a64::cmn_ext_32);
}

#[test]
fn test_cmn_ext_64() {
    test_xnsp_xm_ext("cmn", a64::cmn_ext_64);
}

#[test]
fn test_cmn_imm_32() {
    test2("cmn", a64::cmn_imm_32::<u8>);
    test2("cmn", a64::cmn_imm_32::<ShiftedImm12>);
}

#[test]
fn test_cmn_imm_64() {
    test2("cmn", a64::cmn_imm_64::<u8>);
    test2("cmn", a64::cmn_imm_64::<ShiftedImm12>);
}

#[test]
fn test_cmn_shift_32() {
    test3_filter("cmn", a64::cmn_shift_32, |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_cmn_shift_64() {
    test3_filter("cmn", a64::cmn_shift_64, |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_cmp_ext_32() {
    test3("cmp", a64::cmp_ext_32);
}

#[test]
fn test_cmp_ext_64() {
    test_xnsp_xm_ext("cmp", a64::cmp_ext_64);
}

#[test]
fn test_cmp_imm_32() {
    test2("cmp", a64::cmp_imm_32::<u8>);
    test2("cmp", a64::cmp_imm_32::<ShiftedImm12>);
}

#[test]
fn test_cmp_imm_64() {
    test2("cmp", a64::cmp_imm_64::<u8>);
    test2("cmp", a64::cmp_imm_64::<ShiftedImm12>);
}

#[test]
fn test_cmp_shift_32() {
    test3_filter("cmp", a64::cmp_shift_32, |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_cmp_shift_64() {
    test3_filter("cmp", a64::cmp_shift_64, |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_cneg_32() {
    test3("cneg", a64::cneg_32);
}

#[test]
fn test_cneg_64() {
    test3("cneg", a64::cneg_64);
}

#[test]
fn test_cpp() {
    test_disasm("sys", "#3, c7, c3, #7, x3", specres::cpp_rctx(X3));
}

#[test]
fn test_crc32b() {
    test3("crc32b", crc32::crc32b);
}

#[test]
fn test_crc32cb() {
    test3("crc32cb", crc32::crc32cb);
}

#[test]
fn test_crc32ch() {
    test3("crc32ch", crc32::crc32ch);
}

#[test]
fn test_crc32cw() {
    test3("crc32cw", crc32::crc32cw);
}

#[test]
fn test_crc32cx() {
    test3("crc32cx", crc32::crc32cx);
}

#[test]
fn test_crc32h() {
    test3("crc32h", crc32::crc32h);
}

#[test]
fn test_crc32w() {
    test3("crc32w", crc32::crc32w);
}

#[test]
fn test_crc32x() {
    test3("crc32x", crc32::crc32x);
}

#[test]
fn test_csdb() {
    test_disasm("csdb", "", a64::csdb());
}

#[test]
fn test_csel_32() {
    test4("csel", a64::csel_32);
}

#[test]
fn test_csel_64() {
    test4("csel", a64::csel_64);
}

#[test]
fn test_cset_32() {
    test2("cset", a64::cset_32);
}

#[test]
fn test_cset_64() {
    test2("cset", a64::cset_64);
}

#[test]
fn test_csetm_32() {
    test2("csetm", a64::csetm_32);
}

#[test]
fn test_csetm_64() {
    test2("csetm", a64::csetm_64);
}

#[test]
fn test_csinc_32() {
    test4_filter("csinc", a64::csinc_32, |_, a2, a3, _| a2 != a3);
}

#[test]
fn test_csinc_64() {
    test4_filter("csinc", a64::csinc_64, |_, a2, a3, _| a2 != a3);
}

#[test]
fn test_csinv_32() {
    test4_filter("csinv", a64::csinv_32, |_, a2, a3, _| a2 != a3);
}

#[test]
fn test_csinv_64() {
    test4_filter("csinv", a64::csinv_64, |_, a2, a3, _| a2 != a3);
}

#[test]
fn test_csneg_32() {
    test4_filter("csneg", a64::csneg_32, |_, a2, a3, _| a2 != a3);
}

#[test]
fn test_csneg_64() {
    test4_filter("csneg", a64::csneg_64, |_, a2, a3, _| a2 != a3);
}

#[test]
fn test_dc() {
    test_disasm("dc", "isw, x6", a64::dc(0, 6, 2, X6));
    test_disasm("dc", "cvau, x6", a64::dc(3, 11, 1, X6));
}

#[test]
fn test_dcps1() {
    test1_filter("dcps1", a64::dcps1, |imm| imm != 0);
}

#[test]
fn test_dcps2() {
    test1_filter("dcps2", a64::dcps2, |imm| imm != 0);
}

#[test]
fn test_dcps3() {
    test1_filter("dcps3", a64::dcps3, |imm| imm != 0);
}

#[test]
fn test_dgh() {
    test_disasm("hint", "#6", dgh::dgh());
}

#[test]
fn test_dmb() {
    test_disasm("dmb", "#0", a64::dmb(0));
    test_disasm("dmb", "oshld", a64::dmb(1));
    test_disasm("dmb", "ishst", a64::dmb(10));
    test_disasm("dmb", "sy", a64::dmb(15));
}

#[test]
fn test_drps() {
    test_disasm("drps", "", a64::drps());
}

#[test]
fn test_dsb() {
    test_disasm("dsb", "#0", a64::dsb(0));
    test_disasm("dsb", "oshld", a64::dsb(1));
    test_disasm("dsb", "ishst", a64::dsb(10));
    test_disasm("dsb", "sy", a64::dsb(15));
}

#[test]
fn test_dvp_rctx() {
    test_disasm("sys", "#3, c7, c3, #5, x3", specres::dvp_rctx(X3));
}

#[test]
fn test_eon_32() {
    test4("eon", a64::eon_32);
}

#[test]
fn test_eon_64() {
    test4("eon", a64::eon_64);
}

#[test]
fn test_eor_32() {
    test3("eor", a64::eor_imm_32);
    test4("eor", a64::eor_reg_32);
}

#[test]
fn test_eor_64() {
    test3("eor", a64::eor_imm_64);
    test4("eor", a64::eor_reg_64);
}

#[test]
fn test_eret() {
    test_disasm("eret", "", a64::eret());
}

#[test]
fn test_eretaa() {
    test_disasm("eretaa", "", pauth::eretaa());
}

#[test]
fn test_eretab() {
    test_disasm("eretab", "", pauth::eretab());
}

#[test]
fn test_esb() {
    test_disasm("esb", "", ras::esb());
}

#[test]
fn test_extr_32() {
    test4_filter("extr", a64::extr_32, |_, a2, a3, lsb| a2 != a3 && lsb <= 31);
}

#[test]
fn test_extr_64() {
    test4_filter("extr", a64::extr_64, |_, a2, a3, lsb| a2 != a3 && lsb <= 63);
}

#[test]
fn test_hint() {
    test_disasm("hint", "#0x75", a64::hint(0x75));
}

#[test]
fn test_hlt() {
    test_disasm("hlt", "#0x1234", a64::hlt(0x1234));
}

#[test]
fn test_hvc() {
    test_disasm("hvc", "#0x1234", a64::hvc(0x1234));
}

#[test]
fn test_ic() {
    test_disasm("ic", "ivau, x6", a64::ic(3, 5, 1, X6));
}

#[test]
fn test_isb() {
    test_disasm("isb", "#0", a64::isb(0));
    test_disasm("isb", "", a64::isb(15));
}

#[test]
fn test_ldadd_32() {
    test_ldstpair_filter("ldadd", lse::ldadd_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldadd_64() {
    test_ldstpair_filter("ldadd", lse::ldadd_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldadda_32() {
    test_ldstpair("ldadda", lse::ldadda_32);
}

#[test]
fn test_ldadda_64() {
    test_ldstpair("ldadda", lse::ldadda_64);
}

#[test]
fn test_ldaddab() {
    test_ldstpair("ldaddab", lse::ldaddab);
}

#[test]
fn test_ldaddah() {
    test_ldstpair("ldaddah", lse::ldaddah);
}

#[test]
fn test_ldaddal_32() {
    test_ldstpair("ldaddal", lse::ldaddal_32);
}

#[test]
fn test_ldaddal_64() {
    test_ldstpair("ldaddal", lse::ldaddal_64);
}

#[test]
fn test_ldaddalb() {
    test_ldstpair("ldaddalb", lse::ldaddalb);
}

#[test]
fn test_ldaddalh() {
    test_ldstpair("ldaddalh", lse::ldaddalh);
}

#[test]
fn test_ldaddb() {
    test_ldstpair_filter("ldaddb", lse::ldaddb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldaddh() {
    test_ldstpair_filter("ldaddh", lse::ldaddh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldaddl_32() {
    test_ldstpair_filter("ldaddl", lse::ldaddl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldaddl_64() {
    test_ldstpair_filter("ldaddl", lse::ldaddl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldaddlb() {
    test_ldstpair_filter("ldaddlb", lse::ldaddlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldaddlh() {
    test_ldstpair_filter("ldaddlh", lse::ldaddlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldapr_32() {
    test_ldst("ldapr", lrcpc::ldapr_32);
}

#[test]
fn test_ldapr_64() {
    test_ldst("ldapr", lrcpc::ldapr_64);
}

#[test]
fn test_ldaprb() {
    test_ldst("ldaprb", lrcpc::ldaprb);
}

#[test]
fn test_ldaprh() {
    test_ldst("ldaprh", lrcpc::ldaprh);
}

#[test]
fn test_ldapur_32() {
    test_ldst("ldapur", |rt, rn| lrcpc2::ldapur_32(rt, rn, 0));
    test_ldst_offset_filter("ldapur", lrcpc2::ldapur_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapur_64() {
    test_ldst("ldapur", |rt, rn| lrcpc2::ldapur_64(rt, rn, 0));
    test_ldst_offset_filter("ldapur", lrcpc2::ldapur_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapurb() {
    test_ldst("ldapurb", |rt, rn| lrcpc2::ldapurb(rt, rn, 0));
    test_ldst_offset_filter("ldapurb", lrcpc2::ldapurb, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapurh() {
    test_ldst("ldapurh", |rt, rn| lrcpc2::ldapurh(rt, rn, 0));
    test_ldst_offset_filter("ldapurh", lrcpc2::ldapurh, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapursb_32() {
    test_ldst("ldapursb", |rt, rn| lrcpc2::ldapursb_32(rt, rn, 0));
    test_ldst_offset_filter("ldapursb", lrcpc2::ldapursb_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapursb_64() {
    test_ldst("ldapursb", |rt, rn| lrcpc2::ldapursb_64(rt, rn, 0));
    test_ldst_offset_filter("ldapursb", lrcpc2::ldapursb_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapursh_32() {
    test_ldst("ldapursh", |rt, rn| lrcpc2::ldapursh_32(rt, rn, 0));
    test_ldst_offset_filter("ldapursh", lrcpc2::ldapursh_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapursh_64() {
    test_ldst("ldapursh", |rt, rn| lrcpc2::ldapursh_64(rt, rn, 0));
    test_ldst_offset_filter("ldapursh", lrcpc2::ldapursh_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldapursw() {
    test_ldst("ldapursw", |rt, rn| lrcpc2::ldapursw(rt, rn, 0));
    test_ldst_offset_filter("ldapursw", lrcpc2::ldapursw, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldar_32() {
    test_ldst("ldar", a64::ldar_32);
}

#[test]
fn test_ldar_64() {
    test_ldst("ldar", a64::ldar_64);
}

#[test]
fn test_ldarb() {
    test_ldst("ldarb", a64::ldarb);
}

#[test]
fn test_ldarh() {
    test_ldst("ldarh", a64::ldarh);
}

#[test]
fn test_ldaxp_32() {
    test_ldstpair_filter("ldaxp", a64::ldaxp_32,
        |rt, rt2, _| rt != rt2);
}

#[test]
fn test_ldaxp_64() {
    test_ldstpair_filter("ldaxp", a64::ldaxp_64,
        |rt, rt2, _| rt != rt2);
}

#[test]
fn test_ldaxr_32() {
    test_ldst("ldaxr", a64::ldaxr_32);
}

#[test]
fn test_ldaxr_64() {
    test_ldst("ldaxr", a64::ldaxr_64);
}

#[test]
fn test_ldaxrb() {
    test_ldst("ldaxrb", a64::ldaxrb);
}

#[test]
fn test_ldaxrh() {
    test_ldst("ldaxrh", a64::ldaxrh);
}

#[test]
fn test_ldclr_32() {
    test_ldstpair_filter("ldclr", lse::ldclr_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldclr_64() {
    test_ldstpair_filter("ldclr", lse::ldclr_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldclra_32() {
    test_ldstpair("ldclra", lse::ldclra_32);
}

#[test]
fn test_ldclra_64() {
    test_ldstpair("ldclra", lse::ldclra_64);
}

#[test]
fn test_ldclrab() {
    test_ldstpair("ldclrab", lse::ldclrab);
}

#[test]
fn test_ldclrah() {
    test_ldstpair("ldclrah", lse::ldclrah);
}

#[test]
fn test_ldclral_32() {
    test_ldstpair("ldclral", lse::ldclral_32);
}

#[test]
fn test_ldclral_64() {
    test_ldstpair("ldclral", lse::ldclral_64);
}

#[test]
fn test_ldclralb() {
    test_ldstpair("ldclralb", lse::ldclralb);
}

#[test]
fn test_ldclralh() {
    test_ldstpair("ldclralh", lse::ldclralh);
}

#[test]
fn test_ldclrb() {
    test_ldstpair_filter("ldclrb", lse::ldclrb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldclrh() {
    test_ldstpair_filter("ldclrh", lse::ldclrh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldclrl_32() {
    test_ldstpair_filter("ldclrl", lse::ldclrl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldclrl_64() {
    test_ldstpair_filter("ldclrl", lse::ldclrl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldclrlb() {
    test_ldstpair_filter("ldclrlb", lse::ldclrlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldclrlh() {
    test_ldstpair_filter("ldclrlh", lse::ldclrlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldeor_32() {
    test_ldstpair_filter("ldeor", lse::ldeor_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldeor_64() {
    test_ldstpair_filter("ldeor", lse::ldeor_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldeora_32() {
    test_ldstpair("ldeora", lse::ldeora_32);
}

#[test]
fn test_ldeora_64() {
    test_ldstpair("ldeora", lse::ldeora_64);
}

#[test]
fn test_ldeorab() {
    test_ldstpair("ldeorab", lse::ldeorab);
}

#[test]
fn test_ldeorah() {
    test_ldstpair("ldeorah", lse::ldeorah);
}

#[test]
fn test_ldeoral_32() {
    test_ldstpair("ldeoral", lse::ldeoral_32);
}

#[test]
fn test_ldeoral_64() {
    test_ldstpair("ldeoral", lse::ldeoral_64);
}

#[test]
fn test_ldeoralb() {
    test_ldstpair("ldeoralb", lse::ldeoralb);
}

#[test]
fn test_ldeoralh() {
    test_ldstpair("ldeoralh", lse::ldeoralh);
}

#[test]
fn test_ldeorb() {
    test_ldstpair_filter("ldeorb", lse::ldeorb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldeorh() {
    test_ldstpair_filter("ldeorh", lse::ldeorh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldeorl_32() {
    test_ldstpair_filter("ldeorl", lse::ldeorl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldeorl_64() {
    test_ldstpair_filter("ldeorl", lse::ldeorl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldeorlb() {
    test_ldstpair_filter("ldeorlb", lse::ldeorlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldeorlh() {
    test_ldstpair_filter("ldeorlh", lse::ldeorlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldlar_32() {
    test_ldst("ldlar", lor::ldlar_32);
}

#[test]
fn test_ldlar_64() {
    test_ldst("ldlar", lor::ldlar_64);
}

#[test]
fn test_ldlarb() {
    test_ldst("ldlarb", lor::ldlarb);
}

#[test]
fn test_ldlarh() {
    test_ldst("ldlarh", lor::ldlarh);
}

#[test]
fn test_ldnp_32() {
    test_ldstpair_filter("ldnp", |rt, rt2, rn| a64::ldnp_32(rt, rt2, rn, Index::new(0)),
        |rt, rt2, _| rt != rt2);
    test_ldstpair_offset_filter("ldnp", a64::ldnp_32, |rt, rt2, _, offset| {
        rt != rt2 && offset.index() != 0
    });
}

#[test]
fn test_ldnp_64() {
    test_ldstpair_filter("ldnp", |rt, rt2, rn| a64::ldnp_64(rt, rt2, rn, Index::new(0)),
        |rt, rt2, _| rt != rt2);
    test_ldstpair_offset_filter("ldnp", a64::ldnp_64, |rt, rt2, _, offset| {
        rt != rt2 && offset.index() != 0
    });
}

#[test]
fn test_ldp_offset_32() {
    test_ldstpair_filter("ldp", |rt, rt2, rn| a64::ldp_offset_32(rt, rt2, rn, Index::new(0)),
        |rt, rt2, _| rt != rt2);
    test_ldstpair_offset_filter("ldp", a64::ldp_offset_32, |rt, rt2, _, offset| {
        rt != rt2 && offset.index() != 0
    });
}

#[test]
fn test_ldp_offset_64() {
    test_ldstpair_filter("ldp", |rt, rt2, rn| a64::ldp_offset_64(rt, rt2, rn, Index::new(0)),
        |rt, rt2, _| rt != rt2);
    test_ldstpair_offset_filter("ldp", a64::ldp_offset_64, |rt, rt2, _, offset| {
        rt != rt2 && offset.index() != 0
    });
}

#[test]
fn test_ldp_post_32() {
    test_ldstpair_post_filter("ldp", a64::ldp_post_32, |rt, rt2, rn, _| {
        rt != rt2 && rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_ldp_post_64() {
    test_ldstpair_post_filter("ldp", a64::ldp_post_64, |rt, rt2, rn, _| {
        rt != rt2 && rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_ldp_pre_32() {
    test_ldstpair_pre_filter("ldp", a64::ldp_pre_32, |rt, rt2, rn, _| {
        rt != rt2 && rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_ldp_pre_64() {
    test_ldstpair_pre_filter("ldp", a64::ldp_pre_64, |rt, rt2, rn, _| {
        rt != rt2 && rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_ldpsw_offset() {
    test_ldstpair_filter("ldpsw", |rt, rt2, rn| a64::ldpsw_offset(rt, rt2, rn, Index::new(0)),
        |rt, rt2, _| rt != rt2);
    test_ldstpair_offset_filter("ldpsw", a64::ldpsw_offset, |rt, rt2, _, offset| {
        rt != rt2 && offset.index() != 0
    });
}

#[test]
fn test_ldpsw_post() {
    test_ldstpair_post_filter("ldpsw", a64::ldpsw_post, |rt, rt2, rn, _| {
        rt != rt2 && rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_ldpsw_pre() {
    test_ldstpair_pre_filter("ldpsw", a64::ldpsw_pre, |rt, rt2, rn, _| {
        rt != rt2 && rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_ldr_immpost_32() {
    test_ldst_post_filter("ldr", a64::ldr_immpost_32, |rt, rn, offset| {
        rt.index() != rn.index() && offset >= 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldr_immpost_64() {
    test_ldst_post_filter("ldr", a64::ldr_immpost_64, |rt, rn, offset| {
        rt.index() != rn.index() && offset >= 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldr_immpre_32() {
    test_ldst_pre_filter("ldr", a64::ldr_immpre_32, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldr_immpre_64() {
    test_ldst_pre_filter("ldr", a64::ldr_immpre_64, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldr_lit_32() {
    test2_filter("ldr", a64::ldr_lit_32, |_, label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_ldr_lit_64() {
    test2_filter("ldr", a64::ldr_lit_64, |_, label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_ldr_pos_32() {
    test_ldst("ldr", |rt, rn| a64::ldr_pos_32(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldr", a64::ldr_pos_32, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_ldr_pos_64() {
    test_ldst("ldr", |rt, rn| a64::ldr_pos_64(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldr", a64::ldr_pos_64, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_ldr_regoff_32() {
    test_ldst_offset("ldr", |rt, rn, rm| a64::ldr_regoff_32(rt, rn, rm, 3, 0));
    test_disasm("ldr", "w3, [x4, w5, sxtw]", a64::ldr_regoff_32(W3, X4, X5, 6, 0));
    test_disasm("ldr", "w3, [x4, w5, sxtw #2]", a64::ldr_regoff_32(W3, X4, X5, 6, 1));
    test_disasm("ldr", "w3, [x4, x5, sxtx]", a64::ldr_regoff_32(W3, X4, X5, 7, 0));
    test_disasm("ldr", "w3, [x4, x5, sxtx #2]", a64::ldr_regoff_32(W3, X4, X5, 7, 1));
}

#[test]
fn test_ldr_regoff_64() {
    test_ldst_offset("ldr", |rt, rn, rm| a64::ldr_regoff_64(rt, rn, rm, 3, 0));
    test_disasm("ldr", "x3, [x4, w5, sxtw]", a64::ldr_regoff_64(X3, X4, X5, 6, 0));
    test_disasm("ldr", "x3, [x4, w5, sxtw #3]", a64::ldr_regoff_64(X3, X4, X5, 6, 1));
    test_disasm("ldr", "x3, [x4, x5, sxtx]", a64::ldr_regoff_64(X3, X4, X5, 7, 0));
    test_disasm("ldr", "x3, [x4, x5, sxtx #3]", a64::ldr_regoff_64(X3, X4, X5, 7, 1));
}

#[test]
fn test_ldraa_offset() {
    test_ldst("ldraa", |rt, rn| pauth::ldraa_offset(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldraa", pauth::ldraa_offset, |_, _, offset| {
        offset.index() != 0
    });
}

#[test]
fn test_ldraa_pre() {
    test_ldst_pre("ldraa", pauth::ldraa_pre);
}

#[test]
fn test_ldrab_offset() {
    test_ldst("ldrab", |rt, rn| pauth::ldrab_offset(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldrab", pauth::ldrab_offset, |_, _, offset| {
        offset.index() != 0
    });
}

#[test]
fn test_ldrab_pre() {
    test_ldst_pre("ldrab", pauth::ldrab_pre);
}

#[test]
fn test_ldrb_immpost() {
    test_ldst_post_filter("ldrb", a64::ldrb_immpost, |rt, rn, offset| {
        rt.index() != rn.index() && offset >= 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrb_immpre() {
    test_ldst_pre_filter("ldrb", a64::ldrb_immpre, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrb_pos() {
    test_ldst("ldrb", |rt, rn| a64::ldrb_pos(rt, rn, 0));
    test_ldst_offset_filter("ldrb", a64::ldrb_pos, |_, _, offset| {
        offset != 0 && offset < 0x1000
    });
}

#[test]
fn test_ldrb_regoff() {
    test_ldst_offset("ldrb", |rt, rn, rm| a64::ldrb_regoff(rt, rn, rm, 3, 0));
    test_disasm("ldrb", "w3, [x4, x5, lsl #0]", a64::ldrb_regoff(W3, X4, X5, 3, 1));
    test_disasm("ldrb", "w3, [x4, w5, sxtw]", a64::ldrb_regoff(W3, X4, X5, 6, 0));
    test_disasm("ldrb", "w3, [x4, w5, sxtw #0]", a64::ldrb_regoff(W3, X4, X5, 6, 1));
    test_disasm("ldrb", "w3, [x4, x5, sxtx]", a64::ldrb_regoff(W3, X4, X5, 7, 0));
    test_disasm("ldrb", "w3, [x4, x5, sxtx #0]", a64::ldrb_regoff(W3, X4, X5, 7, 1));
}

#[test]
fn test_ldrh_immpost() {
    test_ldst_post_filter("ldrh", a64::ldrh_immpost, |rt, rn, offset| {
        rt.index() != rn.index() && offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrh_immpre() {
    test_ldst_pre_filter("ldrh", a64::ldrh_immpre, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrh_pos() {
    test_ldst("ldrh", |rt, rn| a64::ldrh_pos(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldrh", a64::ldrh_pos, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_ldrh_regoff() {
    test_ldst_offset("ldrh", |rt, rn, rm| a64::ldrh_regoff(rt, rn, rm, 3, 0));
    test_disasm("ldrh", "w3, [x4, x5, lsl #1]", a64::ldrh_regoff(W3, X4, X5, 3, 1));
    test_disasm("ldrh", "w3, [x4, w5, sxtw]", a64::ldrh_regoff(W3, X4, X5, 6, 0));
    test_disasm("ldrh", "w3, [x4, w5, sxtw #1]", a64::ldrh_regoff(W3, X4, X5, 6, 1));
    test_disasm("ldrh", "w3, [x4, x5, sxtx]", a64::ldrh_regoff(W3, X4, X5, 7, 0));
    test_disasm("ldrh", "w3, [x4, x5, sxtx #1]", a64::ldrh_regoff(W3, X4, X5, 7, 1));
}

#[test]
fn test_ldrsb_immpost_32() {
    test_ldst_post_filter("ldrsb", a64::ldrsb_immpost_32, |rt, rn, offset| {
        rt.index() != rn.index() && offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsb_immpost_64() {
    test_ldst_post_filter("ldrsb", a64::ldrsb_immpost_64, |rt, rn, offset| {
        rt.index() != rn.index() && offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsb_immpre_32() {
    test_ldst_pre_filter("ldrsb", a64::ldrsb_immpre_32, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsb_immpre_64() {
    test_ldst_pre_filter("ldrsb", a64::ldrsb_immpre_64, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsb_pos_32() {
    test_ldst("ldrsb", |rt, rn| a64::ldrsb_pos_32(rt, rn, 0));
    test_ldst_offset_filter("ldrsb", a64::ldrsb_pos_32, |_, _, offset| {
        offset != 0 && offset < 0x1000
    });
}

#[test]
fn test_ldrsb_pos_64() {
    test_ldst("ldrsb", |rt, rn| a64::ldrsb_pos_64(rt, rn, 0));
    test_ldst_offset_filter("ldrsb", a64::ldrsb_pos_64, |_, _, offset| {
        offset != 0 && offset < 0x1000
    });
}

#[test]
fn test_ldrsb_regoff_32() {
    test_ldst_offset("ldrsb", |rt, rn, rm| a64::ldrsb_regoff_32(rt, rn, rm, 3, 0));
    test_disasm("ldrsb", "w3, [x4, x5, lsl #0]", a64::ldrsb_regoff_32(W3, X4, X5, 3, 1));
    test_disasm("ldrsb", "w3, [x4, w5, sxtw]", a64::ldrsb_regoff_32(W3, X4, X5, 6, 0));
    test_disasm("ldrsb", "w3, [x4, w5, sxtw #0]", a64::ldrsb_regoff_32(W3, X4, X5, 6, 1));
    test_disasm("ldrsb", "w3, [x4, x5, sxtx]", a64::ldrsb_regoff_32(W3, X4, X5, 7, 0));
    test_disasm("ldrsb", "w3, [x4, x5, sxtx #0]", a64::ldrsb_regoff_32(W3, X4, X5, 7, 1));
}

#[test]
fn test_ldrsb_regoff_64() {
    test_ldst_offset("ldrsb", |rt, rn, rm| a64::ldrsb_regoff_64(rt, rn, rm, 3, 0));
    test_disasm("ldrsb", "x3, [x4, x5, lsl #0]", a64::ldrsb_regoff_64(X3, X4, X5, 3, 1));
    test_disasm("ldrsb", "x3, [x4, w5, sxtw]", a64::ldrsb_regoff_64(X3, X4, X5, 6, 0));
    test_disasm("ldrsb", "x3, [x4, w5, sxtw #0]", a64::ldrsb_regoff_64(X3, X4, X5, 6, 1));
    test_disasm("ldrsb", "x3, [x4, x5, sxtx]", a64::ldrsb_regoff_64(X3, X4, X5, 7, 0));
    test_disasm("ldrsb", "x3, [x4, x5, sxtx #0]", a64::ldrsb_regoff_64(X3, X4, X5, 7, 1));
}

#[test]
fn test_ldrsh_immpost_32() {
    test_ldst_post_filter("ldrsh", a64::ldrsh_immpost_32, |rt, rn, offset| {
        rt.index() != rn.index() && offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsh_immpost_64() {
    test_ldst_post_filter("ldrsh", a64::ldrsh_immpost_64, |rt, rn, offset| {
        rt.index() != rn.index() && offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsh_immpre_32() {
    test_ldst_pre_filter("ldrsh", a64::ldrsh_immpre_32, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsh_immpre_64() {
    test_ldst_pre_filter("ldrsh", a64::ldrsh_immpre_64, |rt, rn, offset| {
        rt.index() != rn.index() && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsh_pos_32() {
    test_ldst("ldrsh", |rt, rn| a64::ldrsh_pos_32(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldrsh", a64::ldrsh_pos_32, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_ldrsh_pos_64() {
    test_ldst("ldrsh", |rt, rn| a64::ldrsh_pos_64(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldrsh", a64::ldrsh_pos_64, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_ldrsh_regoff_32() {
    test_ldst_offset("ldrsh", |rt, rn, rm| a64::ldrsh_regoff_32(rt, rn, rm, 3, 0));
    test_disasm("ldrsh", "w3, [x4, x5, lsl #1]", a64::ldrsh_regoff_32(W3, X4, X5, 3, 1));
    test_disasm("ldrsh", "w3, [x4, w5, sxtw]", a64::ldrsh_regoff_32(W3, X4, X5, 6, 0));
    test_disasm("ldrsh", "w3, [x4, w5, sxtw #1]", a64::ldrsh_regoff_32(W3, X4, X5, 6, 1));
    test_disasm("ldrsh", "w3, [x4, x5, sxtx]", a64::ldrsh_regoff_32(W3, X4, X5, 7, 0));
    test_disasm("ldrsh", "w3, [x4, x5, sxtx #1]", a64::ldrsh_regoff_32(W3, X4, X5, 7, 1));
}

#[test]
fn test_ldrsh_regoff_64() {
    test_ldst_offset("ldrsh", |rt, rn, rm| a64::ldrsh_regoff_64(rt, rn, rm, 3, 0));
    test_disasm("ldrsh", "x3, [x4, x5, lsl #1]", a64::ldrsh_regoff_64(X3, X4, X5, 3, 1));
    test_disasm("ldrsh", "x3, [x4, w5, sxtw]", a64::ldrsh_regoff_64(X3, X4, X5, 6, 0));
    test_disasm("ldrsh", "x3, [x4, w5, sxtw #1]", a64::ldrsh_regoff_64(X3, X4, X5, 6, 1));
    test_disasm("ldrsh", "x3, [x4, x5, sxtx]", a64::ldrsh_regoff_64(X3, X4, X5, 7, 0));
    test_disasm("ldrsh", "x3, [x4, x5, sxtx #1]", a64::ldrsh_regoff_64(X3, X4, X5, 7, 1));
}

#[test]
fn test_ldrsw_immpost() {
    test_ldst_post_filter("ldrsw", a64::ldrsw_immpost, |_, _, offset| {
        offset >= 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsw_immpre() {
    test_ldst_pre_filter("ldrsw", a64::ldrsw_immpre, |_, _, offset| {
        is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldrsw_lit() {
    test2_filter("ldrsw", a64::ldrsw_lit, |_, label| {
        label.byte_offset() & 3 == 0 && is_signed_nbit_integer(21, label.byte_offset())
    });
}

#[test]
fn test_ldrsw_pos() {
    test_ldst("ldrsw", |rt, rn| a64::ldrsw_pos(rt, rn, Index::new(0)));
    test_ldst_offset_filter("ldrsw", a64::ldrsw_pos, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_ldrsw_regoff() {
    test_ldst_offset("ldrsw", |rt, rn, rm| a64::ldrsw_regoff(rt, rn, rm, 3, 0));
    test_disasm("ldrsw", "x3, [x4, x5, lsl #2]", a64::ldrsw_regoff(X3, X4, X5, 3, 1));
    test_disasm("ldrsw", "x3, [x4, w5, sxtw]", a64::ldrsw_regoff(X3, X4, X5, 6, 0));
    test_disasm("ldrsw", "x3, [x4, w5, sxtw #2]", a64::ldrsw_regoff(X3, X4, X5, 6, 1));
    test_disasm("ldrsw", "x3, [x4, x5, sxtx]", a64::ldrsw_regoff(X3, X4, X5, 7, 0));
    test_disasm("ldrsw", "x3, [x4, x5, sxtx #2]", a64::ldrsw_regoff(X3, X4, X5, 7, 1));
}

#[test]
fn test_ldset_32() {
    test_ldstpair_filter("ldset", lse::ldset_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldset_64() {
    test_ldstpair_filter("ldset", lse::ldset_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldseta_32() {
    test_ldstpair("ldseta", lse::ldseta_32);
}

#[test]
fn test_ldseta_64() {
    test_ldstpair("ldseta", lse::ldseta_64);
}

#[test]
fn test_ldsetab() {
    test_ldstpair("ldsetab", lse::ldsetab);
}

#[test]
fn test_ldsetah() {
    test_ldstpair("ldsetah", lse::ldsetah);
}

#[test]
fn test_ldsetal_32() {
    test_ldstpair("ldsetal", lse::ldsetal_32);
}

#[test]
fn test_ldsetal_64() {
    test_ldstpair("ldsetal", lse::ldsetal_64);
}

#[test]
fn test_ldsetalb() {
    test_ldstpair("ldsetalb", lse::ldsetalb);
}

#[test]
fn test_ldsetalh() {
    test_ldstpair("ldsetalh", lse::ldsetalh);
}

#[test]
fn test_ldsetb() {
    test_ldstpair_filter("ldsetb", lse::ldsetb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldseth() {
    test_ldstpair_filter("ldseth", lse::ldseth,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsetl_32() {
    test_ldstpair_filter("ldsetl", lse::ldsetl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsetl_64() {
    test_ldstpair_filter("ldsetl", lse::ldsetl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldsetlb() {
    test_ldstpair_filter("ldsetlb", lse::ldsetlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsetlh() {
    test_ldstpair_filter("ldsetlh", lse::ldsetlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmax_32() {
    test_ldstpair_filter("ldsmax", lse::ldsmax_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmax_64() {
    test_ldstpair_filter("ldsmax", lse::ldsmax_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldsmaxa_32() {
    test_ldstpair("ldsmaxa", lse::ldsmaxa_32);
}

#[test]
fn test_ldsmaxa_64() {
    test_ldstpair("ldsmaxa", lse::ldsmaxa_64);
}

#[test]
fn test_ldsmaxab() {
    test_ldstpair("ldsmaxab", lse::ldsmaxab);
}

#[test]
fn test_ldsmaxah() {
    test_ldstpair("ldsmaxah", lse::ldsmaxah);
}

#[test]
fn test_ldsmaxal_32() {
    test_ldstpair("ldsmaxal", lse::ldsmaxal_32);
}

#[test]
fn test_ldsmaxal_64() {
    test_ldstpair("ldsmaxal", lse::ldsmaxal_64);
}

#[test]
fn test_ldsmaxalb() {
    test_ldstpair("ldsmaxalb", lse::ldsmaxalb);
}

#[test]
fn test_ldsmaxalh() {
    test_ldstpair("ldsmaxalh", lse::ldsmaxalh);
}

#[test]
fn test_ldsmaxb() {
    test_ldstpair_filter("ldsmaxb", lse::ldsmaxb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmaxh() {
    test_ldstpair_filter("ldsmaxh", lse::ldsmaxh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmaxl_32() {
    test_ldstpair_filter("ldsmaxl", lse::ldsmaxl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmaxl_64() {
    test_ldstpair_filter("ldsmaxl", lse::ldsmaxl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldsmaxlb() {
    test_ldstpair_filter("ldsmaxlb", lse::ldsmaxlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmaxlh() {
    test_ldstpair_filter("ldsmaxlh", lse::ldsmaxlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmin_32() {
    test_ldstpair_filter("ldsmin", lse::ldsmin_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsmin_64() {
    test_ldstpair_filter("ldsmin", lse::ldsmin_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldsmina_32() {
    test_ldstpair("ldsmina", lse::ldsmina_32);
}

#[test]
fn test_ldsmina_64() {
    test_ldstpair("ldsmina", lse::ldsmina_64);
}

#[test]
fn test_ldsminab() {
    test_ldstpair("ldsminab", lse::ldsminab);
}

#[test]
fn test_ldsminah() {
    test_ldstpair("ldsminah", lse::ldsminah);
}

#[test]
fn test_ldsminal_32() {
    test_ldstpair("ldsminal", lse::ldsminal_32);
}

#[test]
fn test_ldsminal_64() {
    test_ldstpair("ldsminal", lse::ldsminal_64);
}

#[test]
fn test_ldsminalb() {
    test_ldstpair("ldsminalb", lse::ldsminalb);
}

#[test]
fn test_ldsminalh() {
    test_ldstpair("ldsminalh", lse::ldsminalh);
}

#[test]
fn test_ldsminb() {
    test_ldstpair_filter("ldsminb", lse::ldsminb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsminh() {
    test_ldstpair_filter("ldsminh", lse::ldsminh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsminl_32() {
    test_ldstpair_filter("ldsminl", lse::ldsminl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsminl_64() {
    test_ldstpair_filter("ldsminl", lse::ldsminl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldsminlb() {
    test_ldstpair_filter("ldsminlb", lse::ldsminlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldsminlh() {
    test_ldstpair_filter("ldsminlh", lse::ldsminlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldtr_32() {
    test_ldst_filter("ldtr", |rt, rn| a64::ldtr_32(rt, rn, 0), |rt, rn| {
        rt.index() != rn.index()
    });
    test_ldst_offset_filter("ldtr", a64::ldtr_32, |rt, rn, offset| {
        rt.index() != rn.index() && offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtr_64() {
    test_ldst_filter("ldtr", |rt, rn| a64::ldtr_64(rt, rn, 0), |rt, rn| {
        rt.index() != rn.index()
    });
    test_ldst_offset_filter("ldtr", a64::ldtr_64, |rt, rn, offset| {
        rt.index() != rn.index() && offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtrb() {
    test_ldst_filter("ldtrb", |rt, rn| a64::ldtrb(rt, rn, 0), |rt, rn| {
        rt.index() != rn.index()
    });
    test_ldst_offset_filter("ldtrb", a64::ldtrb, |rt, rn, offset| {
        rt.index() != rn.index() && offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtrh() {
    test_ldst_filter("ldtrh", |rt, rn| a64::ldtrh(rt, rn, 0), |rt, rn| {
        rt.index() != rn.index()
    });
    test_ldst_offset_filter("ldtrh", a64::ldtrh, |rt, rn, offset| {
        rt.index() != rn.index() && offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtrsb_32() {
    test_ldst_filter("ldtrsb", |rt, rn| a64::ldtrsb_32(rt, rn, 0), |rt, rn| {
        rt.index() != rn.index()
    });
    test_ldst_offset_filter("ldtrsb", a64::ldtrsb_32, |rt, rn, offset| {
        rt.index() != rn.index() && offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtrsb_64() {
    test_ldst("ldtrsb", |rt, rn| a64::ldtrsb_64(rt, rn, 0));
    test_ldst_offset_filter("ldtrsb", a64::ldtrsb_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtrsh_32() {
    test_ldst_filter("ldtrsh", |rt, rn| a64::ldtrsh_32(rt, rn, 0), |rt, rn| {
        rt.index() != rn.index()
    });
    test_ldst_offset_filter("ldtrsh", a64::ldtrsh_32, |rt, rn, offset| {
        rt.index() != rn.index() && offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtrsh_64() {
    test_ldst("ldtrsh", |rt, rn| a64::ldtrsh_64(rt, rn, 0));
    test_ldst_offset_filter("ldtrsh", a64::ldtrsh_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldtrsw() {
    test_ldst("ldtrsw", |rt, rn| a64::ldtrsw(rt, rn, 0));
    test_ldst_offset_filter("ldtrsw", a64::ldtrsw, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldumax_32() {
    test_ldstpair_filter("ldumax", lse::ldumax_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldumax_64() {
    test_ldstpair_filter("ldumax", lse::ldumax_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldumaxa_32() {
    test_ldstpair("ldumaxa", lse::ldumaxa_32);
}

#[test]
fn test_ldumaxa_64() {
    test_ldstpair("ldumaxa", lse::ldumaxa_64);
}

#[test]
fn test_ldumaxab() {
    test_ldstpair("ldumaxab", lse::ldumaxab);
}

#[test]
fn test_ldumaxah() {
    test_ldstpair("ldumaxah", lse::ldumaxah);
}

#[test]
fn test_ldumaxal_32() {
    test_ldstpair("ldumaxal", lse::ldumaxal_32);
}

#[test]
fn test_ldumaxal_64() {
    test_ldstpair("ldumaxal", lse::ldumaxal_64);
}

#[test]
fn test_ldumaxalb() {
    test_ldstpair("ldumaxalb", lse::ldumaxalb);
}

#[test]
fn test_ldumaxalh() {
    test_ldstpair("ldumaxalh", lse::ldumaxalh);
}

#[test]
fn test_ldumaxb() {
    test_ldstpair_filter("ldumaxb", lse::ldumaxb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldumaxh() {
    test_ldstpair_filter("ldumaxh", lse::ldumaxh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldumaxl_32() {
    test_ldstpair_filter("ldumaxl", lse::ldumaxl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldumaxl_64() {
    test_ldstpair_filter("ldumaxl", lse::ldumaxl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldumaxlb() {
    test_ldstpair_filter("ldumaxlb", lse::ldumaxlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldumaxlh() {
    test_ldstpair_filter("ldumaxlh", lse::ldumaxlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldumin_32() {
    test_ldstpair_filter("ldumin", lse::ldumin_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldumin_64() {
    test_ldstpair_filter("ldumin", lse::ldumin_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_ldumina_32() {
    test_ldstpair("ldumina", lse::ldumina_32);
}

#[test]
fn test_ldumina_64() {
    test_ldstpair("ldumina", lse::ldumina_64);
}

#[test]
fn test_lduminab() {
    test_ldstpair("lduminab", lse::lduminab);
}

#[test]
fn test_lduminah() {
    test_ldstpair("lduminah", lse::lduminah);
}

#[test]
fn test_lduminal_32() {
    test_ldstpair("lduminal", lse::lduminal_32);
}

#[test]
fn test_lduminal_64() {
    test_ldstpair("lduminal", lse::lduminal_64);
}

#[test]
fn test_lduminalb() {
    test_ldstpair("lduminalb", lse::lduminalb);
}

#[test]
fn test_lduminalh() {
    test_ldstpair("lduminalh", lse::lduminalh);
}

#[test]
fn test_lduminb() {
    test_ldstpair_filter("lduminb", lse::lduminb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_lduminh() {
    test_ldstpair_filter("lduminh", lse::lduminh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_lduminl_32() {
    test_ldstpair_filter("lduminl", lse::lduminl_32,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_lduminl_64() {
    test_ldstpair_filter("lduminl", lse::lduminl_64,
        |_, rt, _| rt != XZR);
}

#[test]
fn test_lduminlb() {
    test_ldstpair_filter("lduminlb", lse::lduminlb,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_lduminlh() {
    test_ldstpair_filter("lduminlh", lse::lduminlh,
        |_, rt, _| rt != WZR);
}

#[test]
fn test_ldur_32() {
    test_ldst("ldur", |rt, rn| a64::ldur_32(rt, rn, 0));
    test_ldst_offset_filter("ldur", a64::ldur_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldur_64() {
    test_ldst("ldur", |rt, rn| a64::ldur_64(rt, rn, 0));
    test_ldst_offset_filter("ldur", a64::ldur_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldurb() {
    test_ldst("ldurb", |rt, rn| a64::ldurb(rt, rn, 0));
    test_ldst_offset_filter("ldurb", a64::ldurb, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldurh() {
    test_ldst("ldurh", |rt, rn| a64::ldurh(rt, rn, 0));
    test_ldst_offset_filter("ldurh", a64::ldurh, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldursb_32() {
    test_ldst("ldursb", |rt, rn| a64::ldursb_32(rt, rn, 0));
    test_ldst_offset_filter("ldursb", a64::ldursb_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldursb_64() {
    test_ldst("ldursb", |rt, rn| a64::ldursb_64(rt, rn, 0));
    test_ldst_offset_filter("ldursb", a64::ldursb_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldursh_32() {
    test_ldst("ldursh", |rt, rn| a64::ldursh_32(rt, rn, 0));
    test_ldst_offset_filter("ldursh", a64::ldursh_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldursh_64() {
    test_ldst("ldursh", |rt, rn| a64::ldursh_64(rt, rn, 0));
    test_ldst_offset_filter("ldursh", a64::ldursh_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldursw() {
    test_ldst("ldursw", |rt, rn| a64::ldursw(rt, rn, 0));
    test_ldst_offset_filter("ldursw", a64::ldursw, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_ldxp_32() {
    test_ldstpair_filter("ldxp", a64::ldxp_32,
        |rt, rt2, _| rt != rt2);
}

#[test]
fn test_ldxp_64() {
    test_ldstpair_filter("ldxp", a64::ldxp_64,
        |rt, rt2, _| rt != rt2);
}

#[test]
fn test_ldxr_32() {
    test_ldst("ldxr", a64::ldxr_32);
}

#[test]
fn test_ldxr_64() {
    test_ldst("ldxr", a64::ldxr_64);
}

#[test]
fn test_ldxrb() {
    test_ldst("ldxrb", a64::ldxrb);
}

#[test]
fn test_ldxrh() {
    test_ldst("ldxrh", a64::ldxrh);
}

#[test]
fn test_lsl_imm_32() {
    test3_filter("lsl", a64::lsl_imm_32, |_, _, shift| shift > 0 && shift <= 31);
}

#[test]
fn test_lsl_imm_64() {
    test3_filter("lsl", a64::lsl_imm_64, |_, _, shift| shift > 0 && shift <= 63);
}

#[test]
fn test_lsl_reg_32() {
    test3("lsl", a64::lsl_reg_32);
}

#[test]
fn test_lsl_reg_64() {
    test3("lsl", a64::lsl_reg_64);
}

#[test]
fn test_lslv_32() {
    test3("lsl", a64::lslv_32);
}

#[test]
fn test_lslv_64() {
    test3("lsl", a64::lslv_64);
}

#[test]
fn test_lsr_imm_32() {
    test3_filter("lsr", a64::lsr_imm_32, |_, _, shift| shift <= 31);
}

#[test]
fn test_lsr_imm_64() {
    test3_filter("lsr", a64::lsr_imm_64, |_, _, shift| shift <= 63);
}

#[test]
fn test_lsr_reg_32() {
    test3("lsr", a64::lsr_reg_32);
}

#[test]
fn test_lsr_reg_64() {
    test3("lsr", a64::lsr_reg_64);
}

#[test]
fn test_lsrv_32() {
    test3("lsr", a64::lsrv_32);
}

#[test]
fn test_lsrv_64() {
    test3("lsr", a64::lsrv_64);
}

#[test]
fn test_madd_32() {
    test4_filter("madd", a64::madd_32, |_, _, _, a4| a4 != WZR);
    test_disasm("mul", "w3, w4, w5", a64::madd_32(W3, W4, W5, WZR));
}

#[test]
fn test_madd_64() {
    test4_filter("madd", a64::madd_64, |_, _, _, a4| a4 != XZR);
    test_disasm("mul", "x3, x4, x5", a64::madd_64(X3, X4, X5, XZR));
}

#[test]
fn test_mneg_32() {
    test3("mneg", a64::mneg_32);
}

#[test]
fn test_mneg_64() {
    test3("mneg", a64::mneg_64);
}

#[test]
fn test_mov_bitmask_32() {
    test2_filter("mov", a64::mov_bitmask_32, |_, a2| u32::from(a2) != 1);
}

#[test]
fn test_mov_bitmask_64() {
    test_disasm("orr", "sp, xzr, #0x100010001000100",
                a64::mov_bitmask_64(XSP, 0x100010001000100.try_into().unwrap()));
    test_disasm("orr", "x3, xzr, #0x100010001000100",
                a64::mov_bitmask_64(X3, 0x100010001000100.try_into().unwrap()));
}

#[test]
fn test_mov_reg_32() {
    test2("mov", a64::mov_reg_32);
}

#[test]
fn test_mov_reg_64() {
    test2("mov", a64::mov_reg_64);
}

#[test]
fn test_mov_sp_32() {
    test2_filter("mov", a64::mov_sp_32, |a1, a2| a1 == WSP || a2 == WSP);
}

#[test]
fn test_mov_sp_64() {
    test2_filter("mov", a64::mov_sp_64, |a1, a2| a1 == XSP || a2 == XSP);
}

#[test]
fn test_movk_32() {
    test2("movk", a64::movk_32::<u16>);
    test2("movk", a64::movk_32::<MovImm32>);
}

#[test]
fn test_movk_64() {
    test2("movk", a64::movk_64::<u16>);
    test2("movk", a64::movk_64::<MovImm64>);
}

#[test]
fn test_movn_32() {
    test_disasm("mov", "w3, #-0x1235", a64::movn_32(W3, 0x1234));
    test_disasm("mov", "w3, #-0x12340001", a64::movn_32(W3, MovImm32::lsl_16(0x1234)));
}

#[test]
fn test_movn_64() {
    test_disasm("mov", "x3, #-0x1235", a64::movn_64(X3, 0x1234));
    test_disasm("mov", "x3, #-0x12340001", a64::movn_64(X3, MovImm64::lsl_16(0x1234)));
    test_disasm("mov", "x3, #-0x123400000001", a64::movn_64(X3, MovImm64::lsl_32(0x1234)));
    test_disasm("mov", "x3, #-0x1234000000000001", a64::movn_64(X3, MovImm64::lsl_48(0x1234)));
}

#[test]
fn test_movz_32() {
    test_disasm("mov", "w3, #0x1234", a64::movz_32(W3, 0x1234));
    test_disasm("mov", "w3, #0x12340000", a64::movz_32(W3, MovImm32::lsl_16(0x1234)));
}

#[test]
fn test_movz_64() {
    test_disasm("mov", "x3, #0x1234", a64::movz_64(X3, 0x1234));
    test_disasm("mov", "x3, #0x12340000", a64::movz_64(X3, MovImm64::lsl_16(0x1234)));
    test_disasm("mov", "x3, #0x123400000000", a64::movz_64(X3, MovImm64::lsl_32(0x1234)));
    test_disasm("mov", "x3, #0x1234000000000000", a64::movz_64(X3, MovImm64::lsl_48(0x1234)));
}

#[test]
fn test_mrs() {
    test_disasm("mrs", "x2, s3_4_c5_c6_7", a64::mrs(X2, 3, 4, 5, 6, 7));
}

#[test]
fn test_msr_imm() {
    test_disasm("msr", "spsel, #9", a64::msr_imm(0, 5, 9));
}

#[test]
fn test_msr_reg() {
    test_disasm("msr", "s3_4_c5_c6_7, x8", a64::msr_reg(3, 4, 5, 6, 7, X8));
}

#[test]
fn test_msub_32() {
    test4_filter("msub", a64::msub_32, |_, _, _, a4| a4 != WZR);
}

#[test]
fn test_msub_64() {
    test4_filter("msub", a64::msub_64, |_, _, _, a4| a4 != XZR);
}

#[test]
fn test_mul_32() {
    test3("mul", a64::mul_32);
}

#[test]
fn test_mul_64() {
    test3("mul", a64::mul_64);
}

#[test]
fn test_mvn_32() {
    test3("mvn", a64::mvn_32);
}

#[test]
fn test_mvn_64() {
    test3("mvn", a64::mvn_64);
}

#[test]
fn test_neg_32() {
    test3_filter("neg", a64::neg_32, |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_neg_64() {
    test3_filter("neg", a64::neg_64, |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_negs_32() {
    test3_filter("negs", a64::negs_32, |a1, _, shift| {
        a1 != WZR && !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_negs_64() {
    test3_filter("negs", a64::negs_64, |a1, _, shift| {
        a1 != XZR && !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_ngc_32() {
    test2("ngc", a64::ngc_32);
}

#[test]
fn test_ngc_64() {
    test2("ngc", a64::ngc_64);
}

#[test]
fn test_ngcs_32() {
    test2("ngcs", a64::ngcs_32);
}

#[test]
fn test_ngcs_64() {
    test2("ngcs", a64::ngcs_64);
}

#[test]
fn test_nop() {
    test_disasm("nop", "", a64::nop());
}

#[test]
fn test_orn_32() {
    test4_filter("orn", a64::orn_32, |_, a2, _, _| a2 != WZR);
}

#[test]
fn test_orn_64() {
    test4_filter("orn", a64::orn_64, |_, a2, _, _| a2 != XZR);
}

#[test]
fn test_orr_imm_32() {
    test3_filter("orr", a64::orr_imm_32, |_, a2, _| a2 != WZR);
}

#[test]
fn test_orr_imm_64() {
    test3("orr", a64::orr_imm_64);
}

#[test]
fn test_orr_reg_32() {
    test4("orr", a64::orr_reg_32);
}

#[test]
fn test_orr_reg_64() {
    test4("orr", a64::orr_reg_64);
}

#[test]
fn test_pacda() {
    test2("pacda", pauth::pacda);
}

#[test]
fn test_pacdb() {
    test2("pacdb", pauth::pacdb);
}

#[test]
fn test_pacdza() {
    test1("pacdza", pauth::pacdza);
}

#[test]
fn test_pacdzb() {
    test1("pacdzb", pauth::pacdzb);
}

#[test]
fn test_pacga() {
    test3("pacga", pauth::pacga);
}

#[test]
fn test_pacia() {
    test2("pacia", pauth::pacia);
}

#[test]
fn test_pacia1716() {
    test_disasm("pacia1716", "", pauth::pacia1716());
}

#[test]
fn test_paciasp() {
    test_disasm("paciasp", "", pauth::paciasp());
}

#[test]
fn test_paciaz() {
    test_disasm("paciaz", "", pauth::paciaz());
}

#[test]
fn test_pacib() {
    test2("pacib", pauth::pacib);
}

#[test]
fn test_pacib1716() {
    test_disasm("pacib1716", "", pauth::pacib1716());
}

#[test]
fn test_pacibsp() {
    test_disasm("pacibsp", "", pauth::pacibsp());
}

#[test]
fn test_pacibz() {
    test_disasm("pacibz", "", pauth::pacibz());
}

#[test]
fn test_paciza() {
    test1("paciza", pauth::paciza);
}

#[test]
fn test_pacizb() {
    test1("pacizb", pauth::pacizb);
}

#[test]
fn test_prfm_lit() {
    test_disasm("prfm", "pldl1keep, #0x5678", a64::prfm_lit(0, Label::from_byte_offset(0x5678)));
    test_disasm("prfm", "pldl1keep, #0xfffffffffffffff0", a64::prfm_lit(0, Label::from_byte_offset(-0x10)));
}

#[test]
fn test_prfm_pos() {
    test_disasm("prfm", "pldl1keep, [x5, #0x30]", a64::prfm_pos(0, X5, 6));
}

#[test]
fn test_prfm_regoff() {
    test_disasm("prfm", "pldl1keep, [x4, w5, sxtw]", a64::prfm_regoff(0, X4, X5, 6, 0));
    test_disasm("prfm", "pldl1keep, [x4, w5, sxtw #3]", a64::prfm_regoff(0, X4, X5, 6, 1));
    test_disasm("prfm", "pldl1keep, [x4, x5, sxtx]", a64::prfm_regoff(0, X4, X5, 7, 0));
    test_disasm("prfm", "pldl1keep, [x4, x5, sxtx #3]", a64::prfm_regoff(0, X4, X5, 7, 1));
}

#[test]
fn test_prfum() {
    test_disasm("prfum", "pldl1keep, [x5]", a64::prfum(0, X5, 0));
    test_disasm("prfum", "pldl1keep, [x5, #-1]", a64::prfum(0, X5, -1i16));
    test_disasm("prfum", "pstl3strm, [x5, #1]", a64::prfum(0x15, X5, 1));
}

#[test]
fn test_pssbb() {
    test_disasm("dsb", "#4", a64::pssbb());
}

#[test]
fn test_rbit_32() {
    test2("rbit", a64::rbit_32);
}

#[test]
fn test_rbit_64() {
    test2("rbit", a64::rbit_64);
}

#[test]
fn test_ret() {
    test1_filter("ret", a64::ret, |rn| rn != X30);
}

#[test]
fn test_retaa() {
    test_disasm("retaa", "", pauth::retaa());
}

#[test]
fn test_retab() {
    test_disasm("retab", "", pauth::retab());
}

#[test]
fn test_rev_32() {
    test2("rev", a64::rev_32);
}

#[test]
fn test_rev_64() {
    test2("rev", a64::rev_64);
}

#[test]
fn test_rev16_32() {
    test2("rev16", a64::rev16_32);
}

#[test]
fn test_rev16_64() {
    test2("rev16", a64::rev16_64);
}

#[test]
fn test_rev32() {
    test2("rev32", a64::rev32);
}

#[test]
fn test_rev64() {
    test2("rev", a64::rev64);
}

#[test]
fn test_rmif() {
    test_disasm("rmif", "x3, #4, #5", flagm::rmif(X3, 4, 5));
}

#[test]
fn test_ror_imm_32() {
    test3_filter("ror", a64::ror_imm_32, |_, _, shift| shift <= 31);
}

#[test]
fn test_ror_imm_64() {
    test3_filter("ror", a64::ror_imm_64, |_, _, shift| shift <= 63);
}

#[test]
fn test_ror_reg_32() {
    test3("ror", a64::ror_reg_32);
}

#[test]
fn test_ror_reg_64() {
    test3("ror", a64::ror_reg_64);
}

#[test]
fn test_rorv_32() {
    test3("ror", a64::rorv_32);
}

#[test]
fn test_rorv_64() {
    test3("ror", a64::rorv_64);
}

#[test]
fn test_sb() {
    test_disasm("msr", "s0_3_c3_c0_7, xzr", sb::sb());
}

#[test]
fn test_sbc_32() {
    test3_filter("sbc", a64::sbc_32, |_, a2, _| a2 != WZR);
}

#[test]
fn test_sbc_64() {
    test3_filter("sbc", a64::sbc_64, |_, a2, _| a2 != XZR);
}

#[test]
fn test_sbcs_32() {
    test3_filter("sbcs", a64::sbcs_32, |_, a2, _| a2 != WZR);
}

#[test]
fn test_sbcs_64() {
    test3_filter("sbcs", a64::sbcs_64, |_, a2, _| a2 != XZR);
}

#[test]
fn test_sbfiz_32() {
    test4_filter("sbfiz", a64::sbfiz_32, |_, _, lsb, width| {
        lsb > 0 && lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_sbfiz_64() {
    test4_filter("sbfiz", a64::sbfiz_64, |_, _, lsb, width| {
        lsb > 0 && lsb <= 31 && width > 0 && width <= 64 - lsb
    });
}

#[test]
fn test_sbfm_32() {
    test_disasm("asr", "w3, w4, #5", a64::sbfm_32(W3, W4, 0x5, 0x1f));
    test_disasm("sbfiz", "w3, w4, #5, #6", a64::sbfm_32(W3, W4, 0x1b, 5));
    test_disasm("sbfx", "w3, w4, #5, #6", a64::sbfm_32(W3, W4, 5, 10));
    test_disasm("sxtb", "w3, w4", a64::sbfm_32(W3, W4, 0, 7));
    test_disasm("sxth", "w3, w4", a64::sbfm_32(W3, W4, 0, 0xf));
}

#[test]
fn test_sbfm_64() {
    test_disasm("asr", "x3, x4, #5", a64::sbfm_64(X3, X4, 0x5, 0x3f));
    test_disasm("sbfiz", "x3, x4, #5, #6", a64::sbfm_64(X3, X4, 0x3b, 5));
    test_disasm("sbfx", "x3, x4, #5, #6", a64::sbfm_64(X3, X4, 5, 10));
}

#[test]
fn test_sbfx_32() {
    test4_filter("sbfx", a64::sbfx_32, |_, _, lsb, width| {
        lsb <= 31 && width > 0 && width <= 31 - lsb
            && (lsb, width) != (0, 8)
            && (lsb, width) != (0, 16)
    });
}

#[test]
fn test_sbfx_64() {
    test4_filter("sbfx", a64::sbfx_64, |_, _, lsb, width| {
        lsb <= 63 && width > 0 && width <= 63 - lsb
            && (lsb, width) != (0, 8)
            && (lsb, width) != (0, 16)
            && (lsb, width) != (0, 32)
    });
}

#[test]
fn test_sdiv_32() {
    test3("sdiv", a64::sdiv_32);
}

#[test]
fn test_sdiv_64() {
    test3("sdiv", a64::sdiv_64);
}

#[test]
fn test_setf8() {
    test1("setf8", flagm::setf8);
}

#[test]
fn test_setf16() {
    test1("setf16", flagm::setf16);
}

#[test]
fn test_sev() {
    test_disasm("sev", "", a64::sev());
}

#[test]
fn test_sevl() {
    test_disasm("sevl", "", a64::sevl());
}

#[test]
fn test_smaddl() {
    test4_filter("smaddl", a64::smaddl, |_, _, _, a4| a4 != XZR);
}

#[test]
fn test_smc() {
    test_disasm("smc", "#0x1234", a64::smc(0x1234));
}

#[test]
fn test_smnegl() {
    test3("smnegl", a64::smnegl);
}

#[test]
fn test_smsubl() {
    test4_filter("smsubl", a64::smsubl, |_, _, _, a4| a4 != XZR);
}

#[test]
fn test_smulh() {
    test3("smulh", a64::smulh);
}

#[test]
fn test_smull() {
    test3("smull", a64::smull);
}

#[test]
fn test_ssbb() {
    test_disasm("dsb", "#0", a64::ssbb());
}

#[test]
fn test_stadd_32() {
    test_ldst("stadd", lse::stadd_32);
}

#[test]
fn test_stadd_64() {
    test_ldst("stadd", lse::stadd_64);
}

#[test]
fn test_staddb() {
    test_ldst("staddb", lse::staddb);
}

#[test]
fn test_staddh() {
    test_ldst("staddh", lse::staddh);
}

#[test]
fn test_staddl_32() {
    test_ldst("staddl", lse::staddl_32);
}

#[test]
fn test_staddl_64() {
    test_ldst("staddl", lse::staddl_64);
}

#[test]
fn test_staddlb() {
    test_ldst("staddlb", lse::staddlb);
}

#[test]
fn test_staddlh() {
    test_ldst("staddlh", lse::staddlh);
}

#[test]
fn test_stclr_32() {
    test_ldst("stclr", lse::stclr_32);
}

#[test]
fn test_stclr_64() {
    test_ldst("stclr", lse::stclr_64);
}

#[test]
fn test_stclrb() {
    test_ldst("stclrb", lse::stclrb);
}

#[test]
fn test_stclrh() {
    test_ldst("stclrh", lse::stclrh);
}

#[test]
fn test_stclrl_32() {
    test_ldst("stclrl", lse::stclrl_32);
}

#[test]
fn test_stclrl_64() {
    test_ldst("stclrl", lse::stclrl_64);
}

#[test]
fn test_stclrlb() {
    test_ldst("stclrlb", lse::stclrlb);
}

#[test]
fn test_stclrlh() {
    test_ldst("stclrlh", lse::stclrlh);
}

#[test]
fn test_steor_32() {
    test_ldst("steor", lse::steor_32);
}

#[test]
fn test_steor_64() {
    test_ldst("steor", lse::steor_64);
}

#[test]
fn test_steorb() {
    test_ldst("steorb", lse::steorb);
}

#[test]
fn test_steorh() {
    test_ldst("steorh", lse::steorh);
}

#[test]
fn test_steorl_32() {
    test_ldst("steorl", lse::steorl_32);
}

#[test]
fn test_steorl_64() {
    test_ldst("steorl", lse::steorl_64);
}

#[test]
fn test_steorlb() {
    test_ldst("steorlb", lse::steorlb);
}

#[test]
fn test_steorlh() {
    test_ldst("steorlh", lse::steorlh);
}

#[test]
fn test_stllr_32() {
    test_ldst("stllr", lor::stllr_32);
}

#[test]
fn test_stllr_64() {
    test_ldst("stllr", lor::stllr_64);
}

#[test]
fn test_stllrb() {
    test_ldst("stllrb", lor::stllrb);
}

#[test]
fn test_stllrh() {
    test_ldst("stllrh", lor::stllrh);
}

#[test]
fn test_stlr_32() {
    test_ldst("stlr", a64::stlr_32);
}

#[test]
fn test_stlr_64() {
    test_ldst("stlr", a64::stlr_64);
}

#[test]
fn test_stlrb() {
    test_ldst("stlrb", a64::stlrb);
}

#[test]
fn test_stlrh() {
    test_ldst("stlrh", a64::stlrh);
}

#[test]
fn test_stlur_32() {
    test_ldst("stlur", |rt, rn| lrcpc2::stlur_32(rt, rn, 0));
    test_ldst_offset_filter("stlur", lrcpc2::stlur_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_stlur_64() {
    test_ldst("stlur", |rt, rn| lrcpc2::stlur_64(rt, rn, 0));
    test_ldst_offset_filter("stlur", lrcpc2::stlur_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_stlurb() {
    test_ldst("stlurb", |rt, rn| lrcpc2::stlurb(rt, rn, 0));
    test_ldst_offset_filter("stlurb", lrcpc2::stlurb, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_stlurh() {
    test_ldst("stlurh", |rt, rn| lrcpc2::stlurh(rt, rn, 0));
    test_ldst_offset_filter("stlurh", lrcpc2::stlurh, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_stlxp_32() {
    test_disasm("stlxp", "w3, w4, w5, [x6]", a64::stlxp_32(W3, W4, W5, X6));
}

#[test]
fn test_stlxp_64() {
    test_disasm("stlxp", "w3, x4, x5, [x6]", a64::stlxp_64(W3, X4, X5, X6));
}

#[test]
fn test_stlxr_32() {
    test_ldstpair("stlxr", a64::stlxr_32);
}

#[test]
fn test_stlxr_64() {
    test_ldstpair("stlxr", a64::stlxr_64);
}

#[test]
fn test_stlxrb() {
    test_ldstpair("stlxrb", a64::stlxrb);
}

#[test]
fn test_stlxrh() {
    test_ldstpair("stlxrh", a64::stlxrh);
}

#[test]
fn test_stnp_32() {
    test_ldstpair("stnp", |rt, rt2, rn| a64::stnp_32(rt, rt2, rn, Index::new(0)));
    test_ldstpair_offset_filter("stnp", a64::stnp_32, |_, _, _, offset| {
        offset.index() != 0
    });
}

#[test]
fn test_stnp_64() {
    test_ldstpair("stnp", |rt, rt2, rn| a64::stnp_64(rt, rt2, rn, Index::new(0)));
    test_ldstpair_offset_filter("stnp", a64::stnp_64, |_, _, _, offset| {
        offset.index() != 0
    });
}

#[test]
fn test_stp_offset_32() {
    test_ldstpair("stp", |rt, rt2, rn| a64::stp_offset_32(rt, rt2, rn, Index::new(0)));
    test_ldstpair_offset_filter("stp", a64::stp_offset_32, |_, _, _, offset| {
        offset.index() != 0
    });
}

#[test]
fn test_stp_offset_64() {
    test_ldstpair("stp", |rt, rt2, rn| a64::stp_offset_64(rt, rt2, rn, Index::new(0)));
    test_ldstpair_offset_filter("stp", a64::stp_offset_64, |_, _, _, offset| {
        offset.index() != 0
    });
}

#[test]
fn test_stp_post_32() {
    test_ldstpair_post_filter("stp", a64::stp_post_32, |rt, rt2, rn, _| {
        rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_stp_post_64() {
    test_ldstpair_post_filter("stp", a64::stp_post_64, |rt, rt2, rn, _| {
        rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_stp_pre_32() {
    test_ldstpair_pre_filter("stp", a64::stp_pre_32, |rt, rt2, rn, _| {
        rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_stp_pre_64() {
    test_ldstpair_pre_filter("stp", a64::stp_pre_64, |rt, rt2, rn, _| {
        rt.index() != rn.index() && rt2.index() != rn.index()
    });
}

#[test]
fn test_str_immpost_32() {
    test_ldst_post_filter("str", a64::str_immpost_32, |_, _, offset| {
        offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_str_immpost_64() {
    test_ldst_post_filter("str", a64::str_immpost_64, |_, _, offset| {
        offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_str_immpre_32() {
    test_ldst_pre_filter("str", a64::str_immpre_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_str_immpre_64() {
    test_ldst_pre_filter("str", a64::str_immpre_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_str_pos_32() {
    test_ldst("str", |rt, rn| a64::str_pos_32(rt, rn, Index::new(0)));
    test_ldst_offset_filter("str", a64::str_pos_32, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_str_pos_64() {
    test_ldst("str", |rt, rn| a64::str_pos_64(rt, rn, Index::new(0)));
    test_ldst_offset_filter("str", a64::str_pos_64, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_str_regoff_32() {
    test_ldst_offset("str", |rt, rn, rm| a64::str_regoff_32(rt, rn, rm, 3, 0));
    test_disasm("str", "w3, [x4, w5, sxtw]", a64::str_regoff_32(W3, X4, X5, 6, 0));
    test_disasm("str", "w3, [x4, w5, sxtw #2]", a64::str_regoff_32(W3, X4, X5, 6, 1));
    test_disasm("str", "w3, [x4, x5, sxtx]", a64::str_regoff_32(W3, X4, X5, 7, 0));
    test_disasm("str", "w3, [x4, x5, sxtx #2]", a64::str_regoff_32(W3, X4, X5, 7, 1));
}

#[test]
fn test_str_regoff_64() {
    test_ldst_offset("str", |rt, rn, rm| a64::str_regoff_64(rt, rn, rm, 3, 0));
    test_disasm("str", "x3, [x4, w5, sxtw]", a64::str_regoff_64(X3, X4, X5, 6, 0));
    test_disasm("str", "x3, [x4, w5, sxtw #3]", a64::str_regoff_64(X3, X4, X5, 6, 1));
    test_disasm("str", "x3, [x4, x5, sxtx]", a64::str_regoff_64(X3, X4, X5, 7, 0));
    test_disasm("str", "x3, [x4, x5, sxtx #3]", a64::str_regoff_64(X3, X4, X5, 7, 1));
}

#[test]
fn test_strb_immpost() {
    test_ldst_post_filter("strb", a64::strb_immpost, |_, _, offset| {
        offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_strb_immpre() {
    test_ldst_pre_filter("strb", a64::strb_immpre, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_strb_pos() {
    test_ldst("strb", |rt, rn| a64::strb_pos(rt, rn, 0));
    test_ldst_offset_filter("strb", a64::strb_pos, |_, _, offset| {
        offset > 0 && offset < 0x1000
    });
}

#[test]
fn test_strb_regoff() {
    test_ldst_offset("strb", |rt, rn, rm| a64::strb_regoff(rt, rn, rm, 3, 0));
    test_disasm("strb", "w3, [x4, x5, lsl #0]", a64::strb_regoff(W3, X4, X5, 3, 1));
    test_disasm("strb", "w3, [x4, w5, sxtw]", a64::strb_regoff(W3, X4, X5, 6, 0));
    test_disasm("strb", "w3, [x4, w5, sxtw #0]", a64::strb_regoff(W3, X4, X5, 6, 1));
    test_disasm("strb", "w3, [x4, x5, sxtx]", a64::strb_regoff(W3, X4, X5, 7, 0));
    test_disasm("strb", "w3, [x4, x5, sxtx #0]", a64::strb_regoff(W3, X4, X5, 7, 1));
}

#[test]
fn test_strh_immpost() {
    test_ldst_post_filter("strh", a64::strh_immpost, |_, _, offset| {
        offset > 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_strh_immpre() {
    test_ldst_pre_filter("strh", a64::strh_immpre, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_strh_pos() {
    test_ldst("strh", |rt, rn| a64::strh_pos(rt, rn, Index::new(0)));
    test_ldst_offset_filter("strh", a64::strh_pos, |_, _, offset| {
        offset.index() > 0
    });
}

#[test]
fn test_strh_regoff() {
    test_ldst_offset("strh", |rt, rn, rm| a64::strh_regoff(rt, rn, rm, 3, 0));
    test_disasm("strh", "w3, [x4, x5, lsl #1]", a64::strh_regoff(W3, X4, X5, 3, 1));
    test_disasm("strh", "w3, [x4, w5, sxtw]", a64::strh_regoff(W3, X4, X5, 6, 0));
    test_disasm("strh", "w3, [x4, w5, sxtw #1]", a64::strh_regoff(W3, X4, X5, 6, 1));
    test_disasm("strh", "w3, [x4, x5, sxtx]", a64::strh_regoff(W3, X4, X5, 7, 0));
    test_disasm("strh", "w3, [x4, x5, sxtx #1]", a64::strh_regoff(W3, X4, X5, 7, 1));
}

#[test]
fn test_stset_32() {
    test_ldst("stset", lse::stset_32);
}

#[test]
fn test_stset_64() {
    test_ldst("stset", lse::stset_64);
}

#[test]
fn test_stsetb() {
    test_ldst("stsetb", lse::stsetb);
}

#[test]
fn test_stseth() {
    test_ldst("stseth", lse::stseth);
}

#[test]
fn test_stsetl_32() {
    test_ldst("stsetl", lse::stsetl_32);
}

#[test]
fn test_stsetl_64() {
    test_ldst("stsetl", lse::stsetl_64);
}

#[test]
fn test_stsetlb() {
    test_ldst("stsetlb", lse::stsetlb);
}

#[test]
fn test_stsetlh() {
    test_ldst("stsetlh", lse::stsetlh);
}

#[test]
fn test_stsmax_32() {
    test_ldst("stsmax", lse::stsmax_32);
}

#[test]
fn test_stsmax_64() {
    test_ldst("stsmax", lse::stsmax_64);
}

#[test]
fn test_stsmaxb() {
    test_ldst("stsmaxb", lse::stsmaxb);
}

#[test]
fn test_stsmaxh() {
    test_ldst("stsmaxh", lse::stsmaxh);
}

#[test]
fn test_stsmaxl_32() {
    test_ldst("stsmaxl", lse::stsmaxl_32);
}

#[test]
fn test_stsmaxl_64() {
    test_ldst("stsmaxl", lse::stsmaxl_64);
}

#[test]
fn test_stsmaxlb() {
    test_ldst("stsmaxlb", lse::stsmaxlb);
}

#[test]
fn test_stsmaxlh() {
    test_ldst("stsmaxlh", lse::stsmaxlh);
}

#[test]
fn test_stsmin_32() {
    test_ldst("stsmin", lse::stsmin_32);
}

#[test]
fn test_stsmin_64() {
    test_ldst("stsmin", lse::stsmin_64);
}

#[test]
fn test_stsminb() {
    test_ldst("stsminb", lse::stsminb);
}

#[test]
fn test_stsminh() {
    test_ldst("stsminh", lse::stsminh);
}

#[test]
fn test_stsminl_32() {
    test_ldst("stsminl", lse::stsminl_32);
}

#[test]
fn test_stsminl_64() {
    test_ldst("stsminl", lse::stsminl_64);
}

#[test]
fn test_stsminlb() {
    test_ldst("stsminlb", lse::stsminlb);
}

#[test]
fn test_stsminlh() {
    test_ldst("stsminlh", lse::stsminlh);
}

#[test]
fn test_sttr_32() {
    test_ldst("sttr", |rt, rn| a64::sttr_32(rt, rn, 0));
    test_ldst_offset_filter("sttr", a64::sttr_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_sttr_64() {
    test_ldst("sttr", |rt, rn| a64::sttr_64(rt, rn, 0));
    test_ldst_offset_filter("sttr", a64::sttr_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_sttrb() {
    test_ldst("sttrb", |rt, rn| a64::sttrb(rt, rn, 0));
    test_ldst_offset_filter("sttrb", a64::sttrb, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_sttrh() {
    test_ldst("sttrh", |rt, rn| a64::sttrh(rt, rn, 0));
    test_ldst_offset_filter("sttrh", a64::sttrh, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_stumax_32() {
    test_ldst("stumax", lse::stumax_32);
}

#[test]
fn test_stumax_64() {
    test_ldst("stumax", lse::stumax_64);
}

#[test]
fn test_stumaxb() {
    test_ldst("stumaxb", lse::stumaxb);
}

#[test]
fn test_stumaxh() {
    test_ldst("stumaxh", lse::stumaxh);
}

#[test]
fn test_stumaxl_32() {
    test_ldst("stumaxl", lse::stumaxl_32);
}

#[test]
fn test_stumaxl_64() {
    test_ldst("stumaxl", lse::stumaxl_64);
}

#[test]
fn test_stumaxlb() {
    test_ldst("stumaxlb", lse::stumaxlb);
}

#[test]
fn test_stumaxlh() {
    test_ldst("stumaxlh", lse::stumaxlh);
}

#[test]
fn test_stumin_32() {
    test_ldst("stumin", lse::stumin_32);
}

#[test]
fn test_stumin_64() {
    test_ldst("stumin", lse::stumin_64);
}

#[test]
fn test_stuminb() {
    test_ldst("stuminb", lse::stuminb);
}

#[test]
fn test_stuminh() {
    test_ldst("stuminh", lse::stuminh);
}

#[test]
fn test_stuminl_32() {
    test_ldst("stuminl", lse::stuminl_32);
}

#[test]
fn test_stuminl_64() {
    test_ldst("stuminl", lse::stuminl_64);
}

#[test]
fn test_stuminlb() {
    test_ldst("stuminlb", lse::stuminlb);
}

#[test]
fn test_stuminlh() {
    test_ldst("stuminlh", lse::stuminlh);
}

#[test]
fn test_stur_32() {
    test_ldst("stur", |rt, rn| a64::stur_32(rt, rn, 0));
    test_ldst_offset_filter("stur", a64::stur_32, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_stur_64() {
    test_ldst("stur", |rt, rn| a64::stur_64(rt, rn, 0));
    test_ldst_offset_filter("stur", a64::stur_64, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_sturb() {
    test_ldst("sturb", |rt, rn| a64::sturb(rt, rn, 0));
    test_ldst_offset_filter("sturb", a64::sturb, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_sturh() {
    test_ldst("sturh", |rt, rn| a64::sturh(rt, rn, 0));
    test_ldst_offset_filter("sturh", a64::sturh, |_, _, offset| {
        offset != 0 && is_signed_nbit_integer(9, offset)
    });
}

#[test]
fn test_stxp_32() {
    test_disasm("stxp", "w3, w4, w5, [x6]", a64::stxp_32(W3, W4, W5, X6));
}

#[test]
fn test_stxp_64() {
    test_disasm("stxp", "w3, x4, x5, [x6]", a64::stxp_64(W3, X4, X5, X6));
}

#[test]
fn test_stxr_32() {
    test_ldstpair("stxr", a64::stxr_32);
}

#[test]
fn test_stxr_64() {
    test_ldstpair("stxr", a64::stxr_64);
}

#[test]
fn test_stxrb() {
    test_ldstpair("stxrb", a64::stxrb);
}

#[test]
fn test_stxrh() {
    test_ldstpair("stxrh", a64::stxrh);
}

#[test]
fn test_sub_ext_32() {
    test4("sub", a64::sub_ext_32);
}

#[test]
fn test_sub_ext_64() {
    test_xdsp_xnsp_xm_extend("sub", a64::sub_ext_64);
}

#[test]
fn test_sub_imm_32() {
    test3("sub", a64::sub_imm_32::<u8>);
    test3("sub", a64::sub_imm_32::<ShiftedImm12>);
}

#[test]
fn test_sub_imm_64() {
    test3("sub", a64::sub_imm_64::<u8>);
    test3("sub", a64::sub_imm_64::<ShiftedImm12>);
}

#[test]
fn test_sub_shift_32() {
    test4_filter("sub", a64::sub_shift_32, |_, a2, _, shift| {
        a2 != WZR && !matches!(shift, Shift::Ror(_))
    });
    test3_filter("neg", |rd, rm, shift| a64::sub_shift_32(rd, WZR, rm, shift), |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_sub_shift_64() {
    test4_filter("sub", a64::sub_shift_64, |_, a2, _, shift| {
        a2 != XZR && !matches!(shift, Shift::Ror(_))
    });
    test3_filter("neg", |rd, rm, shift| a64::sub_shift_64(rd, XZR, rm, shift), |_, _, shift| {
        !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_subs_ext_32() {
    test4_filter("subs", a64::subs_ext_32, |a1, _, _, _| a1 != WZR);
}

#[test]
fn test_subs_ext_64() {
    test_disasm("subs", "x3, x4, w5, uxth #2", a64::subs_ext_64(X3, X4, X5, Extend::Uxth(2)));
    test_disasm("cmp", "x4, w5, uxth #2", a64::subs_ext_64(XZR, X4, X5, Extend::Uxth(2)));
    test_disasm("subs", "x3, sp, w5, uxth #2", a64::subs_ext_64(X3, XSP, X5, Extend::Uxth(2)));
    test_disasm("subs", "x3, x4, wzr, uxth #2", a64::subs_ext_64(X3, X4, XZR, Extend::Uxth(2)));
}

#[test]
fn test_subs_imm_32() {
    test3_filter("subs", a64::subs_imm_32::<u8>, |a1, _, _| a1 != WZR);
    test3_filter("subs", a64::subs_imm_32::<ShiftedImm12>, |a1, _, _| a1 != WZR);
}

#[test]
fn test_subs_imm_64() {
    test3_filter("subs", a64::subs_imm_64::<u8>, |a1, _, _| a1 != XZR);
    test3_filter("subs", a64::subs_imm_64::<ShiftedImm12>, |a1, _, _| a1 != XZR);
}

#[test]
fn test_subs_shift_32() {
    test4_filter("subs", a64::subs_shift_32, |a1, a2, _, shift| {
        a1 != WZR && a2 != WZR && !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_subs_shift_64() {
    test4_filter("subs", a64::subs_shift_64, |a1, a2, _, shift| {
        a1 != XZR && a2 != XZR && !matches!(shift, Shift::Ror(_))
    });
}

#[test]
fn test_svc() {
    test1("svc", a64::svc);
}

#[test]
fn test_swp_32() {
    test_ldstpair("swp", lse::swp_32);
}

#[test]
fn test_swp_64() {
    test_ldstpair("swp", lse::swp_64);
}

#[test]
fn test_swpa_32() {
    test_ldstpair("swpa", lse::swpa_32);
}

#[test]
fn test_swpa_64() {
    test_ldstpair("swpa", lse::swpa_64);
}

#[test]
fn test_swpab() {
    test_ldstpair("swpab", lse::swpab);
}

#[test]
fn test_swpah() {
    test_ldstpair("swpah", lse::swpah);
}

#[test]
fn test_swpal_32() {
    test_ldstpair("swpal", lse::swpal_32);
}

#[test]
fn test_swpal_64() {
    test_ldstpair("swpal", lse::swpal_64);
}

#[test]
fn test_swpalb() {
    test_ldstpair("swpalb", lse::swpalb);
}

#[test]
fn test_swpalh() {
    test_ldstpair("swpalh", lse::swpalh);
}

#[test]
fn test_swpb() {
    test_ldstpair("swpb", lse::swpb);
}

#[test]
fn test_swph() {
    test_ldstpair("swph", lse::swph);
}

#[test]
fn test_swpl_32() {
    test_ldstpair("swpl", lse::swpl_32);
}

#[test]
fn test_swpl_64() {
    test_ldstpair("swpl", lse::swpl_64);
}

#[test]
fn test_swplb() {
    test_ldstpair("swplb", lse::swplb);
}

#[test]
fn test_swplh() {
    test_ldstpair("swplh", lse::swplh);
}

#[test]
fn test_sxtb_32() {
    test2("sxtb", a64::sxtb_32);
}

#[test]
fn test_sxtb_64() {
    test2("sxtb", a64::sxtb_64);
}

#[test]
fn test_sxth_32() {
    test2("sxth", a64::sxth_32);
}

#[test]
fn test_sxth_64() {
    test2("sxth", a64::sxth_64);
}

#[test]
fn test_sxtw() {
    test2("sxtw", a64::sxtw);
}

#[test]
fn test_sys() {
    test_disasm("sys", "#3, c4, c5, #6, x7", a64::sys(3, 4, 5, 6, X7));
}

#[test]
fn test_sysl() {
    test_disasm("sysl", "x3, #4, c5, c6, #7", a64::sysl(X3, 4, 5, 6, 7));
}

#[test]
fn test_tbnz() {
    test3_filter("tbnz", a64::tbnz, |_, imm, label| {
        imm >= 32 && imm <= 63
            && label.byte_offset() & 3 == 0 && is_signed_nbit_integer(16, label.byte_offset())
    });
    test3_filter("tbnz", |rt: Register32, imm, label| a64::tbnz(rt.to_register64(), imm, label),
        |_, imm, label| {
            imm < 32
                && label.byte_offset() & 3 == 0 && is_signed_nbit_integer(16, label.byte_offset())
        });
}

#[test]
fn test_tbz() {
    test3_filter("tbz", a64::tbz, |_, imm, label| {
        imm >= 32 && imm <= 63
            && label.byte_offset() & 3 == 0 && is_signed_nbit_integer(16, label.byte_offset())
    });
    test3_filter("tbz", |rt: Register32, imm, label| a64::tbz(rt.to_register64(), imm, label),
        |_, imm, label| {
            imm < 32
                && label.byte_offset() & 3 == 0 && is_signed_nbit_integer(16, label.byte_offset())
        });
}

#[test]
fn test_tlbi() {
    test_disasm("tlbi", "vae1, xzr", a64::tlbi(0, 7, 1, XZR));
}

#[test]
fn test_tsb_csync() {
    test_disasm("tsb", "csync", trf::tsb_csync());
}

#[test]
fn test_tst_imm_32() {
    test2("tst", a64::tst_imm_32);
}

#[test]
fn test_tst_imm_64() {
    test2("tst", a64::tst_imm_64);
}

#[test]
fn test_tst_reg_32() {
    test3("tst", a64::tst_reg_32);
}

#[test]
fn test_tst_reg_64() {
    test3("tst", a64::tst_reg_64);
}

#[test]
fn test_ubfiz_32() {
    test4_filter("ubfiz", a64::ubfiz_32, |_, _, lsb, width| {
        lsb > 0 && lsb <= 31 && width > 0 && width <= 31 - lsb
    });
}

#[test]
fn test_ubfiz_64() {
    test4_filter("ubfiz", a64::ubfiz_64, |_, _, lsb, width| {
        lsb > 0 && lsb <= 63 && width > 0 && width <= 63 - lsb
    });
}

#[test]
fn test_ubfm_32() {
    test_disasm("lsl", "w3, w4, #5", a64::ubfm_32(W3, W4, 0x1b, 26));
    test_disasm("lsr", "w3, w4, #5", a64::ubfm_32(W3, W4, 0x5, 0x1f));
    test_disasm("ubfiz", "w3, w4, #5, #6", a64::ubfm_32(W3, W4, 0x1b, 5));
    test_disasm("ubfx", "w3, w4, #5, #6", a64::ubfm_32(W3, W4, 5, 10));
    test_disasm("uxtb", "w3, w4", a64::ubfm_32(W3, W4, 0, 7));
    test_disasm("uxth", "w3, w4", a64::ubfm_32(W3, W4, 0, 0xf));
}

#[test]
fn test_ubfm_64() {
    test_disasm("lsl", "x3, x4, #5", a64::ubfm_64(X3, X4, 0x3b, 58));
    test_disasm("lsr", "x3, x4, #5", a64::ubfm_64(X3, X4, 0x5, 0x3f));
    test_disasm("ubfiz", "x3, x4, #5, #6", a64::ubfm_64(X3, X4, 0x3b, 5));
    test_disasm("ubfx", "x3, x4, #5, #6", a64::ubfm_64(X3, X4, 5, 10));
}

#[test]
fn test_ubfx_32() {
    test4_filter("ubfx", a64::ubfx_32, |_, _, lsb, width| {
        lsb <= 31 && width > 0 && width <= 31 - lsb
            && (lsb, width) != (0, 8)
            && (lsb, width) != (0, 16)
    });
}

#[test]
fn test_ubfx_64() {
    test4_filter("ubfx", a64::ubfx_64,
        |_, _, lsb, width| lsb <= 63 && width > 0 && width <= 63 - lsb);
}

#[test]
fn test_udf() {
    assert_eq!(16, a64::udf(16));
}

#[test]
fn test_udiv_32() {
    test3("udiv", a64::udiv_32);
}

#[test]
fn test_udiv_64() {
    test3("udiv", a64::udiv_64);
}

#[test]
fn test_umaddl() {
    test4_filter("umaddl", a64::umaddl, |_, _, _, a4| a4 != XZR);
}

#[test]
fn test_umnegl() {
    test3("umnegl", a64::umnegl);
}

#[test]
fn test_umsubl() {
    test4_filter("umsubl", a64::umsubl, |_, _, _, a4| a4 != XZR);
}

#[test]
fn test_umulh() {
    test3("umulh", a64::umulh);
}

#[test]
fn test_umull() {
    test3("umull", a64::umull);
}

#[test]
fn test_uxtb() {
    test2("uxtb", a64::uxtb);
}

#[test]
fn test_uxth() {
    test2("uxth", a64::uxth);
}

#[test]
fn test_wfe() {
    test_disasm("wfe", "", a64::wfe());
}

#[test]
fn test_wfi() {
    test_disasm("wfi", "", a64::wfi());
}

#[test]
#[ignore]
fn test_xaflag() {
    test_disasm("xaflag", "", flagm2::xaflag());
}

#[test]
fn test_xpacd() {
    test1("xpacd", pauth::xpacd);
}

#[test]
fn test_xpaci() {
    test1("xpaci", pauth::xpaci);
}

#[test]
fn test_xpaclri() {
    test_disasm("xpaclri", "", pauth::xpaclri());
}

#[test]
fn test_yield() {
    test_disasm("yield", "", a64::yield_());
}
