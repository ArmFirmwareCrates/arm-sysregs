// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Manually implemented methods for system register types.

#[cfg(feature = "el1")]
use crate::{
    ClidrEl1, CsselrEl1, EsrEl1, IdAa64dfr0El1, IdAa64dfr1El1, IdAa64mmfr0El1, IdAa64mmfr1El1,
    IdAa64mmfr2El1, IdAa64mmfr3El1, IdAa64pfr0El1, IdAa64pfr1El1, MpidrEl1, SpsrEl1,
    read_mpidr_el1,
};
#[cfg(feature = "el2")]
use crate::{EsrEl2, SpsrEl2};
#[cfg(feature = "el3")]
use crate::{EsrEl3, MdcrEl3, SmcrEl3, SpsrEl3};
#[cfg(feature = "el1")]
use core::fmt::{self, Debug, Formatter};
use num_enum::TryFromPrimitive;

#[cfg(feature = "el1")]
impl ClidrEl1 {
    /// Returns the inner cache boundary level.
    pub fn icb_level(self) -> Option<CacheLevel> {
        let icb = self.icb();
        if icb != 0 {
            Some(CacheLevel(icb as u8))
        } else {
            None
        }
    }

    /// Returns Cache Type [1-7] fields.
    pub fn cache_type(self, level: CacheLevel) -> CacheType {
        self.ctype(level.level().into()).try_into().unwrap()
    }
}

#[cfg(feature = "el1")]
impl CsselrEl1 {
    /// Creates new instance. TnD is only valid if FEAT_MTE2 is implemented.
    pub fn new(tnd: bool, level: CacheLevel, ind: bool) -> Self {
        let mut instance = Self::from_bits_retain(u64::from(level) << 1);

        if ind {
            instance |= Self::IND;
        } else if tnd {
            // TnD is only valid if InD is not set.
            instance |= Self::TND;
        }

        instance
    }

    /// Returns the cache level of requested cache.
    pub fn cache_level(self) -> CacheLevel {
        CacheLevel(self.level() + 1)
    }
}

#[cfg(feature = "el1")]
impl EsrEl1 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

#[cfg(feature = "el1")]
impl Debug for EsrEl1 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl1({:#x})", self.0)
    }
}

#[cfg(feature = "el2")]
impl EsrEl2 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

#[cfg(feature = "el2")]
impl Debug for EsrEl2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl2({:#x})", self.0)
    }
}

#[cfg(feature = "el3")]
impl EsrEl3 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
}

#[cfg(feature = "el3")]
impl Debug for EsrEl3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl3({:#x})", self.0)
    }
}

#[cfg(feature = "el1")]
impl IdAa64dfr0El1 {
    const SYS_REG_TRACE_SUPPORTED: u8 = 1;
    const SPE_SUPPORTED: u8 = 1;
    const TRF_SUPPORTED: u8 = 1;
    const TRBE_NOT_SUPPORTED: u8 = 0;
    const MTPMU_SUPPORTED: u8 = 1;

    /// Trace support. Indicates whether System register interface to a PE trace unit is
    /// implemented.
    pub fn is_feat_sys_reg_trace_present(self) -> bool {
        self.tracever() == Self::SYS_REG_TRACE_SUPPORTED
    }

    /// Indicates whether Armv8.1 Statistical Profiling Extension is implemented.
    pub fn is_feat_spe_present(self) -> bool {
        self.pmsver() >= Self::SPE_SUPPORTED
    }

    /// Indicates whether Armv8.4 Self-hosted Trace Extension is implemented.
    pub fn is_feat_trf_present(self) -> bool {
        self.tracefilt() == Self::TRF_SUPPORTED
    }

    /// Indicates whether Trace Buffer Extension is implemented.
    pub fn is_feat_trbe_present(self) -> bool {
        self.tracebuffer() != Self::TRBE_NOT_SUPPORTED
    }

    /// Indicates whether Multi Threaded PMU Extension is implemented.
    pub fn is_feat_mtpmu_present(self) -> bool {
        self.mtpmu() == Self::MTPMU_SUPPORTED
    }
}

#[cfg(feature = "el1")]
impl IdAa64dfr1El1 {
    const EBEP_IMPLEMENTED: u8 = 0b1;

    /// Indicates whether FEAT_EBEP is implemented.
    pub fn is_feat_ebep_present(self) -> bool {
        self.ebep() == Self::EBEP_IMPLEMENTED
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr0El1 {
    const FGT_SUPPORTED: u8 = 0b0001;
    const FGT2_SUPPORTED: u8 = 0b0001;

    /// Indicates whether Fine Grain Traps Extension is implemented.
    pub fn is_feat_fgt_present(self) -> bool {
        let val = self.fgt();
        val == Self::FGT_SUPPORTED || val == Self::FGT2_SUPPORTED
    }

    /// Indicates whether Fine Grain Traps 2 Extension is implemented.
    pub fn is_feat_fgt2_present(self) -> bool {
        self.fgt() == Self::FGT2_SUPPORTED
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl IdAa64mmfr2El1 {
    const CCIDX_64_BIT: u8 = 0b0001;

    /// Checks whether 64-bit format is implemented for all levels of the CCSIDR_EL1.
    pub fn has_64_bit_ccsidr_el1(self) -> bool {
        self.ccidx() == Self::CCIDX_64_BIT
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr3El1 {
    const TCRX_SUPPORTED: u8 = 1;

    /// Indicates presence of FEAT_TCR2.
    pub fn is_feat_tcr2_present(self) -> bool {
        self.tcrx() >= Self::TCRX_SUPPORTED
    }
}

#[cfg(feature = "el1")]
impl IdAa64pfr0El1 {
    const SVE_SUPPORTED: u8 = 1;
    const MPAM_SUPPORTED: u8 = 1;

    /// Indicates whether SVE is implemented.
    pub fn is_feat_sve_present(self) -> bool {
        self.sve() == Self::SVE_SUPPORTED
    }

    /// Indicates whether MPAM Extension is implemented.
    pub fn is_feat_mpam_present(self) -> bool {
        self.mpam() == Self::MPAM_SUPPORTED
    }
}

#[cfg(feature = "el1")]
impl IdAa64pfr1El1 {
    const SSBS_IMPLEMENTED: u8 = 0b1;
    const MTE_IMPLEMENTED: u8 = 0b0001;
    const MTE2_IMPLEMENTED: u8 = 0b0010;
    const SME_IMPLEMENTED: u8 = 0b0001;
    const SME2_IMPLEMENTED: u8 = 0b0010;
    const NMI_IMPLEMENTED: u8 = 0b1;
    const GCS_IMPLEMENTED: u8 = 0b1;

    /// Indicates whether FEAT_SSBS is implemented.
    pub fn is_feat_ssbs_present(self) -> bool {
        self.ssbs() >= Self::SSBS_IMPLEMENTED
    }

    /// Indicates whether FEAT_MTE is implemented.
    pub fn is_feat_mte_present(self) -> bool {
        self.mte() >= Self::MTE_IMPLEMENTED
    }

    /// Indicates whether FEAT_MTE2 is implemented.
    pub fn is_feat_mte2_present(self) -> bool {
        self.mte() >= Self::MTE2_IMPLEMENTED
    }

    /// Indicates whether FEAT_SME is implemented.
    pub fn is_feat_sme_present(self) -> bool {
        self.sme() >= Self::SME_IMPLEMENTED
    }

    /// Indicates whether FEAT_SME2 is implemented.
    pub fn is_feat_sme2_present(self) -> bool {
        self.sme() >= Self::SME2_IMPLEMENTED
    }

    /// Indicates whether FEAT_NMI is implemented.
    pub fn is_feat_nmi_present(self) -> bool {
        self.nmi() == Self::NMI_IMPLEMENTED
    }

    /// Indicates whether FEAT_GCS is implemented.
    pub fn is_feat_gcs_present(self) -> bool {
        self.gcs() == Self::GCS_IMPLEMENTED
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el1")]
impl MpidrEl1 {
    /// Size in bits of the affinity fields.
    pub const AFFINITY_BITS: usize = 8;

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

#[cfg(feature = "el3")]
impl SmcrEl3 {
    /// Build SMCR_EL3 register value from given SSVE vector length.
    pub fn from_ssve_vector_len(vector_length: u64) -> Self {
        Self::from_bits_retain(((vector_length - 1) / 128) & Self::LEN_MASK)
    }
}

#[cfg(feature = "el1")]
impl SpsrEl1 {
    /// All of the N, Z, C and V bits.
    pub const NZCV: Self = Self::V.union(Self::C).union(Self::Z).union(Self::N);
}

#[cfg(feature = "el2")]
impl SpsrEl2 {
    /// All of the N, Z, C and V bits.
    pub const NZCV: Self = Self::V.union(Self::C).union(Self::Z).union(Self::N);
}

#[cfg(feature = "el3")]
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

/// Wrapper type for describing cache level in a human readable format, e.g. L3 cache = `CacheLevel(3)`
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

impl From<CacheLevel> for u32 {
    fn from(value: CacheLevel) -> Self {
        (value.0 - 1).into()
    }
}

impl From<CacheLevel> for u64 {
    fn from(value: CacheLevel) -> Self {
        u32::from(value).into()
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

/// Allowed Shareability attributes.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum Shareability {
    /// Non-shareable.
    Non = 0b00,
    /// Outer-shareable.
    Outer = 0b10,
    /// Inner-shareable.
    Inner = 0b11,
}

/// Allowed Cacheability attributes.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum Cacheability {
    /// Normal memory, Non-cacheable.
    Non = 0b00,
    /// Normal memory, Write-Back Read-Allocate Write-Allocate Cacheable.
    WriteBackAllocate = 0b01,
    /// Normal memory, Write-Through Read-Allocate No Write-Allocate Cacheable.
    WriteThrough = 0b10,
    /// Normal memory, Write-Back Read-Allocate No Write-Allocate Cacheable.
    WriteBackNoAllocate = 0b11,
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "el1")]
    use super::*;

    #[test]
    #[cfg(feature = "el1")]
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

    #[cfg(feature = "el1")]
    #[test]
    fn debug_spsr_el1() {
        assert_eq!(format!("{:?}", SpsrEl1::empty()), "SpsrEl1(0x0)");
        assert_eq!(format!("{:?}", SpsrEl1::NZCV), "SpsrEl1(V | C | Z | N)");
    }

    #[cfg(feature = "el2")]
    #[test]
    fn debug_spsr_el2() {
        assert_eq!(format!("{:?}", SpsrEl2::empty()), "SpsrEl2(0x0)");
        assert_eq!(format!("{:?}", SpsrEl2::NZCV), "SpsrEl2(V | C | Z | N)");
    }

    #[cfg(feature = "el3")]
    #[test]
    fn debug_spsr_el3() {
        assert_eq!(format!("{:?}", SpsrEl3::empty()), "SpsrEl3(0x0)");
        assert_eq!(format!("{:?}", SpsrEl3::NZCV), "SpsrEl3(V | C | Z | N)");
        assert_eq!(format!("{:?}", SpsrEl3::M_AARCH64_EL3H), "SpsrEl3(0xd)");
    }

    #[cfg(feature = "el1")]
    #[test]
    fn debug_esr_el1() {
        assert_eq!(format!("{:?}", EsrEl1::empty()), "EsrEl1(0x0)");
        assert_eq!(format!("{:?}", EsrEl1::IL), "EsrEl1(0x2000000)");
        assert_eq!(
            format!("{:?}", EsrEl1::ISS_SYSREG_OPCODE_MASK),
            "EsrEl1(0x3ffc1e)"
        );
    }

    #[cfg(feature = "el2")]
    #[test]
    fn debug_esr_el2() {
        assert_eq!(format!("{:?}", EsrEl2::empty()), "EsrEl2(0x0)");
        assert_eq!(format!("{:?}", EsrEl2::IL), "EsrEl2(0x2000000)");
        assert_eq!(
            format!("{:?}", EsrEl2::ISS_SYSREG_OPCODE_MASK),
            "EsrEl2(0x3ffc1e)"
        );
    }

    #[cfg(feature = "el3")]
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
