// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Manually implemented methods for EL3 system register types.

use crate::registers::{EsrEl3, MdcrEl3, SmcrEl3, SpsrEl3};
use arm_sysregs_common::types::{ExceptionLevel, StackPointer};
use core::fmt::{self, Debug, Formatter};

impl EsrEl3 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

impl Debug for EsrEl3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl3({:#x})", self.bits())
    }
}

impl MdcrEl3 {
    /// Set to 0b10 to disable AArch32 Secure self-hosted privileged debug from S-EL1.
    pub const SPD32: Self = Self::from_bits_retain(0b10 << 14);
    /// Non-secure state owns the Profiling Buffer. Profiling is disabled in Secure and Realm
    /// states.
    pub const NSPB_NS: Self = Self::from_bits_retain(0b11 << 12);
    /// Enable TRBE register access for the security state that owns the buffer.
    pub const NSTB_EN: Self = Self::from_bits_retain(1 << 24);
    /// Together with MDCR_EL3.NSTBE determines which security state owns the trace buffer
    pub const NSTB_SS: Self = Self::from_bits_retain(1 << 25);
}

impl SmcrEl3 {
    /// Build SMCR_EL3 register value from given SSVE vector length.
    pub fn from_ssve_vector_len(vector_length: u64) -> Self {
        Self::from_bits_retain(((vector_length - 1) / 128) & Self::LEN_MASK)
    }
}

impl SpsrEl3 {
    /// AArch64 execution state, EL0.
    pub const M_AARCH64_EL0: Self = Self::from_bits_retain(0b00000);
    /// AArch64 execution state, EL1 with SP_EL0.
    pub const M_AARCH64_EL1T: Self = Self::from_bits_retain(0b00100);
    /// AArch64 execution state, EL1 with SP_EL1.
    pub const M_AARCH64_EL1H: Self = Self::from_bits_retain(0b00101);
    /// AArch64 execution state, EL2 with SP_EL0.
    pub const M_AARCH64_EL2T: Self = Self::from_bits_retain(0b01000);
    /// AArch64 execution state, EL2 with SP_EL2.
    pub const M_AARCH64_EL2H: Self = Self::from_bits_retain(0b01001);
    /// AArch64 execution state, EL3 with SP_EL0.
    pub const M_AARCH64_EL3T: Self = Self::from_bits_retain(0b01100);
    /// AArch64 execution state, EL3 with SP_EL3.
    pub const M_AARCH64_EL3H: Self = Self::from_bits_retain(0b01101);

    /// Exception was taken with PSTATE.SP set to SP_EL0.
    pub const SP_EL0: Self = Self::from_bits_retain(0);
    /// Exception was taken with PSTATE.SP set to SP_ELx.
    pub const SP_ELX: Self = Self::from_bits_retain(1);

    /// All of the N, Z, C and V bits.
    pub const NZCV: Self = Self::V.union(Self::C).union(Self::Z).union(Self::N);

    /// Speculative Store Bypass Safe.
    pub const SSBS: Self = Self::from_bits_retain(1 << 12);

    const EL_MASK: u64 = 0x3;
    const EL_SHIFT: usize = 2;
    const SP_MASK: u64 = 0x1;

    /// Returns the value of the EL field.
    pub const fn exception_level(self) -> ExceptionLevel {
        match (self.bits() >> Self::EL_SHIFT) & Self::EL_MASK {
            0 => ExceptionLevel::El0,
            1 => ExceptionLevel::El1,
            2 => ExceptionLevel::El2,
            3 => ExceptionLevel::El3,
            _ => unreachable!(),
        }
    }

    /// Returns the value of the SP field.
    pub const fn stack_pointer(self) -> StackPointer {
        match self.bits() & Self::SP_MASK {
            0 => StackPointer::El0,
            1 => StackPointer::ElX,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_spsr_el3() {
        assert_eq!(format!("{:?}", SpsrEl3::empty()), "SpsrEl3(0x0)");
        assert_eq!(format!("{:?}", SpsrEl3::NZCV), "SpsrEl3(V | C | Z | N)");
        assert_eq!(format!("{:?}", SpsrEl3::M_AARCH64_EL3H), "SpsrEl3(0xd)");
    }

    #[test]
    fn debug_esr_el3() {
        assert_eq!(format!("{:?}", EsrEl3::empty()), "EsrEl3(0x0)");
        assert_eq!(format!("{:?}", EsrEl3::IL), "EsrEl3(0x2000000)");
        assert_eq!(
            format!("{:?}", EsrEl3::ISS_SYSREG_OPCODE_MASK),
            "EsrEl3(0x3ffc1e)"
        );
    }
}
