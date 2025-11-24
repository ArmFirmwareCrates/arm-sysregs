// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

#![cfg_attr(not(any(test, feature = "fakes")), no_std)]

#[cfg(not(any(test, feature = "fakes")))]
mod aarch64;
#[cfg(any(test, feature = "fakes"))]
pub mod fake;
mod macros;
mod manual;

use bitflags::bitflags;
pub use manual::*;
#[doc(hidden)]
pub use paste as _paste;

bitflags! {
    /// `APIAKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyhiEl1: u64 {
    }
}

impl ApiakeyhiEl1 {
    /// Returns the value of the `APIAKeyHi` field.
    pub const fn apiakeyhi(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `APIAKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyloEl1: u64 {
    }
}

impl ApiakeyloEl1 {
    /// Returns the value of the `APIAKeyLo` field.
    pub const fn apiakeylo(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `CCSIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CcsidrEl1: u64 {
    }
}

impl CcsidrEl1 {
    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        ((self.bits() >> 0) & 0b111) as u8
    }
}

bitflags! {
    /// `CLIDR_EL1` system register value.
    ///
    /// Cache Level ID.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ClidrEl1: u64 {
    }
}

impl ClidrEl1 {
    /// Returns the value of the given `Ctype<n>` field.
    pub fn ctype(self, n: u32) -> crate::manual::CacheType {
        assert!(n >= 1 && n < 8);
        crate::manual::CacheType::try_from(((self.bits() >> (0 + (n - 1) * 3)) & 0b111) as u8).unwrap()
    }

    /// Returns the value of the `LoUIS` field.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn louis(self) -> u8 {
        ((self.bits() >> 21) & 0b111) as u8
    }

    /// Returns the value of the `LoC` field.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn loc(self) -> u8 {
        ((self.bits() >> 24) & 0b111) as u8
    }

    /// Returns the value of the `LoUU` field.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn louu(self) -> u8 {
        ((self.bits() >> 27) & 0b111) as u8
    }

    /// Returns the value of the `ICB` field.
    ///
    /// Inner cache boundary level.
    pub const fn icb(self) -> u8 {
        ((self.bits() >> 30) & 0b111) as u8
    }

    /// Returns the value of the given `Ttype<n>` field.
    pub const fn ttype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        ((self.bits() >> (33 + (n - 1) * 2)) & 0b11) as u8
    }
}

bitflags! {
    /// `CNTFRQ_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntfrqEl0: u64 {
    }
}

impl CntfrqEl0 {
    /// Returns the value of the `ClockFreq` field.
    pub const fn clockfreq(self) -> u32 {
        ((self.bits() >> 0) & 0b11111111111111111111111111111111) as u32
    }
}

bitflags! {
    /// `CNTHCTL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthctlEl2: u64 {
        /// `EL0PCTEN` bit.
        const EL0PCTEN = 1 << 0;
        /// `EL0VCTEN` bit.
        const EL0VCTEN = 1 << 1;
        /// `EL1PCEN` bit.
        const EL1PCEN = 1 << 1;
        /// `EVNTEN` bit.
        const EVNTEN = 1 << 2;
        /// `EVNTDIR` bit.
        const EVNTDIR = 1 << 3;
        /// `EL0VTEN` bit.
        const EL0VTEN = 1 << 8;
        /// `EL0PTEN` bit.
        const EL0PTEN = 1 << 9;
        /// `EL1PTEN` bit.
        const EL1PTEN = 1 << 11;
        /// `ECV` bit.
        const ECV = 1 << 12;
        /// `EL1TVT` bit.
        const EL1TVT = 1 << 13;
        /// `EL1TVCT` bit.
        const EL1TVCT = 1 << 14;
        /// `EL1NVPCT` bit.
        const EL1NVPCT = 1 << 15;
        /// `EL1NVVCT` bit.
        const EL1NVVCT = 1 << 16;
        /// `EVNTIS` bit.
        const EVNTIS = 1 << 17;
        /// `CNTVMASK` bit.
        const CNTVMASK = 1 << 18;
        /// `CNTPMASK` bit.
        const CNTPMASK = 1 << 19;
    }
}

impl CnthctlEl2 {
    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }
}

bitflags! {
    /// `CNTVOFF_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvoffEl2: u64 {
    }
}

impl CntvoffEl2 {
    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `CONTEXTIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl1: u64 {
    }
}

impl ContextidrEl1 {
    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        ((self.bits() >> 0) & 0b11111111111111111111111111111111) as u32
    }
}

bitflags! {
    /// `CONTEXTIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl2: u64 {
    }
}

impl ContextidrEl2 {
    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        ((self.bits() >> 0) & 0b11111111111111111111111111111111) as u32
    }
}

bitflags! {
    /// `CPACR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CpacrEl1: u64 {
        /// `TTA` bit.
        const TTA = 1 << 28;
        /// `E0POE` bit.
        const E0POE = 1 << 29;
        /// `TAM` bit.
        const TAM = 1 << 30;
        /// `TCPAC` bit.
        const TCPAC = 1 << 31;
        /// `E0TP0E` bit.
        const E0TP0E = 1 << 32;
        /// `E0TP1E` bit.
        const E0TP1E = 1 << 33;
    }
}

impl CpacrEl1 {
    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        ((self.bits() >> 16) & 0b11) as u8
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        ((self.bits() >> 20) & 0b11) as u8
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        ((self.bits() >> 24) & 0b11) as u8
    }
}

bitflags! {
    /// `CPTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CptrEl2: u64 {
        /// RES1 bits in the `CPTR_EL2` register.
        const RES1 = 0b10001011111111;
        /// `TZ` bit.
        const TZ = 1 << 8;
        /// `TFP` bit.
        const TFP = 1 << 10;
        /// `TSM` bit.
        const TSM = 1 << 12;
        /// `E0POE` bit.
        const E0POE = 1 << 29;
        /// `TAM` bit.
        const TAM = 1 << 30;
        /// `TCPAC` bit.
        const TCPAC = 1 << 31;
        /// `E0TP0E` bit.
        const E0TP0E = 1 << 32;
        /// `E0TP1E` bit.
        const E0TP1E = 1 << 33;
    }
}

impl CptrEl2 {
    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        ((self.bits() >> 16) & 0b11) as u8
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        ((self.bits() >> 20) & 0b11) as u8
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        ((self.bits() >> 24) & 0b11) as u8
    }
}

bitflags! {
    /// `CPTR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CptrEl3: u64 {
        /// Do not trap execution of SVE instructions.
        const EZ = 1 << 8;
        /// Trap Advanced SIMD instructions execution.
        const TFP = 1 << 10;
        /// When FEAT_SME is implemented, do not trap SME instructions and system registers accesses.
        const ESM = 1 << 12;
        /// Trap trace system register accesses.
        const TTA = 1 << 20;
        /// When FEAT_AMUv1 implemented trap accesses from EL2/EL1/EL0 to AMU registers.
        const TAM = 1 << 30;
        /// Trap EL2 accesses to CPTR_EL2/HCPTR, and EL2/EL1 accesses to CPACR_EL1/CPACR.
        const TCPAC = 1 << 31;
    }
}

bitflags! {
    /// `CSSELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CsselrEl1: u64 {
        /// Instruction not Data bit.
        const IND = 1 << 0;
        /// Allocation Tag not Data bit, only valid if FEAT_MTE2 is implemented.
        const TND = 1 << 4;
    }
}

impl CsselrEl1 {
    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        ((self.bits() >> 1) & 0b111) as u8
    }
}

bitflags! {
    /// `CTR_EL0` system register value.
    ///
    /// Cache Type Register.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CtrEl0: u64 {
        /// RES1 bits in the `CTR_EL0` register.
        const RES1 = 0b10000000000000000000000000000000;
        /// `IDC` bit.
        const IDC = 1 << 28;
        /// `DIC` bit.
        const DIC = 1 << 29;
    }
}

impl CtrEl0 {
    /// Returns the value of the `IminLine` field.
    pub const fn iminline(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `L1Ip` field.
    pub const fn l1ip(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }

    /// Returns the value of the `DminLine` field.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn dminline(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `ERG` field.
    pub const fn erg(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `CWG` field.
    pub const fn cwg(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `TminLine` field.
    pub const fn tminline(self) -> u8 {
        ((self.bits() >> 32) & 0b111111) as u8
    }
}

bitflags! {
    /// `DISR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DisrEl1: u64 {
        /// `WnR` bit.
        const WNR = 1 << 6;
        /// `WnRV` bit.
        const WNRV = 1 << 7;
        /// `EA` bit.
        const EA = 1 << 9;
        /// `IDS` bit.
        const IDS = 1 << 24;
        /// `A` bit.
        const A = 1 << 31;
    }
}

impl DisrEl1 {
    /// Returns the value of the `DFSC` field.
    pub const fn dfsc(self) -> u8 {
        ((self.bits() >> 0) & 0b111111) as u8
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> 10) & 0b111) as u8
    }

    /// Returns the value of the `WU` field.
    pub const fn wu(self) -> u8 {
        ((self.bits() >> 16) & 0b11) as u8
    }
}

bitflags! {
    /// `DIT` system register value.
    ///
    /// Data Independent Timing.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dit: u64 {
        /// Enable data independent timing.
        const DIT = 1 << 24;
    }
}

bitflags! {
    /// `ELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl1: u64 {
    }
}

impl ElrEl1 {
    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `ELR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl2: u64 {
    }
}

impl ElrEl2 {
    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `ESR_EL1` system register value.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl1: u64 {
        /// `IL` bit.
        const IL = 1 << 25;
    }
}

impl EsrEl1 {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> 0) & 0b1111111111111111111111111) as u32
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> 26) & 0b111111) as u8
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> 32) & 0b111111111111111111111111) as u32
    }
}

bitflags! {
    /// `ESR_EL2` system register value.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl2: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

impl EsrEl2 {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> 0) & 0b1111111111111111111111111) as u32
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> 26) & 0b111111) as u8
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> 32) & 0b111111111111111111111111) as u32
    }
}

bitflags! {
    /// `ESR_EL3` system register value.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl3: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

impl EsrEl3 {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> 0) & 0b1111111111111111111111111) as u32
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> 26) & 0b111111) as u8
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> 32) & 0b111111111111111111111111) as u32
    }
}

bitflags! {
    /// `FAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl1: u64 {
    }
}

impl FarEl1 {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `FAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl2: u64 {
    }
}

impl FarEl2 {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `GCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcrEl1: u64 {
        /// `RRND` bit.
        const RRND = 1 << 16;
    }
}

impl GcrEl1 {
    /// Returns the value of the `Exclude` field.
    pub const fn exclude(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `GCSCR_EL1` system register value.
    ///
    /// Guarded Control Stack Control register.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcscrEl1: u64 {
        /// `PCRSEL` bit.
        const PCRSEL = 1 << 0;
        /// `RVCHKEN` bit.
        const RVCHKEN = 1 << 5;
        /// Exception state lock enable.
        const EXLOCKEN = 1 << 6;
        /// `PUSHMEn` bit.
        const PUSHMEN = 1 << 8;
        /// `STREn` bit.
        const STREN = 1 << 9;
    }
}

bitflags! {
    /// `GCSCR_EL2` system register value.
    ///
    /// Guarded Control Stack Control register.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcscrEl2: u64 {
        /// `PCRSEL` bit.
        const PCRSEL = 1 << 0;
        /// `RVCHKEN` bit.
        const RVCHKEN = 1 << 5;
        /// Exception state lock enable.
        const EXLOCKEN = 1 << 6;
        /// `PUSHMEn` bit.
        const PUSHMEN = 1 << 8;
        /// `STREn` bit.
        const STREN = 1 << 9;
    }
}

bitflags! {
    /// `HCRX_EL2` system register value.
    ///
    /// Extended Hypervisor Configuration Register.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HcrxEl2: u64 {
        /// Do not trap execution of an ST64BV0 instruction at EL0 or EL1 to EL2.
        const ENAS0 = 1 << 0;
        /// Do not trap execution of an LD64B or ST64B instruction at EL0 or EL1 to EL2.
        const ENALS = 1 << 1;
        /// Do not trap execution of an ST64BV instruction at EL0 or EL1 to EL2.
        const ENASR = 1 << 2;
        /// Determines the behavior of TLBI instructions affected by the XS attribute.
        const FNXS = 1 << 3;
        /// Determines if the fine-grained traps in HFGITR_EL2 also apply to the corresponding TLBI maintenance instructions with the nXS qualifier.
        const FGTNXS = 1 << 4;
        /// Controls mapping of the value of SMPRI_EL1.Priority for streaming execution priority at EL0 or EL1.
        const SMPME = 1 << 5;
        /// Traps MSR writes of ALLINT at EL1 using AArch64 to EL2.
        const TALLINT = 1 << 6;
        /// Enables signaling of virtual IRQ interrupts with Superpriority.
        const VINMI = 1 << 7;
        /// Enables signaling of virtual FIQ interrupts with Superpriority.
        const VFNMI = 1 << 8;
        /// Controls the required permissions for cache maintenance instructions at EL1 or EL0.
        const CMOW = 1 << 9;
        /// Controls Memory Copy and Memory Set exceptions generated from EL1.
        const MCE2 = 1 << 10;
        /// Enables execution of Memory Set and Memory Copy instructions at EL1 or EL0.
        const MSCEN = 1 << 11;
        /// `TCR2En` bit.
        const TCR2EN = 1 << 14;
        /// `SCTLR2En` bit.
        const SCTLR2EN = 1 << 15;
        /// `PTTWI` bit.
        const PTTWI = 1 << 16;
        /// `D128En` bit.
        const D128EN = 1 << 17;
        /// `EnSNERR` bit.
        const ENSNERR = 1 << 18;
        /// `TMEA` bit.
        const TMEA = 1 << 19;
        /// `EnSDERR` bit.
        const ENSDERR = 1 << 20;
        /// `EnIDCP128` bit.
        const ENIDCP128 = 1 << 21;
        /// `GCSEn` bit.
        const GCSEN = 1 << 22;
        /// `EnFPM` bit.
        const ENFPM = 1 << 23;
        /// `PACMEn` bit.
        const PACMEN = 1 << 24;
        /// `VTLBIDEn` bit.
        const VTLBIDEN = 1 << 25;
        /// `SRMASKEn` bit.
        const SRMASKEN = 1 << 26;
        /// `NVTGE` bit.
        const NVTGE = 1 << 27;
        /// `POE2En` bit.
        const POE2EN = 1 << 29;
        /// `TPLIMEn` bit.
        const TPLIMEN = 1 << 30;
        /// `FDIT` bit.
        const FDIT = 1 << 31;
        /// `NVnTTLB` bit.
        const NVNTTLB = 1 << 32;
        /// `NVnTTLBIS` bit.
        const NVNTTLBIS = 1 << 33;
        /// `NVnTTLBOS` bit.
        const NVNTTLBOS = 1 << 34;
        /// `VTLBIDOSEn` bit.
        const VTLBIDOSEN = 1 << 35;
        /// `FNB` bit.
        const FNB = 1 << 36;
        /// `VTE` bit.
        const VTE = 1 << 37;
        /// `VTAO` bit.
        const VTAO = 1 << 38;
        /// `VTCO` bit.
        const VTCO = 1 << 39;
    }
}

bitflags! {
    /// `HCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HcrEl2: u64 {
        /// `VM` bit.
        const VM = 1 << 0;
        /// `SWIO` bit.
        const SWIO = 1 << 1;
        /// `PTW` bit.
        const PTW = 1 << 2;
        /// `FMO` bit.
        const FMO = 1 << 3;
        /// `IMO` bit.
        const IMO = 1 << 4;
        /// `AMO` bit.
        const AMO = 1 << 5;
        /// `VF` bit.
        const VF = 1 << 6;
        /// `VI` bit.
        const VI = 1 << 7;
        /// `VSE` bit.
        const VSE = 1 << 8;
        /// `FB` bit.
        const FB = 1 << 9;
        /// `DC` bit.
        const DC = 1 << 12;
        /// `TWI` bit.
        const TWI = 1 << 13;
        /// `TWE` bit.
        const TWE = 1 << 14;
        /// `TID0` bit.
        const TID0 = 1 << 15;
        /// `TID1` bit.
        const TID1 = 1 << 16;
        /// `TID2` bit.
        const TID2 = 1 << 17;
        /// `TID3` bit.
        const TID3 = 1 << 18;
        /// `TSC` bit.
        const TSC = 1 << 19;
        /// `TIDCP` bit.
        const TIDCP = 1 << 20;
        /// `TACR` bit.
        const TACR = 1 << 21;
        /// `TSW` bit.
        const TSW = 1 << 22;
        /// `TPCP` bit.
        const TPCP = 1 << 23;
        /// `TPU` bit.
        const TPU = 1 << 24;
        /// `TTLB` bit.
        const TTLB = 1 << 25;
        /// `TVM` bit.
        const TVM = 1 << 26;
        /// Trap general exceptions to EL2.
        const TGE = 1 << 27;
        /// `TDZ` bit.
        const TDZ = 1 << 28;
        /// `HCD` bit.
        const HCD = 1 << 29;
        /// `TRVM` bit.
        const TRVM = 1 << 30;
        /// `RW` bit.
        const RW = 1 << 31;
        /// `CD` bit.
        const CD = 1 << 32;
        /// `ID` bit.
        const ID = 1 << 33;
        /// `E2H` bit.
        const E2H = 1 << 34;
        /// `TLOR` bit.
        const TLOR = 1 << 35;
        /// `TERR` bit.
        const TERR = 1 << 36;
        /// `TEA` bit.
        const TEA = 1 << 37;
        /// `APK` bit.
        const APK = 1 << 40;
        /// `API` bit.
        const API = 1 << 41;
        /// `NV` bit.
        const NV = 1 << 42;
        /// `NV1` bit.
        const NV1 = 1 << 43;
        /// `AT` bit.
        const AT = 1 << 44;
        /// `NV2` bit.
        const NV2 = 1 << 45;
        /// `FWB` bit.
        const FWB = 1 << 46;
        /// `FIEN` bit.
        const FIEN = 1 << 47;
        /// `GPF` bit.
        const GPF = 1 << 48;
        /// `TID4` bit.
        const TID4 = 1 << 49;
        /// `TICAB` bit.
        const TICAB = 1 << 50;
        /// `AMVOFFEN` bit.
        const AMVOFFEN = 1 << 51;
        /// `TOCU` bit.
        const TOCU = 1 << 52;
        /// `EnSCXT` bit.
        const ENSCXT = 1 << 53;
        /// `TTLBIS` bit.
        const TTLBIS = 1 << 54;
        /// `TTLBOS` bit.
        const TTLBOS = 1 << 55;
        /// `ATA` bit.
        const ATA = 1 << 56;
        /// `DCT` bit.
        const DCT = 1 << 57;
        /// `TID5` bit.
        const TID5 = 1 << 58;
        /// `TWEDEn` bit.
        const TWEDEN = 1 << 59;
    }
}

impl HcrEl2 {
    /// Returns the value of the `BSU` field.
    pub const fn bsu(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `HDFGRTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdfgrtr2El2: u64 {
        /// `nPMECR_EL1` bit.
        const NPMECR_EL1 = 1 << 0;
        /// `nPMIAR_EL1` bit.
        const NPMIAR_EL1 = 1 << 1;
        /// `nPMICNTR_EL0` bit.
        const NPMICNTR_EL0 = 1 << 2;
        /// `nPMICFILTR_EL0` bit.
        const NPMICFILTR_EL0 = 1 << 3;
        /// `nPMUACR_EL1` bit.
        const NPMUACR_EL1 = 1 << 4;
        /// `nMDSELR_EL1` bit.
        const NMDSELR_EL1 = 1 << 5;
        /// `nPMSSDATA` bit.
        const NPMSSDATA = 1 << 6;
        /// `nPMSSCR_EL1` bit.
        const NPMSSCR_EL1 = 1 << 7;
        /// `nSPMEVCNTRn_EL0` bit.
        const NSPMEVCNTRN_EL0 = 1 << 8;
        /// `nSPMEVTYPERn_EL0` bit.
        const NSPMEVTYPERN_EL0 = 1 << 9;
        /// `nSPMSELR_EL0` bit.
        const NSPMSELR_EL0 = 1 << 10;
        /// `nSPMCNTEN` bit.
        const NSPMCNTEN = 1 << 11;
        /// `nSPMINTEN` bit.
        const NSPMINTEN = 1 << 12;
        /// `nSPMOVS` bit.
        const NSPMOVS = 1 << 13;
        /// `nSPMCR_EL0` bit.
        const NSPMCR_EL0 = 1 << 14;
        /// `nSPMACCESSR_EL1` bit.
        const NSPMACCESSR_EL1 = 1 << 15;
        /// `nSPMSCR_EL1` bit.
        const NSPMSCR_EL1 = 1 << 16;
        /// `nSPMID` bit.
        const NSPMID = 1 << 17;
        /// `nSPMDEVAFF_EL1` bit.
        const NSPMDEVAFF_EL1 = 1 << 18;
        /// `nPMSDSFR_EL1` bit.
        const NPMSDSFR_EL1 = 1 << 19;
        /// `nTRCITECR_EL1` bit.
        const NTRCITECR_EL1 = 1 << 20;
        /// `nTRBMPAM_EL1` bit.
        const NTRBMPAM_EL1 = 1 << 22;
        /// `nMDSTEPOP_EL1` bit.
        const NMDSTEPOP_EL1 = 1 << 23;
        /// `nPMBMAR_EL1` bit.
        const NPMBMAR_EL1 = 1 << 24;
    }
}

bitflags! {
    /// `HDFGWTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdfgwtr2El2: u64 {
        /// `nPMECR_EL1` bit.
        const NPMECR_EL1 = 1 << 0;
        /// `nPMIAR_EL1` bit.
        const NPMIAR_EL1 = 1 << 1;
        /// `nPMICNTR_EL0` bit.
        const NPMICNTR_EL0 = 1 << 2;
        /// `nPMICFILTR_EL0` bit.
        const NPMICFILTR_EL0 = 1 << 3;
        /// `nPMUACR_EL1` bit.
        const NPMUACR_EL1 = 1 << 4;
        /// `nMDSELR_EL1` bit.
        const NMDSELR_EL1 = 1 << 5;
        /// `nPMSSCR_EL1` bit.
        const NPMSSCR_EL1 = 1 << 7;
        /// `nSPMEVCNTRn_EL0` bit.
        const NSPMEVCNTRN_EL0 = 1 << 8;
        /// `nSPMEVTYPERn_EL0` bit.
        const NSPMEVTYPERN_EL0 = 1 << 9;
        /// `nSPMSELR_EL0` bit.
        const NSPMSELR_EL0 = 1 << 10;
        /// `nSPMCNTEN` bit.
        const NSPMCNTEN = 1 << 11;
        /// `nSPMINTEN` bit.
        const NSPMINTEN = 1 << 12;
        /// `nSPMOVS` bit.
        const NSPMOVS = 1 << 13;
        /// `nSPMCR_EL0` bit.
        const NSPMCR_EL0 = 1 << 14;
        /// `nSPMACCESSR_EL1` bit.
        const NSPMACCESSR_EL1 = 1 << 15;
        /// `nSPMSCR_EL1` bit.
        const NSPMSCR_EL1 = 1 << 16;
        /// `nPMSDSFR_EL1` bit.
        const NPMSDSFR_EL1 = 1 << 19;
        /// `nTRCITECR_EL1` bit.
        const NTRCITECR_EL1 = 1 << 20;
        /// `nPMZR_EL0` bit.
        const NPMZR_EL0 = 1 << 21;
        /// `nTRBMPAM_EL1` bit.
        const NTRBMPAM_EL1 = 1 << 22;
        /// `nMDSTEPOP_EL1` bit.
        const NMDSTEPOP_EL1 = 1 << 23;
        /// `nPMBMAR_EL1` bit.
        const NPMBMAR_EL1 = 1 << 24;
    }
}

bitflags! {
    /// `HFGITR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hfgitr2El2: u64 {
        /// `TSBCSYNC` bit.
        const TSBCSYNC = 1 << 0;
        /// `nDCCIVAPS` bit.
        const NDCCIVAPS = 1 << 1;
        /// `PLBIPERME1OS` bit.
        const PLBIPERME1OS = 1 << 2;
        /// `PLBIASIDE1OS` bit.
        const PLBIASIDE1OS = 1 << 3;
        /// `PLBIVMALLE1OS` bit.
        const PLBIVMALLE1OS = 1 << 4;
        /// `PLBIPERME1IS` bit.
        const PLBIPERME1IS = 1 << 5;
        /// `PLBIASIDE1IS` bit.
        const PLBIASIDE1IS = 1 << 6;
        /// `PLBIVMALLE1IS` bit.
        const PLBIVMALLE1IS = 1 << 7;
        /// `PLBIPERME1` bit.
        const PLBIPERME1 = 1 << 8;
        /// `PLBIASIDE1` bit.
        const PLBIASIDE1 = 1 << 9;
        /// `PLBIVMALLE1` bit.
        const PLBIVMALLE1 = 1 << 10;
        /// `PLBIPERMAE1OS` bit.
        const PLBIPERMAE1OS = 1 << 11;
        /// `PLBIPERMAE1IS` bit.
        const PLBIPERMAE1IS = 1 << 12;
        /// `PLBIPERMAE1` bit.
        const PLBIPERMAE1 = 1 << 13;
        /// `DCGBVA` bit.
        const DCGBVA = 1 << 14;
    }
}

bitflags! {
    /// `HFGRTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hfgrtr2El2: u64 {
        /// `nPFAR_EL1` bit.
        const NPFAR_EL1 = 1 << 0;
        /// `nERXGSR_EL1` bit.
        const NERXGSR_EL1 = 1 << 1;
        /// `nRCWSMASK_EL1` bit.
        const NRCWSMASK_EL1 = 1 << 2;
        /// `nCPACRMASK_EL1` bit.
        const NCPACRMASK_EL1 = 1 << 3;
        /// `nSCTLRMASK_EL1` bit.
        const NSCTLRMASK_EL1 = 1 << 4;
        /// `nSCTLR2MASK_EL1` bit.
        const NSCTLR2MASK_EL1 = 1 << 5;
        /// `nTCRMASK_EL1` bit.
        const NTCRMASK_EL1 = 1 << 6;
        /// `nTCR2MASK_EL1` bit.
        const NTCR2MASK_EL1 = 1 << 7;
        /// `nCPACRALIAS_EL1` bit.
        const NCPACRALIAS_EL1 = 1 << 8;
        /// `nSCTLRALIAS_EL1` bit.
        const NSCTLRALIAS_EL1 = 1 << 9;
        /// `nSCTLR2ALIAS_EL1` bit.
        const NSCTLR2ALIAS_EL1 = 1 << 10;
        /// `nTCRALIAS_EL1` bit.
        const NTCRALIAS_EL1 = 1 << 11;
        /// `nTCR2ALIAS_EL1` bit.
        const NTCR2ALIAS_EL1 = 1 << 12;
        /// `nACTLRMASK_EL1` bit.
        const NACTLRMASK_EL1 = 1 << 13;
        /// `nACTLRALIAS_EL1` bit.
        const NACTLRALIAS_EL1 = 1 << 14;
        /// `nTINDEX_EL0` bit.
        const NTINDEX_EL0 = 1 << 15;
        /// `nTINDEX_EL1` bit.
        const NTINDEX_EL1 = 1 << 16;
        /// `nSTINDEX_EL1` bit.
        const NSTINDEX_EL1 = 1 << 17;
        /// `nTTTBRP_EL1` bit.
        const NTTTBRP_EL1 = 1 << 20;
        /// `nTTTBRU_EL1` bit.
        const NTTTBRU_EL1 = 1 << 21;
        /// `nIRTBRP_EL1` bit.
        const NIRTBRP_EL1 = 1 << 22;
        /// `nIRTBRU_EL1` bit.
        const NIRTBRU_EL1 = 1 << 23;
        /// `nDPOTBR1_EL1` bit.
        const NDPOTBR1_EL1 = 1 << 24;
        /// `nDPOTBR0_EL1` bit.
        const NDPOTBR0_EL1 = 1 << 25;
        /// `nTPMIN1_EL1` bit.
        const NTPMIN1_EL1 = 1 << 26;
        /// `nTPMIN0_EL1` bit.
        const NTPMIN0_EL1 = 1 << 27;
        /// `nTPMIN1_EL0` bit.
        const NTPMIN1_EL0 = 1 << 28;
        /// `nTPMIN0_EL0` bit.
        const NTPMIN0_EL0 = 1 << 29;
        /// `nTLBIDIDR_EL1` bit.
        const NTLBIDIDR_EL1 = 1 << 30;
        /// `TFSR_EL1` bit.
        const TFSR_EL1 = 1 << 33;
        /// `RGSR_EL1` bit.
        const RGSR_EL1 = 1 << 34;
        /// `GCR_EL1` bit.
        const GCR_EL1 = 1 << 35;
        /// `nTPIDR3_EL0` bit.
        const NTPIDR3_EL0 = 1 << 36;
        /// `nTPIDR3_EL1` bit.
        const NTPIDR3_EL1 = 1 << 37;
    }
}

impl Hfgrtr2El2 {
    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        ((self.bits() >> 18) & 0b11) as u8
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        ((self.bits() >> 31) & 0b11) as u8
    }
}

bitflags! {
    /// `HFGWTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hfgwtr2El2: u64 {
        /// `nPFAR_EL1` bit.
        const NPFAR_EL1 = 1 << 0;
        /// `nRCWSMASK_EL1` bit.
        const NRCWSMASK_EL1 = 1 << 2;
        /// `nCPACRMASK_EL1` bit.
        const NCPACRMASK_EL1 = 1 << 3;
        /// `nSCTLRMASK_EL1` bit.
        const NSCTLRMASK_EL1 = 1 << 4;
        /// `nSCTLR2MASK_EL1` bit.
        const NSCTLR2MASK_EL1 = 1 << 5;
        /// `nTCRMASK_EL1` bit.
        const NTCRMASK_EL1 = 1 << 6;
        /// `nTCR2MASK_EL1` bit.
        const NTCR2MASK_EL1 = 1 << 7;
        /// `nCPACRALIAS_EL1` bit.
        const NCPACRALIAS_EL1 = 1 << 8;
        /// `nSCTLRALIAS_EL1` bit.
        const NSCTLRALIAS_EL1 = 1 << 9;
        /// `nSCTLR2ALIAS_EL1` bit.
        const NSCTLR2ALIAS_EL1 = 1 << 10;
        /// `nTCRALIAS_EL1` bit.
        const NTCRALIAS_EL1 = 1 << 11;
        /// `nTCR2ALIAS_EL1` bit.
        const NTCR2ALIAS_EL1 = 1 << 12;
        /// `nACTLRMASK_EL1` bit.
        const NACTLRMASK_EL1 = 1 << 13;
        /// `nACTLRALIAS_EL1` bit.
        const NACTLRALIAS_EL1 = 1 << 14;
        /// `nTINDEX_EL0` bit.
        const NTINDEX_EL0 = 1 << 15;
        /// `nTINDEX_EL1` bit.
        const NTINDEX_EL1 = 1 << 16;
        /// `nSTINDEX_EL1` bit.
        const NSTINDEX_EL1 = 1 << 17;
        /// `nTTTBRP_EL1` bit.
        const NTTTBRP_EL1 = 1 << 20;
        /// `nTTTBRU_EL1` bit.
        const NTTTBRU_EL1 = 1 << 21;
        /// `nIRTBRP_EL1` bit.
        const NIRTBRP_EL1 = 1 << 22;
        /// `nIRTBRU_EL1` bit.
        const NIRTBRU_EL1 = 1 << 23;
        /// `nDPOTBR1_EL1` bit.
        const NDPOTBR1_EL1 = 1 << 24;
        /// `nDPOTBR0_EL1` bit.
        const NDPOTBR0_EL1 = 1 << 25;
        /// `nTPMIN1_EL1` bit.
        const NTPMIN1_EL1 = 1 << 26;
        /// `nTPMIN0_EL1` bit.
        const NTPMIN0_EL1 = 1 << 27;
        /// `nTPMIN1_EL0` bit.
        const NTPMIN1_EL0 = 1 << 28;
        /// `nTPMIN0_EL0` bit.
        const NTPMIN0_EL0 = 1 << 29;
        /// `TFSR_EL1` bit.
        const TFSR_EL1 = 1 << 33;
        /// `RGSR_EL1` bit.
        const RGSR_EL1 = 1 << 34;
        /// `GCR_EL1` bit.
        const GCR_EL1 = 1 << 35;
        /// `nTPIDR3_EL0` bit.
        const NTPIDR3_EL0 = 1 << 36;
        /// `nTPIDR3_EL1` bit.
        const NTPIDR3_EL1 = 1 << 37;
    }
}

impl Hfgwtr2El2 {
    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        ((self.bits() >> 18) & 0b11) as u8
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        ((self.bits() >> 31) & 0b11) as u8
    }
}

bitflags! {
    /// `HFGWTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HfgwtrEl2: u64 {
        /// `AFSR0_EL1` bit.
        const AFSR0_EL1 = 1 << 0;
        /// `AFSR1_EL1` bit.
        const AFSR1_EL1 = 1 << 1;
        /// `AMAIR_EL1` bit.
        const AMAIR_EL1 = 1 << 3;
        /// `APDAKey` bit.
        const APDAKEY = 1 << 4;
        /// `APDBKey` bit.
        const APDBKEY = 1 << 5;
        /// `APGAKey` bit.
        const APGAKEY = 1 << 6;
        /// `APIAKey` bit.
        const APIAKEY = 1 << 7;
        /// `APIBKey` bit.
        const APIBKEY = 1 << 8;
        /// `CONTEXTIDR_EL1` bit.
        const CONTEXTIDR_EL1 = 1 << 11;
        /// `CPACR_EL1` bit.
        const CPACR_EL1 = 1 << 12;
        /// `CSSELR_EL1` bit.
        const CSSELR_EL1 = 1 << 13;
        /// `ESR_EL1` bit.
        const ESR_EL1 = 1 << 16;
        /// `FAR_EL1` bit.
        const FAR_EL1 = 1 << 17;
        /// `LORC_EL1` bit.
        const LORC_EL1 = 1 << 19;
        /// `LOREA_EL1` bit.
        const LOREA_EL1 = 1 << 20;
        /// `LORN_EL1` bit.
        const LORN_EL1 = 1 << 22;
        /// `LORSA_EL1` bit.
        const LORSA_EL1 = 1 << 23;
        /// `MAIR_EL1` bit.
        const MAIR_EL1 = 1 << 24;
        /// `PAR_EL1` bit.
        const PAR_EL1 = 1 << 27;
        /// `SCTLR_EL1` bit.
        const SCTLR_EL1 = 1 << 29;
        /// `SCXTNUM_EL1` bit.
        const SCXTNUM_EL1 = 1 << 30;
        /// `SCXTNUM_EL0` bit.
        const SCXTNUM_EL0 = 1 << 31;
        /// `TCR_EL1` bit.
        const TCR_EL1 = 1 << 32;
        /// `TPIDR_EL1` bit.
        const TPIDR_EL1 = 1 << 33;
        /// `TPIDRRO_EL0` bit.
        const TPIDRRO_EL0 = 1 << 34;
        /// `TPIDR_EL0` bit.
        const TPIDR_EL0 = 1 << 35;
        /// `TTBR0_EL1` bit.
        const TTBR0_EL1 = 1 << 36;
        /// `TTBR1_EL1` bit.
        const TTBR1_EL1 = 1 << 37;
        /// `VBAR_EL1` bit.
        const VBAR_EL1 = 1 << 38;
        /// `ICC_IGRPENn_EL1` bit.
        const ICC_IGRPENN_EL1 = 1 << 39;
        /// `ERRSELR_EL1` bit.
        const ERRSELR_EL1 = 1 << 41;
        /// `ERXCTLR_EL1` bit.
        const ERXCTLR_EL1 = 1 << 43;
        /// `ERXSTATUS_EL1` bit.
        const ERXSTATUS_EL1 = 1 << 44;
        /// `ERXMISCn_EL1` bit.
        const ERXMISCN_EL1 = 1 << 45;
        /// `ERXPFGCTL_EL1` bit.
        const ERXPFGCTL_EL1 = 1 << 47;
        /// `ERXPFGCDN_EL1` bit.
        const ERXPFGCDN_EL1 = 1 << 48;
        /// `ERXADDR_EL1` bit.
        const ERXADDR_EL1 = 1 << 49;
        /// `nACCDATA_EL1` bit.
        const NACCDATA_EL1 = 1 << 50;
        /// `nGCS_EL0` bit.
        const NGCS_EL0 = 1 << 52;
        /// `nGCS_EL1` bit.
        const NGCS_EL1 = 1 << 53;
        /// `nSMPRI_EL1` bit.
        const NSMPRI_EL1 = 1 << 54;
        /// `nTPIDR2_EL0` bit.
        const NTPIDR2_EL0 = 1 << 55;
        /// `nRCWMASK_EL1` bit.
        const NRCWMASK_EL1 = 1 << 56;
        /// `nPIRE0_EL1` bit.
        const NPIRE0_EL1 = 1 << 57;
        /// `nPIR_EL1` bit.
        const NPIR_EL1 = 1 << 58;
        /// `nPOR_EL0` bit.
        const NPOR_EL0 = 1 << 59;
        /// `nPOR_EL1` bit.
        const NPOR_EL1 = 1 << 60;
        /// `nS2POR_EL1` bit.
        const NS2POR_EL1 = 1 << 61;
        /// `nMAIR2_EL1` bit.
        const NMAIR2_EL1 = 1 << 62;
        /// `nAMAIR2_EL1` bit.
        const NAMAIR2_EL1 = 1 << 63;
    }
}

bitflags! {
    /// `HPFAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HpfarEl2: u64 {
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

impl HpfarEl2 {
    /// Returns the value of the `FIPA` field.
    pub const fn fipa(self) -> u64 {
        ((self.bits() >> 4) & 0b11111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `ICC_SRE_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSreEl1: u64 {
        /// Enable the system register interface.
        const SRE = 1 << 0;
        /// Disable FIQ bypass.
        const DFB = 1 << 1;
        /// Disable IRQ bypass.
        const DIB = 1 << 2;
    }
}

bitflags! {
    /// `ICC_SRE_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSreEl2: u64 {
        /// Enable the system register interface.
        const SRE = 1 << 0;
        /// Disable FIQ bypass.
        const DFB = 1 << 1;
        /// Disable IRQ bypass.
        const DIB = 1 << 2;
        /// Enable lower exception level access.
        const ENABLE = 1 << 3;
    }
}

bitflags! {
    /// `ICC_SRE_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSreEl3: u64 {
        /// Enable the system register interface.
        const SRE = 1 << 0;
        /// Disable FIQ bypass.
        const DFB = 1 << 1;
        /// Disable IRQ bypass.
        const DIB = 1 << 2;
        /// Enable lower exception level access.
        const ENABLE = 1 << 3;
    }
}

bitflags! {
    /// `ICH_HCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchHcrEl2: u64 {
        /// `En` bit.
        const EN = 1 << 0;
        /// `UIE` bit.
        const UIE = 1 << 1;
        /// `LRENPIE` bit.
        const LRENPIE = 1 << 2;
        /// `NPIE` bit.
        const NPIE = 1 << 3;
        /// `VGrp0EIE` bit.
        const VGRP0EIE = 1 << 4;
        /// `VGrp0DIE` bit.
        const VGRP0DIE = 1 << 5;
        /// `VGrp1EIE` bit.
        const VGRP1EIE = 1 << 6;
        /// `VGrp1DIE` bit.
        const VGRP1DIE = 1 << 7;
        /// `vSGIEOICount` bit.
        const VSGIEOICOUNT = 1 << 8;
        /// `TC` bit.
        const TC = 1 << 10;
        /// `TALL0` bit.
        const TALL0 = 1 << 11;
        /// `TALL1` bit.
        const TALL1 = 1 << 12;
        /// `TSEI` bit.
        const TSEI = 1 << 13;
        /// `TDIR` bit.
        const TDIR = 1 << 14;
        /// `DVIM` bit.
        const DVIM = 1 << 15;
    }
}

impl IchHcrEl2 {
    /// Returns the value of the `EOIcount` field.
    pub const fn eoicount(self) -> u8 {
        ((self.bits() >> 27) & 0b11111) as u8
    }
}

bitflags! {
    /// `ICH_VMCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchVmcrEl2: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
        /// `VENG0` bit.
        const VENG0 = 1 << 0;
        /// `VENG1` bit.
        const VENG1 = 1 << 1;
        /// `VAckCtl` bit.
        const VACKCTL = 1 << 2;
        /// `VFIQEn` bit.
        const VFIQEN = 1 << 3;
        /// `VCBPR` bit.
        const VCBPR = 1 << 4;
        /// `VEOIM` bit.
        const VEOIM = 1 << 9;
    }
}

impl IchVmcrEl2 {
    /// Returns the value of the `VBPR1` field.
    pub const fn vbpr1(self) -> u8 {
        ((self.bits() >> 18) & 0b111) as u8
    }

    /// Returns the value of the `VBPR0` field.
    pub const fn vbpr0(self) -> u8 {
        ((self.bits() >> 21) & 0b111) as u8
    }
}

bitflags! {
    /// `ID_AA64DFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr0El1: u64 {
    }
}

impl IdAa64dfr0El1 {
    /// Returns the value of the `DebugVer` field.
    pub const fn debugver(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `TraceVer` field.
    pub const fn tracever(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }

    /// Returns the value of the `PMUVer` field.
    pub const fn pmuver(self) -> u8 {
        ((self.bits() >> 8) & 0b1111) as u8
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        ((self.bits() >> 12) & 0b1111) as u8
    }

    /// Returns the value of the `PMSS` field.
    pub const fn pmss(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `SEBEP` field.
    pub const fn sebep(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        ((self.bits() >> 28) & 0b1111) as u8
    }

    /// Returns the value of the `PMSVer` field.
    pub const fn pmsver(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `DoubleLock` field.
    pub const fn doublelock(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `TraceBuffer` field.
    pub const fn tracebuffer(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        ((self.bits() >> 48) & 0b1111) as u8
    }

    /// Returns the value of the `BRBE` field.
    pub const fn brbe(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `ExtTrcBuff` field.
    pub const fn exttrcbuff(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `ID_AA64DFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr1El1: u64 {
    }
}

impl IdAa64dfr1El1 {
    /// Returns the value of the `SYSPMUID` field.
    pub const fn syspmuid(self) -> u8 {
        ((self.bits() >> 0) & 0b11111111) as u8
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        ((self.bits() >> 8) & 0b11111111) as u8
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        ((self.bits() >> 16) & 0b11111111) as u8
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        ((self.bits() >> 24) & 0b11111111) as u8
    }

    /// Returns the value of the `SPMU` field.
    pub const fn spmu(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `PMICNTR` field.
    pub const fn pmicntr(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `ABLE` field.
    pub const fn able(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `ITE` field.
    pub const fn ite(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `EBEP` field.
    pub const fn ebep(self) -> u8 {
        ((self.bits() >> 48) & 0b1111) as u8
    }

    /// Returns the value of the `DPFZS` field.
    pub const fn dpfzs(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `ABL_CMPs` field.
    pub const fn abl_cmps(self) -> u8 {
        ((self.bits() >> 56) & 0b11111111) as u8
    }
}

bitflags! {
    /// `ID_AA64MMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr0El1: u64 {
    }
}

impl IdAa64mmfr0El1 {
    /// Returns the value of the `PARange` field.
    pub const fn parange(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `ASIDBits` field.
    pub const fn asidbits(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }

    /// Returns the value of the `BigEnd` field.
    pub const fn bigend(self) -> u8 {
        ((self.bits() >> 8) & 0b1111) as u8
    }

    /// Returns the value of the `SNSMem` field.
    pub const fn snsmem(self) -> u8 {
        ((self.bits() >> 12) & 0b1111) as u8
    }

    /// Returns the value of the `BigEndEL0` field.
    pub const fn bigendel0(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `TGran16` field.
    pub const fn tgran16(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `TGran64` field.
    pub const fn tgran64(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `TGran4` field.
    pub const fn tgran4(self) -> u8 {
        ((self.bits() >> 28) & 0b1111) as u8
    }

    /// Returns the value of the `TGran16_2` field.
    pub const fn tgran16_2(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `TGran64_2` field.
    pub const fn tgran64_2(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `TGran4_2` field.
    pub const fn tgran4_2(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `ExS` field.
    pub const fn exs(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `FGT` field.
    pub const fn fgt(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }

    /// Returns the value of the `ECV` field.
    pub const fn ecv(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `ID_AA64MMFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr1El1: u64 {
    }
}

impl IdAa64mmfr1El1 {
    /// Returns the value of the `HAFDBS` field.
    pub const fn hafdbs(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `VMIDBits` field.
    pub const fn vmidbits(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }

    /// Returns the value of the `VH` field.
    pub const fn vh(self) -> u8 {
        ((self.bits() >> 8) & 0b1111) as u8
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        ((self.bits() >> 12) & 0b1111) as u8
    }

    /// Returns the value of the `LO` field.
    pub const fn lo(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        ((self.bits() >> 28) & 0b1111) as u8
    }

    /// Returns the value of the `TWED` field.
    pub const fn twed(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `HCX` field.
    pub const fn hcx(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `AFP` field.
    pub const fn afp(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        ((self.bits() >> 48) & 0b1111) as u8
    }

    /// Returns the value of the `TIDCP1` field.
    pub const fn tidcp1(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `CMOW` field.
    pub const fn cmow(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }

    /// Returns the value of the `ECBHB` field.
    pub const fn ecbhb(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `ID_AA64MMFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr2El1: u64 {
    }
}

impl IdAa64mmfr2El1 {
    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `UAO` field.
    pub const fn uao(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        ((self.bits() >> 8) & 0b1111) as u8
    }

    /// Returns the value of the `IESB` field.
    pub const fn iesb(self) -> u8 {
        ((self.bits() >> 12) & 0b1111) as u8
    }

    /// Returns the value of the `VARange` field.
    pub const fn varange(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `NV` field.
    pub const fn nv(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `ST` field.
    pub const fn st(self) -> u8 {
        ((self.bits() >> 28) & 0b1111) as u8
    }

    /// Returns the value of the `AT` field.
    pub const fn at(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `IDS` field.
    pub const fn ids(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `FWB` field.
    pub const fn fwb(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `TTL` field.
    pub const fn ttl(self) -> u8 {
        ((self.bits() >> 48) & 0b1111) as u8
    }

    /// Returns the value of the `BBM` field.
    pub const fn bbm(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }

    /// Returns the value of the `E0PD` field.
    pub const fn e0pd(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `ID_AA64MMFR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr3El1: u64 {
    }
}

impl IdAa64mmfr3El1 {
    /// Returns the value of the `TCRX` field.
    pub const fn tcrx(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `SCTLRX` field.
    pub const fn sctlrx(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }

    /// Returns the value of the `S1PIE` field.
    pub const fn s1pie(self) -> u8 {
        ((self.bits() >> 8) & 0b1111) as u8
    }

    /// Returns the value of the `S2PIE` field.
    pub const fn s2pie(self) -> u8 {
        ((self.bits() >> 12) & 0b1111) as u8
    }

    /// Returns the value of the `S1POE` field.
    pub const fn s1poe(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `S2POE` field.
    pub const fn s2poe(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `AIE` field.
    pub const fn aie(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `MEC` field.
    pub const fn mec(self) -> u8 {
        ((self.bits() >> 28) & 0b1111) as u8
    }

    /// Returns the value of the `D128` field.
    pub const fn d128(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `D128_2` field.
    pub const fn d128_2(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `SNERR` field.
    pub const fn snerr(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `ANERR` field.
    pub const fn anerr(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `SDERR` field.
    pub const fn sderr(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `ADERR` field.
    pub const fn aderr(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }

    /// Returns the value of the `Spec_FPACC` field.
    pub const fn spec_fpacc(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `ID_AA64PFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr0El1: u64 {
    }
}

impl IdAa64pfr0El1 {
    /// Returns the value of the `EL0` field.
    pub const fn el0(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `EL1` field.
    pub const fn el1(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }

    /// Returns the value of the `EL2` field.
    pub const fn el2(self) -> u8 {
        ((self.bits() >> 8) & 0b1111) as u8
    }

    /// Returns the value of the `EL3` field.
    pub const fn el3(self) -> u8 {
        ((self.bits() >> 12) & 0b1111) as u8
    }

    /// Returns the value of the `FP` field.
    pub const fn fp(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `AdvSIMD` field.
    pub const fn advsimd(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        ((self.bits() >> 28) & 0b1111) as u8
    }

    /// Returns the value of the `SVE` field.
    pub const fn sve(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `SEL2` field.
    pub const fn sel2(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `MPAM` field.
    pub const fn mpam(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        ((self.bits() >> 48) & 0b1111) as u8
    }

    /// Returns the value of the `RME` field.
    pub const fn rme(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }

    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `ID_AA64PFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr1El1: u64 {
    }
}

impl IdAa64pfr1El1 {
    /// Returns the value of the `BT` field.
    pub const fn bt(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        ((self.bits() >> 4) & 0b1111) as u8
    }

    /// Returns the value of the `MTE` field.
    pub const fn mte(self) -> u8 {
        ((self.bits() >> 8) & 0b1111) as u8
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        ((self.bits() >> 12) & 0b1111) as u8
    }

    /// Returns the value of the `MPAM_frac` field.
    pub const fn mpam_frac(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `SME` field.
    pub const fn sme(self) -> u8 {
        ((self.bits() >> 24) & 0b1111) as u8
    }

    /// Returns the value of the `RNDR_trap` field.
    pub const fn rndr_trap(self) -> u8 {
        ((self.bits() >> 28) & 0b1111) as u8
    }

    /// Returns the value of the `CSV2_frac` field.
    pub const fn csv2_frac(self) -> u8 {
        ((self.bits() >> 32) & 0b1111) as u8
    }

    /// Returns the value of the `NMI` field.
    pub const fn nmi(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `MTE_frac` field.
    pub const fn mte_frac(self) -> u8 {
        ((self.bits() >> 40) & 0b1111) as u8
    }

    /// Returns the value of the `GCS` field.
    pub const fn gcs(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `THE` field.
    pub const fn the(self) -> u8 {
        ((self.bits() >> 48) & 0b1111) as u8
    }

    /// Returns the value of the `MTEX` field.
    pub const fn mtex(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `DF2` field.
    pub const fn df2(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }

    /// Returns the value of the `PFAR` field.
    pub const fn pfar(self) -> u8 {
        ((self.bits() >> 60) & 0b1111) as u8
    }
}

bitflags! {
    /// `ID_AA64SMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64smfr0El1: u64 {
        /// `SMOP4` bit.
        const SMOP4 = 1 << 0;
        /// `STMOP` bit.
        const STMOP = 1 << 16;
        /// `SFEXPA` bit.
        const SFEXPA = 1 << 23;
        /// `AES` bit.
        const AES = 1 << 24;
        /// `SBitPerm` bit.
        const SBITPERM = 1 << 25;
        /// `SF8DP2` bit.
        const SF8DP2 = 1 << 28;
        /// `SF8DP4` bit.
        const SF8DP4 = 1 << 29;
        /// `SF8FMA` bit.
        const SF8FMA = 1 << 30;
        /// `F32F32` bit.
        const F32F32 = 1 << 32;
        /// `BI32I32` bit.
        const BI32I32 = 1 << 33;
        /// `B16F32` bit.
        const B16F32 = 1 << 34;
        /// `F16F32` bit.
        const F16F32 = 1 << 35;
        /// `F8F32` bit.
        const F8F32 = 1 << 40;
        /// `F8F16` bit.
        const F8F16 = 1 << 41;
        /// `F16F16` bit.
        const F16F16 = 1 << 42;
        /// `B16B16` bit.
        const B16B16 = 1 << 43;
        /// `F64F64` bit.
        const F64F64 = 1 << 48;
        /// `LUTv2` bit.
        const LUTV2 = 1 << 60;
        /// `LUT6` bit.
        const LUT6 = 1 << 61;
        /// `FA64` bit.
        const FA64 = 1 << 63;
    }
}

impl IdAa64smfr0El1 {
    /// Returns the value of the `I8I32` field.
    pub const fn i8i32(self) -> u8 {
        ((self.bits() >> 36) & 0b1111) as u8
    }

    /// Returns the value of the `I16I32` field.
    pub const fn i16i32(self) -> u8 {
        ((self.bits() >> 44) & 0b1111) as u8
    }

    /// Returns the value of the `I16I64` field.
    pub const fn i16i64(self) -> u8 {
        ((self.bits() >> 52) & 0b1111) as u8
    }

    /// Returns the value of the `SMEver` field.
    pub const fn smever(self) -> u8 {
        ((self.bits() >> 56) & 0b1111) as u8
    }
}

bitflags! {
    /// `ISR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IsrEl1: u64 {
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `FS` bit.
        const FS = 1 << 9;
        /// `IS` bit.
        const IS = 1 << 10;
    }
}

bitflags! {
    /// `MAIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl1: u64 {
    }
}

impl MairEl1 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (0 + (n - 0) * 8)) & 0b11111111) as u8
    }
}

bitflags! {
    /// `MAIR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl2: u64 {
    }
}

impl MairEl2 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (0 + (n - 0) * 8)) & 0b11111111) as u8
    }
}

bitflags! {
    /// `MAIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl3: u64 {
    }
}

impl MairEl3 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (0 + (n - 0) * 8)) & 0b11111111) as u8
    }
}

bitflags! {
    /// `MDCCINT_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdccintEl1: u64 {
        /// `TX` bit.
        const TX = 1 << 29;
        /// `RX` bit.
        const RX = 1 << 30;
    }
}

bitflags! {
    /// `MDCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdcrEl2: u64 {
        /// `TPMCR` bit.
        const TPMCR = 1 << 5;
        /// `TPM` bit.
        const TPM = 1 << 6;
        /// `HPME` bit.
        const HPME = 1 << 7;
        /// `TDE` bit.
        const TDE = 1 << 8;
        /// `TDA` bit.
        const TDA = 1 << 9;
        /// `TDOSA` bit.
        const TDOSA = 1 << 10;
        /// `TDRA` bit.
        const TDRA = 1 << 11;
        /// `TPMS` bit.
        const TPMS = 1 << 14;
        /// `EnSPM` bit.
        const ENSPM = 1 << 15;
        /// `HPMD` bit.
        const HPMD = 1 << 17;
        /// `TTRF` bit.
        const TTRF = 1 << 19;
        /// `HCCD` bit.
        const HCCD = 1 << 23;
        /// `HLP` bit.
        const HLP = 1 << 26;
        /// `TDCC` bit.
        const TDCC = 1 << 27;
        /// `MTPME` bit.
        const MTPME = 1 << 28;
        /// `HPMFZO` bit.
        const HPMFZO = 1 << 29;
        /// `HPMFZS` bit.
        const HPMFZS = 1 << 36;
        /// `EBWE` bit.
        const EBWE = 1 << 43;
        /// `EnSTEPOP` bit.
        const ENSTEPOP = 1 << 50;
    }
}

impl MdcrEl2 {
    /// Returns the value of the `HPMN` field.
    pub const fn hpmn(self) -> u8 {
        ((self.bits() >> 0) & 0b11111) as u8
    }

    /// Returns the value of the `E2PB` field.
    pub const fn e2pb(self) -> u8 {
        ((self.bits() >> 12) & 0b11) as u8
    }

    /// Returns the value of the `E2TB` field.
    pub const fn e2tb(self) -> u8 {
        ((self.bits() >> 24) & 0b11) as u8
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        ((self.bits() >> 30) & 0b11) as u8
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        ((self.bits() >> 40) & 0b11) as u8
    }
}

bitflags! {
    /// `MDCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdcrEl3: u64 {
        /// Realm Trace enable. Enables tracing in Realm state.
        const RLTE = 1 << 0;
        /// `EPMADE` bit.
        const EPMADE = 1 << 2;
        /// `ETADE` bit.
        const ETADE = 1 << 3;
        /// `EDADE` bit.
        const EDADE = 1 << 4;
        /// Trap Performance Monitor register accesses
        const TPM = 1 << 6;
        /// Do not trap various PMUv3p9 related system register accesses to EL3.
        const ENPM2 = 1 << 7;
        /// `TDA` bit.
        const TDA = 1 << 9;
        /// `TDOSA` bit.
        const TDOSA = 1 << 10;
        /// Non-secure Profiling Buffer Extended. Together with MDCR_EL3.NSPB, controls the Profiling Buffer owning Security state and accesses to Statistical Profiling and Profiling Buffer System registers from EL2 and EL1.
        const NSPBE = 1 << 11;
        /// Set to one to disable AArch64 Secure self-hosted debug. Debug exceptions, other than Breakpoint Instruction exceptions, are disabled from all ELs in Secure state.
        const SDD = 1 << 16;
        /// Secure Performance Monitors Enable. Controls event counting in Secure state and EL3.
        const SPME = 1 << 17;
        /// Secure Trace enable. Enables tracing in Secure state.
        const STE = 1 << 18;
        /// Trap Trace Filter controls. Traps use of the Trace Filter control registers at EL2 and EL1 to EL3.
        const TTRF = 1 << 19;
        /// `EDAD` bit.
        const EDAD = 1 << 20;
        /// `EPMAD` bit.
        const EPMAD = 1 << 21;
        /// `ETAD` bit.
        const ETAD = 1 << 22;
        /// Secure Cycle Counter Disable. Prohibits PMCCNTR_EL0 from counting in Secure state.
        const SCCD = 1 << 23;
        /// Non-secure Trace Buffer Extended. Together with MDCR_EL3.NSTB, controls the trace buffer owning Security state and accesses to trace buffer System registers from EL2 and EL1.
        const NSTBE = 1 << 26;
        /// `TDCC` bit.
        const TDCC = 1 << 27;
        /// Multi-threaded PMU Enable. Enables use of the PMEVTYPER<n>_EL0.MT bits.
        const MTPME = 1 << 28;
        /// Monitor Cycle Counter Disable. Prohibits the Cycle Counter, PMCCNTR_EL0, from counting at EL3.
        const MCCD = 1 << 34;
        /// Monitor Performance Monitors Extended control. In conjunction with MDCR_EL3.SPME, controls when event counters are enabled at EL3 and in other Secure Exception levels.
        const MPMX = 1 << 35;
        /// Trap accesses to PMSNEVFR_EL1. Controls access to Statistical Profiling PMSNEVFR_EL1 System register from EL2 and EL1.
        const ENPMSN = 1 << 36;
        /// `E3BREW` bit.
        const E3BREW = 1 << 37;
        /// `E3BREC` bit.
        const E3BREC = 1 << 38;
        /// `EnTB2` bit.
        const ENTB2 = 1 << 39;
        /// Enable access to SPE registers. When disabled, accesses to SPE registers generate a trap to EL3.
        const ENPMS3 = 1 << 42;
        /// `EBWE` bit.
        const EBWE = 1 << 43;
        /// `EnPMSS` bit.
        const ENPMSS = 1 << 44;
        /// `EnITE` bit.
        const ENITE = 1 << 47;
        /// `EnSTEPOP` bit.
        const ENSTEPOP = 1 << 50;
        /// `EnPMS4` bit.
        const ENPMS4 = 1 << 55;
    }
}

impl MdcrEl3 {
    /// Returns the value of the `NSPB` field.
    pub const fn nspb(self) -> u8 {
        ((self.bits() >> 12) & 0b11) as u8
    }

    /// Returns the value of the `SPD32` field.
    pub const fn spd32(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }

    /// Returns the value of the `NSTB` field.
    pub const fn nstb(self) -> u8 {
        ((self.bits() >> 24) & 0b11) as u8
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        ((self.bits() >> 30) & 0b11) as u8
    }

    /// Returns the value of the `SBRBE` field.
    pub const fn sbrbe(self) -> u8 {
        ((self.bits() >> 32) & 0b11) as u8
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        ((self.bits() >> 40) & 0b11) as u8
    }

    /// Returns the value of the `EPMSSAD` field.
    pub const fn epmssad(self) -> u8 {
        ((self.bits() >> 45) & 0b11) as u8
    }

    /// Returns the value of the `ETBAD` field.
    pub const fn etbad(self) -> u8 {
        ((self.bits() >> 48) & 0b11) as u8
    }

    /// Returns the value of the `PMSEE` field.
    pub const fn pmsee(self) -> u8 {
        ((self.bits() >> 51) & 0b11) as u8
    }

    /// Returns the value of the `TRBEE` field.
    pub const fn trbee(self) -> u8 {
        ((self.bits() >> 53) & 0b11) as u8
    }
}

bitflags! {
    /// `MDSCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdscrEl1: u64 {
        /// `SS` bit.
        const SS = 1 << 0;
        /// `ERR` bit.
        const ERR = 1 << 6;
        /// `TDCC` bit.
        const TDCC = 1 << 12;
        /// `KDE` bit.
        const KDE = 1 << 13;
        /// `HDE` bit.
        const HDE = 1 << 14;
        /// `MDE` bit.
        const MDE = 1 << 15;
        /// `SC2` bit.
        const SC2 = 1 << 19;
        /// `TDA` bit.
        const TDA = 1 << 21;
        /// `TXU` bit.
        const TXU = 1 << 26;
        /// `RXO` bit.
        const RXO = 1 << 27;
        /// `TXfull` bit.
        const TXFULL = 1 << 29;
        /// `RXfull` bit.
        const RXFULL = 1 << 30;
        /// `TFO` bit.
        const TFO = 1 << 31;
        /// `EMBWE` bit.
        const EMBWE = 1 << 32;
        /// `TTA` bit.
        const TTA = 1 << 33;
        /// `EnSPM` bit.
        const ENSPM = 1 << 34;
        /// `EHBWE` bit.
        const EHBWE = 1 << 35;
        /// `EnSTEPOP` bit.
        const ENSTEPOP = 1 << 50;
    }
}

impl MdscrEl1 {
    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        ((self.bits() >> 22) & 0b11) as u8
    }
}

bitflags! {
    /// `MIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MidrEl1: u64 {
    }
}

impl MidrEl1 {
    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> 4) & 0b111111111111) as u16
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> 24) & 0b11111111) as u8
    }
}

bitflags! {
    /// `MPAM2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpam2El2: u64 {
        /// `TRAPMPAM1EL1` bit.
        const TRAPMPAM1EL1 = 1 << 48;
        /// `TRAPMPAM0EL1` bit.
        const TRAPMPAM0EL1 = 1 << 49;
        /// `EnMPAMSM` bit.
        const ENMPAMSM = 1 << 50;
        /// `ALTSP_FRCD` bit.
        const ALTSP_FRCD = 1 << 54;
        /// `ALTSP_EL2` bit.
        const ALTSP_EL2 = 1 << 55;
        /// `ALTSP_HFC` bit.
        const ALTSP_HFC = 1 << 56;
        /// `TIDR` bit.
        const TIDR = 1 << 58;
        /// `MPAMEN` bit.
        const MPAMEN = 1 << 63;
    }
}

impl Mpam2El2 {
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        ((self.bits() >> 32) & 0b11111111) as u8
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        ((self.bits() >> 40) & 0b11111111) as u8
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAM3_EL3` system register value.
    ///
    /// Holds information to generate MPAM labels for memory requests when executing at EL3.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpam3El3: u64 {
        /// `RT_ALTSP_NS` bit.
        const RT_ALTSP_NS = 1 << 52;
        /// `ALTSP_EL3` bit.
        const ALTSP_EL3 = 1 << 55;
        /// `ALTSP_HFC` bit.
        const ALTSP_HFC = 1 << 56;
        /// `ALTSP_HEN` bit.
        const ALTSP_HEN = 1 << 57;
        /// `FORCE_NS` bit.
        const FORCE_NS = 1 << 60;
        /// `SDEFLT` bit.
        const SDEFLT = 1 << 61;
        /// Trap direct accesses to MPAM System registers that are not UNDEFINED from all ELn lower than EL3.
        const TRAPLOWER = 1 << 62;
        /// MPAM Enable. If set, MPAM information is output based on the MPAMn_ELx register for ELn according the MPAM configuration. If not set, the default PARTID and default PMG are output in MPAM information when executing at any ELn.
        const MPAMEN = 1 << 63;
    }
}

impl Mpam3El3 {
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        ((self.bits() >> 32) & 0b11111111) as u8
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        ((self.bits() >> 40) & 0b11111111) as u8
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMHCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamhcrEl2: u64 {
        /// `EL0_VPMEN` bit.
        const EL0_VPMEN = 1 << 0;
        /// `EL1_VPMEN` bit.
        const EL1_VPMEN = 1 << 1;
        /// `VPMEN` bit.
        const VPMEN = 1 << 2;
        /// `VMMEN` bit.
        const VMMEN = 1 << 3;
        /// `SMVPMEN` bit.
        const SMVPMEN = 1 << 4;
        /// `SMVMMEN` bit.
        const SMVMMEN = 1 << 5;
        /// `GSTAPP_PLK` bit.
        const GSTAPP_PLK = 1 << 8;
        /// `TRAP_MPAMIDR_EL1` bit.
        const TRAP_MPAMIDR_EL1 = 1 << 31;
    }
}

bitflags! {
    /// `MPAMIDR_EL1` system register value.
    ///
    /// Indicates the maximum PARTID and PMG values supported in the implementation and the support for other optional features.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamidrEl1: u64 {
        /// Indicates support for MPAM virtualization.
        const HAS_HCR = 1 << 17;
        /// `HAS_ALT_ID` bit.
        const HAS_ALT_ID = 1 << 21;
        /// `HAS_INSTR_ALT_ID` bit.
        const HAS_INSTR_ALT_ID = 1 << 22;
        /// `HAS_BW_CTRL` bit.
        const HAS_BW_CTRL = 1 << 56;
        /// `HAS_ALTSP` bit.
        const HAS_ALTSP = 1 << 57;
        /// `HAS_TIDR` bit.
        const HAS_TIDR = 1 << 58;
        /// `SP4` bit.
        const SP4 = 1 << 59;
        /// `HAS_FORCE_NS` bit.
        const HAS_FORCE_NS = 1 << 60;
        /// `HAS_SDEFLT` bit.
        const HAS_SDEFLT = 1 << 61;
    }
}

impl MpamidrEl1 {
    /// Returns the value of the `PARTID_MAX` field.
    pub const fn partid_max(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `VPMR_MAX` field.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn vpmr_max(self) -> u8 {
        ((self.bits() >> 18) & 0b111) as u8
    }
}

bitflags! {
    /// `MPAMVPM0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm0El2: u64 {
    }
}

impl Mpamvpm0El2 {
    /// Returns the value of the `PhyPARTID0` field.
    pub const fn phypartid0(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID1` field.
    pub const fn phypartid1(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID2` field.
    pub const fn phypartid2(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID3` field.
    pub const fn phypartid3(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPM1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm1El2: u64 {
    }
}

impl Mpamvpm1El2 {
    /// Returns the value of the `PhyPARTID4` field.
    pub const fn phypartid4(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID5` field.
    pub const fn phypartid5(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID6` field.
    pub const fn phypartid6(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID7` field.
    pub const fn phypartid7(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPM2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm2El2: u64 {
    }
}

impl Mpamvpm2El2 {
    /// Returns the value of the `PhyPARTID8` field.
    pub const fn phypartid8(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID9` field.
    pub const fn phypartid9(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID10` field.
    pub const fn phypartid10(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID11` field.
    pub const fn phypartid11(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPM3_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm3El2: u64 {
    }
}

impl Mpamvpm3El2 {
    /// Returns the value of the `PhyPARTID12` field.
    pub const fn phypartid12(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID13` field.
    pub const fn phypartid13(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID14` field.
    pub const fn phypartid14(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID15` field.
    pub const fn phypartid15(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPM4_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm4El2: u64 {
    }
}

impl Mpamvpm4El2 {
    /// Returns the value of the `PhyPARTID16` field.
    pub const fn phypartid16(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID17` field.
    pub const fn phypartid17(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID18` field.
    pub const fn phypartid18(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID19` field.
    pub const fn phypartid19(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPM5_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm5El2: u64 {
    }
}

impl Mpamvpm5El2 {
    /// Returns the value of the `PhyPARTID20` field.
    pub const fn phypartid20(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID21` field.
    pub const fn phypartid21(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID22` field.
    pub const fn phypartid22(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID23` field.
    pub const fn phypartid23(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPM6_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm6El2: u64 {
    }
}

impl Mpamvpm6El2 {
    /// Returns the value of the `PhyPARTID24` field.
    pub const fn phypartid24(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID25` field.
    pub const fn phypartid25(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID26` field.
    pub const fn phypartid26(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID27` field.
    pub const fn phypartid27(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPM7_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm7El2: u64 {
    }
}

impl Mpamvpm7El2 {
    /// Returns the value of the `PhyPARTID28` field.
    pub const fn phypartid28(self) -> u16 {
        ((self.bits() >> 0) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID29` field.
    pub const fn phypartid29(self) -> u16 {
        ((self.bits() >> 16) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID30` field.
    pub const fn phypartid30(self) -> u16 {
        ((self.bits() >> 32) & 0b1111111111111111) as u16
    }

    /// Returns the value of the `PhyPARTID31` field.
    pub const fn phypartid31(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `MPAMVPMV_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamvpmvEl2: u64 {
        /// `VPM_V<m>` bit 0.
        const VPM_V0 = 1 << 0;
        /// `VPM_V<m>` bit 1.
        const VPM_V1 = 1 << 1;
        /// `VPM_V<m>` bit 2.
        const VPM_V2 = 1 << 2;
        /// `VPM_V<m>` bit 3.
        const VPM_V3 = 1 << 3;
        /// `VPM_V<m>` bit 4.
        const VPM_V4 = 1 << 4;
        /// `VPM_V<m>` bit 5.
        const VPM_V5 = 1 << 5;
        /// `VPM_V<m>` bit 6.
        const VPM_V6 = 1 << 6;
        /// `VPM_V<m>` bit 7.
        const VPM_V7 = 1 << 7;
        /// `VPM_V<m>` bit 8.
        const VPM_V8 = 1 << 8;
        /// `VPM_V<m>` bit 9.
        const VPM_V9 = 1 << 9;
        /// `VPM_V<m>` bit 10.
        const VPM_V10 = 1 << 10;
        /// `VPM_V<m>` bit 11.
        const VPM_V11 = 1 << 11;
        /// `VPM_V<m>` bit 12.
        const VPM_V12 = 1 << 12;
        /// `VPM_V<m>` bit 13.
        const VPM_V13 = 1 << 13;
        /// `VPM_V<m>` bit 14.
        const VPM_V14 = 1 << 14;
        /// `VPM_V<m>` bit 15.
        const VPM_V15 = 1 << 15;
        /// `VPM_V<m>` bit 16.
        const VPM_V16 = 1 << 16;
        /// `VPM_V<m>` bit 17.
        const VPM_V17 = 1 << 17;
        /// `VPM_V<m>` bit 18.
        const VPM_V18 = 1 << 18;
        /// `VPM_V<m>` bit 19.
        const VPM_V19 = 1 << 19;
        /// `VPM_V<m>` bit 20.
        const VPM_V20 = 1 << 20;
        /// `VPM_V<m>` bit 21.
        const VPM_V21 = 1 << 21;
        /// `VPM_V<m>` bit 22.
        const VPM_V22 = 1 << 22;
        /// `VPM_V<m>` bit 23.
        const VPM_V23 = 1 << 23;
        /// `VPM_V<m>` bit 24.
        const VPM_V24 = 1 << 24;
        /// `VPM_V<m>` bit 25.
        const VPM_V25 = 1 << 25;
        /// `VPM_V<m>` bit 26.
        const VPM_V26 = 1 << 26;
        /// `VPM_V<m>` bit 27.
        const VPM_V27 = 1 << 27;
        /// `VPM_V<m>` bit 28.
        const VPM_V28 = 1 << 28;
        /// `VPM_V<m>` bit 29.
        const VPM_V29 = 1 << 29;
        /// `VPM_V<m>` bit 30.
        const VPM_V30 = 1 << 30;
        /// `VPM_V<m>` bit 31.
        const VPM_V31 = 1 << 31;
    }
}

bitflags! {
    /// `MPIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpidrEl1: u64 {
        /// RES1 bits in the `MPIDR_EL1` register.
        const RES1 = 0b10000000000000000000000000000000;
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
    }
}

impl MpidrEl1 {
    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> 0) & 0b11111111) as u8
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> 8) & 0b11111111) as u8
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> 16) & 0b11111111) as u8
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        ((self.bits() >> 32) & 0b11111111) as u8
    }
}

bitflags! {
    /// `PAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ParEl1: u64 {
        /// RES1 bits in the `PAR_EL1` register.
        const RES1 = 0b100000000000;
        /// `F` bit.
        const F = 1 << 0;
        /// `PTW` bit.
        const PTW = 1 << 8;
        /// `NS` bit.
        const NS = 1 << 9;
        /// `S` bit.
        const S = 1 << 9;
        /// `NSE` bit.
        const NSE = 1 << 11;
        /// `AssuredOnly` bit.
        const ASSUREDONLY = 1 << 12;
        /// `TopLevel` bit.
        const TOPLEVEL = 1 << 13;
        /// `Overlay` bit.
        const OVERLAY = 1 << 14;
        /// `DirtyBit` bit.
        const DIRTYBIT = 1 << 15;
    }
}

impl ParEl1 {
    /// Returns the value of the `FST` field.
    pub const fn fst(self) -> u8 {
        ((self.bits() >> 1) & 0b111111) as u8
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        ((self.bits() >> 7) & 0b11) as u8
    }

    /// Returns the value of the `PA[47:12]` field.
    pub const fn pa_47_12(self) -> u64 {
        ((self.bits() >> 12) & 0b111111111111111111111111111111111111) as u64
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        ((self.bits() >> 48) & 0b1111) as u8
    }

    /// Returns the value of the `ATTR` field.
    pub const fn attr(self) -> u8 {
        ((self.bits() >> 56) & 0b11111111) as u8
    }
}

bitflags! {
    /// `PMCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmcrEl0: u64 {
        /// Enable. Affected counters are enabled by PMCNTENSET_EL0.
        const E = 1 << 0;
        /// Event counter reset. Reset all affected event counters PMEVCNTR<n>_EL0 to zero.
        const P = 1 << 1;
        /// Cycle counter reset. Reset PMCCNTR_EL0 to zero.
        const C = 1 << 2;
        /// Clock divider. If set PMCCNTR_EL0 counts once every 64 clock cycles.
        const D = 1 << 3;
        /// Enable export of events in an IMPLEMENTATION DEFINED PMU event export bus. If set, export events where not prohibited.
        const X = 1 << 4;
        /// If set, cycle counting by PMCCNTR_EL0 is disabled in prohibited regions.
        const DP = 1 << 5;
        /// `LC` bit.
        const LC = 1 << 6;
        /// `LP` bit.
        const LP = 1 << 7;
        /// `FZO` bit.
        const FZO = 1 << 9;
        /// `FZS` bit.
        const FZS = 1 << 32;
    }
}

impl PmcrEl0 {
    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> 11) & 0b11111) as u8
    }

    /// Returns the value of the `IDCODE` field.
    pub const fn idcode(self) -> u8 {
        ((self.bits() >> 16) & 0b11111111) as u8
    }

    /// Returns the value of the `IMP` field.
    pub const fn imp(self) -> u8 {
        ((self.bits() >> 24) & 0b11111111) as u8
    }
}

bitflags! {
    /// `RGSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RgsrEl1: u64 {
    }
}

impl RgsrEl1 {
    /// Returns the value of the `TAG` field.
    pub const fn tag(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `SEED` field.
    pub const fn seed(self) -> u16 {
        ((self.bits() >> 8) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `SCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ScrEl3: u64 {
        /// RES1 bits in the `SCR_EL3` register.
        const RES1 = 0b110000;
        /// Non-secure.
        const NS = 1 << 0;
        /// Take physical IRQs at EL3.
        const IRQ = 1 << 1;
        /// Take physical FIQs at EL3.
        const FIQ = 1 << 2;
        /// Take external abort and SError exceptions at EL3.
        const EA = 1 << 3;
        /// Disable SMC instructions.
        const SMD = 1 << 7;
        /// Enable HVC instructions.
        const HCE = 1 << 8;
        /// Disable execution from non-secure memory.
        const SIF = 1 << 9;
        /// Enable AArch64 in lower ELs.
        const RW = 1 << 10;
        /// Trap physical secure timer to EL3.
        const ST = 1 << 11;
        /// Trap WFI to EL3.
        const TWI = 1 << 12;
        /// Trap WFE to EL3.
        const TWE = 1 << 13;
        /// Trap LOR register access to EL3.
        const TLOR = 1 << 14;
        /// Trap error record register access to EL3.
        const TERR = 1 << 15;
        /// Don't trap PAC key registers to EL3.
        const APK = 1 << 16;
        /// Don't trap PAuth instructions to EL3.
        const API = 1 << 17;
        /// Enable Secure EL2.
        const EEL2 = 1 << 18;
        /// Synchronous external aborts are taken as SErrors.
        const EASE = 1 << 19;
        /// Take SError exceptions at EL3.
        const NMEA = 1 << 20;
        /// Enable fault injection at lower ELs.
        const FIEN = 1 << 21;
        /// Trap ID group 3 registers to EL3.
        const TID3 = 1 << 22;
        /// Trap ID group 5 register to EL3.
        const TID5 = 1 << 23;
        /// `POE2En` bit.
        const POE2EN = 1 << 24;
        /// Enable SCXT at lower ELs.
        const ENSCXT = 1 << 25;
        /// Enable memory tagging at lower ELs.
        const ATA = 1 << 26;
        /// Enable fine-grained traps to EL2.
        const FGTEN = 1 << 27;
        /// Enable access to CNTPOFF_EL2.
        const ECVEN = 1 << 28;
        /// Enable a configurable delay for WFE traps.
        const TWEDEN = 1 << 29;
        /// Enable activity monitors virtual offsets.
        const AMVOFFEN = 1 << 35;
        /// Enable ST64BV0 at lower ELs.
        const ENAS0 = 1 << 36;
        /// Enable ACCDATA_EL1 at lower ELs.
        const ADEN = 1 << 37;
        /// Enable HCRX_EL2.
        const HXEN = 1 << 38;
        /// Enable guarded control stack.
        const GCSEN = 1 << 39;
        /// Trap RNDR and RNDRRS to EL3.
        const TRNDR = 1 << 40;
        /// Enable TPIDR2_EL0 at lower ELs.
        const ENTP2 = 1 << 41;
        /// Enable RCW and RCWS mask registers at lower ELs.
        const RCWMASKEN = 1 << 42;
        /// Enable TCR2_ELx registers at lower ELs.
        const TCR2EN = 1 << 43;
        /// Enable SCTLR2_ELx registers at lower ELs.
        const SCTLR2EN = 1 << 44;
        /// Enable permission indirection and overlay registers at lower ELs.
        const PIEN = 1 << 45;
        /// Enable MAIR2_ELx and AMAIR2_ELx at lower ELs.
        const AIEN = 1 << 46;
        /// Enable 128-bit system registers at  lower ELs.
        const D128EN = 1 << 47;
        /// Route GPFs to EL3.
        const GPF = 1 << 48;
        /// Enable MECID registers at EL2.
        const MECEN = 1 << 49;
        /// Enable access to FPMR at lower ELs.
        const ENFPM = 1 << 50;
        /// Take synchronous external abort and physical SError exception to EL3.
        const TMEA = 1 << 51;
        /// Trap writes to Error Record registers to EL3.
        const TWERR = 1 << 52;
        /// Enable access to physical fault address registers at lower ELs.
        const PFAREN = 1 << 53;
        /// Enable access to mask registers at lower ELs.
        const SRMASKEN = 1 << 54;
        /// Enable implementation-defined 128-bit system registers.
        const ENIDCP128 = 1 << 55;
        /// `VTLBIDEn` bit.
        const VTLBIDEN = 1 << 56;
        /// A delegated SError exception is pending.
        const DSE = 1 << 57;
        /// Enable delegated SError exceptions.
        const ENDSE = 1 << 58;
        /// Enable fine-grained traps to EL2.
        const FGTEN2 = 1 << 59;
        /// Enable HDBSSBR_EL2 and HDBSSPROD_EL2 registers at EL2.
        const HDBSSEN = 1 << 60;
        /// Enable HACDBSBR_EL2 and HACDBSCONS_EL2 registers at EL2.
        const HACDBSEN = 1 << 61;
        /// Non-secure realm world bit.
        const NSE = 1 << 62;
        /// `TPLIMEn` bit.
        const TPLIMEN = 1 << 63;
    }
}

impl ScrEl3 {
    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> 30) & 0b1111) as u8
    }
}

bitflags! {
    /// `SCTLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SctlrEl1: u64 {
        /// `M` bit.
        const M = 1 << 0;
        /// `A` bit.
        const A = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `SA` bit.
        const SA = 1 << 3;
        /// `SA0` bit.
        const SA0 = 1 << 4;
        /// `CP15BEN` bit.
        const CP15BEN = 1 << 5;
        /// `nAA` bit.
        const NAA = 1 << 6;
        /// `ITD` bit.
        const ITD = 1 << 7;
        /// `SED` bit.
        const SED = 1 << 8;
        /// `UMA` bit.
        const UMA = 1 << 9;
        /// `EnRCTX` bit.
        const ENRCTX = 1 << 10;
        /// `EOS` bit.
        const EOS = 1 << 11;
        /// `I` bit.
        const I = 1 << 12;
        /// `EnDB` bit.
        const ENDB = 1 << 13;
        /// `DZE` bit.
        const DZE = 1 << 14;
        /// `UCT` bit.
        const UCT = 1 << 15;
        /// `nTWI` bit.
        const NTWI = 1 << 16;
        /// `nTWE` bit.
        const NTWE = 1 << 18;
        /// `WXN` bit.
        const WXN = 1 << 19;
        /// `TSCXT` bit.
        const TSCXT = 1 << 20;
        /// `IESB` bit.
        const IESB = 1 << 21;
        /// `EIS` bit.
        const EIS = 1 << 22;
        /// Do not set Privileged Access Never, on taking an exception to EL1.
        const SPAN = 1 << 23;
        /// `UCI` bit.
        const UCI = 1 << 26;
        /// `EnDA` bit.
        const ENDA = 1 << 27;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 28;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 29;
        /// Enable pointer authentication using APIBKey_EL1.
        const ENIB = 1 << 30;
        /// Enable pointer authentication using APIAKey_EL1.
        const ENIA = 1 << 31;
        /// `CMOW` bit.
        const CMOW = 1 << 32;
        /// `MSCEn` bit.
        const MSCEN = 1 << 33;
        /// `EnFPM` bit.
        const ENFPM = 1 << 34;
        /// `BT0` bit.
        const BT0 = 1 << 35;
        /// `BT1` bit.
        const BT1 = 1 << 36;
        /// `ITFSB` bit.
        const ITFSB = 1 << 37;
        /// `ATA0` bit.
        const ATA0 = 1 << 42;
        /// `ATA` bit.
        const ATA = 1 << 43;
        /// Default PSTATE.SSBS value on Exception Entry.
        const DSSBS = 1 << 44;
        /// `TWEDEn` bit.
        const TWEDEN = 1 << 45;
        /// `EnASR` bit.
        const ENASR = 1 << 54;
        /// `EnAS0` bit.
        const ENAS0 = 1 << 55;
        /// `EnALS` bit.
        const ENALS = 1 << 56;
        /// `EPAN` bit.
        const EPAN = 1 << 57;
        /// `TCSO0` bit.
        const TCSO0 = 1 << 58;
        /// `TCSO` bit.
        const TCSO = 1 << 59;
        /// `EnTP2` bit.
        const ENTP2 = 1 << 60;
        /// `NMI` bit.
        const NMI = 1 << 61;
        /// SP Interrupt Mask enable.
        const SPINTMASK = 1 << 62;
        /// `TIDCP` bit.
        const TIDCP = 1 << 63;
    }
}

impl SctlrEl1 {
    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        ((self.bits() >> 38) & 0b11) as u8
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        ((self.bits() >> 40) & 0b11) as u8
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> 46) & 0b1111) as u8
    }
}

bitflags! {
    /// `SCTLR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SctlrEl2: u64 {
        /// `M` bit.
        const M = 1 << 0;
        /// `A` bit.
        const A = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `SA` bit.
        const SA = 1 << 3;
        /// `SA0` bit.
        const SA0 = 1 << 4;
        /// `CP15BEN` bit.
        const CP15BEN = 1 << 5;
        /// `nAA` bit.
        const NAA = 1 << 6;
        /// `SED` bit.
        const SED = 1 << 8;
        /// `UMA` bit.
        const UMA = 1 << 9;
        /// `EnRCTX` bit.
        const ENRCTX = 1 << 10;
        /// `EOS` bit.
        const EOS = 1 << 11;
        /// `I` bit.
        const I = 1 << 12;
        /// `EnDB` bit.
        const ENDB = 1 << 13;
        /// `DZE` bit.
        const DZE = 1 << 14;
        /// `UCT` bit.
        const UCT = 1 << 15;
        /// `nTWI` bit.
        const NTWI = 1 << 16;
        /// `nTWE` bit.
        const NTWE = 1 << 18;
        /// `WXN` bit.
        const WXN = 1 << 19;
        /// `IESB` bit.
        const IESB = 1 << 21;
        /// `EIS` bit.
        const EIS = 1 << 22;
        /// Do not set Privileged Access Never, on taking an exception to EL2.
        const SPAN = 1 << 23;
        /// `UCI` bit.
        const UCI = 1 << 26;
        /// `EnDA` bit.
        const ENDA = 1 << 27;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 28;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 29;
        /// Enable pointer authentication using APIBKey_EL1.
        const ENIB = 1 << 30;
        /// Enable pointer authentication using APIAKey_EL1.
        const ENIA = 1 << 31;
        /// `CMOW` bit.
        const CMOW = 1 << 32;
        /// `MSCEn` bit.
        const MSCEN = 1 << 33;
        /// `EnFPM` bit.
        const ENFPM = 1 << 34;
        /// `BT0` bit.
        const BT0 = 1 << 35;
        /// `BT` bit.
        const BT = 1 << 36;
        /// `ITFSB` bit.
        const ITFSB = 1 << 37;
        /// `ATA0` bit.
        const ATA0 = 1 << 42;
        /// `ATA` bit.
        const ATA = 1 << 43;
        /// Default PSTATE.SSBS value on Exception Entry.
        const DSSBS = 1 << 44;
        /// `TWEDEn` bit.
        const TWEDEN = 1 << 45;
        /// `EnASR` bit.
        const ENASR = 1 << 54;
        /// `EnAS0` bit.
        const ENAS0 = 1 << 55;
        /// `EnALS` bit.
        const ENALS = 1 << 56;
        /// `EPAN` bit.
        const EPAN = 1 << 57;
        /// `TCSO0` bit.
        const TCSO0 = 1 << 58;
        /// `TCSO` bit.
        const TCSO = 1 << 59;
        /// `EnTP2` bit.
        const ENTP2 = 1 << 60;
        /// `NMI` bit.
        const NMI = 1 << 61;
        /// SP Interrupt Mask enable.
        const SPINTMASK = 1 << 62;
        /// `TIDCP` bit.
        const TIDCP = 1 << 63;
    }
}

impl SctlrEl2 {
    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        ((self.bits() >> 38) & 0b11) as u8
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        ((self.bits() >> 40) & 0b11) as u8
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> 46) & 0b1111) as u8
    }
}

bitflags! {
    /// `SCTLR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SctlrEl3: u64 {
        /// RES1 bits in the `SCTLR_EL3` register.
        const RES1 = 0b110000100001010000000000110000;
        /// MMU enable for EL3 stage 1 address translation.
        const M = 1 << 0;
        /// Alignment check enable.
        const A = 1 << 1;
        /// Cacheability control, for data accesses at EL3.
        const C = 1 << 2;
        /// SP alignment check enable.
        const SA = 1 << 3;
        /// `nAA` bit.
        const NAA = 1 << 6;
        /// `EOS` bit.
        const EOS = 1 << 11;
        /// Cacheability control, for instruction accesses at EL3.
        const I = 1 << 12;
        /// `EnDB` bit.
        const ENDB = 1 << 13;
        /// Write permission implies XN (Execute-never). For the EL3 translation regime, this bit can force all memory regions that are writable to be treated as XN.
        const WXN = 1 << 19;
        /// Enable Implicit Error Synchronization events.
        const IESB = 1 << 21;
        /// `EIS` bit.
        const EIS = 1 << 22;
        /// `EnDA` bit.
        const ENDA = 1 << 27;
        /// Enable pointer authentication using APIBKey_EL1.
        const ENIB = 1 << 30;
        /// Enable pointer authentication using APIAKey_EL1.
        const ENIA = 1 << 31;
        /// `BT` bit.
        const BT = 1 << 36;
        /// `ITFSB` bit.
        const ITFSB = 1 << 37;
        /// `ATA` bit.
        const ATA = 1 << 43;
        /// `DSSBS` bit.
        const DSSBS = 1 << 44;
        /// `TCSO` bit.
        const TCSO = 1 << 59;
        /// `NMI` bit.
        const NMI = 1 << 61;
        /// `SPINTMASK` bit.
        const SPINTMASK = 1 << 62;
    }
}

impl SctlrEl3 {
    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        ((self.bits() >> 40) & 0b11) as u8
    }
}

bitflags! {
    /// `SMCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SmcrEl3: u64 {
        /// `EZT0` bit.
        const EZT0 = 1 << 30;
        /// `FA64` bit.
        const FA64 = 1 << 31;
    }
}

impl SmcrEl3 {
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }
}

bitflags! {
    /// `SPSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrEl1: u64 {
        /// `M[4]` bit.
        const M_4 = 1 << 4;
        /// `T` bit.
        const T = 1 << 5;
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `D` bit.
        const D = 1 << 9;
        /// `E` bit.
        const E = 1 << 9;
        /// `ALLINT` bit.
        const ALLINT = 1 << 13;
        /// `BTYPE2` bit.
        const BTYPE2 = 1 << 14;
        /// `IL` bit.
        const IL = 1 << 20;
        /// `SS` bit.
        const SS = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `UAO` bit.
        const UAO = 1 << 23;
        /// `DIT` bit.
        const DIT = 1 << 24;
        /// `TCO` bit.
        const TCO = 1 << 25;
        /// `Q` bit.
        const Q = 1 << 27;
        /// `V` bit.
        const V = 1 << 28;
        /// `C` bit.
        const C = 1 << 29;
        /// `Z` bit.
        const Z = 1 << 30;
        /// `N` bit.
        const N = 1 << 31;
        /// `PM` bit.
        const PM = 1 << 32;
        /// `PPEND` bit.
        const PPEND = 1 << 33;
        /// `EXLOCK` bit.
        const EXLOCK = 1 << 34;
        /// `PACM` bit.
        const PACM = 1 << 35;
        /// `UINJ` bit.
        const UINJ = 1 << 36;
    }
}

impl SpsrEl1 {
    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }
}

bitflags! {
    /// `SPSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrEl2: u64 {
        /// `M[4]` bit.
        const M_4 = 1 << 4;
        /// `T` bit.
        const T = 1 << 5;
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `D` bit.
        const D = 1 << 9;
        /// `E` bit.
        const E = 1 << 9;
        /// `ALLINT` bit.
        const ALLINT = 1 << 13;
        /// `BTYPE2` bit.
        const BTYPE2 = 1 << 14;
        /// `IL` bit.
        const IL = 1 << 20;
        /// `SS` bit.
        const SS = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `UAO` bit.
        const UAO = 1 << 23;
        /// `DIT` bit.
        const DIT = 1 << 24;
        /// `TCO` bit.
        const TCO = 1 << 25;
        /// `Q` bit.
        const Q = 1 << 27;
        /// `V` bit.
        const V = 1 << 28;
        /// `C` bit.
        const C = 1 << 29;
        /// `Z` bit.
        const Z = 1 << 30;
        /// `N` bit.
        const N = 1 << 31;
        /// `PM` bit.
        const PM = 1 << 32;
        /// `PPEND` bit.
        const PPEND = 1 << 33;
        /// `EXLOCK` bit.
        const EXLOCK = 1 << 34;
        /// `PACM` bit.
        const PACM = 1 << 35;
        /// `UINJ` bit.
        const UINJ = 1 << 36;
    }
}

impl SpsrEl2 {
    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }
}

bitflags! {
    /// `SPSR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrEl3: u64 {
        /// `M[4]` bit.
        const M_4 = 1 << 4;
        /// `T` bit.
        const T = 1 << 5;
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `D` bit.
        const D = 1 << 9;
        /// `E` bit.
        const E = 1 << 9;
        /// `ALLINT` bit.
        const ALLINT = 1 << 13;
        /// `BTYPE2` bit.
        const BTYPE2 = 1 << 14;
        /// `IL` bit.
        const IL = 1 << 20;
        /// `SS` bit.
        const SS = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `UAO` bit.
        const UAO = 1 << 23;
        /// `DIT` bit.
        const DIT = 1 << 24;
        /// `TCO` bit.
        const TCO = 1 << 25;
        /// `Q` bit.
        const Q = 1 << 27;
        /// `V` bit.
        const V = 1 << 28;
        /// `C` bit.
        const C = 1 << 29;
        /// `Z` bit.
        const Z = 1 << 30;
        /// `N` bit.
        const N = 1 << 31;
        /// `PM` bit.
        const PM = 1 << 32;
        /// `PPEND` bit.
        const PPEND = 1 << 33;
        /// `EXLOCK` bit.
        const EXLOCK = 1 << 34;
        /// `PACM` bit.
        const PACM = 1 << 35;
        /// `UINJ` bit.
        const UINJ = 1 << 36;
    }
}

impl SpsrEl3 {
    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }
}

bitflags! {
    /// `SP_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl1: u64 {
    }
}

impl SpEl1 {
    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `SP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl2: u64 {
    }
}

impl SpEl2 {
    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `TCR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tcr2El1: u64 {
        /// `PnCH` bit.
        const PNCH = 1 << 0;
        /// `PIE` bit.
        const PIE = 1 << 1;
        /// `E0POE` bit.
        const E0POE = 1 << 2;
        /// `POE` bit.
        const POE = 1 << 3;
        /// `AIE` bit.
        const AIE = 1 << 4;
        /// `D128` bit.
        const D128 = 1 << 5;
        /// `PTTWI` bit.
        const PTTWI = 1 << 10;
        /// `HAFT` bit.
        const HAFT = 1 << 11;
        /// `DisCH0` bit.
        const DISCH0 = 1 << 14;
        /// `DisCH1` bit.
        const DISCH1 = 1 << 15;
        /// `A2` bit.
        const A2 = 1 << 16;
        /// `FNG0` bit.
        const FNG0 = 1 << 17;
        /// `FNG1` bit.
        const FNG1 = 1 << 18;
        /// `POE2F` bit.
        const POE2F = 1 << 19;
        /// `FNGNA0` bit.
        const FNGNA0 = 1 << 20;
        /// `FNGNA1` bit.
        const FNGNA1 = 1 << 21;
        /// `TVAD0` bit.
        const TVAD0 = 1 << 35;
        /// `TVAD1` bit.
        const TVAD1 = 1 << 36;
    }
}

impl Tcr2El1 {
    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> 22) & 0b111) as u8
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        ((self.bits() >> 25) & 0b11111) as u8
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        ((self.bits() >> 30) & 0b11111) as u8
    }
}

bitflags! {
    /// `TCR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tcr2El2: u64 {
        /// `PnCH` bit.
        const PNCH = 1 << 0;
        /// `PIE` bit.
        const PIE = 1 << 1;
        /// `E0POE` bit.
        const E0POE = 1 << 2;
        /// `POE` bit.
        const POE = 1 << 3;
        /// `AIE` bit.
        const AIE = 1 << 4;
        /// `D128` bit.
        const D128 = 1 << 5;
        /// `PTTWI` bit.
        const PTTWI = 1 << 10;
        /// `HAFT` bit.
        const HAFT = 1 << 11;
        /// `AMEC0` bit.
        const AMEC0 = 1 << 12;
        /// `AMEC1` bit.
        const AMEC1 = 1 << 13;
        /// `DisCH0` bit.
        const DISCH0 = 1 << 14;
        /// `DisCH1` bit.
        const DISCH1 = 1 << 15;
        /// `A2` bit.
        const A2 = 1 << 16;
        /// `FNG0` bit.
        const FNG0 = 1 << 17;
        /// `FNG1` bit.
        const FNG1 = 1 << 18;
        /// `POE2F` bit.
        const POE2F = 1 << 19;
        /// `TVAD0` bit.
        const TVAD0 = 1 << 35;
        /// `TVAD1` bit.
        const TVAD1 = 1 << 36;
    }
}

impl Tcr2El2 {
    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> 22) & 0b111) as u8
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        ((self.bits() >> 25) & 0b11111) as u8
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        ((self.bits() >> 30) & 0b11111) as u8
    }
}

bitflags! {
    /// `TCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrEl1: u64 {
        /// `EPD0` bit.
        const EPD0 = 1 << 7;
        /// `A1` bit.
        const A1 = 1 << 22;
        /// `EPD1` bit.
        const EPD1 = 1 << 23;
        /// `AS` bit.
        const AS = 1 << 36;
        /// `TBI0` bit.
        const TBI0 = 1 << 37;
        /// `TBI1` bit.
        const TBI1 = 1 << 38;
        /// `HA` bit.
        const HA = 1 << 39;
        /// `HD` bit.
        const HD = 1 << 40;
        /// `HPD0` bit.
        const HPD0 = 1 << 41;
        /// `HPD1` bit.
        const HPD1 = 1 << 42;
        /// `HWU059` bit.
        const HWU059 = 1 << 43;
        /// `HWU060` bit.
        const HWU060 = 1 << 44;
        /// `HWU061` bit.
        const HWU061 = 1 << 45;
        /// `HWU062` bit.
        const HWU062 = 1 << 46;
        /// `HWU159` bit.
        const HWU159 = 1 << 47;
        /// `HWU160` bit.
        const HWU160 = 1 << 48;
        /// `HWU161` bit.
        const HWU161 = 1 << 49;
        /// `HWU162` bit.
        const HWU162 = 1 << 50;
        /// `TBID0` bit.
        const TBID0 = 1 << 51;
        /// `TBID1` bit.
        const TBID1 = 1 << 52;
        /// `NFD0` bit.
        const NFD0 = 1 << 53;
        /// `NFD1` bit.
        const NFD1 = 1 << 54;
        /// `E0PD0` bit.
        const E0PD0 = 1 << 55;
        /// `E0PD1` bit.
        const E0PD1 = 1 << 56;
        /// `TCMA0` bit.
        const TCMA0 = 1 << 57;
        /// `TCMA1` bit.
        const TCMA1 = 1 << 58;
        /// `DS` bit.
        const DS = 1 << 59;
        /// `MTX0` bit.
        const MTX0 = 1 << 60;
        /// `MTX1` bit.
        const MTX1 = 1 << 61;
    }
}

impl TcrEl1 {
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> 0) & 0b111111) as u8
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> 8) & 0b11) as u8
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> 12) & 0b11) as u8
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        ((self.bits() >> 16) & 0b111111) as u8
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        ((self.bits() >> 24) & 0b11) as u8
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        ((self.bits() >> 26) & 0b11) as u8
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        ((self.bits() >> 28) & 0b11) as u8
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        ((self.bits() >> 30) & 0b11) as u8
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        ((self.bits() >> 32) & 0b111) as u8
    }
}

bitflags! {
    /// `TCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrEl2: u64 {
        /// RES1 bits in the `TCR_EL2` register.
        const RES1 = 0b10000000100000000000000000000000;
        /// `EPD0` bit.
        const EPD0 = 1 << 7;
        /// `TBI` bit.
        const TBI = 1 << 20;
        /// `A1` bit.
        const A1 = 1 << 22;
        /// `EPD1` bit.
        const EPD1 = 1 << 23;
        /// `HPD` bit.
        const HPD = 1 << 24;
        /// `HWU59` bit.
        const HWU59 = 1 << 25;
        /// `HWU60` bit.
        const HWU60 = 1 << 26;
        /// `HWU61` bit.
        const HWU61 = 1 << 27;
        /// `HWU62` bit.
        const HWU62 = 1 << 28;
        /// `TBID` bit.
        const TBID = 1 << 29;
        /// `TCMA` bit.
        const TCMA = 1 << 30;
        /// `MTX` bit.
        const MTX = 1 << 33;
        /// `AS` bit.
        const AS = 1 << 36;
        /// `TBI0` bit.
        const TBI0 = 1 << 37;
        /// `TBI1` bit.
        const TBI1 = 1 << 38;
        /// `HPD0` bit.
        const HPD0 = 1 << 41;
        /// `HPD1` bit.
        const HPD1 = 1 << 42;
        /// `HWU059` bit.
        const HWU059 = 1 << 43;
        /// `HWU060` bit.
        const HWU060 = 1 << 44;
        /// `HWU061` bit.
        const HWU061 = 1 << 45;
        /// `HWU062` bit.
        const HWU062 = 1 << 46;
        /// `HWU159` bit.
        const HWU159 = 1 << 47;
        /// `HWU160` bit.
        const HWU160 = 1 << 48;
        /// `HWU161` bit.
        const HWU161 = 1 << 49;
        /// `HWU162` bit.
        const HWU162 = 1 << 50;
        /// `TBID0` bit.
        const TBID0 = 1 << 51;
        /// `TBID1` bit.
        const TBID1 = 1 << 52;
        /// `NFD0` bit.
        const NFD0 = 1 << 53;
        /// `TVAD` bit.
        const TVAD = 1 << 53;
        /// `NFD1` bit.
        const NFD1 = 1 << 54;
        /// `E0PD0` bit.
        const E0PD0 = 1 << 55;
        /// `E0PD1` bit.
        const E0PD1 = 1 << 56;
        /// `TCMA0` bit.
        const TCMA0 = 1 << 57;
        /// `TCMA1` bit.
        const TCMA1 = 1 << 58;
        /// `MTX0` bit.
        const MTX0 = 1 << 60;
        /// `MTX1` bit.
        const MTX1 = 1 << 61;
    }
}

impl TcrEl2 {
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> 0) & 0b111111) as u8
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> 8) & 0b11) as u8
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> 12) & 0b11) as u8
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> 16) & 0b111) as u8
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        ((self.bits() >> 16) & 0b111111) as u8
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        ((self.bits() >> 24) & 0b11) as u8
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        ((self.bits() >> 26) & 0b11) as u8
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        ((self.bits() >> 28) & 0b11) as u8
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        ((self.bits() >> 30) & 0b11) as u8
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        ((self.bits() >> 32) & 0b111) as u8
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        ((self.bits() >> 48) & 0b11111) as u8
    }
}

bitflags! {
    /// `TCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrEl3: u64 {
        /// RES1 bits in the `TCR_EL3` register.
        const RES1 = 0b10000000100000000000000000000000;
        /// `TBI` bit.
        const TBI = 1 << 20;
        /// `HA` bit.
        const HA = 1 << 21;
        /// `HD` bit.
        const HD = 1 << 22;
        /// `HPD` bit.
        const HPD = 1 << 24;
        /// `HWU59` bit.
        const HWU59 = 1 << 25;
        /// `HWU60` bit.
        const HWU60 = 1 << 26;
        /// `HWU61` bit.
        const HWU61 = 1 << 27;
        /// `HWU62` bit.
        const HWU62 = 1 << 28;
        /// `TBID` bit.
        const TBID = 1 << 29;
        /// `TCMA` bit.
        const TCMA = 1 << 30;
        /// `DS` bit.
        const DS = 1 << 32;
        /// `MTX` bit.
        const MTX = 1 << 33;
        /// `PnCH` bit.
        const PNCH = 1 << 34;
        /// `PIE` bit.
        const PIE = 1 << 35;
        /// `POE` bit.
        const POE = 1 << 36;
        /// `AIE` bit.
        const AIE = 1 << 37;
        /// `D128` bit.
        const D128 = 1 << 38;
        /// `PTTWI` bit.
        const PTTWI = 1 << 41;
        /// `HAFT` bit.
        const HAFT = 1 << 42;
        /// `DisCH0` bit.
        const DISCH0 = 1 << 43;
        /// `POE2F` bit.
        const POE2F = 1 << 44;
        /// `TVAD` bit.
        const TVAD = 1 << 53;
    }
}

impl TcrEl3 {
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> 0) & 0b111111) as u8
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> 8) & 0b11) as u8
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> 12) & 0b11) as u8
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> 16) & 0b111) as u8
    }

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> 45) & 0b111) as u8
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        ((self.bits() >> 48) & 0b11111) as u8
    }
}

bitflags! {
    /// `TFSRE0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tfsre0El1: u64 {
        /// `TF0` bit.
        const TF0 = 1 << 0;
        /// `TF1` bit.
        const TF1 = 1 << 1;
    }
}

bitflags! {
    /// `TFSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TfsrEl1: u64 {
        /// `TF0` bit.
        const TF0 = 1 << 0;
        /// `TF1` bit.
        const TF1 = 1 << 1;
    }
}

bitflags! {
    /// `TFSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TfsrEl2: u64 {
        /// `TF0` bit.
        const TF0 = 1 << 0;
        /// `TF1` bit.
        const TF1 = 1 << 1;
    }
}

bitflags! {
    /// `TPIDRRO_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrroEl0: u64 {
    }
}

impl TpidrroEl0 {
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `TPIDR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl0: u64 {
    }
}

impl TpidrEl0 {
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `TPIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl1: u64 {
    }
}

impl TpidrEl1 {
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `TPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl2: u64 {
    }
}

impl TpidrEl2 {
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> 0) & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `TTBR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr0El1 {
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> 1) & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> 1) & 0b11) as u8
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `TTBR0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr0El2 {
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> 1) & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> 1) & 0b11) as u8
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `TTBR0_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El3: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr0El3 {
    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> 1) & 0b11) as u8
    }
}

bitflags! {
    /// `TTBR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1El1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr1El1 {
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> 1) & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> 1) & 0b11) as u8
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `TTBR1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1El2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr1El2 {
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> 1) & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> 1) & 0b11) as u8
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `VBAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl1: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

impl VbarEl1 {
    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        ((self.bits() >> 11) & 0b11111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `VBAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl2: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

impl VbarEl2 {
    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        ((self.bits() >> 11) & 0b11111111111111111111111111111111111111111111111111111) as u64
    }
}

bitflags! {
    /// `VDISR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VdisrEl2: u64 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `IDS` bit.
        const IDS = 1 << 24;
        /// `A` bit.
        const A = 1 << 31;
    }
}

impl VdisrEl2 {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> 0) & 0b111111111111111111111111) as u32
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> 0) & 0b111111) as u8
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }
}

bitflags! {
    /// `VMPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VmpidrEl2: u64 {
        /// RES1 bits in the `VMPIDR_EL2` register.
        const RES1 = 0b10000000000000000000000000000000;
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
    }
}

impl VmpidrEl2 {
    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> 0) & 0b11111111) as u8
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> 8) & 0b11111111) as u8
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> 16) & 0b11111111) as u8
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        ((self.bits() >> 32) & 0b11111111) as u8
    }
}

bitflags! {
    /// `VPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VpidrEl2: u64 {
    }
}

impl VpidrEl2 {
    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> 4) & 0b111111111111) as u16
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> 16) & 0b1111) as u8
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> 20) & 0b1111) as u8
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> 24) & 0b11111111) as u8
    }
}

bitflags! {
    /// `VSESR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VsesrEl2: u64 {
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `IDS` bit.
        const IDS = 1 << 24;
    }
}

impl VsesrEl2 {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> 0) & 0b111111111111111111111111) as u32
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }
}

bitflags! {
    /// `VTCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VtcrEl2: u64 {
        /// RES1 bits in the `VTCR_EL2` register.
        const RES1 = 0b10000000000000000000000000000000;
        /// `VS` bit.
        const VS = 1 << 19;
        /// `HA` bit.
        const HA = 1 << 21;
        /// `HD` bit.
        const HD = 1 << 22;
        /// `HWU59` bit.
        const HWU59 = 1 << 25;
        /// `HWU60` bit.
        const HWU60 = 1 << 26;
        /// `HWU61` bit.
        const HWU61 = 1 << 27;
        /// `HWU62` bit.
        const HWU62 = 1 << 28;
        /// `NSW` bit.
        const NSW = 1 << 29;
        /// `NSA` bit.
        const NSA = 1 << 30;
        /// `DS` bit.
        const DS = 1 << 32;
        /// `SL2` bit.
        const SL2 = 1 << 33;
        /// `AssuredOnly` bit.
        const ASSUREDONLY = 1 << 34;
        /// `TL1` bit.
        const TL1 = 1 << 35;
        /// `S2PIE` bit.
        const S2PIE = 1 << 36;
        /// `S2POE` bit.
        const S2POE = 1 << 37;
        /// `D128` bit.
        const D128 = 1 << 38;
        /// `GCSH` bit.
        const GCSH = 1 << 40;
        /// `TL0` bit.
        const TL0 = 1 << 41;
        /// `HAFT` bit.
        const HAFT = 1 << 44;
        /// `HDBSS` bit.
        const HDBSS = 1 << 45;
    }
}

impl VtcrEl2 {
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> 0) & 0b111111) as u8
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        ((self.bits() >> 6) & 0b11) as u8
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> 8) & 0b11) as u8
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> 10) & 0b11) as u8
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> 12) & 0b11) as u8
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> 14) & 0b11) as u8
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> 16) & 0b111) as u8
    }
}

bitflags! {
    /// `VTTBR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VttbrEl2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl VttbrEl2 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        ((self.bits() >> 1) & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> 1) & 0b11) as u8
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u16 {
        ((self.bits() >> 48) & 0b1111111111111111) as u16
    }
}

bitflags! {
    /// `ZCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ZcrEl3: u64 {
    }
}

impl ZcrEl3 {
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        ((self.bits() >> 0) & 0b1111) as u8
    }
}

read_write_sysreg!(actlr_el1, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(actlr_el2, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(afsr0_el1, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(afsr0_el2, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(afsr1_el1, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(afsr1_el2, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(amair_el1, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(amair_el2, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(apiakeyhi_el1, u64: ApiakeyhiEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(apiakeylo_el1, u64: ApiakeyloEl1, safe_read, fake::SYSREGS);
read_sysreg!(ccsidr_el1, u64: CcsidrEl1, safe, fake::SYSREGS);
read_sysreg!(clidr_el1, u64: ClidrEl1, safe, fake::SYSREGS);
read_write_sysreg!(cntfrq_el0, u64: CntfrqEl0, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(cnthctl_el2, u64: CnthctlEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(cntvoff_el2, u64: CntvoffEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(contextidr_el1, u64: ContextidrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(contextidr_el2, u64: ContextidrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(cpacr_el1, u64: CpacrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(cptr_el2, u64: CptrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cptr_el3, u64: CptrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(csselr_el1, u64: CsselrEl1, safe_read, safe_write, fake::SYSREGS);
read_sysreg!(ctr_el0, u64: CtrEl0, safe, fake::SYSREGS);
read_write_sysreg!(disr_el1, u64: DisrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(dit, u64: Dit, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(elr_el1, u64: ElrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(elr_el2, u64: ElrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(esr_el1, u64: EsrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(esr_el2, u64: EsrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(esr_el3, u64: EsrEl3, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(far_el1, u64: FarEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(far_el2, u64: FarEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(gcr_el1, u64: GcrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(gcscr_el1, u64: GcscrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(gcscr_el2, u64: GcscrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hacr_el2, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(hcrx_el2, u64: HcrxEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hcr_el2, u64: HcrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hdfgrtr2_el2, u64: Hdfgrtr2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(hdfgwtr2_el2, u64: Hdfgwtr2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(hfgitr2_el2, u64: Hfgitr2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(hfgrtr2_el2, u64: Hfgrtr2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(hfgwtr2_el2, u64: Hfgwtr2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(hfgwtr_el2, u64: HfgwtrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hpfar_el2, u64: HpfarEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hstr_el2, u64, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(icc_sre_el1, u64: IccSreEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_sre_el2, u64: IccSreEl2, safe_read, fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The SRE bit of `icc_sre_el3` must not be changed from 1 to 0, as this can result in unpredictable behaviour.
    icc_sre_el3, u64: IccSreEl3, safe_read, fake::SYSREGS
}
read_write_sysreg!(ich_hcr_el2, u64: IchHcrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(ich_vmcr_el2, u64: IchVmcrEl2, safe_read, safe_write, fake::SYSREGS);
read_sysreg!(id_aa64dfr0_el1, u64: IdAa64dfr0El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64dfr1_el1, u64: IdAa64dfr1El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64mmfr0_el1, u64: IdAa64mmfr0El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64mmfr1_el1, u64: IdAa64mmfr1El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64mmfr2_el1, u64: IdAa64mmfr2El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64mmfr3_el1, u64: IdAa64mmfr3El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64pfr0_el1, u64: IdAa64pfr0El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64pfr1_el1, u64: IdAa64pfr1El1, safe, fake::SYSREGS);
read_sysreg!(id_aa64smfr0_el1, u64: IdAa64smfr0El1, safe, fake::SYSREGS);
read_sysreg!(isr_el1, u64: IsrEl1, safe, fake::SYSREGS);
read_write_sysreg!(mair_el1, u64: MairEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(mair_el2, u64: MairEl2, safe_read, fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 memory attribute indirection register.
    mair_el3, u64: MairEl3, safe_read, fake::SYSREGS
}
read_write_sysreg!(mdccint_el1, u64: MdccintEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(mdcr_el2, u64: MdcrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(mdcr_el3, u64: MdcrEl3, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(mdscr_el1, u64: MdscrEl1, safe_read, safe_write, fake::SYSREGS);
read_sysreg!(midr_el1, u64: MidrEl1, safe, fake::SYSREGS);
read_write_sysreg!(mpam2_el2, u64: Mpam2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpam3_el3, u64: Mpam3El3, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamhcr_el2, u64: MpamhcrEl2, safe_read, fake::SYSREGS);
read_sysreg!(mpamidr_el1, u64: MpamidrEl1, safe, fake::SYSREGS);
read_write_sysreg!(mpamvpm0_el2, u64: Mpamvpm0El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpm1_el2, u64: Mpamvpm1El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpm2_el2, u64: Mpamvpm2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpm3_el2, u64: Mpamvpm3El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpm4_el2, u64: Mpamvpm4El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpm5_el2, u64: Mpamvpm5El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpm6_el2, u64: Mpamvpm6El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpm7_el2, u64: Mpamvpm7El2, safe_read, fake::SYSREGS);
read_write_sysreg!(mpamvpmv_el2, u64: MpamvpmvEl2, safe_read, fake::SYSREGS);
read_sysreg!(mpidr_el1, u64: MpidrEl1, safe, fake::SYSREGS);
read_write_sysreg!(par_el1, u64: ParEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(pmcr_el0, u64: PmcrEl0, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(rgsr_el1, u64: RgsrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(scr_el3, u64: ScrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(sctlr_el1, u64: SctlrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(sctlr_el2, u64: SctlrEl2, safe_read, fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 system control register.
    sctlr_el3, u64: SctlrEl3, safe_read, fake::SYSREGS
}
read_write_sysreg!(smcr_el3, u64: SmcrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(spsr_el1, u64: SpsrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(spsr_el2, u64: SpsrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(spsr_el3, u64: SpsrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(sp_el1, u64: SpEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(sp_el2, u64: SpEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(tcr2_el1, u64: Tcr2El1, safe_read, fake::SYSREGS);
read_write_sysreg!(tcr2_el2, u64: Tcr2El2, safe_read, fake::SYSREGS);
read_write_sysreg!(tcr_el1, u64: TcrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(tcr_el2, u64: TcrEl2, safe_read, fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 translation control register.
    tcr_el3, u64: TcrEl3, safe_read, fake::SYSREGS
}
read_write_sysreg!(tfsre0_el1, u64: Tfsre0El1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(tfsr_el1, u64: TfsrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(tfsr_el2, u64: TfsrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(tpidrro_el0, u64: TpidrroEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidr_el0, u64: TpidrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidr_el1, u64: TpidrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidr_el2, u64: TpidrEl2, safe_read, fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el1, u64: Ttbr0El1, safe_read, fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el2, u64: Ttbr0El2, safe_read, fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el3, u64: Ttbr0El3, safe_read, fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el1, u64: Ttbr1El1, safe_read, fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el2, u64: Ttbr1El2, safe_read, fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el1, u64: VbarEl1, safe_read, fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el2, u64: VbarEl2, safe_read, fake::SYSREGS
}
read_write_sysreg!(vdisr_el2, u64: VdisrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(vmpidr_el2, u64: VmpidrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(vpidr_el2, u64: VpidrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(vsesr_el2, u64: VsesrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(vtcr_el2, u64: VtcrEl2, safe_read, fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned stage 2 translation table.
    vttbr_el2, u64: VttbrEl2, safe_read, fake::SYSREGS
}
read_write_sysreg!(zcr_el3, u64: ZcrEl3, safe_read, fake::SYSREGS);
