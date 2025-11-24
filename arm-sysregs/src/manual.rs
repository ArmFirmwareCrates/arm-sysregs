// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Manually implemented methods for system register types.

use num_enum::TryFromPrimitive;

use crate::{
    EsrEl1, EsrEl2, EsrEl3, IdAa64mmfr1El1, IdAa64mmfr2El1, IdAa64mmfr3El1, MpidrEl1, SpsrEl1,
    SpsrEl2, SpsrEl3, read_mpidr_el1,
};
use core::fmt::{self, Debug, Formatter};

impl EsrEl1 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

impl Debug for EsrEl1 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl1({:#x})", self.0)
    }
}

impl EsrEl2 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

impl Debug for EsrEl2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl2({:#x})", self.0)
    }
}

impl EsrEl3 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

impl Debug for EsrEl3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl3({:#x})", self.0)
    }
}

impl IdAa64mmfr1El1 {
    const VH_SUPPORTED: u8 = 0b0001;
    const HCX_SUPPORTED: u8 = 0b0001;

    /// Indicates presence of FEAT_VHE.
    pub fn is_feat_vhe_present(self) -> bool {
        self.vh() >= Self::VH_SUPPORTED
    }

    /// Indicates presence of FEAT_HCX.
    pub fn is_feat_hcx_present(self) -> bool {
        self.hcx() >= Self::HCX_SUPPORTED
    }
}

impl IdAa64mmfr2El1 {
    const CCIDX_64_BIT: u8 = 0b0001;

    /// Checks whether 64-bit format is implemented for all levels of the CCSIDR_EL1.
    pub fn has_64_bit_ccsidr_el1(self) -> bool {
        self.ccidx() == Self::CCIDX_64_BIT
    }
}

impl IdAa64mmfr3El1 {
    const TCRX_SHIFT: u64 = 0;
    const TCRX_MASK: u64 = 0b1111;
    const TCRX_SUPPORTED: u64 = 1;

    /// Indicates presence of FEAT_TCR2.
    pub fn is_feat_tcr2_present(self) -> bool {
        (self.bits() >> Self::TCRX_SHIFT) & Self::TCRX_MASK >= Self::TCRX_SUPPORTED
    }
}

impl MpidrEl1 {
    /// Converts a PSCI MPIDR value into the equivalent `MpidrEL1` value.
    ///
    /// This reads the MT and U bits from the current CPU's MPIDR_EL1 value and combines them with
    /// the affinity values from the given `psci_mpidr`.
    ///
    /// This assumes that the MPIDR_EL1 values of all CPUs in a system have the same values for the
    /// MT and U bits.
    pub fn from_psci_mpidr(psci_mpidr: u64) -> Self {
        let mpidr_el1 = read_mpidr_el1();
        Self::from_bits_retain(psci_mpidr) | (mpidr_el1 & (Self::MT | Self::U))
    }
}

impl SpsrEl1 {
    /// All of the N, Z, C and V bits.
    pub const NZCV: Self = Self::V.union(Self::C).union(Self::Z).union(Self::N);
}

impl SpsrEl2 {
    /// All of the N, Z, C and V bits.
    pub const NZCV: Self = Self::V.union(Self::C).union(Self::Z).union(Self::N);
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
}

/// Cache type enum.
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum CacheType {
    /// No cache.
    NoCache = 0b000,
    /// Instruction cache only.
    InstructionOnly = 0b001,
    /// Data cache only.
    DataOnly = 0b010,
    /// Separate instruction and data caches.
    SeparateInstructionAndData = 0b011,
    /// Unified cache.
    Unified = 0b100,
}

/// Wrapper type for describing cache level in a human readable format, i.e. L3 cache = `CacheLevel(3)`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CacheLevel(pub(crate) u8);

impl CacheLevel {
    /// Creates new instance.
    pub fn new(level: u8) -> Self {
        assert!((1..8).contains(&level));
        Self(level)
    }

    /// Returns the level value.
    pub fn level(&self) -> u8 {
        self.0
    }
}

impl From<CacheLevel> for u64 {
    fn from(value: CacheLevel) -> Self {
        (value.0 - 1).into()
    }
}

/// An AArch64 exception level.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum ExceptionLevel {
    /// Exception level 0.
    El0 = 0,
    /// Exception level 1.
    El1 = 1,
    /// Exception level 2.
    El2 = 2,
    /// Exception level 3.
    El3 = 3,
}

/// Values for SPSEL.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum StackPointer {
    /// Use SP_EL0.
    El0 = 0,
    /// Use SP_EL1, SP_EL2 or SP_EL3 according to the current exception level.
    ElX = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_mpidr_el1() {
        assert_eq!(format!("{:?}", MpidrEl1::empty()), "MpidrEl1(0x0)");
        assert_eq!(
            format!("{:?}", MpidrEl1::MT | MpidrEl1::U),
            "MpidrEl1(MT | U)"
        );
        assert_eq!(
            format!("{:?}", MpidrEl1::from_bits_retain(0x12_4134_5678)),
            "MpidrEl1(MT | U | 0x1200345678)"
        );
    }

    #[test]
    fn debug_spsr_el1() {
        assert_eq!(format!("{:?}", SpsrEl1::empty()), "SpsrEl1(0x0)");
        assert_eq!(format!("{:?}", SpsrEl1::NZCV), "SpsrEl1(V | C | Z | N)");
    }

    #[test]
    fn debug_spsr_el2() {
        assert_eq!(format!("{:?}", SpsrEl2::empty()), "SpsrEl2(0x0)");
        assert_eq!(format!("{:?}", SpsrEl2::NZCV), "SpsrEl2(V | C | Z | N)");
    }

    #[test]
    fn debug_spsr_el3() {
        assert_eq!(format!("{:?}", SpsrEl3::empty()), "SpsrEl3(0x0)");
        assert_eq!(format!("{:?}", SpsrEl3::NZCV), "SpsrEl3(V | C | Z | N)");
        assert_eq!(format!("{:?}", SpsrEl3::M_AARCH64_EL3H), "SpsrEl3(0xd)");
    }

    #[test]
    fn debug_esr_el1() {
        assert_eq!(format!("{:?}", EsrEl1::empty()), "EsrEl1(0x0)");
        assert_eq!(format!("{:?}", EsrEl1::IL), "EsrEl1(0x2000000)");
        assert_eq!(
            format!("{:?}", EsrEl1::ISS_SYSREG_OPCODE_MASK),
            "EsrEl1(0x3ffc1e)"
        );
    }

    #[test]
    fn debug_esr_el2() {
        assert_eq!(format!("{:?}", EsrEl2::empty()), "EsrEl2(0x0)");
        assert_eq!(format!("{:?}", EsrEl2::IL), "EsrEl2(0x2000000)");
        assert_eq!(
            format!("{:?}", EsrEl2::ISS_SYSREG_OPCODE_MASK),
            "EsrEl2(0x3ffc1e)"
        );
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
