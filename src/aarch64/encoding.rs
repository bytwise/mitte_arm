#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::encoding::*;
use super::types::*;


// ADC  <Wd>, <Wn>, <Wm>
pub struct ADC_32_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADC_32_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADC  <Xd>, <Xn>, <Xm>
pub struct ADC_64_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADC_64_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADCS  <Wd>, <Wn>, <Wm>
pub struct ADCS_32_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADCS_32_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADCS  <Xd>, <Xn>, <Xm>
pub struct ADCS_64_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADCS_64_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADD  <Wd|WSP>, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
pub struct ADD_32_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADD_32_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADD  <Xd|SP>, <Xn|SP>, <R><m>{, <extend> {#<amount>}}
pub struct ADD_64_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADD_64_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADD  <Wd|WSP>, <Wn|WSP>, #<imm>{, <shift>}
pub struct ADD_32_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADD_32_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADD  <Xd|SP>, <Xn|SP>, #<imm>{, <shift>}
pub struct ADD_64_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADD_64_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADD  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct ADD_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADD_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADD  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct ADD_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADD_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADDS  <Wd>, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
pub struct ADDS_32S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADDS_32S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADDS  <Xd>, <Xn|SP>, <R><m>{, <extend> {#<amount>}}
pub struct ADDS_64S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADDS_64S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADDS  <Wd>, <Wn|WSP>, #<imm>{, <shift>}
pub struct ADDS_32S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADDS_32S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADDS  <Xd>, <Xn|SP>, #<imm>{, <shift>}
pub struct ADDS_64S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADDS_64S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADDS  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct ADDS_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADDS_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADDS  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct ADDS_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ADDS_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ADR  <Xd>, <label>
pub struct ADR_only_pcreladdr {
    pub immlo: u32,
    pub immhi: u32,
    pub rd: RegisterIndex,
}
impl ADR_only_pcreladdr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(self.immlo),
            i5(0b10000),
            i19(self.immhi),
            i5(self.rd.into())
        )
    }
}

// ADRP  <Xd>, <label>
pub struct ADRP_only_pcreladdr {
    pub immlo: u32,
    pub immhi: u32,
    pub rd: RegisterIndex,
}
impl ADRP_only_pcreladdr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(self.immlo),
            i5(0b10000),
            i19(self.immhi),
            i5(self.rd.into())
        )
    }
}

// AND  <Wd|WSP>, <Wn>, #<imm>
pub struct AND_32_log_imm {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AND_32_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100100),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AND  <Xd|SP>, <Xn>, #<imm>
pub struct AND_64_log_imm {
    pub n: u32,
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AND_64_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100100),
            i1(self.n),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AND  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct AND_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AND_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AND  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct AND_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AND_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ANDS  <Wd>, <Wn>, #<imm>
pub struct ANDS_32S_log_imm {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ANDS_32S_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b11),
            i6(0b100100),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ANDS  <Xd>, <Xn>, #<imm>
pub struct ANDS_64S_log_imm {
    pub n: u32,
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ANDS_64S_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b11),
            i6(0b100100),
            i1(self.n),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ANDS  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct ANDS_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ANDS_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b11),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ANDS  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct ANDS_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ANDS_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b11),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ASR  <Wd>, <Wn>, <Wm>
pub struct ASR_ASRV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ASR_ASRV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ASR  <Xd>, <Xn>, <Xm>
pub struct ASR_ASRV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ASR_ASRV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ASR  <Wd>, <Wn>, #<shift>
pub struct ASR_SBFM_32M_bitfield {
    pub immr: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ASR_SBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(0b011111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ASR  <Xd>, <Xn>, #<shift>
pub struct ASR_SBFM_64M_bitfield {
    pub immr: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ASR_SBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(0b111111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ASRV  <Wd>, <Wn>, <Wm>
pub struct ASRV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ASRV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ASRV  <Xd>, <Xn>, <Xm>
pub struct ASRV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ASRV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AT  <at_op>, <Xt>
pub struct AT_SYS_CR_systeminstrs {
    pub op1: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl AT_SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(self.op1),
            i4(0b0111),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// AUTDA  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct AUTDA_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AUTDA_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b110),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AUTDZA  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct AUTDZA_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl AUTDZA_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b110),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// AUTDB  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct AUTDB_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AUTDB_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AUTDZB  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct AUTDZB_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl AUTDZB_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b111),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// AUTIA  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct AUTIA_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AUTIA_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b100),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AUTIZA  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct AUTIZA_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl AUTIZA_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b100),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// AUTIA1716
// ARMv8.3, FEAT_PAuth
pub struct AUTIA1716_HI_hints {
}
impl AUTIA1716_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0001),
            i3(0b100),
            i5(0b11111)
        )
    }
}

// AUTIASP
// ARMv8.3, FEAT_PAuth
pub struct AUTIASP_HI_hints {
}
impl AUTIASP_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b101),
            i5(0b11111)
        )
    }
}

// AUTIAZ
// ARMv8.3, FEAT_PAuth
pub struct AUTIAZ_HI_hints {
}
impl AUTIAZ_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b100),
            i5(0b11111)
        )
    }
}

// AUTIB  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct AUTIB_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl AUTIB_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b101),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// AUTIZB  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct AUTIZB_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl AUTIZB_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b101),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// AUTIB1716
// ARMv8.3, FEAT_PAuth
pub struct AUTIB1716_HI_hints {
}
impl AUTIB1716_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0001),
            i3(0b110),
            i5(0b11111)
        )
    }
}

// AUTIBSP
// ARMv8.3, FEAT_PAuth
pub struct AUTIBSP_HI_hints {
}
impl AUTIBSP_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b111),
            i5(0b11111)
        )
    }
}

// AUTIBZ
// ARMv8.3, FEAT_PAuth
pub struct AUTIBZ_HI_hints {
}
impl AUTIBZ_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b110),
            i5(0b11111)
        )
    }
}

// AXFLAG
// ARMv8.5, FEAT_FlagM2
pub struct AXFLAG_M_pstate {
}
impl AXFLAG_M_pstate {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b000),
            i4(0b0100),
            i4(0b0000),
            i3(0b010),
            i5(0b11111)
        )
    }
}

// B.<cond>  <label>
pub struct B_only_condbranch {
    pub imm19: u32,
    pub cond: Condition,
}
impl B_only_condbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b0101010),
            i1(0b0),
            i19(self.imm19),
            i1(0b0),
            i4(self.cond as u32)
        )
    }
}

// B  <label>
pub struct B_only_branch_imm {
    pub imm26: u32,
}
impl B_only_branch_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i5(0b00101),
            i26(self.imm26)
        )
    }
}

// BC.<cond>  <label>
// ARMv8.8, FEAT_HBC
pub struct BC_only_condbranch {
    pub imm19: u32,
    pub cond: Condition,
}
impl BC_only_condbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b0101010),
            i1(0b0),
            i19(self.imm19),
            i1(0b1),
            i4(self.cond as u32)
        )
    }
}

// BFC  <Wd>, #<lsb>, #<width>
// ARMv8.2, FEAT_ASMv8p2
pub struct BFC_BFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rd: RegisterIndex,
}
impl BFC_BFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// BFC  <Xd>, #<lsb>, #<width>
// ARMv8.2, FEAT_ASMv8p2
pub struct BFC_BFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rd: RegisterIndex,
}
impl BFC_BFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// BFI  <Wd>, <Wn>, #<lsb>, #<width>
pub struct BFI_BFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BFI_BFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BFI  <Xd>, <Xn>, #<lsb>, #<width>
pub struct BFI_BFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BFI_BFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BFM  <Wd>, <Wn>, #<immr>, #<imms>
pub struct BFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BFM  <Xd>, <Xn>, #<immr>, #<imms>
pub struct BFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BFXIL  <Wd>, <Wn>, #<lsb>, #<width>
pub struct BFXIL_BFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BFXIL_BFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BFXIL  <Xd>, <Xn>, #<lsb>, #<width>
pub struct BFXIL_BFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BFXIL_BFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BIC  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct BIC_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BIC_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BIC  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct BIC_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BIC_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BICS  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct BICS_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BICS_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b11),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BICS  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct BICS_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl BICS_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b11),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// BL  <label>
pub struct BL_only_branch_imm {
    pub imm26: u32,
}
impl BL_only_branch_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i5(0b00101),
            i26(self.imm26)
        )
    }
}

// BLR  <Xn>
pub struct BLR_64_branch_reg {
    pub rn: RegisterIndex,
}
impl BLR_64_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b01),
            i5(0b11111),
            i4(0b0000),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(0b00000)
        )
    }
}

// BLRAAZ  <Xn>
// ARMv8.3, FEAT_PAuth
pub struct BLRAAZ_64_branch_reg {
    pub rn: RegisterIndex,
}
impl BLRAAZ_64_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b01),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// BLRAA  <Xn>, <Xm|SP>
// ARMv8.3, FEAT_PAuth
pub struct BLRAA_64P_branch_reg {
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl BLRAA_64P_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rm.into())
        )
    }
}

// BLRABZ  <Xn>
// ARMv8.3, FEAT_PAuth
pub struct BLRABZ_64_branch_reg {
    pub rn: RegisterIndex,
}
impl BLRABZ_64_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b01),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// BLRAB  <Xn>, <Xm|SP>
// ARMv8.3, FEAT_PAuth
pub struct BLRAB_64P_branch_reg {
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl BLRAB_64P_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rm.into())
        )
    }
}

// BR  <Xn>
pub struct BR_64_branch_reg {
    pub rn: RegisterIndex,
}
impl BR_64_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b00),
            i5(0b11111),
            i4(0b0000),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(0b00000)
        )
    }
}

// BRAAZ  <Xn>
// ARMv8.3, FEAT_PAuth
pub struct BRAAZ_64_branch_reg {
    pub rn: RegisterIndex,
}
impl BRAAZ_64_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b00),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// BRAA  <Xn>, <Xm|SP>
// ARMv8.3, FEAT_PAuth
pub struct BRAA_64P_branch_reg {
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl BRAA_64P_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rm.into())
        )
    }
}

// BRABZ  <Xn>
// ARMv8.3, FEAT_PAuth
pub struct BRABZ_64_branch_reg {
    pub rn: RegisterIndex,
}
impl BRABZ_64_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b00),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// BRAB  <Xn>, <Xm|SP>
// ARMv8.3, FEAT_PAuth
pub struct BRAB_64P_branch_reg {
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl BRAB_64P_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rm.into())
        )
    }
}

// BRK  #<imm>
pub struct BRK_EX_exception {
    pub imm16: u32,
}
impl BRK_EX_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b001),
            i16(self.imm16),
            i3(0b000),
            i2(0b00)
        )
    }
}

// BTI  {<targets>}
// ARMv8.5, FEAT_BTI
pub struct BTI_HB_hints {
    pub op2: u32,
}
impl BTI_HB_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0100),
            i3(self.op2),
            i5(0b11111)
        )
    }
}

// CAS  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CAS_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CAS_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASA  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASA_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASA_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASAL  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASAL_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASAL_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASL  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASL_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASL_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CAS  <Xs>, <Xt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CAS_C64_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CAS_C64_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASA  <Xs>, <Xt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASA_C64_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASA_C64_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASAL  <Xs>, <Xt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASAL_C64_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASAL_C64_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASL  <Xs>, <Xt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASL_C64_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASL_C64_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASAB  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASAB_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASAB_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASALB  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASALB_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASALB_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASB  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASB_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASB_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASLB  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASLB_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASLB_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASAH  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASAH_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASAH_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASALH  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASALH_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASALH_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i7(0b0010001),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASH  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASH_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASH_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASLH  <Ws>, <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASLH_C32_comswap {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASLH_C32_comswap {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i7(0b0010001),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASP  <Ws>, <W(s+1)>, <Wt>, <W(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASP_CP32_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASP_CP32_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASPA  <Ws>, <W(s+1)>, <Wt>, <W(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASPA_CP32_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASPA_CP32_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASPAL  <Ws>, <W(s+1)>, <Wt>, <W(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASPAL_CP32_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASPAL_CP32_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASPL  <Ws>, <W(s+1)>, <Wt>, <W(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASPL_CP32_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASPL_CP32_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASP  <Xs>, <X(s+1)>, <Xt>, <X(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASP_CP64_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASP_CP64_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASPA  <Xs>, <X(s+1)>, <Xt>, <X(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASPA_CP64_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASPA_CP64_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASPAL  <Xs>, <X(s+1)>, <Xt>, <X(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASPAL_CP64_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASPAL_CP64_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CASPL  <Xs>, <X(s+1)>, <Xt>, <X(t+1)>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LSE
pub struct CASPL_CP64_comswappr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl CASPL_CP64_comswappr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// CBNZ  <Wt>, <label>
pub struct CBNZ_32_compbranch {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl CBNZ_32_compbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i6(0b011010),
            i1(0b1),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// CBNZ  <Xt>, <label>
pub struct CBNZ_64_compbranch {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl CBNZ_64_compbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i6(0b011010),
            i1(0b1),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// CBZ  <Wt>, <label>
pub struct CBZ_32_compbranch {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl CBZ_32_compbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i6(0b011010),
            i1(0b0),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// CBZ  <Xt>, <label>
pub struct CBZ_64_compbranch {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl CBZ_64_compbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i6(0b011010),
            i1(0b0),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// CCMN  <Wn>, #<imm>, #<nzcv>, <cond>
pub struct CCMN_32_condcmp_imm {
    pub imm5: u32,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMN_32_condcmp_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i8(0b11010010),
            i5(self.imm5),
            i4(self.cond as u32),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CCMN  <Xn>, #<imm>, #<nzcv>, <cond>
pub struct CCMN_64_condcmp_imm {
    pub imm5: u32,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMN_64_condcmp_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i8(0b11010010),
            i5(self.imm5),
            i4(self.cond as u32),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CCMN  <Wn>, <Wm>, #<nzcv>, <cond>
pub struct CCMN_32_condcmp_reg {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMN_32_condcmp_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i8(0b11010010),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CCMN  <Xn>, <Xm>, #<nzcv>, <cond>
pub struct CCMN_64_condcmp_reg {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMN_64_condcmp_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i8(0b11010010),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CCMP  <Wn>, #<imm>, #<nzcv>, <cond>
pub struct CCMP_32_condcmp_imm {
    pub imm5: u32,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMP_32_condcmp_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i8(0b11010010),
            i5(self.imm5),
            i4(self.cond as u32),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CCMP  <Xn>, #<imm>, #<nzcv>, <cond>
pub struct CCMP_64_condcmp_imm {
    pub imm5: u32,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMP_64_condcmp_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i8(0b11010010),
            i5(self.imm5),
            i4(self.cond as u32),
            i1(0b1),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CCMP  <Wn>, <Wm>, #<nzcv>, <cond>
pub struct CCMP_32_condcmp_reg {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMP_32_condcmp_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i8(0b11010010),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CCMP  <Xn>, <Xm>, #<nzcv>, <cond>
pub struct CCMP_64_condcmp_reg {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub nzcv: u32,
}
impl CCMP_64_condcmp_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i8(0b11010010),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.nzcv)
        )
    }
}

// CFINV
// ARMv8.4, FEAT_FlagM
pub struct CFINV_M_pstate {
}
impl CFINV_M_pstate {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b000),
            i4(0b0100),
            i4(0b0000),
            i3(0b000),
            i5(0b11111)
        )
    }
}

// CFP  RCTX, <Xt>
// ARMv8.5, FEAT_SPECRES
pub struct CFP_SYS_CR_systeminstrs {
    pub rt: RegisterIndex,
}
impl CFP_SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(0b011),
            i4(0b0111),
            i4(0b0011),
            i3(0b100),
            i5(self.rt.into())
        )
    }
}

// CINC  <Wd>, <Wn>, <cond>
pub struct CINC_CSINC_32_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CINC_CSINC_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CINC  <Xd>, <Xn>, <cond>
pub struct CINC_CSINC_64_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CINC_CSINC_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CINV  <Wd>, <Wn>, <cond>
pub struct CINV_CSINV_32_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CINV_CSINV_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CINV  <Xd>, <Xn>, <cond>
pub struct CINV_CSINV_64_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CINV_CSINV_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CLREX  {#<imm>}
pub struct CLREX_BN_barriers {
    pub crm: u32,
}
impl CLREX_BN_barriers {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0011),
            i4(self.crm),
            i3(0b010),
            i5(0b11111)
        )
    }
}

// CLS  <Wd>, <Wn>
pub struct CLS_32_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CLS_32_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i5(0b00010),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CLS  <Xd>, <Xn>
pub struct CLS_64_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CLS_64_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i5(0b00010),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CLZ  <Wd>, <Wn>
pub struct CLZ_32_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CLZ_32_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i5(0b00010),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CLZ  <Xd>, <Xn>
pub struct CLZ_64_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CLZ_64_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i5(0b00010),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CMN  <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
pub struct CMN_ADDS_32S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
}
impl CMN_ADDS_32S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMN  <Xn|SP>, <R><m>{, <extend> {#<amount>}}
pub struct CMN_ADDS_64S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
}
impl CMN_ADDS_64S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMN  <Wn|WSP>, #<imm>{, <shift>}
pub struct CMN_ADDS_32S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
}
impl CMN_ADDS_32S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMN  <Xn|SP>, #<imm>{, <shift>}
pub struct CMN_ADDS_64S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
}
impl CMN_ADDS_64S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMN  <Wn>, <Wm>{, <shift> #<amount>}
pub struct CMN_ADDS_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
}
impl CMN_ADDS_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMN  <Xn>, <Xm>{, <shift> #<amount>}
pub struct CMN_ADDS_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
}
impl CMN_ADDS_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMP  <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
pub struct CMP_SUBS_32S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
}
impl CMP_SUBS_32S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMP  <Xn|SP>, <R><m>{, <extend> {#<amount>}}
pub struct CMP_SUBS_64S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
}
impl CMP_SUBS_64S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMP  <Wn|WSP>, #<imm>{, <shift>}
pub struct CMP_SUBS_32S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
}
impl CMP_SUBS_32S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMP  <Xn|SP>, #<imm>{, <shift>}
pub struct CMP_SUBS_64S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
}
impl CMP_SUBS_64S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMP  <Wn>, <Wm>{, <shift> #<amount>}
pub struct CMP_SUBS_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
}
impl CMP_SUBS_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CMP  <Xn>, <Xm>{, <shift> #<amount>}
pub struct CMP_SUBS_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
}
impl CMP_SUBS_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// CNEG  <Wd>, <Wn>, <cond>
pub struct CNEG_CSNEG_32_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CNEG_CSNEG_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CNEG  <Xd>, <Xn>, <cond>
pub struct CNEG_CSNEG_64_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CNEG_CSNEG_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CPP  RCTX, <Xt>
// ARMv8.5, FEAT_SPECRES
pub struct CPP_SYS_CR_systeminstrs {
    pub rt: RegisterIndex,
}
impl CPP_SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(0b011),
            i4(0b0111),
            i4(0b0011),
            i3(0b111),
            i5(self.rt.into())
        )
    }
}

// CRC32B  <Wd>, <Wn>, <Wm>
pub struct CRC32B_32C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32B_32C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b0),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CRC32H  <Wd>, <Wn>, <Wm>
pub struct CRC32H_32C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32H_32C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b0),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CRC32W  <Wd>, <Wn>, <Wm>
pub struct CRC32W_32C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32W_32C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b0),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CRC32X  <Wd>, <Wn>, <Xm>
pub struct CRC32X_64C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32X_64C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b0),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CRC32CB  <Wd>, <Wn>, <Wm>
pub struct CRC32CB_32C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32CB_32C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b1),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CRC32CH  <Wd>, <Wn>, <Wm>
pub struct CRC32CH_32C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32CH_32C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b1),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CRC32CW  <Wd>, <Wn>, <Wm>
pub struct CRC32CW_32C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32CW_32C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b1),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CRC32CX  <Wd>, <Wn>, <Xm>
pub struct CRC32CX_64C_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CRC32CX_64C_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i3(0b010),
            i1(0b1),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSDB
pub struct CSDB_HI_hints {
}
impl CSDB_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0010),
            i3(0b100),
            i5(0b11111)
        )
    }
}

// CSEL  <Wd>, <Wn>, <Wm>, <cond>
pub struct CSEL_32_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSEL_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSEL  <Xd>, <Xn>, <Xm>, <cond>
pub struct CSEL_64_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSEL_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSET  <Wd>, <cond>
pub struct CSET_CSINC_32_condsel {
    pub cond: Condition,
    pub rd: RegisterIndex,
}
impl CSET_CSINC_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(0b11111),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// CSET  <Xd>, <cond>
pub struct CSET_CSINC_64_condsel {
    pub cond: Condition,
    pub rd: RegisterIndex,
}
impl CSET_CSINC_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(0b11111),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// CSETM  <Wd>, <cond>
pub struct CSETM_CSINV_32_condsel {
    pub cond: Condition,
    pub rd: RegisterIndex,
}
impl CSETM_CSINV_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(0b11111),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// CSETM  <Xd>, <cond>
pub struct CSETM_CSINV_64_condsel {
    pub cond: Condition,
    pub rd: RegisterIndex,
}
impl CSETM_CSINV_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(0b11111),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// CSINC  <Wd>, <Wn>, <Wm>, <cond>
pub struct CSINC_32_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSINC_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSINC  <Xd>, <Xn>, <Xm>, <cond>
pub struct CSINC_64_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSINC_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSINV  <Wd>, <Wn>, <Wm>, <cond>
pub struct CSINV_32_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSINV_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSINV  <Xd>, <Xn>, <Xm>, <cond>
pub struct CSINV_64_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSINV_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSNEG  <Wd>, <Wn>, <Wm>, <cond>
pub struct CSNEG_32_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSNEG_32_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// CSNEG  <Xd>, <Xn>, <Xm>, <cond>
pub struct CSNEG_64_condsel {
    pub rm: RegisterIndex,
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl CSNEG_64_condsel {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010100),
            i5(self.rm.into()),
            i4(self.cond as u32),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// DC  <dc_op>, <Xt>
pub struct DC_SYS_CR_systeminstrs {
    pub op1: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl DC_SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(self.op1),
            i4(0b0111),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// DCPS1  {#<imm>}
pub struct DCPS1_DC_exception {
    pub imm16: u32,
}
impl DCPS1_DC_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b101),
            i16(self.imm16),
            i3(0b000),
            i2(0b01)
        )
    }
}

// DCPS2  {#<imm>}
pub struct DCPS2_DC_exception {
    pub imm16: u32,
}
impl DCPS2_DC_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b101),
            i16(self.imm16),
            i3(0b000),
            i2(0b10)
        )
    }
}

// DCPS3  {#<imm>}
pub struct DCPS3_DC_exception {
    pub imm16: u32,
}
impl DCPS3_DC_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b101),
            i16(self.imm16),
            i3(0b000),
            i2(0b11)
        )
    }
}

// DGH
// ARMv8.4, FEAT_DGH
pub struct DGH_HI_hints {
}
impl DGH_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b110),
            i5(0b11111)
        )
    }
}

// DMB  <option>|#<imm>
pub struct DMB_BO_barriers {
    pub crm: u32,
}
impl DMB_BO_barriers {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0011),
            i4(self.crm),
            i1(0b1),
            i2(0b01),
            i5(0b11111)
        )
    }
}

// DRPS
pub struct DRPS_64E_branch_reg {
}
impl DRPS_64E_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i4(0b0101),
            i5(0b11111),
            i6(0b000000),
            i5(0b11111),
            i5(0b00000)
        )
    }
}

// DSB  <option>|#<imm>
pub struct DSB_BO_barriers {
    pub crm: u32,
}
impl DSB_BO_barriers {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0011),
            i4(self.crm),
            i1(0b1),
            i2(0b00),
            i5(0b11111)
        )
    }
}

// DVP  RCTX, <Xt>
// ARMv8.5, FEAT_SPECRES
pub struct DVP_SYS_CR_systeminstrs {
    pub rt: RegisterIndex,
}
impl DVP_SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(0b011),
            i4(0b0111),
            i4(0b0011),
            i3(0b101),
            i5(self.rt.into())
        )
    }
}

// EON  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct EON_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EON_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// EON  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct EON_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EON_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// EOR  <Wd|WSP>, <Wn>, #<imm>
pub struct EOR_32_log_imm {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EOR_32_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100100),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// EOR  <Xd|SP>, <Xn>, #<imm>
pub struct EOR_64_log_imm {
    pub n: u32,
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EOR_64_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i6(0b100100),
            i1(self.n),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// EOR  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct EOR_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EOR_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// EOR  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct EOR_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EOR_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ERET
pub struct ERET_64E_branch_reg {
}
impl ERET_64E_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i3(0b100),
            i5(0b11111),
            i4(0b0000),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i5(0b00000)
        )
    }
}

// ERETAA
// ARMv8.3, FEAT_PAuth
pub struct ERETAA_64E_branch_reg {
}
impl ERETAA_64E_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i3(0b100),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i5(0b11111)
        )
    }
}

// ERETAB
// ARMv8.3, FEAT_PAuth
pub struct ERETAB_64E_branch_reg {
}
impl ERETAB_64E_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i3(0b100),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i5(0b11111),
            i5(0b11111)
        )
    }
}

// ESB
// ARMv8.2, FEAT_RAS
pub struct ESB_HI_hints {
}
impl ESB_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0010),
            i3(0b000),
            i5(0b11111)
        )
    }
}

// EXTR  <Wd>, <Wn>, <Wm>, #<lsb>
pub struct EXTR_32_extract {
    pub rm: RegisterIndex,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EXTR_32_extract {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100111),
            i1(0b0),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// EXTR  <Xd>, <Xn>, <Xm>, #<lsb>
pub struct EXTR_64_extract {
    pub rm: RegisterIndex,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl EXTR_64_extract {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100111),
            i1(0b1),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// HINT  #<imm>
pub struct HINT_HM_hints {
    pub crm: u32,
    pub op2: u32,
}
impl HINT_HM_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(self.crm),
            i3(self.op2),
            i5(0b11111)
        )
    }
}

// HLT  #<imm>
pub struct HLT_EX_exception {
    pub imm16: u32,
}
impl HLT_EX_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b010),
            i16(self.imm16),
            i3(0b000),
            i2(0b00)
        )
    }
}

// HVC  #<imm>
pub struct HVC_EX_exception {
    pub imm16: u32,
}
impl HVC_EX_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b000),
            i16(self.imm16),
            i3(0b000),
            i2(0b10)
        )
    }
}

// IC  <ic_op>{, <Xt>}
pub struct IC_SYS_CR_systeminstrs {
    pub op1: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl IC_SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(self.op1),
            i4(0b0111),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// ISB  {<option>|#<imm>}
pub struct ISB_BI_barriers {
    pub crm: u32,
}
impl ISB_BI_barriers {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0011),
            i4(self.crm),
            i1(0b1),
            i2(0b10),
            i5(0b11111)
        )
    }
}

// LDADD  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADD_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADD_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADD  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADD_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADD_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDADDLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDADDLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDADDLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPR  <Wt>, [<Xn|SP> {,#0}]
// ARMv8.3, FEAT_LRCPC
pub struct LDAPR_32L_memop {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPR_32L_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b11111),
            i1(0b1),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPR  <Xt>, [<Xn|SP> {,#0}]
// ARMv8.3, FEAT_LRCPC
pub struct LDAPR_64L_memop {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPR_64L_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b11111),
            i1(0b1),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPRB  <Wt>, [<Xn|SP> {,#0}]
// ARMv8.3, FEAT_LRCPC
pub struct LDAPRB_32L_memop {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPRB_32L_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b11111),
            i1(0b1),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPRH  <Wt>, [<Xn|SP> {,#0}]
// ARMv8.3, FEAT_LRCPC
pub struct LDAPRH_32L_memop {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPRH_32L_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(0b11111),
            i1(0b1),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPUR  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPUR_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPUR_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b011001),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPUR  <Xt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPUR_64_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPUR_64_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b011001),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPURB  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPURB_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPURB_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b011001),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPURH  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPURH_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPURH_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b011001),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPURSB  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPURSB_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPURSB_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b011001),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPURSB  <Xt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPURSB_64_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPURSB_64_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b011001),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPURSH  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPURSH_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPURSH_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b011001),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPURSH  <Xt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPURSH_64_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPURSH_64_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b011001),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAPURSW  <Xt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct LDAPURSW_64_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAPURSW_64_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b011001),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAR  <Wt>, [<Xn|SP>{,#0}]
pub struct LDAR_LR32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAR_LR32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAR  <Xt>, [<Xn|SP>{,#0}]
pub struct LDAR_LR64_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAR_LR64_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDARB  <Wt>, [<Xn|SP>{,#0}]
pub struct LDARB_LR32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDARB_LR32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDARH  <Wt>, [<Xn|SP>{,#0}]
pub struct LDARH_LR32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDARH_LR32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAXP  <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
pub struct LDAXP_LP32_ldstexclp {
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAXP_LP32_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b11111),
            i1(0b1),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAXP  <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
pub struct LDAXP_LP64_ldstexclp {
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAXP_LP64_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b11111),
            i1(0b1),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAXR  <Wt>, [<Xn|SP>{,#0}]
pub struct LDAXR_LR32_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAXR_LR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAXR  <Xt>, [<Xn|SP>{,#0}]
pub struct LDAXR_LR64_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAXR_LR64_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAXRB  <Wt>, [<Xn|SP>{,#0}]
pub struct LDAXRB_LR32_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAXRB_LR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDAXRH  <Wt>, [<Xn|SP>{,#0}]
pub struct LDAXRH_LR32_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAXRH_LR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLR  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLR_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLR_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLR  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLR_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLR_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDCLRLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDCLRLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDCLRLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEOR  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEOR_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEOR_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEOR  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEOR_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEOR_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDEORLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDEORLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDEORLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDLAR  <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct LDLAR_LR32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDLAR_LR32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDLAR  <Xt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct LDLAR_LR64_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDLAR_LR64_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDLARB  <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct LDLARB_LR32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDLARB_LR32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDLARH  <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct LDLARH_LR32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDLARH_LR32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDNP  <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
pub struct LDNP_32_ldstnapair_offs {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDNP_32_ldstnapair_offs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b000),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDNP  <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
pub struct LDNP_64_ldstnapair_offs {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDNP_64_ldstnapair_offs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b000),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDP  <Wt1>, <Wt2>, [<Xn|SP>], #<imm>
pub struct LDP_32_ldstpair_post {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDP_32_ldstpair_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b001),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDP  <Xt1>, <Xt2>, [<Xn|SP>], #<imm>
pub struct LDP_64_ldstpair_post {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDP_64_ldstpair_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b001),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDP  <Wt1>, <Wt2>, [<Xn|SP>, #<imm>]!
pub struct LDP_32_ldstpair_pre {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDP_32_ldstpair_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b011),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDP  <Xt1>, <Xt2>, [<Xn|SP>, #<imm>]!
pub struct LDP_64_ldstpair_pre {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDP_64_ldstpair_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b011),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDP  <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
pub struct LDP_32_ldstpair_off {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDP_32_ldstpair_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b010),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDP  <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
pub struct LDP_64_ldstpair_off {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDP_64_ldstpair_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b010),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDPSW  <Xt1>, <Xt2>, [<Xn|SP>], #<imm>
pub struct LDPSW_64_ldstpair_post {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDPSW_64_ldstpair_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b101),
            i1(0b0),
            i3(0b001),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDPSW  <Xt1>, <Xt2>, [<Xn|SP>, #<imm>]!
pub struct LDPSW_64_ldstpair_pre {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDPSW_64_ldstpair_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b101),
            i1(0b0),
            i3(0b011),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDPSW  <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
pub struct LDPSW_64_ldstpair_off {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDPSW_64_ldstpair_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b101),
            i1(0b0),
            i3(0b010),
            i1(0b1),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Wt>, [<Xn|SP>], #<simm>
pub struct LDR_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Xt>, [<Xn|SP>], #<simm>
pub struct LDR_64_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_64_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Wt>, [<Xn|SP>, #<simm>]!
pub struct LDR_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Xt>, [<Xn|SP>, #<simm>]!
pub struct LDR_64_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_64_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct LDR_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b01),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Xt>, [<Xn|SP>{, #<pimm>}]
pub struct LDR_64_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_64_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b01),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Wt>, <label>
pub struct LDR_32_loadlit {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl LDR_32_loadlit {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b011),
            i1(0b0),
            i2(0b00),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// LDR  <Xt>, <label>
pub struct LDR_64_loadlit {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl LDR_64_loadlit {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b011),
            i1(0b0),
            i2(0b00),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// LDR  <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct LDR_32_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_32_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDR  <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct LDR_64_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDR_64_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRAA  <Xt>, [<Xn|SP>{, #<simm>}]
// ARMv8.3, FEAT_PAuth
pub struct LDRAA_64_ldst_pac {
    pub s: u32,
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRAA_64_ldst_pac {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(self.s),
            i1(0b1),
            i9(self.imm9),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRAA  <Xt>, [<Xn|SP>{, #<simm>}]!
// ARMv8.3, FEAT_PAuth
pub struct LDRAA_64W_ldst_pac {
    pub s: u32,
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRAA_64W_ldst_pac {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(self.s),
            i1(0b1),
            i9(self.imm9),
            i1(0b1),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRAB  <Xt>, [<Xn|SP>{, #<simm>}]
// ARMv8.3, FEAT_PAuth
pub struct LDRAB_64_ldst_pac {
    pub s: u32,
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRAB_64_ldst_pac {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(self.s),
            i1(0b1),
            i9(self.imm9),
            i1(0b0),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRAB  <Xt>, [<Xn|SP>{, #<simm>}]!
// ARMv8.3, FEAT_PAuth
pub struct LDRAB_64W_ldst_pac {
    pub s: u32,
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRAB_64W_ldst_pac {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(self.s),
            i1(0b1),
            i9(self.imm9),
            i1(0b1),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRB  <Wt>, [<Xn|SP>], #<simm>
pub struct LDRB_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRB_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRB  <Wt>, [<Xn|SP>, #<simm>]!
pub struct LDRB_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRB_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRB  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct LDRB_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRB_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b01),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRB  <Wt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
pub struct LDRB_32B_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRB_32B_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRB  <Wt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
pub struct LDRB_32BL_ldst_regoff {
    pub rm: RegisterIndex,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRB_32BL_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b1),
            i5(self.rm.into()),
            i3(0b011),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRH  <Wt>, [<Xn|SP>], #<simm>
pub struct LDRH_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRH_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRH  <Wt>, [<Xn|SP>, #<simm>]!
pub struct LDRH_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRH_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRH  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct LDRH_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRH_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b01),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRH  <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct LDRH_32_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRH_32_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Wt>, [<Xn|SP>], #<simm>
pub struct LDRSB_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Xt>, [<Xn|SP>], #<simm>
pub struct LDRSB_64_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_64_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Wt>, [<Xn|SP>, #<simm>]!
pub struct LDRSB_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Xt>, [<Xn|SP>, #<simm>]!
pub struct LDRSB_64_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_64_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct LDRSB_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b11),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Xt>, [<Xn|SP>{, #<pimm>}]
pub struct LDRSB_64_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_64_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b10),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Wt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
pub struct LDRSB_32B_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_32B_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Wt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
pub struct LDRSB_32BL_ldst_regoff {
    pub rm: RegisterIndex,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_32BL_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b1),
            i5(self.rm.into()),
            i3(0b011),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Xt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
pub struct LDRSB_64B_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_64B_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSB  <Xt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
pub struct LDRSB_64BL_ldst_regoff {
    pub rm: RegisterIndex,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSB_64BL_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b1),
            i5(self.rm.into()),
            i3(0b011),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Wt>, [<Xn|SP>], #<simm>
pub struct LDRSH_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Xt>, [<Xn|SP>], #<simm>
pub struct LDRSH_64_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_64_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Wt>, [<Xn|SP>, #<simm>]!
pub struct LDRSH_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Xt>, [<Xn|SP>, #<simm>]!
pub struct LDRSH_64_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_64_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct LDRSH_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b11),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Xt>, [<Xn|SP>{, #<pimm>}]
pub struct LDRSH_64_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_64_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b10),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct LDRSH_32_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_32_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSH  <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct LDRSH_64_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSH_64_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSW  <Xt>, [<Xn|SP>], #<simm>
pub struct LDRSW_64_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSW_64_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSW  <Xt>, [<Xn|SP>, #<simm>]!
pub struct LDRSW_64_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSW_64_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSW  <Xt>, [<Xn|SP>{, #<pimm>}]
pub struct LDRSW_64_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSW_64_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b10),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDRSW  <Xt>, <label>
pub struct LDRSW_64_loadlit {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl LDRSW_64_loadlit {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b011),
            i1(0b0),
            i2(0b00),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// LDRSW  <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct LDRSW_64_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDRSW_64_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSET  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSET_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSET_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSET  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSET_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSET_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSETLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSETLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSETLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAX  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAX_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAX_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAX  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAX_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAX_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMAXLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMAXLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMAXLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMIN  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMIN_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMIN_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMIN  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMIN_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMIN_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDSMINLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDSMINLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDSMINLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTR  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDTR_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTR_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTR  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDTR_64_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTR_64_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTRB  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDTRB_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTRB_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTRH  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDTRH_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTRH_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTRSB  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDTRSB_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTRSB_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTRSB  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDTRSB_64_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTRSB_64_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTRSH  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDTRSH_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTRSH_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTRSH  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDTRSH_64_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTRSH_64_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDTRSW  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDTRSW_64_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDTRSW_64_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAX  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAX_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAX_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAX  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAX_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAX_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMAXLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMAXLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMAXLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMIN  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMIN_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMIN_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMIN  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMIN_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMIN_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUMINLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct LDUMINLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUMINLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUR  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDUR_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUR_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDUR  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDUR_64_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDUR_64_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDURB  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDURB_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDURB_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDURH  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDURH_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDURH_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b01),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDURSB  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDURSB_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDURSB_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDURSB  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDURSB_64_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDURSB_64_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDURSH  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct LDURSH_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDURSH_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b11),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDURSH  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDURSH_64_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDURSH_64_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDURSW  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct LDURSW_64_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDURSW_64_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDXP  <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
pub struct LDXP_LP32_ldstexclp {
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDXP_LP32_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b11111),
            i1(0b0),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDXP  <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
pub struct LDXP_LP64_ldstexclp {
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDXP_LP64_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b11111),
            i1(0b0),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDXR  <Wt>, [<Xn|SP>{,#0}]
pub struct LDXR_LR32_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDXR_LR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDXR  <Xt>, [<Xn|SP>{,#0}]
pub struct LDXR_LR64_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDXR_LR64_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDXRB  <Wt>, [<Xn|SP>{,#0}]
pub struct LDXRB_LR32_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDXRB_LR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LDXRH  <Wt>, [<Xn|SP>{,#0}]
pub struct LDXRH_LR32_ldstexclr {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDXRH_LR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// LSL  <Wd>, <Wn>, <Wm>
pub struct LSL_LSLV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSL_LSLV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSL  <Xd>, <Xn>, <Xm>
pub struct LSL_LSLV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSL_LSLV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSL  <Wd>, <Wn>, #<shift>
pub struct LSL_UBFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSL_UBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSL  <Xd>, <Xn>, #<shift>
pub struct LSL_UBFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSL_UBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSLV  <Wd>, <Wn>, <Wm>
pub struct LSLV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSLV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSLV  <Xd>, <Xn>, <Xm>
pub struct LSLV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSLV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSR  <Wd>, <Wn>, <Wm>
pub struct LSR_LSRV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSR_LSRV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSR  <Xd>, <Xn>, <Xm>
pub struct LSR_LSRV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSR_LSRV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSR  <Wd>, <Wn>, #<shift>
pub struct LSR_UBFM_32M_bitfield {
    pub immr: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSR_UBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(0b011111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSR  <Xd>, <Xn>, #<shift>
pub struct LSR_UBFM_64M_bitfield {
    pub immr: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSR_UBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(0b111111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSRV  <Wd>, <Wn>, <Wm>
pub struct LSRV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSRV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// LSRV  <Xd>, <Xn>, <Xm>
pub struct LSRV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl LSRV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MADD  <Wd>, <Wn>, <Wm>, <Wa>
pub struct MADD_32A_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MADD_32A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b0),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MADD  <Xd>, <Xn>, <Xm>, <Xa>
pub struct MADD_64A_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MADD_64A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b0),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MNEG  <Wd>, <Wn>, <Wm>
pub struct MNEG_MSUB_32A_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MNEG_MSUB_32A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MNEG  <Xd>, <Xn>, <Xm>
pub struct MNEG_MSUB_64A_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MNEG_MSUB_64A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MOV  <Wd|WSP>, <Wn|WSP>
pub struct MOV_ADD_32_addsub_imm {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MOV_ADD_32_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i6(0b100010),
            i1(0b0),
            i12(0b000000000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MOV  <Xd|SP>, <Xn|SP>
pub struct MOV_ADD_64_addsub_imm {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MOV_ADD_64_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i6(0b100010),
            i1(0b0),
            i12(0b000000000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MOV  <Wd|WSP>, #<imm>
pub struct MOV_ORR_32_log_imm {
    pub immr: u32,
    pub imms: u32,
    pub rd: RegisterIndex,
}
impl MOV_ORR_32_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i6(0b100100),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// MOV  <Xd|SP>, #<imm>
pub struct MOV_ORR_64_log_imm {
    pub n: u32,
    pub immr: u32,
    pub imms: u32,
    pub rd: RegisterIndex,
}
impl MOV_ORR_64_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i6(0b100100),
            i1(self.n),
            i6(self.immr),
            i6(self.imms),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// MOV  <Wd>, <Wm>
pub struct MOV_ORR_32_log_shift {
    pub rm: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MOV_ORR_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i5(0b01010),
            i2(0b00),
            i1(0b0),
            i5(self.rm.into()),
            i6(0b000000),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// MOV  <Xd>, <Xm>
pub struct MOV_ORR_64_log_shift {
    pub rm: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MOV_ORR_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i5(0b01010),
            i2(0b00),
            i1(0b0),
            i5(self.rm.into()),
            i6(0b000000),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// MOVK  <Wd>, #<imm>{, LSL #<shift>}
pub struct MOVK_32_movewide {
    pub hw: u32,
    pub imm16: u32,
    pub rd: RegisterIndex,
}
impl MOVK_32_movewide {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b11),
            i6(0b100101),
            i2(self.hw),
            i16(self.imm16),
            i5(self.rd.into())
        )
    }
}

// MOVK  <Xd>, #<imm>{, LSL #<shift>}
pub struct MOVK_64_movewide {
    pub hw: u32,
    pub imm16: u32,
    pub rd: RegisterIndex,
}
impl MOVK_64_movewide {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b11),
            i6(0b100101),
            i2(self.hw),
            i16(self.imm16),
            i5(self.rd.into())
        )
    }
}

// MOVN  <Wd>, #<imm>{, LSL #<shift>}
pub struct MOVN_32_movewide {
    pub hw: u32,
    pub imm16: u32,
    pub rd: RegisterIndex,
}
impl MOVN_32_movewide {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100101),
            i2(self.hw),
            i16(self.imm16),
            i5(self.rd.into())
        )
    }
}

// MOVN  <Xd>, #<imm>{, LSL #<shift>}
pub struct MOVN_64_movewide {
    pub hw: u32,
    pub imm16: u32,
    pub rd: RegisterIndex,
}
impl MOVN_64_movewide {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100101),
            i2(self.hw),
            i16(self.imm16),
            i5(self.rd.into())
        )
    }
}

// MOVZ  <Wd>, #<imm>{, LSL #<shift>}
pub struct MOVZ_32_movewide {
    pub hw: u32,
    pub imm16: u32,
    pub rd: RegisterIndex,
}
impl MOVZ_32_movewide {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100101),
            i2(self.hw),
            i16(self.imm16),
            i5(self.rd.into())
        )
    }
}

// MOVZ  <Xd>, #<imm>{, LSL #<shift>}
pub struct MOVZ_64_movewide {
    pub hw: u32,
    pub imm16: u32,
    pub rd: RegisterIndex,
}
impl MOVZ_64_movewide {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i6(0b100101),
            i2(self.hw),
            i16(self.imm16),
            i5(self.rd.into())
        )
    }
}

// MRS  <Xt>, (<systemreg>|S<op0>_<op1>_<Cn>_<Cm>_<op2>)
pub struct MRS_RS_systemmove {
    pub o0: u32,
    pub op1: u32,
    pub crn: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl MRS_RS_systemmove {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b1),
            i1(0b1),
            i1(self.o0),
            i3(self.op1),
            i4(self.crn),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// MSR  <pstatefield>, #<imm>
pub struct MSR_SI_pstate {
    pub op1: u32,
    pub crm: u32,
    pub op2: u32,
}
impl MSR_SI_pstate {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(self.op1),
            i4(0b0100),
            i4(self.crm),
            i3(self.op2),
            i5(0b11111)
        )
    }
}

// MSR  (<systemreg>|S<op0>_<op1>_<Cn>_<Cm>_<op2>), <Xt>
pub struct MSR_SR_systemmove {
    pub o0: u32,
    pub op1: u32,
    pub crn: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl MSR_SR_systemmove {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i1(0b1),
            i1(self.o0),
            i3(self.op1),
            i4(self.crn),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// MSUB  <Wd>, <Wn>, <Wm>, <Wa>
pub struct MSUB_32A_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MSUB_32A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b1),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MSUB  <Xd>, <Xn>, <Xm>, <Xa>
pub struct MSUB_64A_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MSUB_64A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b1),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MUL  <Wd>, <Wn>, <Wm>
pub struct MUL_MADD_32A_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MUL_MADD_32A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MUL  <Xd>, <Xn>, <Xm>
pub struct MUL_MADD_64A_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl MUL_MADD_64A_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i3(0b000),
            i5(self.rm.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// MVN  <Wd>, <Wm>{, <shift> #<amount>}
pub struct MVN_ORN_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rd: RegisterIndex,
}
impl MVN_ORN_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// MVN  <Xd>, <Xm>{, <shift> #<amount>}
pub struct MVN_ORN_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rd: RegisterIndex,
}
impl MVN_ORN_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NEG  <Wd>, <Wm>{, <shift> #<amount>}
pub struct NEG_SUB_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rd: RegisterIndex,
}
impl NEG_SUB_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NEG  <Xd>, <Xm>{, <shift> #<amount>}
pub struct NEG_SUB_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rd: RegisterIndex,
}
impl NEG_SUB_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NEGS  <Wd>, <Wm>{, <shift> #<amount>}
pub struct NEGS_SUBS_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rd: RegisterIndex,
}
impl NEGS_SUBS_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NEGS  <Xd>, <Xm>{, <shift> #<amount>}
pub struct NEGS_SUBS_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rd: RegisterIndex,
}
impl NEGS_SUBS_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NGC  <Wd>, <Wm>
pub struct NGC_SBC_32_addsub_carry {
    pub rm: RegisterIndex,
    pub rd: RegisterIndex,
}
impl NGC_SBC_32_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NGC  <Xd>, <Xm>
pub struct NGC_SBC_64_addsub_carry {
    pub rm: RegisterIndex,
    pub rd: RegisterIndex,
}
impl NGC_SBC_64_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NGCS  <Wd>, <Wm>
pub struct NGCS_SBCS_32_addsub_carry {
    pub rm: RegisterIndex,
    pub rd: RegisterIndex,
}
impl NGCS_SBCS_32_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NGCS  <Xd>, <Xm>
pub struct NGCS_SBCS_64_addsub_carry {
    pub rm: RegisterIndex,
    pub rd: RegisterIndex,
}
impl NGCS_SBCS_64_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// NOP
pub struct NOP_HI_hints {
}
impl NOP_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b000),
            i5(0b11111)
        )
    }
}

// ORN  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct ORN_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ORN_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ORN  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct ORN_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ORN_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i5(0b01010),
            i2(self.shift),
            i1(0b1),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ORR  <Wd|WSP>, <Wn>, #<imm>
pub struct ORR_32_log_imm {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ORR_32_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i6(0b100100),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ORR  <Xd|SP>, <Xn>, #<imm>
pub struct ORR_64_log_imm {
    pub n: u32,
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ORR_64_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i6(0b100100),
            i1(self.n),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ORR  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct ORR_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ORR_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b01),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ORR  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct ORR_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ORR_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b01),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// PACDA  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct PACDA_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl PACDA_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b010),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// PACDZA  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct PACDZA_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl PACDZA_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b010),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// PACDB  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct PACDB_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl PACDB_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b011),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// PACDZB  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct PACDZB_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl PACDZB_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b011),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// PACGA  <Xd>, <Xn>, <Xm|SP>
// ARMv8.3, FEAT_PAuth
pub struct PACGA_64P_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl PACGA_64P_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i6(0b001100),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// PACIA  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct PACIA_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl PACIA_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// PACIZA  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct PACIZA_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl PACIZA_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b000),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// PACIA1716
// ARMv8.3, FEAT_PAuth
pub struct PACIA1716_HI_hints {
}
impl PACIA1716_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0001),
            i3(0b000),
            i5(0b11111)
        )
    }
}

// PACIASP
// ARMv8.3, FEAT_PAuth
pub struct PACIASP_HI_hints {
}
impl PACIASP_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b001),
            i5(0b11111)
        )
    }
}

// PACIAZ
// ARMv8.3, FEAT_PAuth
pub struct PACIAZ_HI_hints {
}
impl PACIAZ_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b000),
            i5(0b11111)
        )
    }
}

// PACIB  <Xd>, <Xn|SP>
// ARMv8.3, FEAT_PAuth
pub struct PACIB_64P_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl PACIB_64P_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i3(0b001),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// PACIZB  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct PACIZB_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl PACIZB_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i3(0b001),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// PACIB1716
// ARMv8.3, FEAT_PAuth
pub struct PACIB1716_HI_hints {
}
impl PACIB1716_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0001),
            i3(0b010),
            i5(0b11111)
        )
    }
}

// PACIBSP
// ARMv8.3, FEAT_PAuth
pub struct PACIBSP_HI_hints {
}
impl PACIBSP_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b011),
            i5(0b11111)
        )
    }
}

// PACIBZ
// ARMv8.3, FEAT_PAuth
pub struct PACIBZ_HI_hints {
}
impl PACIBZ_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0011),
            i3(0b010),
            i5(0b11111)
        )
    }
}

// PRFM  (<prfop>|#<imm5>), [<Xn|SP>{, #<pimm>}]
pub struct PRFM_P_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl PRFM_P_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b10),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// PRFM  (<prfop>|#<imm5>), <label>
pub struct PRFM_P_loadlit {
    pub imm19: u32,
    pub rt: RegisterIndex,
}
impl PRFM_P_loadlit {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b011),
            i1(0b0),
            i2(0b00),
            i19(self.imm19),
            i5(self.rt.into())
        )
    }
}

// PRFM  (<prfop>|#<imm5>), [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct PRFM_P_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl PRFM_P_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// PRFUM (<prfop>|#<imm5>), [<Xn|SP>{, #<simm>}]
pub struct PRFUM_P_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl PRFUM_P_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b10),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// PSSBB
pub struct PSSBB_DSB_BO_barriers {
}
impl PSSBB_DSB_BO_barriers {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0011),
            i4(0b0100),
            i1(0b1),
            i2(0b00),
            i5(0b11111)
        )
    }
}

// RBIT  <Wd>, <Wn>
pub struct RBIT_32_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl RBIT_32_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// RBIT  <Xd>, <Xn>
pub struct RBIT_64_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl RBIT_64_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// RET  {<Xn>}
pub struct RET_64R_branch_reg {
    pub rn: RegisterIndex,
}
impl RET_64R_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b10),
            i5(0b11111),
            i4(0b0000),
            i1(0b0),
            i1(0b0),
            i5(self.rn.into()),
            i5(0b00000)
        )
    }
}

// RETAA
// ARMv8.3, FEAT_PAuth
pub struct RETAA_64E_branch_reg {
}
impl RETAA_64E_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b10),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b0),
            i5(0b11111),
            i5(0b11111)
        )
    }
}

// RETAB
// ARMv8.3, FEAT_PAuth
pub struct RETAB_64E_branch_reg {
}
impl RETAB_64E_branch_reg {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1101011),
            i1(0b0),
            i1(0b0),
            i2(0b10),
            i5(0b11111),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i5(0b11111),
            i5(0b11111)
        )
    }
}

// REV  <Wd>, <Wn>
pub struct REV_32_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl REV_32_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// REV  <Xd>, <Xn>
pub struct REV_64_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl REV_64_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// REV16  <Wd>, <Wn>
pub struct REV16_32_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl REV16_32_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// REV16  <Xd>, <Xn>
pub struct REV16_64_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl REV16_64_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// REV32  <Xd>, <Xn>
pub struct REV32_64_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl REV32_64_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// REV64  <Xd>, <Xn>
pub struct REV64_REV_64_dp_1src {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl REV64_REV_64_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00000),
            i4(0b0000),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// RMIF  <Xn>, #<shift>, #<mask>
// ARMv8.4, FEAT_FlagM
pub struct RMIF_only_rmif {
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub mask: u32,
}
impl RMIF_only_rmif {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i8(0b11010000),
            i6(self.imm6),
            i5(0b00001),
            i5(self.rn.into()),
            i1(0b0),
            i4(self.mask)
        )
    }
}

// ROR  <Wd>, <Ws>, #<shift>
pub struct ROR_EXTR_32_extract {
    pub rm: RegisterIndex,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ROR_EXTR_32_extract {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100111),
            i1(0b0),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ROR  <Xd>, <Xs>, #<shift>
pub struct ROR_EXTR_64_extract {
    pub rm: RegisterIndex,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ROR_EXTR_64_extract {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100111),
            i1(0b1),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ROR  <Wd>, <Wn>, <Wm>
pub struct ROR_RORV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ROR_RORV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// ROR  <Xd>, <Xn>, <Xm>
pub struct ROR_RORV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl ROR_RORV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// RORV  <Wd>, <Wn>, <Wm>
pub struct RORV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl RORV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// RORV  <Xd>, <Xn>, <Xm>
pub struct RORV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl RORV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i4(0b0010),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SB
// ARMv8.5, FEAT_SB
pub struct SB_only_barriers {
}
impl SB_only_barriers {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0011),
            i4(0b0000),
            i1(0b1),
            i2(0b11),
            i5(0b11111)
        )
    }
}

// SBC  <Wd>, <Wn>, <Wm>
pub struct SBC_32_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBC_32_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBC  <Xd>, <Xn>, <Xm>
pub struct SBC_64_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBC_64_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBCS  <Wd>, <Wn>, <Wm>
pub struct SBCS_32_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBCS_32_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBCS  <Xd>, <Xn>, <Xm>
pub struct SBCS_64_addsub_carry {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBCS_64_addsub_carry {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i8(0b11010000),
            i5(self.rm.into()),
            i6(0b000000),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBFIZ  <Wd>, <Wn>, #<lsb>, #<width>
pub struct SBFIZ_SBFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBFIZ_SBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBFIZ  <Xd>, <Xn>, #<lsb>, #<width>
pub struct SBFIZ_SBFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBFIZ_SBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBFM  <Wd>, <Wn>, #<immr>, #<imms>
pub struct SBFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBFM  <Xd>, <Xn>, #<immr>, #<imms>
pub struct SBFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBFX  <Wd>, <Wn>, #<lsb>, #<width>
pub struct SBFX_SBFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBFX_SBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SBFX  <Xd>, <Xn>, #<lsb>, #<width>
pub struct SBFX_SBFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SBFX_SBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SDIV  <Wd>, <Wn>, <Wm>
pub struct SDIV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SDIV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i5(0b00001),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SDIV  <Xd>, <Xn>, <Xm>
pub struct SDIV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SDIV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i5(0b00001),
            i1(0b1),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SETF8  <Wn>
// ARMv8.4, FEAT_FlagM
pub struct SETF8_only_setf {
    pub rn: RegisterIndex,
}
impl SETF8_only_setf {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i8(0b11010000),
            i6(0b000000),
            i1(0b0),
            i4(0b0010),
            i5(self.rn.into()),
            i1(0b0),
            i4(0b1101)
        )
    }
}

// SETF16  <Wn>
// ARMv8.4, FEAT_FlagM
pub struct SETF16_only_setf {
    pub rn: RegisterIndex,
}
impl SETF16_only_setf {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i8(0b11010000),
            i6(0b000000),
            i1(0b1),
            i4(0b0010),
            i5(self.rn.into()),
            i1(0b0),
            i4(0b1101)
        )
    }
}

// SEV
pub struct SEV_HI_hints {
}
impl SEV_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b100),
            i5(0b11111)
        )
    }
}

// SEVL
pub struct SEVL_HI_hints {
}
impl SEVL_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b101),
            i5(0b11111)
        )
    }
}

// SMADDL  <Xd>, <Wn>, <Wm>, <Xa>
pub struct SMADDL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SMADDL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b0),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b0),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SMC  #<imm>
pub struct SMC_EX_exception {
    pub imm16: u32,
}
impl SMC_EX_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b000),
            i16(self.imm16),
            i3(0b000),
            i2(0b11)
        )
    }
}

// SMNEGL  <Xd>, <Wn>, <Wm>
pub struct SMNEGL_SMSUBL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SMNEGL_SMSUBL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b0),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SMSUBL  <Xd>, <Wn>, <Wm>, <Xa>
pub struct SMSUBL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SMSUBL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b0),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b1),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SMULH  <Xd>, <Xn>, <Xm>
pub struct SMULH_64_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SMULH_64_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b0),
            i2(0b10),
            i5(self.rm.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SMULL  <Xd>, <Wn>, <Wm>
pub struct SMULL_SMADDL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SMULL_SMADDL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b0),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SSBB
pub struct SSBB_DSB_BO_barriers {
}
impl SSBB_DSB_BO_barriers {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0011),
            i4(0b0000),
            i1(0b1),
            i2(0b00),
            i5(0b11111)
        )
    }
}

// STADD  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADD_LDADD_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADD_LDADD_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STADDL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADDL_LDADDL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADDL_LDADDL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STADD  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADD_LDADD_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADD_LDADD_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STADDL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADDL_LDADDL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADDL_LDADDL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STADDB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADDB_LDADDB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADDB_LDADDB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STADDLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADDLB_LDADDLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADDLB_LDADDLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STADDH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADDH_LDADDH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADDH_LDADDH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STADDLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STADDLH_LDADDLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STADDLH_LDADDLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLR  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLR_LDCLR_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLR_LDCLR_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLRL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLRL_LDCLRL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLRL_LDCLRL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLR  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLR_LDCLR_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLR_LDCLR_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLRL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLRL_LDCLRL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLRL_LDCLRL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLRB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLRB_LDCLRB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLRB_LDCLRB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLRLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLRLB_LDCLRLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLRLB_LDCLRLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLRH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLRH_LDCLRH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLRH_LDCLRH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STCLRLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STCLRLH_LDCLRLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STCLRLH_LDCLRLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b001),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEOR  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEOR_LDEOR_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEOR_LDEOR_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEORL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEORL_LDEORL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEORL_LDEORL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEOR  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEOR_LDEOR_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEOR_LDEOR_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEORL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEORL_LDEORL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEORL_LDEORL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEORB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEORB_LDEORB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEORB_LDEORB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEORLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEORLB_LDEORLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEORLB_LDEORLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEORH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEORH_LDEORH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEORH_LDEORH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STEORLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STEORLH_LDEORLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STEORLH_LDEORLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b010),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STLLR  <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct STLLR_SL32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLLR_SL32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLLR  <Xt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct STLLR_SL64_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLLR_SL64_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLLRB  <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct STLLRB_SL32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLLRB_SL32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLLRH  <Wt>, [<Xn|SP>{,#0}]
// ARMv8.1, FEAT_LOR
pub struct STLLRH_SL32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLLRH_SL32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLR  <Wt>, [<Xn|SP>{,#0}]
pub struct STLR_SL32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLR_SL32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLR  <Xt>, [<Xn|SP>{,#0}]
pub struct STLR_SL64_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLR_SL64_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLRB  <Wt>, [<Xn|SP>{,#0}]
pub struct STLRB_SL32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLRB_SL32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLRH  <Wt>, [<Xn|SP>{,#0}]
pub struct STLRH_SL32_ldstord {
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLRH_SL32_ldstord {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i5(0b11111),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLUR  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct STLUR_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLUR_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b011001),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLUR  <Xt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct STLUR_64_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLUR_64_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b011001),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLURB  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct STLURB_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLURB_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b011001),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLURH  <Wt>, [<Xn|SP>{, #<simm>}]
// ARMv8.4, FEAT_LRCPC2
pub struct STLURH_32_ldapstl_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLURH_32_ldapstl_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b011001),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLXP  <Ws>, <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
pub struct STLXP_SP32_ldstexclp {
    pub rs: RegisterIndex,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLXP_SP32_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLXP  <Ws>, <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
pub struct STLXP_SP64_ldstexclp {
    pub rs: RegisterIndex,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLXP_SP64_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLXR  <Ws>, <Wt>, [<Xn|SP>{,#0}]
pub struct STLXR_SR32_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLXR_SR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLXR  <Ws>, <Xt>, [<Xn|SP>{,#0}]
pub struct STLXR_SR64_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLXR_SR64_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLXRB  <Ws>, <Wt>, [<Xn|SP>{,#0}]
pub struct STLXRB_SR32_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLXRB_SR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STLXRH  <Ws>, <Wt>, [<Xn|SP>{,#0}]
pub struct STLXRH_SR32_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLXRH_SR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STNP  <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
pub struct STNP_32_ldstnapair_offs {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STNP_32_ldstnapair_offs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b000),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STNP  <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
pub struct STNP_64_ldstnapair_offs {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STNP_64_ldstnapair_offs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b000),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STP  <Wt1>, <Wt2>, [<Xn|SP>], #<imm>
pub struct STP_32_ldstpair_post {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STP_32_ldstpair_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b001),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STP  <Xt1>, <Xt2>, [<Xn|SP>], #<imm>
pub struct STP_64_ldstpair_post {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STP_64_ldstpair_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b001),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STP  <Wt1>, <Wt2>, [<Xn|SP>, #<imm>]!
pub struct STP_32_ldstpair_pre {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STP_32_ldstpair_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b011),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STP  <Xt1>, <Xt2>, [<Xn|SP>, #<imm>]!
pub struct STP_64_ldstpair_pre {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STP_64_ldstpair_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b011),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STP  <Wt1>, <Wt2>, [<Xn|SP>{, #<imm>}]
pub struct STP_32_ldstpair_off {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STP_32_ldstpair_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b101),
            i1(0b0),
            i3(0b010),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STP  <Xt1>, <Xt2>, [<Xn|SP>{, #<imm>}]
pub struct STP_64_ldstpair_off {
    pub imm7: u32,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STP_64_ldstpair_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b101),
            i1(0b0),
            i3(0b010),
            i1(0b0),
            i7(self.imm7),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Wt>, [<Xn|SP>], #<simm>
pub struct STR_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Xt>, [<Xn|SP>], #<simm>
pub struct STR_64_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_64_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Wt>, [<Xn|SP>, #<simm>]!
pub struct STR_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Xt>, [<Xn|SP>, #<simm>]!
pub struct STR_64_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_64_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct STR_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b00),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Xt>, [<Xn|SP>{, #<pimm>}]
pub struct STR_64_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_64_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b00),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct STR_32_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_32_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STR  <Xt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct STR_64_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STR_64_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRB  <Wt>, [<Xn|SP>], #<simm>
pub struct STRB_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRB_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRB  <Wt>, [<Xn|SP>, #<simm>]!
pub struct STRB_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRB_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRB  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct STRB_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRB_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b00),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRB  <Wt>, [<Xn|SP>, (<Wm>|<Xm>), <extend> {<amount>}]
pub struct STRB_32B_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRB_32B_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRB  <Wt>, [<Xn|SP>, <Xm>{, LSL <amount>}]
pub struct STRB_32BL_ldst_regoff {
    pub rm: RegisterIndex,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRB_32BL_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(0b011),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRH  <Wt>, [<Xn|SP>], #<simm>
pub struct STRH_32_ldst_immpost {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRH_32_ldst_immpost {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b01),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRH  <Wt>, [<Xn|SP>, #<simm>]!
pub struct STRH_32_ldst_immpre {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRH_32_ldst_immpre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b11),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRH  <Wt>, [<Xn|SP>{, #<pimm>}]
pub struct STRH_32_ldst_pos {
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRH_32_ldst_pos {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b01),
            i2(0b00),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STRH  <Wt>, [<Xn|SP>, (<Wm>|<Xm>){, <extend> {<amount>}}]
pub struct STRH_32_ldst_regoff {
    pub rm: RegisterIndex,
    pub option: u32,
    pub s: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STRH_32_ldst_regoff {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i1(self.s),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STSET  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSET_LDSET_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSET_LDSET_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSETL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSETL_LDSETL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSETL_LDSETL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSET  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSET_LDSET_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSET_LDSET_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSETL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSETL_LDSETL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSETL_LDSETL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSETB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSETB_LDSETB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSETB_LDSETB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSETLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSETLB_LDSETLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSETLB_LDSETLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSETH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSETH_LDSETH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSETH_LDSETH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSETLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSETLH_LDSETLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSETLH_LDSETLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b011),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAX  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAX_LDSMAX_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAX_LDSMAX_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAXL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAXL_LDSMAXL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAXL_LDSMAXL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAX  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAX_LDSMAX_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAX_LDSMAX_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAXL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAXL_LDSMAXL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAXL_LDSMAXL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAXB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAXB_LDSMAXB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAXB_LDSMAXB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAXLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAXLB_LDSMAXLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAXLB_LDSMAXLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAXH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAXH_LDSMAXH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAXH_LDSMAXH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMAXLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMAXLH_LDSMAXLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMAXLH_LDSMAXLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b100),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMIN  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMIN_LDSMIN_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMIN_LDSMIN_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMINL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMINL_LDSMINL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMINL_LDSMINL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMIN  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMIN_LDSMIN_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMIN_LDSMIN_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMINL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMINL_LDSMINL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMINL_LDSMINL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMINB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMINB_LDSMINB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMINB_LDSMINB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMINLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMINLB_LDSMINLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMINLB_LDSMINLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMINH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMINH_LDSMINH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMINH_LDSMINH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STSMINLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STSMINLH_LDSMINLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STSMINLH_LDSMINLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b101),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STTR  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct STTR_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STTR_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STTR  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct STTR_64_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STTR_64_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STTRB  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct STTRB_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STTRB_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STTRH  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct STTRH_32_ldst_unpriv {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STTRH_32_ldst_unpriv {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b10),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STUMAX  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAX_LDUMAX_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAX_LDUMAX_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMAXL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAXL_LDUMAXL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAXL_LDUMAXL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMAX  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAX_LDUMAX_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAX_LDUMAX_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMAXL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAXL_LDUMAXL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAXL_LDUMAXL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMAXB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAXB_LDUMAXB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAXB_LDUMAXB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMAXLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAXLB_LDUMAXLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAXLB_LDUMAXLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMAXH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAXH_LDUMAXH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAXH_LDUMAXH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMAXLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMAXLH_LDUMAXLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMAXLH_LDUMAXLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b110),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMIN  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMIN_LDUMIN_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMIN_LDUMIN_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMINL  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMINL_LDUMINL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMINL_LDUMINL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMIN  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMIN_LDUMIN_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMIN_LDUMIN_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMINL  <Xs>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMINL_LDUMINL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMINL_LDUMINL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMINB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMINB_LDUMINB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMINB_LDUMINB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMINLB  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMINLB_LDUMINLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMINLB_LDUMINLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMINH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMINH_LDUMINH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMINH_LDUMINH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUMINLH  <Ws>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct STUMINLH_LDUMINLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
}
impl STUMINLH_LDUMINLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i3(0b111),
            i2(0b00),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// STUR  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct STUR_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STUR_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STUR  <Xt>, [<Xn|SP>{, #<simm>}]
pub struct STUR_64_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STUR_64_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STURB  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct STURB_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STURB_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STURH  <Wt>, [<Xn|SP>{, #<simm>}]
pub struct STURH_32_ldst_unscaled {
    pub imm9: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STURH_32_ldst_unscaled {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i2(0b00),
            i1(0b0),
            i9(self.imm9),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STXP  <Ws>, <Wt1>, <Wt2>, [<Xn|SP>{,#0}]
pub struct STXP_SP32_ldstexclp {
    pub rs: RegisterIndex,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STXP_SP32_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STXP  <Ws>, <Xt1>, <Xt2>, [<Xn|SP>{,#0}]
pub struct STXP_SP64_ldstexclp {
    pub rs: RegisterIndex,
    pub rt2: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STXP_SP64_ldstexclp {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b0),
            i5(self.rt2.into()),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STXR  <Ws>, <Wt>, [<Xn|SP>{,#0}]
pub struct STXR_SR32_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STXR_SR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STXR  <Ws>, <Xt>, [<Xn|SP>{,#0}]
pub struct STXR_SR64_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STXR_SR64_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STXRB  <Ws>, <Wt>, [<Xn|SP>{,#0}]
pub struct STXRB_SR32_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STXRB_SR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// STXRH  <Ws>, <Wt>, [<Xn|SP>{,#0}]
pub struct STXRH_SR32_ldstexclr {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STXRH_SR32_ldstexclr {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i6(0b001000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.rs.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SUB  <Wd|WSP>, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
pub struct SUB_32_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUB_32_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUB  <Xd|SP>, <Xn|SP>, <R><m>{, <extend> {#<amount>}}
pub struct SUB_64_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUB_64_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUB  <Wd|WSP>, <Wn|WSP>, #<imm>{, <shift>}
pub struct SUB_32_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUB_32_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUB  <Xd|SP>, <Xn|SP>, #<imm>{, <shift>}
pub struct SUB_64_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUB_64_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUB  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct SUB_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUB_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUB  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct SUB_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUB_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUBS  <Wd>, <Wn|WSP>, <Wm>{, <extend> {#<amount>}}
pub struct SUBS_32S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUBS_32S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUBS  <Xd>, <Xn|SP>, <R><m>{, <extend> {#<amount>}}
pub struct SUBS_64S_addsub_ext {
    pub rm: RegisterIndex,
    pub option: u32,
    pub imm3: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUBS_64S_addsub_ext {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(0b00),
            i1(0b1),
            i5(self.rm.into()),
            i3(self.option),
            i3(self.imm3),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUBS  <Wd>, <Wn|WSP>, #<imm>{, <shift>}
pub struct SUBS_32S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUBS_32S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUBS  <Xd>, <Xn|SP>, #<imm>{, <shift>}
pub struct SUBS_64S_addsub_imm {
    pub sh: u32,
    pub imm12: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUBS_64S_addsub_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i6(0b100010),
            i1(self.sh),
            i12(self.imm12),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUBS  <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
pub struct SUBS_32_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUBS_32_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SUBS  <Xd>, <Xn>, <Xm>{, <shift> #<amount>}
pub struct SUBS_64_addsub_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SUBS_64_addsub_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b01011),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SVC  #<imm>
pub struct SVC_EX_exception {
    pub imm16: u32,
}
impl SVC_EX_exception {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i8(0b11010100),
            i3(0b000),
            i16(self.imm16),
            i3(0b000),
            i2(0b01)
        )
    }
}

// SWP  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWP_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWP_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPA  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPA_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPA_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPAL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPAL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPAL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPL  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPL_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPL_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b10),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWP  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWP_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWP_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPA  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPA_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPA_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPAL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPAL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPAL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPL  <Xs>, <Xt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPL_64_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPL_64_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b11),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPAB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPAB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPAB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPALB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPALB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPALB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPLB  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPLB_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPLB_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b00),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPAH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPAH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPAH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPALH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPALH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPALH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SWPLH  <Ws>, <Wt>, [<Xn|SP>]
// ARMv8.1, FEAT_LSE
pub struct SWPLH_32_memop {
    pub rs: RegisterIndex,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl SWPLH_32_memop {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i2(0b01),
            i3(0b111),
            i1(0b0),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i5(self.rs.into()),
            i1(0b1),
            i3(0b000),
            i2(0b00),
            i5(self.rn.into()),
            i5(self.rt.into())
        )
    }
}

// SXTB  <Wd>, <Wn>
pub struct SXTB_SBFM_32M_bitfield {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SXTB_SBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100110),
            i1(0b0),
            i6(0b000000),
            i6(0b000111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SXTB  <Xd>, <Wn>
pub struct SXTB_SBFM_64M_bitfield {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SXTB_SBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100110),
            i1(0b1),
            i6(0b000000),
            i6(0b000111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SXTH  <Wd>, <Wn>
pub struct SXTH_SBFM_32M_bitfield {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SXTH_SBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b00),
            i6(0b100110),
            i1(0b0),
            i6(0b000000),
            i6(0b001111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SXTH  <Xd>, <Wn>
pub struct SXTH_SBFM_64M_bitfield {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SXTH_SBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100110),
            i1(0b1),
            i6(0b000000),
            i6(0b001111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SXTW  <Xd>, <Wn>
pub struct SXTW_SBFM_64M_bitfield {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl SXTW_SBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i6(0b100110),
            i1(0b1),
            i6(0b000000),
            i6(0b011111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// SYS  #<op1>, <Cn>, <Cm>, #<op2>{, <Xt>}
pub struct SYS_CR_systeminstrs {
    pub op1: u32,
    pub crn: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(self.op1),
            i4(self.crn),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// SYSL  <Xt>, #<op1>, <Cn>, <Cm>, #<op2>
pub struct SYSL_RC_systeminstrs {
    pub op1: u32,
    pub crn: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl SYSL_RC_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b1),
            i2(0b01),
            i3(self.op1),
            i4(self.crn),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// TBNZ  <R><t>, #<imm>, <label>
pub struct TBNZ_only_testbranch {
    pub b5: u32,
    pub b40: u32,
    pub imm14: u32,
    pub rt: RegisterIndex,
}
impl TBNZ_only_testbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(self.b5),
            i6(0b011011),
            i1(0b1),
            i5(self.b40),
            i14(self.imm14),
            i5(self.rt.into())
        )
    }
}

// TBZ  <R><t>, #<imm>, <label>
pub struct TBZ_only_testbranch {
    pub b5: u32,
    pub b40: u32,
    pub imm14: u32,
    pub rt: RegisterIndex,
}
impl TBZ_only_testbranch {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(self.b5),
            i6(0b011011),
            i1(0b0),
            i5(self.b40),
            i14(self.imm14),
            i5(self.rt.into())
        )
    }
}

// TLBI  <tlbi_op>{, <Xt>}
pub struct TLBI_SYS_CR_systeminstrs {
    pub op1: u32,
    pub crm: u32,
    pub op2: u32,
    pub rt: RegisterIndex,
}
impl TLBI_SYS_CR_systeminstrs {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b01),
            i3(self.op1),
            i4(0b1000),
            i4(self.crm),
            i3(self.op2),
            i5(self.rt.into())
        )
    }
}

// TSB CSYNC
// ARMv8.4, FEAT_TRF
pub struct TSB_HC_hints {
}
impl TSB_HC_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0010),
            i3(0b010),
            i5(0b11111)
        )
    }
}

// TST  <Wn>, #<imm>
pub struct TST_ANDS_32S_log_imm {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
}
impl TST_ANDS_32S_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b11),
            i6(0b100100),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// TST  <Xn>, #<imm>
pub struct TST_ANDS_64S_log_imm {
    pub n: u32,
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
}
impl TST_ANDS_64S_log_imm {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b11),
            i6(0b100100),
            i1(self.n),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// TST  <Wn>, <Wm>{, <shift> #<amount>}
pub struct TST_ANDS_32_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
}
impl TST_ANDS_32_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b11),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// TST  <Xn>, <Xm>{, <shift> #<amount>}
pub struct TST_ANDS_64_log_shift {
    pub shift: u32,
    pub rm: RegisterIndex,
    pub imm6: u32,
    pub rn: RegisterIndex,
}
impl TST_ANDS_64_log_shift {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b11),
            i5(0b01010),
            i2(self.shift),
            i1(0b0),
            i5(self.rm.into()),
            i6(self.imm6),
            i5(self.rn.into()),
            i5(0b11111)
        )
    }
}

// UBFIZ  <Wd>, <Wn>, #<lsb>, #<width>
pub struct UBFIZ_UBFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UBFIZ_UBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UBFIZ  <Xd>, <Xn>, #<lsb>, #<width>
pub struct UBFIZ_UBFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UBFIZ_UBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UBFM  <Wd>, <Wn>, #<immr>, #<imms>
pub struct UBFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UBFM  <Xd>, <Xn>, #<immr>, #<imms>
pub struct UBFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UBFX  <Wd>, <Wn>, #<lsb>, #<width>
pub struct UBFX_UBFM_32M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UBFX_UBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100110),
            i1(0b0),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UBFX  <Xd>, <Xn>, #<lsb>, #<width>
pub struct UBFX_UBFM_64M_bitfield {
    pub immr: u32,
    pub imms: u32,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UBFX_UBFM_64M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b10),
            i6(0b100110),
            i1(0b1),
            i6(self.immr),
            i6(self.imms),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UDF  #<imm>
pub struct UDF_only_perm_undef {
    pub imm16: u32,
}
impl UDF_only_perm_undef {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i16(0b0000000000000000),
            i16(self.imm16)
        )
    }
}

// UDIV  <Wd>, <Wn>, <Wm>
pub struct UDIV_32_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UDIV_32_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i5(0b00001),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UDIV  <Xd>, <Xn>, <Xm>
pub struct UDIV_64_dp_2src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UDIV_64_dp_2src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i8(0b11010110),
            i5(self.rm.into()),
            i5(0b00001),
            i1(0b0),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UMADDL  <Xd>, <Wn>, <Wm>, <Xa>
pub struct UMADDL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UMADDL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b1),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b0),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UMNEGL  <Xd>, <Wn>, <Wm>
pub struct UMNEGL_UMSUBL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UMNEGL_UMSUBL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b1),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b1),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UMSUBL  <Xd>, <Wn>, <Wm>, <Xa>
pub struct UMSUBL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub ra: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UMSUBL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b1),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b1),
            i5(self.ra.into()),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UMULH  <Xd>, <Xn>, <Xm>
pub struct UMULH_64_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UMULH_64_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b1),
            i2(0b10),
            i5(self.rm.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UMULL  <Xd>, <Wn>, <Wm>
pub struct UMULL_UMADDL_64WA_dp_3src {
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UMULL_UMADDL_64WA_dp_3src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i2(0b00),
            i5(0b11011),
            i1(0b1),
            i2(0b01),
            i5(self.rm.into()),
            i1(0b0),
            i5(0b11111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UXTB  <Wd>, <Wn>
pub struct UXTB_UBFM_32M_bitfield {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UXTB_UBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100110),
            i1(0b0),
            i6(0b000000),
            i6(0b000111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// UXTH  <Wd>, <Wn>
pub struct UXTH_UBFM_32M_bitfield {
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
}
impl UXTH_UBFM_32M_bitfield {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b0),
            i2(0b10),
            i6(0b100110),
            i1(0b0),
            i6(0b000000),
            i6(0b001111),
            i5(self.rn.into()),
            i5(self.rd.into())
        )
    }
}

// WFE
pub struct WFE_HI_hints {
}
impl WFE_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b010),
            i5(0b11111)
        )
    }
}

// WFI
pub struct WFI_HI_hints {
}
impl WFI_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b011),
            i5(0b11111)
        )
    }
}

// XAFLAG
// ARMv8.5, FEAT_FlagM2
pub struct XAFLAG_M_pstate {
}
impl XAFLAG_M_pstate {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b000),
            i4(0b0100),
            i4(0b0000),
            i3(0b001),
            i5(0b11111)
        )
    }
}

// XPACD  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct XPACD_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl XPACD_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b1),
            i3(0b000),
            i1(0b1),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// XPACI  <Xd>
// ARMv8.3, FEAT_PAuth
pub struct XPACI_64Z_dp_1src {
    pub rd: RegisterIndex,
}
impl XPACI_64Z_dp_1src {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i8(0b11010110),
            i5(0b00001),
            i1(0b0),
            i1(0b1),
            i3(0b000),
            i1(0b0),
            i5(0b11111),
            i5(self.rd.into())
        )
    }
}

// XPACLRI
// ARMv8.3, FEAT_PAuth
pub struct XPACLRI_HI_hints {
}
impl XPACLRI_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b111),
            i5(0b11111)
        )
    }
}

// YIELD
pub struct YIELD_HI_hints {
}
impl YIELD_HI_hints {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i10(0b1101010100),
            i1(0b0),
            i2(0b00),
            i3(0b011),
            i4(0b0010),
            i4(0b0000),
            i3(0b001),
            i5(0b11111)
        )
    }
}
