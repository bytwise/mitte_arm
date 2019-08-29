extern crate mitte_arm;
extern crate capstone;

use mitte_arm::*;
use mitte_arm::Register::*;

use capstone::Capstone;
use capstone::arch::BuildsCapstone;
use capstone::arch::arm::ArchMode;


fn print_code(code: &[u8]) {
    print!("[");
    for b in code {
        print!("{:02x}", b);
    }
    println!("]");
}


fn test_disasm<S>(mnemonic: &str, expected: &[S], code: &[u8]) where S: AsRef<str> {
    let capstone = Capstone::new().arm().mode(ArchMode::Arm).build().unwrap();
    let disasm = match capstone.disasm_all(code, 0x0) {
        Ok(disasm) => disasm,
        Err(error) => {
            print_code(code);
            panic!("failed to disassemble: {:?}", error);
        }
    };

    for (i, e) in disasm.iter().zip(expected) {
        println!("bytes: {:?}", i.bytes());
        assert_eq!(mnemonic, i.mnemonic().unwrap());
        assert_eq!(Some(e.as_ref()), i.op_str(),
            "{} {:?} != {0} {:?}", mnemonic,
            e.as_ref(), i.op_str().unwrap());
    }
    assert_eq!(disasm.len() as usize, expected.len());
}


#[test]
fn test_add() {
    test_disasm("add", &["r3, r4, #5"], &add_imm(R3, R4, 5));
    test_disasm("add", &["r3, r4, #0x50"], &add_imm(R3, R4, 0x50));
    test_disasm("add", &["r3, r4, #0x500"], &add_imm(R3, R4, 0x500));
    test_disasm("add", &["r3, r4, #0x5000"], &add_imm(R3, R4, 0x5000));
    test_disasm("add", &["r3, r4, #0x50000"], &add_imm(R3, R4, 0x50000));
    test_disasm("add", &["r3, r4, #0x500000"], &add_imm(R3, R4, 0x500000));
    test_disasm("add", &["r3, r4, #0x5000000"], &add_imm(R3, R4, 0x5000000));
    test_disasm("add", &["r3, r4, #0x50000000"], &add_imm(R3, R4, 0x50000000));
    test_disasm("add", &["r3, r4, #0x50000006"], &add_imm(R3, R4, 0x50000006));
    test_disasm("add", &["r3, r4, r5"], &add_reg(R3, R4, R5, Shift::Lsl(0)));
    test_disasm("add", &["r3, r4, r5, lsl #6"], &add_reg(R3, R4, R5, Shift::Lsl(6)));
    test_disasm("add", &["r3, r4, r5, lsr #32"], &add_reg(R3, R4, R5, Shift::Lsr(32)));
    test_disasm("add", &["r3, r4, r5, lsr #6"], &add_reg(R3, R4, R5, Shift::Lsr(6)));
    test_disasm("add", &["r3, r4, r5, asr #32"], &add_reg(R3, R4, R5, Shift::Asr(32)));
    test_disasm("add", &["r3, r4, r5, asr #6"], &add_reg(R3, R4, R5, Shift::Asr(6)));
    test_disasm("add", &["r3, r4, r5, rrx"], &add_reg(R3, R4, R5, Shift::Ror(0)));
    test_disasm("add", &["r3, r4, r5, ror #6"], &add_reg(R3, R4, R5, Shift::Ror(6)));
}

#[test]
fn test_orr() {
    test_disasm("orr", &["r3, r4, r5"], &orr_reg(R3, R4, R5, Shift::Lsl(0)));
    test_disasm("orr", &["r3, r4, r5, lsl r6"], &orr_reg_shifted_reg(R3, R4, R5, Shift::Lsl(R6)));
}

#[test]
fn test_and() {
    test_disasm("and", &["r3, r4, #5"], &and_imm(R3, R4, 5));
    test_disasm("and", &["r3, r4, r5"], &and_reg(R3, R4, R5, Shift::Lsl(0)));
    test_disasm("and", &["r3, r4, r5, lsl r6"], &and_reg_shifted_reg(R3, R4, R5, Shift::Lsl(R6)));
}

#[test]
fn test_eor() {
    test_disasm("eor", &["r3, r4, r5"], &eor_reg(R3, R4, R5, Shift::Lsl(0)));
}

#[test]
fn test_sub() {
    test_disasm("sub", &["r3, r4, #5"], &sub_imm(R3, R4, 5));
    test_disasm("sub", &["r3, r4, r5"], &sub_reg(R3, R4, R5, Shift::Lsl(0)));
}

#[test]
fn test_rsb() {
    test_disasm("rsb", &["r3, r4, #5"], &rsb_imm(R3, R4, 5));
}

#[test]
fn test_bic() {
    test_disasm("bic", &["r3, r4, #5"], &bic_imm(R3, R4, 5));
    test_disasm("bic", &["r3, r4, r5, lsl r6"], &bic_reg_shifted_reg(R3, R4, R5, Shift::Lsl(R6)));
}

#[test]
fn test_lsl() {
    test_disasm("lsl", &["r3, r4, #5"], &lsl_imm(R3, R4, 5));
    test_disasm("lsl", &["r3, r4, r5"], &lsl_reg(R3, R4, R5));
}

#[test]
fn test_lsr() {
    test_disasm("lsr", &["r3, r4, #5"], &lsr_imm(R3, R4, 5));
    test_disasm("lsr", &["r3, r4, r5"], &lsr_reg(R3, R4, R5));
}

#[test]
fn test_asr() {
    test_disasm("asr", &["r3, r4, #5"], &asr_imm(R3, R4, 5));
    test_disasm("asr", &["r3, r4, r5"], &asr_reg(R3, R4, R5));
}

#[test]
fn test_ror() {
    test_disasm("ror", &["r3, r4, #5"], &ror_imm(R3, R4, 5));
    test_disasm("ror", &["r3, r4, r5"], &ror_reg(R3, R4, R5));
}

#[test]
fn test_mov() {
    test_disasm("mov", &["r3, #4"], &mov_imm(R3, 4));
    test_disasm("mov", &["r3, #0x4000"], &mov_imm(R3, 0x4000));
    test_disasm("mov", &["r3, #0x40000000"], &mov_imm(R3, 0x40000000));
    test_disasm("mov", &["r3, r4"], &mov_reg(R3, R4));
}

#[test]
fn test_movt() {
    test_disasm("movt", &["r3, #0x1234"], &movt(R3, 0x1234));
}

#[test]
fn test_movw() {
    test_disasm("movw", &["r3, #0x1234"], &movw(R3, 0x1234));
}

#[test]
fn test_sxtb() {
    test_disasm("sxtb", &["r3, r4"], &sxtb(R3, R4));
}

#[test]
fn test_sxth() {
    test_disasm("sxth", &["r3, r4"], &sxth(R3, R4));
}

#[test]
fn test_uxth() {
    test_disasm("uxth", &["r3, r4"], &uxth(R3, R4));
}

#[test]
fn test_blx() {
    test_disasm("blx", &["r3"], &blx(R3));
}

#[test]
fn test_cmp() {
    test_disasm("cmp", &["r3, #4"], &cmp_imm(R3, 4));
    test_disasm("cmp", &["r3, #4"], &cmp_imm(R3, 4u32));
    test_disasm("cmp", &["r3, #0x40"], &cmp_imm(R3, 0x40u32));
    test_disasm("cmp", &["r3, #0x400"], &cmp_imm(R3, 0x400u32));
    test_disasm("cmp", &["r3, #0x4000"], &cmp_imm(R3, 0x4000u32));
    test_disasm("cmp", &["r3, #0x40000"], &cmp_imm(R3, 0x40000u32));
    test_disasm("cmp", &["r3, #0x400000"], &cmp_imm(R3, 0x400000u32));
    test_disasm("cmp", &["r3, #0x4000000"], &cmp_imm(R3, 0x4000000u32));
    test_disasm("cmp", &["r3, #0x40000000"], &cmp_imm(R3, 0x40000000u32));
    test_disasm("cmp", &["r3, #0x40000005"], &cmp_imm(R3, 0x40000005u32));
    test_disasm("cmp", &["r3, r4"], &cmp_reg(R3, R4, Shift::Lsl(0)));
    test_disasm("cmp", &["r3, r4, lsl #5"], &cmp_reg(R3, R4, Shift::Lsl(5)));
}

#[test]
fn test_cmn() {
    test_disasm("cmn", &["r3, #4"], &cmn_imm_cc(Condition::Al, R3, 4));
    test_disasm("cmneq", &["r3, #4"], &cmn_imm_cc(Condition::Eq, R3, 4));
}

#[test]
fn test_b() {
    test_disasm("b", &["#0x345678"], &b((0x345678 - 8) >> 2));
    test_disasm("b", &["#0x345678"], &b_cc(Condition::Al, (0x345678 - 8) >> 2));
    test_disasm("beq", &["#0x345678"], &beq((0x345678 - 8) >> 2));
    test_disasm("beq", &["#0x345678"], &b_cc(Condition::Eq, (0x345678 - 8) >> 2));
    test_disasm("bne", &["#0x345678"], &bne((0x345678 - 8) >> 2));
    test_disasm("bgt", &["#0x345678"], &bgt((0x345678 - 8) >> 2));
}

#[test]
fn test_bl() {
    test_disasm("bl", &["#0x345678"], &bl((0x345678 - 8) >> 2));
}

#[test]
fn test_ldr() {
    test_disasm("ldr", &["r3, [r4, #0x42]"], &ldr_imm(R3, R4, 0x42));
    test_disasm("ldr", &["r3, [r4, r5]"], &ldr_reg(R3, R4, R5));
}

#[test]
fn test_ldrh() {
    test_disasm("ldrh", &["r3, [r4, #5]"], &ldrh_imm(R3, R4, 5));
    test_disasm("ldrh", &["r3, [r4, r5]"], &ldrh_reg(R3, R4, R5));
}

#[test]
fn test_ldrsh() {
    test_disasm("ldrsh", &["r3, [r4, #5]"], &ldrsh_imm(R3, R4, 5));
    test_disasm("ldrsh", &["r3, [r4, r5]"], &ldrsh_reg(R3, R4, R5));
}

#[test]
fn test_str() {
    test_disasm("str", &["r3, [r4, #0x42]"], &str_imm(R3, R4, 0x42));
    test_disasm("str", &["r3, [r4, r5]"], &str_reg(R3, R4, R5));
}

#[test]
fn test_strh() {
    test_disasm("strh", &["r3, [r4]"], &strh(R3, R4));
}

#[test]
fn test_push() {
    test_disasm("push", &["{r3, r4}"], &push([R3, R4]));
    test_disasm("push", &["{r3, r4}"], &push(RegisterList::from_slice(&[R3, R4])));
}

#[test]
fn test_pop() {
    test_disasm("pop", &["{r3, r4}"], &pop([R3, R4]));
    test_disasm("pop", &["{r3, r4}"], &pop(RegisterList::from_slice(&[R3, R4])));
}

#[test]
fn test_smull() {
    test_disasm("smull", &["r3, r4, r5, r6"], &smull(R3, R4, R5, R6));
}

#[test]
fn test_umull() {
    test_disasm("umull", &["r3, r4, r5, r6"], &umull(R3, R4, R5, R6));
}

#[test]
fn test_sdiv() {
    test_disasm("sdiv", &["r3, r4, r5"], &sdiv(R3, R4, R5));
}

#[test]
fn test_udiv() {
    test_disasm("udiv", &["r3, r4, r5"], &udiv(R3, R4, R5));
}

#[test]
fn test_mls() {
    test_disasm("mls", &["r3, r4, r5, r6"], &mls(R3, R4, R5, R6));
}
