#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::encoding::*;
use super::types::*;


// ADC{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct ADC_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADC_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b101),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADCS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct ADCS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADCS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b101),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct ADC_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ADC_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct ADC_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADC_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct ADCS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ADCS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct ADCS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADCS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct ADCS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADCS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ADC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct ADC_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADC_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ADD{<c>}{<q>} <Rd>, PC, #<const>
pub struct ADD_ADR_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADD_ADR_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b100),
            i1(0b0),
            i4(0b1111),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADD{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct ADD_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADD_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b100),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADDS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct ADDS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADDS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b100),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct ADD_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ADD_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADD{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct ADD_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADD_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct ADDS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ADDS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct ADDS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADDS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct ADDS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADDS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ADD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct ADD_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADD_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ADD{<c>}{<q>} {<Rd>,} SP, #<const>
pub struct ADD_SP_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADD_SP_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b100),
            i1(0b0),
            i4(0b1101),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADDS{<c>}{<q>} {<Rd>,} SP, #<const>
pub struct ADDS_SP_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADDS_SP_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b100),
            i1(0b1),
            i4(0b1101),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADD{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX
pub struct ADD_SP_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ADD_SP_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b0),
            i4(0b1101),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADD{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}
pub struct ADD_SP_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADD_SP_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b0),
            i4(0b1101),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADDS{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX
pub struct ADDS_SP_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ADDS_SP_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b1),
            i4(0b1101),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADDS{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}
pub struct ADDS_SP_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ADDS_SP_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b1),
            i4(0b1101),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ADR{<c>}{<q>} <Rd>, <label>
pub struct ADR_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADR_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b100),
            i1(0b0),
            i4(0b1111),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ADR{<c>}{<q>} <Rd>, <label>
pub struct ADR_A2 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ADR_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b010),
            i1(0b0),
            i4(0b1111),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// AND{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct AND_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl AND_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b000),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ANDS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct ANDS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ANDS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b000),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// AND{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct AND_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl AND_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// AND{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct AND_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl AND_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ANDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct ANDS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ANDS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ANDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct ANDS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ANDS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ANDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct ANDS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ANDS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// AND{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct AND_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl AND_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ASR{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct ASR_MOV_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl ASR_MOV_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b10),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ASR{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct ASR_MOV_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ASR_MOV_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ASRS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct ASRS_MOVS_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl ASRS_MOVS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b10),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ASRS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct ASRS_MOVS_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ASRS_MOVS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// B{<c>}{<q>} <label>
pub struct B_A1 {
    pub cond: Condition,
    pub imm24: u32,
}
impl B_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b101),
            i1(0b0),
            i24(self.imm24)
        )
    }
}

// BFC{<c>}{<q>} <Rd>, #<lsb>, #<width>
pub struct BFC_A1 {
    pub cond: Condition,
    pub msb: u32,
    pub rd: RegisterIndex,
    pub lsb: u32,
}
impl BFC_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i7(0b0111110),
            i5(self.msb),
            i4(self.rd.into()),
            i5(self.lsb),
            i3(0b001),
            i4(0b1111)
        )
    }
}

// BFI{<c>}{<q>} <Rd>, <Rn>, #<lsb>, #<width>
pub struct BFI_A1 {
    pub cond: Condition,
    pub msb: u32,
    pub rd: RegisterIndex,
    pub lsb: u32,
    pub rn: RegisterIndex,
}
impl BFI_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i7(0b0111110),
            i5(self.msb),
            i4(self.rd.into()),
            i5(self.lsb),
            i3(0b001),
            i4(self.rn.into())
        )
    }
}

// BIC{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct BIC_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl BIC_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// BICS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct BICS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl BICS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// BIC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct BIC_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl BIC_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// BIC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct BIC_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl BIC_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// BICS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct BICS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl BICS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// BICS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct BICS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl BICS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// BICS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct BICS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl BICS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// BIC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct BIC_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl BIC_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// BKPT{<q>} {#}<imm>
pub struct BKPT_A1 {
    pub cond: Condition,
    pub imm12: u32,
    pub imm4: u32,
}
impl BKPT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i12(self.imm12),
            i4(0b0111),
            i4(self.imm4)
        )
    }
}

// BL{<c>}{<q>} <label>
pub struct BL_i_A1 {
    pub cond: Condition,
    pub imm24: u32,
}
impl BL_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b101),
            i1(0b1),
            i24(self.imm24)
        )
    }
}

// BLX{<c>}{<q>} <label>
pub struct BL_i_A2 {
    pub h: u32,
    pub imm24: u32,
}
impl BL_i_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(0b1111),
            i3(0b101),
            i1(self.h),
            i24(self.imm24)
        )
    }
}

// BLX{<c>}{<q>} <Rm>
pub struct BLX_r_A1 {
    pub cond: Condition,
    pub rm: RegisterIndex,
}
impl BLX_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b00010010),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b0011),
            i4(self.rm.into())
        )
    }
}

// BX{<c>}{<q>} <Rm>
pub struct BX_A1 {
    pub cond: Condition,
    pub rm: RegisterIndex,
}
impl BX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b00010010),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b0001),
            i4(self.rm.into())
        )
    }
}

// BXJ{<c>}{<q>} <Rm>
pub struct BXJ_A1 {
    pub cond: Condition,
    pub rm: RegisterIndex,
}
impl BXJ_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b00010010),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b0010),
            i4(self.rm.into())
        )
    }
}

// CLREX{<c>}{<q>}
pub struct CLREX_A1 {
}
impl CLREX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111101010111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0001),
            i4(0b1111)
        )
    }
}

// CLZ{<c>}{<q>} <Rd>, <Rm>
pub struct CLZ_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CLZ_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b00010110),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b0001),
            i4(self.rm.into())
        )
    }
}

// CMN{<c>}{<q>} <Rn>, #<const>
pub struct CMN_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm12: u32,
}
impl CMN_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i2(0b11),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i12(self.imm12)
        )
    }
}

// CMN{<c>}{<q>} <Rn>, <Rm>, RRX
pub struct CMN_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CMN_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// CMN{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}
pub struct CMN_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl CMN_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// CMN{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>
pub struct CMN_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl CMN_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// CMP{<c>}{<q>} <Rn>, #<const>
pub struct CMP_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm12: u32,
}
impl CMP_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i12(self.imm12)
        )
    }
}

// CMP{<c>}{<q>} <Rn>, <Rm>, RRX
pub struct CMP_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CMP_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// CMP{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}
pub struct CMP_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl CMP_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// CMP{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>
pub struct CMP_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl CMP_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// CPS{<q>} #<mode>
pub struct CPS_A1_AS {
    pub a: u32,
    pub i: u32,
    pub f: u32,
    pub mode: u32,
}
impl CPS_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111100010000),
            i2(0b00),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.a),
            i1(self.i),
            i1(self.f),
            i1(0b0),
            i5(self.mode)
        )
    }
}

// CPSID{<q>} <iflags>
pub struct CPSID_A1_AS {
    pub a: u32,
    pub i: u32,
    pub f: u32,
    pub mode: u32,
}
impl CPSID_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111100010000),
            i2(0b11),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.a),
            i1(self.i),
            i1(self.f),
            i1(0b0),
            i5(self.mode)
        )
    }
}

// CPSID{<q>} <iflags> , #<mode>
pub struct CPSID_A1_ASM {
    pub a: u32,
    pub i: u32,
    pub f: u32,
    pub mode: u32,
}
impl CPSID_A1_ASM {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111100010000),
            i2(0b11),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.a),
            i1(self.i),
            i1(self.f),
            i1(0b0),
            i5(self.mode)
        )
    }
}

// CPSIE{<q>} <iflags>
pub struct CPSIE_A1_AS {
    pub a: u32,
    pub i: u32,
    pub f: u32,
    pub mode: u32,
}
impl CPSIE_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111100010000),
            i2(0b10),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.a),
            i1(self.i),
            i1(self.f),
            i1(0b0),
            i5(self.mode)
        )
    }
}

// CPSIE{<q>} <iflags> , #<mode>
pub struct CPSIE_A1_ASM {
    pub a: u32,
    pub i: u32,
    pub f: u32,
    pub mode: u32,
}
impl CPSIE_A1_ASM {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111100010000),
            i2(0b10),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.a),
            i1(self.i),
            i1(self.f),
            i1(0b0),
            i5(self.mode)
        )
    }
}

// CRC32B{<q>} <Rd>, <Rn>, <Rm>
pub struct CRC32B_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CRC32B_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0100),
            i4(self.rm.into())
        )
    }
}

// CRC32H{<q>} <Rd>, <Rn>, <Rm>
pub struct CRC32H_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CRC32H_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0100),
            i4(self.rm.into())
        )
    }
}

// CRC32W{<q>} <Rd>, <Rn>, <Rm>
pub struct CRC32W_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CRC32W_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0100),
            i4(self.rm.into())
        )
    }
}

// CRC32CB{<q>} <Rd>, <Rn>, <Rm>
pub struct CRC32CB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CRC32CB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(0b0100),
            i4(self.rm.into())
        )
    }
}

// CRC32CH{<q>} <Rd>, <Rn>, <Rm>
pub struct CRC32CH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CRC32CH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(0b0100),
            i4(self.rm.into())
        )
    }
}

// CRC32CW{<q>} <Rd>, <Rn>, <Rm>
pub struct CRC32CW_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl CRC32CW_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(0b0100),
            i4(self.rm.into())
        )
    }
}

// CSDB{<c>}{<q>}
pub struct CSDB_A1 {
    pub cond: Condition,
}
impl CSDB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000010100)
        )
    }
}

// DBG{<c>}{<q>} #<option>
pub struct DBG_A1 {
    pub cond: Condition,
    pub option: u32,
}
impl DBG_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i8(0b00001111),
            i4(self.option)
        )
    }
}

// DMB{<c>}{<q>} {<option>}
pub struct DMB_A1 {
    pub option: u32,
}
impl DMB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111101010111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0101),
            i4(self.option)
        )
    }
}

// DSB{<c>}{<q>} {<option>}
pub struct DSB_A1 {
    pub option: u32,
}
impl DSB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111101010111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0100),
            i4(self.option)
        )
    }
}

// EOR{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct EOR_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl EOR_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b001),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// EORS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct EORS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl EORS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b001),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// EOR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct EOR_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl EOR_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// EOR{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct EOR_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl EOR_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// EORS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct EORS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl EORS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// EORS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct EORS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl EORS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// EORS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct EORS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl EORS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// EOR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct EOR_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl EOR_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ERET{<c>}{<q>}
pub struct ERET_A1 {
    pub cond: Condition,
}
impl ERET_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b00010110),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0110),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0)
        )
    }
}

// ESB{<c>}{<q>}
// ARMv8.2, FEAT_RAS
pub struct ESB_A1 {
    pub cond: Condition,
}
impl ESB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000010000)
        )
    }
}

// HLT{<q>} {#}<imm>
pub struct HLT_A1 {
    pub cond: Condition,
    pub imm12: u32,
    pub imm4: u32,
}
impl HLT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i12(self.imm12),
            i4(0b0111),
            i4(self.imm4)
        )
    }
}

// HVC{<q>} {#}<imm16>
pub struct HVC_A1 {
    pub cond: Condition,
    pub imm12: u32,
    pub imm4: u32,
}
impl HVC_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i12(self.imm12),
            i4(0b0111),
            i4(self.imm4)
        )
    }
}

// ISB{<c>}{<q>} {<option>}
pub struct ISB_A1 {
    pub option: u32,
}
impl ISB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111101010111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0110),
            i4(self.option)
        )
    }
}

// LDA{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDA_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDA_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDAB{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDAB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDAEX{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDAEX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAEX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDAEXB{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDAEXB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAEXB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDAEXD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>]
pub struct LDAEXD_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAEXD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDAEXH{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDAEXH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAEXH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDAH{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDAH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDAH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDC{<c>}{<q>} p14, c5, [<Rn>{, #{+/-}<imm>}]
pub struct LDC_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl LDC_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// LDC{<c>}{<q>} p14, c5, [<Rn>], #{+/-}<imm>
pub struct LDC_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl LDC_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// LDC{<c>}{<q>} p14, c5, [<Rn>, #{+/-}<imm>]!
pub struct LDC_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl LDC_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// LDC{<c>}{<q>} p14, c5, [<Rn>], <option>
pub struct LDC_i_A1_unind {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl LDC_i_A1_unind {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// LDC{<c>}{<q>} p14, c5, <label>
// LDC{<c>}{<q>} p14, c5, [PC, #{+/-}<imm>]
// LDC{<c>}{<q>} p14, c5, [PC], <option>
pub struct LDC_l_A1 {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub w: u32,
    pub imm8: u32,
}
impl LDC_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(self.p),
            i1(self.u),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(0b1111),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// LDM{IA}{<c>}{<q>} <Rn>{!}, <registers>
// LDMFD{<c>}{<q>} <Rn>{!}, <registers>
pub struct LDM_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl LDM_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// LDM{<amode>}{<c>}{<q>} <Rn>{!}, <registers_with_pc>^
pub struct LDM_e_A1_AS {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl LDM_e_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(self.p),
            i1(self.u),
            i1(0b1),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b1),
            i15(self.register_list)
        )
    }
}

// LDM{<amode>}{<c>}{<q>} <Rn>, <registers_without_pc>^
pub struct LDM_u_A1_AS {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl LDM_u_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(self.p),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i15(self.register_list)
        )
    }
}

// LDMDA{<c>}{<q>} <Rn>{!}, <registers>
// LDMFA{<c>}{<q>} <Rn>{!}, <registers>
pub struct LDMDA_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl LDMDA_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// LDMDB{<c>}{<q>} <Rn>{!}, <registers>
// LDMEA{<c>}{<q>} <Rn>{!}, <registers>
pub struct LDMDB_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl LDMDB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// LDMIB{<c>}{<q>} <Rn>{!}, <registers>
// LDMED{<c>}{<q>} <Rn>{!}, <registers>
pub struct LDMIB_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl LDMIB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// LDR{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct LDR_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDR_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDR{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct LDR_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDR_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDR{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct LDR_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDR_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDR{<c>}{<q>} <Rt>, <label>
// LDR{<c>}{<q>} <Rt>, [PC, #{+/-}<imm>]
pub struct LDR_l_A1 {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub w: u32,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDR_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(self.p),
            i1(self.u),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(0b1111),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]
pub struct LDR_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDR_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LDR{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct LDR_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDR_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LDR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!
pub struct LDR_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDR_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LDRB{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct LDRB_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDRB_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDRB{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct LDRB_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDRB_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDRB{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct LDRB_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDRB_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDRB{<c>}{<q>} <Rt>, <label>
// LDRB{<c>}{<q>} <Rt>, [PC, #{+/-}<imm>]
pub struct LDRB_l_A1 {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub w: u32,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDRB_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(self.p),
            i1(self.u),
            i1(0b1),
            i1(self.w),
            i1(0b1),
            i4(0b1111),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]
pub struct LDRB_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDRB_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LDRB{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct LDRB_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDRB_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LDRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!
pub struct LDRB_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDRB_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LDRBT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct LDRBT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDRBT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDRBT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct LDRBT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDRBT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn> {, #{+/-}<imm>}]
pub struct LDRD_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRD_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], #{+/-}<imm>
pub struct LDRD_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRD_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, #{+/-}<imm>]!
pub struct LDRD_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRD_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRD{<c>}{<q>} <Rt>, <Rt2>, <label>
// LDRD{<c>}{<q>} <Rt>, <Rt2>, [PC, #{+/-}<imm>]
pub struct LDRD_l_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRD_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(0b1111),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]
pub struct LDRD_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRD_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], {+/-}<Rm>
pub struct LDRD_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRD_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]!
pub struct LDRD_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRD_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDREX{<c>}{<q>} <Rt>, [<Rn> {, {#}<imm>}]
pub struct LDREX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDREX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDREXB{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDREXB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDREXB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDREXD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>]
pub struct LDREXD_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDREXD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDREXH{<c>}{<q>} <Rt>, [<Rn>]
pub struct LDREXH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl LDREXH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(0b1111)
        )
    }
}

// LDRH{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct LDRH_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRH_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRH{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct LDRH_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRH_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRH{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct LDRH_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRH_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRH{<c>}{<q>} <Rt>, <label>
// LDRH{<c>}{<q>} <Rt>, [PC, #{+/-}<imm>]
pub struct LDRH_l_A1 {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub w: u32,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRH_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(self.p),
            i1(self.u),
            i1(0b1),
            i1(self.w),
            i1(0b1),
            i4(0b1111),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]
pub struct LDRH_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRH_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRH{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct LDRH_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRH_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!
pub struct LDRH_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRH_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRHT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct LDRHT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRHT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRHT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct LDRHT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRHT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSB{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct LDRSB_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSB_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSB{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct LDRSB_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSB_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSB{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct LDRSB_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSB_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSB{<c>}{<q>} <Rt>, <label>
// LDRSB{<c>}{<q>} <Rt>, [PC, #{+/-}<imm>]
pub struct LDRSB_l_A1 {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub w: u32,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSB_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(self.p),
            i1(self.u),
            i1(0b1),
            i1(self.w),
            i1(0b1),
            i4(0b1111),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]
pub struct LDRSB_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSB_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSB{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct LDRSB_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSB_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!
pub struct LDRSB_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSB_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSBT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct LDRSBT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSBT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSBT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct LDRSBT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSBT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSH{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct LDRSH_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSH_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSH{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct LDRSH_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSH_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSH{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct LDRSH_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSH_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSH{<c>}{<q>} <Rt>, <label>
// LDRSH{<c>}{<q>} <Rt>, [PC, #{+/-}<imm>]
pub struct LDRSH_l_A1 {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub w: u32,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSH_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(self.p),
            i1(self.u),
            i1(0b1),
            i1(self.w),
            i1(0b1),
            i4(0b1111),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]
pub struct LDRSH_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSH_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSH{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct LDRSH_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSH_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!
pub struct LDRSH_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSH_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRSHT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct LDRSHT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl LDRSHT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// LDRSHT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct LDRSHT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LDRSHT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LDRT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct LDRT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl LDRT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// LDRT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct LDRT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl LDRT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LSL{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct LSL_MOV_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl LSL_MOV_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b00),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LSL{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct LSL_MOV_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LSL_MOV_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LSLS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct LSLS_MOVS_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl LSLS_MOVS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b00),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LSLS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct LSLS_MOVS_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LSLS_MOVS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LSR{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct LSR_MOV_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl LSR_MOV_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b01),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LSR{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct LSR_MOV_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LSR_MOV_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// LSRS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct LSRS_MOVS_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl LSRS_MOVS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b01),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// LSRS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct LSRS_MOVS_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl LSRS_MOVS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// MCR{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <CRn>, <CRm>{, {#}<opc2>}
pub struct MCR_A1 {
    pub cond: Condition,
    pub opc1: u32,
    pub crn: u32,
    pub rt: RegisterIndex,
    pub coproc: u32,
    pub opc2: u32,
    pub crm: u32,
}
impl MCR_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b1110),
            i3(self.opc1),
            i1(0b0),
            i4(self.crn),
            i4(self.rt.into()),
            i3(0b111),
            i1(self.coproc),
            i3(self.opc2),
            i1(0b1),
            i4(self.crm)
        )
    }
}

// MCRR{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <Rt2>, <CRm>
pub struct MCRR_A1 {
    pub cond: Condition,
    pub rt2: RegisterIndex,
    pub rt: RegisterIndex,
    pub coproc: u32,
    pub opc1: u32,
    pub crm: u32,
}
impl MCRR_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b11000),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rt2.into()),
            i4(self.rt.into()),
            i3(0b111),
            i1(self.coproc),
            i4(self.opc1),
            i4(self.crm)
        )
    }
}

// MLAS{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct MLAS_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl MLAS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b1),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// MLA{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct MLA_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl MLA_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b001),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// MLS{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct MLS_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl MLS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b011),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// MOV{<c>}{<q>} <Rd>, #<const>
pub struct MOV_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl MOV_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// MOVS{<c>}{<q>} <Rd>, #<const>
pub struct MOVS_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl MOVS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// MOV{<c>}{<q>} <Rd>, #<imm16>
// MOVW{<c>}{<q>} <Rd>, #<imm16>
pub struct MOV_i_A2 {
    pub cond: Condition,
    pub imm4: u32,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl MOV_i_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b00),
            i4(self.imm4),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// MOV{<c>}{<q>} <Rd>, <Rm>, RRX
pub struct MOV_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl MOV_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MOV{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}
pub struct MOV_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MOV_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MOVS{<c>}{<q>} <Rd>, <Rm>, RRX
pub struct MOVS_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl MOVS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MOVS{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}
pub struct MOVS_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MOVS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MOVS{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>
pub struct MOVS_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MOVS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// MOV{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>
pub struct MOV_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MOV_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// MOVT{<c>}{<q>} <Rd>, #<imm16>
pub struct MOVT_A1 {
    pub cond: Condition,
    pub imm4: u32,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl MOVT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b1),
            i2(0b00),
            i4(self.imm4),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// MRC{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <CRn>, <CRm>{, {#}<opc2>}
pub struct MRC_A1 {
    pub cond: Condition,
    pub opc1: u32,
    pub crn: u32,
    pub rt: RegisterIndex,
    pub coproc: u32,
    pub opc2: u32,
    pub crm: u32,
}
impl MRC_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b1110),
            i3(self.opc1),
            i1(0b1),
            i4(self.crn),
            i4(self.rt.into()),
            i3(0b111),
            i1(self.coproc),
            i3(self.opc2),
            i1(0b1),
            i4(self.crm)
        )
    }
}

// MRRC{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <Rt2>, <CRm>
pub struct MRRC_A1 {
    pub cond: Condition,
    pub rt2: RegisterIndex,
    pub rt: RegisterIndex,
    pub coproc: u32,
    pub opc1: u32,
    pub crm: u32,
}
impl MRRC_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b11000),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i4(self.rt2.into()),
            i4(self.rt.into()),
            i3(0b111),
            i1(self.coproc),
            i4(self.opc1),
            i4(self.crm)
        )
    }
}

// MRS{<c>}{<q>} <Rd>, <spec_reg>
pub struct MRS_A1_AS {
    pub cond: Condition,
    pub r: u32,
    pub rd: RegisterIndex,
}
impl MRS_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i1(self.r),
            i1(0b0),
            i1(0b0),
            i4(0b1111),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0000),
            i4(0b0000)
        )
    }
}

// MRS{<c>}{<q>} <Rd>, <banked_reg>
pub struct MRS_br_A1_AS {
    pub cond: Condition,
    pub r: u32,
    pub m1: u32,
    pub rd: RegisterIndex,
    pub m: u32,
}
impl MRS_br_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i1(self.r),
            i1(0b0),
            i1(0b0),
            i4(self.m1),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(self.m),
            i4(0b0000),
            i4(0b0000)
        )
    }
}

// MSR{<c>}{<q>} <banked_reg>, <Rn>
pub struct MSR_br_A1_AS {
    pub cond: Condition,
    pub r: u32,
    pub m1: u32,
    pub m: u32,
    pub rn: RegisterIndex,
}
impl MSR_br_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i1(self.r),
            i1(0b1),
            i1(0b0),
            i4(self.m1),
            i4(0b1111),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(self.m),
            i4(0b0000),
            i4(self.rn.into())
        )
    }
}

// MSR{<c>}{<q>} <spec_reg>, #<imm>
pub struct MSR_i_A1_AS {
    pub cond: Condition,
    pub r: u32,
    pub mask: u32,
    pub imm12: u32,
}
impl MSR_i_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(self.r),
            i2(0b10),
            i4(self.mask),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(self.imm12)
        )
    }
}

// MSR{<c>}{<q>} <spec_reg>, <Rn>
pub struct MSR_r_A1_AS {
    pub cond: Condition,
    pub r: u32,
    pub mask: u32,
    pub rn: RegisterIndex,
}
impl MSR_r_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i1(self.r),
            i1(0b1),
            i1(0b0),
            i4(self.mask),
            i4(0b1111),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0000),
            i4(self.rn.into())
        )
    }
}

// MULS{<c>}{<q>} <Rd>, <Rn>{, <Rm>}
pub struct MULS_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl MULS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b1),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// MUL{<c>}{<q>} <Rd>, <Rn>{, <Rm>}
pub struct MUL_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl MUL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b000),
            i1(0b0),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// MVN{<c>}{<q>} <Rd>, #<const>
pub struct MVN_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl MVN_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b11),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// MVNS{<c>}{<q>} <Rd>, #<const>
pub struct MVNS_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl MVNS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b11),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// MVN{<c>}{<q>} <Rd>, <Rm>, RRX
pub struct MVN_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl MVN_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MVN{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}
pub struct MVN_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MVN_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MVNS{<c>}{<q>} <Rd>, <Rm>, RRX
pub struct MVNS_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl MVNS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MVNS{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}
pub struct MVNS_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MVNS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// MVNS{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>
pub struct MVNS_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MVNS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// MVN{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>
pub struct MVN_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl MVN_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// NOP{<c>}{<q>}
pub struct NOP_A1 {
    pub cond: Condition,
}
impl NOP_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000000000)
        )
    }
}

// ORR{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct ORR_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ORR_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ORRS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct ORRS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl ORRS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00111),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// ORR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct ORR_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ORR_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ORR{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct ORR_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ORR_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ORRS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct ORRS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ORRS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ORRS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct ORRS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ORRS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ORRS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct ORRS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ORRS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// ORR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct ORR_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl ORR_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// PKHBT{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, LSL #<imm>}
pub struct PKHBT_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl PKHBT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b01101000),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i1(0b0),
            i2(0b01),
            i4(self.rm.into())
        )
    }
}

// PKHTB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ASR #<imm>}
pub struct PKHTB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl PKHTB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b01101000),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i1(0b1),
            i2(0b01),
            i4(self.rm.into())
        )
    }
}

// PLD{<c>}{<q>} [<Rn> {, #{+/-}<imm>}]
pub struct PLD_i_A1 {
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm12: u32,
}
impl PLD_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111010),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(self.imm12)
        )
    }
}

// PLDW{<c>}{<q>} [<Rn> {, #{+/-}<imm>}]
pub struct PLDW_i_A1 {
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm12: u32,
}
impl PLDW_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111010),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(self.imm12)
        )
    }
}

// PLD{<c>}{<q>} <label>
// PLD{<c>}{<q>} [PC, #{+/-}<imm>]
pub struct PLD_l_A1 {
    pub u: u32,
    pub imm12: u32,
}
impl PLD_l_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111010),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i2(0b01),
            i4(0b1111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(self.imm12)
        )
    }
}

// PLD{<c>}{<q>} [<Rn>, {+/-}<Rm> {, <shift> #<amount>}]
pub struct PLD_r_A1 {
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl PLD_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111011),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// PLD{<c>}{<q>} [<Rn>, {+/-}<Rm> , RRX]
pub struct PLD_r_A1_RRX {
    pub u: u32,
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl PLD_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111011),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// PLDW{<c>}{<q>} [<Rn>, {+/-}<Rm> {, <shift> #<amount>}]
pub struct PLDW_r_A1 {
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl PLDW_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111011),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// PLDW{<c>}{<q>} [<Rn>, {+/-}<Rm> , RRX]
pub struct PLDW_r_A1_RRX {
    pub u: u32,
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl PLDW_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111011),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// PLI{<c>}{<q>} [<Rn> {, #{+/-}<imm>}]
// PLI{<c>}{<q>} <label>
// PLI{<c>}{<q>} [PC, #{+/-}<imm>]
pub struct PLI_i_A1 {
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm12: u32,
}
impl PLI_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111010),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(self.imm12)
        )
    }
}

// PLI{<c>}{<q>} [<Rn>, {+/-}<Rm> , RRX]
pub struct PLI_r_A1_RRX {
    pub u: u32,
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl PLI_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111011),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// PLI{<c>}{<q>} [<Rn>, {+/-}<Rm> {, <shift> #<amount>}]
pub struct PLI_r_A1 {
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl PLI_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111011),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// POP{<c>}{<q>} <registers>
pub struct POP_LDM_A1 {
    pub cond: Condition,
    pub register_list: u32,
}
impl POP_LDM_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i4(0b1101),
            i16(self.register_list)
        )
    }
}

// POP{<c>}{<q>} <single_register_list>
pub struct POP_LDR_i_A1_post {
    pub cond: Condition,
    pub rt: RegisterIndex,
}
impl POP_LDR_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i4(0b1101),
            i4(self.rt.into()),
            i12(0b000000000100)
        )
    }
}

// PSSBB{<q>}
pub struct PSSBB_A1 {
}
impl PSSBB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111101010111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0100),
            i4(0b0100)
        )
    }
}

// PUSH{<c>}{<q>} <registers>
pub struct PUSH_STMDB_A1 {
    pub cond: Condition,
    pub register_list: u32,
}
impl PUSH_STMDB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(0b1101),
            i16(self.register_list)
        )
    }
}

// PUSH{<c>}{<q>} <single_register_list>
pub struct PUSH_STR_i_A1_pre {
    pub cond: Condition,
    pub rt: RegisterIndex,
}
impl PUSH_STR_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(0b1101),
            i4(self.rt.into()),
            i12(0b000000000100)
        )
    }
}

// QADD{<c>}{<q>} {<Rd>,} <Rm>, <Rn>
pub struct QADD_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QADD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0101),
            i4(self.rm.into())
        )
    }
}

// QADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct QADD16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QADD16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b010),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// QADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct QADD8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QADD8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b010),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// QASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct QASX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QASX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b010),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// QDADD{<c>}{<q>} {<Rd>,} <Rm>, <Rn>
pub struct QDADD_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QDADD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0101),
            i4(self.rm.into())
        )
    }
}

// QDSUB{<c>}{<q>} {<Rd>,} <Rm>, <Rn>
pub struct QDSUB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QDSUB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0101),
            i4(self.rm.into())
        )
    }
}

// QSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct QSAX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QSAX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b010),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// QSUB{<c>}{<q>} {<Rd>,} <Rm>, <Rn>
pub struct QSUB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QSUB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0101),
            i4(self.rm.into())
        )
    }
}

// QSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct QSUB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QSUB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b010),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// QSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct QSUB8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl QSUB8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b010),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// RBIT{<c>}{<q>} <Rd>, <Rm>
pub struct RBIT_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RBIT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i3(0b011),
            i4(self.rm.into())
        )
    }
}

// REV{<c>}{<q>} <Rd>, <Rm>
pub struct REV_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl REV_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i3(0b011),
            i4(self.rm.into())
        )
    }
}

// REV16{<c>}{<q>} <Rd>, <Rm>
pub struct REV16_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl REV16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i3(0b011),
            i4(self.rm.into())
        )
    }
}

// REVSH{<c>}{<q>} <Rd>, <Rm>
pub struct REVSH_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl REVSH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i3(0b011),
            i4(self.rm.into())
        )
    }
}

// RFEDA{<c>}{<q>} <Rn>{!}
// RFEFA{<c>}{<q>} <Rn>{!}
pub struct RFEDA_A1_AS {
    pub w: u32,
    pub rn: RegisterIndex,
}
impl RFEDA_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i11(0b00001010000),
            i5(0b00000)
        )
    }
}

// RFEDB{<c>}{<q>} <Rn>{!}
// RFEEA{<c>}{<q>} <Rn>{!}
pub struct RFEDB_A1_AS {
    pub w: u32,
    pub rn: RegisterIndex,
}
impl RFEDB_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i11(0b00001010000),
            i5(0b00000)
        )
    }
}

// RFE{IA}{<c>}{<q>} <Rn>{!}
// RFEFD{<c>}{<q>} <Rn>{!}
pub struct RFEIA_A1_AS {
    pub w: u32,
    pub rn: RegisterIndex,
}
impl RFEIA_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i11(0b00001010000),
            i5(0b00000)
        )
    }
}

// RFEIB{<c>}{<q>} <Rn>{!}
// RFEED{<c>}{<q>} <Rn>{!}
pub struct RFEIB_A1_AS {
    pub w: u32,
    pub rn: RegisterIndex,
}
impl RFEIB_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(self.w),
            i1(0b1),
            i4(self.rn.into()),
            i11(0b00001010000),
            i5(0b00000)
        )
    }
}

// ROR{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct ROR_MOV_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl ROR_MOV_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// ROR{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct ROR_MOV_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl ROR_MOV_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// RORS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>
pub struct RORS_MOVS_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rm: RegisterIndex,
}
impl RORS_MOVS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RORS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>
pub struct RORS_MOVS_rr_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RORS_MOVS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// RRX{<c>}{<q>} {<Rd>,} <Rm>
pub struct RRX_MOV_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RRX_MOV_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(0b0000),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RRXS{<c>}{<q>} {<Rd>,} <Rm>
pub struct RRXS_MOVS_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RRXS_MOVS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b1),
            i4(0b0000),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSB{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct RSB_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl RSB_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b011),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// RSBS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct RSBS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl RSBS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b011),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// RSB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct RSB_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RSB_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b011),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct RSB_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSB_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b011),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct RSBS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RSBS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b011),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct RSBS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSBS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b011),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct RSBS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSBS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b011),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// RSB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct RSB_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSB_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b011),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// RSC{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct RSC_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl RSC_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b111),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// RSCS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct RSCS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl RSCS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b111),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// RSC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct RSC_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RSC_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct RSC_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSC_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct RSCS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl RSCS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct RSCS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSCS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// RSCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct RSCS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSCS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// RSC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct RSC_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl RSC_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SADD16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SADD16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b001),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SADD8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SADD8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b001),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SASX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SASX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b001),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SB{<q>}
pub struct SB_A1 {
}
impl SB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111101010111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(0b0000)
        )
    }
}

// SBC{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct SBC_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl SBC_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b110),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// SBCS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct SBCS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl SBCS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b110),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// SBC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct SBC_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SBC_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SBC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct SBC_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SBC_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SBCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct SBCS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SBCS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SBCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct SBCS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SBCS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SBCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct SBCS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SBCS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SBC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct SBC_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SBC_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SBFX{<c>}{<q>} <Rd>, <Rn>, #<lsb>, #<width>
pub struct SBFX_A1 {
    pub cond: Condition,
    pub widthm1: u32,
    pub rd: RegisterIndex,
    pub lsb: u32,
    pub rn: RegisterIndex,
}
impl SBFX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01111),
            i1(0b0),
            i1(0b1),
            i5(self.widthm1),
            i4(self.rd.into()),
            i5(self.lsb),
            i3(0b101),
            i4(self.rn.into())
        )
    }
}

// SDIV{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SDIV_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SDIV_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b001),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i3(0b000),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SEL{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SEL_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SEL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b01101000),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1011),
            i4(self.rm.into())
        )
    }
}

// SETEND{<q>} <endian_specifier>
pub struct SETEND_A1 {
    pub e: u32,
}
impl SETEND_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111100010000),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.e),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(0b00000)
        )
    }
}

// SETPAN{<q>} #<imm>
// ARMv8.1, FEAT_PAN
pub struct SETPAN_A1 {
    pub imm1: u32,
}
impl SETPAN_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111100010001),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.imm1),
            i1(0b0),
            i4(0b0000),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0)
        )
    }
}

// SEV{<c>}{<q>}
pub struct SEV_A1 {
    pub cond: Condition,
}
impl SEV_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000000100)
        )
    }
}

// SEVL{<c>}{<q>}
pub struct SEVL_A1 {
    pub cond: Condition,
}
impl SEVL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000000101)
        )
    }
}

// SHADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SHADD16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SHADD16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b011),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SHADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SHADD8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SHADD8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b011),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SHASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SHASX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SHASX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b011),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SHSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SHSAX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SHSAX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b011),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SHSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SHSUB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SHSUB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b011),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SHSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SHSUB8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SHSUB8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b011),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SMC{<c>}{<q>} {#}<imm4>
pub struct SMC_A1_AS {
    pub cond: Condition,
    pub imm4: u32,
}
impl SMC_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b0),
            i12(0b000000000000),
            i4(0b0111),
            i4(self.imm4)
        )
    }
}

// SMLABB{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLABB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLABB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLABT{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLABT_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLABT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLATB{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLATB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLATB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLATT{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLATT_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLATT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLAD{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLAD_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLAD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMLADX{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLADX_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLADX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMLALS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLALS_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLALS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b1),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// SMLAL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLAL_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLAL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b111),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// SMLALBB{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLALBB_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLALBB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLALBT{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLALBT_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLALBT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLALTB{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLALTB_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLALTB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLALTT{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLALTT_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLALTT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b10),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLALD{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLALD_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLALD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b100),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMLALDX{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLALDX_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLALDX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b100),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMLAWB{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLAWB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLAWB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLAWT{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLAWT_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLAWT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMLSD{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLSD_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLSD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b01),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMLSDX{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMLSDX_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLSDX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b01),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMLSLD{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLSLD_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLSLD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b100),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i2(0b01),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMLSLDX{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMLSLDX_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMLSLDX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b100),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i2(0b01),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMMLA{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMMLA_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMMLA_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b101),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMMLAR{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMMLAR_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMMLAR_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b101),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMMLS{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMMLS_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMMLS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b101),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b11),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMMLSR{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct SMMLSR_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMMLSR_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b101),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i2(0b11),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMMUL{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMMUL_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMMUL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b101),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMMULR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMMULR_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMMULR_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b101),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMUAD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMUAD_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMUAD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMUADX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMUADX_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMUADX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMULBB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMULBB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULBB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b0),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMULBT{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMULBT_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULBT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b0),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMULTB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMULTB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULTB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b0),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMULTT{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMULTT_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULTT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b11),
            i1(0b0),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMULLS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMULLS_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULLS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b1),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// SMULL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct SMULL_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b110),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// SMULWB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMULWB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULWB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMULWT{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMULWT_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMULWT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b0),
            i4(self.rd.into()),
            i4(0b0000),
            i4(self.rm.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into())
        )
    }
}

// SMUSD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMUSD_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMUSD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i2(0b01),
            i1(0b0),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SMUSDX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SMUSDX_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SMUSDX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b000),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i2(0b01),
            i1(0b1),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// SRSDA{<c>}{<q>} SP{!}, #<mode>
pub struct SRSDA_A1_AS {
    pub w: u32,
    pub mode: u32,
}
impl SRSDA_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i1(self.w),
            i1(0b0),
            i4(0b1101),
            i11(0b00000101000),
            i5(self.mode)
        )
    }
}

// SRSDB{<c>}{<q>} SP{!}, #<mode>
pub struct SRSDB_A1_AS {
    pub w: u32,
    pub mode: u32,
}
impl SRSDB_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b1),
            i1(0b0),
            i1(0b1),
            i1(self.w),
            i1(0b0),
            i4(0b1101),
            i11(0b00000101000),
            i5(self.mode)
        )
    }
}

// SRS{IA}{<c>}{<q>} SP{!}, #<mode>
pub struct SRSIA_A1_AS {
    pub w: u32,
    pub mode: u32,
}
impl SRSIA_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b0),
            i1(0b1),
            i1(0b1),
            i1(self.w),
            i1(0b0),
            i4(0b1101),
            i11(0b00000101000),
            i5(self.mode)
        )
    }
}

// SRSIB{<c>}{<q>} SP{!}, #<mode>
pub struct SRSIB_A1_AS {
    pub w: u32,
    pub mode: u32,
}
impl SRSIB_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(0b1111100),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(self.w),
            i1(0b0),
            i4(0b1101),
            i11(0b00000101000),
            i5(self.mode)
        )
    }
}

// SSAT{<c>}{<q>} <Rd>, #<imm>, <Rn>, ASR #<amount>
pub struct SSAT_A1_ASR {
    pub cond: Condition,
    pub sat_imm: u32,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rn: RegisterIndex,
}
impl SSAT_A1_ASR {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i1(0b1),
            i5(self.sat_imm),
            i4(self.rd.into()),
            i5(self.imm5),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into())
        )
    }
}

// SSAT{<c>}{<q>} <Rd>, #<imm>, <Rn> {, LSL #<amount>}
pub struct SSAT_A1_LSL {
    pub cond: Condition,
    pub sat_imm: u32,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rn: RegisterIndex,
}
impl SSAT_A1_LSL {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i1(0b1),
            i5(self.sat_imm),
            i4(self.rd.into()),
            i5(self.imm5),
            i1(0b0),
            i2(0b01),
            i4(self.rn.into())
        )
    }
}

// SSAT16{<c>}{<q>} <Rd>, #<imm>, <Rn>
pub struct SSAT16_A1 {
    pub cond: Condition,
    pub sat_imm: u32,
    pub rd: RegisterIndex,
    pub rn: RegisterIndex,
}
impl SSAT16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b10),
            i4(self.sat_imm),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b0011),
            i4(self.rn.into())
        )
    }
}

// SSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SSAX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SSAX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b001),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SSBB{<q>}
pub struct SSBB_A1 {
}
impl SSBB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i12(0b111101010111),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(0b0100),
            i4(0b0000)
        )
    }
}

// SSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SSUB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SSUB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b001),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct SSUB8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SSUB8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b001),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STC{<c>}{<q>} p14, c5, [<Rn>{, #{+/-}<imm>}]
pub struct STC_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl STC_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// STC{<c>}{<q>} p14, c5, [<Rn>], #{+/-}<imm>
pub struct STC_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl STC_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// STC{<c>}{<q>} p14, c5, [<Rn>, #{+/-}<imm>]!
pub struct STC_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl STC_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// STC{<c>}{<q>} p14, c5, [<Rn>], <option>
pub struct STC_A1_unind {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm8: u32,
}
impl STC_A1_unind {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b110),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(0b0101),
            i3(0b111),
            i1(0b0),
            i8(self.imm8)
        )
    }
}

// STL{<c>}{<q>} <Rt>, [<Rn>]
pub struct STL_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(0b1111),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STLB{<c>}{<q>} <Rt>, [<Rn>]
pub struct STLB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(0b1111),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STLEX{<c>}{<q>} <Rd>, <Rt>, [<Rn>]
pub struct STLEX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLEX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STLEXB{<c>}{<q>} <Rd>, <Rt>, [<Rn>]
pub struct STLEXB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLEXB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STLEXD{<c>}{<q>} <Rd>, <Rt>, <Rt2>, [<Rn>]
pub struct STLEXD_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLEXD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STLEXH{<c>}{<q>} <Rd>, <Rt>, [<Rn>]
pub struct STLEXH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLEXH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STLH{<c>}{<q>} <Rt>, [<Rn>]
pub struct STLH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STLH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b0),
            i4(self.rn.into()),
            i4(0b1111),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STM{IA}{<c>}{<q>} <Rn>{!}, <registers>
// STMEA{<c>}{<q>} <Rn>{!}, <registers>
pub struct STM_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl STM_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i1(self.w),
            i1(0b0),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// STM{<amode>}{<c>}{<q>} <Rn>, <registers>^
pub struct STM_u_A1_AS {
    pub cond: Condition,
    pub p: u32,
    pub u: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl STM_u_A1_AS {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(self.p),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// STMDA{<c>}{<q>} <Rn>{!}, <registers>
// STMED{<c>}{<q>} <Rn>{!}, <registers>
pub struct STMDA_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl STMDA_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(self.w),
            i1(0b0),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// STMDB{<c>}{<q>} <Rn>{!}, <registers>
// STMFD{<c>}{<q>} <Rn>{!}, <registers>
pub struct STMDB_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl STMDB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i1(self.w),
            i1(0b0),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// STMIB{<c>}{<q>} <Rn>{!}, <registers>
// STMFA{<c>}{<q>} <Rn>{!}, <registers>
pub struct STMIB_A1 {
    pub cond: Condition,
    pub w: u32,
    pub rn: RegisterIndex,
    pub register_list: u32,
}
impl STMIB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b100),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i1(self.w),
            i1(0b0),
            i4(self.rn.into()),
            i16(self.register_list)
        )
    }
}

// STR{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct STR_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STR_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STR{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct STR_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STR_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STR{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct STR_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STR_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]
pub struct STR_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STR_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// STR{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct STR_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STR_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// STR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!
pub struct STR_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STR_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// STRB{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct STRB_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STRB_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STRB{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct STRB_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STRB_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STRB{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct STRB_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STRB_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]
pub struct STRB_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STRB_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// STRB{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct STRB_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STRB_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// STRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!
pub struct STRB_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STRB_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// STRBT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct STRBT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STRBT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STRBT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct STRBT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STRBT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn> {, #{+/-}<imm>}]
pub struct STRD_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl STRD_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], #{+/-}<imm>
pub struct STRD_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl STRD_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, #{+/-}<imm>]!
pub struct STRD_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl STRD_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]
pub struct STRD_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl STRD_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], {+/-}<Rm>
pub struct STRD_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl STRD_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]!
pub struct STRD_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl STRD_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STREX{<c>}{<q>} <Rd>, <Rt>, [<Rn> {, {#}<imm>}]
pub struct STREX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STREX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b00),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STREXB{<c>}{<q>} <Rd>, <Rt>, [<Rn>]
pub struct STREXB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STREXB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b10),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STREXD{<c>}{<q>} <Rd>, <Rt>, <Rt2>, [<Rn>]
pub struct STREXD_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STREXD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b01),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STREXH{<c>}{<q>} <Rd>, <Rt>, [<Rn>]
pub struct STREXH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rt: RegisterIndex,
}
impl STREXH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00011),
            i2(0b11),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b1001),
            i4(self.rt.into())
        )
    }
}

// STRH{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]
pub struct STRH_i_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl STRH_i_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// STRH{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>
pub struct STRH_i_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl STRH_i_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// STRH{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!
pub struct STRH_i_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl STRH_i_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// STRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]
pub struct STRH_r_A1_off {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl STRH_r_A1_off {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STRH{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct STRH_r_A1_post {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl STRH_r_A1_post {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!
pub struct STRH_r_A1_pre {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl STRH_r_A1_pre {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b1),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STRHT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct STRHT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm4h: u32,
    pub imm4l: u32,
}
impl STRHT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i4(self.imm4h),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.imm4l)
        )
    }
}

// STRHT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>
pub struct STRHT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub rm: RegisterIndex,
}
impl STRHT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b000),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b1),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// STRT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}
pub struct STRT_A1 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm12: u32,
}
impl STRT_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b010),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i12(self.imm12)
        )
    }
}

// STRT{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}
pub struct STRT_A2 {
    pub cond: Condition,
    pub u: u32,
    pub rn: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl STRT_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i3(0b011),
            i1(0b0),
            i1(self.u),
            i1(0b0),
            i1(0b1),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rt.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUB{<c>}{<q>} <Rd>, PC, #<const>
pub struct SUB_ADR_A2 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl SUB_ADR_A2 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b010),
            i1(0b0),
            i4(0b1111),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// SUB{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct SUB_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl SUB_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b010),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// SUBS{<c>}{<q>} {<Rd>,} <Rn>, #<const>
pub struct SUBS_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl SUBS_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b010),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// SUB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct SUB_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SUB_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct SUB_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SUB_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX
pub struct SUBS_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SUBS_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}
pub struct SUBS_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SUBS_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct SUBS_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SUBS_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b1),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SUB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>
pub struct SUB_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SUB_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b0),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// SUB{<c>}{<q>} {<Rd>,} SP, #<const>
pub struct SUB_SP_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl SUB_SP_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b010),
            i1(0b0),
            i4(0b1101),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// SUBS{<c>}{<q>} {<Rd>,} SP, #<const>
pub struct SUBS_SP_i_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm12: u32,
}
impl SUBS_SP_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0010),
            i3(0b010),
            i1(0b1),
            i4(0b1101),
            i4(self.rd.into()),
            i12(self.imm12)
        )
    }
}

// SUB{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX
pub struct SUB_SP_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SUB_SP_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b0),
            i4(0b1101),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUB{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}
pub struct SUB_SP_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SUB_SP_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b0),
            i4(0b1101),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUBS{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX
pub struct SUBS_SP_r_A1_RRX {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl SUBS_SP_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b1),
            i4(0b1101),
            i4(self.rd.into()),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SUBS{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}
pub struct SUBS_SP_r_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl SUBS_SP_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b1),
            i4(0b1101),
            i4(self.rd.into()),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// SVC{<c>}{<q>} {#}<imm>
pub struct SVC_A1 {
    pub cond: Condition,
    pub imm24: u32,
}
impl SVC_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b1111),
            i24(self.imm24)
        )
    }
}

// SXTAB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}
pub struct SXTAB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl SXTAB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b10),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// SXTAB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}
pub struct SXTAB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl SXTAB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b00),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// SXTAH{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}
pub struct SXTAH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl SXTAH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b11),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// SXTB{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}
pub struct SXTB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl SXTB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b10),
            i4(0b1111),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// SXTB16{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}
pub struct SXTB16_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl SXTB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b00),
            i4(0b1111),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// SXTH{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}
pub struct SXTH_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl SXTH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b0),
            i2(0b11),
            i4(0b1111),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// TEQ{<c>}{<q>} <Rn>, #<const>
pub struct TEQ_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm12: u32,
}
impl TEQ_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i2(0b01),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i12(self.imm12)
        )
    }
}

// TEQ{<c>}{<q>} <Rn>, <Rm>, RRX
pub struct TEQ_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl TEQ_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// TEQ{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}
pub struct TEQ_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl TEQ_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// TEQ{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>
pub struct TEQ_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl TEQ_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b01),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// TSB{<c>}{<q>} CSYNC
// ARMv8.4, FEAT_TRF
pub struct TSB_A1 {
    pub cond: Condition,
}
impl TSB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i4(0b0000),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000010010)
        )
    }
}

// TST{<c>}{<q>} <Rn>, #<const>
pub struct TST_i_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm12: u32,
}
impl TST_i_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i12(self.imm12)
        )
    }
}

// TST{<c>}{<q>} <Rn>, <Rm>, RRX
pub struct TST_r_A1_RRX {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rm: RegisterIndex,
}
impl TST_r_A1_RRX {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(0b00000),
            i2(0b11),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// TST{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}
pub struct TST_r_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub imm5: u32,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl TST_r_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i5(self.imm5),
            i2(self.stype),
            i1(0b0),
            i4(self.rm.into())
        )
    }
}

// TST{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>
pub struct TST_rr_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rs: RegisterIndex,
    pub stype: u32,
    pub rm: RegisterIndex,
}
impl TST_rr_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00010),
            i2(0b00),
            i1(0b1),
            i4(self.rn.into()),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i1(0b0),
            i4(self.rs.into()),
            i1(0b0),
            i2(self.stype),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UADD16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UADD16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b101),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UADD8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UADD8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b101),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UASX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UASX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b101),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UBFX{<c>}{<q>} <Rd>, <Rn>, #<lsb>, #<width>
pub struct UBFX_A1 {
    pub cond: Condition,
    pub widthm1: u32,
    pub rd: RegisterIndex,
    pub lsb: u32,
    pub rn: RegisterIndex,
}
impl UBFX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01111),
            i1(0b1),
            i1(0b1),
            i5(self.widthm1),
            i4(self.rd.into()),
            i5(self.lsb),
            i3(0b101),
            i4(self.rn.into())
        )
    }
}

// UDF{<c>}{<q>} {#}<imm>
pub struct UDF_A1 {
    pub imm12: u32,
    pub imm4: u32,
}
impl UDF_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(0b1110),
            i8(0b01111111),
            i12(self.imm12),
            i4(0b1111),
            i4(self.imm4)
        )
    }
}

// UDIV{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UDIV_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl UDIV_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01110),
            i3(0b011),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i3(0b000),
            i1(0b1),
            i4(self.rn.into())
        )
    }
}

// UHADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UHADD16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UHADD16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b111),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UHADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UHADD8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UHADD8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b111),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UHASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UHASX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UHASX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b111),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UHSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UHSAX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UHSAX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b111),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UHSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UHSUB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UHSUB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b111),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UHSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UHSUB8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UHSUB8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b111),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UMAAL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct UMAAL_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl UMAAL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b010),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// UMLALS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct UMLALS_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl UMLALS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b1),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// UMLAL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct UMLAL_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl UMLAL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b101),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// UMULLS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct UMULLS_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl UMULLS_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b1),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// UMULL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>
pub struct UMULL_A1 {
    pub cond: Condition,
    pub rdhi: RegisterIndex,
    pub rdlo: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl UMULL_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i4(0b0000),
            i3(0b100),
            i1(0b0),
            i4(self.rdhi.into()),
            i4(self.rdlo.into()),
            i4(self.rm.into()),
            i4(0b1001),
            i4(self.rn.into())
        )
    }
}

// UQADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UQADD16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UQADD16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b110),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UQADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UQADD8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UQADD8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b110),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b00),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UQASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UQASX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UQASX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b110),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b01),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UQSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UQSAX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UQSAX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b110),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UQSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UQSUB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UQSUB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b110),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UQSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct UQSUB8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl UQSUB8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b110),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// USAD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct USAD8_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl USAD8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b01111000),
            i4(self.rd.into()),
            i4(0b1111),
            i4(self.rm.into()),
            i4(0b0001),
            i4(self.rn.into())
        )
    }
}

// USADA8{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>
pub struct USADA8_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub ra: RegisterIndex,
    pub rm: RegisterIndex,
    pub rn: RegisterIndex,
}
impl USADA8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i8(0b01111000),
            i4(self.rd.into()),
            i4(self.ra.into()),
            i4(self.rm.into()),
            i4(0b0001),
            i4(self.rn.into())
        )
    }
}

// USAT{<c>}{<q>} <Rd>, #<imm>, <Rn>, ASR #<amount>
pub struct USAT_A1_ASR {
    pub cond: Condition,
    pub sat_imm: u32,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rn: RegisterIndex,
}
impl USAT_A1_ASR {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i1(0b1),
            i5(self.sat_imm),
            i4(self.rd.into()),
            i5(self.imm5),
            i1(0b1),
            i2(0b01),
            i4(self.rn.into())
        )
    }
}

// USAT{<c>}{<q>} <Rd>, #<imm>, <Rn> {, LSL #<amount>}
pub struct USAT_A1_LSL {
    pub cond: Condition,
    pub sat_imm: u32,
    pub rd: RegisterIndex,
    pub imm5: u32,
    pub rn: RegisterIndex,
}
impl USAT_A1_LSL {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i1(0b1),
            i5(self.sat_imm),
            i4(self.rd.into()),
            i5(self.imm5),
            i1(0b0),
            i2(0b01),
            i4(self.rn.into())
        )
    }
}

// USAT16{<c>}{<q>} <Rd>, #<imm>, <Rn>
pub struct USAT16_A1 {
    pub cond: Condition,
    pub sat_imm: u32,
    pub rd: RegisterIndex,
    pub rn: RegisterIndex,
}
impl USAT16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b10),
            i4(self.sat_imm),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i4(0b0011),
            i4(self.rn.into())
        )
    }
}

// USAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct USAX_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl USAX_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b101),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b10),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// USUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct USUB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl USUB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b101),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b0),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// USUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>
pub struct USUB8_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rm: RegisterIndex,
}
impl USUB8_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01100),
            i3(0b101),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i2(0b11),
            i1(0b1),
            i4(self.rm.into())
        )
    }
}

// UXTAB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}
pub struct UXTAB_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl UXTAB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b10),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// UXTAB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}
pub struct UXTAB16_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl UXTAB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b00),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// UXTAH{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}
pub struct UXTAH_A1 {
    pub cond: Condition,
    pub rn: RegisterIndex,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl UXTAH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b11),
            i4(self.rn.into()),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// UXTB{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}
pub struct UXTB_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl UXTB_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b10),
            i4(0b1111),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// UXTB16{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}
pub struct UXTB16_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl UXTB16_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b00),
            i4(0b1111),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// UXTH{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}
pub struct UXTH_A1 {
    pub cond: Condition,
    pub rd: RegisterIndex,
    pub rotate: u32,
    pub rm: RegisterIndex,
}
impl UXTH_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b01101),
            i1(0b1),
            i2(0b11),
            i4(0b1111),
            i4(self.rd.into()),
            i2(self.rotate),
            i1(0b0),
            i1(0b0),
            i4(0b0111),
            i4(self.rm.into())
        )
    }
}

// WFE{<c>}{<q>}
pub struct WFE_A1 {
    pub cond: Condition,
}
impl WFE_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000000010)
        )
    }
}

// WFI{<c>}{<q>}
pub struct WFI_A1 {
    pub cond: Condition,
}
impl WFI_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000000011)
        )
    }
}

// YIELD{<c>}{<q>}
pub struct YIELD_A1 {
    pub cond: Condition,
}
impl YIELD_A1 {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i4(self.cond as u32),
            i5(0b00110),
            i1(0b0),
            i2(0b10),
            i2(0b00),
            i2(0b00),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i1(0b1),
            i12(0b000000000001)
        )
    }
}
