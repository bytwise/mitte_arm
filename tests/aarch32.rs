extern crate mitte_arm;
extern crate capstone;

use std::convert::TryInto;

use mitte_arm::aarch32::*;
use mitte_arm::aarch32::Register::*;
use mitte_arm::aarch32::Ror::*;

use capstone::Capstone;
use capstone::arch::{BuildsCapstone, BuildsCapstoneExtraMode};
use capstone::arch::arm::{ArchMode, ArchExtraMode};


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
    let capstone = Capstone::new().arm()
        .mode(ArchMode::Arm)
        .extra_mode([ArchExtraMode::V8].iter().copied())
        .build().unwrap();
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


impl TestCases for Register {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (R0, "r0".into()),
            (R1, "r1".into()),
            (R2, "r2".into()),
            (R13, "sp".into()),
            (R15, "pc".into()),
        ]
    }
}

impl TestCases for RegisterPair {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (RegisterPair::R0R1, "r0, r1".into()),
            (RegisterPair::R2R3, "r2, r3".into()),
            (RegisterPair::R4R5, "r4, r5".into()),
            (RegisterPair::R6R7, "r6, r7".into()),
            (RegisterPair::R8R9, "r8, sb".into()),
            (RegisterPair::R10R11, "sl, fp".into()),
            (RegisterPair::R12R13, "ip, sp".into()),
        ]
    }
}

impl TestCases for RegisterList {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            ([R0].into(), "{r0}".into()),
            ([R5].into(), "{r5}".into()),
            ([R15].into(), "{pc}".into()),
            ([R0, R5].into(), "{r0, r5}".into()),
            ([R0, R15].into(), "{r0, pc}".into()),
            ([R5, R15].into(), "{r5, pc}".into()),
            ([R0, R5, R15].into(), "{r0, r5, pc}".into()),
            (
                [R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14].into(),
                "{r0, r1, r2, r3, r4, r5, r6, r7, r8, sb, sl, fp, ip, sp, lr}".into()
            ),
            (
                [R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15].into(),
                "{r0, r1, r2, r3, r4, r5, r6, r7, r8, sb, sl, fp, ip, sp, lr, pc}".into()
            ),
        ]
    }
}

impl TestCases for &'static [Register] {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (&[R0], "{r0}".into()),
            (&[R5], "{r5}".into()),
            (&[R15], "{pc}".into()),
            (&[R0, R5], "{r0, r5}".into()),
            (&[R0, R15], "{r0, pc}".into()),
            (&[R5, R15], "{r5, pc}".into()),
            (&[R0, R5, R15], "{r0, r5, pc}".into()),
            (
                &[R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14],
                "{r0, r1, r2, r3, r4, r5, r6, r7, r8, sb, sl, fp, ip, sp, lr}".into()
            ),
            (
                &[R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15],
                "{r0, r1, r2, r3, r4, r5, r6, r7, r8, sb, sl, fp, ip, sp, lr, pc}".into()
            ),
        ]
    }
}

impl TestCases for BankedRegister {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (BankedRegister::R8_usr, "r8_usr".into()),
            (BankedRegister::R9_usr, "r9_usr".into()),
            (BankedRegister::R10_usr, "r10_usr".into()),
            (BankedRegister::R11_usr, "r11_usr".into()),
            (BankedRegister::R12_usr, "r12_usr".into()),
            (BankedRegister::SP_usr, "sp_usr".into()),
            (BankedRegister::LR_usr, "lr_usr".into()),
            (BankedRegister::R8_fiq, "r8_fiq".into()),
            (BankedRegister::R9_fiq, "r9_fiq".into()),
            (BankedRegister::R10_fiq, "r10_fiq".into()),
            (BankedRegister::R11_fiq, "r11_fiq".into()),
            (BankedRegister::R12_fiq, "r12_fiq".into()),
            (BankedRegister::SP_fiq, "sp_fiq".into()),
            (BankedRegister::LR_fiq, "lr_fiq".into()),
            (BankedRegister::LR_irq, "lr_irq".into()),
            (BankedRegister::SP_irq, "sp_irq".into()),
            (BankedRegister::LR_svc, "lr_svc".into()),
            (BankedRegister::SP_svc, "sp_svc".into()),
            (BankedRegister::LR_abt, "lr_abt".into()),
            (BankedRegister::SP_abt, "sp_abt".into()),
            (BankedRegister::LR_und, "lr_und".into()),
            (BankedRegister::SP_und, "sp_und".into()),
            // (BankedRegister::LR_mon, "lr_mon".into()),
            // (BankedRegister::SP_mon, "sp_mon".into()),
            // (BankedRegister::ELR_hyp, "elr_hyp".into()),
            // (BankedRegister::SP_hyp, "sp_hyp".into()),
            (BankedRegister::SPSR_fiq, "spsr_fiq".into()),
            (BankedRegister::SPSR_irq, "spsr_irq".into()),
            (BankedRegister::SPSR_svc, "spsr_svc".into()),
            (BankedRegister::SPSR_abt, "spsr_abt".into()),
            (BankedRegister::SPSR_und, "spsr_und".into()),
            (BankedRegister::SPSR_mon, "spsr_mon".into()),
            (BankedRegister::SPSR_hyp, "spsr_hyp".into()),
        ]
    }
}

impl TestCases for Label {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Label::from_byte_offset(0), "#0".into()),
            (Label::from_byte_offset(0xff_fffc), "#0xfffffc".into()),
            (Label::from_byte_offset(0x1ff_fffc), "#0x1fffffc".into()),
            (Label::from_byte_offset(-0x80_0000), "#0xff800000".into()),
            (Label::from_byte_offset(-0x1ff_fff8), "#0xfe000008".into()),
        ]
    }
}

impl TestCases for Const {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0u32.try_into().unwrap(), "#0".into()),
            (5u32.try_into().unwrap(), "#5".into()),
            (0x50u32.try_into().unwrap(), "#0x50".into()),
            (0x500u32.try_into().unwrap(), "#0x500".into()),
            (0x5000u32.try_into().unwrap(), "#0x5000".into()),
            (0x5_0000u32.try_into().unwrap(), "#0x50000".into()),
            (0x50_0000u32.try_into().unwrap(), "#0x500000".into()),
            (0x500_0000u32.try_into().unwrap(), "#0x5000000".into()),
            (0x5000_0000u32.try_into().unwrap(), "#0x50000000".into()),
            (0x5000_0006u32.try_into().unwrap(), "#0x50000006".into()),
        ]
    }
}

impl TestCases for Shift<Register> {
    fn test_cases() -> Vec<(Self, String)> {
        let mut test_cases = Vec::new();
        for reg in Register::test_cases() {
            test_cases.push((Shift::Lsl(reg.0), format!("lsl {}", reg.1)));
            test_cases.push((Shift::Lsr(reg.0), format!("lsr {}", reg.1)));
            test_cases.push((Shift::Asr(reg.0), format!("asr {}", reg.1)));
            test_cases.push((Shift::Ror(reg.0), format!("ror {}", reg.1)));
        }
        test_cases
    }
}

impl TestCases for Offset<u8> {
    fn test_cases() -> Vec<(Self, String)> {
        let mut test_cases = Vec::new();
        for offset in u8::test_cases() {
            let n = match offset.0 {
                0..=9 => format!("{}", offset.0),
                _ => format!("0x{:x}", offset.0),
            };
            test_cases.push((Offset::Sub(offset.0), format!("#-{}", n)));
            if offset.0 != 0 {
                test_cases.push((Offset::Add(offset.0), format!("#{}", n)));
            }
        }
        test_cases
    }
}

impl TestCases for Offset<u16> {
    fn test_cases() -> Vec<(Self, String)> {
        let mut test_cases = Vec::new();
        for offset in u16::test_cases() {
            let n = match offset.0 {
                0..=9 => format!("{}", offset.0),
                _ => format!("0x{:x}", offset.0),
            };
            test_cases.push((Offset::Sub(offset.0), format!("#-{}", n)));
            if offset.0 != 0 {
                test_cases.push((Offset::Add(offset.0), format!("#{}", n)));
            }
        }
        test_cases
    }
}

impl TestCases for Offset<Register> {
    fn test_cases() -> Vec<(Self, String)> {
        let mut test_cases = Vec::new();
        for reg in Register::test_cases() {
            test_cases.push((Offset::Sub(reg.0), format!("-{}", reg.1)));
            test_cases.push((Offset::Add(reg.0), format!("{}", reg.1)));
        }
        test_cases
    }
}

impl TestCases for Mem {
    fn test_cases() -> Vec<(Self, String)> {
        let mut test_cases = Vec::new();
        for reg in Register::test_cases() {
            test_cases.push((Mem::writeback(reg.0),
                             format!("{}!", reg.1)));
            test_cases.push((Mem::from(reg.0), reg.1));
        }
        test_cases
    }
}

impl TestCases for Index {
    fn test_cases() -> Vec<(Self, String)> {
        let mut test_cases = Vec::new();
        for index in Register::test_cases() {
            for shift in Shift::test_cases() {
                test_cases.push((Index::Sub(index.0, shift.0),
                                 format!("-{}, {}", index.1, shift.1)));
                test_cases.push((Index::Add(index.0, shift.0),
                                 format!("{}, {}", index.1, shift.1)));
            }
        }
        test_cases
    }
}

impl TestCases for Ror {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Ror0, "ror #0".into()),
            (Ror8, "ror #8".into()),
            (Ror16, "ror #16".into()),
            (Ror24, "ror #24".into()),
        ]
    }
}


#[track_caller]
fn test1_filter_cond(
    mnemonic: &str,
    f: fn(Condition) -> u32,
    mut filter: impl FnMut(Condition) -> bool
) {
    for (a1, s1) in Condition::test_cases() {
        if filter(a1) {
            let mnemonic = if a1 == Condition::Al {
                String::from(mnemonic)
            } else {
                format!("{}{}", mnemonic, s1)
            };
            test_disasm(&mnemonic, "", f(a1));
        }
    }
}

#[track_caller]
fn test1_cond(mnemonic: &str, f: fn(Condition) -> u32) {
    test1_filter_cond(mnemonic, f, |_| true);
}

#[track_caller]
fn test1<A1>(mnemonic: &str, f: fn(A1) -> u32)
    where A1: TestCases
{
    for (a1, s1) in A1::test_cases() {
        test_disasm(mnemonic,
                    format!("{}", s1),
                    f(a1));
    }
}

#[track_caller]
fn test2_format_filter<A1, A2>(
    mnemonic: impl Fn(A1, &str) -> String,
    f: fn(A1, A2) -> u32,
    s: impl Fn(&str, &str) -> String,
    mut filter: impl FnMut(A1, A2) -> bool
)
    where A1: TestCases, A2: TestCases
{
    for (a1, s1) in A1::test_cases() {
        for (a2, s2) in A2::test_cases() {
            if filter(a1, a2) {
                test_disasm(&mnemonic(a1, &s1), s(&s1, &s2), f(a1, a2));
            }
        }
    }
}

#[track_caller]
fn test2_format_filter_cond<A2>(
    mnemonic: &str,
    f: fn(Condition, A2) -> u32,
    s: fn(&str) -> String,
    filter: impl FnMut(Condition, A2) -> bool
)
    where A2: TestCases
{
    test2_format_filter(|a1, s1| {
        if a1 == Condition::Al {
            String::from(mnemonic)
        } else {
            format!("{}{}", mnemonic, s1)
        }
    }, f, |_, s2| s(s2), filter)
}

#[track_caller]
fn test2_filter<A1, A2>(
    mnemonic: &str,
    f: fn(A1, A2) -> u32,
    filter: impl FnMut(A1, A2) -> bool
)
    where A1: TestCases, A2: TestCases
{
    test2_format_filter(|_, _| String::from(mnemonic),
        f,
        |s1, s2| format!("{}, {}", s1, s2),
        filter)
}

#[track_caller]
fn test2_filter_cond<A2>(
    mnemonic: &str,
    f: fn(Condition, A2) -> u32,
    filter: impl FnMut(Condition, A2) -> bool
)
    where A2: TestCases
{
    test2_format_filter_cond(mnemonic, f, |s2| format!("{}", s2), filter)
}

#[track_caller]
fn test2_format<A1, A2>(
    mnemonic: &str,
    f: fn(A1, A2) -> u32,
    s: fn(&str, &str) -> String
)
    where A1: TestCases, A2: TestCases
{
    test2_format_filter(|_, _| String::from(mnemonic), f, s, |_, _| true);
}

#[track_caller]
fn test2_format_cond<A2>(
    mnemonic: &str,
    f: fn(Condition, A2) -> u32,
    s: fn(&str) -> String
)
    where A2: TestCases
{
    test2_format_filter_cond(mnemonic, f, s, |_, _| true);
}

#[track_caller]
fn test2<A1, A2>(mnemonic: &str, f: fn(A1, A2) -> u32)
    where A1: TestCases, A2: TestCases
{
    test2_filter(mnemonic, f, |_, _| true);
}

#[track_caller]
fn test2_cond<A2>(mnemonic: &str, f: fn(Condition, A2) -> u32)
    where A2: TestCases
{
    test2_filter_cond(mnemonic, f, |_, _| true);
}

#[track_caller]
fn test3_format_filter<A1, A2, A3>(
    mnemonic: impl Fn(A1, &str) -> String,
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
                    test_disasm(&mnemonic(a1, &s1), s(&s1, &s2, &s3), f(a1, a2, a3));
                }
            }
        }
    }
}

#[track_caller]
fn test3_format_filter_cond<A2, A3>(
    mnemonic: &str,
    f: fn(Condition, A2, A3) -> u32,
    s: fn(&str, &str) -> String,
    filter: impl FnMut(Condition, A2, A3) -> bool
)
    where A2: TestCases, A3: TestCases
{
    test3_format_filter(|a1, s1| {
        if a1 == Condition::Al {
            String::from(mnemonic)
        } else {
            format!("{}{}", mnemonic, s1)
        }
    }, f, |_, s2, s3| s(s2, s3), filter)
}

#[track_caller]
fn test3_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(|_, _| String::from(mnemonic),
        f,
        |s1, s2, s3| format!("{}, {}, {}", s1, s2, s3),
        filter)
}

#[track_caller]
fn test3_filter_cond<A2, A3>(
    mnemonic: &str,
    f: fn(Condition, A2, A3) -> u32,
    filter: impl FnMut(Condition, A2, A3) -> bool
)
    where A2: TestCases, A3: TestCases
{
    test3_format_filter_cond(mnemonic, f, |s2, s3| format!("{}, {}", s2, s3), filter)
}

#[track_caller]
fn test3_format<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    s: fn(&str, &str, &str) -> String
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(|_, _| String::from(mnemonic), f, s, |_, _, _| true);
}

#[track_caller]
fn test3_format_cond<A2, A3>(
    mnemonic: &str,
    f: fn(Condition, A2, A3) -> u32,
    s: fn(&str, &str) -> String
)
    where A2: TestCases, A3: TestCases
{
    test3_format_filter_cond(mnemonic, f, s, |_, _, _| true);
}

#[track_caller]
fn test3<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_filter(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test3_cond<A2, A3>(mnemonic: &str, f: fn(Condition, A2, A3) -> u32)
    where A2: TestCases, A3: TestCases
{
    test3_filter_cond(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test4_format_filter<A1, A2, A3, A4>(
    mnemonic: impl Fn(A1, &str) -> String,
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
                        test_disasm(&mnemonic(a1, &s1),
                                    s(&s1, &s2, &s3, &s4),
                                    f(a1, a2, a3, a4));
                    }
                }
            }
        }
    }
}

#[track_caller]
fn test4_format_filter_cond<A2, A3, A4>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4) -> u32,
    s: fn(&str, &str, &str) -> String,
    filter: impl FnMut(Condition, A2, A3, A4) -> bool
)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter(|a1, s1| {
        if a1 == Condition::Al {
            String::from(mnemonic)
        } else {
            format!("{}{}", mnemonic, s1)
        }
    }, f, |_, s2, s3, s4| s(s2, s3, s4), filter)
}

#[track_caller]
fn test4_filter<A1, A2, A3, A4>(
    mnemonic: &str,
    f: fn(A1, A2, A3, A4) -> u32,
    filter: impl FnMut(A1, A2, A3, A4) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter(|_, _| String::from(mnemonic),
        f,
        |s1, s2, s3, s4| format!("{}, {}, {}, {}", s1, s2, s3, s4),
        filter)
}

#[track_caller]
fn test4_filter_cond<A2, A3, A4>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4) -> u32,
    filter: impl FnMut(Condition, A2, A3, A4) -> bool
)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter_cond(mnemonic, f, |s2, s3, s4| format!("{}, {}, {}", s2, s3, s4), filter)
}

#[track_caller]
fn test4_format_cond<A2, A3, A4>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4) -> u32,
    s: fn(&str, &str, &str) -> String
)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter_cond(mnemonic, f, s, |_, _, _, _| true)
}

#[track_caller]
fn test4<A1, A2, A3, A4>(mnemonic: &str, f: fn(A1, A2, A3, A4) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_filter(mnemonic, f, |_, _, _, _| true);
}

#[track_caller]
fn test4_cond<A2, A3, A4>(mnemonic: &str, f: fn(Condition, A2, A3, A4) -> u32)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_filter_cond(mnemonic, f, |_, _, _, _| true);
}

#[track_caller]
fn test5_format_filter_cond<A2, A3, A4, A5>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4, A5) -> u32,
    s: fn(&str, &str, &str, &str) -> String,
    mut filter: impl FnMut(Condition, A2, A3, A4, A5) -> bool
)
    where A2: TestCases, A3: TestCases, A4: TestCases, A5: TestCases
{
    for (a1, s1) in Condition::test_cases() {
        for (a2, s2) in A2::test_cases() {
            for (a3, s3) in A3::test_cases() {
                for (a4, s4) in A4::test_cases() {
                    for (a5, s5) in A5::test_cases() {
                        if filter(a1, a2, a3, a4, a5) {
                            let mnemonic = if a1 == Condition::Al {
                                String::from(mnemonic)
                            } else {
                                format!("{}{}", mnemonic, s1)
                            };
                            test_disasm(&mnemonic,
                                        s(&s2, &s3, &s4, &s5),
                                        f(a1, a2, a3, a4, a5));
                        }
                    }
                }
            }
        }
    }
}

#[track_caller]
fn test5_filter_cond<A2, A3, A4, A5>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4, A5) -> u32,
    filter: impl FnMut(Condition, A2, A3, A4, A5) -> bool
)
    where A2: TestCases, A3: TestCases, A4: TestCases, A5: TestCases
{
    test5_format_filter_cond(mnemonic, f,
        |s2, s3, s4, s5| format!("{}, {}, {}, {}", s2, s3, s4, s5),
        filter)
}

#[track_caller]
fn test5_cond<A2, A3, A4, A5>(mnemonic: &str, f: fn(Condition, A2, A3, A4, A5) -> u32)
    where A2: TestCases, A3: TestCases, A4: TestCases, A5: TestCases
{
    test5_filter_cond(mnemonic, f, |_, _, _, _, _| true);
}


#[track_caller]
fn test_ldst<A1, A2>(mnemonic: &str, f: fn(A1, A2) -> u32)
    where A1: TestCases, A2: TestCases
{
    test2_format(mnemonic, f, |s1, s2| format!("{}, [{}]", s1, s2))
}

#[track_caller]
fn test_ldst_cond<A2, A3>(mnemonic: &str, f: fn(Condition, A2, A3) -> u32)
    where A2: TestCases, A3: TestCases
{
    test3_format_cond(mnemonic, f, |s2, s3| format!("{}, [{}]", s2, s3))
}

#[track_caller]
fn test_ldst_offset_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(|_, _| String::from(mnemonic),
        f,
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
fn test_ldst_offset_filter_cond<A2, A3, A4>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4) -> u32,
    filter: impl FnMut(Condition, A2, A3, A4) -> bool
)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter_cond(mnemonic, f,
        |s2, s3, s4| format!("{}, [{}, {}]", s2, s3, s4),
        filter);
}

#[track_caller]
fn test_ldst_offset_cond<A2, A3, A4>(mnemonic: &str, f: fn(Condition, A2, A3, A4) -> u32)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test_ldst_offset_filter_cond(mnemonic, f, |_, _, _, _| true);
}

#[track_caller]
fn test_ldst_post_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(|_, _| String::from(mnemonic),
        f,
        |s1, s2, s3| format!("{}, [{}], {}", s1, s2, s3),
        filter);
}

#[track_caller]
fn test_ldst_post<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test_ldst_post_filter(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test_ldst_post_filter_cond<A2, A3, A4>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4) -> u32,
    filter: impl FnMut(Condition, A2, A3, A4) -> bool
)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter_cond(mnemonic, f,
        |s2, s3, s4| format!("{}, [{}], {}", s2, s3, s4),
        filter);
}

#[track_caller]
fn test_ldst_post_cond<A2, A3, A4>(mnemonic: &str, f: fn(Condition, A2, A3, A4) -> u32)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test_ldst_post_filter_cond(mnemonic, f, |_, _, _, _| true);
}

#[track_caller]
fn test_ldst_pre_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(|_, _| String::from(mnemonic),
        f,
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
fn test_ldst_pre_filter_cond<A2, A3, A4>(
    mnemonic: &str,
    f: fn(Condition, A2, A3, A4) -> u32,
    filter: impl FnMut(Condition, A2, A3, A4) -> bool
)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test4_format_filter_cond(mnemonic, f,
        |s2, s3, s4| format!("{}, [{}, {}]!", s2, s3, s4),
        filter);
}

#[track_caller]
fn test_ldst_pre_cond<A2, A3, A4>(mnemonic: &str, f: fn(Condition, A2, A3, A4) -> u32)
    where A2: TestCases, A3: TestCases, A4: TestCases
{
    test_ldst_pre_filter_cond(mnemonic, f, |_, _, _, _| true);
}


#[test]
fn test_adc_imm() {
    test3("adc", a32::adc_imm::<u8>);
    test3("adc", a32::adc_imm::<Const>);
    test4_cond("adc", a32::adc_imm::cond::<u8>);
    test4_cond("adc", a32::adc_imm::cond::<Const>);
}

#[test]
fn test_adc_reg() {
    test4("adc", a32::adc_reg);
    test5_cond("adc", a32::adc_reg::cond);
}

#[test]
fn test_adc_reg_rrx() {
    test3_format("adc", a32::adc_reg_rrx,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
    test4_format_cond("adc", a32::adc_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_adc_reg_shifted_reg() {
    test4("adc", a32::adc_reg_shifted_reg);
    test5_cond("adc", a32::adc_reg_shifted_reg::cond);
}

#[test]
fn test_adcs_imm() {
    test3("adcs", a32::adcs_imm::<u8>);
    test3("adcs", a32::adcs_imm::<Const>);
    test4_cond("adcs", a32::adcs_imm::cond::<u8>);
    test4_cond("adcs", a32::adcs_imm::cond::<Const>);
}

#[test]
fn test_adcs_reg() {
    test4("adcs", a32::adcs_reg);
    test5_cond("adcs", a32::adcs_reg::cond);
}

#[test]
fn test_adcs_reg_rrx() {
    test3_format("adcs", a32::adcs_reg_rrx,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
    test4_format_cond("adcs", a32::adcs_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_adcs_reg_shifted_reg() {
    test4("adcs", a32::adcs_reg_shifted_reg);
    test5_cond("adcs", a32::adcs_reg_shifted_reg::cond);
}

#[test]
fn test_add_imm() {
    test3("add", a32::add_imm::<u8>);
    test3("add", a32::add_imm::<Const>);
    test4_cond("add", a32::add_imm::cond::<u8>);
    test4_cond("add", a32::add_imm::cond::<Const>);
}

#[test]
fn test_add_reg() {
    test4("add", a32::add_reg);
    test5_cond("add", a32::add_reg::cond);
}

#[test]
fn test_add_reg_rrx() {
    test3_format("add", a32::add_reg_rrx,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
    test4_format_cond("add", a32::add_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_add_reg_shifted_reg() {
    test4("add", a32::add_reg_shifted_reg);
    test5_cond("add", a32::add_reg_shifted_reg::cond);
}

#[test]
fn test_adds_imm() {
    test3("adds", a32::adds_imm::<u8>);
    test3("adds", a32::adds_imm::<Const>);
    test4_cond("adds", a32::adds_imm::cond::<u8>);
    test4_cond("adds", a32::adds_imm::cond::<Const>);
}

#[test]
fn test_adds_reg() {
    test4("adds", a32::adds_reg);
    test5_cond("adds", a32::adds_reg::cond);
}

#[test]
fn test_adds_reg_rrx() {
    test3_format("adds", a32::adds_reg_rrx,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
    test4_format_cond("adds", a32::adds_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_adds_reg_shifted_reg() {
    test4("adds", a32::adds_reg_shifted_reg);
    test5_cond("adds", a32::adds_reg_shifted_reg::cond);
}

#[test]
fn test_adr_add() {
    test2_format("add", a32::adr_add::<u8>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
    test2_format("add", a32::adr_add::<Const>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
    test3_format_cond("add", a32::adr_add::cond::<u8>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
    test3_format_cond("add", a32::adr_add::cond::<Const>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
}

#[test]
fn test_adr_sub() {
    test2_format("sub", a32::adr_sub::<u8>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
    test2_format("sub", a32::adr_sub::<Const>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
    test3_format_cond("sub", a32::adr_sub::cond::<u8>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
    test3_format_cond("sub", a32::adr_sub::cond::<Const>,
        |rd, imm| format!("{}, pc, {}", rd, imm));
}

#[test]
fn test_and_imm() {
    test3("and", a32::and_imm::<u8>);
    test3("and", a32::and_imm::<Const>);
    test4_cond("and", a32::and_imm::cond::<u8>);
    test4_cond("and", a32::and_imm::cond::<Const>);
}

#[test]
fn test_and_reg() {
    test4("and", a32::and_reg);
    test5_cond("and", a32::and_reg::cond);
}

#[test]
fn test_and_reg_rrx() {
    test3_format("and", a32::and_reg_rrx,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
    test4_format_cond("and", a32::and_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_and_reg_shifted_reg() {
    test4("and", a32::and_reg_shifted_reg);
    test5_cond("and", a32::and_reg_shifted_reg::cond);
}

#[test]
fn test_ands_imm() {
    test3("ands", a32::ands_imm::<u8>);
    test3("ands", a32::ands_imm::<Const>);
    test4_cond("ands", a32::ands_imm::cond::<u8>);
    test4_cond("ands", a32::ands_imm::cond::<Const>);
}

#[test]
fn test_ands_reg() {
    test4("ands", a32::ands_reg);
    test5_cond("ands", a32::ands_reg::cond);
}

#[test]
fn test_ands_reg_rrx() {
    test3_format("ands", a32::ands_reg_rrx,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
    test4_format_cond("ands", a32::ands_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_ands_reg_shifted_reg() {
    test4("ands", a32::ands_reg_shifted_reg);
    test5_cond("ands", a32::ands_reg_shifted_reg::cond);
}

#[test]
fn test_asr_imm() {
    test3_filter("asr", a32::asr_imm, |_, _, imm5| imm5 > 0 && imm5 < 0x20);
    test4_filter_cond("asr", a32::asr_imm::cond, |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_asr_reg() {
    test3("asr", a32::asr_reg);
    test4_cond("asr", a32::asr_reg::cond);
}

#[test]
fn test_asrs_imm() {
    test3_filter("asrs", a32::asrs_imm, |_, _, imm5| imm5 > 0 && imm5 < 0x20);
    test4_filter_cond("asrs", a32::asrs_imm::cond, |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_asrs_reg() {
    test3("asrs", a32::asrs_reg);
    test4_cond("asrs", a32::asrs_reg::cond);
}

#[test]
fn test_b() {
    test1("b", a32::b);
    test2_cond("b", a32::b::cond);
}

#[test]
fn test_bfc() {
    test4_filter_cond("bfc", a32::bfc::cond, |_, rd, lsb, width| {
        rd != R15 && lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_bfi() {
    test5_filter_cond("bfi", a32::bfi::cond, |_, rd, rn, lsb, width| {
        rd != R15 && rn != R15 && lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_bic_imm() {
    test4_cond("bic", a32::bic_imm::cond::<u8>);
    test4_cond("bic", a32::bic_imm::cond::<Const>);
}

#[test]
fn test_bic_reg() {
    test5_cond("bic", a32::bic_reg::cond);
}

#[test]
fn test_bic_reg_rrx() {
    test4_format_cond("bic", a32::bic_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_bic_reg_shifted_reg() {
    test5_cond("bic", a32::bic_reg_shifted_reg::cond);
}

#[test]
fn test_bics_imm() {
    test4_cond("bics", a32::bics_imm::cond::<u8>);
    test4_cond("bics", a32::bics_imm::cond::<Const>);
}

#[test]
fn test_bics_reg() {
    test5_cond("bics", a32::bics_reg::cond);
}

#[test]
fn test_bics_reg_rrx() {
    test4_format_cond("bics", a32::bics_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_bics_reg_shifted_reg() {
    test5_cond("bics", a32::bics_reg_shifted_reg::cond);
}

#[test]
fn test_bkpt() {
    test_disasm("bkpt", "#0x1234", a32::bkpt(0x1234));
}

#[test]
fn test_bl() {
    test1("bl", a32::bl);
    test2_cond("bl", a32::bl::cond);
}

#[test]
fn test_blx_imm() {
    test1("blx", a32::blx_imm);
}

#[test]
fn test_blx_reg() {
    test2_filter_cond("blx", a32::blx_reg::cond, |_, rm| rm != R15);
}

#[test]
fn test_bx() {
    test2_filter_cond("bx", a32::bx::cond, |_, rm| rm != R15);
}

#[test]
fn test_bxj() {
    test2_filter_cond("bxj", a32::bxj::cond, |_, rm| rm != R15);
}

#[test]
fn test_clrex() {
    test_disasm("clrex", "", a32::clrex());
}

#[test]
fn test_clz() {
    test3_cond("clz", a32::clz::cond);
}

#[test]
fn test_cmn_imm() {
    test3_cond("cmn", a32::cmn_imm::cond::<u8>);
    test3_cond("cmn", a32::cmn_imm::cond::<Const>);
}

#[test]
fn test_cmn_reg() {
    test4_cond("cmn", a32::cmn_reg::cond);
}

#[test]
fn test_cmn_reg_rrx() {
    test3_format_cond("cmn", a32::cmn_reg_rrx::cond,
        |rn, rm| format!("{}, {}, rrx", rn, rm));
}

#[test]
fn test_cmn_reg_shifted_reg() {
    test4_cond("cmn", a32::cmn_reg_shifted_reg::cond);
}

#[test]
fn test_cmp_imm() {
    test3_cond("cmp", a32::cmp_imm::cond::<u8>);
    test3_cond("cmp", a32::cmp_imm::cond::<Const>);
}

#[test]
fn test_cmp_reg() {
    test4_cond("cmp", a32::cmp_reg::cond);
}

#[test]
fn test_cmp_reg_rrx() {
    test3_format_cond("cmp", a32::cmp_reg_rrx::cond,
        |rn, rm| format!("{}, {}, rrx", rn, rm));
}

#[test]
fn test_cmp_reg_shifted_reg() {
    test4_cond("cmp", a32::cmp_reg_shifted_reg::cond);
}

#[test]
fn test_cps() {
    test_disasm("cps", "#0", a32::cps(0));
    test_disasm("cps", "#5", a32::cps(5));
}

#[test]
fn test_cpsid() {
    test_disasm("cpsid", "a", a32::cpsid(true, false, false));
    test_disasm("cpsid", "i", a32::cpsid(false, true, false));
    test_disasm("cpsid", "f", a32::cpsid(false, false, true));
}

#[test]
fn test_cpsid_mode() {
    test_disasm("cpsid", "a, #0", a32::cpsid_mode(true, false, false, 0));
    test_disasm("cpsid", "a, #5", a32::cpsid_mode(true, false, false, 5));
    test_disasm("cpsid", "i, #0", a32::cpsid_mode(false, true, false, 0));
    test_disasm("cpsid", "f, #0", a32::cpsid_mode(false, false, true, 0));
}

#[test]
fn test_cpsie() {
    test_disasm("cpsie", "a", a32::cpsie(true, false, false));
    test_disasm("cpsie", "i", a32::cpsie(false, true, false));
    test_disasm("cpsie", "f", a32::cpsie(false, false, true));
}

#[test]
fn test_cpsie_mode() {
    test_disasm("cpsie", "a, #0", a32::cpsie_mode(true, false, false, 0));
    test_disasm("cpsie", "a, #5", a32::cpsie_mode(true, false, false, 5));
    test_disasm("cpsie", "i, #0", a32::cpsie_mode(false, true, false, 0));
    test_disasm("cpsie", "f, #0", a32::cpsie_mode(false, false, true, 0));
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
fn test_crc32h() {
    test3("crc32h", crc32::crc32h);
}

#[test]
fn test_crc32w() {
    test3("crc32w", crc32::crc32w);
}

#[test]
fn test_csdb() {
    test_disasm("csdb", "", a32::csdb());
    test1_cond("csdb", a32::csdb::cond);
}

#[test]
fn test_dbg() {
    test_disasm("dbg", "#5", a32::dbg(5));
    test_disasm("dbgge", "#5", a32::dbg::cond(Condition::Ge, 5));
}

#[test]
fn test_dmb() {
    test_disasm("dmb", "nshld", a32::dmb(5));
}

#[test]
fn test_dsb() {
    test_disasm("dsb", "nshld", a32::dsb(5));
}

#[test]
fn test_eor_imm() {
    test4_cond("eor", a32::eor_imm::cond::<u8>);
    test4_cond("eor", a32::eor_imm::cond::<Const>);
}

#[test]
fn test_eor_reg() {
    test5_cond("eor", a32::eor_reg::cond);
}

#[test]
fn test_eor_reg_rrx() {
    test4_format_cond("eor", a32::eor_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_eor_reg_shifted_reg() {
    test5_cond("eor", a32::eor_reg_shifted_reg::cond);
}

#[test]
fn test_eors_imm() {
    test4_cond("eors", a32::eors_imm::cond::<u8>);
    test4_cond("eors", a32::eors_imm::cond::<Const>);
}

#[test]
fn test_eors_reg() {
    test5_cond("eors", a32::eors_reg::cond);
}

#[test]
fn test_eors_reg_rrx() {
    test4_format_cond("eors", a32::eors_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_eors_reg_shifted_reg() {
    test5_cond("eors", a32::eors_reg_shifted_reg::cond);
}

#[test]
fn test_eret() {
    test1_cond("eret", a32::eret::cond);
}

#[test]
fn test_esb() {
    test_disasm("esb", "", ras::esb());
}

#[test]
fn test_hlt() {
    test_disasm("hlt", "#0x1234", armv8::hlt(0x1234));
}

#[test]
fn test_hvc() {
    test_disasm("hvc", "#0x1234", a32::hvc(0x1234));
}

#[test]
fn test_isb() {
    test_disasm("isb", "sy", a32::isb(0xf));
}

#[test]
fn test_lda() {
    test_ldst("lda", armv8::lda);
    test_ldst_cond("lda", armv8::lda::cond);
}

#[test]
fn test_ldab() {
    test_ldst("ldab", armv8::ldab);
    test_ldst_cond("ldab", armv8::ldab::cond);
}

#[test]
fn test_ldaex() {
    test_ldst("ldaex", armv8::ldaex);
    test_ldst_cond("ldaex", armv8::ldaex::cond);
}

#[test]
fn test_ldaexb() {
    test_ldst("ldaexb", armv8::ldaexb);
    test_ldst_cond("ldaexb", armv8::ldaexb::cond);
}

#[test]
fn test_ldaexd() {
    test_ldst("ldaexd", armv8::ldaexd);
    test_ldst_cond("ldaexd", armv8::ldaexd::cond);
}

#[test]
fn test_ldaexh() {
    test_ldst("ldaexh", armv8::ldaexh);
    test_ldst_cond("ldaexh", armv8::ldaexh::cond);
}

#[test]
fn test_ldah() {
    test_ldst("ldah", armv8::ldah);
    test_ldst_cond("ldah", armv8::ldah::cond);
}

#[test]
fn test_ldc_imm_offset() {
    test_disasm("ldc", "p14, c5, [r4, #-0x14]", a32::ldc_imm_offset(R4, Offset::Sub(5)));
    test_disasm("ldc", "p14, c5, [r4, #0x14]", a32::ldc_imm_offset(R4, Offset::Add(5)));
}

#[test]
fn test_ldc_imm_post() {
    test_disasm("ldc", "p14, c5, [r4], #-0x14", a32::ldc_imm_post(R4, Offset::Sub(5)));
    test_disasm("ldc", "p14, c5, [r4], #0x14", a32::ldc_imm_post(R4, Offset::Add(5)));
}

#[test]
fn test_ldc_imm_pre() {
    test_disasm("ldc", "p14, c5, [r4, #-0x14]!", a32::ldc_imm_pre(R4, Offset::Sub(5)));
    test_disasm("ldc", "p14, c5, [r4, #0x14]!", a32::ldc_imm_pre(R4, Offset::Add(5)));
}

#[test]
fn test_ldc_imm_unindexed() {
    test_disasm("ldc", "p14, c5, [r4], {6}", a32::ldc_imm_unindexed(R4, 6));
}

#[test]
fn test_ldc_lit_offset() {
    test_disasm("ldc", "p14, c5, [pc, #-0x14]", a32::ldc_lit_offset(Offset::Sub(5)));
    test_disasm("ldc", "p14, c5, [pc, #0x14]", a32::ldc_lit_offset(Offset::Add(5)));
}

#[test]
fn test_ldc_lit_unindexed() {
    test_disasm("ldc", "p14, c5, [pc], {6}", a32::ldc_lit_unindexed(6));
}

#[test]
fn test_ldm() {
    test3_filter_cond("ldm", a32::ldm::cond::<Mem, RegisterList>,
        |_, rn, _| rn != Mem::writeback(R13));
    test3_cond("ldm", a32::ldm::cond::<Register, &[Register]>);
}

#[test]
fn test_ldm_exception_return() {
    test_disasm("ldmda", "r3, {r4, pc} ^", a32::ldm_exception_return(AddressMode::DecrementAfter, R3, [R4, R15]));
    test_disasm("ldmdb", "r3, {r4, pc} ^", a32::ldm_exception_return(AddressMode::DecrementBefore, R3, [R4, R15]));
    test_disasm("ldm", "r3, {r4, pc} ^", a32::ldm_exception_return(AddressMode::IncrementAfter, R3, [R4, R15]));
    test_disasm("ldmib", "r3, {r4, pc} ^", a32::ldm_exception_return(AddressMode::IncrementBefore, R3, [R4, R15]));
}

#[test]
fn test_ldm_user_registers() {
    test_disasm("ldmda", "r3, {r4} ^", a32::ldm_user_registers(AddressMode::DecrementAfter, R3, [R4]));
    test_disasm("ldmdb", "r3, {r4} ^", a32::ldm_user_registers(AddressMode::DecrementBefore, R3, [R4]));
    test_disasm("ldm", "r3, {r4} ^", a32::ldm_user_registers(AddressMode::IncrementAfter, R3, [R4]));
    test_disasm("ldmib", "r3, {r4} ^", a32::ldm_user_registers(AddressMode::IncrementBefore, R3, [R4]));
}

#[test]
fn test_ldmda() {
    test3_cond("ldmda", a32::ldmda::cond::<Mem, RegisterList>);
    test3_cond("ldmda", a32::ldmda::cond::<Register, &[Register]>);
}

#[test]
fn test_ldmdb() {
    test3_cond("ldmdb", a32::ldmdb::cond::<Mem, RegisterList>);
    test3_cond("ldmdb", a32::ldmdb::cond::<Register, &[Register]>);
}

#[test]
fn test_ldmib() {
    test3_cond("ldmib", a32::ldmib::cond::<Mem, RegisterList>);
    test3_cond("ldmib", a32::ldmib::cond::<Register, &[Register]>);
}

#[test]
fn test_ldr_imm_offset() {
    test_ldst_offset_filter("ldr", a32::ldr_imm_offset, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_offset_filter_cond("ldr", a32::ldr_imm_offset::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldr_imm_post() {
    test_ldst_post_filter("ldr", a32::ldr_imm_post, |_, rn, offset| {
        (rn, offset.abs_offset()) != (R13, 4) && offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("ldr", a32::ldr_imm_post::cond, |_, _, rn, offset| {
        (rn, offset.abs_offset()) != (R13, 4) && offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldr_imm_pre() {
    test_ldst_pre_filter("ldr", a32::ldr_imm_pre, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_pre_filter_cond("ldr", a32::ldr_imm_pre::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldr_lit() {
    test_disasm("ldr", "r3, [pc, #-5]", a32::ldr_lit(R3, Offset::Sub(5)));
    test_disasm("ldr", "r3, [pc, #5]", a32::ldr_lit(R3, Offset::Add(5)));
}

#[test]
fn test_ldr_reg_offset() {
    test_ldst_offset("ldr", a32::ldr_reg_offset::<Register>);
    test_ldst_offset("ldr", a32::ldr_reg_offset::<Index>);
    test_ldst_offset_cond("ldr", a32::ldr_reg_offset::cond::<Register>);
    test_ldst_offset_cond("ldr", a32::ldr_reg_offset::cond::<Index>);
}

#[test]
fn test_ldr_reg_post() {
    test_ldst_post("ldr", a32::ldr_reg_post::<Register>);
    test_ldst_post("ldr", a32::ldr_reg_post::<Index>);
    test_ldst_post_cond("ldr", a32::ldr_reg_post::cond::<Register>);
    test_ldst_post_cond("ldr", a32::ldr_reg_post::cond::<Index>);
}

#[test]
fn test_ldr_reg_pre() {
    test_ldst_pre("ldr", a32::ldr_reg_pre::<Register>);
    test_ldst_pre("ldr", a32::ldr_reg_pre::<Index>);
    test_ldst_pre_cond("ldr", a32::ldr_reg_pre::cond::<Register>);
    test_ldst_pre_cond("ldr", a32::ldr_reg_pre::cond::<Index>);
}

#[test]
fn test_ldrb_imm_offset() {
    test_ldst_offset_filter("ldrb", a32::ldrb_imm_offset, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_offset_filter_cond("ldrb", a32::ldrb_imm_offset::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldrb_imm_post() {
    test_ldst_post_filter("ldrb", a32::ldrb_imm_post, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("ldrb", a32::ldrb_imm_post::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldrb_imm_pre() {
    test_ldst_pre_filter("ldrb", a32::ldrb_imm_pre, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_pre_filter_cond("ldrb", a32::ldrb_imm_pre::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldrb_lit() {
    test_disasm("ldrb", "r3, [pc, #-5]", a32::ldrb_lit(R3, Offset::Sub(5)));
    test_disasm("ldrb", "r3, [pc, #5]", a32::ldrb_lit(R3, Offset::Add(5)));
}

#[test]
fn test_ldrb_reg_offset() {
    test_ldst_offset("ldrb", a32::ldrb_reg_offset::<Register>);
    test_ldst_offset("ldrb", a32::ldrb_reg_offset::<Index>);
    test_ldst_offset_cond("ldrb", a32::ldrb_reg_offset::cond::<Register>);
    test_ldst_offset_cond("ldrb", a32::ldrb_reg_offset::cond::<Index>);
}

#[test]
fn test_ldrb_reg_post() {
    test_ldst_post("ldrb", a32::ldrb_reg_post::<Register>);
    test_ldst_post("ldrb", a32::ldrb_reg_post::<Index>);
    test_ldst_post_cond("ldrb", a32::ldrb_reg_post::cond::<Register>);
    test_ldst_post_cond("ldrb", a32::ldrb_reg_post::cond::<Index>);
}

#[test]
fn test_ldrb_reg_pre() {
    test_ldst_pre("ldrb", a32::ldrb_reg_pre::<Register>);
    test_ldst_pre("ldrb", a32::ldrb_reg_pre::<Index>);
    test_ldst_pre_cond("ldrb", a32::ldrb_reg_pre::cond::<Register>);
    test_ldst_pre_cond("ldrb", a32::ldrb_reg_pre::cond::<Index>);
}

#[test]
fn test_ldrbt_imm() {
    test_ldst_post_filter("ldrbt", a32::ldrbt_imm, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("ldrbt", a32::ldrbt_imm::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldrbt_reg() {
    test_ldst_post("ldrbt", a32::ldrbt_reg::<Register>);
    test_ldst_post("ldrbt", a32::ldrbt_reg::<Index>);
    test_ldst_post_cond("ldrbt", a32::ldrbt_reg::cond::<Register>);
    test_ldst_post_cond("ldrbt", a32::ldrbt_reg::cond::<Index>);
}

#[test]
fn test_ldrd_imm_offset() {
    test_ldst_offset("ldrd", a32::ldrd_imm_offset);
    test_ldst_offset_cond("ldrd", a32::ldrd_imm_offset::cond);
}

#[test]
fn test_ldrd_imm_post() {
    test_ldst_post("ldrd", a32::ldrd_imm_post);
    test_ldst_post_cond("ldrd", a32::ldrd_imm_post::cond);
}

#[test]
fn test_ldrd_imm_pre() {
    test_ldst_pre("ldrd", a32::ldrd_imm_pre);
    test_ldst_pre_cond("ldrd", a32::ldrd_imm_pre::cond);
}

#[test]
fn test_ldrd_lit() {
    test3_format_cond("ldrd", a32::ldrd_lit::cond,
        |rt, offset| format!("{}, [pc, {}]", rt, offset));
}

#[test]
fn test_ldrd_reg_offset() {
    test_ldst_offset("ldrd", a32::ldrd_reg_offset);
    test_ldst_offset_cond("ldrd", a32::ldrd_reg_offset::cond);
}

#[test]
fn test_ldrd_reg_post() {
    test_ldst_post("ldrd", a32::ldrd_reg_post);
    test_ldst_post_cond("ldrd", a32::ldrd_reg_post::cond);
}

#[test]
fn test_ldrd_reg_pre() {
    test_ldst_pre("ldrd", a32::ldrd_reg_pre);
    test_ldst_pre_cond("ldrd", a32::ldrd_reg_pre::cond);
}

#[test]
fn test_ldrex() {
    test_ldst("ldrex", a32::ldrex);
    test_ldst_cond("ldrex", a32::ldrex::cond);
}

#[test]
fn test_ldrexb() {
    test_ldst("ldrexb", a32::ldrexb);
    test_ldst_cond("ldrexb", a32::ldrexb::cond);
}

#[test]
fn test_ldrexd() {
    test_ldst("ldrexd", a32::ldrexd);
    test_ldst_cond("ldrexd", a32::ldrexd::cond);
}

#[test]
fn test_ldrexh() {
    test_ldst("ldrexh", a32::ldrexh);
    test_ldst_cond("ldrexh", a32::ldrexh::cond);
}

#[test]
fn test_ldrh_imm_offset() {
    test_ldst_offset("ldrh", a32::ldrh_imm_offset);
    test_ldst_offset_cond("ldrh", a32::ldrh_imm_offset::cond);
}

#[test]
fn test_ldrh_imm_post() {
    test_ldst_post("ldrh", a32::ldrh_imm_post);
    test_ldst_post_cond("ldrh", a32::ldrh_imm_post::cond);
}

#[test]
fn test_ldrh_imm_pre() {
    test_ldst_pre("ldrh", a32::ldrh_imm_pre);
    test_ldst_pre_cond("ldrh", a32::ldrh_imm_pre::cond);
}

#[test]
fn test_ldrh_lit() {
    test_disasm("ldrh", "r3, [pc, #-5]", a32::ldrh_lit(R3, Offset::Sub(5)));
    test_disasm("ldrh", "r3, [pc, #5]", a32::ldrh_lit(R3, Offset::Add(5)));
}

#[test]
fn test_ldrh_reg_offset() {
    test_ldst_offset("ldrh", a32::ldrh_reg_offset);
    test_ldst_offset_cond("ldrh", a32::ldrh_reg_offset::cond);
}

#[test]
fn test_ldrh_reg_post() {
    test_ldst_post("ldrh", a32::ldrh_reg_post);
    test_ldst_post_cond("ldrh", a32::ldrh_reg_post::cond);
}

#[test]
fn test_ldrh_reg_pre() {
    test_ldst_pre("ldrh", a32::ldrh_reg_pre);
    test_ldst_pre_cond("ldrh", a32::ldrh_reg_pre::cond);
}

#[test]
fn test_ldrht_imm() {
    test_ldst_post("ldrht", a32::ldrht_imm);
    test_ldst_post_cond("ldrht", a32::ldrht_imm::cond);
}

#[test]
fn test_ldrht_reg() {
    test_ldst_post("ldrht", a32::ldrht_reg);
    test_ldst_post_cond("ldrht", a32::ldrht_reg::cond);
}

#[test]
fn test_ldrsb_imm_offset() {
    test_ldst_offset("ldrsb", a32::ldrsb_imm_offset);
    test_ldst_offset_cond("ldrsb", a32::ldrsb_imm_offset::cond);
}

#[test]
fn test_ldrsb_imm_post() {
    test_ldst_post("ldrsb", a32::ldrsb_imm_post);
    test_ldst_post_cond("ldrsb", a32::ldrsb_imm_post::cond);
}

#[test]
fn test_ldrsb_imm_pre() {
    test_ldst_pre("ldrsb", a32::ldrsb_imm_pre);
    test_ldst_pre_cond("ldrsb", a32::ldrsb_imm_pre::cond);
}

#[test]
fn test_ldrsb_lit() {
    test_disasm("ldrsb", "r3, [pc, #-5]", a32::ldrsb_lit(R3, Offset::Sub(5)));
    test_disasm("ldrsb", "r3, [pc, #5]", a32::ldrsb_lit(R3, Offset::Add(5)));
}

#[test]
fn test_ldrsb_reg_offset() {
    test_ldst_offset("ldrsb", a32::ldrsb_reg_offset);
    test_ldst_offset_cond("ldrsb", a32::ldrsb_reg_offset::cond);
}

#[test]
fn test_ldrsb_reg_post() {
    test_ldst_post("ldrsb", a32::ldrsb_reg_post);
    test_ldst_post_cond("ldrsb", a32::ldrsb_reg_post::cond);
}

#[test]
fn test_ldrsb_reg_pre() {
    test_ldst_pre("ldrsb", a32::ldrsb_reg_pre);
    test_ldst_pre_cond("ldrsb", a32::ldrsb_reg_pre::cond);
}

#[test]
fn test_ldrsbt_imm() {
    test_ldst_post("ldrsbt", a32::ldrsbt_imm);
    test_ldst_post_cond("ldrsbt", a32::ldrsbt_imm::cond);
}

#[test]
fn test_ldrsbt_reg() {
    test_ldst_post("ldrsbt", a32::ldrsbt_reg);
    test_ldst_post_cond("ldrsbt", a32::ldrsbt_reg::cond);
}

#[test]
fn test_ldrsh_imm_offset() {
    test_ldst_offset("ldrsh", a32::ldrsh_imm_offset);
    test_ldst_offset_cond("ldrsh", a32::ldrsh_imm_offset::cond);
}

#[test]
fn test_ldrsh_imm_post() {
    test_ldst_post("ldrsh", a32::ldrsh_imm_post);
    test_ldst_post_cond("ldrsh", a32::ldrsh_imm_post::cond);
}

#[test]
fn test_ldrsh_imm_pre() {
    test_ldst_pre("ldrsh", a32::ldrsh_imm_pre);
    test_ldst_pre_cond("ldrsh", a32::ldrsh_imm_pre::cond);
}

#[test]
fn test_ldrsh_lit() {
    test_disasm("ldrsh", "r3, [pc, #-5]", a32::ldrsh_lit(R3, Offset::Sub(5)));
    test_disasm("ldrsh", "r3, [pc, #5]", a32::ldrsh_lit(R3, Offset::Add(5)));
}

#[test]
fn test_ldrsh_reg_offset() {
    test_ldst_offset("ldrsh", a32::ldrsh_reg_offset);
    test_ldst_offset_cond("ldrsh", a32::ldrsh_reg_offset::cond);
}

#[test]
fn test_ldrsh_reg_post() {
    test_ldst_post("ldrsh", a32::ldrsh_reg_post);
    test_ldst_post_cond("ldrsh", a32::ldrsh_reg_post::cond);
}

#[test]
fn test_ldrsh_reg_pre() {
    test_ldst_pre("ldrsh", a32::ldrsh_reg_pre);
    test_ldst_pre_cond("ldrsh", a32::ldrsh_reg_pre::cond);
}

#[test]
fn test_ldrsht_imm() {
    test_ldst_post("ldrsht", a32::ldrsht_imm);
    test_ldst_post_cond("ldrsht", a32::ldrsht_imm::cond);
}

#[test]
fn test_ldrsht_reg() {
    test_ldst_post("ldrsht", a32::ldrsht_reg);
    test_ldst_post_cond("ldrsht", a32::ldrsht_reg::cond);
}

#[test]
fn test_ldrt_imm() {
    test_ldst_post_filter("ldrt", a32::ldrt_imm, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("ldrt", a32::ldrt_imm::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_ldrt_reg() {
    test_ldst_post("ldrt", a32::ldrt_reg::<Register>);
    test_ldst_post("ldrt", a32::ldrt_reg::<Index>);
    test_ldst_post_cond("ldrt", a32::ldrt_reg::cond::<Register>);
    test_ldst_post_cond("ldrt", a32::ldrt_reg::cond::<Index>);
}

#[test]
fn test_lsl_imm() {
    test4_filter_cond("lsl", a32::lsl_imm::cond,
        |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_lsl_reg() {
    test4_cond("lsl", a32::lsl_reg::cond);
}

#[test]
fn test_lsls_imm() {
    test4_filter_cond("lsls", a32::lsls_imm::cond,
        |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_lsls_reg() {
    test4_cond("lsls", a32::lsls_reg::cond);
}

#[test]
fn test_lsr_imm() {
    test4_filter_cond("lsr", a32::lsr_imm::cond,
        |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_lsr_reg() {
    test4_cond("lsr", a32::lsr_reg::cond);
}

#[test]
fn test_lsrs_imm() {
    test4_filter_cond("lsrs", a32::lsrs_imm::cond,
        |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_lsrs_reg() {
    test4_cond("lsrs", a32::lsrs_reg::cond);
}

#[test]
fn test_mcr() {
    test_disasm("mcr", "p14, #3, r4, c5, c6, #7", a32::mcr(14, 3, R4, 5, 6, 7));
}

#[test]
fn test_mcrr() {
    test_disasm("mcrr", "p14, #3, r4, r5, c6", a32::mcrr(14, 3, R4, R5, 6));
}

#[test]
fn test_mla() {
    test5_filter_cond("mla", a32::mla::cond, |_, a2, a3, a4, a5| {
        a2 != R15 && a3 != R15 && a4 != R15 && a5 != R15
    });
}

#[test]
fn test_mlas() {
    test5_filter_cond("mlas", a32::mlas::cond, |_, a2, a3, a4, a5| {
        a2 != R15 && a3 != R15 && a4 != R15 && a5 != R15
    });
}

#[test]
fn test_mls() {
    test5_filter_cond("mls", a32::mls::cond, |_, a2, a3, a4, a5| {
        a2 != R15 && a3 != R15 && a4 != R15 && a5 != R15
    });
}

#[test]
fn test_mov_imm() {
    test3_cond("mov", a32::mov_imm::cond::<u8>);
    test3_cond("mov", a32::mov_imm::cond::<Const>);
}

#[test]
fn test_mov_reg() {
    test3_cond("mov", a32::mov_reg::cond);
}

#[test]
fn test_movs_imm() {
    test3_cond("movs", a32::movs_imm::cond::<u8>);
    test3_cond("movs", a32::movs_imm::cond::<Const>);
}

#[test]
fn test_movs_reg() {
    test3_cond("movs", a32::movs_reg::cond);
}

#[test]
fn test_movt() {
    test3_filter_cond("movt", a32::movt::cond, |_, rd, _| rd != R15);
}

#[test]
fn test_movw() {
    test3_filter_cond("movw", a32::movw::cond, |_, rd, _| rd != R15);
}

#[test]
fn test_mrc() {
    test_disasm("mrc", "p14, #3, r4, c5, c6, #7", a32::mrc(14, 3, R4, 5, 6, 7));
}

#[test]
fn test_mrrc() {
    test_disasm("mrrc", "p14, #3, r4, r5, c6", a32::mrrc(14, 3, R4, R5, 6));
}

#[test]
fn test_mrs() {
    test_disasm("mrs", "r3, apsr", a32::mrs(R3, 0));
    test_disasm("mrs", "r3, spsr", a32::mrs(R3, 1));
}

#[test]
fn test_mrs_banked() {
    test3_cond("mrs", a32::mrs_banked::cond);
}

#[test]
fn test_msr_banked() {
    test3_cond("msr", a32::msr_banked::cond);
}

#[test]
fn test_msr_imm() {
    test_disasm("sevl", "", a32::msr_imm(0, 0, 5));
    test_disasm("msr", "cpsr_c, #5", a32::msr_imm(0, 1, 5));
    test_disasm("msr", "cpsr_x, #5", a32::msr_imm(0, 2, 5));
    test_disasm("msr", "cpsr_sc, #5", a32::msr_imm(0, 5, 5));
    test_disasm("msr", "cpsr_fsxc, #5", a32::msr_imm(0, 15, 5));
    test_disasm("msr", "spsr_c, #5", a32::msr_imm(1, 1, 5));
    test_disasm("msr", "spsr_x, #5", a32::msr_imm(1, 2, 5));
    test_disasm("msr", "spsr_sc, #5", a32::msr_imm(1, 5, 5));
    test_disasm("msr", "spsr_fsxc, #5", a32::msr_imm(1, 15, 5));
}

#[test]
fn test_msr_reg() {
    test_disasm("msr", "cpsr_c, r5", a32::msr_reg(0, 1, R5));
    test_disasm("msr", "cpsr_x, r5", a32::msr_reg(0, 2, R5));
    test_disasm("msr", "cpsr_sc, r5", a32::msr_reg(0, 5, R5));
    test_disasm("msr", "cpsr_fsxc, r5", a32::msr_reg(0, 15, R5));
    test_disasm("msr", "spsr_c, r5", a32::msr_reg(1, 1, R5));
    test_disasm("msr", "spsr_x, r5", a32::msr_reg(1, 2, R5));
    test_disasm("msr", "spsr_sc, r5", a32::msr_reg(1, 5, R5));
    test_disasm("msr", "spsr_fsxc, r5", a32::msr_reg(1, 15, R5));
}

#[test]
fn test_mul() {
    test4_filter_cond("mul", a32::mul::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_muls() {
    test4_filter_cond("muls", a32::muls::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_mvn() {
    test3_cond("mvn", a32::mvn_imm::cond::<u8>);
    test3_cond("mvn", a32::mvn_imm::cond::<Const>);
}

#[test]
fn test_mvn_reg() {
    test4_cond("mvn", a32::mvn_reg::cond);
}

#[test]
fn test_mvn_reg_rrx() {
    test3_format_cond("mvn", a32::mvn_reg_rrx::cond,
        |rd, rm| format!("{}, {}, rrx", rd, rm));
}

#[test]
fn test_mvn_reg_shifted_reg() {
    test4_cond("mvn", a32::mvn_reg_shifted_reg::cond);
}

#[test]
fn test_mvns() {
    test3_cond("mvns", a32::mvns_imm::cond::<u8>);
    test3_cond("mvns", a32::mvns_imm::cond::<Const>);
}

#[test]
fn test_mvns_reg() {
    test4_cond("mvns", a32::mvns_reg::cond);
}

#[test]
fn test_mvns_reg_rrx() {
    test3_format_cond("mvns", a32::mvns_reg_rrx::cond,
        |rd, rm| format!("{}, {}, rrx", rd, rm));
}

#[test]
fn test_mvns_reg_shifted_reg() {
    test4_cond("mvns", a32::mvns_reg_shifted_reg::cond);
}

#[test]
fn test_nop() {
    test1_cond("nop", a32::nop::cond);
}

#[test]
fn test_orr_imm() {
    test4_cond("orr", a32::orr_imm::cond::<u8>);
    test4_cond("orr", a32::orr_imm::cond::<Const>);
}

#[test]
fn test_orr_reg() {
    test5_cond("orr", a32::orr_reg::cond);
}

#[test]
fn test_orr_reg_rrx() {
    test4_format_cond("orr", a32::orr_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_orr_reg_shifted_reg() {
    test5_cond("orr", a32::orr_reg_shifted_reg::cond);
}

#[test]
fn test_orrs_imm() {
    test4_cond("orrs", a32::orrs_imm::cond::<u8>);
    test4_cond("orrs", a32::orrs_imm::cond::<Const>);
}

#[test]
fn test_orrs_reg() {
    test5_cond("orrs", a32::orrs_reg::cond);
}

#[test]
fn test_orrs_reg_rrx() {
    test4_format_cond("orrs", a32::orrs_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_orrs_reg_shifted_reg() {
    test5_cond("orrs", a32::orrs_reg_shifted_reg::cond);
}

#[test]
fn test_pkhbt() {
    test5_format_filter_cond("pkhbt", a32::pkhbt::cond,
        |rd, rn, rm, imm5| format!("{}, {}, {}, lsl {}", rd, rn, rm, imm5),
        |_, _, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_pkhtb() {
    test5_format_filter_cond("pkhtb", a32::pkhtb::cond,
        |rd, rn, rm, imm5| format!("{}, {}, {}, asr {}", rd, rn, rm, imm5),
        |_, _, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_pld_imm() {
    test2_format_filter(|_, _| "pld".into(), a32::pld_imm,
        |rn, offset| format!("[{}, {}]", rn, offset),
        |_, offset| offset.abs_offset() < 0x1000);
}

#[test]
fn test_pld_lit() {
    test_disasm("pld", "[pc, #-5]", a32::pld_lit(Offset::Sub(5)));
    test_disasm("pld", "[pc, #5]", a32::pld_lit(Offset::Add(5)));
}

#[test]
fn test_pld_reg() {
    test2_format("pld", a32::pld_reg::<Register>,
        |rn, rm| format!("[{}, {}]", rn, rm));
    test2_format("pld", a32::pld_reg::<Index>,
        |rn, index| format!("[{}, {}]", rn, index));
}

#[test]
fn test_pld_reg_rrx() {
    test_disasm("pld", "[r4, -r5, rrx]", a32::pld_reg_rrx(R4, 0, R5));
    test_disasm("pld", "[r4, r5, rrx]", a32::pld_reg_rrx(R4, 1, R5));
}

#[test]
fn test_pldw_imm() {
    test2_format_filter(|_, _| "pldw".into(), a32::pldw_imm,
        |rn, offset| format!("[{}, {}]", rn, offset),
        |_, offset| offset.abs_offset() < 0x1000);
}

#[test]
fn test_pldw_reg() {
    test2_format("pldw", a32::pldw_reg::<Register>,
        |rn, rm| format!("[{}, {}]", rn, rm));
    test2_format("pldw", a32::pldw_reg::<Index>,
        |rn, index| format!("[{}, {}]", rn, index));
}

#[test]
fn test_pldw_reg_rrx() {
    test_disasm("pldw", "[r4, -r5, rrx]", a32::pldw_reg_rrx(R4, 0, R5));
    test_disasm("pldw", "[r4, r5, rrx]", a32::pldw_reg_rrx(R4, 1, R5));
}

#[test]
fn test_pli_imm() {
    test2_format_filter(|_, _| "pli".into(), a32::pli_imm,
        |rn, offset| format!("[{}, {}]", rn, offset),
        |_, offset| offset.abs_offset() < 0x1000);
}

#[test]
fn test_pli_reg() {
    test2_format("pli", a32::pli_reg::<Register>,
        |rn, rm| format!("[{}, {}]", rn, rm));
    test2_format("pli", a32::pli_reg::<Index>,
        |rn, index| format!("[{}, {}]", rn, index));
}

#[test]
fn test_pli_reg_rrx() {
    test_disasm("pli", "[r4, -r5, rrx]", a32::pli_reg_rrx(R4, 0, R5));
    test_disasm("pli", "[r4, r5, rrx]", a32::pli_reg_rrx(R4, 1, R5));
}

#[test]
fn test_pop_list() {
    test2_filter_cond("pop", a32::pop_list::cond::<RegisterList>, |_, regs| regs.len() > 1);
    test2_filter_cond("pop", a32::pop_list::cond::<&[Register]>, |_, regs| regs.len() > 1);
}

#[test]
fn test_pop_reg() {
    test2_format_cond("pop", a32::pop_reg::cond,
        |rt| format!("{{{}}}", rt));
}

#[test]
fn test_pssbb() {
    test_disasm("dsb", "#4", armv8::pssbb());
}

#[test]
fn test_push_list() {
    test2_filter_cond("push", a32::push_list::cond::<RegisterList>, |_, regs| regs.len() > 1);
    test2_filter_cond("push", a32::push_list::cond::<&[Register]>, |_, regs| regs.len() > 1);
}

#[test]
fn test_push_reg() {
    test2_format_cond("str", a32::push_reg::cond,
        |rt| format!("{}, [sp, #-4]!", rt));
}

#[test]
fn test_qadd() {
    test4_cond("qadd", a32::qadd::cond);
}

#[test]
fn test_qadd8() {
    test4_cond("qadd8", a32::qadd8::cond);
}

#[test]
fn test_qadd16() {
    test4_cond("qadd16", a32::qadd16::cond);
}

#[test]
fn test_qasx() {
    test4_cond("qasx", a32::qasx::cond);
}

#[test]
fn test_qdadd() {
    test4_cond("qdadd", a32::qdadd::cond);
}

#[test]
fn test_qdsub() {
    test4_cond("qdsub", a32::qdsub::cond);
}

#[test]
fn test_qsax() {
    test4_cond("qsax", a32::qsax::cond);
}

#[test]
fn test_qsub() {
    test4_cond("qsub", a32::qsub::cond);
}

#[test]
fn test_qsub8() {
    test4_cond("qsub8", a32::qsub8::cond);
}

#[test]
fn test_qsub16() {
    test4_cond("qsub16", a32::qsub16::cond);
}

#[test]
fn test_rbit() {
    test3_cond("rbit", a32::rbit::cond);
}

#[test]
fn test_rev() {
    test3_cond("rev", a32::rev::cond);
}

#[test]
fn test_rev16() {
    test3_cond("rev16", a32::rev16::cond);
}

#[test]
fn test_revsh() {
    test3_cond("revsh", a32::revsh::cond);
}

#[test]
fn test_rfeda() {
    test1("rfeda", a32::rfeda);
}

#[test]
fn test_rfedb() {
    test1("rfedb", a32::rfedb);
}

#[test]
fn test_rfeia() {
    test1("rfeia", a32::rfeia);
}

#[test]
fn test_rfeib() {
    test1("rfeib", a32::rfeib);
}

#[test]
fn test_ror_imm() {
    test4_filter_cond("ror", a32::ror_imm::cond,
        |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_ror_reg() {
    test4_cond("ror", a32::ror_reg::cond);
}

#[test]
fn test_rors_imm() {
    test4_filter_cond("rors", a32::rors_imm::cond,
        |_, _, _, imm5| imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_rors_reg() {
    test4_cond("rors", a32::rors_reg::cond);
}

#[test]
fn test_rrx() {
    test3_cond("rrx", a32::rrx::cond);
}

#[test]
fn test_rrxs() {
    test3_cond("rrxs", a32::rrxs::cond);
}

#[test]
fn test_rsb_imm() {
    test4_cond("rsb", a32::rsb_imm::cond::<u8>);
    test4_cond("rsb", a32::rsb_imm::cond::<Const>);
}

#[test]
fn test_rsb_reg() {
    test5_cond("rsb", a32::rsb_reg::cond);
}

#[test]
fn test_rsb_reg_rrx() {
    test4_format_cond("rsb", a32::rsb_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_rsb_reg_shifted_reg() {
    test5_cond("rsb", a32::rsb_reg_shifted_reg::cond);
}

#[test]
fn test_rsbs_imm() {
    test4_cond("rsbs", a32::rsbs_imm::cond::<u8>);
    test4_cond("rsbs", a32::rsbs_imm::cond::<Const>);
}

#[test]
fn test_rsbs_reg() {
    test5_cond("rsbs", a32::rsbs_reg::cond);
}

#[test]
fn test_rsbs_reg_rrx() {
    test4_format_cond("rsbs", a32::rsbs_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_rsbs_reg_shifted_reg() {
    test5_cond("rsbs", a32::rsbs_reg_shifted_reg::cond);
}

#[test]
fn test_rsc_imm() {
    test4_cond("rsc", a32::rsc_imm::cond::<u8>);
    test4_cond("rsc", a32::rsc_imm::cond::<Const>);
}

#[test]
fn test_rsc_reg() {
    test5_cond("rsc", a32::rsc_reg::cond);
}

#[test]
fn test_rsc_reg_rrx() {
    test4_format_cond("rsc", a32::rsc_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_rsc_reg_shifted_reg() {
    test5_cond("rsc", a32::rsc_reg_shifted_reg::cond);
}

#[test]
fn test_rscs_imm() {
    test4_cond("rscs", a32::rscs_imm::cond::<u8>);
    test4_cond("rscs", a32::rscs_imm::cond::<Const>);
}

#[test]
fn test_rscs_reg() {
    test5_cond("rscs", a32::rscs_reg::cond);
}

#[test]
fn test_rscs_reg_rrx() {
    test4_format_cond("rscs", a32::rscs_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_rscs_reg_shifted_reg() {
    test5_cond("rscs", a32::rscs_reg_shifted_reg::cond);
}

#[test]
fn test_sadd8() {
    test4_cond("sadd8", a32::sadd8::cond);
}

#[test]
fn test_sadd16() {
    test4_cond("sadd16", a32::sadd16::cond);
}

#[test]
fn test_sasx() {
    test4_cond("sasx", a32::sasx::cond);
}

#[test]
#[ignore]
fn test_sb() {
    test_disasm("sb", "", sb::sb());
}

#[test]
fn test_sbc_imm() {
    test4_cond("sbc", a32::sbc_imm::cond::<u8>);
    test4_cond("sbc", a32::sbc_imm::cond::<Const>);
}

#[test]
fn test_sbc_reg() {
    test5_cond("sbc", a32::sbc_reg::cond);
}

#[test]
fn test_sbc_reg_rrx() {
    test4_format_cond("sbc", a32::sbc_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_sbc_reg_shifted_reg() {
    test5_cond("sbc", a32::sbc_reg_shifted_reg::cond);
}

#[test]
fn test_sbcs_imm() {
    test4_cond("sbcs", a32::sbcs_imm::cond::<u8>);
    test4_cond("sbcs", a32::sbcs_imm::cond::<Const>);
}

#[test]
fn test_sbcs_reg() {
    test5_cond("sbcs", a32::sbcs_reg::cond);
}

#[test]
fn test_sbcs_reg_rrx() {
    test4_format_cond("sbcs", a32::sbcs_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_sbcs_reg_shifted_reg() {
    test5_cond("sbcs", a32::sbcs_reg_shifted_reg::cond);
}

#[test]
fn test_sbfx() {
    test5_filter_cond("sbfx", a32::sbfx::cond, |_, rd, rn, lsb, width| {
        rd != R15 && rn != R15 && lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_sdiv() {
    test4_filter_cond("sdiv", a32::sdiv::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_sel() {
    test4_filter_cond("sel", a32::sel::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_setend() {
    test_disasm("setend", "le", a32::setend(Endianness::Little));
    test_disasm("setend", "be", a32::setend(Endianness::Big));
}

#[test]
fn test_setpan() {
    test_disasm("setpan", "#0", pan::setpan(false));
    test_disasm("setpan", "#1", pan::setpan(true));
}

#[test]
fn test_sev() {
    test_disasm("sev", "", a32::sev());
    test1_cond("sev", a32::sev::cond);
}

#[test]
fn test_sevl() {
    test_disasm("sevl", "", armv8::sevl());
    test1_cond("sevl", armv8::sevl::cond);
}

#[test]
fn test_shadd8() {
    test4_filter_cond("shadd8", a32::shadd8::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_shadd16() {
    test4_filter_cond("shadd16", a32::shadd16::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_shasx() {
    test4_filter_cond("shasx", a32::shasx::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_shsax() {
    test4_filter_cond("shsax", a32::shsax::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_shsub8() {
    test4_filter_cond("shsub8", a32::shsub8::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_shsub16() {
    test4_filter_cond("shsub16", a32::shsub16::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smc() {
    test2_filter_cond("smc", a32::smc::cond, |_, imm4| imm4 < 0x10);
}

#[test]
fn test_smlabb() {
    test5_filter_cond("smlabb", a32::smlabb::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlabt() {
    test5_filter_cond("smlabt", a32::smlabt::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlad() {
    test5_filter_cond("smlad", a32::smlad::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smladx() {
    test5_filter_cond("smladx", a32::smladx::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlal() {
    test5_filter_cond("smlal", a32::smlal::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlalbb() {
    test5_filter_cond("smlalbb", a32::smlalbb::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlalbt() {
    test5_filter_cond("smlalbt", a32::smlalbt::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlald() {
    test5_filter_cond("smlald", a32::smlald::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlaldx() {
    test5_filter_cond("smlaldx", a32::smlaldx::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlals() {
    test5_filter_cond("smlals", a32::smlals::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlaltb() {
    test5_filter_cond("smlaltb", a32::smlaltb::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlaltt() {
    test5_filter_cond("smlaltt", a32::smlaltt::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlatb() {
    test5_filter_cond("smlatb", a32::smlatb::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlatt() {
    test5_filter_cond("smlatt", a32::smlatt::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlawb() {
    test5_filter_cond("smlawb", a32::smlawb::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlawt() {
    test5_filter_cond("smlawt", a32::smlawt::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlsd() {
    test5_filter_cond("smlsd", a32::smlsd::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlsdx() {
    test5_filter_cond("smlsdx", a32::smlsdx::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smlsld() {
    test5_filter_cond("smlsld", a32::smlsld::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smlsldx() {
    test5_filter_cond("smlsldx", a32::smlsldx::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smmla() {
    test5_filter_cond("smmla", a32::smmla::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smmlar() {
    test5_filter_cond("smmlar", a32::smmlar::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smmls() {
    test5_filter_cond("smmls", a32::smmls::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smmlsr() {
    test5_filter_cond("smmlsr", a32::smmlsr::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_smmul() {
    test4_filter_cond("smmul", a32::smmul::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smmulr() {
    test4_filter_cond("smmulr", a32::smmulr::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smuad() {
    test4_filter_cond("smuad", a32::smuad::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smuadx() {
    test4_filter_cond("smuadx", a32::smuadx::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smulbb() {
    test4_filter_cond("smulbb", a32::smulbb::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smulbt() {
    test4_filter_cond("smulbt", a32::smulbt::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smull() {
    test5_filter_cond("smull", a32::smull::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smulls() {
    test5_filter_cond("smulls", a32::smulls::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smultb() {
    test4_filter_cond("smultb", a32::smultb::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smultt() {
    test4_filter_cond("smultt", a32::smultt::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smulwb() {
    test4_filter_cond("smulwb", a32::smulwb::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smulwt() {
    test4_filter_cond("smulwt", a32::smulwt::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smusd() {
    test4_filter_cond("smusd", a32::smusd::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_smusdx() {
    test4_filter_cond("smusdx", a32::smusdx::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_srsda() {
    test_disasm("srsda", "sp, #3", a32::srsda(false, 3));
    test_disasm("srsda", "sp!, #3", a32::srsda(true, 3));
}

#[test]
fn test_srsdb() {
    test_disasm("srsdb", "sp, #3", a32::srsdb(false, 3));
    test_disasm("srsdb", "sp!, #3", a32::srsdb(true, 3));
}

#[test]
fn test_srsia() {
    test_disasm("srsia", "sp, #3", a32::srsia(false, 3));
    test_disasm("srsia", "sp!, #3", a32::srsia(true, 3));
}

#[test]
fn test_srsib() {
    test_disasm("srsib", "sp, #3", a32::srsib(false, 3));
    test_disasm("srsib", "sp!, #3", a32::srsib(true, 3));
}

#[test]
fn test_ssat_asr() {
    test5_format_filter_cond("ssat", a32::ssat_asr::cond,
        |rd, sat_imm, rn, imm5| format!("{}, {}, {}, asr {}", rd, sat_imm, rn, imm5),
        |_, _, sat_imm, _, imm5| sat_imm > 0 && sat_imm <= 0x20 && imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_ssat_lsl() {
    test5_format_filter_cond("ssat", a32::ssat_lsl::cond,
        |rd, sat_imm, rn, imm5| format!("{}, {}, {}, lsl {}", rd, sat_imm, rn, imm5),
        |_, _, sat_imm, _, imm5| sat_imm > 0 && sat_imm <= 0x20 && imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_ssat16() {
    test4_filter_cond("ssat16", a32::ssat16::cond, |_, rd, sat_imm, rn| {
        rd != R15 && rn != R15 && sat_imm > 0 && sat_imm <= 16
    });
}

#[test]
fn test_ssax() {
    test4_filter_cond("ssax", a32::ssax::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_ssbb() {
    test_disasm("dsb", "#0", armv8::ssbb());
}

#[test]
fn test_ssub8() {
    test4_cond("ssub8", a32::ssub8::cond);
}

#[test]
fn test_ssub16() {
    test4_cond("ssub16", a32::ssub16::cond);
}

#[test]
fn test_stc_imm_offset() {
    test_disasm("stc", "p14, c5, [r4, #-0x14]", a32::stc_imm_offset(R4, Offset::Sub(5)));
    test_disasm("stc", "p14, c5, [r4, #0x14]", a32::stc_imm_offset(R4, Offset::Add(5)));
}

#[test]
fn test_stc_imm_post() {
    test_disasm("stc", "p14, c5, [r4], #-0x14", a32::stc_imm_post(R4, Offset::Sub(5)));
    test_disasm("stc", "p14, c5, [r4], #0x14", a32::stc_imm_post(R4, Offset::Add(5)));
}

#[test]
fn test_stc_imm_pre() {
    test_disasm("stc", "p14, c5, [r4, #-0x14]!", a32::stc_imm_pre(R4, Offset::Sub(5)));
    test_disasm("stc", "p14, c5, [r4, #0x14]!", a32::stc_imm_pre(R4, Offset::Add(5)));
}

#[test]
fn test_stl() {
    test_ldst("stl", armv8::stl);
    test_ldst_cond("stl", armv8::stl::cond);
}

#[test]
fn test_stlb() {
    test_ldst("stlb", armv8::stlb);
    test_ldst_cond("stlb", armv8::stlb::cond);
}

#[test]
fn test_stlex() {
    test4_format_cond("stlex", armv8::stlex::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_stlexb() {
    test4_format_cond("stlexb", armv8::stlexb::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_stlexd() {
    test4_format_cond("stlexd", armv8::stlexd::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_stlexh() {
    test4_format_cond("stlexh", armv8::stlexh::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_stlh() {
    test_ldst("stlh", armv8::stlh);
    test_ldst_cond("stlh", armv8::stlh::cond);
}

#[test]
fn test_stm() {
    test3_cond("stm", a32::stm::cond::<Mem, RegisterList>);
    test3_cond("stm", a32::stm::cond::<Register, &[Register]>);
}

#[test]
fn test_stm_user_registers() {
    test3_format_cond("stm", a32::stm_user_registers::cond::<RegisterList>,
        |rn, regs| format!("{}, {} ^", rn, regs));
    test3_format_cond("stm", a32::stm_user_registers::cond::<&[Register]>,
        |rn, regs| format!("{}, {} ^", rn, regs));
}

#[test]
fn test_stmda() {
    test3_cond("stmda", a32::stmda::cond::<Mem, RegisterList>);
    test3_cond("stmda", a32::stmda::cond::<Register, &[Register]>);
}

#[test]
fn test_stmda_user_registers() {
    test3_format_cond("stmda", a32::stmda_user_registers::cond::<RegisterList>,
        |rn, regs| format!("{}, {} ^", rn, regs));
    test3_format_cond("stmda", a32::stmda_user_registers::cond::<&[Register]>,
        |rn, regs| format!("{}, {} ^", rn, regs));
}

#[test]
fn test_stmdb() {
    test3_filter_cond("stmdb", a32::stmdb::cond::<Mem, RegisterList>,
        |_, rn, _| rn != Mem::writeback(R13));
    test3_cond("stmdb", a32::stmdb::cond::<Register, &[Register]>);
}

#[test]
fn test_stmdb_user_registers() {
    test3_format_cond("stmdb", a32::stmdb_user_registers::cond::<RegisterList>,
        |rn, regs| format!("{}, {} ^", rn, regs));
    test3_format_cond("stmdb", a32::stmdb_user_registers::cond::<&[Register]>,
        |rn, regs| format!("{}, {} ^", rn, regs));
}

#[test]
fn test_stmib() {
    test3_cond("stmib", a32::stmib::cond::<Mem, RegisterList>);
    test3_cond("stmib", a32::stmib::cond::<Register, &[Register]>);
}

#[test]
fn test_stmib_user_registers() {
    test3_format_cond("stmib", a32::stmib_user_registers::cond::<RegisterList>,
        |rn, regs| format!("{}, {} ^", rn, regs));
    test3_format_cond("stmib", a32::stmib_user_registers::cond::<&[Register]>,
        |rn, regs| format!("{}, {} ^", rn, regs));
}

#[test]
fn test_str_imm_offset() {
    test_ldst_offset_filter("str", a32::str_imm_offset, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_offset_filter_cond("str", a32::str_imm_offset::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_str_imm_post() {
    test_ldst_post_filter("str", a32::str_imm_post, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("str", a32::str_imm_post::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_str_imm_pre() {
    test_ldst_pre_filter("str", a32::str_imm_pre, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_pre_filter_cond("str", a32::str_imm_pre::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_str_reg_offset() {
    test_ldst_offset("str", a32::str_reg_offset::<Register>);
    test_ldst_offset("str", a32::str_reg_offset::<Index>);
    test_ldst_offset_cond("str", a32::str_reg_offset::cond::<Register>);
    test_ldst_offset_cond("str", a32::str_reg_offset::cond::<Index>);
}

#[test]
fn test_str_reg_post() {
    test_ldst_post("str", a32::str_reg_post::<Register>);
    test_ldst_post("str", a32::str_reg_post::<Index>);
    test_ldst_post_cond("str", a32::str_reg_post::cond::<Register>);
    test_ldst_post_cond("str", a32::str_reg_post::cond::<Index>);
}

#[test]
fn test_str_reg_pre() {
    test_ldst_pre("str", a32::str_reg_pre::<Register>);
    test_ldst_pre("str", a32::str_reg_pre::<Index>);
    test_ldst_pre_cond("str", a32::str_reg_pre::cond::<Register>);
    test_ldst_pre_cond("str", a32::str_reg_pre::cond::<Index>);
}

#[test]
fn test_strb_imm_offset() {
    test_ldst_offset_filter("strb", a32::strb_imm_offset, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_offset_filter_cond("strb", a32::strb_imm_offset::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_strb_imm_post() {
    test_ldst_post_filter("strb", a32::strb_imm_post, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("strb", a32::strb_imm_post::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_strb_imm_pre() {
    test_ldst_pre_filter("strb", a32::strb_imm_pre, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_pre_filter_cond("strb", a32::strb_imm_pre::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_strb_reg_offset() {
    test_ldst_offset("strb", a32::strb_reg_offset::<Register>);
    test_ldst_offset("strb", a32::strb_reg_offset::<Index>);
    test_ldst_offset_cond("strb", a32::strb_reg_offset::cond::<Register>);
    test_ldst_offset_cond("strb", a32::strb_reg_offset::cond::<Index>);
}

#[test]
fn test_strb_reg_post() {
    test_ldst_post("strb", a32::strb_reg_post::<Register>);
    test_ldst_post("strb", a32::strb_reg_post::<Index>);
    test_ldst_post_cond("strb", a32::strb_reg_post::cond::<Register>);
    test_ldst_post_cond("strb", a32::strb_reg_post::cond::<Index>);
}

#[test]
fn test_strb_reg_pre() {
    test_ldst_pre("strb", a32::strb_reg_pre::<Register>);
    test_ldst_pre("strb", a32::strb_reg_pre::<Index>);
    test_ldst_pre_cond("strb", a32::strb_reg_pre::cond::<Register>);
    test_ldst_pre_cond("strb", a32::strb_reg_pre::cond::<Index>);
}

#[test]
fn test_strbt_imm_post() {
    test_ldst_post_filter("strbt", a32::strbt_imm_post, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("strbt", a32::strbt_imm_post::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_strbt_reg_post() {
    test_ldst_post("strbt", a32::strbt_reg_post::<Register>);
    test_ldst_post("strbt", a32::strbt_reg_post::<Index>);
    test_ldst_post_cond("strbt", a32::strbt_reg_post::cond::<Register>);
    test_ldst_post_cond("strbt", a32::strbt_reg_post::cond::<Index>);
}

#[test]
fn test_strd_imm_offset() {
    test_ldst_offset("strd", a32::strd_imm_offset);
    test_ldst_offset_cond("strd", a32::strd_imm_offset::cond);
}

#[test]
fn test_strd_imm_post() {
    test_ldst_post("strd", a32::strd_imm_post);
    test_ldst_post_cond("strd", a32::strd_imm_post::cond);
}

#[test]
fn test_strd_imm_pre() {
    test_ldst_pre("strd", a32::strd_imm_pre);
    test_ldst_pre_cond("strd", a32::strd_imm_pre::cond);
}

#[test]
fn test_strd_reg_offset() {
    test_ldst_offset("strd", a32::strd_reg_offset);
    test_ldst_offset_cond("strd", a32::strd_reg_offset::cond);
}

#[test]
fn test_strd_reg_post() {
    test_ldst_post("strd", a32::strd_reg_post);
    test_ldst_post_cond("strd", a32::strd_reg_post::cond);
}

#[test]
fn test_strd_reg_pre() {
    test_ldst_pre("strd", a32::strd_reg_pre);
    test_ldst_pre_cond("strd", a32::strd_reg_pre::cond);
}

#[test]
fn test_strex() {
    test4_format_cond("strex", a32::strex::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_strexb() {
    test4_format_cond("strexb", a32::strexb::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_strexd() {
    test4_format_cond("strexd", a32::strexd::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_strexh() {
    test4_format_cond("strexh", a32::strexh::cond,
        |rd, rt, rn| format!("{}, {}, [{}]", rd, rt, rn));
}

#[test]
fn test_strh_imm_offset() {
    test_ldst_offset("strh", a32::strh_imm_offset);
    test_ldst_offset_cond("strh", a32::strh_imm_offset::cond);
}

#[test]
fn test_strh_imm_post() {
    test_ldst_post("strh", a32::strh_imm_post);
    test_ldst_post_cond("strh", a32::strh_imm_post::cond);
}

#[test]
fn test_strh_imm_pre() {
    test_ldst_pre("strh", a32::strh_imm_pre);
    test_ldst_pre_cond("strh", a32::strh_imm_pre::cond);
}

#[test]
fn test_strh_reg_offset() {
    test_ldst_offset("strh", a32::strh_reg_offset);
    test_ldst_offset_cond("strh", a32::strh_reg_offset::cond);
}

#[test]
fn test_strh_reg_post() {
    test_ldst_post("strh", a32::strh_reg_post);
    test_ldst_post_cond("strh", a32::strh_reg_post::cond);
}

#[test]
fn test_strh_reg_pre() {
    test_ldst_pre("strh", a32::strh_reg_pre);
    test_ldst_pre_cond("strh", a32::strh_reg_pre::cond);
}

#[test]
fn test_strht_imm() {
    test_ldst_post("strht", a32::strht_imm);
    test_ldst_post_cond("strht", a32::strht_imm::cond);
}

#[test]
fn test_strht_reg() {
    test_ldst_post("strht", a32::strht_reg);
    test_ldst_post_cond("strht", a32::strht_reg::cond);
}

#[test]
fn test_strt_imm() {
    test_ldst_post_filter("strt", a32::strt_imm, |_, _, offset| {
        offset.abs_offset() < 0x1000
    });
    test_ldst_post_filter_cond("strt", a32::strt_imm::cond, |_, _, _, offset| {
        offset.abs_offset() < 0x1000
    });
}

#[test]
fn test_strt_reg() {
    test_ldst_post("strt", a32::strt_reg::<Register>);
    test_ldst_post("strt", a32::strt_reg::<Index>);
    test_ldst_post_cond("strt", a32::strt_reg::cond::<Register>);
    test_ldst_post_cond("strt", a32::strt_reg::cond::<Index>);
}

#[test]
fn test_sub_imm() {
    test4_cond("sub", a32::sub_imm::cond::<u8>);
    test4_cond("sub", a32::sub_imm::cond::<Const>);
}

#[test]
fn test_sub_reg() {
    test5_cond("sub", a32::sub_reg::cond);
}

#[test]
fn test_sub_reg_rrx() {
    test4_format_cond("sub", a32::sub_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_sub_reg_shifted_reg() {
    test5_cond("sub", a32::sub_reg_shifted_reg::cond);
}

#[test]
fn test_subs_imm() {
    test4_cond("subs", a32::subs_imm::cond::<u8>);
    test4_cond("subs", a32::subs_imm::cond::<Const>);
}

#[test]
fn test_subs_reg() {
    test5_cond("subs", a32::subs_reg::cond);
}

#[test]
fn test_subs_reg_rrx() {
    test4_format_cond("subs", a32::subs_reg_rrx::cond,
        |rd, rn, rm| format!("{}, {}, {}, rrx", rd, rn, rm));
}

#[test]
fn test_subs_reg_shifted_reg() {
    test5_cond("subs", a32::subs_reg_shifted_reg::cond);
}

#[test]
fn test_svc() {
    test_disasm("svc", "#0x345678", a32::svc(0x345678));
    test_disasm("svcge", "#0x345678", a32::svc::cond(Condition::Ge, 0x345678));
}

#[test]
fn test_sxtab() {
    test3_filter("sxtab", |rd, rn, rm| a32::sxtab(rd, rn, rm, Ror0),
        |rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test4_filter("sxtab", a32::sxtab,
        |rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
    test4_filter_cond("sxtab", |cond, rd, rn, rm| a32::sxtab::cond(cond, rd, rn, rm, Ror0),
        |_, rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test5_filter_cond("sxtab", a32::sxtab::cond,
        |_, rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
}

#[test]
fn test_sxtab16() {
    test3_filter("sxtab16", |rd, rn, rm| a32::sxtab16(rd, rn, rm, Ror0),
        |rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test4_filter("sxtab16", a32::sxtab16,
        |rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
    test4_filter_cond("sxtab16", |cond, rd, rn, rm| a32::sxtab16::cond(cond, rd, rn, rm, Ror0),
        |_, rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test5_filter_cond("sxtab16", a32::sxtab16::cond,
        |_, rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
}

#[test]
fn test_sxtah() {
    test3_filter("sxtah", |rd, rn, rm| a32::sxtah(rd, rn, rm, Ror0),
        |rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test4_filter("sxtah", a32::sxtah,
        |rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
    test4_filter_cond("sxtah", |cond, rd, rn, rm| a32::sxtah::cond(cond, rd, rn, rm, Ror0),
        |_, rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test5_filter_cond("sxtah", a32::sxtah::cond,
        |_, rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
}

#[test]
fn test_sxtb() {
    test2("sxtb", |rd, rm| a32::sxtb(rd, rm, Ror0));
    test3_filter("sxtb", a32::sxtb, |_, _, ror| ror != Ror0);
    test3_cond("sxtb", |cond, rd, rm| a32::sxtb::cond(cond, rd, rm, Ror0));
    test4_filter_cond("sxtb", a32::sxtb::cond, |_, _, _, ror| ror != Ror0);
}

#[test]
fn test_sxtb16() {
    test2("sxtb16", |rd, rm| a32::sxtb16(rd, rm, Ror0));
    test3_filter("sxtb16", a32::sxtb16, |_, _, ror| ror != Ror0);
    test3_cond("sxtb16", |cond, rd, rm| a32::sxtb16::cond(cond, rd, rm, Ror0));
    test4_filter_cond("sxtb16", a32::sxtb16::cond, |_, _, _, ror| ror != Ror0);
}

#[test]
fn test_sxth() {
    test2("sxth", |rd, rm| a32::sxth(rd, rm, Ror0));
    test3_filter("sxth", a32::sxth, |_, _, ror| ror != Ror0);
    test3_cond("sxth", |cond, rd, rm| a32::sxth::cond(cond, rd, rm, Ror0));
    test4_filter_cond("sxth", a32::sxth::cond, |_, _, _, ror| ror != Ror0);
}

#[test]
fn test_teq_imm() {
    test2("teq", a32::teq_imm::<u8>);
    test2("teq", a32::teq_imm::<Const>);
    test3_cond("teq", a32::teq_imm::cond::<u8>);
    test3_cond("teq", a32::teq_imm::cond::<Const>);
}

#[test]
fn test_teq_reg() {
    test3("teq", a32::teq_reg);
    test4_cond("teq", a32::teq_reg::cond);
}

#[test]
fn test_teq_reg_rrx() {
    test2_format("teq", a32::teq_reg_rrx,
        |rn, rm| format!("{}, {}, rrx", rn, rm));
    test3_format_cond("teq", a32::teq_reg_rrx::cond,
        |rn, rm| format!("{}, {}, rrx", rn, rm));
}

#[test]
fn test_teq_reg_shifted_reg() {
    test3("teq", a32::teq_reg_shifted_reg);
    test4_cond("teq", a32::teq_reg_shifted_reg::cond);
}

#[test]
fn test_tsb_csync() {
    test_disasm("tsb", "csync", trf::tsb_csync());
}

#[test]
fn test_tst_imm() {
    test2("tst", a32::tst_imm::<u8>);
    test2("tst", a32::tst_imm::<Const>);
    test3_cond("tst", a32::tst_imm::cond::<u8>);
    test3_cond("tst", a32::tst_imm::cond::<Const>);
}

#[test]
fn test_tst_reg() {
    test3("tst", a32::tst_reg);
    test4_cond("tst", a32::tst_reg::cond);
}

#[test]
fn test_tst_reg_rrx() {
    test2_format("tst", a32::tst_reg_rrx,
        |rn, rm| format!("{}, {}, rrx", rn, rm));
    test3_format_cond("tst", a32::tst_reg_rrx::cond,
        |rn, rm| format!("{}, {}, rrx", rn, rm));
}

#[test]
fn test_tst_reg_shifted_reg() {
    test3("tst", a32::tst_reg_shifted_reg);
    test4_cond("tst", a32::tst_reg_shifted_reg::cond);
}

#[test]
fn test_uadd8() {
    test4_cond("uadd8", a32::uadd8::cond);
}

#[test]
fn test_uadd16() {
    test4_cond("uadd16", a32::uadd16::cond);
}

#[test]
fn test_uasx() {
    test4_filter_cond("uasx", a32::uasx::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_ubfx() {
    test5_filter_cond("ubfx", a32::ubfx::cond, |_, rd, rn, lsb, width| {
        rd != R15 && rn != R15 && lsb <= 31 && width > 0 && width <= 32 - lsb
    });
}

#[test]
fn test_udf() {
    test_disasm("udf", "#0x10", a32::udf(16));
    test_disasm("udf", "#0x1234", a32::udf(0x1234));
    test_disasm("trap", "", a32::udf(0xfdee));
    test_disasm("udf", "#0xfdef", a32::udf(0xfdef));
}

#[test]
fn test_udiv() {
    test4_filter_cond("udiv", a32::udiv::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_uhadd8() {
    test4_filter_cond("uhadd8", a32::uhadd8::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_uhadd16() {
    test4_filter_cond("uhadd16", a32::uhadd16::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_uhasx() {
    test4_filter_cond("uhasx", a32::uhasx::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_uhsax() {
    test4_filter_cond("uhsax", a32::uhsax::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_uhsub8() {
    test4_filter_cond("uhsub8", a32::uhsub8::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_uhsub16() {
    test4_filter_cond("uhsub16", a32::uhsub16::cond, |_, a2, a3, a4| {
        a2 != R15 && a3 != R15 && a4 != R15
    });
}

#[test]
fn test_umaal() {
    test5_filter_cond("umaal", a32::umaal::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_umlal() {
    test5_filter_cond("umlal", a32::umlal::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_umlals() {
    test5_filter_cond("umlals", a32::umlals::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_umull() {
    test5_filter_cond("umull", a32::umull::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_umulls() {
    test5_filter_cond("umulls", a32::umulls::cond, |_, rdlo, rdhi, rn, rm| {
        rdlo != rdhi && rdlo != R15 && rdhi != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_uqadd8() {
    test4_filter_cond("uqadd8", a32::uqadd8::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_uqadd16() {
    test4_filter_cond("uqadd16", a32::uqadd16::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_uqasx() {
    test4_filter_cond("uqasx", a32::uqasx::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_uqsax() {
    test4_filter_cond("uqsax", a32::uqsax::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_uqsub8() {
    test4_filter_cond("uqsub8", a32::uqsub8::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_uqsub16() {
    test4_filter_cond("uqsub16", a32::uqsub16::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_usad8() {
    test4_filter_cond("usad8", a32::usad8::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_usada8() {
    test5_filter_cond("usada8", a32::usada8::cond, |_, rd, rn, rm, ra| {
        rd != R15 && rn != R15 && rm != R15 && ra != R15
    });
}

#[test]
fn test_usat_asr() {
    test5_format_filter_cond("usat", a32::usat_asr::cond,
        |rd, sat_imm, rn, imm5| format!("{}, {}, {}, asr {}", rd, sat_imm, rn, imm5),
        |_, _, sat_imm, _, imm5| sat_imm <= 31 && imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_usat_lsl() {
    test5_format_filter_cond("usat", a32::usat_lsl::cond,
        |rd, sat_imm, rn, imm5| format!("{}, {}, {}, lsl {}", rd, sat_imm, rn, imm5),
        |_, _, sat_imm, _, imm5| sat_imm <= 31 && imm5 > 0 && imm5 < 0x20);
}

#[test]
fn test_usat16() {
    test4_filter_cond("usat16", a32::usat16::cond, |_, rd, sat_imm, rn| {
        rd != R15 && rn != R15 && sat_imm <= 15
    });
}

#[test]
fn test_usax() {
    test4_filter_cond("usax", a32::usax::cond, |_, rd, rn, rm| {
        rd != R15 && rn != R15 && rm != R15
    });
}

#[test]
fn test_usub8() {
    test4_cond("usub8", a32::usub8::cond);
}

#[test]
fn test_usub16() {
    test4_cond("usub16", a32::usub16::cond);
}

#[test]
fn test_uxtab() {
    test3_filter("uxtab", |rd, rn, rm| a32::uxtab(rd, rn, rm, Ror0),
        |rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test4_filter("uxtab", a32::uxtab,
        |rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
    test4_filter_cond("uxtab", |cond, rd, rn, rm| a32::uxtab::cond(cond, rd, rn, rm, Ror0),
        |_, rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test5_filter_cond("uxtab", a32::uxtab::cond,
        |_, rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
}

#[test]
fn test_uxtab16() {
    test3_filter("uxtab16", |rd, rn, rm| a32::uxtab16(rd, rn, rm, Ror0),
        |rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test4_filter("uxtab16", a32::uxtab16,
        |rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
    test4_filter_cond("uxtab16", |cond, rd, rn, rm| a32::uxtab16::cond(cond, rd, rn, rm, Ror0),
        |_, rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test5_filter_cond("uxtab16", a32::uxtab16::cond,
        |_, rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
}

#[test]
fn test_uxtah() {
    test3_filter("uxtah", |rd, rn, rm| a32::uxtah(rd, rn, rm, Ror0),
        |rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test4_filter("uxtah", a32::uxtah,
        |rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
    test4_filter_cond("uxtah", |cond, rd, rn, rm| a32::uxtah::cond(cond, rd, rn, rm, Ror0),
        |_, rd, rn, rm| rd != R15 && rn != R15 && rm != R15);
    test5_filter_cond("uxtah", a32::uxtah::cond,
        |_, rd, rn, rm, ror| rd != R15 && rn != R15 && rm != R15 && ror != Ror0);
}

#[test]
fn test_uxtb() {
    test2("uxtb", |rd, rm| a32::uxtb(rd, rm, Ror0));
    test3_filter("uxtb", a32::uxtb, |_, _, ror| ror != Ror0);
    test3_cond("uxtb", |cond, rd, rm| a32::uxtb::cond(cond, rd, rm, Ror0));
    test4_filter_cond("uxtb", a32::uxtb::cond, |_, _, _, ror| ror != Ror0);
}

#[test]
fn test_uxtb16() {
    test2("uxtb16", |rd, rm| a32::uxtb16(rd, rm, Ror0));
    test3_filter("uxtb16", a32::uxtb16, |_, _, ror| ror != Ror0);
    test3_cond("uxtb16", |cond, rd, rm| a32::uxtb16::cond(cond, rd, rm, Ror0));
    test4_filter_cond("uxtb16", a32::uxtb16::cond, |_, _, _, ror| ror != Ror0);
}

#[test]
fn test_uxth() {
    test2("uxth", |rd, rm| a32::uxth(rd, rm, Ror0));
    test3_filter("uxth", a32::uxth, |_, _, ror| ror != Ror0);
    test3_cond("uxth", |cond, rd, rm| a32::uxth::cond(cond, rd, rm, Ror0));
    test4_filter_cond("uxth", a32::uxth::cond, |_, _, _, ror| ror != Ror0);
}

#[test]
fn test_wfe() {
    test_disasm("wfe", "", a32::wfe());
    test1_cond("wfe", a32::wfe::cond);
}

#[test]
fn test_wfi() {
    test_disasm("wfi", "", a32::wfi());
    test1_cond("wfi", a32::wfi::cond);
}

#[test]
fn test_yield() {
    test_disasm("yield", "", a32::yield_());
    test1_cond("yield", a32::yield_::cond);
}
