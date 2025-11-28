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

#[cfg(feature = "el1")]
bitflags! {
    /// `APIAKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyhiEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApiakeyhiEl1 {
    /// Offset of the APIAKeyHi field.
    pub const APIAKEYHI_SHIFT: u32 = 0;
    /// Mask for the APIAKeyHi field.
    pub const APIAKEYHI_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `APIAKeyHi` field.
    pub const fn apiakeyhi(self) -> u64 {
        (self.bits() >> Self::APIAKEYHI_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APIAKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyloEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApiakeyloEl1 {
    /// Offset of the APIAKeyLo field.
    pub const APIAKEYLO_SHIFT: u32 = 0;
    /// Mask for the APIAKeyLo field.
    pub const APIAKEYLO_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `APIAKeyLo` field.
    pub const fn apiakeylo(self) -> u64 {
        (self.bits() >> Self::APIAKEYLO_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CCSIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CcsidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl CcsidrEl1 {
    /// Offset of the LineSize field.
    pub const LINESIZE_SHIFT: u32 = 0;
    /// Mask for the LineSize field.
    pub const LINESIZE_MASK: u64 = 0b111;

    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        (self.bits() >> Self::LINESIZE_SHIFT) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CLIDR_EL1` system register value.
    ///
    /// Cache Level ID.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ClidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ClidrEl1 {
    /// Offset of the Ctype<n> field.
    pub const CTYPE_SHIFT: u32 = 0;
    /// Mask for the Ctype<n> field.
    pub const CTYPE_MASK: u64 = 0b111;
    /// Offset of the LoUIS field.
    pub const LOUIS_SHIFT: u32 = 21;
    /// Mask for the LoUIS field.
    pub const LOUIS_MASK: u64 = 0b111;
    /// Offset of the LoC field.
    pub const LOC_SHIFT: u32 = 24;
    /// Mask for the LoC field.
    pub const LOC_MASK: u64 = 0b111;
    /// Offset of the LoUU field.
    pub const LOUU_SHIFT: u32 = 27;
    /// Mask for the LoUU field.
    pub const LOUU_MASK: u64 = 0b111;
    /// Offset of the ICB field.
    pub const ICB_SHIFT: u32 = 30;
    /// Mask for the ICB field.
    pub const ICB_MASK: u64 = 0b111;
    /// Offset of the Ttype<n> field.
    pub const TTYPE_SHIFT: u32 = 33;
    /// Mask for the Ttype<n> field.
    pub const TTYPE_MASK: u64 = 0b11;

    /// Returns the value of the given `Ctype<n>` field.
    pub const fn ctype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        (self.bits() >> (Self::CTYPE_SHIFT + (n - 1) * 3)) as u8 & 0b111
    }

    /// Returns the value of the `LoUIS` field.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn louis(self) -> u8 {
        (self.bits() >> Self::LOUIS_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `LoC` field.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn loc(self) -> u8 {
        (self.bits() >> Self::LOC_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `LoUU` field.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn louu(self) -> u8 {
        (self.bits() >> Self::LOUU_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `ICB` field.
    ///
    /// Inner cache boundary level.
    pub const fn icb(self) -> u8 {
        (self.bits() >> Self::ICB_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the given `Ttype<n>` field.
    pub const fn ttype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        (self.bits() >> (Self::TTYPE_SHIFT + (n - 1) * 2)) as u8 & 0b11
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
    /// Offset of the ClockFreq field.
    pub const CLOCKFREQ_SHIFT: u32 = 0;
    /// Mask for the ClockFreq field.
    pub const CLOCKFREQ_MASK: u64 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ClockFreq` field.
    pub const fn clockfreq(self) -> u32 {
        (self.bits() >> Self::CLOCKFREQ_SHIFT) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl CnthctlEl2 {
    /// Offset of the EL0PCTEN field.
    pub const EL0PCTEN_SHIFT: u32 = 0;
    /// Offset of the EL0VCTEN field.
    pub const EL0VCTEN_SHIFT: u32 = 1;
    /// Offset of the EL1PCEN field.
    pub const EL1PCEN_SHIFT: u32 = 1;
    /// Offset of the EVNTEN field.
    pub const EVNTEN_SHIFT: u32 = 2;
    /// Offset of the EVNTDIR field.
    pub const EVNTDIR_SHIFT: u32 = 3;
    /// Offset of the EVNTI field.
    pub const EVNTI_SHIFT: u32 = 4;
    /// Mask for the EVNTI field.
    pub const EVNTI_MASK: u64 = 0b1111;
    /// Offset of the EL0VTEN field.
    pub const EL0VTEN_SHIFT: u32 = 8;
    /// Offset of the EL0PTEN field.
    pub const EL0PTEN_SHIFT: u32 = 9;
    /// Offset of the EL1PTEN field.
    pub const EL1PTEN_SHIFT: u32 = 11;
    /// Offset of the ECV field.
    pub const ECV_SHIFT: u32 = 12;
    /// Offset of the EL1TVT field.
    pub const EL1TVT_SHIFT: u32 = 13;
    /// Offset of the EL1TVCT field.
    pub const EL1TVCT_SHIFT: u32 = 14;
    /// Offset of the EL1NVPCT field.
    pub const EL1NVPCT_SHIFT: u32 = 15;
    /// Offset of the EL1NVVCT field.
    pub const EL1NVVCT_SHIFT: u32 = 16;
    /// Offset of the EVNTIS field.
    pub const EVNTIS_SHIFT: u32 = 17;
    /// Offset of the CNTVMASK field.
    pub const CNTVMASK_SHIFT: u32 = 18;
    /// Offset of the CNTPMASK field.
    pub const CNTPMASK_SHIFT: u32 = 19;

    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        (self.bits() >> Self::EVNTI_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTVOFF_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvoffEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CntvoffEl2 {
    /// Offset of the VOffset field.
    pub const VOFFSET_SHIFT: u32 = 0;
    /// Mask for the VOffset field.
    pub const VOFFSET_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        (self.bits() >> Self::VOFFSET_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CONTEXTIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ContextidrEl1 {
    /// Offset of the PROCID field.
    pub const PROCID_SHIFT: u32 = 0;
    /// Mask for the PROCID field.
    pub const PROCID_MASK: u64 = 0b11111111111111111111111111111111;

    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        (self.bits() >> Self::PROCID_SHIFT) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CONTEXTIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl ContextidrEl2 {
    /// Offset of the PROCID field.
    pub const PROCID_SHIFT: u32 = 0;
    /// Mask for the PROCID field.
    pub const PROCID_MASK: u64 = 0b11111111111111111111111111111111;

    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        (self.bits() >> Self::PROCID_SHIFT) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl CpacrEl1 {
    /// Offset of the ZEN field.
    pub const ZEN_SHIFT: u32 = 16;
    /// Mask for the ZEN field.
    pub const ZEN_MASK: u64 = 0b11;
    /// Offset of the FPEN field.
    pub const FPEN_SHIFT: u32 = 20;
    /// Mask for the FPEN field.
    pub const FPEN_MASK: u64 = 0b11;
    /// Offset of the SMEN field.
    pub const SMEN_SHIFT: u32 = 24;
    /// Mask for the SMEN field.
    pub const SMEN_MASK: u64 = 0b11;
    /// Offset of the TTA field.
    pub const TTA_SHIFT: u32 = 28;
    /// Offset of the E0POE field.
    pub const E0POE_SHIFT: u32 = 29;
    /// Offset of the TAM field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the TCPAC field.
    pub const TCPAC_SHIFT: u32 = 31;
    /// Offset of the E0TP0E field.
    pub const E0TP0E_SHIFT: u32 = 32;
    /// Offset of the E0TP1E field.
    pub const E0TP1E_SHIFT: u32 = 33;

    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        (self.bits() >> Self::ZEN_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        (self.bits() >> Self::FPEN_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        (self.bits() >> Self::SMEN_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl CptrEl2 {
    /// Offset of the TZ field.
    pub const TZ_SHIFT: u32 = 8;
    /// Offset of the TFP field.
    pub const TFP_SHIFT: u32 = 10;
    /// Offset of the TSM field.
    pub const TSM_SHIFT: u32 = 12;
    /// Offset of the ZEN field.
    pub const ZEN_SHIFT: u32 = 16;
    /// Mask for the ZEN field.
    pub const ZEN_MASK: u64 = 0b11;
    /// Offset of the FPEN field.
    pub const FPEN_SHIFT: u32 = 20;
    /// Mask for the FPEN field.
    pub const FPEN_MASK: u64 = 0b11;
    /// Offset of the SMEN field.
    pub const SMEN_SHIFT: u32 = 24;
    /// Mask for the SMEN field.
    pub const SMEN_MASK: u64 = 0b11;
    /// Offset of the E0POE field.
    pub const E0POE_SHIFT: u32 = 29;
    /// Offset of the TAM field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the TCPAC field.
    pub const TCPAC_SHIFT: u32 = 31;
    /// Offset of the E0TP0E field.
    pub const E0TP0E_SHIFT: u32 = 32;
    /// Offset of the E0TP1E field.
    pub const E0TP1E_SHIFT: u32 = 33;

    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        (self.bits() >> Self::ZEN_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        (self.bits() >> Self::FPEN_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        (self.bits() >> Self::SMEN_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl CptrEl3 {
    /// Offset of the EZ field.
    pub const EZ_SHIFT: u32 = 8;
    /// Offset of the TFP field.
    pub const TFP_SHIFT: u32 = 10;
    /// Offset of the ESM field.
    pub const ESM_SHIFT: u32 = 12;
    /// Offset of the TTA field.
    pub const TTA_SHIFT: u32 = 20;
    /// Offset of the TAM field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the TCPAC field.
    pub const TCPAC_SHIFT: u32 = 31;
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl CsselrEl1 {
    /// Offset of the InD field.
    pub const IND_SHIFT: u32 = 0;
    /// Offset of the Level field.
    pub const LEVEL_SHIFT: u32 = 1;
    /// Mask for the Level field.
    pub const LEVEL_MASK: u64 = 0b111;
    /// Offset of the TnD field.
    pub const TND_SHIFT: u32 = 4;

    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        (self.bits() >> Self::LEVEL_SHIFT) as u8 & 0b111
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
    /// Offset of the IminLine field.
    pub const IMINLINE_SHIFT: u32 = 0;
    /// Mask for the IminLine field.
    pub const IMINLINE_MASK: u64 = 0b1111;
    /// Offset of the L1Ip field.
    pub const L1IP_SHIFT: u32 = 14;
    /// Mask for the L1Ip field.
    pub const L1IP_MASK: u64 = 0b11;
    /// Offset of the DminLine field.
    pub const DMINLINE_SHIFT: u32 = 16;
    /// Mask for the DminLine field.
    pub const DMINLINE_MASK: u64 = 0b1111;
    /// Offset of the ERG field.
    pub const ERG_SHIFT: u32 = 20;
    /// Mask for the ERG field.
    pub const ERG_MASK: u64 = 0b1111;
    /// Offset of the CWG field.
    pub const CWG_SHIFT: u32 = 24;
    /// Mask for the CWG field.
    pub const CWG_MASK: u64 = 0b1111;
    /// Offset of the IDC field.
    pub const IDC_SHIFT: u32 = 28;
    /// Offset of the DIC field.
    pub const DIC_SHIFT: u32 = 29;
    /// Offset of the TminLine field.
    pub const TMINLINE_SHIFT: u32 = 32;
    /// Mask for the TminLine field.
    pub const TMINLINE_MASK: u64 = 0b111111;

    /// Returns the value of the `IminLine` field.
    pub const fn iminline(self) -> u8 {
        (self.bits() >> Self::IMINLINE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `L1Ip` field.
    pub const fn l1ip(self) -> u8 {
        (self.bits() >> Self::L1IP_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `DminLine` field.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn dminline(self) -> u8 {
        (self.bits() >> Self::DMINLINE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ERG` field.
    pub const fn erg(self) -> u8 {
        (self.bits() >> Self::ERG_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `CWG` field.
    pub const fn cwg(self) -> u8 {
        (self.bits() >> Self::CWG_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TminLine` field.
    pub const fn tminline(self) -> u8 {
        (self.bits() >> Self::TMINLINE_SHIFT) as u8 & 0b111111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl DisrEl1 {
    /// Offset of the DFSC field.
    pub const DFSC_SHIFT: u32 = 0;
    /// Mask for the DFSC field.
    pub const DFSC_MASK: u64 = 0b111111;
    /// Offset of the WnR field.
    pub const WNR_SHIFT: u32 = 6;
    /// Offset of the WnRV field.
    pub const WNRV_SHIFT: u32 = 7;
    /// Offset of the EA field.
    pub const EA_SHIFT: u32 = 9;
    /// Offset of the AET field.
    pub const AET_SHIFT: u32 = 10;
    /// Mask for the AET field.
    pub const AET_MASK: u64 = 0b111;
    /// Offset of the WU field.
    pub const WU_SHIFT: u32 = 16;
    /// Mask for the WU field.
    pub const WU_MASK: u64 = 0b11;
    /// Offset of the IDS field.
    pub const IDS_SHIFT: u32 = 24;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `DFSC` field.
    pub const fn dfsc(self) -> u8 {
        (self.bits() >> Self::DFSC_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> Self::AET_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `WU` field.
    pub const fn wu(self) -> u8 {
        (self.bits() >> Self::WU_SHIFT) as u8 & 0b11
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

impl Dit {
    /// Offset of the DIT field.
    pub const DIT_SHIFT: u32 = 24;
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ElrEl1 {
    /// Offset of the ADDR field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the ADDR field.
    pub const ADDR_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> Self::ADDR_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ELR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl ElrEl2 {
    /// Offset of the ADDR field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the ADDR field.
    pub const ADDR_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> Self::ADDR_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ESR_EL1` system register value.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl1: u64 {
        /// `IL` bit.
        const IL = 1 << 25;
    }
}

#[cfg(feature = "el1")]
impl EsrEl1 {
    /// Offset of the ISS field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the ISS field.
    pub const ISS_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the IL field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the EC field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the EC field.
    pub const EC_MASK: u64 = 0b111111;
    /// Offset of the ISS2 field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the ISS2 field.
    pub const ISS2_MASK: u64 = 0b111111111111111111111111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> Self::ISS_SHIFT) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> Self::EC_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        (self.bits() >> Self::ISS2_SHIFT) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ESR_EL2` system register value.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl2: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

#[cfg(feature = "el2")]
impl EsrEl2 {
    /// Offset of the ISS field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the ISS field.
    pub const ISS_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the IL field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the EC field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the EC field.
    pub const EC_MASK: u64 = 0b111111;
    /// Offset of the ISS2 field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the ISS2 field.
    pub const ISS2_MASK: u64 = 0b111111111111111111111111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> Self::ISS_SHIFT) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> Self::EC_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        (self.bits() >> Self::ISS2_SHIFT) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ESR_EL3` system register value.
    #[derive(Clone, Copy, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl3: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

#[cfg(feature = "el3")]
impl EsrEl3 {
    /// Offset of the ISS field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the ISS field.
    pub const ISS_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the IL field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the EC field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the EC field.
    pub const EC_MASK: u64 = 0b111111;
    /// Offset of the ISS2 field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the ISS2 field.
    pub const ISS2_MASK: u64 = 0b111111111111111111111111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> Self::ISS_SHIFT) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> Self::EC_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        (self.bits() >> Self::ISS2_SHIFT) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `FAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl FarEl1 {
    /// Offset of the VA field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the VA field.
    pub const VA_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        (self.bits() >> Self::VA_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `FAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl FarEl2 {
    /// Offset of the VA field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the VA field.
    pub const VA_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        (self.bits() >> Self::VA_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `GCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcrEl1: u64 {
        /// `RRND` bit.
        const RRND = 1 << 16;
    }
}

#[cfg(feature = "el1")]
impl GcrEl1 {
    /// Offset of the Exclude field.
    pub const EXCLUDE_SHIFT: u32 = 0;
    /// Mask for the Exclude field.
    pub const EXCLUDE_MASK: u64 = 0b1111111111111111;
    /// Offset of the RRND field.
    pub const RRND_SHIFT: u32 = 16;

    /// Returns the value of the `Exclude` field.
    pub const fn exclude(self) -> u16 {
        (self.bits() >> Self::EXCLUDE_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl GcscrEl1 {
    /// Offset of the PCRSEL field.
    pub const PCRSEL_SHIFT: u32 = 0;
    /// Offset of the RVCHKEN field.
    pub const RVCHKEN_SHIFT: u32 = 5;
    /// Offset of the EXLOCKEN field.
    pub const EXLOCKEN_SHIFT: u32 = 6;
    /// Offset of the PUSHMEn field.
    pub const PUSHMEN_SHIFT: u32 = 8;
    /// Offset of the STREn field.
    pub const STREN_SHIFT: u32 = 9;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl GcscrEl2 {
    /// Offset of the PCRSEL field.
    pub const PCRSEL_SHIFT: u32 = 0;
    /// Offset of the RVCHKEN field.
    pub const RVCHKEN_SHIFT: u32 = 5;
    /// Offset of the EXLOCKEN field.
    pub const EXLOCKEN_SHIFT: u32 = 6;
    /// Offset of the PUSHMEn field.
    pub const PUSHMEN_SHIFT: u32 = 8;
    /// Offset of the STREn field.
    pub const STREN_SHIFT: u32 = 9;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl HcrxEl2 {
    /// Offset of the EnAS0 field.
    pub const ENAS0_SHIFT: u32 = 0;
    /// Offset of the EnALS field.
    pub const ENALS_SHIFT: u32 = 1;
    /// Offset of the EnASR field.
    pub const ENASR_SHIFT: u32 = 2;
    /// Offset of the FnXS field.
    pub const FNXS_SHIFT: u32 = 3;
    /// Offset of the FGTnXS field.
    pub const FGTNXS_SHIFT: u32 = 4;
    /// Offset of the SMPME field.
    pub const SMPME_SHIFT: u32 = 5;
    /// Offset of the TALLINT field.
    pub const TALLINT_SHIFT: u32 = 6;
    /// Offset of the VINMI field.
    pub const VINMI_SHIFT: u32 = 7;
    /// Offset of the VFNMI field.
    pub const VFNMI_SHIFT: u32 = 8;
    /// Offset of the CMOW field.
    pub const CMOW_SHIFT: u32 = 9;
    /// Offset of the MCE2 field.
    pub const MCE2_SHIFT: u32 = 10;
    /// Offset of the MSCEn field.
    pub const MSCEN_SHIFT: u32 = 11;
    /// Offset of the TCR2En field.
    pub const TCR2EN_SHIFT: u32 = 14;
    /// Offset of the SCTLR2En field.
    pub const SCTLR2EN_SHIFT: u32 = 15;
    /// Offset of the PTTWI field.
    pub const PTTWI_SHIFT: u32 = 16;
    /// Offset of the D128En field.
    pub const D128EN_SHIFT: u32 = 17;
    /// Offset of the EnSNERR field.
    pub const ENSNERR_SHIFT: u32 = 18;
    /// Offset of the TMEA field.
    pub const TMEA_SHIFT: u32 = 19;
    /// Offset of the EnSDERR field.
    pub const ENSDERR_SHIFT: u32 = 20;
    /// Offset of the EnIDCP128 field.
    pub const ENIDCP128_SHIFT: u32 = 21;
    /// Offset of the GCSEn field.
    pub const GCSEN_SHIFT: u32 = 22;
    /// Offset of the EnFPM field.
    pub const ENFPM_SHIFT: u32 = 23;
    /// Offset of the PACMEn field.
    pub const PACMEN_SHIFT: u32 = 24;
    /// Offset of the VTLBIDEn field.
    pub const VTLBIDEN_SHIFT: u32 = 25;
    /// Offset of the SRMASKEn field.
    pub const SRMASKEN_SHIFT: u32 = 26;
    /// Offset of the NVTGE field.
    pub const NVTGE_SHIFT: u32 = 27;
    /// Offset of the POE2En field.
    pub const POE2EN_SHIFT: u32 = 29;
    /// Offset of the TPLIMEn field.
    pub const TPLIMEN_SHIFT: u32 = 30;
    /// Offset of the FDIT field.
    pub const FDIT_SHIFT: u32 = 31;
    /// Offset of the NVnTTLB field.
    pub const NVNTTLB_SHIFT: u32 = 32;
    /// Offset of the NVnTTLBIS field.
    pub const NVNTTLBIS_SHIFT: u32 = 33;
    /// Offset of the NVnTTLBOS field.
    pub const NVNTTLBOS_SHIFT: u32 = 34;
    /// Offset of the VTLBIDOSEn field.
    pub const VTLBIDOSEN_SHIFT: u32 = 35;
    /// Offset of the FNB field.
    pub const FNB_SHIFT: u32 = 36;
    /// Offset of the VTE field.
    pub const VTE_SHIFT: u32 = 37;
    /// Offset of the VTAO field.
    pub const VTAO_SHIFT: u32 = 38;
    /// Offset of the VTCO field.
    pub const VTCO_SHIFT: u32 = 39;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl HcrEl2 {
    /// Offset of the VM field.
    pub const VM_SHIFT: u32 = 0;
    /// Offset of the SWIO field.
    pub const SWIO_SHIFT: u32 = 1;
    /// Offset of the PTW field.
    pub const PTW_SHIFT: u32 = 2;
    /// Offset of the FMO field.
    pub const FMO_SHIFT: u32 = 3;
    /// Offset of the IMO field.
    pub const IMO_SHIFT: u32 = 4;
    /// Offset of the AMO field.
    pub const AMO_SHIFT: u32 = 5;
    /// Offset of the VF field.
    pub const VF_SHIFT: u32 = 6;
    /// Offset of the VI field.
    pub const VI_SHIFT: u32 = 7;
    /// Offset of the VSE field.
    pub const VSE_SHIFT: u32 = 8;
    /// Offset of the FB field.
    pub const FB_SHIFT: u32 = 9;
    /// Offset of the BSU field.
    pub const BSU_SHIFT: u32 = 10;
    /// Mask for the BSU field.
    pub const BSU_MASK: u64 = 0b11;
    /// Offset of the DC field.
    pub const DC_SHIFT: u32 = 12;
    /// Offset of the TWI field.
    pub const TWI_SHIFT: u32 = 13;
    /// Offset of the TWE field.
    pub const TWE_SHIFT: u32 = 14;
    /// Offset of the TID0 field.
    pub const TID0_SHIFT: u32 = 15;
    /// Offset of the TID1 field.
    pub const TID1_SHIFT: u32 = 16;
    /// Offset of the TID2 field.
    pub const TID2_SHIFT: u32 = 17;
    /// Offset of the TID3 field.
    pub const TID3_SHIFT: u32 = 18;
    /// Offset of the TSC field.
    pub const TSC_SHIFT: u32 = 19;
    /// Offset of the TIDCP field.
    pub const TIDCP_SHIFT: u32 = 20;
    /// Offset of the TACR field.
    pub const TACR_SHIFT: u32 = 21;
    /// Offset of the TSW field.
    pub const TSW_SHIFT: u32 = 22;
    /// Offset of the TPCP field.
    pub const TPCP_SHIFT: u32 = 23;
    /// Offset of the TPU field.
    pub const TPU_SHIFT: u32 = 24;
    /// Offset of the TTLB field.
    pub const TTLB_SHIFT: u32 = 25;
    /// Offset of the TVM field.
    pub const TVM_SHIFT: u32 = 26;
    /// Offset of the TGE field.
    pub const TGE_SHIFT: u32 = 27;
    /// Offset of the TDZ field.
    pub const TDZ_SHIFT: u32 = 28;
    /// Offset of the HCD field.
    pub const HCD_SHIFT: u32 = 29;
    /// Offset of the TRVM field.
    pub const TRVM_SHIFT: u32 = 30;
    /// Offset of the RW field.
    pub const RW_SHIFT: u32 = 31;
    /// Offset of the CD field.
    pub const CD_SHIFT: u32 = 32;
    /// Offset of the ID field.
    pub const ID_SHIFT: u32 = 33;
    /// Offset of the E2H field.
    pub const E2H_SHIFT: u32 = 34;
    /// Offset of the TLOR field.
    pub const TLOR_SHIFT: u32 = 35;
    /// Offset of the TERR field.
    pub const TERR_SHIFT: u32 = 36;
    /// Offset of the TEA field.
    pub const TEA_SHIFT: u32 = 37;
    /// Offset of the APK field.
    pub const APK_SHIFT: u32 = 40;
    /// Offset of the API field.
    pub const API_SHIFT: u32 = 41;
    /// Offset of the NV field.
    pub const NV_SHIFT: u32 = 42;
    /// Offset of the NV1 field.
    pub const NV1_SHIFT: u32 = 43;
    /// Offset of the AT field.
    pub const AT_SHIFT: u32 = 44;
    /// Offset of the NV2 field.
    pub const NV2_SHIFT: u32 = 45;
    /// Offset of the FWB field.
    pub const FWB_SHIFT: u32 = 46;
    /// Offset of the FIEN field.
    pub const FIEN_SHIFT: u32 = 47;
    /// Offset of the GPF field.
    pub const GPF_SHIFT: u32 = 48;
    /// Offset of the TID4 field.
    pub const TID4_SHIFT: u32 = 49;
    /// Offset of the TICAB field.
    pub const TICAB_SHIFT: u32 = 50;
    /// Offset of the AMVOFFEN field.
    pub const AMVOFFEN_SHIFT: u32 = 51;
    /// Offset of the TOCU field.
    pub const TOCU_SHIFT: u32 = 52;
    /// Offset of the EnSCXT field.
    pub const ENSCXT_SHIFT: u32 = 53;
    /// Offset of the TTLBIS field.
    pub const TTLBIS_SHIFT: u32 = 54;
    /// Offset of the TTLBOS field.
    pub const TTLBOS_SHIFT: u32 = 55;
    /// Offset of the ATA field.
    pub const ATA_SHIFT: u32 = 56;
    /// Offset of the DCT field.
    pub const DCT_SHIFT: u32 = 57;
    /// Offset of the TID5 field.
    pub const TID5_SHIFT: u32 = 58;
    /// Offset of the TWEDEn field.
    pub const TWEDEN_SHIFT: u32 = 59;
    /// Offset of the TWEDEL field.
    pub const TWEDEL_SHIFT: u32 = 60;
    /// Mask for the TWEDEL field.
    pub const TWEDEL_MASK: u64 = 0b1111;

    /// Returns the value of the `BSU` field.
    pub const fn bsu(self) -> u8 {
        (self.bits() >> Self::BSU_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> Self::TWEDEL_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Hdfgrtr2El2 {
    /// Offset of the nPMECR_EL1 field.
    pub const NPMECR_EL1_SHIFT: u32 = 0;
    /// Offset of the nPMIAR_EL1 field.
    pub const NPMIAR_EL1_SHIFT: u32 = 1;
    /// Offset of the nPMICNTR_EL0 field.
    pub const NPMICNTR_EL0_SHIFT: u32 = 2;
    /// Offset of the nPMICFILTR_EL0 field.
    pub const NPMICFILTR_EL0_SHIFT: u32 = 3;
    /// Offset of the nPMUACR_EL1 field.
    pub const NPMUACR_EL1_SHIFT: u32 = 4;
    /// Offset of the nMDSELR_EL1 field.
    pub const NMDSELR_EL1_SHIFT: u32 = 5;
    /// Offset of the nPMSSDATA field.
    pub const NPMSSDATA_SHIFT: u32 = 6;
    /// Offset of the nPMSSCR_EL1 field.
    pub const NPMSSCR_EL1_SHIFT: u32 = 7;
    /// Offset of the nSPMEVCNTRn_EL0 field.
    pub const NSPMEVCNTRN_EL0_SHIFT: u32 = 8;
    /// Offset of the nSPMEVTYPERn_EL0 field.
    pub const NSPMEVTYPERN_EL0_SHIFT: u32 = 9;
    /// Offset of the nSPMSELR_EL0 field.
    pub const NSPMSELR_EL0_SHIFT: u32 = 10;
    /// Offset of the nSPMCNTEN field.
    pub const NSPMCNTEN_SHIFT: u32 = 11;
    /// Offset of the nSPMINTEN field.
    pub const NSPMINTEN_SHIFT: u32 = 12;
    /// Offset of the nSPMOVS field.
    pub const NSPMOVS_SHIFT: u32 = 13;
    /// Offset of the nSPMCR_EL0 field.
    pub const NSPMCR_EL0_SHIFT: u32 = 14;
    /// Offset of the nSPMACCESSR_EL1 field.
    pub const NSPMACCESSR_EL1_SHIFT: u32 = 15;
    /// Offset of the nSPMSCR_EL1 field.
    pub const NSPMSCR_EL1_SHIFT: u32 = 16;
    /// Offset of the nSPMID field.
    pub const NSPMID_SHIFT: u32 = 17;
    /// Offset of the nSPMDEVAFF_EL1 field.
    pub const NSPMDEVAFF_EL1_SHIFT: u32 = 18;
    /// Offset of the nPMSDSFR_EL1 field.
    pub const NPMSDSFR_EL1_SHIFT: u32 = 19;
    /// Offset of the nTRCITECR_EL1 field.
    pub const NTRCITECR_EL1_SHIFT: u32 = 20;
    /// Offset of the nTRBMPAM_EL1 field.
    pub const NTRBMPAM_EL1_SHIFT: u32 = 22;
    /// Offset of the nMDSTEPOP_EL1 field.
    pub const NMDSTEPOP_EL1_SHIFT: u32 = 23;
    /// Offset of the nPMBMAR_EL1 field.
    pub const NPMBMAR_EL1_SHIFT: u32 = 24;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Hdfgwtr2El2 {
    /// Offset of the nPMECR_EL1 field.
    pub const NPMECR_EL1_SHIFT: u32 = 0;
    /// Offset of the nPMIAR_EL1 field.
    pub const NPMIAR_EL1_SHIFT: u32 = 1;
    /// Offset of the nPMICNTR_EL0 field.
    pub const NPMICNTR_EL0_SHIFT: u32 = 2;
    /// Offset of the nPMICFILTR_EL0 field.
    pub const NPMICFILTR_EL0_SHIFT: u32 = 3;
    /// Offset of the nPMUACR_EL1 field.
    pub const NPMUACR_EL1_SHIFT: u32 = 4;
    /// Offset of the nMDSELR_EL1 field.
    pub const NMDSELR_EL1_SHIFT: u32 = 5;
    /// Offset of the nPMSSCR_EL1 field.
    pub const NPMSSCR_EL1_SHIFT: u32 = 7;
    /// Offset of the nSPMEVCNTRn_EL0 field.
    pub const NSPMEVCNTRN_EL0_SHIFT: u32 = 8;
    /// Offset of the nSPMEVTYPERn_EL0 field.
    pub const NSPMEVTYPERN_EL0_SHIFT: u32 = 9;
    /// Offset of the nSPMSELR_EL0 field.
    pub const NSPMSELR_EL0_SHIFT: u32 = 10;
    /// Offset of the nSPMCNTEN field.
    pub const NSPMCNTEN_SHIFT: u32 = 11;
    /// Offset of the nSPMINTEN field.
    pub const NSPMINTEN_SHIFT: u32 = 12;
    /// Offset of the nSPMOVS field.
    pub const NSPMOVS_SHIFT: u32 = 13;
    /// Offset of the nSPMCR_EL0 field.
    pub const NSPMCR_EL0_SHIFT: u32 = 14;
    /// Offset of the nSPMACCESSR_EL1 field.
    pub const NSPMACCESSR_EL1_SHIFT: u32 = 15;
    /// Offset of the nSPMSCR_EL1 field.
    pub const NSPMSCR_EL1_SHIFT: u32 = 16;
    /// Offset of the nPMSDSFR_EL1 field.
    pub const NPMSDSFR_EL1_SHIFT: u32 = 19;
    /// Offset of the nTRCITECR_EL1 field.
    pub const NTRCITECR_EL1_SHIFT: u32 = 20;
    /// Offset of the nPMZR_EL0 field.
    pub const NPMZR_EL0_SHIFT: u32 = 21;
    /// Offset of the nTRBMPAM_EL1 field.
    pub const NTRBMPAM_EL1_SHIFT: u32 = 22;
    /// Offset of the nMDSTEPOP_EL1 field.
    pub const NMDSTEPOP_EL1_SHIFT: u32 = 23;
    /// Offset of the nPMBMAR_EL1 field.
    pub const NPMBMAR_EL1_SHIFT: u32 = 24;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Hfgitr2El2 {
    /// Offset of the TSBCSYNC field.
    pub const TSBCSYNC_SHIFT: u32 = 0;
    /// Offset of the nDCCIVAPS field.
    pub const NDCCIVAPS_SHIFT: u32 = 1;
    /// Offset of the PLBIPERME1OS field.
    pub const PLBIPERME1OS_SHIFT: u32 = 2;
    /// Offset of the PLBIASIDE1OS field.
    pub const PLBIASIDE1OS_SHIFT: u32 = 3;
    /// Offset of the PLBIVMALLE1OS field.
    pub const PLBIVMALLE1OS_SHIFT: u32 = 4;
    /// Offset of the PLBIPERME1IS field.
    pub const PLBIPERME1IS_SHIFT: u32 = 5;
    /// Offset of the PLBIASIDE1IS field.
    pub const PLBIASIDE1IS_SHIFT: u32 = 6;
    /// Offset of the PLBIVMALLE1IS field.
    pub const PLBIVMALLE1IS_SHIFT: u32 = 7;
    /// Offset of the PLBIPERME1 field.
    pub const PLBIPERME1_SHIFT: u32 = 8;
    /// Offset of the PLBIASIDE1 field.
    pub const PLBIASIDE1_SHIFT: u32 = 9;
    /// Offset of the PLBIVMALLE1 field.
    pub const PLBIVMALLE1_SHIFT: u32 = 10;
    /// Offset of the PLBIPERMAE1OS field.
    pub const PLBIPERMAE1OS_SHIFT: u32 = 11;
    /// Offset of the PLBIPERMAE1IS field.
    pub const PLBIPERMAE1IS_SHIFT: u32 = 12;
    /// Offset of the PLBIPERMAE1 field.
    pub const PLBIPERMAE1_SHIFT: u32 = 13;
    /// Offset of the DCGBVA field.
    pub const DCGBVA_SHIFT: u32 = 14;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Hfgrtr2El2 {
    /// Offset of the nPFAR_EL1 field.
    pub const NPFAR_EL1_SHIFT: u32 = 0;
    /// Offset of the nERXGSR_EL1 field.
    pub const NERXGSR_EL1_SHIFT: u32 = 1;
    /// Offset of the nRCWSMASK_EL1 field.
    pub const NRCWSMASK_EL1_SHIFT: u32 = 2;
    /// Offset of the nCPACRMASK_EL1 field.
    pub const NCPACRMASK_EL1_SHIFT: u32 = 3;
    /// Offset of the nSCTLRMASK_EL1 field.
    pub const NSCTLRMASK_EL1_SHIFT: u32 = 4;
    /// Offset of the nSCTLR2MASK_EL1 field.
    pub const NSCTLR2MASK_EL1_SHIFT: u32 = 5;
    /// Offset of the nTCRMASK_EL1 field.
    pub const NTCRMASK_EL1_SHIFT: u32 = 6;
    /// Offset of the nTCR2MASK_EL1 field.
    pub const NTCR2MASK_EL1_SHIFT: u32 = 7;
    /// Offset of the nCPACRALIAS_EL1 field.
    pub const NCPACRALIAS_EL1_SHIFT: u32 = 8;
    /// Offset of the nSCTLRALIAS_EL1 field.
    pub const NSCTLRALIAS_EL1_SHIFT: u32 = 9;
    /// Offset of the nSCTLR2ALIAS_EL1 field.
    pub const NSCTLR2ALIAS_EL1_SHIFT: u32 = 10;
    /// Offset of the nTCRALIAS_EL1 field.
    pub const NTCRALIAS_EL1_SHIFT: u32 = 11;
    /// Offset of the nTCR2ALIAS_EL1 field.
    pub const NTCR2ALIAS_EL1_SHIFT: u32 = 12;
    /// Offset of the nACTLRMASK_EL1 field.
    pub const NACTLRMASK_EL1_SHIFT: u32 = 13;
    /// Offset of the nACTLRALIAS_EL1 field.
    pub const NACTLRALIAS_EL1_SHIFT: u32 = 14;
    /// Offset of the nTINDEX_EL0 field.
    pub const NTINDEX_EL0_SHIFT: u32 = 15;
    /// Offset of the nTINDEX_EL1 field.
    pub const NTINDEX_EL1_SHIFT: u32 = 16;
    /// Offset of the nSTINDEX_EL1 field.
    pub const NSTINDEX_EL1_SHIFT: u32 = 17;
    /// Offset of the nFGDTn_EL1 field.
    pub const NFGDTN_EL1_SHIFT: u32 = 18;
    /// Mask for the nFGDTn_EL1 field.
    pub const NFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the nTTTBRP_EL1 field.
    pub const NTTTBRP_EL1_SHIFT: u32 = 20;
    /// Offset of the nTTTBRU_EL1 field.
    pub const NTTTBRU_EL1_SHIFT: u32 = 21;
    /// Offset of the nIRTBRP_EL1 field.
    pub const NIRTBRP_EL1_SHIFT: u32 = 22;
    /// Offset of the nIRTBRU_EL1 field.
    pub const NIRTBRU_EL1_SHIFT: u32 = 23;
    /// Offset of the nDPOTBR1_EL1 field.
    pub const NDPOTBR1_EL1_SHIFT: u32 = 24;
    /// Offset of the nDPOTBR0_EL1 field.
    pub const NDPOTBR0_EL1_SHIFT: u32 = 25;
    /// Offset of the nTPMIN1_EL1 field.
    pub const NTPMIN1_EL1_SHIFT: u32 = 26;
    /// Offset of the nTPMIN0_EL1 field.
    pub const NTPMIN0_EL1_SHIFT: u32 = 27;
    /// Offset of the nTPMIN1_EL0 field.
    pub const NTPMIN1_EL0_SHIFT: u32 = 28;
    /// Offset of the nTPMIN0_EL0 field.
    pub const NTPMIN0_EL0_SHIFT: u32 = 29;
    /// Offset of the nTLBIDIDR_EL1 field.
    pub const NTLBIDIDR_EL1_SHIFT: u32 = 30;
    /// Offset of the nAFGDTn_EL1 field.
    pub const NAFGDTN_EL1_SHIFT: u32 = 31;
    /// Mask for the nAFGDTn_EL1 field.
    pub const NAFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the TFSR_EL1 field.
    pub const TFSR_EL1_SHIFT: u32 = 33;
    /// Offset of the RGSR_EL1 field.
    pub const RGSR_EL1_SHIFT: u32 = 34;
    /// Offset of the GCR_EL1 field.
    pub const GCR_EL1_SHIFT: u32 = 35;
    /// Offset of the nTPIDR3_EL0 field.
    pub const NTPIDR3_EL0_SHIFT: u32 = 36;
    /// Offset of the nTPIDR3_EL1 field.
    pub const NTPIDR3_EL1_SHIFT: u32 = 37;

    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        (self.bits() >> Self::NFGDTN_EL1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        (self.bits() >> Self::NAFGDTN_EL1_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Hfgwtr2El2 {
    /// Offset of the nPFAR_EL1 field.
    pub const NPFAR_EL1_SHIFT: u32 = 0;
    /// Offset of the nRCWSMASK_EL1 field.
    pub const NRCWSMASK_EL1_SHIFT: u32 = 2;
    /// Offset of the nCPACRMASK_EL1 field.
    pub const NCPACRMASK_EL1_SHIFT: u32 = 3;
    /// Offset of the nSCTLRMASK_EL1 field.
    pub const NSCTLRMASK_EL1_SHIFT: u32 = 4;
    /// Offset of the nSCTLR2MASK_EL1 field.
    pub const NSCTLR2MASK_EL1_SHIFT: u32 = 5;
    /// Offset of the nTCRMASK_EL1 field.
    pub const NTCRMASK_EL1_SHIFT: u32 = 6;
    /// Offset of the nTCR2MASK_EL1 field.
    pub const NTCR2MASK_EL1_SHIFT: u32 = 7;
    /// Offset of the nCPACRALIAS_EL1 field.
    pub const NCPACRALIAS_EL1_SHIFT: u32 = 8;
    /// Offset of the nSCTLRALIAS_EL1 field.
    pub const NSCTLRALIAS_EL1_SHIFT: u32 = 9;
    /// Offset of the nSCTLR2ALIAS_EL1 field.
    pub const NSCTLR2ALIAS_EL1_SHIFT: u32 = 10;
    /// Offset of the nTCRALIAS_EL1 field.
    pub const NTCRALIAS_EL1_SHIFT: u32 = 11;
    /// Offset of the nTCR2ALIAS_EL1 field.
    pub const NTCR2ALIAS_EL1_SHIFT: u32 = 12;
    /// Offset of the nACTLRMASK_EL1 field.
    pub const NACTLRMASK_EL1_SHIFT: u32 = 13;
    /// Offset of the nACTLRALIAS_EL1 field.
    pub const NACTLRALIAS_EL1_SHIFT: u32 = 14;
    /// Offset of the nTINDEX_EL0 field.
    pub const NTINDEX_EL0_SHIFT: u32 = 15;
    /// Offset of the nTINDEX_EL1 field.
    pub const NTINDEX_EL1_SHIFT: u32 = 16;
    /// Offset of the nSTINDEX_EL1 field.
    pub const NSTINDEX_EL1_SHIFT: u32 = 17;
    /// Offset of the nFGDTn_EL1 field.
    pub const NFGDTN_EL1_SHIFT: u32 = 18;
    /// Mask for the nFGDTn_EL1 field.
    pub const NFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the nTTTBRP_EL1 field.
    pub const NTTTBRP_EL1_SHIFT: u32 = 20;
    /// Offset of the nTTTBRU_EL1 field.
    pub const NTTTBRU_EL1_SHIFT: u32 = 21;
    /// Offset of the nIRTBRP_EL1 field.
    pub const NIRTBRP_EL1_SHIFT: u32 = 22;
    /// Offset of the nIRTBRU_EL1 field.
    pub const NIRTBRU_EL1_SHIFT: u32 = 23;
    /// Offset of the nDPOTBR1_EL1 field.
    pub const NDPOTBR1_EL1_SHIFT: u32 = 24;
    /// Offset of the nDPOTBR0_EL1 field.
    pub const NDPOTBR0_EL1_SHIFT: u32 = 25;
    /// Offset of the nTPMIN1_EL1 field.
    pub const NTPMIN1_EL1_SHIFT: u32 = 26;
    /// Offset of the nTPMIN0_EL1 field.
    pub const NTPMIN0_EL1_SHIFT: u32 = 27;
    /// Offset of the nTPMIN1_EL0 field.
    pub const NTPMIN1_EL0_SHIFT: u32 = 28;
    /// Offset of the nTPMIN0_EL0 field.
    pub const NTPMIN0_EL0_SHIFT: u32 = 29;
    /// Offset of the nAFGDTn_EL1 field.
    pub const NAFGDTN_EL1_SHIFT: u32 = 31;
    /// Mask for the nAFGDTn_EL1 field.
    pub const NAFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the TFSR_EL1 field.
    pub const TFSR_EL1_SHIFT: u32 = 33;
    /// Offset of the RGSR_EL1 field.
    pub const RGSR_EL1_SHIFT: u32 = 34;
    /// Offset of the GCR_EL1 field.
    pub const GCR_EL1_SHIFT: u32 = 35;
    /// Offset of the nTPIDR3_EL0 field.
    pub const NTPIDR3_EL0_SHIFT: u32 = 36;
    /// Offset of the nTPIDR3_EL1 field.
    pub const NTPIDR3_EL1_SHIFT: u32 = 37;

    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        (self.bits() >> Self::NFGDTN_EL1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        (self.bits() >> Self::NAFGDTN_EL1_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl HfgwtrEl2 {
    /// Offset of the AFSR0_EL1 field.
    pub const AFSR0_EL1_SHIFT: u32 = 0;
    /// Offset of the AFSR1_EL1 field.
    pub const AFSR1_EL1_SHIFT: u32 = 1;
    /// Offset of the AMAIR_EL1 field.
    pub const AMAIR_EL1_SHIFT: u32 = 3;
    /// Offset of the APDAKey field.
    pub const APDAKEY_SHIFT: u32 = 4;
    /// Offset of the APDBKey field.
    pub const APDBKEY_SHIFT: u32 = 5;
    /// Offset of the APGAKey field.
    pub const APGAKEY_SHIFT: u32 = 6;
    /// Offset of the APIAKey field.
    pub const APIAKEY_SHIFT: u32 = 7;
    /// Offset of the APIBKey field.
    pub const APIBKEY_SHIFT: u32 = 8;
    /// Offset of the CONTEXTIDR_EL1 field.
    pub const CONTEXTIDR_EL1_SHIFT: u32 = 11;
    /// Offset of the CPACR_EL1 field.
    pub const CPACR_EL1_SHIFT: u32 = 12;
    /// Offset of the CSSELR_EL1 field.
    pub const CSSELR_EL1_SHIFT: u32 = 13;
    /// Offset of the ESR_EL1 field.
    pub const ESR_EL1_SHIFT: u32 = 16;
    /// Offset of the FAR_EL1 field.
    pub const FAR_EL1_SHIFT: u32 = 17;
    /// Offset of the LORC_EL1 field.
    pub const LORC_EL1_SHIFT: u32 = 19;
    /// Offset of the LOREA_EL1 field.
    pub const LOREA_EL1_SHIFT: u32 = 20;
    /// Offset of the LORN_EL1 field.
    pub const LORN_EL1_SHIFT: u32 = 22;
    /// Offset of the LORSA_EL1 field.
    pub const LORSA_EL1_SHIFT: u32 = 23;
    /// Offset of the MAIR_EL1 field.
    pub const MAIR_EL1_SHIFT: u32 = 24;
    /// Offset of the PAR_EL1 field.
    pub const PAR_EL1_SHIFT: u32 = 27;
    /// Offset of the SCTLR_EL1 field.
    pub const SCTLR_EL1_SHIFT: u32 = 29;
    /// Offset of the SCXTNUM_EL1 field.
    pub const SCXTNUM_EL1_SHIFT: u32 = 30;
    /// Offset of the SCXTNUM_EL0 field.
    pub const SCXTNUM_EL0_SHIFT: u32 = 31;
    /// Offset of the TCR_EL1 field.
    pub const TCR_EL1_SHIFT: u32 = 32;
    /// Offset of the TPIDR_EL1 field.
    pub const TPIDR_EL1_SHIFT: u32 = 33;
    /// Offset of the TPIDRRO_EL0 field.
    pub const TPIDRRO_EL0_SHIFT: u32 = 34;
    /// Offset of the TPIDR_EL0 field.
    pub const TPIDR_EL0_SHIFT: u32 = 35;
    /// Offset of the TTBR0_EL1 field.
    pub const TTBR0_EL1_SHIFT: u32 = 36;
    /// Offset of the TTBR1_EL1 field.
    pub const TTBR1_EL1_SHIFT: u32 = 37;
    /// Offset of the VBAR_EL1 field.
    pub const VBAR_EL1_SHIFT: u32 = 38;
    /// Offset of the ICC_IGRPENn_EL1 field.
    pub const ICC_IGRPENN_EL1_SHIFT: u32 = 39;
    /// Offset of the ERRSELR_EL1 field.
    pub const ERRSELR_EL1_SHIFT: u32 = 41;
    /// Offset of the ERXCTLR_EL1 field.
    pub const ERXCTLR_EL1_SHIFT: u32 = 43;
    /// Offset of the ERXSTATUS_EL1 field.
    pub const ERXSTATUS_EL1_SHIFT: u32 = 44;
    /// Offset of the ERXMISCn_EL1 field.
    pub const ERXMISCN_EL1_SHIFT: u32 = 45;
    /// Offset of the ERXPFGCTL_EL1 field.
    pub const ERXPFGCTL_EL1_SHIFT: u32 = 47;
    /// Offset of the ERXPFGCDN_EL1 field.
    pub const ERXPFGCDN_EL1_SHIFT: u32 = 48;
    /// Offset of the ERXADDR_EL1 field.
    pub const ERXADDR_EL1_SHIFT: u32 = 49;
    /// Offset of the nACCDATA_EL1 field.
    pub const NACCDATA_EL1_SHIFT: u32 = 50;
    /// Offset of the nGCS_EL0 field.
    pub const NGCS_EL0_SHIFT: u32 = 52;
    /// Offset of the nGCS_EL1 field.
    pub const NGCS_EL1_SHIFT: u32 = 53;
    /// Offset of the nSMPRI_EL1 field.
    pub const NSMPRI_EL1_SHIFT: u32 = 54;
    /// Offset of the nTPIDR2_EL0 field.
    pub const NTPIDR2_EL0_SHIFT: u32 = 55;
    /// Offset of the nRCWMASK_EL1 field.
    pub const NRCWMASK_EL1_SHIFT: u32 = 56;
    /// Offset of the nPIRE0_EL1 field.
    pub const NPIRE0_EL1_SHIFT: u32 = 57;
    /// Offset of the nPIR_EL1 field.
    pub const NPIR_EL1_SHIFT: u32 = 58;
    /// Offset of the nPOR_EL0 field.
    pub const NPOR_EL0_SHIFT: u32 = 59;
    /// Offset of the nPOR_EL1 field.
    pub const NPOR_EL1_SHIFT: u32 = 60;
    /// Offset of the nS2POR_EL1 field.
    pub const NS2POR_EL1_SHIFT: u32 = 61;
    /// Offset of the nMAIR2_EL1 field.
    pub const NMAIR2_EL1_SHIFT: u32 = 62;
    /// Offset of the nAMAIR2_EL1 field.
    pub const NAMAIR2_EL1_SHIFT: u32 = 63;
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HPFAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HpfarEl2: u64 {
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

#[cfg(feature = "el2")]
impl HpfarEl2 {
    /// Offset of the FIPA field.
    pub const FIPA_SHIFT: u32 = 4;
    /// Mask for the FIPA field.
    pub const FIPA_MASK: u64 = 0b11111111111111111111111111111111111111111111;
    /// Offset of the NS field.
    pub const NS_SHIFT: u32 = 63;

    /// Returns the value of the `FIPA` field.
    pub const fn fipa(self) -> u64 {
        (self.bits() >> Self::FIPA_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl IccSreEl1 {
    /// Offset of the SRE field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the DFB field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the DIB field.
    pub const DIB_SHIFT: u32 = 2;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl IccSreEl2 {
    /// Offset of the SRE field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the DFB field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the DIB field.
    pub const DIB_SHIFT: u32 = 2;
    /// Offset of the Enable field.
    pub const ENABLE_SHIFT: u32 = 3;
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl IccSreEl3 {
    /// Offset of the SRE field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the DFB field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the DIB field.
    pub const DIB_SHIFT: u32 = 2;
    /// Offset of the Enable field.
    pub const ENABLE_SHIFT: u32 = 3;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl IchHcrEl2 {
    /// Offset of the En field.
    pub const EN_SHIFT: u32 = 0;
    /// Offset of the UIE field.
    pub const UIE_SHIFT: u32 = 1;
    /// Offset of the LRENPIE field.
    pub const LRENPIE_SHIFT: u32 = 2;
    /// Offset of the NPIE field.
    pub const NPIE_SHIFT: u32 = 3;
    /// Offset of the VGrp0EIE field.
    pub const VGRP0EIE_SHIFT: u32 = 4;
    /// Offset of the VGrp0DIE field.
    pub const VGRP0DIE_SHIFT: u32 = 5;
    /// Offset of the VGrp1EIE field.
    pub const VGRP1EIE_SHIFT: u32 = 6;
    /// Offset of the VGrp1DIE field.
    pub const VGRP1DIE_SHIFT: u32 = 7;
    /// Offset of the vSGIEOICount field.
    pub const VSGIEOICOUNT_SHIFT: u32 = 8;
    /// Offset of the TC field.
    pub const TC_SHIFT: u32 = 10;
    /// Offset of the TALL0 field.
    pub const TALL0_SHIFT: u32 = 11;
    /// Offset of the TALL1 field.
    pub const TALL1_SHIFT: u32 = 12;
    /// Offset of the TSEI field.
    pub const TSEI_SHIFT: u32 = 13;
    /// Offset of the TDIR field.
    pub const TDIR_SHIFT: u32 = 14;
    /// Offset of the DVIM field.
    pub const DVIM_SHIFT: u32 = 15;
    /// Offset of the EOIcount field.
    pub const EOICOUNT_SHIFT: u32 = 27;
    /// Mask for the EOIcount field.
    pub const EOICOUNT_MASK: u64 = 0b11111;

    /// Returns the value of the `EOIcount` field.
    pub const fn eoicount(self) -> u8 {
        (self.bits() >> Self::EOICOUNT_SHIFT) as u8 & 0b11111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl IchVmcrEl2 {
    /// Offset of the EN field.
    pub const EN_SHIFT: u32 = 0;
    /// Offset of the VENG0 field.
    pub const VENG0_SHIFT: u32 = 0;
    /// Offset of the VENG1 field.
    pub const VENG1_SHIFT: u32 = 1;
    /// Offset of the VAckCtl field.
    pub const VACKCTL_SHIFT: u32 = 2;
    /// Offset of the VFIQEn field.
    pub const VFIQEN_SHIFT: u32 = 3;
    /// Offset of the VCBPR field.
    pub const VCBPR_SHIFT: u32 = 4;
    /// Offset of the VEOIM field.
    pub const VEOIM_SHIFT: u32 = 9;
    /// Offset of the VBPR1 field.
    pub const VBPR1_SHIFT: u32 = 18;
    /// Mask for the VBPR1 field.
    pub const VBPR1_MASK: u64 = 0b111;
    /// Offset of the VBPR0 field.
    pub const VBPR0_SHIFT: u32 = 21;
    /// Mask for the VBPR0 field.
    pub const VBPR0_MASK: u64 = 0b111;

    /// Returns the value of the `VBPR1` field.
    pub const fn vbpr1(self) -> u8 {
        (self.bits() >> Self::VBPR1_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `VBPR0` field.
    pub const fn vbpr0(self) -> u8 {
        (self.bits() >> Self::VBPR0_SHIFT) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64DFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64dfr0El1 {
    /// Offset of the DebugVer field.
    pub const DEBUGVER_SHIFT: u32 = 0;
    /// Mask for the DebugVer field.
    pub const DEBUGVER_MASK: u64 = 0b1111;
    /// Offset of the TraceVer field.
    pub const TRACEVER_SHIFT: u32 = 4;
    /// Mask for the TraceVer field.
    pub const TRACEVER_MASK: u64 = 0b1111;
    /// Offset of the PMUVer field.
    pub const PMUVER_SHIFT: u32 = 8;
    /// Mask for the PMUVer field.
    pub const PMUVER_MASK: u64 = 0b1111;
    /// Offset of the BRPs field.
    pub const BRPS_SHIFT: u32 = 12;
    /// Mask for the BRPs field.
    pub const BRPS_MASK: u64 = 0b1111;
    /// Offset of the PMSS field.
    pub const PMSS_SHIFT: u32 = 16;
    /// Mask for the PMSS field.
    pub const PMSS_MASK: u64 = 0b1111;
    /// Offset of the WRPs field.
    pub const WRPS_SHIFT: u32 = 20;
    /// Mask for the WRPs field.
    pub const WRPS_MASK: u64 = 0b1111;
    /// Offset of the SEBEP field.
    pub const SEBEP_SHIFT: u32 = 24;
    /// Mask for the SEBEP field.
    pub const SEBEP_MASK: u64 = 0b1111;
    /// Offset of the CTX_CMPs field.
    pub const CTX_CMPS_SHIFT: u32 = 28;
    /// Mask for the CTX_CMPs field.
    pub const CTX_CMPS_MASK: u64 = 0b1111;
    /// Offset of the PMSVer field.
    pub const PMSVER_SHIFT: u32 = 32;
    /// Mask for the PMSVer field.
    pub const PMSVER_MASK: u64 = 0b1111;
    /// Offset of the DoubleLock field.
    pub const DOUBLELOCK_SHIFT: u32 = 36;
    /// Mask for the DoubleLock field.
    pub const DOUBLELOCK_MASK: u64 = 0b1111;
    /// Offset of the TraceFilt field.
    pub const TRACEFILT_SHIFT: u32 = 40;
    /// Mask for the TraceFilt field.
    pub const TRACEFILT_MASK: u64 = 0b1111;
    /// Offset of the TraceBuffer field.
    pub const TRACEBUFFER_SHIFT: u32 = 44;
    /// Mask for the TraceBuffer field.
    pub const TRACEBUFFER_MASK: u64 = 0b1111;
    /// Offset of the MTPMU field.
    pub const MTPMU_SHIFT: u32 = 48;
    /// Mask for the MTPMU field.
    pub const MTPMU_MASK: u64 = 0b1111;
    /// Offset of the BRBE field.
    pub const BRBE_SHIFT: u32 = 52;
    /// Mask for the BRBE field.
    pub const BRBE_MASK: u64 = 0b1111;
    /// Offset of the ExtTrcBuff field.
    pub const EXTTRCBUFF_SHIFT: u32 = 56;
    /// Mask for the ExtTrcBuff field.
    pub const EXTTRCBUFF_MASK: u64 = 0b1111;
    /// Offset of the HPMN0 field.
    pub const HPMN0_SHIFT: u32 = 60;
    /// Mask for the HPMN0 field.
    pub const HPMN0_MASK: u64 = 0b1111;

    /// Returns the value of the `DebugVer` field.
    pub const fn debugver(self) -> u8 {
        (self.bits() >> Self::DEBUGVER_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TraceVer` field.
    pub const fn tracever(self) -> u8 {
        (self.bits() >> Self::TRACEVER_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PMUVer` field.
    pub const fn pmuver(self) -> u8 {
        (self.bits() >> Self::PMUVER_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        (self.bits() >> Self::BRPS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PMSS` field.
    pub const fn pmss(self) -> u8 {
        (self.bits() >> Self::PMSS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        (self.bits() >> Self::WRPS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SEBEP` field.
    pub const fn sebep(self) -> u8 {
        (self.bits() >> Self::SEBEP_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        (self.bits() >> Self::CTX_CMPS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PMSVer` field.
    pub const fn pmsver(self) -> u8 {
        (self.bits() >> Self::PMSVER_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `DoubleLock` field.
    pub const fn doublelock(self) -> u8 {
        (self.bits() >> Self::DOUBLELOCK_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        (self.bits() >> Self::TRACEFILT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TraceBuffer` field.
    pub const fn tracebuffer(self) -> u8 {
        (self.bits() >> Self::TRACEBUFFER_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        (self.bits() >> Self::MTPMU_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BRBE` field.
    pub const fn brbe(self) -> u8 {
        (self.bits() >> Self::BRBE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ExtTrcBuff` field.
    pub const fn exttrcbuff(self) -> u8 {
        (self.bits() >> Self::EXTTRCBUFF_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        (self.bits() >> Self::HPMN0_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64DFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64dfr1El1 {
    /// Offset of the SYSPMUID field.
    pub const SYSPMUID_SHIFT: u32 = 0;
    /// Mask for the SYSPMUID field.
    pub const SYSPMUID_MASK: u64 = 0b11111111;
    /// Offset of the BRPs field.
    pub const BRPS_SHIFT: u32 = 8;
    /// Mask for the BRPs field.
    pub const BRPS_MASK: u64 = 0b11111111;
    /// Offset of the WRPs field.
    pub const WRPS_SHIFT: u32 = 16;
    /// Mask for the WRPs field.
    pub const WRPS_MASK: u64 = 0b11111111;
    /// Offset of the CTX_CMPs field.
    pub const CTX_CMPS_SHIFT: u32 = 24;
    /// Mask for the CTX_CMPs field.
    pub const CTX_CMPS_MASK: u64 = 0b11111111;
    /// Offset of the SPMU field.
    pub const SPMU_SHIFT: u32 = 32;
    /// Mask for the SPMU field.
    pub const SPMU_MASK: u64 = 0b1111;
    /// Offset of the PMICNTR field.
    pub const PMICNTR_SHIFT: u32 = 36;
    /// Mask for the PMICNTR field.
    pub const PMICNTR_MASK: u64 = 0b1111;
    /// Offset of the ABLE field.
    pub const ABLE_SHIFT: u32 = 40;
    /// Mask for the ABLE field.
    pub const ABLE_MASK: u64 = 0b1111;
    /// Offset of the ITE field.
    pub const ITE_SHIFT: u32 = 44;
    /// Mask for the ITE field.
    pub const ITE_MASK: u64 = 0b1111;
    /// Offset of the EBEP field.
    pub const EBEP_SHIFT: u32 = 48;
    /// Mask for the EBEP field.
    pub const EBEP_MASK: u64 = 0b1111;
    /// Offset of the DPFZS field.
    pub const DPFZS_SHIFT: u32 = 52;
    /// Mask for the DPFZS field.
    pub const DPFZS_MASK: u64 = 0b1111;
    /// Offset of the ABL_CMPs field.
    pub const ABL_CMPS_SHIFT: u32 = 56;
    /// Mask for the ABL_CMPs field.
    pub const ABL_CMPS_MASK: u64 = 0b11111111;

    /// Returns the value of the `SYSPMUID` field.
    pub const fn syspmuid(self) -> u8 {
        (self.bits() >> Self::SYSPMUID_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        (self.bits() >> Self::BRPS_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        (self.bits() >> Self::WRPS_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        (self.bits() >> Self::CTX_CMPS_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `SPMU` field.
    pub const fn spmu(self) -> u8 {
        (self.bits() >> Self::SPMU_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PMICNTR` field.
    pub const fn pmicntr(self) -> u8 {
        (self.bits() >> Self::PMICNTR_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ABLE` field.
    pub const fn able(self) -> u8 {
        (self.bits() >> Self::ABLE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ITE` field.
    pub const fn ite(self) -> u8 {
        (self.bits() >> Self::ITE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `EBEP` field.
    pub const fn ebep(self) -> u8 {
        (self.bits() >> Self::EBEP_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `DPFZS` field.
    pub const fn dpfzs(self) -> u8 {
        (self.bits() >> Self::DPFZS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ABL_CMPs` field.
    pub const fn abl_cmps(self) -> u8 {
        (self.bits() >> Self::ABL_CMPS_SHIFT) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr0El1 {
    /// Offset of the PARange field.
    pub const PARANGE_SHIFT: u32 = 0;
    /// Mask for the PARange field.
    pub const PARANGE_MASK: u64 = 0b1111;
    /// Offset of the ASIDBits field.
    pub const ASIDBITS_SHIFT: u32 = 4;
    /// Mask for the ASIDBits field.
    pub const ASIDBITS_MASK: u64 = 0b1111;
    /// Offset of the BigEnd field.
    pub const BIGEND_SHIFT: u32 = 8;
    /// Mask for the BigEnd field.
    pub const BIGEND_MASK: u64 = 0b1111;
    /// Offset of the SNSMem field.
    pub const SNSMEM_SHIFT: u32 = 12;
    /// Mask for the SNSMem field.
    pub const SNSMEM_MASK: u64 = 0b1111;
    /// Offset of the BigEndEL0 field.
    pub const BIGENDEL0_SHIFT: u32 = 16;
    /// Mask for the BigEndEL0 field.
    pub const BIGENDEL0_MASK: u64 = 0b1111;
    /// Offset of the TGran16 field.
    pub const TGRAN16_SHIFT: u32 = 20;
    /// Mask for the TGran16 field.
    pub const TGRAN16_MASK: u64 = 0b1111;
    /// Offset of the TGran64 field.
    pub const TGRAN64_SHIFT: u32 = 24;
    /// Mask for the TGran64 field.
    pub const TGRAN64_MASK: u64 = 0b1111;
    /// Offset of the TGran4 field.
    pub const TGRAN4_SHIFT: u32 = 28;
    /// Mask for the TGran4 field.
    pub const TGRAN4_MASK: u64 = 0b1111;
    /// Offset of the TGran16_2 field.
    pub const TGRAN16_2_SHIFT: u32 = 32;
    /// Mask for the TGran16_2 field.
    pub const TGRAN16_2_MASK: u64 = 0b1111;
    /// Offset of the TGran64_2 field.
    pub const TGRAN64_2_SHIFT: u32 = 36;
    /// Mask for the TGran64_2 field.
    pub const TGRAN64_2_MASK: u64 = 0b1111;
    /// Offset of the TGran4_2 field.
    pub const TGRAN4_2_SHIFT: u32 = 40;
    /// Mask for the TGran4_2 field.
    pub const TGRAN4_2_MASK: u64 = 0b1111;
    /// Offset of the ExS field.
    pub const EXS_SHIFT: u32 = 44;
    /// Mask for the ExS field.
    pub const EXS_MASK: u64 = 0b1111;
    /// Offset of the FGT field.
    pub const FGT_SHIFT: u32 = 56;
    /// Mask for the FGT field.
    pub const FGT_MASK: u64 = 0b1111;
    /// Offset of the ECV field.
    pub const ECV_SHIFT: u32 = 60;
    /// Mask for the ECV field.
    pub const ECV_MASK: u64 = 0b1111;

    /// Returns the value of the `PARange` field.
    pub const fn parange(self) -> u8 {
        (self.bits() >> Self::PARANGE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ASIDBits` field.
    pub const fn asidbits(self) -> u8 {
        (self.bits() >> Self::ASIDBITS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BigEnd` field.
    pub const fn bigend(self) -> u8 {
        (self.bits() >> Self::BIGEND_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SNSMem` field.
    pub const fn snsmem(self) -> u8 {
        (self.bits() >> Self::SNSMEM_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BigEndEL0` field.
    pub const fn bigendel0(self) -> u8 {
        (self.bits() >> Self::BIGENDEL0_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TGran16` field.
    pub const fn tgran16(self) -> u8 {
        (self.bits() >> Self::TGRAN16_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TGran64` field.
    pub const fn tgran64(self) -> u8 {
        (self.bits() >> Self::TGRAN64_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TGran4` field.
    pub const fn tgran4(self) -> u8 {
        (self.bits() >> Self::TGRAN4_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TGran16_2` field.
    pub const fn tgran16_2(self) -> u8 {
        (self.bits() >> Self::TGRAN16_2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TGran64_2` field.
    pub const fn tgran64_2(self) -> u8 {
        (self.bits() >> Self::TGRAN64_2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TGran4_2` field.
    pub const fn tgran4_2(self) -> u8 {
        (self.bits() >> Self::TGRAN4_2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ExS` field.
    pub const fn exs(self) -> u8 {
        (self.bits() >> Self::EXS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `FGT` field.
    pub const fn fgt(self) -> u8 {
        (self.bits() >> Self::FGT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ECV` field.
    pub const fn ecv(self) -> u8 {
        (self.bits() >> Self::ECV_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr1El1 {
    /// Offset of the HAFDBS field.
    pub const HAFDBS_SHIFT: u32 = 0;
    /// Mask for the HAFDBS field.
    pub const HAFDBS_MASK: u64 = 0b1111;
    /// Offset of the VMIDBits field.
    pub const VMIDBITS_SHIFT: u32 = 4;
    /// Mask for the VMIDBits field.
    pub const VMIDBITS_MASK: u64 = 0b1111;
    /// Offset of the VH field.
    pub const VH_SHIFT: u32 = 8;
    /// Mask for the VH field.
    pub const VH_MASK: u64 = 0b1111;
    /// Offset of the HPDS field.
    pub const HPDS_SHIFT: u32 = 12;
    /// Mask for the HPDS field.
    pub const HPDS_MASK: u64 = 0b1111;
    /// Offset of the LO field.
    pub const LO_SHIFT: u32 = 16;
    /// Mask for the LO field.
    pub const LO_MASK: u64 = 0b1111;
    /// Offset of the PAN field.
    pub const PAN_SHIFT: u32 = 20;
    /// Mask for the PAN field.
    pub const PAN_MASK: u64 = 0b1111;
    /// Offset of the SpecSEI field.
    pub const SPECSEI_SHIFT: u32 = 24;
    /// Mask for the SpecSEI field.
    pub const SPECSEI_MASK: u64 = 0b1111;
    /// Offset of the XNX field.
    pub const XNX_SHIFT: u32 = 28;
    /// Mask for the XNX field.
    pub const XNX_MASK: u64 = 0b1111;
    /// Offset of the TWED field.
    pub const TWED_SHIFT: u32 = 32;
    /// Mask for the TWED field.
    pub const TWED_MASK: u64 = 0b1111;
    /// Offset of the ETS field.
    pub const ETS_SHIFT: u32 = 36;
    /// Mask for the ETS field.
    pub const ETS_MASK: u64 = 0b1111;
    /// Offset of the HCX field.
    pub const HCX_SHIFT: u32 = 40;
    /// Mask for the HCX field.
    pub const HCX_MASK: u64 = 0b1111;
    /// Offset of the AFP field.
    pub const AFP_SHIFT: u32 = 44;
    /// Mask for the AFP field.
    pub const AFP_MASK: u64 = 0b1111;
    /// Offset of the nTLBPA field.
    pub const NTLBPA_SHIFT: u32 = 48;
    /// Mask for the nTLBPA field.
    pub const NTLBPA_MASK: u64 = 0b1111;
    /// Offset of the TIDCP1 field.
    pub const TIDCP1_SHIFT: u32 = 52;
    /// Mask for the TIDCP1 field.
    pub const TIDCP1_MASK: u64 = 0b1111;
    /// Offset of the CMOW field.
    pub const CMOW_SHIFT: u32 = 56;
    /// Mask for the CMOW field.
    pub const CMOW_MASK: u64 = 0b1111;
    /// Offset of the ECBHB field.
    pub const ECBHB_SHIFT: u32 = 60;
    /// Mask for the ECBHB field.
    pub const ECBHB_MASK: u64 = 0b1111;

    /// Returns the value of the `HAFDBS` field.
    pub const fn hafdbs(self) -> u8 {
        (self.bits() >> Self::HAFDBS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `VMIDBits` field.
    pub const fn vmidbits(self) -> u8 {
        (self.bits() >> Self::VMIDBITS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `VH` field.
    pub const fn vh(self) -> u8 {
        (self.bits() >> Self::VH_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        (self.bits() >> Self::HPDS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `LO` field.
    pub const fn lo(self) -> u8 {
        (self.bits() >> Self::LO_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        (self.bits() >> Self::PAN_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        (self.bits() >> Self::SPECSEI_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        (self.bits() >> Self::XNX_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TWED` field.
    pub const fn twed(self) -> u8 {
        (self.bits() >> Self::TWED_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        (self.bits() >> Self::ETS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `HCX` field.
    pub const fn hcx(self) -> u8 {
        (self.bits() >> Self::HCX_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `AFP` field.
    pub const fn afp(self) -> u8 {
        (self.bits() >> Self::AFP_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        (self.bits() >> Self::NTLBPA_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TIDCP1` field.
    pub const fn tidcp1(self) -> u8 {
        (self.bits() >> Self::TIDCP1_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `CMOW` field.
    pub const fn cmow(self) -> u8 {
        (self.bits() >> Self::CMOW_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ECBHB` field.
    pub const fn ecbhb(self) -> u8 {
        (self.bits() >> Self::ECBHB_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr2El1 {
    /// Offset of the CnP field.
    pub const CNP_SHIFT: u32 = 0;
    /// Mask for the CnP field.
    pub const CNP_MASK: u64 = 0b1111;
    /// Offset of the UAO field.
    pub const UAO_SHIFT: u32 = 4;
    /// Mask for the UAO field.
    pub const UAO_MASK: u64 = 0b1111;
    /// Offset of the LSM field.
    pub const LSM_SHIFT: u32 = 8;
    /// Mask for the LSM field.
    pub const LSM_MASK: u64 = 0b1111;
    /// Offset of the IESB field.
    pub const IESB_SHIFT: u32 = 12;
    /// Mask for the IESB field.
    pub const IESB_MASK: u64 = 0b1111;
    /// Offset of the VARange field.
    pub const VARANGE_SHIFT: u32 = 16;
    /// Mask for the VARange field.
    pub const VARANGE_MASK: u64 = 0b1111;
    /// Offset of the CCIDX field.
    pub const CCIDX_SHIFT: u32 = 20;
    /// Mask for the CCIDX field.
    pub const CCIDX_MASK: u64 = 0b1111;
    /// Offset of the NV field.
    pub const NV_SHIFT: u32 = 24;
    /// Mask for the NV field.
    pub const NV_MASK: u64 = 0b1111;
    /// Offset of the ST field.
    pub const ST_SHIFT: u32 = 28;
    /// Mask for the ST field.
    pub const ST_MASK: u64 = 0b1111;
    /// Offset of the AT field.
    pub const AT_SHIFT: u32 = 32;
    /// Mask for the AT field.
    pub const AT_MASK: u64 = 0b1111;
    /// Offset of the IDS field.
    pub const IDS_SHIFT: u32 = 36;
    /// Mask for the IDS field.
    pub const IDS_MASK: u64 = 0b1111;
    /// Offset of the FWB field.
    pub const FWB_SHIFT: u32 = 40;
    /// Mask for the FWB field.
    pub const FWB_MASK: u64 = 0b1111;
    /// Offset of the TTL field.
    pub const TTL_SHIFT: u32 = 48;
    /// Mask for the TTL field.
    pub const TTL_MASK: u64 = 0b1111;
    /// Offset of the BBM field.
    pub const BBM_SHIFT: u32 = 52;
    /// Mask for the BBM field.
    pub const BBM_MASK: u64 = 0b1111;
    /// Offset of the EVT field.
    pub const EVT_SHIFT: u32 = 56;
    /// Mask for the EVT field.
    pub const EVT_MASK: u64 = 0b1111;
    /// Offset of the E0PD field.
    pub const E0PD_SHIFT: u32 = 60;
    /// Mask for the E0PD field.
    pub const E0PD_MASK: u64 = 0b1111;

    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        (self.bits() >> Self::CNP_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `UAO` field.
    pub const fn uao(self) -> u8 {
        (self.bits() >> Self::UAO_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        (self.bits() >> Self::LSM_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `IESB` field.
    pub const fn iesb(self) -> u8 {
        (self.bits() >> Self::IESB_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `VARange` field.
    pub const fn varange(self) -> u8 {
        (self.bits() >> Self::VARANGE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        (self.bits() >> Self::CCIDX_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `NV` field.
    pub const fn nv(self) -> u8 {
        (self.bits() >> Self::NV_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ST` field.
    pub const fn st(self) -> u8 {
        (self.bits() >> Self::ST_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `AT` field.
    pub const fn at(self) -> u8 {
        (self.bits() >> Self::AT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `IDS` field.
    pub const fn ids(self) -> u8 {
        (self.bits() >> Self::IDS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `FWB` field.
    pub const fn fwb(self) -> u8 {
        (self.bits() >> Self::FWB_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `TTL` field.
    pub const fn ttl(self) -> u8 {
        (self.bits() >> Self::TTL_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BBM` field.
    pub const fn bbm(self) -> u8 {
        (self.bits() >> Self::BBM_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        (self.bits() >> Self::EVT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `E0PD` field.
    pub const fn e0pd(self) -> u8 {
        (self.bits() >> Self::E0PD_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr3El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr3El1 {
    /// Offset of the TCRX field.
    pub const TCRX_SHIFT: u32 = 0;
    /// Mask for the TCRX field.
    pub const TCRX_MASK: u64 = 0b1111;
    /// Offset of the SCTLRX field.
    pub const SCTLRX_SHIFT: u32 = 4;
    /// Mask for the SCTLRX field.
    pub const SCTLRX_MASK: u64 = 0b1111;
    /// Offset of the S1PIE field.
    pub const S1PIE_SHIFT: u32 = 8;
    /// Mask for the S1PIE field.
    pub const S1PIE_MASK: u64 = 0b1111;
    /// Offset of the S2PIE field.
    pub const S2PIE_SHIFT: u32 = 12;
    /// Mask for the S2PIE field.
    pub const S2PIE_MASK: u64 = 0b1111;
    /// Offset of the S1POE field.
    pub const S1POE_SHIFT: u32 = 16;
    /// Mask for the S1POE field.
    pub const S1POE_MASK: u64 = 0b1111;
    /// Offset of the S2POE field.
    pub const S2POE_SHIFT: u32 = 20;
    /// Mask for the S2POE field.
    pub const S2POE_MASK: u64 = 0b1111;
    /// Offset of the AIE field.
    pub const AIE_SHIFT: u32 = 24;
    /// Mask for the AIE field.
    pub const AIE_MASK: u64 = 0b1111;
    /// Offset of the MEC field.
    pub const MEC_SHIFT: u32 = 28;
    /// Mask for the MEC field.
    pub const MEC_MASK: u64 = 0b1111;
    /// Offset of the D128 field.
    pub const D128_SHIFT: u32 = 32;
    /// Mask for the D128 field.
    pub const D128_MASK: u64 = 0b1111;
    /// Offset of the D128_2 field.
    pub const D128_2_SHIFT: u32 = 36;
    /// Mask for the D128_2 field.
    pub const D128_2_MASK: u64 = 0b1111;
    /// Offset of the SNERR field.
    pub const SNERR_SHIFT: u32 = 40;
    /// Mask for the SNERR field.
    pub const SNERR_MASK: u64 = 0b1111;
    /// Offset of the ANERR field.
    pub const ANERR_SHIFT: u32 = 44;
    /// Mask for the ANERR field.
    pub const ANERR_MASK: u64 = 0b1111;
    /// Offset of the SDERR field.
    pub const SDERR_SHIFT: u32 = 52;
    /// Mask for the SDERR field.
    pub const SDERR_MASK: u64 = 0b1111;
    /// Offset of the ADERR field.
    pub const ADERR_SHIFT: u32 = 56;
    /// Mask for the ADERR field.
    pub const ADERR_MASK: u64 = 0b1111;
    /// Offset of the Spec_FPACC field.
    pub const SPEC_FPACC_SHIFT: u32 = 60;
    /// Mask for the Spec_FPACC field.
    pub const SPEC_FPACC_MASK: u64 = 0b1111;

    /// Returns the value of the `TCRX` field.
    pub const fn tcrx(self) -> u8 {
        (self.bits() >> Self::TCRX_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SCTLRX` field.
    pub const fn sctlrx(self) -> u8 {
        (self.bits() >> Self::SCTLRX_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `S1PIE` field.
    pub const fn s1pie(self) -> u8 {
        (self.bits() >> Self::S1PIE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `S2PIE` field.
    pub const fn s2pie(self) -> u8 {
        (self.bits() >> Self::S2PIE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `S1POE` field.
    pub const fn s1poe(self) -> u8 {
        (self.bits() >> Self::S1POE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `S2POE` field.
    pub const fn s2poe(self) -> u8 {
        (self.bits() >> Self::S2POE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `AIE` field.
    pub const fn aie(self) -> u8 {
        (self.bits() >> Self::AIE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `MEC` field.
    pub const fn mec(self) -> u8 {
        (self.bits() >> Self::MEC_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `D128` field.
    pub const fn d128(self) -> u8 {
        (self.bits() >> Self::D128_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `D128_2` field.
    pub const fn d128_2(self) -> u8 {
        (self.bits() >> Self::D128_2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SNERR` field.
    pub const fn snerr(self) -> u8 {
        (self.bits() >> Self::SNERR_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ANERR` field.
    pub const fn anerr(self) -> u8 {
        (self.bits() >> Self::ANERR_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SDERR` field.
    pub const fn sderr(self) -> u8 {
        (self.bits() >> Self::SDERR_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ADERR` field.
    pub const fn aderr(self) -> u8 {
        (self.bits() >> Self::ADERR_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `Spec_FPACC` field.
    pub const fn spec_fpacc(self) -> u8 {
        (self.bits() >> Self::SPEC_FPACC_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64PFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64pfr0El1 {
    /// Offset of the EL0 field.
    pub const EL0_SHIFT: u32 = 0;
    /// Mask for the EL0 field.
    pub const EL0_MASK: u64 = 0b1111;
    /// Offset of the EL1 field.
    pub const EL1_SHIFT: u32 = 4;
    /// Mask for the EL1 field.
    pub const EL1_MASK: u64 = 0b1111;
    /// Offset of the EL2 field.
    pub const EL2_SHIFT: u32 = 8;
    /// Mask for the EL2 field.
    pub const EL2_MASK: u64 = 0b1111;
    /// Offset of the EL3 field.
    pub const EL3_SHIFT: u32 = 12;
    /// Mask for the EL3 field.
    pub const EL3_MASK: u64 = 0b1111;
    /// Offset of the FP field.
    pub const FP_SHIFT: u32 = 16;
    /// Mask for the FP field.
    pub const FP_MASK: u64 = 0b1111;
    /// Offset of the AdvSIMD field.
    pub const ADVSIMD_SHIFT: u32 = 20;
    /// Mask for the AdvSIMD field.
    pub const ADVSIMD_MASK: u64 = 0b1111;
    /// Offset of the GIC field.
    pub const GIC_SHIFT: u32 = 24;
    /// Mask for the GIC field.
    pub const GIC_MASK: u64 = 0b1111;
    /// Offset of the RAS field.
    pub const RAS_SHIFT: u32 = 28;
    /// Mask for the RAS field.
    pub const RAS_MASK: u64 = 0b1111;
    /// Offset of the SVE field.
    pub const SVE_SHIFT: u32 = 32;
    /// Mask for the SVE field.
    pub const SVE_MASK: u64 = 0b1111;
    /// Offset of the SEL2 field.
    pub const SEL2_SHIFT: u32 = 36;
    /// Mask for the SEL2 field.
    pub const SEL2_MASK: u64 = 0b1111;
    /// Offset of the MPAM field.
    pub const MPAM_SHIFT: u32 = 40;
    /// Mask for the MPAM field.
    pub const MPAM_MASK: u64 = 0b1111;
    /// Offset of the AMU field.
    pub const AMU_SHIFT: u32 = 44;
    /// Mask for the AMU field.
    pub const AMU_MASK: u64 = 0b1111;
    /// Offset of the DIT field.
    pub const DIT_SHIFT: u32 = 48;
    /// Mask for the DIT field.
    pub const DIT_MASK: u64 = 0b1111;
    /// Offset of the RME field.
    pub const RME_SHIFT: u32 = 52;
    /// Mask for the RME field.
    pub const RME_MASK: u64 = 0b1111;
    /// Offset of the CSV2 field.
    pub const CSV2_SHIFT: u32 = 56;
    /// Mask for the CSV2 field.
    pub const CSV2_MASK: u64 = 0b1111;
    /// Offset of the CSV3 field.
    pub const CSV3_SHIFT: u32 = 60;
    /// Mask for the CSV3 field.
    pub const CSV3_MASK: u64 = 0b1111;

    /// Returns the value of the `EL0` field.
    pub const fn el0(self) -> u8 {
        (self.bits() >> Self::EL0_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `EL1` field.
    pub const fn el1(self) -> u8 {
        (self.bits() >> Self::EL1_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `EL2` field.
    pub const fn el2(self) -> u8 {
        (self.bits() >> Self::EL2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `EL3` field.
    pub const fn el3(self) -> u8 {
        (self.bits() >> Self::EL3_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `FP` field.
    pub const fn fp(self) -> u8 {
        (self.bits() >> Self::FP_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `AdvSIMD` field.
    pub const fn advsimd(self) -> u8 {
        (self.bits() >> Self::ADVSIMD_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        (self.bits() >> Self::GIC_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        (self.bits() >> Self::RAS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SVE` field.
    pub const fn sve(self) -> u8 {
        (self.bits() >> Self::SVE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SEL2` field.
    pub const fn sel2(self) -> u8 {
        (self.bits() >> Self::SEL2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `MPAM` field.
    pub const fn mpam(self) -> u8 {
        (self.bits() >> Self::MPAM_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        (self.bits() >> Self::AMU_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        (self.bits() >> Self::DIT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `RME` field.
    pub const fn rme(self) -> u8 {
        (self.bits() >> Self::RME_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        (self.bits() >> Self::CSV2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        (self.bits() >> Self::CSV3_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64PFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64pfr1El1 {
    /// Offset of the BT field.
    pub const BT_SHIFT: u32 = 0;
    /// Mask for the BT field.
    pub const BT_MASK: u64 = 0b1111;
    /// Offset of the SSBS field.
    pub const SSBS_SHIFT: u32 = 4;
    /// Mask for the SSBS field.
    pub const SSBS_MASK: u64 = 0b1111;
    /// Offset of the MTE field.
    pub const MTE_SHIFT: u32 = 8;
    /// Mask for the MTE field.
    pub const MTE_MASK: u64 = 0b1111;
    /// Offset of the RAS_frac field.
    pub const RAS_FRAC_SHIFT: u32 = 12;
    /// Mask for the RAS_frac field.
    pub const RAS_FRAC_MASK: u64 = 0b1111;
    /// Offset of the MPAM_frac field.
    pub const MPAM_FRAC_SHIFT: u32 = 16;
    /// Mask for the MPAM_frac field.
    pub const MPAM_FRAC_MASK: u64 = 0b1111;
    /// Offset of the SME field.
    pub const SME_SHIFT: u32 = 24;
    /// Mask for the SME field.
    pub const SME_MASK: u64 = 0b1111;
    /// Offset of the RNDR_trap field.
    pub const RNDR_TRAP_SHIFT: u32 = 28;
    /// Mask for the RNDR_trap field.
    pub const RNDR_TRAP_MASK: u64 = 0b1111;
    /// Offset of the CSV2_frac field.
    pub const CSV2_FRAC_SHIFT: u32 = 32;
    /// Mask for the CSV2_frac field.
    pub const CSV2_FRAC_MASK: u64 = 0b1111;
    /// Offset of the NMI field.
    pub const NMI_SHIFT: u32 = 36;
    /// Mask for the NMI field.
    pub const NMI_MASK: u64 = 0b1111;
    /// Offset of the MTE_frac field.
    pub const MTE_FRAC_SHIFT: u32 = 40;
    /// Mask for the MTE_frac field.
    pub const MTE_FRAC_MASK: u64 = 0b1111;
    /// Offset of the GCS field.
    pub const GCS_SHIFT: u32 = 44;
    /// Mask for the GCS field.
    pub const GCS_MASK: u64 = 0b1111;
    /// Offset of the THE field.
    pub const THE_SHIFT: u32 = 48;
    /// Mask for the THE field.
    pub const THE_MASK: u64 = 0b1111;
    /// Offset of the MTEX field.
    pub const MTEX_SHIFT: u32 = 52;
    /// Mask for the MTEX field.
    pub const MTEX_MASK: u64 = 0b1111;
    /// Offset of the DF2 field.
    pub const DF2_SHIFT: u32 = 56;
    /// Mask for the DF2 field.
    pub const DF2_MASK: u64 = 0b1111;
    /// Offset of the PFAR field.
    pub const PFAR_SHIFT: u32 = 60;
    /// Mask for the PFAR field.
    pub const PFAR_MASK: u64 = 0b1111;

    /// Returns the value of the `BT` field.
    pub const fn bt(self) -> u8 {
        (self.bits() >> Self::BT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        (self.bits() >> Self::SSBS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `MTE` field.
    pub const fn mte(self) -> u8 {
        (self.bits() >> Self::MTE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        (self.bits() >> Self::RAS_FRAC_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `MPAM_frac` field.
    pub const fn mpam_frac(self) -> u8 {
        (self.bits() >> Self::MPAM_FRAC_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SME` field.
    pub const fn sme(self) -> u8 {
        (self.bits() >> Self::SME_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `RNDR_trap` field.
    pub const fn rndr_trap(self) -> u8 {
        (self.bits() >> Self::RNDR_TRAP_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `CSV2_frac` field.
    pub const fn csv2_frac(self) -> u8 {
        (self.bits() >> Self::CSV2_FRAC_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `NMI` field.
    pub const fn nmi(self) -> u8 {
        (self.bits() >> Self::NMI_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `MTE_frac` field.
    pub const fn mte_frac(self) -> u8 {
        (self.bits() >> Self::MTE_FRAC_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `GCS` field.
    pub const fn gcs(self) -> u8 {
        (self.bits() >> Self::GCS_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `THE` field.
    pub const fn the(self) -> u8 {
        (self.bits() >> Self::THE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `MTEX` field.
    pub const fn mtex(self) -> u8 {
        (self.bits() >> Self::MTEX_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `DF2` field.
    pub const fn df2(self) -> u8 {
        (self.bits() >> Self::DF2_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PFAR` field.
    pub const fn pfar(self) -> u8 {
        (self.bits() >> Self::PFAR_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl IdAa64smfr0El1 {
    /// Offset of the SMOP4 field.
    pub const SMOP4_SHIFT: u32 = 0;
    /// Offset of the STMOP field.
    pub const STMOP_SHIFT: u32 = 16;
    /// Offset of the SFEXPA field.
    pub const SFEXPA_SHIFT: u32 = 23;
    /// Offset of the AES field.
    pub const AES_SHIFT: u32 = 24;
    /// Offset of the SBitPerm field.
    pub const SBITPERM_SHIFT: u32 = 25;
    /// Offset of the SF8DP2 field.
    pub const SF8DP2_SHIFT: u32 = 28;
    /// Offset of the SF8DP4 field.
    pub const SF8DP4_SHIFT: u32 = 29;
    /// Offset of the SF8FMA field.
    pub const SF8FMA_SHIFT: u32 = 30;
    /// Offset of the F32F32 field.
    pub const F32F32_SHIFT: u32 = 32;
    /// Offset of the BI32I32 field.
    pub const BI32I32_SHIFT: u32 = 33;
    /// Offset of the B16F32 field.
    pub const B16F32_SHIFT: u32 = 34;
    /// Offset of the F16F32 field.
    pub const F16F32_SHIFT: u32 = 35;
    /// Offset of the I8I32 field.
    pub const I8I32_SHIFT: u32 = 36;
    /// Mask for the I8I32 field.
    pub const I8I32_MASK: u64 = 0b1111;
    /// Offset of the F8F32 field.
    pub const F8F32_SHIFT: u32 = 40;
    /// Offset of the F8F16 field.
    pub const F8F16_SHIFT: u32 = 41;
    /// Offset of the F16F16 field.
    pub const F16F16_SHIFT: u32 = 42;
    /// Offset of the B16B16 field.
    pub const B16B16_SHIFT: u32 = 43;
    /// Offset of the I16I32 field.
    pub const I16I32_SHIFT: u32 = 44;
    /// Mask for the I16I32 field.
    pub const I16I32_MASK: u64 = 0b1111;
    /// Offset of the F64F64 field.
    pub const F64F64_SHIFT: u32 = 48;
    /// Offset of the I16I64 field.
    pub const I16I64_SHIFT: u32 = 52;
    /// Mask for the I16I64 field.
    pub const I16I64_MASK: u64 = 0b1111;
    /// Offset of the SMEver field.
    pub const SMEVER_SHIFT: u32 = 56;
    /// Mask for the SMEver field.
    pub const SMEVER_MASK: u64 = 0b1111;
    /// Offset of the LUTv2 field.
    pub const LUTV2_SHIFT: u32 = 60;
    /// Offset of the LUT6 field.
    pub const LUT6_SHIFT: u32 = 61;
    /// Offset of the FA64 field.
    pub const FA64_SHIFT: u32 = 63;

    /// Returns the value of the `I8I32` field.
    pub const fn i8i32(self) -> u8 {
        (self.bits() >> Self::I8I32_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `I16I32` field.
    pub const fn i16i32(self) -> u8 {
        (self.bits() >> Self::I16I32_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `I16I64` field.
    pub const fn i16i64(self) -> u8 {
        (self.bits() >> Self::I16I64_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SMEver` field.
    pub const fn smever(self) -> u8 {
        (self.bits() >> Self::SMEVER_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl IsrEl1 {
    /// Offset of the F field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the I field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the FS field.
    pub const FS_SHIFT: u32 = 9;
    /// Offset of the IS field.
    pub const IS_SHIFT: u32 = 10;
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MAIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MairEl1 {
    /// Offset of the Attr<n> field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the Attr<n> field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) as u8 & 0b11111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MAIR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MairEl2 {
    /// Offset of the Attr<n> field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the Attr<n> field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) as u8 & 0b11111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `MAIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl MairEl3 {
    /// Offset of the Attr<n> field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the Attr<n> field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl MdccintEl1 {
    /// Offset of the TX field.
    pub const TX_SHIFT: u32 = 29;
    /// Offset of the RX field.
    pub const RX_SHIFT: u32 = 30;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl MdcrEl2 {
    /// Offset of the HPMN field.
    pub const HPMN_SHIFT: u32 = 0;
    /// Mask for the HPMN field.
    pub const HPMN_MASK: u64 = 0b11111;
    /// Offset of the TPMCR field.
    pub const TPMCR_SHIFT: u32 = 5;
    /// Offset of the TPM field.
    pub const TPM_SHIFT: u32 = 6;
    /// Offset of the HPME field.
    pub const HPME_SHIFT: u32 = 7;
    /// Offset of the TDE field.
    pub const TDE_SHIFT: u32 = 8;
    /// Offset of the TDA field.
    pub const TDA_SHIFT: u32 = 9;
    /// Offset of the TDOSA field.
    pub const TDOSA_SHIFT: u32 = 10;
    /// Offset of the TDRA field.
    pub const TDRA_SHIFT: u32 = 11;
    /// Offset of the E2PB field.
    pub const E2PB_SHIFT: u32 = 12;
    /// Mask for the E2PB field.
    pub const E2PB_MASK: u64 = 0b11;
    /// Offset of the TPMS field.
    pub const TPMS_SHIFT: u32 = 14;
    /// Offset of the EnSPM field.
    pub const ENSPM_SHIFT: u32 = 15;
    /// Offset of the HPMD field.
    pub const HPMD_SHIFT: u32 = 17;
    /// Offset of the TTRF field.
    pub const TTRF_SHIFT: u32 = 19;
    /// Offset of the HCCD field.
    pub const HCCD_SHIFT: u32 = 23;
    /// Offset of the E2TB field.
    pub const E2TB_SHIFT: u32 = 24;
    /// Mask for the E2TB field.
    pub const E2TB_MASK: u64 = 0b11;
    /// Offset of the HLP field.
    pub const HLP_SHIFT: u32 = 26;
    /// Offset of the TDCC field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the MTPME field.
    pub const MTPME_SHIFT: u32 = 28;
    /// Offset of the HPMFZO field.
    pub const HPMFZO_SHIFT: u32 = 29;
    /// Offset of the PMSSE field.
    pub const PMSSE_SHIFT: u32 = 30;
    /// Mask for the PMSSE field.
    pub const PMSSE_MASK: u64 = 0b11;
    /// Offset of the HPMFZS field.
    pub const HPMFZS_SHIFT: u32 = 36;
    /// Offset of the PMEE field.
    pub const PMEE_SHIFT: u32 = 40;
    /// Mask for the PMEE field.
    pub const PMEE_MASK: u64 = 0b11;
    /// Offset of the EBWE field.
    pub const EBWE_SHIFT: u32 = 43;
    /// Offset of the EnSTEPOP field.
    pub const ENSTEPOP_SHIFT: u32 = 50;

    /// Returns the value of the `HPMN` field.
    pub const fn hpmn(self) -> u8 {
        (self.bits() >> Self::HPMN_SHIFT) as u8 & 0b11111
    }

    /// Returns the value of the `E2PB` field.
    pub const fn e2pb(self) -> u8 {
        (self.bits() >> Self::E2PB_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `E2TB` field.
    pub const fn e2tb(self) -> u8 {
        (self.bits() >> Self::E2TB_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        (self.bits() >> Self::PMSSE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        (self.bits() >> Self::PMEE_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl MdcrEl3 {
    /// Offset of the RLTE field.
    pub const RLTE_SHIFT: u32 = 0;
    /// Offset of the EPMADE field.
    pub const EPMADE_SHIFT: u32 = 2;
    /// Offset of the ETADE field.
    pub const ETADE_SHIFT: u32 = 3;
    /// Offset of the EDADE field.
    pub const EDADE_SHIFT: u32 = 4;
    /// Offset of the TPM field.
    pub const TPM_SHIFT: u32 = 6;
    /// Offset of the EnPM2 field.
    pub const ENPM2_SHIFT: u32 = 7;
    /// Offset of the TDA field.
    pub const TDA_SHIFT: u32 = 9;
    /// Offset of the TDOSA field.
    pub const TDOSA_SHIFT: u32 = 10;
    /// Offset of the NSPBE field.
    pub const NSPBE_SHIFT: u32 = 11;
    /// Offset of the NSPB field.
    pub const NSPB_SHIFT: u32 = 12;
    /// Mask for the NSPB field.
    pub const NSPB_MASK: u64 = 0b11;
    /// Offset of the SPD32 field.
    pub const SPD32_SHIFT: u32 = 14;
    /// Mask for the SPD32 field.
    pub const SPD32_MASK: u64 = 0b11;
    /// Offset of the SDD field.
    pub const SDD_SHIFT: u32 = 16;
    /// Offset of the SPME field.
    pub const SPME_SHIFT: u32 = 17;
    /// Offset of the STE field.
    pub const STE_SHIFT: u32 = 18;
    /// Offset of the TTRF field.
    pub const TTRF_SHIFT: u32 = 19;
    /// Offset of the EDAD field.
    pub const EDAD_SHIFT: u32 = 20;
    /// Offset of the EPMAD field.
    pub const EPMAD_SHIFT: u32 = 21;
    /// Offset of the ETAD field.
    pub const ETAD_SHIFT: u32 = 22;
    /// Offset of the SCCD field.
    pub const SCCD_SHIFT: u32 = 23;
    /// Offset of the NSTB field.
    pub const NSTB_SHIFT: u32 = 24;
    /// Mask for the NSTB field.
    pub const NSTB_MASK: u64 = 0b11;
    /// Offset of the NSTBE field.
    pub const NSTBE_SHIFT: u32 = 26;
    /// Offset of the TDCC field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the MTPME field.
    pub const MTPME_SHIFT: u32 = 28;
    /// Offset of the PMSSE field.
    pub const PMSSE_SHIFT: u32 = 30;
    /// Mask for the PMSSE field.
    pub const PMSSE_MASK: u64 = 0b11;
    /// Offset of the SBRBE field.
    pub const SBRBE_SHIFT: u32 = 32;
    /// Mask for the SBRBE field.
    pub const SBRBE_MASK: u64 = 0b11;
    /// Offset of the MCCD field.
    pub const MCCD_SHIFT: u32 = 34;
    /// Offset of the MPMX field.
    pub const MPMX_SHIFT: u32 = 35;
    /// Offset of the EnPMSN field.
    pub const ENPMSN_SHIFT: u32 = 36;
    /// Offset of the E3BREW field.
    pub const E3BREW_SHIFT: u32 = 37;
    /// Offset of the E3BREC field.
    pub const E3BREC_SHIFT: u32 = 38;
    /// Offset of the EnTB2 field.
    pub const ENTB2_SHIFT: u32 = 39;
    /// Offset of the PMEE field.
    pub const PMEE_SHIFT: u32 = 40;
    /// Mask for the PMEE field.
    pub const PMEE_MASK: u64 = 0b11;
    /// Offset of the EnPMS3 field.
    pub const ENPMS3_SHIFT: u32 = 42;
    /// Offset of the EBWE field.
    pub const EBWE_SHIFT: u32 = 43;
    /// Offset of the EnPMSS field.
    pub const ENPMSS_SHIFT: u32 = 44;
    /// Offset of the EPMSSAD field.
    pub const EPMSSAD_SHIFT: u32 = 45;
    /// Mask for the EPMSSAD field.
    pub const EPMSSAD_MASK: u64 = 0b11;
    /// Offset of the EnITE field.
    pub const ENITE_SHIFT: u32 = 47;
    /// Offset of the ETBAD field.
    pub const ETBAD_SHIFT: u32 = 48;
    /// Mask for the ETBAD field.
    pub const ETBAD_MASK: u64 = 0b11;
    /// Offset of the EnSTEPOP field.
    pub const ENSTEPOP_SHIFT: u32 = 50;
    /// Offset of the PMSEE field.
    pub const PMSEE_SHIFT: u32 = 51;
    /// Mask for the PMSEE field.
    pub const PMSEE_MASK: u64 = 0b11;
    /// Offset of the TRBEE field.
    pub const TRBEE_SHIFT: u32 = 53;
    /// Mask for the TRBEE field.
    pub const TRBEE_MASK: u64 = 0b11;
    /// Offset of the EnPMS4 field.
    pub const ENPMS4_SHIFT: u32 = 55;

    /// Returns the value of the `NSPB` field.
    pub const fn nspb(self) -> u8 {
        (self.bits() >> Self::NSPB_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SPD32` field.
    pub const fn spd32(self) -> u8 {
        (self.bits() >> Self::SPD32_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `NSTB` field.
    pub const fn nstb(self) -> u8 {
        (self.bits() >> Self::NSTB_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        (self.bits() >> Self::PMSSE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SBRBE` field.
    pub const fn sbrbe(self) -> u8 {
        (self.bits() >> Self::SBRBE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        (self.bits() >> Self::PMEE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `EPMSSAD` field.
    pub const fn epmssad(self) -> u8 {
        (self.bits() >> Self::EPMSSAD_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ETBAD` field.
    pub const fn etbad(self) -> u8 {
        (self.bits() >> Self::ETBAD_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PMSEE` field.
    pub const fn pmsee(self) -> u8 {
        (self.bits() >> Self::PMSEE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TRBEE` field.
    pub const fn trbee(self) -> u8 {
        (self.bits() >> Self::TRBEE_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl MdscrEl1 {
    /// Offset of the SS field.
    pub const SS_SHIFT: u32 = 0;
    /// Offset of the ERR field.
    pub const ERR_SHIFT: u32 = 6;
    /// Offset of the TDCC field.
    pub const TDCC_SHIFT: u32 = 12;
    /// Offset of the KDE field.
    pub const KDE_SHIFT: u32 = 13;
    /// Offset of the HDE field.
    pub const HDE_SHIFT: u32 = 14;
    /// Offset of the MDE field.
    pub const MDE_SHIFT: u32 = 15;
    /// Offset of the SC2 field.
    pub const SC2_SHIFT: u32 = 19;
    /// Offset of the TDA field.
    pub const TDA_SHIFT: u32 = 21;
    /// Offset of the INTdis field.
    pub const INTDIS_SHIFT: u32 = 22;
    /// Mask for the INTdis field.
    pub const INTDIS_MASK: u64 = 0b11;
    /// Offset of the TXU field.
    pub const TXU_SHIFT: u32 = 26;
    /// Offset of the RXO field.
    pub const RXO_SHIFT: u32 = 27;
    /// Offset of the TXfull field.
    pub const TXFULL_SHIFT: u32 = 29;
    /// Offset of the RXfull field.
    pub const RXFULL_SHIFT: u32 = 30;
    /// Offset of the TFO field.
    pub const TFO_SHIFT: u32 = 31;
    /// Offset of the EMBWE field.
    pub const EMBWE_SHIFT: u32 = 32;
    /// Offset of the TTA field.
    pub const TTA_SHIFT: u32 = 33;
    /// Offset of the EnSPM field.
    pub const ENSPM_SHIFT: u32 = 34;
    /// Offset of the EHBWE field.
    pub const EHBWE_SHIFT: u32 = 35;
    /// Offset of the EnSTEPOP field.
    pub const ENSTEPOP_SHIFT: u32 = 50;

    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        (self.bits() >> Self::INTDIS_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MidrEl1 {
    /// Offset of the Revision field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the Revision field.
    pub const REVISION_MASK: u64 = 0b1111;
    /// Offset of the PartNum field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the PartNum field.
    pub const PARTNUM_MASK: u64 = 0b111111111111;
    /// Offset of the Architecture field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the Architecture field.
    pub const ARCHITECTURE_MASK: u64 = 0b1111;
    /// Offset of the Variant field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the Variant field.
    pub const VARIANT_MASK: u64 = 0b1111;
    /// Offset of the Implementer field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the Implementer field.
    pub const IMPLEMENTER_MASK: u64 = 0b11111111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> Self::REVISION_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        (self.bits() >> Self::PARTNUM_SHIFT) as u16 & 0b111111111111
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        (self.bits() >> Self::ARCHITECTURE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        (self.bits() >> Self::VARIANT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        (self.bits() >> Self::IMPLEMENTER_SHIFT) as u8 & 0b11111111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Mpam2El2 {
    /// Offset of the PARTID field.
    pub const PARTID_SHIFT: u32 = 0;
    /// Mask for the PARTID field.
    pub const PARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the PARTID_I field.
    pub const PARTID_I_SHIFT: u32 = 0;
    /// Mask for the PARTID_I field.
    pub const PARTID_I_MASK: u64 = 0b1111111111111111;
    /// Offset of the PARTID_D field.
    pub const PARTID_D_SHIFT: u32 = 16;
    /// Mask for the PARTID_D field.
    pub const PARTID_D_MASK: u64 = 0b1111111111111111;
    /// Offset of the altPARTID field.
    pub const ALTPARTID_SHIFT: u32 = 16;
    /// Mask for the altPARTID field.
    pub const ALTPARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the PMG field.
    pub const PMG_SHIFT: u32 = 32;
    /// Mask for the PMG field.
    pub const PMG_MASK: u64 = 0b1111111111111111;
    /// Offset of the PMG_I field.
    pub const PMG_I_SHIFT: u32 = 32;
    /// Mask for the PMG_I field.
    pub const PMG_I_MASK: u64 = 0b11111111;
    /// Offset of the PMG_D field.
    pub const PMG_D_SHIFT: u32 = 40;
    /// Mask for the PMG_D field.
    pub const PMG_D_MASK: u64 = 0b11111111;
    /// Offset of the TRAPMPAM1EL1 field.
    pub const TRAPMPAM1EL1_SHIFT: u32 = 48;
    /// Offset of the altPMG field.
    pub const ALTPMG_SHIFT: u32 = 48;
    /// Mask for the altPMG field.
    pub const ALTPMG_MASK: u64 = 0b1111111111111111;
    /// Offset of the TRAPMPAM0EL1 field.
    pub const TRAPMPAM0EL1_SHIFT: u32 = 49;
    /// Offset of the EnMPAMSM field.
    pub const ENMPAMSM_SHIFT: u32 = 50;
    /// Offset of the ALTSP_FRCD field.
    pub const ALTSP_FRCD_SHIFT: u32 = 54;
    /// Offset of the ALTSP_EL2 field.
    pub const ALTSP_EL2_SHIFT: u32 = 55;
    /// Offset of the ALTSP_HFC field.
    pub const ALTSP_HFC_SHIFT: u32 = 56;
    /// Offset of the TIDR field.
    pub const TIDR_SHIFT: u32 = 58;
    /// Offset of the MPAMEN field.
    pub const MPAMEN_SHIFT: u32 = 63;

    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> Self::PARTID_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        (self.bits() >> Self::PARTID_I_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        (self.bits() >> Self::PARTID_D_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        (self.bits() >> Self::ALTPARTID_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        (self.bits() >> Self::PMG_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        (self.bits() >> Self::PMG_I_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        (self.bits() >> Self::PMG_D_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        (self.bits() >> Self::ALTPMG_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl Mpam3El3 {
    /// Offset of the PARTID field.
    pub const PARTID_SHIFT: u32 = 0;
    /// Mask for the PARTID field.
    pub const PARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the PARTID_I field.
    pub const PARTID_I_SHIFT: u32 = 0;
    /// Mask for the PARTID_I field.
    pub const PARTID_I_MASK: u64 = 0b1111111111111111;
    /// Offset of the PARTID_D field.
    pub const PARTID_D_SHIFT: u32 = 16;
    /// Mask for the PARTID_D field.
    pub const PARTID_D_MASK: u64 = 0b1111111111111111;
    /// Offset of the altPARTID field.
    pub const ALTPARTID_SHIFT: u32 = 16;
    /// Mask for the altPARTID field.
    pub const ALTPARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the PMG field.
    pub const PMG_SHIFT: u32 = 32;
    /// Mask for the PMG field.
    pub const PMG_MASK: u64 = 0b1111111111111111;
    /// Offset of the PMG_I field.
    pub const PMG_I_SHIFT: u32 = 32;
    /// Mask for the PMG_I field.
    pub const PMG_I_MASK: u64 = 0b11111111;
    /// Offset of the PMG_D field.
    pub const PMG_D_SHIFT: u32 = 40;
    /// Mask for the PMG_D field.
    pub const PMG_D_MASK: u64 = 0b11111111;
    /// Offset of the altPMG field.
    pub const ALTPMG_SHIFT: u32 = 48;
    /// Mask for the altPMG field.
    pub const ALTPMG_MASK: u64 = 0b1111111111111111;
    /// Offset of the RT_ALTSP_NS field.
    pub const RT_ALTSP_NS_SHIFT: u32 = 52;
    /// Offset of the ALTSP_EL3 field.
    pub const ALTSP_EL3_SHIFT: u32 = 55;
    /// Offset of the ALTSP_HFC field.
    pub const ALTSP_HFC_SHIFT: u32 = 56;
    /// Offset of the ALTSP_HEN field.
    pub const ALTSP_HEN_SHIFT: u32 = 57;
    /// Offset of the FORCE_NS field.
    pub const FORCE_NS_SHIFT: u32 = 60;
    /// Offset of the SDEFLT field.
    pub const SDEFLT_SHIFT: u32 = 61;
    /// Offset of the TRAPLOWER field.
    pub const TRAPLOWER_SHIFT: u32 = 62;
    /// Offset of the MPAMEN field.
    pub const MPAMEN_SHIFT: u32 = 63;

    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> Self::PARTID_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        (self.bits() >> Self::PARTID_I_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        (self.bits() >> Self::PARTID_D_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        (self.bits() >> Self::ALTPARTID_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        (self.bits() >> Self::PMG_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        (self.bits() >> Self::PMG_I_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        (self.bits() >> Self::PMG_D_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        (self.bits() >> Self::ALTPMG_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl MpamhcrEl2 {
    /// Offset of the EL0_VPMEN field.
    pub const EL0_VPMEN_SHIFT: u32 = 0;
    /// Offset of the EL1_VPMEN field.
    pub const EL1_VPMEN_SHIFT: u32 = 1;
    /// Offset of the VPMEN field.
    pub const VPMEN_SHIFT: u32 = 2;
    /// Offset of the VMMEN field.
    pub const VMMEN_SHIFT: u32 = 3;
    /// Offset of the SMVPMEN field.
    pub const SMVPMEN_SHIFT: u32 = 4;
    /// Offset of the SMVMMEN field.
    pub const SMVMMEN_SHIFT: u32 = 5;
    /// Offset of the GSTAPP_PLK field.
    pub const GSTAPP_PLK_SHIFT: u32 = 8;
    /// Offset of the TRAP_MPAMIDR_EL1 field.
    pub const TRAP_MPAMIDR_EL1_SHIFT: u32 = 31;
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl MpamidrEl1 {
    /// Offset of the PARTID_MAX field.
    pub const PARTID_MAX_SHIFT: u32 = 0;
    /// Mask for the PARTID_MAX field.
    pub const PARTID_MAX_MASK: u64 = 0b1111111111111111;
    /// Offset of the HAS_HCR field.
    pub const HAS_HCR_SHIFT: u32 = 17;
    /// Offset of the VPMR_MAX field.
    pub const VPMR_MAX_SHIFT: u32 = 18;
    /// Mask for the VPMR_MAX field.
    pub const VPMR_MAX_MASK: u64 = 0b111;
    /// Offset of the HAS_ALT_ID field.
    pub const HAS_ALT_ID_SHIFT: u32 = 21;
    /// Offset of the HAS_INSTR_ALT_ID field.
    pub const HAS_INSTR_ALT_ID_SHIFT: u32 = 22;
    /// Offset of the HAS_BW_CTRL field.
    pub const HAS_BW_CTRL_SHIFT: u32 = 56;
    /// Offset of the HAS_ALTSP field.
    pub const HAS_ALTSP_SHIFT: u32 = 57;
    /// Offset of the HAS_TIDR field.
    pub const HAS_TIDR_SHIFT: u32 = 58;
    /// Offset of the SP4 field.
    pub const SP4_SHIFT: u32 = 59;
    /// Offset of the HAS_FORCE_NS field.
    pub const HAS_FORCE_NS_SHIFT: u32 = 60;
    /// Offset of the HAS_SDEFLT field.
    pub const HAS_SDEFLT_SHIFT: u32 = 61;

    /// Returns the value of the `PARTID_MAX` field.
    pub const fn partid_max(self) -> u16 {
        (self.bits() >> Self::PARTID_MAX_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `VPMR_MAX` field.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn vpmr_max(self) -> u8 {
        (self.bits() >> Self::VPMR_MAX_SHIFT) as u8 & 0b111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm0El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm0El2 {
    /// Offset of the PhyPARTID0 field.
    pub const PHYPARTID0_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID0 field.
    pub const PHYPARTID0_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID1 field.
    pub const PHYPARTID1_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID1 field.
    pub const PHYPARTID1_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID2 field.
    pub const PHYPARTID2_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID2 field.
    pub const PHYPARTID2_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID3 field.
    pub const PHYPARTID3_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID3 field.
    pub const PHYPARTID3_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID0` field.
    pub const fn phypartid0(self) -> u16 {
        (self.bits() >> Self::PHYPARTID0_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID1` field.
    pub const fn phypartid1(self) -> u16 {
        (self.bits() >> Self::PHYPARTID1_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID2` field.
    pub const fn phypartid2(self) -> u16 {
        (self.bits() >> Self::PHYPARTID2_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID3` field.
    pub const fn phypartid3(self) -> u16 {
        (self.bits() >> Self::PHYPARTID3_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm1El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm1El2 {
    /// Offset of the PhyPARTID4 field.
    pub const PHYPARTID4_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID4 field.
    pub const PHYPARTID4_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID5 field.
    pub const PHYPARTID5_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID5 field.
    pub const PHYPARTID5_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID6 field.
    pub const PHYPARTID6_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID6 field.
    pub const PHYPARTID6_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID7 field.
    pub const PHYPARTID7_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID7 field.
    pub const PHYPARTID7_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID4` field.
    pub const fn phypartid4(self) -> u16 {
        (self.bits() >> Self::PHYPARTID4_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID5` field.
    pub const fn phypartid5(self) -> u16 {
        (self.bits() >> Self::PHYPARTID5_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID6` field.
    pub const fn phypartid6(self) -> u16 {
        (self.bits() >> Self::PHYPARTID6_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID7` field.
    pub const fn phypartid7(self) -> u16 {
        (self.bits() >> Self::PHYPARTID7_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm2El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm2El2 {
    /// Offset of the PhyPARTID8 field.
    pub const PHYPARTID8_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID8 field.
    pub const PHYPARTID8_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID9 field.
    pub const PHYPARTID9_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID9 field.
    pub const PHYPARTID9_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID10 field.
    pub const PHYPARTID10_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID10 field.
    pub const PHYPARTID10_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID11 field.
    pub const PHYPARTID11_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID11 field.
    pub const PHYPARTID11_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID8` field.
    pub const fn phypartid8(self) -> u16 {
        (self.bits() >> Self::PHYPARTID8_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID9` field.
    pub const fn phypartid9(self) -> u16 {
        (self.bits() >> Self::PHYPARTID9_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID10` field.
    pub const fn phypartid10(self) -> u16 {
        (self.bits() >> Self::PHYPARTID10_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID11` field.
    pub const fn phypartid11(self) -> u16 {
        (self.bits() >> Self::PHYPARTID11_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM3_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm3El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm3El2 {
    /// Offset of the PhyPARTID12 field.
    pub const PHYPARTID12_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID12 field.
    pub const PHYPARTID12_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID13 field.
    pub const PHYPARTID13_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID13 field.
    pub const PHYPARTID13_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID14 field.
    pub const PHYPARTID14_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID14 field.
    pub const PHYPARTID14_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID15 field.
    pub const PHYPARTID15_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID15 field.
    pub const PHYPARTID15_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID12` field.
    pub const fn phypartid12(self) -> u16 {
        (self.bits() >> Self::PHYPARTID12_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID13` field.
    pub const fn phypartid13(self) -> u16 {
        (self.bits() >> Self::PHYPARTID13_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID14` field.
    pub const fn phypartid14(self) -> u16 {
        (self.bits() >> Self::PHYPARTID14_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID15` field.
    pub const fn phypartid15(self) -> u16 {
        (self.bits() >> Self::PHYPARTID15_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM4_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm4El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm4El2 {
    /// Offset of the PhyPARTID16 field.
    pub const PHYPARTID16_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID16 field.
    pub const PHYPARTID16_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID17 field.
    pub const PHYPARTID17_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID17 field.
    pub const PHYPARTID17_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID18 field.
    pub const PHYPARTID18_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID18 field.
    pub const PHYPARTID18_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID19 field.
    pub const PHYPARTID19_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID19 field.
    pub const PHYPARTID19_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID16` field.
    pub const fn phypartid16(self) -> u16 {
        (self.bits() >> Self::PHYPARTID16_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID17` field.
    pub const fn phypartid17(self) -> u16 {
        (self.bits() >> Self::PHYPARTID17_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID18` field.
    pub const fn phypartid18(self) -> u16 {
        (self.bits() >> Self::PHYPARTID18_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID19` field.
    pub const fn phypartid19(self) -> u16 {
        (self.bits() >> Self::PHYPARTID19_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM5_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm5El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm5El2 {
    /// Offset of the PhyPARTID20 field.
    pub const PHYPARTID20_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID20 field.
    pub const PHYPARTID20_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID21 field.
    pub const PHYPARTID21_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID21 field.
    pub const PHYPARTID21_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID22 field.
    pub const PHYPARTID22_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID22 field.
    pub const PHYPARTID22_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID23 field.
    pub const PHYPARTID23_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID23 field.
    pub const PHYPARTID23_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID20` field.
    pub const fn phypartid20(self) -> u16 {
        (self.bits() >> Self::PHYPARTID20_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID21` field.
    pub const fn phypartid21(self) -> u16 {
        (self.bits() >> Self::PHYPARTID21_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID22` field.
    pub const fn phypartid22(self) -> u16 {
        (self.bits() >> Self::PHYPARTID22_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID23` field.
    pub const fn phypartid23(self) -> u16 {
        (self.bits() >> Self::PHYPARTID23_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM6_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm6El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm6El2 {
    /// Offset of the PhyPARTID24 field.
    pub const PHYPARTID24_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID24 field.
    pub const PHYPARTID24_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID25 field.
    pub const PHYPARTID25_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID25 field.
    pub const PHYPARTID25_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID26 field.
    pub const PHYPARTID26_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID26 field.
    pub const PHYPARTID26_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID27 field.
    pub const PHYPARTID27_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID27 field.
    pub const PHYPARTID27_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID24` field.
    pub const fn phypartid24(self) -> u16 {
        (self.bits() >> Self::PHYPARTID24_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID25` field.
    pub const fn phypartid25(self) -> u16 {
        (self.bits() >> Self::PHYPARTID25_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID26` field.
    pub const fn phypartid26(self) -> u16 {
        (self.bits() >> Self::PHYPARTID26_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID27` field.
    pub const fn phypartid27(self) -> u16 {
        (self.bits() >> Self::PHYPARTID27_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM7_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm7El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm7El2 {
    /// Offset of the PhyPARTID28 field.
    pub const PHYPARTID28_SHIFT: u32 = 0;
    /// Mask for the PhyPARTID28 field.
    pub const PHYPARTID28_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID29 field.
    pub const PHYPARTID29_SHIFT: u32 = 16;
    /// Mask for the PhyPARTID29 field.
    pub const PHYPARTID29_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID30 field.
    pub const PHYPARTID30_SHIFT: u32 = 32;
    /// Mask for the PhyPARTID30 field.
    pub const PHYPARTID30_MASK: u64 = 0b1111111111111111;
    /// Offset of the PhyPARTID31 field.
    pub const PHYPARTID31_SHIFT: u32 = 48;
    /// Mask for the PhyPARTID31 field.
    pub const PHYPARTID31_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID28` field.
    pub const fn phypartid28(self) -> u16 {
        (self.bits() >> Self::PHYPARTID28_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID29` field.
    pub const fn phypartid29(self) -> u16 {
        (self.bits() >> Self::PHYPARTID29_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID30` field.
    pub const fn phypartid30(self) -> u16 {
        (self.bits() >> Self::PHYPARTID30_SHIFT) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID31` field.
    pub const fn phypartid31(self) -> u16 {
        (self.bits() >> Self::PHYPARTID31_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl MpamvpmvEl2 {
    /// Offset of the VPM_V<m> field.
    pub const VPM_V_SHIFT: u32 = 0;
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl MpidrEl1 {
    /// Offset of the Aff0 field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the Aff0 field.
    pub const AFF0_MASK: u64 = 0b11111111;
    /// Offset of the Aff1 field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the Aff1 field.
    pub const AFF1_MASK: u64 = 0b11111111;
    /// Offset of the Aff2 field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the Aff2 field.
    pub const AFF2_MASK: u64 = 0b11111111;
    /// Offset of the MT field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the U field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the Aff3 field.
    pub const AFF3_SHIFT: u32 = 32;
    /// Mask for the Aff3 field.
    pub const AFF3_MASK: u64 = 0b11111111;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        (self.bits() >> Self::AFF0_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> Self::AFF1_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> Self::AFF2_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> Self::AFF3_SHIFT) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl ParEl1 {
    /// Offset of the F field.
    pub const F_SHIFT: u32 = 0;
    /// Offset of the FST field.
    pub const FST_SHIFT: u32 = 1;
    /// Mask for the FST field.
    pub const FST_MASK: u64 = 0b111111;
    /// Offset of the SH field.
    pub const SH_SHIFT: u32 = 7;
    /// Mask for the SH field.
    pub const SH_MASK: u64 = 0b11;
    /// Offset of the PTW field.
    pub const PTW_SHIFT: u32 = 8;
    /// Offset of the NS field.
    pub const NS_SHIFT: u32 = 9;
    /// Offset of the S field.
    pub const S_SHIFT: u32 = 9;
    /// Offset of the NSE field.
    pub const NSE_SHIFT: u32 = 11;
    /// Offset of the AssuredOnly field.
    pub const ASSUREDONLY_SHIFT: u32 = 12;
    /// Offset of the PA[47:12] field.
    pub const PA_47_12_SHIFT: u32 = 12;
    /// Mask for the PA[47:12] field.
    pub const PA_47_12_MASK: u64 = 0b111111111111111111111111111111111111;
    /// Offset of the TopLevel field.
    pub const TOPLEVEL_SHIFT: u32 = 13;
    /// Offset of the Overlay field.
    pub const OVERLAY_SHIFT: u32 = 14;
    /// Offset of the DirtyBit field.
    pub const DIRTYBIT_SHIFT: u32 = 15;
    /// Offset of the PA[51:48] field.
    pub const PA_51_48_SHIFT: u32 = 48;
    /// Mask for the PA[51:48] field.
    pub const PA_51_48_MASK: u64 = 0b1111;
    /// Offset of the ATTR field.
    pub const ATTR_SHIFT: u32 = 56;
    /// Mask for the ATTR field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the `FST` field.
    pub const fn fst(self) -> u8 {
        (self.bits() >> Self::FST_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        (self.bits() >> Self::SH_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PA[47:12]` field.
    pub const fn pa_47_12(self) -> u64 {
        (self.bits() >> Self::PA_47_12_SHIFT) as u64 & 0b111111111111111111111111111111111111
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        (self.bits() >> Self::PA_51_48_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `ATTR` field.
    pub const fn attr(self) -> u8 {
        (self.bits() >> Self::ATTR_SHIFT) as u8 & 0b11111111
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
    /// Offset of the E field.
    pub const E_SHIFT: u32 = 0;
    /// Offset of the P field.
    pub const P_SHIFT: u32 = 1;
    /// Offset of the C field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the D field.
    pub const D_SHIFT: u32 = 3;
    /// Offset of the X field.
    pub const X_SHIFT: u32 = 4;
    /// Offset of the DP field.
    pub const DP_SHIFT: u32 = 5;
    /// Offset of the LC field.
    pub const LC_SHIFT: u32 = 6;
    /// Offset of the LP field.
    pub const LP_SHIFT: u32 = 7;
    /// Offset of the FZO field.
    pub const FZO_SHIFT: u32 = 9;
    /// Offset of the N field.
    pub const N_SHIFT: u32 = 11;
    /// Mask for the N field.
    pub const N_MASK: u64 = 0b11111;
    /// Offset of the IDCODE field.
    pub const IDCODE_SHIFT: u32 = 16;
    /// Mask for the IDCODE field.
    pub const IDCODE_MASK: u64 = 0b11111111;
    /// Offset of the IMP field.
    pub const IMP_SHIFT: u32 = 24;
    /// Mask for the IMP field.
    pub const IMP_MASK: u64 = 0b11111111;
    /// Offset of the FZS field.
    pub const FZS_SHIFT: u32 = 32;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        (self.bits() >> Self::N_SHIFT) as u8 & 0b11111
    }

    /// Returns the value of the `IDCODE` field.
    pub const fn idcode(self) -> u8 {
        (self.bits() >> Self::IDCODE_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `IMP` field.
    pub const fn imp(self) -> u8 {
        (self.bits() >> Self::IMP_SHIFT) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `RGSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RgsrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl RgsrEl1 {
    /// Offset of the TAG field.
    pub const TAG_SHIFT: u32 = 0;
    /// Mask for the TAG field.
    pub const TAG_MASK: u64 = 0b1111;
    /// Offset of the SEED field.
    pub const SEED_SHIFT: u32 = 8;
    /// Mask for the SEED field.
    pub const SEED_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `TAG` field.
    pub const fn tag(self) -> u8 {
        (self.bits() >> Self::TAG_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `SEED` field.
    pub const fn seed(self) -> u16 {
        (self.bits() >> Self::SEED_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl ScrEl3 {
    /// Offset of the NS field.
    pub const NS_SHIFT: u32 = 0;
    /// Offset of the IRQ field.
    pub const IRQ_SHIFT: u32 = 1;
    /// Offset of the FIQ field.
    pub const FIQ_SHIFT: u32 = 2;
    /// Offset of the EA field.
    pub const EA_SHIFT: u32 = 3;
    /// Offset of the SMD field.
    pub const SMD_SHIFT: u32 = 7;
    /// Offset of the HCE field.
    pub const HCE_SHIFT: u32 = 8;
    /// Offset of the SIF field.
    pub const SIF_SHIFT: u32 = 9;
    /// Offset of the RW field.
    pub const RW_SHIFT: u32 = 10;
    /// Offset of the ST field.
    pub const ST_SHIFT: u32 = 11;
    /// Offset of the TWI field.
    pub const TWI_SHIFT: u32 = 12;
    /// Offset of the TWE field.
    pub const TWE_SHIFT: u32 = 13;
    /// Offset of the TLOR field.
    pub const TLOR_SHIFT: u32 = 14;
    /// Offset of the TERR field.
    pub const TERR_SHIFT: u32 = 15;
    /// Offset of the APK field.
    pub const APK_SHIFT: u32 = 16;
    /// Offset of the API field.
    pub const API_SHIFT: u32 = 17;
    /// Offset of the EEL2 field.
    pub const EEL2_SHIFT: u32 = 18;
    /// Offset of the EASE field.
    pub const EASE_SHIFT: u32 = 19;
    /// Offset of the NMEA field.
    pub const NMEA_SHIFT: u32 = 20;
    /// Offset of the FIEN field.
    pub const FIEN_SHIFT: u32 = 21;
    /// Offset of the TID3 field.
    pub const TID3_SHIFT: u32 = 22;
    /// Offset of the TID5 field.
    pub const TID5_SHIFT: u32 = 23;
    /// Offset of the POE2En field.
    pub const POE2EN_SHIFT: u32 = 24;
    /// Offset of the EnSCXT field.
    pub const ENSCXT_SHIFT: u32 = 25;
    /// Offset of the ATA field.
    pub const ATA_SHIFT: u32 = 26;
    /// Offset of the FGTEn field.
    pub const FGTEN_SHIFT: u32 = 27;
    /// Offset of the ECVEn field.
    pub const ECVEN_SHIFT: u32 = 28;
    /// Offset of the TWEDEn field.
    pub const TWEDEN_SHIFT: u32 = 29;
    /// Offset of the TWEDEL field.
    pub const TWEDEL_SHIFT: u32 = 30;
    /// Mask for the TWEDEL field.
    pub const TWEDEL_MASK: u64 = 0b1111;
    /// Offset of the AMVOFFEN field.
    pub const AMVOFFEN_SHIFT: u32 = 35;
    /// Offset of the EnAS0 field.
    pub const ENAS0_SHIFT: u32 = 36;
    /// Offset of the ADEn field.
    pub const ADEN_SHIFT: u32 = 37;
    /// Offset of the HXEn field.
    pub const HXEN_SHIFT: u32 = 38;
    /// Offset of the GCSEn field.
    pub const GCSEN_SHIFT: u32 = 39;
    /// Offset of the TRNDR field.
    pub const TRNDR_SHIFT: u32 = 40;
    /// Offset of the EnTP2 field.
    pub const ENTP2_SHIFT: u32 = 41;
    /// Offset of the RCWMASKEn field.
    pub const RCWMASKEN_SHIFT: u32 = 42;
    /// Offset of the TCR2En field.
    pub const TCR2EN_SHIFT: u32 = 43;
    /// Offset of the SCTLR2En field.
    pub const SCTLR2EN_SHIFT: u32 = 44;
    /// Offset of the PIEn field.
    pub const PIEN_SHIFT: u32 = 45;
    /// Offset of the AIEn field.
    pub const AIEN_SHIFT: u32 = 46;
    /// Offset of the D128En field.
    pub const D128EN_SHIFT: u32 = 47;
    /// Offset of the GPF field.
    pub const GPF_SHIFT: u32 = 48;
    /// Offset of the MECEn field.
    pub const MECEN_SHIFT: u32 = 49;
    /// Offset of the EnFPM field.
    pub const ENFPM_SHIFT: u32 = 50;
    /// Offset of the TMEA field.
    pub const TMEA_SHIFT: u32 = 51;
    /// Offset of the TWERR field.
    pub const TWERR_SHIFT: u32 = 52;
    /// Offset of the PFAREn field.
    pub const PFAREN_SHIFT: u32 = 53;
    /// Offset of the SRMASKEn field.
    pub const SRMASKEN_SHIFT: u32 = 54;
    /// Offset of the EnIDCP128 field.
    pub const ENIDCP128_SHIFT: u32 = 55;
    /// Offset of the VTLBIDEn field.
    pub const VTLBIDEN_SHIFT: u32 = 56;
    /// Offset of the DSE field.
    pub const DSE_SHIFT: u32 = 57;
    /// Offset of the EnDSE field.
    pub const ENDSE_SHIFT: u32 = 58;
    /// Offset of the FGTEn2 field.
    pub const FGTEN2_SHIFT: u32 = 59;
    /// Offset of the HDBSSEn field.
    pub const HDBSSEN_SHIFT: u32 = 60;
    /// Offset of the HACDBSEn field.
    pub const HACDBSEN_SHIFT: u32 = 61;
    /// Offset of the NSE field.
    pub const NSE_SHIFT: u32 = 62;
    /// Offset of the TPLIMEn field.
    pub const TPLIMEN_SHIFT: u32 = 63;

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> Self::TWEDEL_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl SctlrEl1 {
    /// Offset of the M field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the C field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the SA field.
    pub const SA_SHIFT: u32 = 3;
    /// Offset of the SA0 field.
    pub const SA0_SHIFT: u32 = 4;
    /// Offset of the CP15BEN field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the nAA field.
    pub const NAA_SHIFT: u32 = 6;
    /// Offset of the ITD field.
    pub const ITD_SHIFT: u32 = 7;
    /// Offset of the SED field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the UMA field.
    pub const UMA_SHIFT: u32 = 9;
    /// Offset of the EnRCTX field.
    pub const ENRCTX_SHIFT: u32 = 10;
    /// Offset of the EOS field.
    pub const EOS_SHIFT: u32 = 11;
    /// Offset of the I field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the EnDB field.
    pub const ENDB_SHIFT: u32 = 13;
    /// Offset of the DZE field.
    pub const DZE_SHIFT: u32 = 14;
    /// Offset of the UCT field.
    pub const UCT_SHIFT: u32 = 15;
    /// Offset of the nTWI field.
    pub const NTWI_SHIFT: u32 = 16;
    /// Offset of the nTWE field.
    pub const NTWE_SHIFT: u32 = 18;
    /// Offset of the WXN field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the TSCXT field.
    pub const TSCXT_SHIFT: u32 = 20;
    /// Offset of the IESB field.
    pub const IESB_SHIFT: u32 = 21;
    /// Offset of the EIS field.
    pub const EIS_SHIFT: u32 = 22;
    /// Offset of the SPAN field.
    pub const SPAN_SHIFT: u32 = 23;
    /// Offset of the UCI field.
    pub const UCI_SHIFT: u32 = 26;
    /// Offset of the EnDA field.
    pub const ENDA_SHIFT: u32 = 27;
    /// Offset of the nTLSMD field.
    pub const NTLSMD_SHIFT: u32 = 28;
    /// Offset of the LSMAOE field.
    pub const LSMAOE_SHIFT: u32 = 29;
    /// Offset of the EnIB field.
    pub const ENIB_SHIFT: u32 = 30;
    /// Offset of the EnIA field.
    pub const ENIA_SHIFT: u32 = 31;
    /// Offset of the CMOW field.
    pub const CMOW_SHIFT: u32 = 32;
    /// Offset of the MSCEn field.
    pub const MSCEN_SHIFT: u32 = 33;
    /// Offset of the EnFPM field.
    pub const ENFPM_SHIFT: u32 = 34;
    /// Offset of the BT0 field.
    pub const BT0_SHIFT: u32 = 35;
    /// Offset of the BT1 field.
    pub const BT1_SHIFT: u32 = 36;
    /// Offset of the ITFSB field.
    pub const ITFSB_SHIFT: u32 = 37;
    /// Offset of the TCF0 field.
    pub const TCF0_SHIFT: u32 = 38;
    /// Mask for the TCF0 field.
    pub const TCF0_MASK: u64 = 0b11;
    /// Offset of the TCF field.
    pub const TCF_SHIFT: u32 = 40;
    /// Mask for the TCF field.
    pub const TCF_MASK: u64 = 0b11;
    /// Offset of the ATA0 field.
    pub const ATA0_SHIFT: u32 = 42;
    /// Offset of the ATA field.
    pub const ATA_SHIFT: u32 = 43;
    /// Offset of the DSSBS field.
    pub const DSSBS_SHIFT: u32 = 44;
    /// Offset of the TWEDEn field.
    pub const TWEDEN_SHIFT: u32 = 45;
    /// Offset of the TWEDEL field.
    pub const TWEDEL_SHIFT: u32 = 46;
    /// Mask for the TWEDEL field.
    pub const TWEDEL_MASK: u64 = 0b1111;
    /// Offset of the EnASR field.
    pub const ENASR_SHIFT: u32 = 54;
    /// Offset of the EnAS0 field.
    pub const ENAS0_SHIFT: u32 = 55;
    /// Offset of the EnALS field.
    pub const ENALS_SHIFT: u32 = 56;
    /// Offset of the EPAN field.
    pub const EPAN_SHIFT: u32 = 57;
    /// Offset of the TCSO0 field.
    pub const TCSO0_SHIFT: u32 = 58;
    /// Offset of the TCSO field.
    pub const TCSO_SHIFT: u32 = 59;
    /// Offset of the EnTP2 field.
    pub const ENTP2_SHIFT: u32 = 60;
    /// Offset of the NMI field.
    pub const NMI_SHIFT: u32 = 61;
    /// Offset of the SPINTMASK field.
    pub const SPINTMASK_SHIFT: u32 = 62;
    /// Offset of the TIDCP field.
    pub const TIDCP_SHIFT: u32 = 63;

    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        (self.bits() >> Self::TCF0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        (self.bits() >> Self::TCF_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> Self::TWEDEL_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl SctlrEl2 {
    /// Offset of the M field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the C field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the SA field.
    pub const SA_SHIFT: u32 = 3;
    /// Offset of the SA0 field.
    pub const SA0_SHIFT: u32 = 4;
    /// Offset of the CP15BEN field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the nAA field.
    pub const NAA_SHIFT: u32 = 6;
    /// Offset of the SED field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the UMA field.
    pub const UMA_SHIFT: u32 = 9;
    /// Offset of the EnRCTX field.
    pub const ENRCTX_SHIFT: u32 = 10;
    /// Offset of the EOS field.
    pub const EOS_SHIFT: u32 = 11;
    /// Offset of the I field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the EnDB field.
    pub const ENDB_SHIFT: u32 = 13;
    /// Offset of the DZE field.
    pub const DZE_SHIFT: u32 = 14;
    /// Offset of the UCT field.
    pub const UCT_SHIFT: u32 = 15;
    /// Offset of the nTWI field.
    pub const NTWI_SHIFT: u32 = 16;
    /// Offset of the nTWE field.
    pub const NTWE_SHIFT: u32 = 18;
    /// Offset of the WXN field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the IESB field.
    pub const IESB_SHIFT: u32 = 21;
    /// Offset of the EIS field.
    pub const EIS_SHIFT: u32 = 22;
    /// Offset of the SPAN field.
    pub const SPAN_SHIFT: u32 = 23;
    /// Offset of the UCI field.
    pub const UCI_SHIFT: u32 = 26;
    /// Offset of the EnDA field.
    pub const ENDA_SHIFT: u32 = 27;
    /// Offset of the nTLSMD field.
    pub const NTLSMD_SHIFT: u32 = 28;
    /// Offset of the LSMAOE field.
    pub const LSMAOE_SHIFT: u32 = 29;
    /// Offset of the EnIB field.
    pub const ENIB_SHIFT: u32 = 30;
    /// Offset of the EnIA field.
    pub const ENIA_SHIFT: u32 = 31;
    /// Offset of the CMOW field.
    pub const CMOW_SHIFT: u32 = 32;
    /// Offset of the MSCEn field.
    pub const MSCEN_SHIFT: u32 = 33;
    /// Offset of the EnFPM field.
    pub const ENFPM_SHIFT: u32 = 34;
    /// Offset of the BT0 field.
    pub const BT0_SHIFT: u32 = 35;
    /// Offset of the BT field.
    pub const BT_SHIFT: u32 = 36;
    /// Offset of the ITFSB field.
    pub const ITFSB_SHIFT: u32 = 37;
    /// Offset of the TCF0 field.
    pub const TCF0_SHIFT: u32 = 38;
    /// Mask for the TCF0 field.
    pub const TCF0_MASK: u64 = 0b11;
    /// Offset of the TCF field.
    pub const TCF_SHIFT: u32 = 40;
    /// Mask for the TCF field.
    pub const TCF_MASK: u64 = 0b11;
    /// Offset of the ATA0 field.
    pub const ATA0_SHIFT: u32 = 42;
    /// Offset of the ATA field.
    pub const ATA_SHIFT: u32 = 43;
    /// Offset of the DSSBS field.
    pub const DSSBS_SHIFT: u32 = 44;
    /// Offset of the TWEDEn field.
    pub const TWEDEN_SHIFT: u32 = 45;
    /// Offset of the TWEDEL field.
    pub const TWEDEL_SHIFT: u32 = 46;
    /// Mask for the TWEDEL field.
    pub const TWEDEL_MASK: u64 = 0b1111;
    /// Offset of the EnASR field.
    pub const ENASR_SHIFT: u32 = 54;
    /// Offset of the EnAS0 field.
    pub const ENAS0_SHIFT: u32 = 55;
    /// Offset of the EnALS field.
    pub const ENALS_SHIFT: u32 = 56;
    /// Offset of the EPAN field.
    pub const EPAN_SHIFT: u32 = 57;
    /// Offset of the TCSO0 field.
    pub const TCSO0_SHIFT: u32 = 58;
    /// Offset of the TCSO field.
    pub const TCSO_SHIFT: u32 = 59;
    /// Offset of the EnTP2 field.
    pub const ENTP2_SHIFT: u32 = 60;
    /// Offset of the NMI field.
    pub const NMI_SHIFT: u32 = 61;
    /// Offset of the SPINTMASK field.
    pub const SPINTMASK_SHIFT: u32 = 62;
    /// Offset of the TIDCP field.
    pub const TIDCP_SHIFT: u32 = 63;

    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        (self.bits() >> Self::TCF0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        (self.bits() >> Self::TCF_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> Self::TWEDEL_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl SctlrEl3 {
    /// Offset of the M field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the C field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the SA field.
    pub const SA_SHIFT: u32 = 3;
    /// Offset of the nAA field.
    pub const NAA_SHIFT: u32 = 6;
    /// Offset of the EOS field.
    pub const EOS_SHIFT: u32 = 11;
    /// Offset of the I field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the EnDB field.
    pub const ENDB_SHIFT: u32 = 13;
    /// Offset of the WXN field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the IESB field.
    pub const IESB_SHIFT: u32 = 21;
    /// Offset of the EIS field.
    pub const EIS_SHIFT: u32 = 22;
    /// Offset of the EnDA field.
    pub const ENDA_SHIFT: u32 = 27;
    /// Offset of the EnIB field.
    pub const ENIB_SHIFT: u32 = 30;
    /// Offset of the EnIA field.
    pub const ENIA_SHIFT: u32 = 31;
    /// Offset of the BT field.
    pub const BT_SHIFT: u32 = 36;
    /// Offset of the ITFSB field.
    pub const ITFSB_SHIFT: u32 = 37;
    /// Offset of the TCF field.
    pub const TCF_SHIFT: u32 = 40;
    /// Mask for the TCF field.
    pub const TCF_MASK: u64 = 0b11;
    /// Offset of the ATA field.
    pub const ATA_SHIFT: u32 = 43;
    /// Offset of the DSSBS field.
    pub const DSSBS_SHIFT: u32 = 44;
    /// Offset of the TCSO field.
    pub const TCSO_SHIFT: u32 = 59;
    /// Offset of the NMI field.
    pub const NMI_SHIFT: u32 = 61;
    /// Offset of the SPINTMASK field.
    pub const SPINTMASK_SHIFT: u32 = 62;

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        (self.bits() >> Self::TCF_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl SmcrEl3 {
    /// Offset of the LEN field.
    pub const LEN_SHIFT: u32 = 0;
    /// Mask for the LEN field.
    pub const LEN_MASK: u64 = 0b1111;
    /// Offset of the EZT0 field.
    pub const EZT0_SHIFT: u32 = 30;
    /// Offset of the FA64 field.
    pub const FA64_SHIFT: u32 = 31;

    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> Self::LEN_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl SpsrEl1 {
    /// Offset of the M[3:0] field.
    pub const M_3_0_SHIFT: u32 = 0;
    /// Mask for the M[3:0] field.
    pub const M_3_0_MASK: u64 = 0b1111;
    /// Offset of the M[4] field.
    pub const M_4_SHIFT: u32 = 4;
    /// Offset of the T field.
    pub const T_SHIFT: u32 = 5;
    /// Offset of the F field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the I field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the D field.
    pub const D_SHIFT: u32 = 9;
    /// Offset of the E field.
    pub const E_SHIFT: u32 = 9;
    /// Offset of the BTYPE field.
    pub const BTYPE_SHIFT: u32 = 10;
    /// Mask for the BTYPE field.
    pub const BTYPE_MASK: u64 = 0b11;
    /// Offset of the ALLINT field.
    pub const ALLINT_SHIFT: u32 = 13;
    /// Offset of the BTYPE2 field.
    pub const BTYPE2_SHIFT: u32 = 14;
    /// Offset of the GE field.
    pub const GE_SHIFT: u32 = 16;
    /// Mask for the GE field.
    pub const GE_MASK: u64 = 0b1111;
    /// Offset of the IL field.
    pub const IL_SHIFT: u32 = 20;
    /// Offset of the SS field.
    pub const SS_SHIFT: u32 = 21;
    /// Offset of the PAN field.
    pub const PAN_SHIFT: u32 = 22;
    /// Offset of the UAO field.
    pub const UAO_SHIFT: u32 = 23;
    /// Offset of the DIT field.
    pub const DIT_SHIFT: u32 = 24;
    /// Offset of the TCO field.
    pub const TCO_SHIFT: u32 = 25;
    /// Offset of the Q field.
    pub const Q_SHIFT: u32 = 27;
    /// Offset of the V field.
    pub const V_SHIFT: u32 = 28;
    /// Offset of the C field.
    pub const C_SHIFT: u32 = 29;
    /// Offset of the Z field.
    pub const Z_SHIFT: u32 = 30;
    /// Offset of the N field.
    pub const N_SHIFT: u32 = 31;
    /// Offset of the PM field.
    pub const PM_SHIFT: u32 = 32;
    /// Offset of the PPEND field.
    pub const PPEND_SHIFT: u32 = 33;
    /// Offset of the EXLOCK field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the PACM field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the UINJ field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        (self.bits() >> Self::M_3_0_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        (self.bits() >> Self::BTYPE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> Self::GE_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl SpsrEl2 {
    /// Offset of the M[3:0] field.
    pub const M_3_0_SHIFT: u32 = 0;
    /// Mask for the M[3:0] field.
    pub const M_3_0_MASK: u64 = 0b1111;
    /// Offset of the M[4] field.
    pub const M_4_SHIFT: u32 = 4;
    /// Offset of the T field.
    pub const T_SHIFT: u32 = 5;
    /// Offset of the F field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the I field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the D field.
    pub const D_SHIFT: u32 = 9;
    /// Offset of the E field.
    pub const E_SHIFT: u32 = 9;
    /// Offset of the BTYPE field.
    pub const BTYPE_SHIFT: u32 = 10;
    /// Mask for the BTYPE field.
    pub const BTYPE_MASK: u64 = 0b11;
    /// Offset of the ALLINT field.
    pub const ALLINT_SHIFT: u32 = 13;
    /// Offset of the BTYPE2 field.
    pub const BTYPE2_SHIFT: u32 = 14;
    /// Offset of the GE field.
    pub const GE_SHIFT: u32 = 16;
    /// Mask for the GE field.
    pub const GE_MASK: u64 = 0b1111;
    /// Offset of the IL field.
    pub const IL_SHIFT: u32 = 20;
    /// Offset of the SS field.
    pub const SS_SHIFT: u32 = 21;
    /// Offset of the PAN field.
    pub const PAN_SHIFT: u32 = 22;
    /// Offset of the UAO field.
    pub const UAO_SHIFT: u32 = 23;
    /// Offset of the DIT field.
    pub const DIT_SHIFT: u32 = 24;
    /// Offset of the TCO field.
    pub const TCO_SHIFT: u32 = 25;
    /// Offset of the Q field.
    pub const Q_SHIFT: u32 = 27;
    /// Offset of the V field.
    pub const V_SHIFT: u32 = 28;
    /// Offset of the C field.
    pub const C_SHIFT: u32 = 29;
    /// Offset of the Z field.
    pub const Z_SHIFT: u32 = 30;
    /// Offset of the N field.
    pub const N_SHIFT: u32 = 31;
    /// Offset of the PM field.
    pub const PM_SHIFT: u32 = 32;
    /// Offset of the PPEND field.
    pub const PPEND_SHIFT: u32 = 33;
    /// Offset of the EXLOCK field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the PACM field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the UINJ field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        (self.bits() >> Self::M_3_0_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        (self.bits() >> Self::BTYPE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> Self::GE_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl SpsrEl3 {
    /// Offset of the M[3:0] field.
    pub const M_3_0_SHIFT: u32 = 0;
    /// Mask for the M[3:0] field.
    pub const M_3_0_MASK: u64 = 0b1111;
    /// Offset of the M[4] field.
    pub const M_4_SHIFT: u32 = 4;
    /// Offset of the T field.
    pub const T_SHIFT: u32 = 5;
    /// Offset of the F field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the I field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the D field.
    pub const D_SHIFT: u32 = 9;
    /// Offset of the E field.
    pub const E_SHIFT: u32 = 9;
    /// Offset of the BTYPE field.
    pub const BTYPE_SHIFT: u32 = 10;
    /// Mask for the BTYPE field.
    pub const BTYPE_MASK: u64 = 0b11;
    /// Offset of the ALLINT field.
    pub const ALLINT_SHIFT: u32 = 13;
    /// Offset of the BTYPE2 field.
    pub const BTYPE2_SHIFT: u32 = 14;
    /// Offset of the GE field.
    pub const GE_SHIFT: u32 = 16;
    /// Mask for the GE field.
    pub const GE_MASK: u64 = 0b1111;
    /// Offset of the IL field.
    pub const IL_SHIFT: u32 = 20;
    /// Offset of the SS field.
    pub const SS_SHIFT: u32 = 21;
    /// Offset of the PAN field.
    pub const PAN_SHIFT: u32 = 22;
    /// Offset of the UAO field.
    pub const UAO_SHIFT: u32 = 23;
    /// Offset of the DIT field.
    pub const DIT_SHIFT: u32 = 24;
    /// Offset of the TCO field.
    pub const TCO_SHIFT: u32 = 25;
    /// Offset of the Q field.
    pub const Q_SHIFT: u32 = 27;
    /// Offset of the V field.
    pub const V_SHIFT: u32 = 28;
    /// Offset of the C field.
    pub const C_SHIFT: u32 = 29;
    /// Offset of the Z field.
    pub const Z_SHIFT: u32 = 30;
    /// Offset of the N field.
    pub const N_SHIFT: u32 = 31;
    /// Offset of the PM field.
    pub const PM_SHIFT: u32 = 32;
    /// Offset of the PPEND field.
    pub const PPEND_SHIFT: u32 = 33;
    /// Offset of the EXLOCK field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the PACM field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the UINJ field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        (self.bits() >> Self::M_3_0_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        (self.bits() >> Self::BTYPE_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> Self::GE_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SP_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl SpEl1 {
    /// Offset of the StackPointer field.
    pub const STACKPOINTER_SHIFT: u32 = 0;
    /// Mask for the StackPointer field.
    pub const STACKPOINTER_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        (self.bits() >> Self::STACKPOINTER_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl SpEl2 {
    /// Offset of the StackPointer field.
    pub const STACKPOINTER_SHIFT: u32 = 0;
    /// Mask for the StackPointer field.
    pub const STACKPOINTER_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        (self.bits() >> Self::STACKPOINTER_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl Tcr2El1 {
    /// Offset of the PnCH field.
    pub const PNCH_SHIFT: u32 = 0;
    /// Offset of the PIE field.
    pub const PIE_SHIFT: u32 = 1;
    /// Offset of the E0POE field.
    pub const E0POE_SHIFT: u32 = 2;
    /// Offset of the POE field.
    pub const POE_SHIFT: u32 = 3;
    /// Offset of the AIE field.
    pub const AIE_SHIFT: u32 = 4;
    /// Offset of the D128 field.
    pub const D128_SHIFT: u32 = 5;
    /// Offset of the PTTWI field.
    pub const PTTWI_SHIFT: u32 = 10;
    /// Offset of the HAFT field.
    pub const HAFT_SHIFT: u32 = 11;
    /// Offset of the DisCH0 field.
    pub const DISCH0_SHIFT: u32 = 14;
    /// Offset of the DisCH1 field.
    pub const DISCH1_SHIFT: u32 = 15;
    /// Offset of the A2 field.
    pub const A2_SHIFT: u32 = 16;
    /// Offset of the FNG0 field.
    pub const FNG0_SHIFT: u32 = 17;
    /// Offset of the FNG1 field.
    pub const FNG1_SHIFT: u32 = 18;
    /// Offset of the POE2F field.
    pub const POE2F_SHIFT: u32 = 19;
    /// Offset of the FNGNA0 field.
    pub const FNGNA0_SHIFT: u32 = 20;
    /// Offset of the FNGNA1 field.
    pub const FNGNA1_SHIFT: u32 = 21;
    /// Offset of the POIW field.
    pub const POIW_SHIFT: u32 = 22;
    /// Mask for the POIW field.
    pub const POIW_MASK: u64 = 0b111;
    /// Offset of the VTB0 field.
    pub const VTB0_SHIFT: u32 = 25;
    /// Mask for the VTB0 field.
    pub const VTB0_MASK: u64 = 0b11111;
    /// Offset of the VTB1 field.
    pub const VTB1_SHIFT: u32 = 30;
    /// Mask for the VTB1 field.
    pub const VTB1_MASK: u64 = 0b11111;
    /// Offset of the TVAD0 field.
    pub const TVAD0_SHIFT: u32 = 35;
    /// Offset of the TVAD1 field.
    pub const TVAD1_SHIFT: u32 = 36;

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        (self.bits() >> Self::POIW_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        (self.bits() >> Self::VTB0_SHIFT) as u8 & 0b11111
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        (self.bits() >> Self::VTB1_SHIFT) as u8 & 0b11111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Tcr2El2 {
    /// Offset of the PnCH field.
    pub const PNCH_SHIFT: u32 = 0;
    /// Offset of the PIE field.
    pub const PIE_SHIFT: u32 = 1;
    /// Offset of the E0POE field.
    pub const E0POE_SHIFT: u32 = 2;
    /// Offset of the POE field.
    pub const POE_SHIFT: u32 = 3;
    /// Offset of the AIE field.
    pub const AIE_SHIFT: u32 = 4;
    /// Offset of the D128 field.
    pub const D128_SHIFT: u32 = 5;
    /// Offset of the PTTWI field.
    pub const PTTWI_SHIFT: u32 = 10;
    /// Offset of the HAFT field.
    pub const HAFT_SHIFT: u32 = 11;
    /// Offset of the AMEC0 field.
    pub const AMEC0_SHIFT: u32 = 12;
    /// Offset of the AMEC1 field.
    pub const AMEC1_SHIFT: u32 = 13;
    /// Offset of the DisCH0 field.
    pub const DISCH0_SHIFT: u32 = 14;
    /// Offset of the DisCH1 field.
    pub const DISCH1_SHIFT: u32 = 15;
    /// Offset of the A2 field.
    pub const A2_SHIFT: u32 = 16;
    /// Offset of the FNG0 field.
    pub const FNG0_SHIFT: u32 = 17;
    /// Offset of the FNG1 field.
    pub const FNG1_SHIFT: u32 = 18;
    /// Offset of the POE2F field.
    pub const POE2F_SHIFT: u32 = 19;
    /// Offset of the POIW field.
    pub const POIW_SHIFT: u32 = 22;
    /// Mask for the POIW field.
    pub const POIW_MASK: u64 = 0b111;
    /// Offset of the VTB0 field.
    pub const VTB0_SHIFT: u32 = 25;
    /// Mask for the VTB0 field.
    pub const VTB0_MASK: u64 = 0b11111;
    /// Offset of the VTB1 field.
    pub const VTB1_SHIFT: u32 = 30;
    /// Mask for the VTB1 field.
    pub const VTB1_MASK: u64 = 0b11111;
    /// Offset of the TVAD0 field.
    pub const TVAD0_SHIFT: u32 = 35;
    /// Offset of the TVAD1 field.
    pub const TVAD1_SHIFT: u32 = 36;

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        (self.bits() >> Self::POIW_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        (self.bits() >> Self::VTB0_SHIFT) as u8 & 0b11111
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        (self.bits() >> Self::VTB1_SHIFT) as u8 & 0b11111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl TcrEl1 {
    /// Offset of the T0SZ field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the T0SZ field.
    pub const T0SZ_MASK: u64 = 0b111111;
    /// Offset of the EPD0 field.
    pub const EPD0_SHIFT: u32 = 7;
    /// Offset of the IRGN0 field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the IRGN0 field.
    pub const IRGN0_MASK: u64 = 0b11;
    /// Offset of the ORGN0 field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the ORGN0 field.
    pub const ORGN0_MASK: u64 = 0b11;
    /// Offset of the SH0 field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the SH0 field.
    pub const SH0_MASK: u64 = 0b11;
    /// Offset of the TG0 field.
    pub const TG0_SHIFT: u32 = 14;
    /// Mask for the TG0 field.
    pub const TG0_MASK: u64 = 0b11;
    /// Offset of the T1SZ field.
    pub const T1SZ_SHIFT: u32 = 16;
    /// Mask for the T1SZ field.
    pub const T1SZ_MASK: u64 = 0b111111;
    /// Offset of the A1 field.
    pub const A1_SHIFT: u32 = 22;
    /// Offset of the EPD1 field.
    pub const EPD1_SHIFT: u32 = 23;
    /// Offset of the IRGN1 field.
    pub const IRGN1_SHIFT: u32 = 24;
    /// Mask for the IRGN1 field.
    pub const IRGN1_MASK: u64 = 0b11;
    /// Offset of the ORGN1 field.
    pub const ORGN1_SHIFT: u32 = 26;
    /// Mask for the ORGN1 field.
    pub const ORGN1_MASK: u64 = 0b11;
    /// Offset of the SH1 field.
    pub const SH1_SHIFT: u32 = 28;
    /// Mask for the SH1 field.
    pub const SH1_MASK: u64 = 0b11;
    /// Offset of the TG1 field.
    pub const TG1_SHIFT: u32 = 30;
    /// Mask for the TG1 field.
    pub const TG1_MASK: u64 = 0b11;
    /// Offset of the IPS field.
    pub const IPS_SHIFT: u32 = 32;
    /// Mask for the IPS field.
    pub const IPS_MASK: u64 = 0b111;
    /// Offset of the AS field.
    pub const AS_SHIFT: u32 = 36;
    /// Offset of the TBI0 field.
    pub const TBI0_SHIFT: u32 = 37;
    /// Offset of the TBI1 field.
    pub const TBI1_SHIFT: u32 = 38;
    /// Offset of the HA field.
    pub const HA_SHIFT: u32 = 39;
    /// Offset of the HD field.
    pub const HD_SHIFT: u32 = 40;
    /// Offset of the HPD0 field.
    pub const HPD0_SHIFT: u32 = 41;
    /// Offset of the HPD1 field.
    pub const HPD1_SHIFT: u32 = 42;
    /// Offset of the HWU059 field.
    pub const HWU059_SHIFT: u32 = 43;
    /// Offset of the HWU060 field.
    pub const HWU060_SHIFT: u32 = 44;
    /// Offset of the HWU061 field.
    pub const HWU061_SHIFT: u32 = 45;
    /// Offset of the HWU062 field.
    pub const HWU062_SHIFT: u32 = 46;
    /// Offset of the HWU159 field.
    pub const HWU159_SHIFT: u32 = 47;
    /// Offset of the HWU160 field.
    pub const HWU160_SHIFT: u32 = 48;
    /// Offset of the HWU161 field.
    pub const HWU161_SHIFT: u32 = 49;
    /// Offset of the HWU162 field.
    pub const HWU162_SHIFT: u32 = 50;
    /// Offset of the TBID0 field.
    pub const TBID0_SHIFT: u32 = 51;
    /// Offset of the TBID1 field.
    pub const TBID1_SHIFT: u32 = 52;
    /// Offset of the NFD0 field.
    pub const NFD0_SHIFT: u32 = 53;
    /// Offset of the NFD1 field.
    pub const NFD1_SHIFT: u32 = 54;
    /// Offset of the E0PD0 field.
    pub const E0PD0_SHIFT: u32 = 55;
    /// Offset of the E0PD1 field.
    pub const E0PD1_SHIFT: u32 = 56;
    /// Offset of the TCMA0 field.
    pub const TCMA0_SHIFT: u32 = 57;
    /// Offset of the TCMA1 field.
    pub const TCMA1_SHIFT: u32 = 58;
    /// Offset of the DS field.
    pub const DS_SHIFT: u32 = 59;
    /// Offset of the MTX0 field.
    pub const MTX0_SHIFT: u32 = 60;
    /// Offset of the MTX1 field.
    pub const MTX1_SHIFT: u32 = 61;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> Self::T0SZ_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> Self::IRGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> Self::ORGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> Self::SH0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> Self::TG0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        (self.bits() >> Self::T1SZ_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        (self.bits() >> Self::IRGN1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        (self.bits() >> Self::ORGN1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        (self.bits() >> Self::SH1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        (self.bits() >> Self::TG1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        (self.bits() >> Self::IPS_SHIFT) as u8 & 0b111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl TcrEl2 {
    /// Offset of the T0SZ field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the T0SZ field.
    pub const T0SZ_MASK: u64 = 0b111111;
    /// Offset of the EPD0 field.
    pub const EPD0_SHIFT: u32 = 7;
    /// Offset of the IRGN0 field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the IRGN0 field.
    pub const IRGN0_MASK: u64 = 0b11;
    /// Offset of the ORGN0 field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the ORGN0 field.
    pub const ORGN0_MASK: u64 = 0b11;
    /// Offset of the SH0 field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the SH0 field.
    pub const SH0_MASK: u64 = 0b11;
    /// Offset of the TG0 field.
    pub const TG0_SHIFT: u32 = 14;
    /// Mask for the TG0 field.
    pub const TG0_MASK: u64 = 0b11;
    /// Offset of the PS field.
    pub const PS_SHIFT: u32 = 16;
    /// Mask for the PS field.
    pub const PS_MASK: u64 = 0b111;
    /// Offset of the T1SZ field.
    pub const T1SZ_SHIFT: u32 = 16;
    /// Mask for the T1SZ field.
    pub const T1SZ_MASK: u64 = 0b111111;
    /// Offset of the TBI field.
    pub const TBI_SHIFT: u32 = 20;
    /// Offset of the A1 field.
    pub const A1_SHIFT: u32 = 22;
    /// Offset of the EPD1 field.
    pub const EPD1_SHIFT: u32 = 23;
    /// Offset of the HPD field.
    pub const HPD_SHIFT: u32 = 24;
    /// Offset of the IRGN1 field.
    pub const IRGN1_SHIFT: u32 = 24;
    /// Mask for the IRGN1 field.
    pub const IRGN1_MASK: u64 = 0b11;
    /// Offset of the HWU59 field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the HWU60 field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the ORGN1 field.
    pub const ORGN1_SHIFT: u32 = 26;
    /// Mask for the ORGN1 field.
    pub const ORGN1_MASK: u64 = 0b11;
    /// Offset of the HWU61 field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the HWU62 field.
    pub const HWU62_SHIFT: u32 = 28;
    /// Offset of the SH1 field.
    pub const SH1_SHIFT: u32 = 28;
    /// Mask for the SH1 field.
    pub const SH1_MASK: u64 = 0b11;
    /// Offset of the TBID field.
    pub const TBID_SHIFT: u32 = 29;
    /// Offset of the TCMA field.
    pub const TCMA_SHIFT: u32 = 30;
    /// Offset of the TG1 field.
    pub const TG1_SHIFT: u32 = 30;
    /// Mask for the TG1 field.
    pub const TG1_MASK: u64 = 0b11;
    /// Offset of the IPS field.
    pub const IPS_SHIFT: u32 = 32;
    /// Mask for the IPS field.
    pub const IPS_MASK: u64 = 0b111;
    /// Offset of the MTX field.
    pub const MTX_SHIFT: u32 = 33;
    /// Offset of the AS field.
    pub const AS_SHIFT: u32 = 36;
    /// Offset of the TBI0 field.
    pub const TBI0_SHIFT: u32 = 37;
    /// Offset of the TBI1 field.
    pub const TBI1_SHIFT: u32 = 38;
    /// Offset of the HPD0 field.
    pub const HPD0_SHIFT: u32 = 41;
    /// Offset of the HPD1 field.
    pub const HPD1_SHIFT: u32 = 42;
    /// Offset of the HWU059 field.
    pub const HWU059_SHIFT: u32 = 43;
    /// Offset of the HWU060 field.
    pub const HWU060_SHIFT: u32 = 44;
    /// Offset of the HWU061 field.
    pub const HWU061_SHIFT: u32 = 45;
    /// Offset of the HWU062 field.
    pub const HWU062_SHIFT: u32 = 46;
    /// Offset of the HWU159 field.
    pub const HWU159_SHIFT: u32 = 47;
    /// Offset of the HWU160 field.
    pub const HWU160_SHIFT: u32 = 48;
    /// Offset of the VTB field.
    pub const VTB_SHIFT: u32 = 48;
    /// Mask for the VTB field.
    pub const VTB_MASK: u64 = 0b11111;
    /// Offset of the HWU161 field.
    pub const HWU161_SHIFT: u32 = 49;
    /// Offset of the HWU162 field.
    pub const HWU162_SHIFT: u32 = 50;
    /// Offset of the TBID0 field.
    pub const TBID0_SHIFT: u32 = 51;
    /// Offset of the TBID1 field.
    pub const TBID1_SHIFT: u32 = 52;
    /// Offset of the NFD0 field.
    pub const NFD0_SHIFT: u32 = 53;
    /// Offset of the TVAD field.
    pub const TVAD_SHIFT: u32 = 53;
    /// Offset of the NFD1 field.
    pub const NFD1_SHIFT: u32 = 54;
    /// Offset of the E0PD0 field.
    pub const E0PD0_SHIFT: u32 = 55;
    /// Offset of the E0PD1 field.
    pub const E0PD1_SHIFT: u32 = 56;
    /// Offset of the TCMA0 field.
    pub const TCMA0_SHIFT: u32 = 57;
    /// Offset of the TCMA1 field.
    pub const TCMA1_SHIFT: u32 = 58;
    /// Offset of the MTX0 field.
    pub const MTX0_SHIFT: u32 = 60;
    /// Offset of the MTX1 field.
    pub const MTX1_SHIFT: u32 = 61;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> Self::T0SZ_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> Self::IRGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> Self::ORGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> Self::SH0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> Self::TG0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        (self.bits() >> Self::PS_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        (self.bits() >> Self::T1SZ_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        (self.bits() >> Self::IRGN1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        (self.bits() >> Self::ORGN1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        (self.bits() >> Self::SH1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        (self.bits() >> Self::TG1_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        (self.bits() >> Self::IPS_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        (self.bits() >> Self::VTB_SHIFT) as u8 & 0b11111
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl TcrEl3 {
    /// Offset of the T0SZ field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the T0SZ field.
    pub const T0SZ_MASK: u64 = 0b111111;
    /// Offset of the IRGN0 field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the IRGN0 field.
    pub const IRGN0_MASK: u64 = 0b11;
    /// Offset of the ORGN0 field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the ORGN0 field.
    pub const ORGN0_MASK: u64 = 0b11;
    /// Offset of the SH0 field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the SH0 field.
    pub const SH0_MASK: u64 = 0b11;
    /// Offset of the TG0 field.
    pub const TG0_SHIFT: u32 = 14;
    /// Mask for the TG0 field.
    pub const TG0_MASK: u64 = 0b11;
    /// Offset of the PS field.
    pub const PS_SHIFT: u32 = 16;
    /// Mask for the PS field.
    pub const PS_MASK: u64 = 0b111;
    /// Offset of the TBI field.
    pub const TBI_SHIFT: u32 = 20;
    /// Offset of the HA field.
    pub const HA_SHIFT: u32 = 21;
    /// Offset of the HD field.
    pub const HD_SHIFT: u32 = 22;
    /// Offset of the HPD field.
    pub const HPD_SHIFT: u32 = 24;
    /// Offset of the HWU59 field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the HWU60 field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the HWU61 field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the HWU62 field.
    pub const HWU62_SHIFT: u32 = 28;
    /// Offset of the TBID field.
    pub const TBID_SHIFT: u32 = 29;
    /// Offset of the TCMA field.
    pub const TCMA_SHIFT: u32 = 30;
    /// Offset of the DS field.
    pub const DS_SHIFT: u32 = 32;
    /// Offset of the MTX field.
    pub const MTX_SHIFT: u32 = 33;
    /// Offset of the PnCH field.
    pub const PNCH_SHIFT: u32 = 34;
    /// Offset of the PIE field.
    pub const PIE_SHIFT: u32 = 35;
    /// Offset of the POE field.
    pub const POE_SHIFT: u32 = 36;
    /// Offset of the AIE field.
    pub const AIE_SHIFT: u32 = 37;
    /// Offset of the D128 field.
    pub const D128_SHIFT: u32 = 38;
    /// Offset of the PTTWI field.
    pub const PTTWI_SHIFT: u32 = 41;
    /// Offset of the HAFT field.
    pub const HAFT_SHIFT: u32 = 42;
    /// Offset of the DisCH0 field.
    pub const DISCH0_SHIFT: u32 = 43;
    /// Offset of the POE2F field.
    pub const POE2F_SHIFT: u32 = 44;
    /// Offset of the POIW field.
    pub const POIW_SHIFT: u32 = 45;
    /// Mask for the POIW field.
    pub const POIW_MASK: u64 = 0b111;
    /// Offset of the VTB field.
    pub const VTB_SHIFT: u32 = 48;
    /// Mask for the VTB field.
    pub const VTB_MASK: u64 = 0b11111;
    /// Offset of the TVAD field.
    pub const TVAD_SHIFT: u32 = 53;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> Self::T0SZ_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> Self::IRGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> Self::ORGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> Self::SH0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> Self::TG0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        (self.bits() >> Self::PS_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        (self.bits() >> Self::POIW_SHIFT) as u8 & 0b111
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        (self.bits() >> Self::VTB_SHIFT) as u8 & 0b11111
    }
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl Tfsre0El1 {
    /// Offset of the TF0 field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the TF1 field.
    pub const TF1_SHIFT: u32 = 1;
}

#[cfg(feature = "el1")]
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

#[cfg(feature = "el1")]
impl TfsrEl1 {
    /// Offset of the TF0 field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the TF1 field.
    pub const TF1_SHIFT: u32 = 1;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl TfsrEl2 {
    /// Offset of the TF0 field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the TF1 field.
    pub const TF1_SHIFT: u32 = 1;
}

bitflags! {
    /// `TPIDRRO_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrroEl0: u64 {
    }
}

impl TpidrroEl0 {
    /// Offset of the ThreadID field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the ThreadID field.
    pub const THREADID_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> Self::THREADID_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Offset of the ThreadID field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the ThreadID field.
    pub const THREADID_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> Self::THREADID_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TPIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl TpidrEl1 {
    /// Offset of the ThreadID field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the ThreadID field.
    pub const THREADID_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> Self::THREADID_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl TpidrEl2 {
    /// Offset of the ThreadID field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the ThreadID field.
    pub const THREADID_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> Self::THREADID_SHIFT) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TTBR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl Ttbr0El1 {
    /// Offset of the CnP field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the BADDR[47:1] field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the BADDR[47:1] field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the SKL field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the SKL field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the ASID field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the ASID field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> Self::BADDR_47_1_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> Self::SKL_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> Self::ASID_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TTBR0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl Ttbr0El2 {
    /// Offset of the CnP field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the BADDR[47:1] field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the BADDR[47:1] field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the SKL field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the SKL field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the ASID field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the ASID field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> Self::BADDR_47_1_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> Self::SKL_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> Self::ASID_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `TTBR0_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El3: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el3")]
impl Ttbr0El3 {
    /// Offset of the CnP field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the SKL field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the SKL field.
    pub const SKL_MASK: u64 = 0b11;

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> Self::SKL_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TTBR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1El1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl Ttbr1El1 {
    /// Offset of the CnP field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the BADDR[47:1] field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the BADDR[47:1] field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the SKL field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the SKL field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the ASID field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the ASID field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> Self::BADDR_47_1_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> Self::SKL_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> Self::ASID_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TTBR1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1El2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl Ttbr1El2 {
    /// Offset of the CnP field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the BADDR[47:1] field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the BADDR[47:1] field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the SKL field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the SKL field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the ASID field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the ASID field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> Self::BADDR_47_1_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> Self::SKL_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> Self::ASID_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `VBAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl1: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl VbarEl1 {
    /// Offset of the UT field.
    pub const UT_SHIFT: u32 = 0;
    /// Offset of the VBA field.
    pub const VBA_SHIFT: u32 = 11;
    /// Mask for the VBA field.
    pub const VBA_MASK: u64 = 0b11111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        (self.bits() >> Self::VBA_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VBAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl2: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl VbarEl2 {
    /// Offset of the UT field.
    pub const UT_SHIFT: u32 = 0;
    /// Offset of the VBA field.
    pub const VBA_SHIFT: u32 = 11;
    /// Mask for the VBA field.
    pub const VBA_MASK: u64 = 0b11111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        (self.bits() >> Self::VBA_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl VdisrEl2 {
    /// Offset of the ISS field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the ISS field.
    pub const ISS_MASK: u64 = 0b111111111111111111111111;
    /// Offset of the STATUS field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the STATUS field.
    pub const STATUS_MASK: u64 = 0b111111;
    /// Offset of the LPAE field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the ExT field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the AET field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the AET field.
    pub const AET_MASK: u64 = 0b11;
    /// Offset of the IDS field.
    pub const IDS_SHIFT: u32 = 24;
    /// Offset of the A field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> Self::ISS_SHIFT) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        (self.bits() >> Self::STATUS_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> Self::AET_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl VmpidrEl2 {
    /// Offset of the Aff0 field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the Aff0 field.
    pub const AFF0_MASK: u64 = 0b11111111;
    /// Offset of the Aff1 field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the Aff1 field.
    pub const AFF1_MASK: u64 = 0b11111111;
    /// Offset of the Aff2 field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the Aff2 field.
    pub const AFF2_MASK: u64 = 0b11111111;
    /// Offset of the MT field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the U field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the Aff3 field.
    pub const AFF3_SHIFT: u32 = 32;
    /// Mask for the Aff3 field.
    pub const AFF3_MASK: u64 = 0b11111111;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        (self.bits() >> Self::AFF0_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> Self::AFF1_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> Self::AFF2_SHIFT) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> Self::AFF3_SHIFT) as u8 & 0b11111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VpidrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl VpidrEl2 {
    /// Offset of the Revision field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the Revision field.
    pub const REVISION_MASK: u64 = 0b1111;
    /// Offset of the PartNum field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the PartNum field.
    pub const PARTNUM_MASK: u64 = 0b111111111111;
    /// Offset of the Architecture field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the Architecture field.
    pub const ARCHITECTURE_MASK: u64 = 0b1111;
    /// Offset of the Variant field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the Variant field.
    pub const VARIANT_MASK: u64 = 0b1111;
    /// Offset of the Implementer field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the Implementer field.
    pub const IMPLEMENTER_MASK: u64 = 0b11111111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> Self::REVISION_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        (self.bits() >> Self::PARTNUM_SHIFT) as u16 & 0b111111111111
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        (self.bits() >> Self::ARCHITECTURE_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        (self.bits() >> Self::VARIANT_SHIFT) as u8 & 0b1111
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        (self.bits() >> Self::IMPLEMENTER_SHIFT) as u8 & 0b11111111
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl VsesrEl2 {
    /// Offset of the ISS field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the ISS field.
    pub const ISS_MASK: u64 = 0b111111111111111111111111;
    /// Offset of the ExT field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the AET field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the AET field.
    pub const AET_MASK: u64 = 0b11;
    /// Offset of the IDS field.
    pub const IDS_SHIFT: u32 = 24;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> Self::ISS_SHIFT) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> Self::AET_SHIFT) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl VtcrEl2 {
    /// Offset of the T0SZ field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the T0SZ field.
    pub const T0SZ_MASK: u64 = 0b111111;
    /// Offset of the SL0 field.
    pub const SL0_SHIFT: u32 = 6;
    /// Mask for the SL0 field.
    pub const SL0_MASK: u64 = 0b11;
    /// Offset of the IRGN0 field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the IRGN0 field.
    pub const IRGN0_MASK: u64 = 0b11;
    /// Offset of the ORGN0 field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the ORGN0 field.
    pub const ORGN0_MASK: u64 = 0b11;
    /// Offset of the SH0 field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the SH0 field.
    pub const SH0_MASK: u64 = 0b11;
    /// Offset of the TG0 field.
    pub const TG0_SHIFT: u32 = 14;
    /// Mask for the TG0 field.
    pub const TG0_MASK: u64 = 0b11;
    /// Offset of the PS field.
    pub const PS_SHIFT: u32 = 16;
    /// Mask for the PS field.
    pub const PS_MASK: u64 = 0b111;
    /// Offset of the VS field.
    pub const VS_SHIFT: u32 = 19;
    /// Offset of the HA field.
    pub const HA_SHIFT: u32 = 21;
    /// Offset of the HD field.
    pub const HD_SHIFT: u32 = 22;
    /// Offset of the HWU59 field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the HWU60 field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the HWU61 field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the HWU62 field.
    pub const HWU62_SHIFT: u32 = 28;
    /// Offset of the NSW field.
    pub const NSW_SHIFT: u32 = 29;
    /// Offset of the NSA field.
    pub const NSA_SHIFT: u32 = 30;
    /// Offset of the DS field.
    pub const DS_SHIFT: u32 = 32;
    /// Offset of the SL2 field.
    pub const SL2_SHIFT: u32 = 33;
    /// Offset of the AssuredOnly field.
    pub const ASSUREDONLY_SHIFT: u32 = 34;
    /// Offset of the TL1 field.
    pub const TL1_SHIFT: u32 = 35;
    /// Offset of the S2PIE field.
    pub const S2PIE_SHIFT: u32 = 36;
    /// Offset of the S2POE field.
    pub const S2POE_SHIFT: u32 = 37;
    /// Offset of the D128 field.
    pub const D128_SHIFT: u32 = 38;
    /// Offset of the GCSH field.
    pub const GCSH_SHIFT: u32 = 40;
    /// Offset of the TL0 field.
    pub const TL0_SHIFT: u32 = 41;
    /// Offset of the HAFT field.
    pub const HAFT_SHIFT: u32 = 44;
    /// Offset of the HDBSS field.
    pub const HDBSS_SHIFT: u32 = 45;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> Self::T0SZ_SHIFT) as u8 & 0b111111
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        (self.bits() >> Self::SL0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> Self::IRGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> Self::ORGN0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> Self::SH0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> Self::TG0_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        (self.bits() >> Self::PS_SHIFT) as u8 & 0b111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VTTBR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VttbrEl2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl VttbrEl2 {
    /// Offset of the CnP field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the BADDR field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the BADDR field.
    pub const BADDR_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the SKL field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the SKL field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the VMID field.
    pub const VMID_SHIFT: u32 = 48;
    /// Mask for the VMID field.
    pub const VMID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> Self::BADDR_SHIFT) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> Self::SKL_SHIFT) as u8 & 0b11
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u16 {
        (self.bits() >> Self::VMID_SHIFT) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ZCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ZcrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl ZcrEl3 {
    /// Offset of the LEN field.
    pub const LEN_SHIFT: u32 = 0;
    /// Mask for the LEN field.
    pub const LEN_MASK: u64 = 0b1111;

    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> Self::LEN_SHIFT) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
read_write_sysreg!(actlr_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(actlr_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(afsr0_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(afsr0_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(afsr1_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(afsr1_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(amair_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(amair_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apiakeyhi_el1, u64: ApiakeyhiEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apiakeylo_el1, u64: ApiakeyloEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(ccsidr_el1, u64: CcsidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(clidr_el1, u64: ClidrEl1, safe, fake::SYSREGS);
read_write_sysreg!(cntfrq_el0, u64: CntfrqEl0, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthctl_el2, u64: CnthctlEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cntvoff_el2, u64: CntvoffEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(contextidr_el1, u64: ContextidrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(contextidr_el2, u64: ContextidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(cpacr_el1, u64: CpacrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cptr_el2, u64: CptrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(cptr_el3, u64: CptrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(csselr_el1, u64: CsselrEl1, safe_read, safe_write, fake::SYSREGS);
read_sysreg!(ctr_el0, u64: CtrEl0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(disr_el1: s3_0_c12_c1_1, u64: DisrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(dit, u64: Dit, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(elr_el1, u64: ElrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(elr_el2, u64: ElrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(esr_el1, u64: EsrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(esr_el2, u64: EsrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(esr_el3, u64: EsrEl3, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(far_el1, u64: FarEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(far_el2, u64: FarEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(gcr_el1: s3_0_c1_c0_6, u64: GcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(gcscr_el1, u64: GcscrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(gcscr_el2, u64: GcscrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hacr_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hcrx_el2: s3_4_c1_c2_2, u64: HcrxEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hcr_el2, u64: HcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdfgrtr2_el2: s3_4_c3_c1_0, u64: Hdfgrtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdfgwtr2_el2: s3_4_c3_c1_1, u64: Hdfgwtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgitr2_el2: s3_4_c3_c1_7, u64: Hfgitr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgrtr2_el2: s3_4_c3_c1_2, u64: Hfgrtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgwtr2_el2: s3_4_c3_c1_3, u64: Hfgwtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgwtr_el2: s3_4_c1_c1_5, u64: HfgwtrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hpfar_el2, u64: HpfarEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hstr_el2, u64, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_sre_el1, u64: IccSreEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(icc_sre_el2, u64: IccSreEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The SRE bit of `icc_sre_el3` must not be changed from 1 to 0, as this can result in unpredictable behaviour.
    icc_sre_el3, u64: IccSreEl3, safe_read, fake::SYSREGS
}
#[cfg(feature = "el2")]
read_write_sysreg!(ich_hcr_el2, u64: IchHcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_vmcr_el2, u64: IchVmcrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64dfr0_el1, u64: IdAa64dfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64dfr1_el1, u64: IdAa64dfr1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr0_el1, u64: IdAa64mmfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr1_el1, u64: IdAa64mmfr1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr2_el1, u64: IdAa64mmfr2El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr3_el1, u64: IdAa64mmfr3El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64pfr0_el1, u64: IdAa64pfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64pfr1_el1, u64: IdAa64pfr1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64smfr0_el1, u64: IdAa64smfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(isr_el1, u64: IsrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mair_el1, u64: MairEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mair_el2, u64: MairEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 memory attribute indirection register.
    mair_el3, u64: MairEl3, safe_read, fake::SYSREGS
}
#[cfg(feature = "el1")]
read_write_sysreg!(mdccint_el1, u64: MdccintEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mdcr_el2, u64: MdcrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mdcr_el3, u64: MdcrEl3, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mdscr_el1, u64: MdscrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(midr_el1, u64: MidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpam2_el2: s3_4_c10_c5_0, u64: Mpam2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mpam3_el3: s3_6_c10_c5_0, u64: Mpam3El3, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamhcr_el2: s3_4_c10_c4_0, u64: MpamhcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mpamidr_el1: s3_0_c10_c4_4, u64: MpamidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm0_el2: s3_4_c10_c6_0, u64: Mpamvpm0El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm1_el2: s3_4_c10_c6_1, u64: Mpamvpm1El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm2_el2: s3_4_c10_c6_2, u64: Mpamvpm2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm3_el2: s3_4_c10_c6_3, u64: Mpamvpm3El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm4_el2: s3_4_c10_c6_4, u64: Mpamvpm4El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm5_el2: s3_4_c10_c6_5, u64: Mpamvpm5El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm6_el2: s3_4_c10_c6_6, u64: Mpamvpm6El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm7_el2: s3_4_c10_c6_7, u64: Mpamvpm7El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpmv_el2: s3_4_c10_c4_1, u64: MpamvpmvEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mpidr_el1, u64: MpidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(par_el1, u64: ParEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(pmcr_el0, u64: PmcrEl0, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(rgsr_el1: s3_0_c1_c0_5, u64: RgsrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(scr_el3, u64: ScrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(sctlr_el1, u64: SctlrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(sctlr_el2, u64: SctlrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 system control register.
    sctlr_el3, u64: SctlrEl3, safe_read, fake::SYSREGS
}
#[cfg(feature = "el3")]
read_write_sysreg!(smcr_el3, u64: SmcrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(spsr_el1, u64: SpsrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(spsr_el2, u64: SpsrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(spsr_el3, u64: SpsrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(sp_el1, u64: SpEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(sp_el2, u64: SpEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tcr2_el1, u64: Tcr2El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tcr2_el2, u64: Tcr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tcr_el1, u64: TcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tcr_el2, u64: TcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 translation control register.
    tcr_el3, u64: TcrEl3, safe_read, fake::SYSREGS
}
#[cfg(feature = "el1")]
read_write_sysreg!(tfsre0_el1: s3_0_c5_c6_1, u64: Tfsre0El1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tfsr_el1: s3_0_c5_c6_0, u64: TfsrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tfsr_el2: s3_4_c5_c6_0, u64: TfsrEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(tpidrro_el0, u64: TpidrroEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidr_el0, u64: TpidrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tpidr_el1, u64: TpidrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tpidr_el2, u64: TpidrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el1, u64: Ttbr0El1, safe_read, fake::SYSREGS
}
#[cfg(feature = "el2")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el2, u64: Ttbr0El2, safe_read, fake::SYSREGS
}
#[cfg(feature = "el3")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el3, u64: Ttbr0El3, safe_read, fake::SYSREGS
}
#[cfg(feature = "el1")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el1, u64: Ttbr1El1, safe_read, fake::SYSREGS
}
#[cfg(feature = "el2")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el2, u64: Ttbr1El2, safe_read, fake::SYSREGS
}
#[cfg(feature = "el1")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el1, u64: VbarEl1, safe_read, fake::SYSREGS
}
#[cfg(feature = "el2")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el2, u64: VbarEl2, safe_read, fake::SYSREGS
}
#[cfg(feature = "el2")]
read_write_sysreg!(vdisr_el2: s3_4_c12_c1_1, u64: VdisrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vmpidr_el2, u64: VmpidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vpidr_el2, u64: VpidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vsesr_el2: s3_4_c5_c2_3, u64: VsesrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vtcr_el2, u64: VtcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned stage 2 translation table.
    vttbr_el2, u64: VttbrEl2, safe_read, fake::SYSREGS
}
#[cfg(feature = "el3")]
read_write_sysreg!(zcr_el3, u64: ZcrEl3, safe_read, fake::SYSREGS);
