// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Arm CPU system registers.

// This file is generated, do not edit manually.

use bitflags::bitflags;

bitflags! {
    /// `AMEVCNTVOFF00_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amevcntvoff00El2: u64 {
    }
}

impl Amevcntvoff00El2 {
    /// Offset of the `VOffset` field.
    pub const VOFFSET_SHIFT: u32 = 0;
    /// Mask for the `VOffset` field.
    pub const VOFFSET_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        (self.bits() >> Self::VOFFSET_SHIFT) & Self::VOFFSET_MASK
    }

    /// Sets the value of the `VOffset` field.
    pub const fn set_voffset(&mut self, value: u64) {
        let offset = Self::VOFFSET_SHIFT;
        assert!(value & Self::VOFFSET_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VOFFSET_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `VOffset` field set to the given value.
    pub const fn with_voffset(mut self, value: u64) -> Self {
        self.set_voffset(value);
        self
    }
}

/// `AMEVCNTVOFF010_EL2` system register value.
pub type Amevcntvoff010El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF011_EL2` system register value.
pub type Amevcntvoff011El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF012_EL2` system register value.
pub type Amevcntvoff012El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF013_EL2` system register value.
pub type Amevcntvoff013El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF014_EL2` system register value.
pub type Amevcntvoff014El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF015_EL2` system register value.
pub type Amevcntvoff015El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF01_EL2` system register value.
pub type Amevcntvoff01El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF02_EL2` system register value.
pub type Amevcntvoff02El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF03_EL2` system register value.
pub type Amevcntvoff03El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF04_EL2` system register value.
pub type Amevcntvoff04El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF05_EL2` system register value.
pub type Amevcntvoff05El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF06_EL2` system register value.
pub type Amevcntvoff06El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF07_EL2` system register value.
pub type Amevcntvoff07El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF08_EL2` system register value.
pub type Amevcntvoff08El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF09_EL2` system register value.
pub type Amevcntvoff09El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF10_EL2` system register value.
pub type Amevcntvoff10El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF110_EL2` system register value.
pub type Amevcntvoff110El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF111_EL2` system register value.
pub type Amevcntvoff111El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF112_EL2` system register value.
pub type Amevcntvoff112El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF113_EL2` system register value.
pub type Amevcntvoff113El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF114_EL2` system register value.
pub type Amevcntvoff114El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF115_EL2` system register value.
pub type Amevcntvoff115El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF11_EL2` system register value.
pub type Amevcntvoff11El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF12_EL2` system register value.
pub type Amevcntvoff12El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF13_EL2` system register value.
pub type Amevcntvoff13El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF14_EL2` system register value.
pub type Amevcntvoff14El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF15_EL2` system register value.
pub type Amevcntvoff15El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF16_EL2` system register value.
pub type Amevcntvoff16El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF17_EL2` system register value.
pub type Amevcntvoff17El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF18_EL2` system register value.
pub type Amevcntvoff18El2 = Amevcntvoff00El2;

/// `AMEVCNTVOFF19_EL2` system register value.
pub type Amevcntvoff19El2 = Amevcntvoff00El2;

bitflags! {
    /// `BRBCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct BrbcrEl2: u64 {
        /// `E0HBRE` bit.
        const E0HBRE = 1 << 0;
        /// `E2BRE` bit.
        const E2BRE = 1 << 1;
        /// `CC` bit.
        const CC = 1 << 3;
        /// `MPRED` bit.
        const MPRED = 1 << 4;
        /// `FZP` bit.
        const FZP = 1 << 8;
        /// `FZPSS` bit.
        const FZPSS = 1 << 9;
        /// `ERTN` bit.
        const ERTN = 1 << 22;
        /// `EXCEPTION` bit.
        const EXCEPTION = 1 << 23;
    }
}

impl BrbcrEl2 {
    /// Offset of the `E0HBRE` field.
    pub const E0HBRE_SHIFT: u32 = 0;
    /// Offset of the `E2BRE` field.
    pub const E2BRE_SHIFT: u32 = 1;
    /// Offset of the `CC` field.
    pub const CC_SHIFT: u32 = 3;
    /// Offset of the `MPRED` field.
    pub const MPRED_SHIFT: u32 = 4;
    /// Offset of the `TS` field.
    pub const TS_SHIFT: u32 = 5;
    /// Mask for the `TS` field.
    pub const TS_MASK: u64 = 0b11;
    /// Offset of the `FZP` field.
    pub const FZP_SHIFT: u32 = 8;
    /// Offset of the `FZPSS` field.
    pub const FZPSS_SHIFT: u32 = 9;
    /// Offset of the `ERTN` field.
    pub const ERTN_SHIFT: u32 = 22;
    /// Offset of the `EXCEPTION` field.
    pub const EXCEPTION_SHIFT: u32 = 23;

    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        ((self.bits() >> Self::TS_SHIFT) & Self::TS_MASK) as u8
    }

    /// Sets the value of the `TS` field.
    pub const fn set_ts(&mut self, value: u8) {
        let offset = Self::TS_SHIFT;
        assert!(value & (Self::TS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TS` field set to the given value.
    pub const fn with_ts(mut self, value: u8) -> Self {
        self.set_ts(value);
        self
    }
}

bitflags! {
    /// `CNTHCTL_EL2` system register value.
    ///
    /// Counter-timer Hypervisor Control Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `EL0PCTEN` field.
    pub const EL0PCTEN_SHIFT: u32 = 0;
    /// Offset of the `EL0VCTEN` field.
    pub const EL0VCTEN_SHIFT: u32 = 1;
    /// Offset of the `EL1PCEN` field.
    pub const EL1PCEN_SHIFT: u32 = 1;
    /// Offset of the `EVNTEN` field.
    pub const EVNTEN_SHIFT: u32 = 2;
    /// Offset of the `EVNTDIR` field.
    pub const EVNTDIR_SHIFT: u32 = 3;
    /// Offset of the `EVNTI` field.
    pub const EVNTI_SHIFT: u32 = 4;
    /// Mask for the `EVNTI` field.
    pub const EVNTI_MASK: u64 = 0b1111;
    /// Offset of the `EL0VTEN` field.
    pub const EL0VTEN_SHIFT: u32 = 8;
    /// Offset of the `EL0PTEN` field.
    pub const EL0PTEN_SHIFT: u32 = 9;
    /// Offset of the `EL1PTEN` field.
    pub const EL1PTEN_SHIFT: u32 = 11;
    /// Offset of the `ECV` field.
    pub const ECV_SHIFT: u32 = 12;
    /// Offset of the `EL1TVT` field.
    pub const EL1TVT_SHIFT: u32 = 13;
    /// Offset of the `EL1TVCT` field.
    pub const EL1TVCT_SHIFT: u32 = 14;
    /// Offset of the `EL1NVPCT` field.
    pub const EL1NVPCT_SHIFT: u32 = 15;
    /// Offset of the `EL1NVVCT` field.
    pub const EL1NVVCT_SHIFT: u32 = 16;
    /// Offset of the `EVNTIS` field.
    pub const EVNTIS_SHIFT: u32 = 17;
    /// Offset of the `CNTVMASK` field.
    pub const CNTVMASK_SHIFT: u32 = 18;
    /// Offset of the `CNTPMASK` field.
    pub const CNTPMASK_SHIFT: u32 = 19;

    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        ((self.bits() >> Self::EVNTI_SHIFT) & Self::EVNTI_MASK) as u8
    }

    /// Sets the value of the `EVNTI` field.
    pub const fn set_evnti(&mut self, value: u8) {
        let offset = Self::EVNTI_SHIFT;
        assert!(value & (Self::EVNTI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVNTI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EVNTI` field set to the given value.
    pub const fn with_evnti(mut self, value: u8) -> Self {
        self.set_evnti(value);
        self
    }
}

bitflags! {
    /// `CNTHPS_CTL_EL2` system register value.
    ///
    /// Counter-timer Secure Physical Timer Control Register (EL2)
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsCtlEl2: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CnthpsCtlEl2 {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTHPS_CVAL_EL2` system register value.
    ///
    /// Counter-timer Secure Physical Timer CompareValue Register (EL2)
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsCvalEl2: u64 {
    }
}

impl CnthpsCvalEl2 {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> Self::COMPAREVALUE_SHIFT) & Self::COMPAREVALUE_MASK
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & Self::COMPAREVALUE_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `CompareValue` field set to the given value.
    pub const fn with_comparevalue(mut self, value: u64) -> Self {
        self.set_comparevalue(value);
        self
    }
}

bitflags! {
    /// `CNTHPS_TVAL_EL2` system register value.
    ///
    /// Counter-timer Secure Physical Timer TimerValue Register (EL2)
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsTvalEl2: u64 {
    }
}

impl CnthpsTvalEl2 {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & Self::TIMERVALUE_MASK) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TimerValue` field set to the given value.
    pub const fn with_timervalue(mut self, value: u32) -> Self {
        self.set_timervalue(value);
        self
    }
}

/// `CNTHP_CTL_EL2` system register value.
///
/// Counter-timer Hypervisor Physical Timer Control Register
pub type CnthpCtlEl2 = CnthpsCtlEl2;

/// `CNTHP_CVAL_EL2` system register value.
///
/// Counter-timer Physical Timer CompareValue Register (EL2)
pub type CnthpCvalEl2 = CnthpsCvalEl2;

/// `CNTHP_TVAL_EL2` system register value.
///
/// Counter-timer Physical Timer TimerValue Register (EL2)
pub type CnthpTvalEl2 = CnthpsTvalEl2;

/// `CNTHVS_CTL_EL2` system register value.
///
/// Counter-timer Secure Virtual Timer Control Register (EL2)
pub type CnthvsCtlEl2 = CnthpsCtlEl2;

/// `CNTHVS_CVAL_EL2` system register value.
///
/// Counter-timer Secure Virtual Timer CompareValue Register (EL2)
pub type CnthvsCvalEl2 = CnthpsCvalEl2;

/// `CNTHVS_TVAL_EL2` system register value.
///
/// Counter-timer Secure Virtual Timer TimerValue Register (EL2)
pub type CnthvsTvalEl2 = CnthpsTvalEl2;

/// `CNTHV_CTL_EL2` system register value.
///
/// Counter-timer Virtual Timer Control Register (EL2)
pub type CnthvCtlEl2 = CnthpsCtlEl2;

/// `CNTHV_CVAL_EL2` system register value.
///
/// Counter-timer Virtual Timer CompareValue Register (EL2)
pub type CnthvCvalEl2 = CnthpsCvalEl2;

/// `CNTHV_TVAL_EL2` system register value.
///
/// Counter-timer Virtual Timer TimerValue Register (EL2)
pub type CnthvTvalEl2 = CnthpsTvalEl2;

bitflags! {
    /// `CNTPOFF_EL2` system register value.
    ///
    /// Counter-timer Physical Offset Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpoffEl2: u64 {
    }
}

impl CntpoffEl2 {
    /// Offset of the `PO` field.
    pub const PO_SHIFT: u32 = 0;
    /// Mask for the `PO` field.
    pub const PO_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `PO` field.
    pub const fn po(self) -> u64 {
        (self.bits() >> Self::PO_SHIFT) & Self::PO_MASK
    }

    /// Sets the value of the `PO` field.
    pub const fn set_po(&mut self, value: u64) {
        let offset = Self::PO_SHIFT;
        assert!(value & Self::PO_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::PO_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `PO` field set to the given value.
    pub const fn with_po(mut self, value: u64) -> Self {
        self.set_po(value);
        self
    }
}

bitflags! {
    /// `CNTVOFF_EL2` system register value.
    ///
    /// Counter-timer Virtual Offset Register
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvoffEl2: u64 {
    }
}

impl CntvoffEl2 {
    /// Offset of the `VOffset` field.
    pub const VOFFSET_SHIFT: u32 = 0;
    /// Mask for the `VOffset` field.
    pub const VOFFSET_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        (self.bits() >> Self::VOFFSET_SHIFT) & Self::VOFFSET_MASK
    }

    /// Sets the value of the `VOffset` field.
    pub const fn set_voffset(&mut self, value: u64) {
        let offset = Self::VOFFSET_SHIFT;
        assert!(value & Self::VOFFSET_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VOFFSET_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `VOffset` field set to the given value.
    pub const fn with_voffset(mut self, value: u64) -> Self {
        self.set_voffset(value);
        self
    }
}

bitflags! {
    /// `CONTEXTIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl2: u64 {
    }
}

impl ContextidrEl2 {
    /// Offset of the `PROCID` field.
    pub const PROCID_SHIFT: u32 = 0;
    /// Mask for the `PROCID` field.
    pub const PROCID_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        ((self.bits() >> Self::PROCID_SHIFT) & Self::PROCID_MASK) as u32
    }

    /// Sets the value of the `PROCID` field.
    pub const fn set_procid(&mut self, value: u32) {
        let offset = Self::PROCID_SHIFT;
        assert!(value & (Self::PROCID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PROCID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PROCID` field set to the given value.
    pub const fn with_procid(mut self, value: u32) -> Self {
        self.set_procid(value);
        self
    }
}

bitflags! {
    /// `CPTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CptrEl2: u64 {
        /// RES1 bits in the `CPTR_EL2` register.
        const RES1 = 0b10_0010_1111_1111;
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
    /// Offset of the `TZ` field.
    pub const TZ_SHIFT: u32 = 8;
    /// Offset of the `TFP` field.
    pub const TFP_SHIFT: u32 = 10;
    /// Offset of the `TSM` field.
    pub const TSM_SHIFT: u32 = 12;
    /// Offset of the `ZEN` field.
    pub const ZEN_SHIFT: u32 = 16;
    /// Mask for the `ZEN` field.
    pub const ZEN_MASK: u64 = 0b11;
    /// Offset of the `FPEN` field.
    pub const FPEN_SHIFT: u32 = 20;
    /// Mask for the `FPEN` field.
    pub const FPEN_MASK: u64 = 0b11;
    /// Offset of the `SMEN` field.
    pub const SMEN_SHIFT: u32 = 24;
    /// Mask for the `SMEN` field.
    pub const SMEN_MASK: u64 = 0b11;
    /// Offset of the `E0POE` field.
    pub const E0POE_SHIFT: u32 = 29;
    /// Offset of the `TAM` field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the `TCPAC` field.
    pub const TCPAC_SHIFT: u32 = 31;
    /// Offset of the `E0TP0E` field.
    pub const E0TP0E_SHIFT: u32 = 32;
    /// Offset of the `E0TP1E` field.
    pub const E0TP1E_SHIFT: u32 = 33;

    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        ((self.bits() >> Self::ZEN_SHIFT) & Self::ZEN_MASK) as u8
    }

    /// Sets the value of the `ZEN` field.
    pub const fn set_zen(&mut self, value: u8) {
        let offset = Self::ZEN_SHIFT;
        assert!(value & (Self::ZEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ZEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ZEN` field set to the given value.
    pub const fn with_zen(mut self, value: u8) -> Self {
        self.set_zen(value);
        self
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        ((self.bits() >> Self::FPEN_SHIFT) & Self::FPEN_MASK) as u8
    }

    /// Sets the value of the `FPEN` field.
    pub const fn set_fpen(&mut self, value: u8) {
        let offset = Self::FPEN_SHIFT;
        assert!(value & (Self::FPEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FPEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `FPEN` field set to the given value.
    pub const fn with_fpen(mut self, value: u8) -> Self {
        self.set_fpen(value);
        self
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        ((self.bits() >> Self::SMEN_SHIFT) & Self::SMEN_MASK) as u8
    }

    /// Sets the value of the `SMEN` field.
    pub const fn set_smen(&mut self, value: u8) {
        let offset = Self::SMEN_SHIFT;
        assert!(value & (Self::SMEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SMEN` field set to the given value.
    pub const fn with_smen(mut self, value: u8) -> Self {
        self.set_smen(value);
        self
    }
}

bitflags! {
    /// `ELR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl2: u64 {
    }
}

impl ElrEl2 {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> Self::ADDR_SHIFT) & Self::ADDR_MASK
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u64) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & Self::ADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ADDR` field set to the given value.
    pub const fn with_addr(mut self, value: u64) -> Self {
        self.set_addr(value);
        self
    }
}

bitflags! {
    /// `ESR_EL2` system register value.
    #[derive(Clone, Copy, Eq, Default, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl2: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

impl EsrEl2 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b1_1111_1111_1111_1111_1111_1111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the `EC` field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the `EC` field.
    pub const EC_MASK: u64 = 0b11_1111;
    /// Offset of the `ISS2` field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the `ISS2` field.
    pub const ISS2_MASK: u64 = 0b1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & Self::ISS_MASK) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ISS` field set to the given value.
    pub const fn with_iss(mut self, value: u32) -> Self {
        self.set_iss(value);
        self
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> Self::EC_SHIFT) & Self::EC_MASK) as u8
    }

    /// Sets the value of the `EC` field.
    pub const fn set_ec(&mut self, value: u8) {
        let offset = Self::EC_SHIFT;
        assert!(value & (Self::EC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EC` field set to the given value.
    pub const fn with_ec(mut self, value: u8) -> Self {
        self.set_ec(value);
        self
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> Self::ISS2_SHIFT) & Self::ISS2_MASK) as u32
    }

    /// Sets the value of the `ISS2` field.
    pub const fn set_iss2(&mut self, value: u32) {
        let offset = Self::ISS2_SHIFT;
        assert!(value & (Self::ISS2_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ISS2` field set to the given value.
    pub const fn with_iss2(mut self, value: u32) -> Self {
        self.set_iss2(value);
        self
    }
}

bitflags! {
    /// `FAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl2: u64 {
    }
}

impl FarEl2 {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        (self.bits() >> Self::VA_SHIFT) & Self::VA_MASK
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u64) {
        let offset = Self::VA_SHIFT;
        assert!(value & Self::VA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VA` field set to the given value.
    pub const fn with_va(mut self, value: u64) -> Self {
        self.set_va(value);
        self
    }
}

bitflags! {
    /// `GCSCR_EL2` system register value.
    ///
    /// Guarded Control Stack Control register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl GcscrEl2 {
    /// Offset of the `PCRSEL` field.
    pub const PCRSEL_SHIFT: u32 = 0;
    /// Offset of the `RVCHKEN` field.
    pub const RVCHKEN_SHIFT: u32 = 5;
    /// Offset of the `EXLOCKEN` field.
    pub const EXLOCKEN_SHIFT: u32 = 6;
    /// Offset of the `PUSHMEn` field.
    pub const PUSHMEN_SHIFT: u32 = 8;
    /// Offset of the `STREn` field.
    pub const STREN_SHIFT: u32 = 9;
}

bitflags! {
    /// `GCSPR_EL2` system register value.
    ///
    /// Guarded Control Stack Pointer register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcsprEl2: u64 {
    }
}

impl GcsprEl2 {
    /// Offset of the `PTR[63:3]` field.
    pub const PTR_63_3_SHIFT: u32 = 3;
    /// Mask for the `PTR[63:3]` field.
    pub const PTR_63_3_MASK: u64 =
        0b1_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `PTR[63:3]` field.
    pub const fn ptr_63_3(self) -> u64 {
        (self.bits() >> Self::PTR_63_3_SHIFT) & Self::PTR_63_3_MASK
    }

    /// Sets the value of the `PTR[63:3]` field.
    pub const fn set_ptr_63_3(&mut self, value: u64) {
        let offset = Self::PTR_63_3_SHIFT;
        assert!(value & Self::PTR_63_3_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PTR_63_3_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `PTR[63:3]` field set to the given value.
    pub const fn with_ptr_63_3(mut self, value: u64) -> Self {
        self.set_ptr_63_3(value);
        self
    }
}

bitflags! {
    /// `HAFGRTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HafgrtrEl2: u64 {
        /// `AMEVCNTR0<x>_EL0` bit 0.
        const AMEVCNTR00_EL0 = 1 << 1;
        /// `AMEVCNTR0<x>_EL0` bit 1.
        const AMEVCNTR01_EL0 = 1 << 2;
        /// `AMEVCNTR0<x>_EL0` bit 2.
        const AMEVCNTR02_EL0 = 1 << 3;
        /// `AMEVCNTR0<x>_EL0` bit 3.
        const AMEVCNTR03_EL0 = 1 << 4;
    }
}

impl HafgrtrEl2 {
    /// Offset of the `AMEVCNTR0<x>_EL0` field.
    pub const AMEVCNTR0_EL0_SHIFT: u32 = 1;
}

bitflags! {
    /// `HCRX_EL2` system register value.
    ///
    /// Extended Hypervisor Configuration Register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl HcrxEl2 {
    /// Offset of the `EnAS0` field.
    pub const ENAS0_SHIFT: u32 = 0;
    /// Offset of the `EnALS` field.
    pub const ENALS_SHIFT: u32 = 1;
    /// Offset of the `EnASR` field.
    pub const ENASR_SHIFT: u32 = 2;
    /// Offset of the `FnXS` field.
    pub const FNXS_SHIFT: u32 = 3;
    /// Offset of the `FGTnXS` field.
    pub const FGTNXS_SHIFT: u32 = 4;
    /// Offset of the `SMPME` field.
    pub const SMPME_SHIFT: u32 = 5;
    /// Offset of the `TALLINT` field.
    pub const TALLINT_SHIFT: u32 = 6;
    /// Offset of the `VINMI` field.
    pub const VINMI_SHIFT: u32 = 7;
    /// Offset of the `VFNMI` field.
    pub const VFNMI_SHIFT: u32 = 8;
    /// Offset of the `CMOW` field.
    pub const CMOW_SHIFT: u32 = 9;
    /// Offset of the `MCE2` field.
    pub const MCE2_SHIFT: u32 = 10;
    /// Offset of the `MSCEn` field.
    pub const MSCEN_SHIFT: u32 = 11;
    /// Offset of the `TCR2En` field.
    pub const TCR2EN_SHIFT: u32 = 14;
    /// Offset of the `SCTLR2En` field.
    pub const SCTLR2EN_SHIFT: u32 = 15;
    /// Offset of the `PTTWI` field.
    pub const PTTWI_SHIFT: u32 = 16;
    /// Offset of the `D128En` field.
    pub const D128EN_SHIFT: u32 = 17;
    /// Offset of the `EnSNERR` field.
    pub const ENSNERR_SHIFT: u32 = 18;
    /// Offset of the `TMEA` field.
    pub const TMEA_SHIFT: u32 = 19;
    /// Offset of the `EnSDERR` field.
    pub const ENSDERR_SHIFT: u32 = 20;
    /// Offset of the `EnIDCP128` field.
    pub const ENIDCP128_SHIFT: u32 = 21;
    /// Offset of the `GCSEn` field.
    pub const GCSEN_SHIFT: u32 = 22;
    /// Offset of the `EnFPM` field.
    pub const ENFPM_SHIFT: u32 = 23;
    /// Offset of the `PACMEn` field.
    pub const PACMEN_SHIFT: u32 = 24;
    /// Offset of the `VTLBIDEn` field.
    pub const VTLBIDEN_SHIFT: u32 = 25;
    /// Offset of the `SRMASKEn` field.
    pub const SRMASKEN_SHIFT: u32 = 26;
    /// Offset of the `NVTGE` field.
    pub const NVTGE_SHIFT: u32 = 27;
    /// Offset of the `POE2En` field.
    pub const POE2EN_SHIFT: u32 = 29;
    /// Offset of the `TPLIMEn` field.
    pub const TPLIMEN_SHIFT: u32 = 30;
    /// Offset of the `FDIT` field.
    pub const FDIT_SHIFT: u32 = 31;
    /// Offset of the `NVnTTLB` field.
    pub const NVNTTLB_SHIFT: u32 = 32;
    /// Offset of the `NVnTTLBIS` field.
    pub const NVNTTLBIS_SHIFT: u32 = 33;
    /// Offset of the `NVnTTLBOS` field.
    pub const NVNTTLBOS_SHIFT: u32 = 34;
    /// Offset of the `VTLBIDOSEn` field.
    pub const VTLBIDOSEN_SHIFT: u32 = 35;
    /// Offset of the `FNB` field.
    pub const FNB_SHIFT: u32 = 36;
    /// Offset of the `VTE` field.
    pub const VTE_SHIFT: u32 = 37;
    /// Offset of the `VTAO` field.
    pub const VTAO_SHIFT: u32 = 38;
    /// Offset of the `VTCO` field.
    pub const VTCO_SHIFT: u32 = 39;
}

bitflags! {
    /// `HCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `VM` field.
    pub const VM_SHIFT: u32 = 0;
    /// Offset of the `SWIO` field.
    pub const SWIO_SHIFT: u32 = 1;
    /// Offset of the `PTW` field.
    pub const PTW_SHIFT: u32 = 2;
    /// Offset of the `FMO` field.
    pub const FMO_SHIFT: u32 = 3;
    /// Offset of the `IMO` field.
    pub const IMO_SHIFT: u32 = 4;
    /// Offset of the `AMO` field.
    pub const AMO_SHIFT: u32 = 5;
    /// Offset of the `VF` field.
    pub const VF_SHIFT: u32 = 6;
    /// Offset of the `VI` field.
    pub const VI_SHIFT: u32 = 7;
    /// Offset of the `VSE` field.
    pub const VSE_SHIFT: u32 = 8;
    /// Offset of the `FB` field.
    pub const FB_SHIFT: u32 = 9;
    /// Offset of the `BSU` field.
    pub const BSU_SHIFT: u32 = 10;
    /// Mask for the `BSU` field.
    pub const BSU_MASK: u64 = 0b11;
    /// Offset of the `DC` field.
    pub const DC_SHIFT: u32 = 12;
    /// Offset of the `TWI` field.
    pub const TWI_SHIFT: u32 = 13;
    /// Offset of the `TWE` field.
    pub const TWE_SHIFT: u32 = 14;
    /// Offset of the `TID0` field.
    pub const TID0_SHIFT: u32 = 15;
    /// Offset of the `TID1` field.
    pub const TID1_SHIFT: u32 = 16;
    /// Offset of the `TID2` field.
    pub const TID2_SHIFT: u32 = 17;
    /// Offset of the `TID3` field.
    pub const TID3_SHIFT: u32 = 18;
    /// Offset of the `TSC` field.
    pub const TSC_SHIFT: u32 = 19;
    /// Offset of the `TIDCP` field.
    pub const TIDCP_SHIFT: u32 = 20;
    /// Offset of the `TACR` field.
    pub const TACR_SHIFT: u32 = 21;
    /// Offset of the `TSW` field.
    pub const TSW_SHIFT: u32 = 22;
    /// Offset of the `TPCP` field.
    pub const TPCP_SHIFT: u32 = 23;
    /// Offset of the `TPU` field.
    pub const TPU_SHIFT: u32 = 24;
    /// Offset of the `TTLB` field.
    pub const TTLB_SHIFT: u32 = 25;
    /// Offset of the `TVM` field.
    pub const TVM_SHIFT: u32 = 26;
    /// Offset of the `TGE` field.
    pub const TGE_SHIFT: u32 = 27;
    /// Offset of the `TDZ` field.
    pub const TDZ_SHIFT: u32 = 28;
    /// Offset of the `HCD` field.
    pub const HCD_SHIFT: u32 = 29;
    /// Offset of the `TRVM` field.
    pub const TRVM_SHIFT: u32 = 30;
    /// Offset of the `RW` field.
    pub const RW_SHIFT: u32 = 31;
    /// Offset of the `CD` field.
    pub const CD_SHIFT: u32 = 32;
    /// Offset of the `ID` field.
    pub const ID_SHIFT: u32 = 33;
    /// Offset of the `E2H` field.
    pub const E2H_SHIFT: u32 = 34;
    /// Offset of the `TLOR` field.
    pub const TLOR_SHIFT: u32 = 35;
    /// Offset of the `TERR` field.
    pub const TERR_SHIFT: u32 = 36;
    /// Offset of the `TEA` field.
    pub const TEA_SHIFT: u32 = 37;
    /// Offset of the `APK` field.
    pub const APK_SHIFT: u32 = 40;
    /// Offset of the `API` field.
    pub const API_SHIFT: u32 = 41;
    /// Offset of the `NV` field.
    pub const NV_SHIFT: u32 = 42;
    /// Offset of the `NV1` field.
    pub const NV1_SHIFT: u32 = 43;
    /// Offset of the `AT` field.
    pub const AT_SHIFT: u32 = 44;
    /// Offset of the `NV2` field.
    pub const NV2_SHIFT: u32 = 45;
    /// Offset of the `FWB` field.
    pub const FWB_SHIFT: u32 = 46;
    /// Offset of the `FIEN` field.
    pub const FIEN_SHIFT: u32 = 47;
    /// Offset of the `GPF` field.
    pub const GPF_SHIFT: u32 = 48;
    /// Offset of the `TID4` field.
    pub const TID4_SHIFT: u32 = 49;
    /// Offset of the `TICAB` field.
    pub const TICAB_SHIFT: u32 = 50;
    /// Offset of the `AMVOFFEN` field.
    pub const AMVOFFEN_SHIFT: u32 = 51;
    /// Offset of the `TOCU` field.
    pub const TOCU_SHIFT: u32 = 52;
    /// Offset of the `EnSCXT` field.
    pub const ENSCXT_SHIFT: u32 = 53;
    /// Offset of the `TTLBIS` field.
    pub const TTLBIS_SHIFT: u32 = 54;
    /// Offset of the `TTLBOS` field.
    pub const TTLBOS_SHIFT: u32 = 55;
    /// Offset of the `ATA` field.
    pub const ATA_SHIFT: u32 = 56;
    /// Offset of the `DCT` field.
    pub const DCT_SHIFT: u32 = 57;
    /// Offset of the `TID5` field.
    pub const TID5_SHIFT: u32 = 58;
    /// Offset of the `TWEDEn` field.
    pub const TWEDEN_SHIFT: u32 = 59;
    /// Offset of the `TWEDEL` field.
    pub const TWEDEL_SHIFT: u32 = 60;
    /// Mask for the `TWEDEL` field.
    pub const TWEDEL_MASK: u64 = 0b1111;

    /// Returns the value of the `BSU` field.
    pub const fn bsu(self) -> u8 {
        ((self.bits() >> Self::BSU_SHIFT) & Self::BSU_MASK) as u8
    }

    /// Sets the value of the `BSU` field.
    pub const fn set_bsu(&mut self, value: u8) {
        let offset = Self::BSU_SHIFT;
        assert!(value & (Self::BSU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BSU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BSU` field set to the given value.
    pub const fn with_bsu(mut self, value: u8) -> Self {
        self.set_bsu(value);
        self
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> Self::TWEDEL_SHIFT) & Self::TWEDEL_MASK) as u8
    }

    /// Sets the value of the `TWEDEL` field.
    pub const fn set_twedel(&mut self, value: u8) {
        let offset = Self::TWEDEL_SHIFT;
        assert!(value & (Self::TWEDEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWEDEL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TWEDEL` field set to the given value.
    pub const fn with_twedel(mut self, value: u8) -> Self {
        self.set_twedel(value);
        self
    }
}

bitflags! {
    /// `HDFGRTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdfgrtr2El2: u64 {
        /// `nPMECR_EL1` bit.
        const NPMECR_EL1 = 1 << 0;
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

impl Hdfgrtr2El2 {
    /// Offset of the `nPMECR_EL1` field.
    pub const NPMECR_EL1_SHIFT: u32 = 0;
    /// Offset of the `nPMICNTR_EL0` field.
    pub const NPMICNTR_EL0_SHIFT: u32 = 2;
    /// Offset of the `nPMICFILTR_EL0` field.
    pub const NPMICFILTR_EL0_SHIFT: u32 = 3;
    /// Offset of the `nPMUACR_EL1` field.
    pub const NPMUACR_EL1_SHIFT: u32 = 4;
    /// Offset of the `nMDSELR_EL1` field.
    pub const NMDSELR_EL1_SHIFT: u32 = 5;
    /// Offset of the `nPMSSDATA` field.
    pub const NPMSSDATA_SHIFT: u32 = 6;
    /// Offset of the `nPMSSCR_EL1` field.
    pub const NPMSSCR_EL1_SHIFT: u32 = 7;
    /// Offset of the `nSPMEVCNTRn_EL0` field.
    pub const NSPMEVCNTRN_EL0_SHIFT: u32 = 8;
    /// Offset of the `nSPMEVTYPERn_EL0` field.
    pub const NSPMEVTYPERN_EL0_SHIFT: u32 = 9;
    /// Offset of the `nSPMSELR_EL0` field.
    pub const NSPMSELR_EL0_SHIFT: u32 = 10;
    /// Offset of the `nSPMCNTEN` field.
    pub const NSPMCNTEN_SHIFT: u32 = 11;
    /// Offset of the `nSPMINTEN` field.
    pub const NSPMINTEN_SHIFT: u32 = 12;
    /// Offset of the `nSPMOVS` field.
    pub const NSPMOVS_SHIFT: u32 = 13;
    /// Offset of the `nSPMCR_EL0` field.
    pub const NSPMCR_EL0_SHIFT: u32 = 14;
    /// Offset of the `nSPMACCESSR_EL1` field.
    pub const NSPMACCESSR_EL1_SHIFT: u32 = 15;
    /// Offset of the `nSPMSCR_EL1` field.
    pub const NSPMSCR_EL1_SHIFT: u32 = 16;
    /// Offset of the `nSPMID` field.
    pub const NSPMID_SHIFT: u32 = 17;
    /// Offset of the `nSPMDEVAFF_EL1` field.
    pub const NSPMDEVAFF_EL1_SHIFT: u32 = 18;
    /// Offset of the `nPMSDSFR_EL1` field.
    pub const NPMSDSFR_EL1_SHIFT: u32 = 19;
    /// Offset of the `nTRCITECR_EL1` field.
    pub const NTRCITECR_EL1_SHIFT: u32 = 20;
    /// Offset of the `nTRBMPAM_EL1` field.
    pub const NTRBMPAM_EL1_SHIFT: u32 = 22;
    /// Offset of the `nMDSTEPOP_EL1` field.
    pub const NMDSTEPOP_EL1_SHIFT: u32 = 23;
    /// Offset of the `nPMBMAR_EL1` field.
    pub const NPMBMAR_EL1_SHIFT: u32 = 24;
}

bitflags! {
    /// `HDFGRTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HdfgrtrEl2: u64 {
        /// `DBGBCRn_EL1` bit.
        const DBGBCRN_EL1 = 1 << 0;
        /// `DBGBVRn_EL1` bit.
        const DBGBVRN_EL1 = 1 << 1;
        /// `DBGWCRn_EL1` bit.
        const DBGWCRN_EL1 = 1 << 2;
        /// `DBGWVRn_EL1` bit.
        const DBGWVRN_EL1 = 1 << 3;
        /// `MDSCR_EL1` bit.
        const MDSCR_EL1 = 1 << 4;
        /// `DBGCLAIM` bit.
        const DBGCLAIM = 1 << 5;
        /// `DBGAUTHSTATUS_EL1` bit.
        const DBGAUTHSTATUS_EL1 = 1 << 6;
        /// `DBGPRCR_EL1` bit.
        const DBGPRCR_EL1 = 1 << 7;
        /// `OSLSR_EL1` bit.
        const OSLSR_EL1 = 1 << 9;
        /// `OSECCR_EL1` bit.
        const OSECCR_EL1 = 1 << 10;
        /// `OSDLR_EL1` bit.
        const OSDLR_EL1 = 1 << 11;
        /// `PMEVCNTRn_EL0` bit.
        const PMEVCNTRN_EL0 = 1 << 12;
        /// `PMEVTYPERn_EL0` bit.
        const PMEVTYPERN_EL0 = 1 << 13;
        /// `PMCCFILTR_EL0` bit.
        const PMCCFILTR_EL0 = 1 << 14;
        /// `PMCCNTR_EL0` bit.
        const PMCCNTR_EL0 = 1 << 15;
        /// `PMCNTEN` bit.
        const PMCNTEN = 1 << 16;
        /// `PMINTEN` bit.
        const PMINTEN = 1 << 17;
        /// `PMOVS` bit.
        const PMOVS = 1 << 18;
        /// `PMSELR_EL0` bit.
        const PMSELR_EL0 = 1 << 19;
        /// `PMMIR_EL1` bit.
        const PMMIR_EL1 = 1 << 22;
        /// `PMBLIMITR_EL1` bit.
        const PMBLIMITR_EL1 = 1 << 23;
        /// `PMBPTR_EL1` bit.
        const PMBPTR_EL1 = 1 << 24;
        /// `PMBSR_EL1` bit.
        const PMBSR_EL1 = 1 << 25;
        /// `PMSCR_EL1` bit.
        const PMSCR_EL1 = 1 << 26;
        /// `PMSEVFR_EL1` bit.
        const PMSEVFR_EL1 = 1 << 27;
        /// `PMSFCR_EL1` bit.
        const PMSFCR_EL1 = 1 << 28;
        /// `PMSICR_EL1` bit.
        const PMSICR_EL1 = 1 << 29;
        /// `PMSIDR_EL1` bit.
        const PMSIDR_EL1 = 1 << 30;
        /// `PMSIRR_EL1` bit.
        const PMSIRR_EL1 = 1 << 31;
        /// `PMSLATFR_EL1` bit.
        const PMSLATFR_EL1 = 1 << 32;
        /// `TRC` bit.
        const TRC = 1 << 33;
        /// `TRCAUTHSTATUS` bit.
        const TRCAUTHSTATUS = 1 << 34;
        /// `TRCAUXCTLR` bit.
        const TRCAUXCTLR = 1 << 35;
        /// `TRCCLAIM` bit.
        const TRCCLAIM = 1 << 36;
        /// `TRCCNTVRn` bit.
        const TRCCNTVRN = 1 << 37;
        /// `TRCID` bit.
        const TRCID = 1 << 40;
        /// `TRCIMSPECn` bit.
        const TRCIMSPECN = 1 << 41;
        /// `TRCOSLSR` bit.
        const TRCOSLSR = 1 << 43;
        /// `TRCPRGCTLR` bit.
        const TRCPRGCTLR = 1 << 44;
        /// `TRCSEQSTR` bit.
        const TRCSEQSTR = 1 << 45;
        /// `TRCSSCSRn` bit.
        const TRCSSCSRN = 1 << 46;
        /// `TRCSTATR` bit.
        const TRCSTATR = 1 << 47;
        /// `TRCVICTLR` bit.
        const TRCVICTLR = 1 << 48;
        /// `TRBBASER_EL1` bit.
        const TRBBASER_EL1 = 1 << 50;
        /// `TRBIDR_EL1` bit.
        const TRBIDR_EL1 = 1 << 51;
        /// `TRBLIMITR_EL1` bit.
        const TRBLIMITR_EL1 = 1 << 52;
        /// `TRBMAR_EL1` bit.
        const TRBMAR_EL1 = 1 << 53;
        /// `TRBPTR_EL1` bit.
        const TRBPTR_EL1 = 1 << 54;
        /// `TRBSR_EL1` bit.
        const TRBSR_EL1 = 1 << 55;
        /// `TRBTRG_EL1` bit.
        const TRBTRG_EL1 = 1 << 56;
        /// `PMUSERENR_EL0` bit.
        const PMUSERENR_EL0 = 1 << 57;
        /// `PMCEIDn_EL0` bit.
        const PMCEIDN_EL0 = 1 << 58;
        /// `nBRBIDR` bit.
        const NBRBIDR = 1 << 59;
        /// `nBRBCTL` bit.
        const NBRBCTL = 1 << 60;
        /// `nBRBDATA` bit.
        const NBRBDATA = 1 << 61;
        /// `nPMSNEVFR_EL1` bit.
        const NPMSNEVFR_EL1 = 1 << 62;
        /// `PMBIDR_EL1` bit.
        const PMBIDR_EL1 = 1 << 63;
    }
}

impl HdfgrtrEl2 {
    /// Offset of the `DBGBCRn_EL1` field.
    pub const DBGBCRN_EL1_SHIFT: u32 = 0;
    /// Offset of the `DBGBVRn_EL1` field.
    pub const DBGBVRN_EL1_SHIFT: u32 = 1;
    /// Offset of the `DBGWCRn_EL1` field.
    pub const DBGWCRN_EL1_SHIFT: u32 = 2;
    /// Offset of the `DBGWVRn_EL1` field.
    pub const DBGWVRN_EL1_SHIFT: u32 = 3;
    /// Offset of the `MDSCR_EL1` field.
    pub const MDSCR_EL1_SHIFT: u32 = 4;
    /// Offset of the `DBGCLAIM` field.
    pub const DBGCLAIM_SHIFT: u32 = 5;
    /// Offset of the `DBGAUTHSTATUS_EL1` field.
    pub const DBGAUTHSTATUS_EL1_SHIFT: u32 = 6;
    /// Offset of the `DBGPRCR_EL1` field.
    pub const DBGPRCR_EL1_SHIFT: u32 = 7;
    /// Offset of the `OSLSR_EL1` field.
    pub const OSLSR_EL1_SHIFT: u32 = 9;
    /// Offset of the `OSECCR_EL1` field.
    pub const OSECCR_EL1_SHIFT: u32 = 10;
    /// Offset of the `OSDLR_EL1` field.
    pub const OSDLR_EL1_SHIFT: u32 = 11;
    /// Offset of the `PMEVCNTRn_EL0` field.
    pub const PMEVCNTRN_EL0_SHIFT: u32 = 12;
    /// Offset of the `PMEVTYPERn_EL0` field.
    pub const PMEVTYPERN_EL0_SHIFT: u32 = 13;
    /// Offset of the `PMCCFILTR_EL0` field.
    pub const PMCCFILTR_EL0_SHIFT: u32 = 14;
    /// Offset of the `PMCCNTR_EL0` field.
    pub const PMCCNTR_EL0_SHIFT: u32 = 15;
    /// Offset of the `PMCNTEN` field.
    pub const PMCNTEN_SHIFT: u32 = 16;
    /// Offset of the `PMINTEN` field.
    pub const PMINTEN_SHIFT: u32 = 17;
    /// Offset of the `PMOVS` field.
    pub const PMOVS_SHIFT: u32 = 18;
    /// Offset of the `PMSELR_EL0` field.
    pub const PMSELR_EL0_SHIFT: u32 = 19;
    /// Offset of the `PMMIR_EL1` field.
    pub const PMMIR_EL1_SHIFT: u32 = 22;
    /// Offset of the `PMBLIMITR_EL1` field.
    pub const PMBLIMITR_EL1_SHIFT: u32 = 23;
    /// Offset of the `PMBPTR_EL1` field.
    pub const PMBPTR_EL1_SHIFT: u32 = 24;
    /// Offset of the `PMBSR_EL1` field.
    pub const PMBSR_EL1_SHIFT: u32 = 25;
    /// Offset of the `PMSCR_EL1` field.
    pub const PMSCR_EL1_SHIFT: u32 = 26;
    /// Offset of the `PMSEVFR_EL1` field.
    pub const PMSEVFR_EL1_SHIFT: u32 = 27;
    /// Offset of the `PMSFCR_EL1` field.
    pub const PMSFCR_EL1_SHIFT: u32 = 28;
    /// Offset of the `PMSICR_EL1` field.
    pub const PMSICR_EL1_SHIFT: u32 = 29;
    /// Offset of the `PMSIDR_EL1` field.
    pub const PMSIDR_EL1_SHIFT: u32 = 30;
    /// Offset of the `PMSIRR_EL1` field.
    pub const PMSIRR_EL1_SHIFT: u32 = 31;
    /// Offset of the `PMSLATFR_EL1` field.
    pub const PMSLATFR_EL1_SHIFT: u32 = 32;
    /// Offset of the `TRC` field.
    pub const TRC_SHIFT: u32 = 33;
    /// Offset of the `TRCAUTHSTATUS` field.
    pub const TRCAUTHSTATUS_SHIFT: u32 = 34;
    /// Offset of the `TRCAUXCTLR` field.
    pub const TRCAUXCTLR_SHIFT: u32 = 35;
    /// Offset of the `TRCCLAIM` field.
    pub const TRCCLAIM_SHIFT: u32 = 36;
    /// Offset of the `TRCCNTVRn` field.
    pub const TRCCNTVRN_SHIFT: u32 = 37;
    /// Offset of the `TRCID` field.
    pub const TRCID_SHIFT: u32 = 40;
    /// Offset of the `TRCIMSPECn` field.
    pub const TRCIMSPECN_SHIFT: u32 = 41;
    /// Offset of the `TRCOSLSR` field.
    pub const TRCOSLSR_SHIFT: u32 = 43;
    /// Offset of the `TRCPRGCTLR` field.
    pub const TRCPRGCTLR_SHIFT: u32 = 44;
    /// Offset of the `TRCSEQSTR` field.
    pub const TRCSEQSTR_SHIFT: u32 = 45;
    /// Offset of the `TRCSSCSRn` field.
    pub const TRCSSCSRN_SHIFT: u32 = 46;
    /// Offset of the `TRCSTATR` field.
    pub const TRCSTATR_SHIFT: u32 = 47;
    /// Offset of the `TRCVICTLR` field.
    pub const TRCVICTLR_SHIFT: u32 = 48;
    /// Offset of the `TRBBASER_EL1` field.
    pub const TRBBASER_EL1_SHIFT: u32 = 50;
    /// Offset of the `TRBIDR_EL1` field.
    pub const TRBIDR_EL1_SHIFT: u32 = 51;
    /// Offset of the `TRBLIMITR_EL1` field.
    pub const TRBLIMITR_EL1_SHIFT: u32 = 52;
    /// Offset of the `TRBMAR_EL1` field.
    pub const TRBMAR_EL1_SHIFT: u32 = 53;
    /// Offset of the `TRBPTR_EL1` field.
    pub const TRBPTR_EL1_SHIFT: u32 = 54;
    /// Offset of the `TRBSR_EL1` field.
    pub const TRBSR_EL1_SHIFT: u32 = 55;
    /// Offset of the `TRBTRG_EL1` field.
    pub const TRBTRG_EL1_SHIFT: u32 = 56;
    /// Offset of the `PMUSERENR_EL0` field.
    pub const PMUSERENR_EL0_SHIFT: u32 = 57;
    /// Offset of the `PMCEIDn_EL0` field.
    pub const PMCEIDN_EL0_SHIFT: u32 = 58;
    /// Offset of the `nBRBIDR` field.
    pub const NBRBIDR_SHIFT: u32 = 59;
    /// Offset of the `nBRBCTL` field.
    pub const NBRBCTL_SHIFT: u32 = 60;
    /// Offset of the `nBRBDATA` field.
    pub const NBRBDATA_SHIFT: u32 = 61;
    /// Offset of the `nPMSNEVFR_EL1` field.
    pub const NPMSNEVFR_EL1_SHIFT: u32 = 62;
    /// Offset of the `PMBIDR_EL1` field.
    pub const PMBIDR_EL1_SHIFT: u32 = 63;
}

bitflags! {
    /// `HDFGWTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdfgwtr2El2: u64 {
        /// `nPMECR_EL1` bit.
        const NPMECR_EL1 = 1 << 0;
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

impl Hdfgwtr2El2 {
    /// Offset of the `nPMECR_EL1` field.
    pub const NPMECR_EL1_SHIFT: u32 = 0;
    /// Offset of the `nPMICNTR_EL0` field.
    pub const NPMICNTR_EL0_SHIFT: u32 = 2;
    /// Offset of the `nPMICFILTR_EL0` field.
    pub const NPMICFILTR_EL0_SHIFT: u32 = 3;
    /// Offset of the `nPMUACR_EL1` field.
    pub const NPMUACR_EL1_SHIFT: u32 = 4;
    /// Offset of the `nMDSELR_EL1` field.
    pub const NMDSELR_EL1_SHIFT: u32 = 5;
    /// Offset of the `nPMSSCR_EL1` field.
    pub const NPMSSCR_EL1_SHIFT: u32 = 7;
    /// Offset of the `nSPMEVCNTRn_EL0` field.
    pub const NSPMEVCNTRN_EL0_SHIFT: u32 = 8;
    /// Offset of the `nSPMEVTYPERn_EL0` field.
    pub const NSPMEVTYPERN_EL0_SHIFT: u32 = 9;
    /// Offset of the `nSPMSELR_EL0` field.
    pub const NSPMSELR_EL0_SHIFT: u32 = 10;
    /// Offset of the `nSPMCNTEN` field.
    pub const NSPMCNTEN_SHIFT: u32 = 11;
    /// Offset of the `nSPMINTEN` field.
    pub const NSPMINTEN_SHIFT: u32 = 12;
    /// Offset of the `nSPMOVS` field.
    pub const NSPMOVS_SHIFT: u32 = 13;
    /// Offset of the `nSPMCR_EL0` field.
    pub const NSPMCR_EL0_SHIFT: u32 = 14;
    /// Offset of the `nSPMACCESSR_EL1` field.
    pub const NSPMACCESSR_EL1_SHIFT: u32 = 15;
    /// Offset of the `nSPMSCR_EL1` field.
    pub const NSPMSCR_EL1_SHIFT: u32 = 16;
    /// Offset of the `nPMSDSFR_EL1` field.
    pub const NPMSDSFR_EL1_SHIFT: u32 = 19;
    /// Offset of the `nTRCITECR_EL1` field.
    pub const NTRCITECR_EL1_SHIFT: u32 = 20;
    /// Offset of the `nPMZR_EL0` field.
    pub const NPMZR_EL0_SHIFT: u32 = 21;
    /// Offset of the `nTRBMPAM_EL1` field.
    pub const NTRBMPAM_EL1_SHIFT: u32 = 22;
    /// Offset of the `nMDSTEPOP_EL1` field.
    pub const NMDSTEPOP_EL1_SHIFT: u32 = 23;
    /// Offset of the `nPMBMAR_EL1` field.
    pub const NPMBMAR_EL1_SHIFT: u32 = 24;
}

bitflags! {
    /// `HDFGWTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HdfgwtrEl2: u64 {
        /// `DBGBCRn_EL1` bit.
        const DBGBCRN_EL1 = 1 << 0;
        /// `DBGBVRn_EL1` bit.
        const DBGBVRN_EL1 = 1 << 1;
        /// `DBGWCRn_EL1` bit.
        const DBGWCRN_EL1 = 1 << 2;
        /// `DBGWVRn_EL1` bit.
        const DBGWVRN_EL1 = 1 << 3;
        /// `MDSCR_EL1` bit.
        const MDSCR_EL1 = 1 << 4;
        /// `DBGCLAIM` bit.
        const DBGCLAIM = 1 << 5;
        /// `DBGPRCR_EL1` bit.
        const DBGPRCR_EL1 = 1 << 7;
        /// `OSLAR_EL1` bit.
        const OSLAR_EL1 = 1 << 8;
        /// `OSECCR_EL1` bit.
        const OSECCR_EL1 = 1 << 10;
        /// `OSDLR_EL1` bit.
        const OSDLR_EL1 = 1 << 11;
        /// `PMEVCNTRn_EL0` bit.
        const PMEVCNTRN_EL0 = 1 << 12;
        /// `PMEVTYPERn_EL0` bit.
        const PMEVTYPERN_EL0 = 1 << 13;
        /// `PMCCFILTR_EL0` bit.
        const PMCCFILTR_EL0 = 1 << 14;
        /// `PMCCNTR_EL0` bit.
        const PMCCNTR_EL0 = 1 << 15;
        /// `PMCNTEN` bit.
        const PMCNTEN = 1 << 16;
        /// `PMINTEN` bit.
        const PMINTEN = 1 << 17;
        /// `PMOVS` bit.
        const PMOVS = 1 << 18;
        /// `PMSELR_EL0` bit.
        const PMSELR_EL0 = 1 << 19;
        /// `PMSWINC_EL0` bit.
        const PMSWINC_EL0 = 1 << 20;
        /// `PMCR_EL0` bit.
        const PMCR_EL0 = 1 << 21;
        /// `PMBLIMITR_EL1` bit.
        const PMBLIMITR_EL1 = 1 << 23;
        /// `PMBPTR_EL1` bit.
        const PMBPTR_EL1 = 1 << 24;
        /// `PMBSR_EL1` bit.
        const PMBSR_EL1 = 1 << 25;
        /// `PMSCR_EL1` bit.
        const PMSCR_EL1 = 1 << 26;
        /// `PMSEVFR_EL1` bit.
        const PMSEVFR_EL1 = 1 << 27;
        /// `PMSFCR_EL1` bit.
        const PMSFCR_EL1 = 1 << 28;
        /// `PMSICR_EL1` bit.
        const PMSICR_EL1 = 1 << 29;
        /// `PMSIRR_EL1` bit.
        const PMSIRR_EL1 = 1 << 31;
        /// `PMSLATFR_EL1` bit.
        const PMSLATFR_EL1 = 1 << 32;
        /// `TRC` bit.
        const TRC = 1 << 33;
        /// `TRCAUXCTLR` bit.
        const TRCAUXCTLR = 1 << 35;
        /// `TRCCLAIM` bit.
        const TRCCLAIM = 1 << 36;
        /// `TRCCNTVRn` bit.
        const TRCCNTVRN = 1 << 37;
        /// `TRCIMSPECn` bit.
        const TRCIMSPECN = 1 << 41;
        /// `TRCOSLAR` bit.
        const TRCOSLAR = 1 << 42;
        /// `TRCPRGCTLR` bit.
        const TRCPRGCTLR = 1 << 44;
        /// `TRCSEQSTR` bit.
        const TRCSEQSTR = 1 << 45;
        /// `TRCSSCSRn` bit.
        const TRCSSCSRN = 1 << 46;
        /// `TRCVICTLR` bit.
        const TRCVICTLR = 1 << 48;
        /// `TRFCR_EL1` bit.
        const TRFCR_EL1 = 1 << 49;
        /// `TRBBASER_EL1` bit.
        const TRBBASER_EL1 = 1 << 50;
        /// `TRBLIMITR_EL1` bit.
        const TRBLIMITR_EL1 = 1 << 52;
        /// `TRBMAR_EL1` bit.
        const TRBMAR_EL1 = 1 << 53;
        /// `TRBPTR_EL1` bit.
        const TRBPTR_EL1 = 1 << 54;
        /// `TRBSR_EL1` bit.
        const TRBSR_EL1 = 1 << 55;
        /// `TRBTRG_EL1` bit.
        const TRBTRG_EL1 = 1 << 56;
        /// `PMUSERENR_EL0` bit.
        const PMUSERENR_EL0 = 1 << 57;
        /// `nBRBCTL` bit.
        const NBRBCTL = 1 << 60;
        /// `nBRBDATA` bit.
        const NBRBDATA = 1 << 61;
        /// `nPMSNEVFR_EL1` bit.
        const NPMSNEVFR_EL1 = 1 << 62;
    }
}

impl HdfgwtrEl2 {
    /// Offset of the `DBGBCRn_EL1` field.
    pub const DBGBCRN_EL1_SHIFT: u32 = 0;
    /// Offset of the `DBGBVRn_EL1` field.
    pub const DBGBVRN_EL1_SHIFT: u32 = 1;
    /// Offset of the `DBGWCRn_EL1` field.
    pub const DBGWCRN_EL1_SHIFT: u32 = 2;
    /// Offset of the `DBGWVRn_EL1` field.
    pub const DBGWVRN_EL1_SHIFT: u32 = 3;
    /// Offset of the `MDSCR_EL1` field.
    pub const MDSCR_EL1_SHIFT: u32 = 4;
    /// Offset of the `DBGCLAIM` field.
    pub const DBGCLAIM_SHIFT: u32 = 5;
    /// Offset of the `DBGPRCR_EL1` field.
    pub const DBGPRCR_EL1_SHIFT: u32 = 7;
    /// Offset of the `OSLAR_EL1` field.
    pub const OSLAR_EL1_SHIFT: u32 = 8;
    /// Offset of the `OSECCR_EL1` field.
    pub const OSECCR_EL1_SHIFT: u32 = 10;
    /// Offset of the `OSDLR_EL1` field.
    pub const OSDLR_EL1_SHIFT: u32 = 11;
    /// Offset of the `PMEVCNTRn_EL0` field.
    pub const PMEVCNTRN_EL0_SHIFT: u32 = 12;
    /// Offset of the `PMEVTYPERn_EL0` field.
    pub const PMEVTYPERN_EL0_SHIFT: u32 = 13;
    /// Offset of the `PMCCFILTR_EL0` field.
    pub const PMCCFILTR_EL0_SHIFT: u32 = 14;
    /// Offset of the `PMCCNTR_EL0` field.
    pub const PMCCNTR_EL0_SHIFT: u32 = 15;
    /// Offset of the `PMCNTEN` field.
    pub const PMCNTEN_SHIFT: u32 = 16;
    /// Offset of the `PMINTEN` field.
    pub const PMINTEN_SHIFT: u32 = 17;
    /// Offset of the `PMOVS` field.
    pub const PMOVS_SHIFT: u32 = 18;
    /// Offset of the `PMSELR_EL0` field.
    pub const PMSELR_EL0_SHIFT: u32 = 19;
    /// Offset of the `PMSWINC_EL0` field.
    pub const PMSWINC_EL0_SHIFT: u32 = 20;
    /// Offset of the `PMCR_EL0` field.
    pub const PMCR_EL0_SHIFT: u32 = 21;
    /// Offset of the `PMBLIMITR_EL1` field.
    pub const PMBLIMITR_EL1_SHIFT: u32 = 23;
    /// Offset of the `PMBPTR_EL1` field.
    pub const PMBPTR_EL1_SHIFT: u32 = 24;
    /// Offset of the `PMBSR_EL1` field.
    pub const PMBSR_EL1_SHIFT: u32 = 25;
    /// Offset of the `PMSCR_EL1` field.
    pub const PMSCR_EL1_SHIFT: u32 = 26;
    /// Offset of the `PMSEVFR_EL1` field.
    pub const PMSEVFR_EL1_SHIFT: u32 = 27;
    /// Offset of the `PMSFCR_EL1` field.
    pub const PMSFCR_EL1_SHIFT: u32 = 28;
    /// Offset of the `PMSICR_EL1` field.
    pub const PMSICR_EL1_SHIFT: u32 = 29;
    /// Offset of the `PMSIRR_EL1` field.
    pub const PMSIRR_EL1_SHIFT: u32 = 31;
    /// Offset of the `PMSLATFR_EL1` field.
    pub const PMSLATFR_EL1_SHIFT: u32 = 32;
    /// Offset of the `TRC` field.
    pub const TRC_SHIFT: u32 = 33;
    /// Offset of the `TRCAUXCTLR` field.
    pub const TRCAUXCTLR_SHIFT: u32 = 35;
    /// Offset of the `TRCCLAIM` field.
    pub const TRCCLAIM_SHIFT: u32 = 36;
    /// Offset of the `TRCCNTVRn` field.
    pub const TRCCNTVRN_SHIFT: u32 = 37;
    /// Offset of the `TRCIMSPECn` field.
    pub const TRCIMSPECN_SHIFT: u32 = 41;
    /// Offset of the `TRCOSLAR` field.
    pub const TRCOSLAR_SHIFT: u32 = 42;
    /// Offset of the `TRCPRGCTLR` field.
    pub const TRCPRGCTLR_SHIFT: u32 = 44;
    /// Offset of the `TRCSEQSTR` field.
    pub const TRCSEQSTR_SHIFT: u32 = 45;
    /// Offset of the `TRCSSCSRn` field.
    pub const TRCSSCSRN_SHIFT: u32 = 46;
    /// Offset of the `TRCVICTLR` field.
    pub const TRCVICTLR_SHIFT: u32 = 48;
    /// Offset of the `TRFCR_EL1` field.
    pub const TRFCR_EL1_SHIFT: u32 = 49;
    /// Offset of the `TRBBASER_EL1` field.
    pub const TRBBASER_EL1_SHIFT: u32 = 50;
    /// Offset of the `TRBLIMITR_EL1` field.
    pub const TRBLIMITR_EL1_SHIFT: u32 = 52;
    /// Offset of the `TRBMAR_EL1` field.
    pub const TRBMAR_EL1_SHIFT: u32 = 53;
    /// Offset of the `TRBPTR_EL1` field.
    pub const TRBPTR_EL1_SHIFT: u32 = 54;
    /// Offset of the `TRBSR_EL1` field.
    pub const TRBSR_EL1_SHIFT: u32 = 55;
    /// Offset of the `TRBTRG_EL1` field.
    pub const TRBTRG_EL1_SHIFT: u32 = 56;
    /// Offset of the `PMUSERENR_EL0` field.
    pub const PMUSERENR_EL0_SHIFT: u32 = 57;
    /// Offset of the `nBRBCTL` field.
    pub const NBRBCTL_SHIFT: u32 = 60;
    /// Offset of the `nBRBDATA` field.
    pub const NBRBDATA_SHIFT: u32 = 61;
    /// Offset of the `nPMSNEVFR_EL1` field.
    pub const NPMSNEVFR_EL1_SHIFT: u32 = 62;
}

bitflags! {
    /// `HFGITR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl Hfgitr2El2 {
    /// Offset of the `TSBCSYNC` field.
    pub const TSBCSYNC_SHIFT: u32 = 0;
    /// Offset of the `nDCCIVAPS` field.
    pub const NDCCIVAPS_SHIFT: u32 = 1;
    /// Offset of the `PLBIPERME1OS` field.
    pub const PLBIPERME1OS_SHIFT: u32 = 2;
    /// Offset of the `PLBIASIDE1OS` field.
    pub const PLBIASIDE1OS_SHIFT: u32 = 3;
    /// Offset of the `PLBIVMALLE1OS` field.
    pub const PLBIVMALLE1OS_SHIFT: u32 = 4;
    /// Offset of the `PLBIPERME1IS` field.
    pub const PLBIPERME1IS_SHIFT: u32 = 5;
    /// Offset of the `PLBIASIDE1IS` field.
    pub const PLBIASIDE1IS_SHIFT: u32 = 6;
    /// Offset of the `PLBIVMALLE1IS` field.
    pub const PLBIVMALLE1IS_SHIFT: u32 = 7;
    /// Offset of the `PLBIPERME1` field.
    pub const PLBIPERME1_SHIFT: u32 = 8;
    /// Offset of the `PLBIASIDE1` field.
    pub const PLBIASIDE1_SHIFT: u32 = 9;
    /// Offset of the `PLBIVMALLE1` field.
    pub const PLBIVMALLE1_SHIFT: u32 = 10;
    /// Offset of the `PLBIPERMAE1OS` field.
    pub const PLBIPERMAE1OS_SHIFT: u32 = 11;
    /// Offset of the `PLBIPERMAE1IS` field.
    pub const PLBIPERMAE1IS_SHIFT: u32 = 12;
    /// Offset of the `PLBIPERMAE1` field.
    pub const PLBIPERMAE1_SHIFT: u32 = 13;
    /// Offset of the `DCGBVA` field.
    pub const DCGBVA_SHIFT: u32 = 14;
}

bitflags! {
    /// `HFGITR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HfgitrEl2: u64 {
        /// `ICIALLUIS` bit.
        const ICIALLUIS = 1 << 0;
        /// `ICIALLU` bit.
        const ICIALLU = 1 << 1;
        /// `ICIVAU` bit.
        const ICIVAU = 1 << 2;
        /// `DCIVAC` bit.
        const DCIVAC = 1 << 3;
        /// `DCISW` bit.
        const DCISW = 1 << 4;
        /// `DCCSW` bit.
        const DCCSW = 1 << 5;
        /// `DCCISW` bit.
        const DCCISW = 1 << 6;
        /// `DCCVAU` bit.
        const DCCVAU = 1 << 7;
        /// `DCCVAP` bit.
        const DCCVAP = 1 << 8;
        /// `DCCVADP` bit.
        const DCCVADP = 1 << 9;
        /// `DCCIVAC` bit.
        const DCCIVAC = 1 << 10;
        /// `DCZVA` bit.
        const DCZVA = 1 << 11;
        /// `ATS1E1R` bit.
        const ATS1E1R = 1 << 12;
        /// `ATS1E1W` bit.
        const ATS1E1W = 1 << 13;
        /// `ATS1E0R` bit.
        const ATS1E0R = 1 << 14;
        /// `ATS1E0W` bit.
        const ATS1E0W = 1 << 15;
        /// `ATS1E1RP` bit.
        const ATS1E1RP = 1 << 16;
        /// `ATS1E1WP` bit.
        const ATS1E1WP = 1 << 17;
        /// `TLBIVMALLE1OS` bit.
        const TLBIVMALLE1OS = 1 << 18;
        /// `TLBIVAE1OS` bit.
        const TLBIVAE1OS = 1 << 19;
        /// `TLBIASIDE1OS` bit.
        const TLBIASIDE1OS = 1 << 20;
        /// `TLBIVAAE1OS` bit.
        const TLBIVAAE1OS = 1 << 21;
        /// `TLBIVALE1OS` bit.
        const TLBIVALE1OS = 1 << 22;
        /// `TLBIVAALE1OS` bit.
        const TLBIVAALE1OS = 1 << 23;
        /// `TLBIRVAE1OS` bit.
        const TLBIRVAE1OS = 1 << 24;
        /// `TLBIRVAAE1OS` bit.
        const TLBIRVAAE1OS = 1 << 25;
        /// `TLBIRVALE1OS` bit.
        const TLBIRVALE1OS = 1 << 26;
        /// `TLBIRVAALE1OS` bit.
        const TLBIRVAALE1OS = 1 << 27;
        /// `TLBIVMALLE1IS` bit.
        const TLBIVMALLE1IS = 1 << 28;
        /// `TLBIVAE1IS` bit.
        const TLBIVAE1IS = 1 << 29;
        /// `TLBIASIDE1IS` bit.
        const TLBIASIDE1IS = 1 << 30;
        /// `TLBIVAAE1IS` bit.
        const TLBIVAAE1IS = 1 << 31;
        /// `TLBIVALE1IS` bit.
        const TLBIVALE1IS = 1 << 32;
        /// `TLBIVAALE1IS` bit.
        const TLBIVAALE1IS = 1 << 33;
        /// `TLBIRVAE1IS` bit.
        const TLBIRVAE1IS = 1 << 34;
        /// `TLBIRVAAE1IS` bit.
        const TLBIRVAAE1IS = 1 << 35;
        /// `TLBIRVALE1IS` bit.
        const TLBIRVALE1IS = 1 << 36;
        /// `TLBIRVAALE1IS` bit.
        const TLBIRVAALE1IS = 1 << 37;
        /// `TLBIRVAE1` bit.
        const TLBIRVAE1 = 1 << 38;
        /// `TLBIRVAAE1` bit.
        const TLBIRVAAE1 = 1 << 39;
        /// `TLBIRVALE1` bit.
        const TLBIRVALE1 = 1 << 40;
        /// `TLBIRVAALE1` bit.
        const TLBIRVAALE1 = 1 << 41;
        /// `TLBIVMALLE1` bit.
        const TLBIVMALLE1 = 1 << 42;
        /// `TLBIVAE1` bit.
        const TLBIVAE1 = 1 << 43;
        /// `TLBIASIDE1` bit.
        const TLBIASIDE1 = 1 << 44;
        /// `TLBIVAAE1` bit.
        const TLBIVAAE1 = 1 << 45;
        /// `TLBIVALE1` bit.
        const TLBIVALE1 = 1 << 46;
        /// `TLBIVAALE1` bit.
        const TLBIVAALE1 = 1 << 47;
        /// `CFPRCTX` bit.
        const CFPRCTX = 1 << 48;
        /// `DVPRCTX` bit.
        const DVPRCTX = 1 << 49;
        /// `CPPRCTX` bit.
        const CPPRCTX = 1 << 50;
        /// `ERET` bit.
        const ERET = 1 << 51;
        /// `SVC_EL0` bit.
        const SVC_EL0 = 1 << 52;
        /// `SVC_EL1` bit.
        const SVC_EL1 = 1 << 53;
        /// `DCCVAC` bit.
        const DCCVAC = 1 << 54;
        /// `nBRBINJ` bit.
        const NBRBINJ = 1 << 55;
        /// `nBRBIALL` bit.
        const NBRBIALL = 1 << 56;
        /// `nGCSPUSHM_EL1` bit.
        const NGCSPUSHM_EL1 = 1 << 57;
        /// `nGCSSTR_EL1` bit.
        const NGCSSTR_EL1 = 1 << 58;
        /// `nGCSEPP` bit.
        const NGCSEPP = 1 << 59;
        /// `COSPRCTX` bit.
        const COSPRCTX = 1 << 60;
        /// `ATS1E1A` bit.
        const ATS1E1A = 1 << 62;
        /// `PSBCSYNC` bit.
        const PSBCSYNC = 1 << 63;
    }
}

impl HfgitrEl2 {
    /// Offset of the `ICIALLUIS` field.
    pub const ICIALLUIS_SHIFT: u32 = 0;
    /// Offset of the `ICIALLU` field.
    pub const ICIALLU_SHIFT: u32 = 1;
    /// Offset of the `ICIVAU` field.
    pub const ICIVAU_SHIFT: u32 = 2;
    /// Offset of the `DCIVAC` field.
    pub const DCIVAC_SHIFT: u32 = 3;
    /// Offset of the `DCISW` field.
    pub const DCISW_SHIFT: u32 = 4;
    /// Offset of the `DCCSW` field.
    pub const DCCSW_SHIFT: u32 = 5;
    /// Offset of the `DCCISW` field.
    pub const DCCISW_SHIFT: u32 = 6;
    /// Offset of the `DCCVAU` field.
    pub const DCCVAU_SHIFT: u32 = 7;
    /// Offset of the `DCCVAP` field.
    pub const DCCVAP_SHIFT: u32 = 8;
    /// Offset of the `DCCVADP` field.
    pub const DCCVADP_SHIFT: u32 = 9;
    /// Offset of the `DCCIVAC` field.
    pub const DCCIVAC_SHIFT: u32 = 10;
    /// Offset of the `DCZVA` field.
    pub const DCZVA_SHIFT: u32 = 11;
    /// Offset of the `ATS1E1R` field.
    pub const ATS1E1R_SHIFT: u32 = 12;
    /// Offset of the `ATS1E1W` field.
    pub const ATS1E1W_SHIFT: u32 = 13;
    /// Offset of the `ATS1E0R` field.
    pub const ATS1E0R_SHIFT: u32 = 14;
    /// Offset of the `ATS1E0W` field.
    pub const ATS1E0W_SHIFT: u32 = 15;
    /// Offset of the `ATS1E1RP` field.
    pub const ATS1E1RP_SHIFT: u32 = 16;
    /// Offset of the `ATS1E1WP` field.
    pub const ATS1E1WP_SHIFT: u32 = 17;
    /// Offset of the `TLBIVMALLE1OS` field.
    pub const TLBIVMALLE1OS_SHIFT: u32 = 18;
    /// Offset of the `TLBIVAE1OS` field.
    pub const TLBIVAE1OS_SHIFT: u32 = 19;
    /// Offset of the `TLBIASIDE1OS` field.
    pub const TLBIASIDE1OS_SHIFT: u32 = 20;
    /// Offset of the `TLBIVAAE1OS` field.
    pub const TLBIVAAE1OS_SHIFT: u32 = 21;
    /// Offset of the `TLBIVALE1OS` field.
    pub const TLBIVALE1OS_SHIFT: u32 = 22;
    /// Offset of the `TLBIVAALE1OS` field.
    pub const TLBIVAALE1OS_SHIFT: u32 = 23;
    /// Offset of the `TLBIRVAE1OS` field.
    pub const TLBIRVAE1OS_SHIFT: u32 = 24;
    /// Offset of the `TLBIRVAAE1OS` field.
    pub const TLBIRVAAE1OS_SHIFT: u32 = 25;
    /// Offset of the `TLBIRVALE1OS` field.
    pub const TLBIRVALE1OS_SHIFT: u32 = 26;
    /// Offset of the `TLBIRVAALE1OS` field.
    pub const TLBIRVAALE1OS_SHIFT: u32 = 27;
    /// Offset of the `TLBIVMALLE1IS` field.
    pub const TLBIVMALLE1IS_SHIFT: u32 = 28;
    /// Offset of the `TLBIVAE1IS` field.
    pub const TLBIVAE1IS_SHIFT: u32 = 29;
    /// Offset of the `TLBIASIDE1IS` field.
    pub const TLBIASIDE1IS_SHIFT: u32 = 30;
    /// Offset of the `TLBIVAAE1IS` field.
    pub const TLBIVAAE1IS_SHIFT: u32 = 31;
    /// Offset of the `TLBIVALE1IS` field.
    pub const TLBIVALE1IS_SHIFT: u32 = 32;
    /// Offset of the `TLBIVAALE1IS` field.
    pub const TLBIVAALE1IS_SHIFT: u32 = 33;
    /// Offset of the `TLBIRVAE1IS` field.
    pub const TLBIRVAE1IS_SHIFT: u32 = 34;
    /// Offset of the `TLBIRVAAE1IS` field.
    pub const TLBIRVAAE1IS_SHIFT: u32 = 35;
    /// Offset of the `TLBIRVALE1IS` field.
    pub const TLBIRVALE1IS_SHIFT: u32 = 36;
    /// Offset of the `TLBIRVAALE1IS` field.
    pub const TLBIRVAALE1IS_SHIFT: u32 = 37;
    /// Offset of the `TLBIRVAE1` field.
    pub const TLBIRVAE1_SHIFT: u32 = 38;
    /// Offset of the `TLBIRVAAE1` field.
    pub const TLBIRVAAE1_SHIFT: u32 = 39;
    /// Offset of the `TLBIRVALE1` field.
    pub const TLBIRVALE1_SHIFT: u32 = 40;
    /// Offset of the `TLBIRVAALE1` field.
    pub const TLBIRVAALE1_SHIFT: u32 = 41;
    /// Offset of the `TLBIVMALLE1` field.
    pub const TLBIVMALLE1_SHIFT: u32 = 42;
    /// Offset of the `TLBIVAE1` field.
    pub const TLBIVAE1_SHIFT: u32 = 43;
    /// Offset of the `TLBIASIDE1` field.
    pub const TLBIASIDE1_SHIFT: u32 = 44;
    /// Offset of the `TLBIVAAE1` field.
    pub const TLBIVAAE1_SHIFT: u32 = 45;
    /// Offset of the `TLBIVALE1` field.
    pub const TLBIVALE1_SHIFT: u32 = 46;
    /// Offset of the `TLBIVAALE1` field.
    pub const TLBIVAALE1_SHIFT: u32 = 47;
    /// Offset of the `CFPRCTX` field.
    pub const CFPRCTX_SHIFT: u32 = 48;
    /// Offset of the `DVPRCTX` field.
    pub const DVPRCTX_SHIFT: u32 = 49;
    /// Offset of the `CPPRCTX` field.
    pub const CPPRCTX_SHIFT: u32 = 50;
    /// Offset of the `ERET` field.
    pub const ERET_SHIFT: u32 = 51;
    /// Offset of the `SVC_EL0` field.
    pub const SVC_EL0_SHIFT: u32 = 52;
    /// Offset of the `SVC_EL1` field.
    pub const SVC_EL1_SHIFT: u32 = 53;
    /// Offset of the `DCCVAC` field.
    pub const DCCVAC_SHIFT: u32 = 54;
    /// Offset of the `nBRBINJ` field.
    pub const NBRBINJ_SHIFT: u32 = 55;
    /// Offset of the `nBRBIALL` field.
    pub const NBRBIALL_SHIFT: u32 = 56;
    /// Offset of the `nGCSPUSHM_EL1` field.
    pub const NGCSPUSHM_EL1_SHIFT: u32 = 57;
    /// Offset of the `nGCSSTR_EL1` field.
    pub const NGCSSTR_EL1_SHIFT: u32 = 58;
    /// Offset of the `nGCSEPP` field.
    pub const NGCSEPP_SHIFT: u32 = 59;
    /// Offset of the `COSPRCTX` field.
    pub const COSPRCTX_SHIFT: u32 = 60;
    /// Offset of the `ATS1E1A` field.
    pub const ATS1E1A_SHIFT: u32 = 62;
    /// Offset of the `PSBCSYNC` field.
    pub const PSBCSYNC_SHIFT: u32 = 63;
}

bitflags! {
    /// `HFGRTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        /// `nLDSTT_EL1` bit.
        const NLDSTT_EL1 = 1 << 38;
        /// `ACTLR_EL1` bit.
        const ACTLR_EL1 = 1 << 39;
    }
}

impl Hfgrtr2El2 {
    /// Offset of the `nPFAR_EL1` field.
    pub const NPFAR_EL1_SHIFT: u32 = 0;
    /// Offset of the `nERXGSR_EL1` field.
    pub const NERXGSR_EL1_SHIFT: u32 = 1;
    /// Offset of the `nRCWSMASK_EL1` field.
    pub const NRCWSMASK_EL1_SHIFT: u32 = 2;
    /// Offset of the `nCPACRMASK_EL1` field.
    pub const NCPACRMASK_EL1_SHIFT: u32 = 3;
    /// Offset of the `nSCTLRMASK_EL1` field.
    pub const NSCTLRMASK_EL1_SHIFT: u32 = 4;
    /// Offset of the `nSCTLR2MASK_EL1` field.
    pub const NSCTLR2MASK_EL1_SHIFT: u32 = 5;
    /// Offset of the `nTCRMASK_EL1` field.
    pub const NTCRMASK_EL1_SHIFT: u32 = 6;
    /// Offset of the `nTCR2MASK_EL1` field.
    pub const NTCR2MASK_EL1_SHIFT: u32 = 7;
    /// Offset of the `nCPACRALIAS_EL1` field.
    pub const NCPACRALIAS_EL1_SHIFT: u32 = 8;
    /// Offset of the `nSCTLRALIAS_EL1` field.
    pub const NSCTLRALIAS_EL1_SHIFT: u32 = 9;
    /// Offset of the `nSCTLR2ALIAS_EL1` field.
    pub const NSCTLR2ALIAS_EL1_SHIFT: u32 = 10;
    /// Offset of the `nTCRALIAS_EL1` field.
    pub const NTCRALIAS_EL1_SHIFT: u32 = 11;
    /// Offset of the `nTCR2ALIAS_EL1` field.
    pub const NTCR2ALIAS_EL1_SHIFT: u32 = 12;
    /// Offset of the `nACTLRMASK_EL1` field.
    pub const NACTLRMASK_EL1_SHIFT: u32 = 13;
    /// Offset of the `nACTLRALIAS_EL1` field.
    pub const NACTLRALIAS_EL1_SHIFT: u32 = 14;
    /// Offset of the `nTINDEX_EL0` field.
    pub const NTINDEX_EL0_SHIFT: u32 = 15;
    /// Offset of the `nTINDEX_EL1` field.
    pub const NTINDEX_EL1_SHIFT: u32 = 16;
    /// Offset of the `nSTINDEX_EL1` field.
    pub const NSTINDEX_EL1_SHIFT: u32 = 17;
    /// Offset of the `nFGDTn_EL1` field.
    pub const NFGDTN_EL1_SHIFT: u32 = 18;
    /// Mask for the `nFGDTn_EL1` field.
    pub const NFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the `nTTTBRP_EL1` field.
    pub const NTTTBRP_EL1_SHIFT: u32 = 20;
    /// Offset of the `nTTTBRU_EL1` field.
    pub const NTTTBRU_EL1_SHIFT: u32 = 21;
    /// Offset of the `nIRTBRP_EL1` field.
    pub const NIRTBRP_EL1_SHIFT: u32 = 22;
    /// Offset of the `nIRTBRU_EL1` field.
    pub const NIRTBRU_EL1_SHIFT: u32 = 23;
    /// Offset of the `nDPOTBR1_EL1` field.
    pub const NDPOTBR1_EL1_SHIFT: u32 = 24;
    /// Offset of the `nDPOTBR0_EL1` field.
    pub const NDPOTBR0_EL1_SHIFT: u32 = 25;
    /// Offset of the `nTPMIN1_EL1` field.
    pub const NTPMIN1_EL1_SHIFT: u32 = 26;
    /// Offset of the `nTPMIN0_EL1` field.
    pub const NTPMIN0_EL1_SHIFT: u32 = 27;
    /// Offset of the `nTPMIN1_EL0` field.
    pub const NTPMIN1_EL0_SHIFT: u32 = 28;
    /// Offset of the `nTPMIN0_EL0` field.
    pub const NTPMIN0_EL0_SHIFT: u32 = 29;
    /// Offset of the `nTLBIDIDR_EL1` field.
    pub const NTLBIDIDR_EL1_SHIFT: u32 = 30;
    /// Offset of the `nAFGDTn_EL1` field.
    pub const NAFGDTN_EL1_SHIFT: u32 = 31;
    /// Mask for the `nAFGDTn_EL1` field.
    pub const NAFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the `TFSR_EL1` field.
    pub const TFSR_EL1_SHIFT: u32 = 33;
    /// Offset of the `RGSR_EL1` field.
    pub const RGSR_EL1_SHIFT: u32 = 34;
    /// Offset of the `GCR_EL1` field.
    pub const GCR_EL1_SHIFT: u32 = 35;
    /// Offset of the `nTPIDR3_EL0` field.
    pub const NTPIDR3_EL0_SHIFT: u32 = 36;
    /// Offset of the `nTPIDR3_EL1` field.
    pub const NTPIDR3_EL1_SHIFT: u32 = 37;
    /// Offset of the `nLDSTT_EL1` field.
    pub const NLDSTT_EL1_SHIFT: u32 = 38;
    /// Offset of the `ACTLR_EL1` field.
    pub const ACTLR_EL1_SHIFT: u32 = 39;

    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NFGDTN_EL1_SHIFT) & Self::NFGDTN_EL1_MASK) as u8
    }

    /// Sets the value of the `nFGDTn_EL1` field.
    pub const fn set_nfgdtn_el1(&mut self, value: u8) {
        let offset = Self::NFGDTN_EL1_SHIFT;
        assert!(value & (Self::NFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `nFGDTn_EL1` field set to the given value.
    pub const fn with_nfgdtn_el1(mut self, value: u8) -> Self {
        self.set_nfgdtn_el1(value);
        self
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NAFGDTN_EL1_SHIFT) & Self::NAFGDTN_EL1_MASK) as u8
    }

    /// Sets the value of the `nAFGDTn_EL1` field.
    pub const fn set_nafgdtn_el1(&mut self, value: u8) {
        let offset = Self::NAFGDTN_EL1_SHIFT;
        assert!(value & (Self::NAFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NAFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `nAFGDTn_EL1` field set to the given value.
    pub const fn with_nafgdtn_el1(mut self, value: u8) -> Self {
        self.set_nafgdtn_el1(value);
        self
    }
}

bitflags! {
    /// `HFGRTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HfgrtrEl2: u64 {
        /// `AFSR0_EL1` bit.
        const AFSR0_EL1 = 1 << 0;
        /// `AFSR1_EL1` bit.
        const AFSR1_EL1 = 1 << 1;
        /// `AIDR_EL1` bit.
        const AIDR_EL1 = 1 << 2;
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
        /// `CCSIDR_EL1` bit.
        const CCSIDR_EL1 = 1 << 9;
        /// `CLIDR_EL1` bit.
        const CLIDR_EL1 = 1 << 10;
        /// `CONTEXTIDR_EL1` bit.
        const CONTEXTIDR_EL1 = 1 << 11;
        /// `CPACR_EL1` bit.
        const CPACR_EL1 = 1 << 12;
        /// `CSSELR_EL1` bit.
        const CSSELR_EL1 = 1 << 13;
        /// `CTR_EL0` bit.
        const CTR_EL0 = 1 << 14;
        /// `DCZID_EL0` bit.
        const DCZID_EL0 = 1 << 15;
        /// `ESR_EL1` bit.
        const ESR_EL1 = 1 << 16;
        /// `FAR_EL1` bit.
        const FAR_EL1 = 1 << 17;
        /// `ISR_EL1` bit.
        const ISR_EL1 = 1 << 18;
        /// `LORC_EL1` bit.
        const LORC_EL1 = 1 << 19;
        /// `LOREA_EL1` bit.
        const LOREA_EL1 = 1 << 20;
        /// `LORID_EL1` bit.
        const LORID_EL1 = 1 << 21;
        /// `LORN_EL1` bit.
        const LORN_EL1 = 1 << 22;
        /// `LORSA_EL1` bit.
        const LORSA_EL1 = 1 << 23;
        /// `MAIR_EL1` bit.
        const MAIR_EL1 = 1 << 24;
        /// `MIDR_EL1` bit.
        const MIDR_EL1 = 1 << 25;
        /// `MPIDR_EL1` bit.
        const MPIDR_EL1 = 1 << 26;
        /// `PAR_EL1` bit.
        const PAR_EL1 = 1 << 27;
        /// `REVIDR_EL1` bit.
        const REVIDR_EL1 = 1 << 28;
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
        /// `ERRIDR_EL1` bit.
        const ERRIDR_EL1 = 1 << 40;
        /// `ERRSELR_EL1` bit.
        const ERRSELR_EL1 = 1 << 41;
        /// `ERXFR_EL1` bit.
        const ERXFR_EL1 = 1 << 42;
        /// `ERXCTLR_EL1` bit.
        const ERXCTLR_EL1 = 1 << 43;
        /// `ERXSTATUS_EL1` bit.
        const ERXSTATUS_EL1 = 1 << 44;
        /// `ERXMISCn_EL1` bit.
        const ERXMISCN_EL1 = 1 << 45;
        /// `ERXPFGF_EL1` bit.
        const ERXPFGF_EL1 = 1 << 46;
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

impl HfgrtrEl2 {
    /// Offset of the `AFSR0_EL1` field.
    pub const AFSR0_EL1_SHIFT: u32 = 0;
    /// Offset of the `AFSR1_EL1` field.
    pub const AFSR1_EL1_SHIFT: u32 = 1;
    /// Offset of the `AIDR_EL1` field.
    pub const AIDR_EL1_SHIFT: u32 = 2;
    /// Offset of the `AMAIR_EL1` field.
    pub const AMAIR_EL1_SHIFT: u32 = 3;
    /// Offset of the `APDAKey` field.
    pub const APDAKEY_SHIFT: u32 = 4;
    /// Offset of the `APDBKey` field.
    pub const APDBKEY_SHIFT: u32 = 5;
    /// Offset of the `APGAKey` field.
    pub const APGAKEY_SHIFT: u32 = 6;
    /// Offset of the `APIAKey` field.
    pub const APIAKEY_SHIFT: u32 = 7;
    /// Offset of the `APIBKey` field.
    pub const APIBKEY_SHIFT: u32 = 8;
    /// Offset of the `CCSIDR_EL1` field.
    pub const CCSIDR_EL1_SHIFT: u32 = 9;
    /// Offset of the `CLIDR_EL1` field.
    pub const CLIDR_EL1_SHIFT: u32 = 10;
    /// Offset of the `CONTEXTIDR_EL1` field.
    pub const CONTEXTIDR_EL1_SHIFT: u32 = 11;
    /// Offset of the `CPACR_EL1` field.
    pub const CPACR_EL1_SHIFT: u32 = 12;
    /// Offset of the `CSSELR_EL1` field.
    pub const CSSELR_EL1_SHIFT: u32 = 13;
    /// Offset of the `CTR_EL0` field.
    pub const CTR_EL0_SHIFT: u32 = 14;
    /// Offset of the `DCZID_EL0` field.
    pub const DCZID_EL0_SHIFT: u32 = 15;
    /// Offset of the `ESR_EL1` field.
    pub const ESR_EL1_SHIFT: u32 = 16;
    /// Offset of the `FAR_EL1` field.
    pub const FAR_EL1_SHIFT: u32 = 17;
    /// Offset of the `ISR_EL1` field.
    pub const ISR_EL1_SHIFT: u32 = 18;
    /// Offset of the `LORC_EL1` field.
    pub const LORC_EL1_SHIFT: u32 = 19;
    /// Offset of the `LOREA_EL1` field.
    pub const LOREA_EL1_SHIFT: u32 = 20;
    /// Offset of the `LORID_EL1` field.
    pub const LORID_EL1_SHIFT: u32 = 21;
    /// Offset of the `LORN_EL1` field.
    pub const LORN_EL1_SHIFT: u32 = 22;
    /// Offset of the `LORSA_EL1` field.
    pub const LORSA_EL1_SHIFT: u32 = 23;
    /// Offset of the `MAIR_EL1` field.
    pub const MAIR_EL1_SHIFT: u32 = 24;
    /// Offset of the `MIDR_EL1` field.
    pub const MIDR_EL1_SHIFT: u32 = 25;
    /// Offset of the `MPIDR_EL1` field.
    pub const MPIDR_EL1_SHIFT: u32 = 26;
    /// Offset of the `PAR_EL1` field.
    pub const PAR_EL1_SHIFT: u32 = 27;
    /// Offset of the `REVIDR_EL1` field.
    pub const REVIDR_EL1_SHIFT: u32 = 28;
    /// Offset of the `SCTLR_EL1` field.
    pub const SCTLR_EL1_SHIFT: u32 = 29;
    /// Offset of the `SCXTNUM_EL1` field.
    pub const SCXTNUM_EL1_SHIFT: u32 = 30;
    /// Offset of the `SCXTNUM_EL0` field.
    pub const SCXTNUM_EL0_SHIFT: u32 = 31;
    /// Offset of the `TCR_EL1` field.
    pub const TCR_EL1_SHIFT: u32 = 32;
    /// Offset of the `TPIDR_EL1` field.
    pub const TPIDR_EL1_SHIFT: u32 = 33;
    /// Offset of the `TPIDRRO_EL0` field.
    pub const TPIDRRO_EL0_SHIFT: u32 = 34;
    /// Offset of the `TPIDR_EL0` field.
    pub const TPIDR_EL0_SHIFT: u32 = 35;
    /// Offset of the `TTBR0_EL1` field.
    pub const TTBR0_EL1_SHIFT: u32 = 36;
    /// Offset of the `TTBR1_EL1` field.
    pub const TTBR1_EL1_SHIFT: u32 = 37;
    /// Offset of the `VBAR_EL1` field.
    pub const VBAR_EL1_SHIFT: u32 = 38;
    /// Offset of the `ICC_IGRPENn_EL1` field.
    pub const ICC_IGRPENN_EL1_SHIFT: u32 = 39;
    /// Offset of the `ERRIDR_EL1` field.
    pub const ERRIDR_EL1_SHIFT: u32 = 40;
    /// Offset of the `ERRSELR_EL1` field.
    pub const ERRSELR_EL1_SHIFT: u32 = 41;
    /// Offset of the `ERXFR_EL1` field.
    pub const ERXFR_EL1_SHIFT: u32 = 42;
    /// Offset of the `ERXCTLR_EL1` field.
    pub const ERXCTLR_EL1_SHIFT: u32 = 43;
    /// Offset of the `ERXSTATUS_EL1` field.
    pub const ERXSTATUS_EL1_SHIFT: u32 = 44;
    /// Offset of the `ERXMISCn_EL1` field.
    pub const ERXMISCN_EL1_SHIFT: u32 = 45;
    /// Offset of the `ERXPFGF_EL1` field.
    pub const ERXPFGF_EL1_SHIFT: u32 = 46;
    /// Offset of the `ERXPFGCTL_EL1` field.
    pub const ERXPFGCTL_EL1_SHIFT: u32 = 47;
    /// Offset of the `ERXPFGCDN_EL1` field.
    pub const ERXPFGCDN_EL1_SHIFT: u32 = 48;
    /// Offset of the `ERXADDR_EL1` field.
    pub const ERXADDR_EL1_SHIFT: u32 = 49;
    /// Offset of the `nACCDATA_EL1` field.
    pub const NACCDATA_EL1_SHIFT: u32 = 50;
    /// Offset of the `nGCS_EL0` field.
    pub const NGCS_EL0_SHIFT: u32 = 52;
    /// Offset of the `nGCS_EL1` field.
    pub const NGCS_EL1_SHIFT: u32 = 53;
    /// Offset of the `nSMPRI_EL1` field.
    pub const NSMPRI_EL1_SHIFT: u32 = 54;
    /// Offset of the `nTPIDR2_EL0` field.
    pub const NTPIDR2_EL0_SHIFT: u32 = 55;
    /// Offset of the `nRCWMASK_EL1` field.
    pub const NRCWMASK_EL1_SHIFT: u32 = 56;
    /// Offset of the `nPIRE0_EL1` field.
    pub const NPIRE0_EL1_SHIFT: u32 = 57;
    /// Offset of the `nPIR_EL1` field.
    pub const NPIR_EL1_SHIFT: u32 = 58;
    /// Offset of the `nPOR_EL0` field.
    pub const NPOR_EL0_SHIFT: u32 = 59;
    /// Offset of the `nPOR_EL1` field.
    pub const NPOR_EL1_SHIFT: u32 = 60;
    /// Offset of the `nS2POR_EL1` field.
    pub const NS2POR_EL1_SHIFT: u32 = 61;
    /// Offset of the `nMAIR2_EL1` field.
    pub const NMAIR2_EL1_SHIFT: u32 = 62;
    /// Offset of the `nAMAIR2_EL1` field.
    pub const NAMAIR2_EL1_SHIFT: u32 = 63;
}

bitflags! {
    /// `HFGWTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        /// `nLDSTT_EL1` bit.
        const NLDSTT_EL1 = 1 << 38;
        /// `ACTLR_EL1` bit.
        const ACTLR_EL1 = 1 << 39;
    }
}

impl Hfgwtr2El2 {
    /// Offset of the `nPFAR_EL1` field.
    pub const NPFAR_EL1_SHIFT: u32 = 0;
    /// Offset of the `nRCWSMASK_EL1` field.
    pub const NRCWSMASK_EL1_SHIFT: u32 = 2;
    /// Offset of the `nCPACRMASK_EL1` field.
    pub const NCPACRMASK_EL1_SHIFT: u32 = 3;
    /// Offset of the `nSCTLRMASK_EL1` field.
    pub const NSCTLRMASK_EL1_SHIFT: u32 = 4;
    /// Offset of the `nSCTLR2MASK_EL1` field.
    pub const NSCTLR2MASK_EL1_SHIFT: u32 = 5;
    /// Offset of the `nTCRMASK_EL1` field.
    pub const NTCRMASK_EL1_SHIFT: u32 = 6;
    /// Offset of the `nTCR2MASK_EL1` field.
    pub const NTCR2MASK_EL1_SHIFT: u32 = 7;
    /// Offset of the `nCPACRALIAS_EL1` field.
    pub const NCPACRALIAS_EL1_SHIFT: u32 = 8;
    /// Offset of the `nSCTLRALIAS_EL1` field.
    pub const NSCTLRALIAS_EL1_SHIFT: u32 = 9;
    /// Offset of the `nSCTLR2ALIAS_EL1` field.
    pub const NSCTLR2ALIAS_EL1_SHIFT: u32 = 10;
    /// Offset of the `nTCRALIAS_EL1` field.
    pub const NTCRALIAS_EL1_SHIFT: u32 = 11;
    /// Offset of the `nTCR2ALIAS_EL1` field.
    pub const NTCR2ALIAS_EL1_SHIFT: u32 = 12;
    /// Offset of the `nACTLRMASK_EL1` field.
    pub const NACTLRMASK_EL1_SHIFT: u32 = 13;
    /// Offset of the `nACTLRALIAS_EL1` field.
    pub const NACTLRALIAS_EL1_SHIFT: u32 = 14;
    /// Offset of the `nTINDEX_EL0` field.
    pub const NTINDEX_EL0_SHIFT: u32 = 15;
    /// Offset of the `nTINDEX_EL1` field.
    pub const NTINDEX_EL1_SHIFT: u32 = 16;
    /// Offset of the `nSTINDEX_EL1` field.
    pub const NSTINDEX_EL1_SHIFT: u32 = 17;
    /// Offset of the `nFGDTn_EL1` field.
    pub const NFGDTN_EL1_SHIFT: u32 = 18;
    /// Mask for the `nFGDTn_EL1` field.
    pub const NFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the `nTTTBRP_EL1` field.
    pub const NTTTBRP_EL1_SHIFT: u32 = 20;
    /// Offset of the `nTTTBRU_EL1` field.
    pub const NTTTBRU_EL1_SHIFT: u32 = 21;
    /// Offset of the `nIRTBRP_EL1` field.
    pub const NIRTBRP_EL1_SHIFT: u32 = 22;
    /// Offset of the `nIRTBRU_EL1` field.
    pub const NIRTBRU_EL1_SHIFT: u32 = 23;
    /// Offset of the `nDPOTBR1_EL1` field.
    pub const NDPOTBR1_EL1_SHIFT: u32 = 24;
    /// Offset of the `nDPOTBR0_EL1` field.
    pub const NDPOTBR0_EL1_SHIFT: u32 = 25;
    /// Offset of the `nTPMIN1_EL1` field.
    pub const NTPMIN1_EL1_SHIFT: u32 = 26;
    /// Offset of the `nTPMIN0_EL1` field.
    pub const NTPMIN0_EL1_SHIFT: u32 = 27;
    /// Offset of the `nTPMIN1_EL0` field.
    pub const NTPMIN1_EL0_SHIFT: u32 = 28;
    /// Offset of the `nTPMIN0_EL0` field.
    pub const NTPMIN0_EL0_SHIFT: u32 = 29;
    /// Offset of the `nAFGDTn_EL1` field.
    pub const NAFGDTN_EL1_SHIFT: u32 = 31;
    /// Mask for the `nAFGDTn_EL1` field.
    pub const NAFGDTN_EL1_MASK: u64 = 0b11;
    /// Offset of the `TFSR_EL1` field.
    pub const TFSR_EL1_SHIFT: u32 = 33;
    /// Offset of the `RGSR_EL1` field.
    pub const RGSR_EL1_SHIFT: u32 = 34;
    /// Offset of the `GCR_EL1` field.
    pub const GCR_EL1_SHIFT: u32 = 35;
    /// Offset of the `nTPIDR3_EL0` field.
    pub const NTPIDR3_EL0_SHIFT: u32 = 36;
    /// Offset of the `nTPIDR3_EL1` field.
    pub const NTPIDR3_EL1_SHIFT: u32 = 37;
    /// Offset of the `nLDSTT_EL1` field.
    pub const NLDSTT_EL1_SHIFT: u32 = 38;
    /// Offset of the `ACTLR_EL1` field.
    pub const ACTLR_EL1_SHIFT: u32 = 39;

    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NFGDTN_EL1_SHIFT) & Self::NFGDTN_EL1_MASK) as u8
    }

    /// Sets the value of the `nFGDTn_EL1` field.
    pub const fn set_nfgdtn_el1(&mut self, value: u8) {
        let offset = Self::NFGDTN_EL1_SHIFT;
        assert!(value & (Self::NFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `nFGDTn_EL1` field set to the given value.
    pub const fn with_nfgdtn_el1(mut self, value: u8) -> Self {
        self.set_nfgdtn_el1(value);
        self
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NAFGDTN_EL1_SHIFT) & Self::NAFGDTN_EL1_MASK) as u8
    }

    /// Sets the value of the `nAFGDTn_EL1` field.
    pub const fn set_nafgdtn_el1(&mut self, value: u8) {
        let offset = Self::NAFGDTN_EL1_SHIFT;
        assert!(value & (Self::NAFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NAFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `nAFGDTn_EL1` field set to the given value.
    pub const fn with_nafgdtn_el1(mut self, value: u8) -> Self {
        self.set_nafgdtn_el1(value);
        self
    }
}

bitflags! {
    /// `HFGWTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl HfgwtrEl2 {
    /// Offset of the `AFSR0_EL1` field.
    pub const AFSR0_EL1_SHIFT: u32 = 0;
    /// Offset of the `AFSR1_EL1` field.
    pub const AFSR1_EL1_SHIFT: u32 = 1;
    /// Offset of the `AMAIR_EL1` field.
    pub const AMAIR_EL1_SHIFT: u32 = 3;
    /// Offset of the `APDAKey` field.
    pub const APDAKEY_SHIFT: u32 = 4;
    /// Offset of the `APDBKey` field.
    pub const APDBKEY_SHIFT: u32 = 5;
    /// Offset of the `APGAKey` field.
    pub const APGAKEY_SHIFT: u32 = 6;
    /// Offset of the `APIAKey` field.
    pub const APIAKEY_SHIFT: u32 = 7;
    /// Offset of the `APIBKey` field.
    pub const APIBKEY_SHIFT: u32 = 8;
    /// Offset of the `CONTEXTIDR_EL1` field.
    pub const CONTEXTIDR_EL1_SHIFT: u32 = 11;
    /// Offset of the `CPACR_EL1` field.
    pub const CPACR_EL1_SHIFT: u32 = 12;
    /// Offset of the `CSSELR_EL1` field.
    pub const CSSELR_EL1_SHIFT: u32 = 13;
    /// Offset of the `ESR_EL1` field.
    pub const ESR_EL1_SHIFT: u32 = 16;
    /// Offset of the `FAR_EL1` field.
    pub const FAR_EL1_SHIFT: u32 = 17;
    /// Offset of the `LORC_EL1` field.
    pub const LORC_EL1_SHIFT: u32 = 19;
    /// Offset of the `LOREA_EL1` field.
    pub const LOREA_EL1_SHIFT: u32 = 20;
    /// Offset of the `LORN_EL1` field.
    pub const LORN_EL1_SHIFT: u32 = 22;
    /// Offset of the `LORSA_EL1` field.
    pub const LORSA_EL1_SHIFT: u32 = 23;
    /// Offset of the `MAIR_EL1` field.
    pub const MAIR_EL1_SHIFT: u32 = 24;
    /// Offset of the `PAR_EL1` field.
    pub const PAR_EL1_SHIFT: u32 = 27;
    /// Offset of the `SCTLR_EL1` field.
    pub const SCTLR_EL1_SHIFT: u32 = 29;
    /// Offset of the `SCXTNUM_EL1` field.
    pub const SCXTNUM_EL1_SHIFT: u32 = 30;
    /// Offset of the `SCXTNUM_EL0` field.
    pub const SCXTNUM_EL0_SHIFT: u32 = 31;
    /// Offset of the `TCR_EL1` field.
    pub const TCR_EL1_SHIFT: u32 = 32;
    /// Offset of the `TPIDR_EL1` field.
    pub const TPIDR_EL1_SHIFT: u32 = 33;
    /// Offset of the `TPIDRRO_EL0` field.
    pub const TPIDRRO_EL0_SHIFT: u32 = 34;
    /// Offset of the `TPIDR_EL0` field.
    pub const TPIDR_EL0_SHIFT: u32 = 35;
    /// Offset of the `TTBR0_EL1` field.
    pub const TTBR0_EL1_SHIFT: u32 = 36;
    /// Offset of the `TTBR1_EL1` field.
    pub const TTBR1_EL1_SHIFT: u32 = 37;
    /// Offset of the `VBAR_EL1` field.
    pub const VBAR_EL1_SHIFT: u32 = 38;
    /// Offset of the `ICC_IGRPENn_EL1` field.
    pub const ICC_IGRPENN_EL1_SHIFT: u32 = 39;
    /// Offset of the `ERRSELR_EL1` field.
    pub const ERRSELR_EL1_SHIFT: u32 = 41;
    /// Offset of the `ERXCTLR_EL1` field.
    pub const ERXCTLR_EL1_SHIFT: u32 = 43;
    /// Offset of the `ERXSTATUS_EL1` field.
    pub const ERXSTATUS_EL1_SHIFT: u32 = 44;
    /// Offset of the `ERXMISCn_EL1` field.
    pub const ERXMISCN_EL1_SHIFT: u32 = 45;
    /// Offset of the `ERXPFGCTL_EL1` field.
    pub const ERXPFGCTL_EL1_SHIFT: u32 = 47;
    /// Offset of the `ERXPFGCDN_EL1` field.
    pub const ERXPFGCDN_EL1_SHIFT: u32 = 48;
    /// Offset of the `ERXADDR_EL1` field.
    pub const ERXADDR_EL1_SHIFT: u32 = 49;
    /// Offset of the `nACCDATA_EL1` field.
    pub const NACCDATA_EL1_SHIFT: u32 = 50;
    /// Offset of the `nGCS_EL0` field.
    pub const NGCS_EL0_SHIFT: u32 = 52;
    /// Offset of the `nGCS_EL1` field.
    pub const NGCS_EL1_SHIFT: u32 = 53;
    /// Offset of the `nSMPRI_EL1` field.
    pub const NSMPRI_EL1_SHIFT: u32 = 54;
    /// Offset of the `nTPIDR2_EL0` field.
    pub const NTPIDR2_EL0_SHIFT: u32 = 55;
    /// Offset of the `nRCWMASK_EL1` field.
    pub const NRCWMASK_EL1_SHIFT: u32 = 56;
    /// Offset of the `nPIRE0_EL1` field.
    pub const NPIRE0_EL1_SHIFT: u32 = 57;
    /// Offset of the `nPIR_EL1` field.
    pub const NPIR_EL1_SHIFT: u32 = 58;
    /// Offset of the `nPOR_EL0` field.
    pub const NPOR_EL0_SHIFT: u32 = 59;
    /// Offset of the `nPOR_EL1` field.
    pub const NPOR_EL1_SHIFT: u32 = 60;
    /// Offset of the `nS2POR_EL1` field.
    pub const NS2POR_EL1_SHIFT: u32 = 61;
    /// Offset of the `nMAIR2_EL1` field.
    pub const NMAIR2_EL1_SHIFT: u32 = 62;
    /// Offset of the `nAMAIR2_EL1` field.
    pub const NAMAIR2_EL1_SHIFT: u32 = 63;
}

bitflags! {
    /// `HPFAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HpfarEl2: u64 {
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

impl HpfarEl2 {
    /// Offset of the `FIPA` field.
    pub const FIPA_SHIFT: u32 = 4;
    /// Mask for the `FIPA` field.
    pub const FIPA_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 63;

    /// Returns the value of the `FIPA` field.
    pub const fn fipa(self) -> u64 {
        (self.bits() >> Self::FIPA_SHIFT) & Self::FIPA_MASK
    }

    /// Sets the value of the `FIPA` field.
    pub const fn set_fipa(&mut self, value: u64) {
        let offset = Self::FIPA_SHIFT;
        assert!(value & Self::FIPA_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FIPA_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `FIPA` field set to the given value.
    pub const fn with_fipa(mut self, value: u64) -> Self {
        self.set_fipa(value);
        self
    }
}

bitflags! {
    /// `ICC_SRE_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl IccSreEl2 {
    /// Offset of the `SRE` field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the `DFB` field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the `DIB` field.
    pub const DIB_SHIFT: u32 = 2;
    /// Offset of the `Enable` field.
    pub const ENABLE_SHIFT: u32 = 3;
}

bitflags! {
    /// `ICH_HCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `En` field.
    pub const EN_SHIFT: u32 = 0;
    /// Offset of the `UIE` field.
    pub const UIE_SHIFT: u32 = 1;
    /// Offset of the `LRENPIE` field.
    pub const LRENPIE_SHIFT: u32 = 2;
    /// Offset of the `NPIE` field.
    pub const NPIE_SHIFT: u32 = 3;
    /// Offset of the `VGrp0EIE` field.
    pub const VGRP0EIE_SHIFT: u32 = 4;
    /// Offset of the `VGrp0DIE` field.
    pub const VGRP0DIE_SHIFT: u32 = 5;
    /// Offset of the `VGrp1EIE` field.
    pub const VGRP1EIE_SHIFT: u32 = 6;
    /// Offset of the `VGrp1DIE` field.
    pub const VGRP1DIE_SHIFT: u32 = 7;
    /// Offset of the `vSGIEOICount` field.
    pub const VSGIEOICOUNT_SHIFT: u32 = 8;
    /// Offset of the `TC` field.
    pub const TC_SHIFT: u32 = 10;
    /// Offset of the `TALL0` field.
    pub const TALL0_SHIFT: u32 = 11;
    /// Offset of the `TALL1` field.
    pub const TALL1_SHIFT: u32 = 12;
    /// Offset of the `TSEI` field.
    pub const TSEI_SHIFT: u32 = 13;
    /// Offset of the `TDIR` field.
    pub const TDIR_SHIFT: u32 = 14;
    /// Offset of the `DVIM` field.
    pub const DVIM_SHIFT: u32 = 15;
    /// Offset of the `EOIcount` field.
    pub const EOICOUNT_SHIFT: u32 = 27;
    /// Mask for the `EOIcount` field.
    pub const EOICOUNT_MASK: u64 = 0b1_1111;

    /// Returns the value of the `EOIcount` field.
    pub const fn eoicount(self) -> u8 {
        ((self.bits() >> Self::EOICOUNT_SHIFT) & Self::EOICOUNT_MASK) as u8
    }

    /// Sets the value of the `EOIcount` field.
    pub const fn set_eoicount(&mut self, value: u8) {
        let offset = Self::EOICOUNT_SHIFT;
        assert!(value & (Self::EOICOUNT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EOICOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EOIcount` field set to the given value.
    pub const fn with_eoicount(mut self, value: u8) -> Self {
        self.set_eoicount(value);
        self
    }
}

bitflags! {
    /// `ICH_VMCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `EN` field.
    pub const EN_SHIFT: u32 = 0;
    /// Offset of the `VENG0` field.
    pub const VENG0_SHIFT: u32 = 0;
    /// Offset of the `VENG1` field.
    pub const VENG1_SHIFT: u32 = 1;
    /// Offset of the `VAckCtl` field.
    pub const VACKCTL_SHIFT: u32 = 2;
    /// Offset of the `VFIQEn` field.
    pub const VFIQEN_SHIFT: u32 = 3;
    /// Offset of the `VCBPR` field.
    pub const VCBPR_SHIFT: u32 = 4;
    /// Offset of the `VEOIM` field.
    pub const VEOIM_SHIFT: u32 = 9;
    /// Offset of the `VBPR1` field.
    pub const VBPR1_SHIFT: u32 = 18;
    /// Mask for the `VBPR1` field.
    pub const VBPR1_MASK: u64 = 0b111;
    /// Offset of the `VBPR0` field.
    pub const VBPR0_SHIFT: u32 = 21;
    /// Mask for the `VBPR0` field.
    pub const VBPR0_MASK: u64 = 0b111;

    /// Returns the value of the `VBPR1` field.
    pub const fn vbpr1(self) -> u8 {
        ((self.bits() >> Self::VBPR1_SHIFT) & Self::VBPR1_MASK) as u8
    }

    /// Sets the value of the `VBPR1` field.
    pub const fn set_vbpr1(&mut self, value: u8) {
        let offset = Self::VBPR1_SHIFT;
        assert!(value & (Self::VBPR1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBPR1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VBPR1` field set to the given value.
    pub const fn with_vbpr1(mut self, value: u8) -> Self {
        self.set_vbpr1(value);
        self
    }

    /// Returns the value of the `VBPR0` field.
    pub const fn vbpr0(self) -> u8 {
        ((self.bits() >> Self::VBPR0_SHIFT) & Self::VBPR0_MASK) as u8
    }

    /// Sets the value of the `VBPR0` field.
    pub const fn set_vbpr0(&mut self, value: u8) {
        let offset = Self::VBPR0_SHIFT;
        assert!(value & (Self::VBPR0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBPR0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VBPR0` field set to the given value.
    pub const fn with_vbpr0(mut self, value: u8) -> Self {
        self.set_vbpr0(value);
        self
    }
}

bitflags! {
    /// `MAIR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl2: u64 {
    }
}

impl MairEl2 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u64 = 0b1111_1111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + n * 8)) & Self::ATTR_MASK) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::ATTR_SHIFT + n * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Attr<n>` field set to the given value.
    pub const fn with_attr(mut self, n: u32, value: u8) -> Self {
        self.set_attr(n, value);
        self
    }
}

bitflags! {
    /// `MDCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `HPMN` field.
    pub const HPMN_SHIFT: u32 = 0;
    /// Mask for the `HPMN` field.
    pub const HPMN_MASK: u64 = 0b1_1111;
    /// Offset of the `TPMCR` field.
    pub const TPMCR_SHIFT: u32 = 5;
    /// Offset of the `TPM` field.
    pub const TPM_SHIFT: u32 = 6;
    /// Offset of the `HPME` field.
    pub const HPME_SHIFT: u32 = 7;
    /// Offset of the `TDE` field.
    pub const TDE_SHIFT: u32 = 8;
    /// Offset of the `TDA` field.
    pub const TDA_SHIFT: u32 = 9;
    /// Offset of the `TDOSA` field.
    pub const TDOSA_SHIFT: u32 = 10;
    /// Offset of the `TDRA` field.
    pub const TDRA_SHIFT: u32 = 11;
    /// Offset of the `E2PB` field.
    pub const E2PB_SHIFT: u32 = 12;
    /// Mask for the `E2PB` field.
    pub const E2PB_MASK: u64 = 0b11;
    /// Offset of the `TPMS` field.
    pub const TPMS_SHIFT: u32 = 14;
    /// Offset of the `EnSPM` field.
    pub const ENSPM_SHIFT: u32 = 15;
    /// Offset of the `HPMD` field.
    pub const HPMD_SHIFT: u32 = 17;
    /// Offset of the `TTRF` field.
    pub const TTRF_SHIFT: u32 = 19;
    /// Offset of the `HCCD` field.
    pub const HCCD_SHIFT: u32 = 23;
    /// Offset of the `E2TB` field.
    pub const E2TB_SHIFT: u32 = 24;
    /// Mask for the `E2TB` field.
    pub const E2TB_MASK: u64 = 0b11;
    /// Offset of the `HLP` field.
    pub const HLP_SHIFT: u32 = 26;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the `MTPME` field.
    pub const MTPME_SHIFT: u32 = 28;
    /// Offset of the `HPMFZO` field.
    pub const HPMFZO_SHIFT: u32 = 29;
    /// Offset of the `PMSSE` field.
    pub const PMSSE_SHIFT: u32 = 30;
    /// Mask for the `PMSSE` field.
    pub const PMSSE_MASK: u64 = 0b11;
    /// Offset of the `HPMFZS` field.
    pub const HPMFZS_SHIFT: u32 = 36;
    /// Offset of the `PMEE` field.
    pub const PMEE_SHIFT: u32 = 40;
    /// Mask for the `PMEE` field.
    pub const PMEE_MASK: u64 = 0b11;
    /// Offset of the `EBWE` field.
    pub const EBWE_SHIFT: u32 = 43;
    /// Offset of the `EnSTEPOP` field.
    pub const ENSTEPOP_SHIFT: u32 = 50;

    /// Returns the value of the `HPMN` field.
    pub const fn hpmn(self) -> u8 {
        ((self.bits() >> Self::HPMN_SHIFT) & Self::HPMN_MASK) as u8
    }

    /// Sets the value of the `HPMN` field.
    pub const fn set_hpmn(&mut self, value: u8) {
        let offset = Self::HPMN_SHIFT;
        assert!(value & (Self::HPMN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPMN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `HPMN` field set to the given value.
    pub const fn with_hpmn(mut self, value: u8) -> Self {
        self.set_hpmn(value);
        self
    }

    /// Returns the value of the `E2PB` field.
    pub const fn e2pb(self) -> u8 {
        ((self.bits() >> Self::E2PB_SHIFT) & Self::E2PB_MASK) as u8
    }

    /// Sets the value of the `E2PB` field.
    pub const fn set_e2pb(&mut self, value: u8) {
        let offset = Self::E2PB_SHIFT;
        assert!(value & (Self::E2PB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E2PB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `E2PB` field set to the given value.
    pub const fn with_e2pb(mut self, value: u8) -> Self {
        self.set_e2pb(value);
        self
    }

    /// Returns the value of the `E2TB` field.
    pub const fn e2tb(self) -> u8 {
        ((self.bits() >> Self::E2TB_SHIFT) & Self::E2TB_MASK) as u8
    }

    /// Sets the value of the `E2TB` field.
    pub const fn set_e2tb(&mut self, value: u8) {
        let offset = Self::E2TB_SHIFT;
        assert!(value & (Self::E2TB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E2TB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `E2TB` field set to the given value.
    pub const fn with_e2tb(mut self, value: u8) -> Self {
        self.set_e2tb(value);
        self
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        ((self.bits() >> Self::PMSSE_SHIFT) & Self::PMSSE_MASK) as u8
    }

    /// Sets the value of the `PMSSE` field.
    pub const fn set_pmsse(&mut self, value: u8) {
        let offset = Self::PMSSE_SHIFT;
        assert!(value & (Self::PMSSE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSSE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMSSE` field set to the given value.
    pub const fn with_pmsse(mut self, value: u8) -> Self {
        self.set_pmsse(value);
        self
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        ((self.bits() >> Self::PMEE_SHIFT) & Self::PMEE_MASK) as u8
    }

    /// Sets the value of the `PMEE` field.
    pub const fn set_pmee(&mut self, value: u8) {
        let offset = Self::PMEE_SHIFT;
        assert!(value & (Self::PMEE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMEE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMEE` field set to the given value.
    pub const fn with_pmee(mut self, value: u8) -> Self {
        self.set_pmee(value);
        self
    }
}

bitflags! {
    /// `MPAM2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `PARTID` field.
    pub const PARTID_SHIFT: u32 = 0;
    /// Mask for the `PARTID` field.
    pub const PARTID_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PARTID_I` field.
    pub const PARTID_I_SHIFT: u32 = 0;
    /// Mask for the `PARTID_I` field.
    pub const PARTID_I_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PARTID_D` field.
    pub const PARTID_D_SHIFT: u32 = 16;
    /// Mask for the `PARTID_D` field.
    pub const PARTID_D_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `altPARTID` field.
    pub const ALTPARTID_SHIFT: u32 = 16;
    /// Mask for the `altPARTID` field.
    pub const ALTPARTID_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PMG` field.
    pub const PMG_SHIFT: u32 = 32;
    /// Mask for the `PMG` field.
    pub const PMG_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PMG_I` field.
    pub const PMG_I_SHIFT: u32 = 32;
    /// Mask for the `PMG_I` field.
    pub const PMG_I_MASK: u64 = 0b1111_1111;
    /// Offset of the `PMG_D` field.
    pub const PMG_D_SHIFT: u32 = 40;
    /// Mask for the `PMG_D` field.
    pub const PMG_D_MASK: u64 = 0b1111_1111;
    /// Offset of the `TRAPMPAM1EL1` field.
    pub const TRAPMPAM1EL1_SHIFT: u32 = 48;
    /// Offset of the `altPMG` field.
    pub const ALTPMG_SHIFT: u32 = 48;
    /// Mask for the `altPMG` field.
    pub const ALTPMG_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `TRAPMPAM0EL1` field.
    pub const TRAPMPAM0EL1_SHIFT: u32 = 49;
    /// Offset of the `EnMPAMSM` field.
    pub const ENMPAMSM_SHIFT: u32 = 50;
    /// Offset of the `ALTSP_FRCD` field.
    pub const ALTSP_FRCD_SHIFT: u32 = 54;
    /// Offset of the `ALTSP_EL2` field.
    pub const ALTSP_EL2_SHIFT: u32 = 55;
    /// Offset of the `ALTSP_HFC` field.
    pub const ALTSP_HFC_SHIFT: u32 = 56;
    /// Offset of the `TIDR` field.
    pub const TIDR_SHIFT: u32 = 58;
    /// Offset of the `MPAMEN` field.
    pub const MPAMEN_SHIFT: u32 = 63;

    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        ((self.bits() >> Self::PARTID_SHIFT) & Self::PARTID_MASK) as u16
    }

    /// Sets the value of the `PARTID` field.
    pub const fn set_partid(&mut self, value: u16) {
        let offset = Self::PARTID_SHIFT;
        assert!(value & (Self::PARTID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PARTID` field set to the given value.
    pub const fn with_partid(mut self, value: u16) -> Self {
        self.set_partid(value);
        self
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        ((self.bits() >> Self::PARTID_I_SHIFT) & Self::PARTID_I_MASK) as u16
    }

    /// Sets the value of the `PARTID_I` field.
    pub const fn set_partid_i(&mut self, value: u16) {
        let offset = Self::PARTID_I_SHIFT;
        assert!(value & (Self::PARTID_I_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_I_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PARTID_I` field set to the given value.
    pub const fn with_partid_i(mut self, value: u16) -> Self {
        self.set_partid_i(value);
        self
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        ((self.bits() >> Self::PARTID_D_SHIFT) & Self::PARTID_D_MASK) as u16
    }

    /// Sets the value of the `PARTID_D` field.
    pub const fn set_partid_d(&mut self, value: u16) {
        let offset = Self::PARTID_D_SHIFT;
        assert!(value & (Self::PARTID_D_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_D_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PARTID_D` field set to the given value.
    pub const fn with_partid_d(mut self, value: u16) -> Self {
        self.set_partid_d(value);
        self
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        ((self.bits() >> Self::ALTPARTID_SHIFT) & Self::ALTPARTID_MASK) as u16
    }

    /// Sets the value of the `altPARTID` field.
    pub const fn set_altpartid(&mut self, value: u16) {
        let offset = Self::ALTPARTID_SHIFT;
        assert!(value & (Self::ALTPARTID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ALTPARTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `altPARTID` field set to the given value.
    pub const fn with_altpartid(mut self, value: u16) -> Self {
        self.set_altpartid(value);
        self
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        ((self.bits() >> Self::PMG_SHIFT) & Self::PMG_MASK) as u16
    }

    /// Sets the value of the `PMG` field.
    pub const fn set_pmg(&mut self, value: u16) {
        let offset = Self::PMG_SHIFT;
        assert!(value & (Self::PMG_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMG` field set to the given value.
    pub const fn with_pmg(mut self, value: u16) -> Self {
        self.set_pmg(value);
        self
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        ((self.bits() >> Self::PMG_I_SHIFT) & Self::PMG_I_MASK) as u8
    }

    /// Sets the value of the `PMG_I` field.
    pub const fn set_pmg_i(&mut self, value: u8) {
        let offset = Self::PMG_I_SHIFT;
        assert!(value & (Self::PMG_I_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_I_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMG_I` field set to the given value.
    pub const fn with_pmg_i(mut self, value: u8) -> Self {
        self.set_pmg_i(value);
        self
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        ((self.bits() >> Self::PMG_D_SHIFT) & Self::PMG_D_MASK) as u8
    }

    /// Sets the value of the `PMG_D` field.
    pub const fn set_pmg_d(&mut self, value: u8) {
        let offset = Self::PMG_D_SHIFT;
        assert!(value & (Self::PMG_D_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_D_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMG_D` field set to the given value.
    pub const fn with_pmg_d(mut self, value: u8) -> Self {
        self.set_pmg_d(value);
        self
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        ((self.bits() >> Self::ALTPMG_SHIFT) & Self::ALTPMG_MASK) as u16
    }

    /// Sets the value of the `altPMG` field.
    pub const fn set_altpmg(&mut self, value: u16) {
        let offset = Self::ALTPMG_SHIFT;
        assert!(value & (Self::ALTPMG_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ALTPMG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `altPMG` field set to the given value.
    pub const fn with_altpmg(mut self, value: u16) -> Self {
        self.set_altpmg(value);
        self
    }
}

bitflags! {
    /// `MPAMHCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamhcrEl2: u64 {
        /// `EL0_VPMEN` bit.
        const EL0_VPMEN = 1 << 0;
        /// `EL1_VPMEN` bit.
        const EL1_VPMEN = 1 << 1;
        /// `GSTAPP_PLK` bit.
        const GSTAPP_PLK = 1 << 8;
        /// `TRAP_MPAMIDR_EL1` bit.
        const TRAP_MPAMIDR_EL1 = 1 << 31;
        /// `nTRAPMPAM1EL1` bit.
        const NTRAPMPAM1EL1 = 1 << 48;
        /// `nTRAPMPAM0EL1` bit.
        const NTRAPMPAM0EL1 = 1 << 49;
        /// `nTRAPMPAMSM` bit.
        const NTRAPMPAMSM = 1 << 50;
        /// `nTIDR` bit.
        const NTIDR = 1 << 58;
    }
}

impl MpamhcrEl2 {
    /// Offset of the `EL0_VPMEN` field.
    pub const EL0_VPMEN_SHIFT: u32 = 0;
    /// Offset of the `EL1_VPMEN` field.
    pub const EL1_VPMEN_SHIFT: u32 = 1;
    /// Offset of the `GSTAPP_PLK` field.
    pub const GSTAPP_PLK_SHIFT: u32 = 8;
    /// Offset of the `TRAP_MPAMIDR_EL1` field.
    pub const TRAP_MPAMIDR_EL1_SHIFT: u32 = 31;
    /// Offset of the `nTRAPMPAM1EL1` field.
    pub const NTRAPMPAM1EL1_SHIFT: u32 = 48;
    /// Offset of the `nTRAPMPAM0EL1` field.
    pub const NTRAPMPAM0EL1_SHIFT: u32 = 49;
    /// Offset of the `nTRAPMPAMSM` field.
    pub const NTRAPMPAMSM_SHIFT: u32 = 50;
    /// Offset of the `nTIDR` field.
    pub const NTIDR_SHIFT: u32 = 58;
}

bitflags! {
    /// `MPAMVPM0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm0El2: u64 {
    }
}

impl Mpamvpm0El2 {
    /// Offset of the `PhyPARTID0` field.
    pub const PHYPARTID0_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID0` field.
    pub const PHYPARTID0_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID1` field.
    pub const PHYPARTID1_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID1` field.
    pub const PHYPARTID1_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID2` field.
    pub const PHYPARTID2_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID2` field.
    pub const PHYPARTID2_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID3` field.
    pub const PHYPARTID3_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID3` field.
    pub const PHYPARTID3_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID0` field.
    pub const fn phypartid0(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID0_SHIFT) & Self::PHYPARTID0_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID0` field.
    pub const fn set_phypartid0(&mut self, value: u16) {
        let offset = Self::PHYPARTID0_SHIFT;
        assert!(value & (Self::PHYPARTID0_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID0` field set to the given value.
    pub const fn with_phypartid0(mut self, value: u16) -> Self {
        self.set_phypartid0(value);
        self
    }

    /// Returns the value of the `PhyPARTID1` field.
    pub const fn phypartid1(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID1_SHIFT) & Self::PHYPARTID1_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID1` field.
    pub const fn set_phypartid1(&mut self, value: u16) {
        let offset = Self::PHYPARTID1_SHIFT;
        assert!(value & (Self::PHYPARTID1_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID1` field set to the given value.
    pub const fn with_phypartid1(mut self, value: u16) -> Self {
        self.set_phypartid1(value);
        self
    }

    /// Returns the value of the `PhyPARTID2` field.
    pub const fn phypartid2(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID2_SHIFT) & Self::PHYPARTID2_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID2` field.
    pub const fn set_phypartid2(&mut self, value: u16) {
        let offset = Self::PHYPARTID2_SHIFT;
        assert!(value & (Self::PHYPARTID2_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID2` field set to the given value.
    pub const fn with_phypartid2(mut self, value: u16) -> Self {
        self.set_phypartid2(value);
        self
    }

    /// Returns the value of the `PhyPARTID3` field.
    pub const fn phypartid3(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID3_SHIFT) & Self::PHYPARTID3_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID3` field.
    pub const fn set_phypartid3(&mut self, value: u16) {
        let offset = Self::PHYPARTID3_SHIFT;
        assert!(value & (Self::PHYPARTID3_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID3` field set to the given value.
    pub const fn with_phypartid3(mut self, value: u16) -> Self {
        self.set_phypartid3(value);
        self
    }
}

bitflags! {
    /// `MPAMVPM1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm1El2: u64 {
    }
}

impl Mpamvpm1El2 {
    /// Offset of the `PhyPARTID4` field.
    pub const PHYPARTID4_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID4` field.
    pub const PHYPARTID4_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID5` field.
    pub const PHYPARTID5_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID5` field.
    pub const PHYPARTID5_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID6` field.
    pub const PHYPARTID6_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID6` field.
    pub const PHYPARTID6_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID7` field.
    pub const PHYPARTID7_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID7` field.
    pub const PHYPARTID7_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID4` field.
    pub const fn phypartid4(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID4_SHIFT) & Self::PHYPARTID4_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID4` field.
    pub const fn set_phypartid4(&mut self, value: u16) {
        let offset = Self::PHYPARTID4_SHIFT;
        assert!(value & (Self::PHYPARTID4_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID4_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID4` field set to the given value.
    pub const fn with_phypartid4(mut self, value: u16) -> Self {
        self.set_phypartid4(value);
        self
    }

    /// Returns the value of the `PhyPARTID5` field.
    pub const fn phypartid5(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID5_SHIFT) & Self::PHYPARTID5_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID5` field.
    pub const fn set_phypartid5(&mut self, value: u16) {
        let offset = Self::PHYPARTID5_SHIFT;
        assert!(value & (Self::PHYPARTID5_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID5_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID5` field set to the given value.
    pub const fn with_phypartid5(mut self, value: u16) -> Self {
        self.set_phypartid5(value);
        self
    }

    /// Returns the value of the `PhyPARTID6` field.
    pub const fn phypartid6(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID6_SHIFT) & Self::PHYPARTID6_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID6` field.
    pub const fn set_phypartid6(&mut self, value: u16) {
        let offset = Self::PHYPARTID6_SHIFT;
        assert!(value & (Self::PHYPARTID6_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID6_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID6` field set to the given value.
    pub const fn with_phypartid6(mut self, value: u16) -> Self {
        self.set_phypartid6(value);
        self
    }

    /// Returns the value of the `PhyPARTID7` field.
    pub const fn phypartid7(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID7_SHIFT) & Self::PHYPARTID7_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID7` field.
    pub const fn set_phypartid7(&mut self, value: u16) {
        let offset = Self::PHYPARTID7_SHIFT;
        assert!(value & (Self::PHYPARTID7_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID7_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID7` field set to the given value.
    pub const fn with_phypartid7(mut self, value: u16) -> Self {
        self.set_phypartid7(value);
        self
    }
}

bitflags! {
    /// `MPAMVPM2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm2El2: u64 {
    }
}

impl Mpamvpm2El2 {
    /// Offset of the `PhyPARTID8` field.
    pub const PHYPARTID8_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID8` field.
    pub const PHYPARTID8_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID9` field.
    pub const PHYPARTID9_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID9` field.
    pub const PHYPARTID9_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID10` field.
    pub const PHYPARTID10_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID10` field.
    pub const PHYPARTID10_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID11` field.
    pub const PHYPARTID11_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID11` field.
    pub const PHYPARTID11_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID8` field.
    pub const fn phypartid8(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID8_SHIFT) & Self::PHYPARTID8_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID8` field.
    pub const fn set_phypartid8(&mut self, value: u16) {
        let offset = Self::PHYPARTID8_SHIFT;
        assert!(value & (Self::PHYPARTID8_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID8_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID8` field set to the given value.
    pub const fn with_phypartid8(mut self, value: u16) -> Self {
        self.set_phypartid8(value);
        self
    }

    /// Returns the value of the `PhyPARTID9` field.
    pub const fn phypartid9(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID9_SHIFT) & Self::PHYPARTID9_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID9` field.
    pub const fn set_phypartid9(&mut self, value: u16) {
        let offset = Self::PHYPARTID9_SHIFT;
        assert!(value & (Self::PHYPARTID9_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID9_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID9` field set to the given value.
    pub const fn with_phypartid9(mut self, value: u16) -> Self {
        self.set_phypartid9(value);
        self
    }

    /// Returns the value of the `PhyPARTID10` field.
    pub const fn phypartid10(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID10_SHIFT) & Self::PHYPARTID10_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID10` field.
    pub const fn set_phypartid10(&mut self, value: u16) {
        let offset = Self::PHYPARTID10_SHIFT;
        assert!(value & (Self::PHYPARTID10_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID10_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID10` field set to the given value.
    pub const fn with_phypartid10(mut self, value: u16) -> Self {
        self.set_phypartid10(value);
        self
    }

    /// Returns the value of the `PhyPARTID11` field.
    pub const fn phypartid11(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID11_SHIFT) & Self::PHYPARTID11_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID11` field.
    pub const fn set_phypartid11(&mut self, value: u16) {
        let offset = Self::PHYPARTID11_SHIFT;
        assert!(value & (Self::PHYPARTID11_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID11_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID11` field set to the given value.
    pub const fn with_phypartid11(mut self, value: u16) -> Self {
        self.set_phypartid11(value);
        self
    }
}

bitflags! {
    /// `MPAMVPM3_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm3El2: u64 {
    }
}

impl Mpamvpm3El2 {
    /// Offset of the `PhyPARTID12` field.
    pub const PHYPARTID12_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID12` field.
    pub const PHYPARTID12_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID13` field.
    pub const PHYPARTID13_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID13` field.
    pub const PHYPARTID13_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID14` field.
    pub const PHYPARTID14_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID14` field.
    pub const PHYPARTID14_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID15` field.
    pub const PHYPARTID15_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID15` field.
    pub const PHYPARTID15_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID12` field.
    pub const fn phypartid12(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID12_SHIFT) & Self::PHYPARTID12_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID12` field.
    pub const fn set_phypartid12(&mut self, value: u16) {
        let offset = Self::PHYPARTID12_SHIFT;
        assert!(value & (Self::PHYPARTID12_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID12_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID12` field set to the given value.
    pub const fn with_phypartid12(mut self, value: u16) -> Self {
        self.set_phypartid12(value);
        self
    }

    /// Returns the value of the `PhyPARTID13` field.
    pub const fn phypartid13(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID13_SHIFT) & Self::PHYPARTID13_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID13` field.
    pub const fn set_phypartid13(&mut self, value: u16) {
        let offset = Self::PHYPARTID13_SHIFT;
        assert!(value & (Self::PHYPARTID13_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID13_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID13` field set to the given value.
    pub const fn with_phypartid13(mut self, value: u16) -> Self {
        self.set_phypartid13(value);
        self
    }

    /// Returns the value of the `PhyPARTID14` field.
    pub const fn phypartid14(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID14_SHIFT) & Self::PHYPARTID14_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID14` field.
    pub const fn set_phypartid14(&mut self, value: u16) {
        let offset = Self::PHYPARTID14_SHIFT;
        assert!(value & (Self::PHYPARTID14_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID14_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID14` field set to the given value.
    pub const fn with_phypartid14(mut self, value: u16) -> Self {
        self.set_phypartid14(value);
        self
    }

    /// Returns the value of the `PhyPARTID15` field.
    pub const fn phypartid15(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID15_SHIFT) & Self::PHYPARTID15_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID15` field.
    pub const fn set_phypartid15(&mut self, value: u16) {
        let offset = Self::PHYPARTID15_SHIFT;
        assert!(value & (Self::PHYPARTID15_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID15_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID15` field set to the given value.
    pub const fn with_phypartid15(mut self, value: u16) -> Self {
        self.set_phypartid15(value);
        self
    }
}

bitflags! {
    /// `MPAMVPM4_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm4El2: u64 {
    }
}

impl Mpamvpm4El2 {
    /// Offset of the `PhyPARTID16` field.
    pub const PHYPARTID16_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID16` field.
    pub const PHYPARTID16_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID17` field.
    pub const PHYPARTID17_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID17` field.
    pub const PHYPARTID17_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID18` field.
    pub const PHYPARTID18_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID18` field.
    pub const PHYPARTID18_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID19` field.
    pub const PHYPARTID19_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID19` field.
    pub const PHYPARTID19_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID16` field.
    pub const fn phypartid16(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID16_SHIFT) & Self::PHYPARTID16_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID16` field.
    pub const fn set_phypartid16(&mut self, value: u16) {
        let offset = Self::PHYPARTID16_SHIFT;
        assert!(value & (Self::PHYPARTID16_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID16_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID16` field set to the given value.
    pub const fn with_phypartid16(mut self, value: u16) -> Self {
        self.set_phypartid16(value);
        self
    }

    /// Returns the value of the `PhyPARTID17` field.
    pub const fn phypartid17(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID17_SHIFT) & Self::PHYPARTID17_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID17` field.
    pub const fn set_phypartid17(&mut self, value: u16) {
        let offset = Self::PHYPARTID17_SHIFT;
        assert!(value & (Self::PHYPARTID17_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID17_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID17` field set to the given value.
    pub const fn with_phypartid17(mut self, value: u16) -> Self {
        self.set_phypartid17(value);
        self
    }

    /// Returns the value of the `PhyPARTID18` field.
    pub const fn phypartid18(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID18_SHIFT) & Self::PHYPARTID18_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID18` field.
    pub const fn set_phypartid18(&mut self, value: u16) {
        let offset = Self::PHYPARTID18_SHIFT;
        assert!(value & (Self::PHYPARTID18_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID18_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID18` field set to the given value.
    pub const fn with_phypartid18(mut self, value: u16) -> Self {
        self.set_phypartid18(value);
        self
    }

    /// Returns the value of the `PhyPARTID19` field.
    pub const fn phypartid19(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID19_SHIFT) & Self::PHYPARTID19_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID19` field.
    pub const fn set_phypartid19(&mut self, value: u16) {
        let offset = Self::PHYPARTID19_SHIFT;
        assert!(value & (Self::PHYPARTID19_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID19_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID19` field set to the given value.
    pub const fn with_phypartid19(mut self, value: u16) -> Self {
        self.set_phypartid19(value);
        self
    }
}

bitflags! {
    /// `MPAMVPM5_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm5El2: u64 {
    }
}

impl Mpamvpm5El2 {
    /// Offset of the `PhyPARTID20` field.
    pub const PHYPARTID20_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID20` field.
    pub const PHYPARTID20_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID21` field.
    pub const PHYPARTID21_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID21` field.
    pub const PHYPARTID21_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID22` field.
    pub const PHYPARTID22_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID22` field.
    pub const PHYPARTID22_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID23` field.
    pub const PHYPARTID23_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID23` field.
    pub const PHYPARTID23_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID20` field.
    pub const fn phypartid20(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID20_SHIFT) & Self::PHYPARTID20_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID20` field.
    pub const fn set_phypartid20(&mut self, value: u16) {
        let offset = Self::PHYPARTID20_SHIFT;
        assert!(value & (Self::PHYPARTID20_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID20_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID20` field set to the given value.
    pub const fn with_phypartid20(mut self, value: u16) -> Self {
        self.set_phypartid20(value);
        self
    }

    /// Returns the value of the `PhyPARTID21` field.
    pub const fn phypartid21(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID21_SHIFT) & Self::PHYPARTID21_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID21` field.
    pub const fn set_phypartid21(&mut self, value: u16) {
        let offset = Self::PHYPARTID21_SHIFT;
        assert!(value & (Self::PHYPARTID21_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID21_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID21` field set to the given value.
    pub const fn with_phypartid21(mut self, value: u16) -> Self {
        self.set_phypartid21(value);
        self
    }

    /// Returns the value of the `PhyPARTID22` field.
    pub const fn phypartid22(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID22_SHIFT) & Self::PHYPARTID22_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID22` field.
    pub const fn set_phypartid22(&mut self, value: u16) {
        let offset = Self::PHYPARTID22_SHIFT;
        assert!(value & (Self::PHYPARTID22_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID22_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID22` field set to the given value.
    pub const fn with_phypartid22(mut self, value: u16) -> Self {
        self.set_phypartid22(value);
        self
    }

    /// Returns the value of the `PhyPARTID23` field.
    pub const fn phypartid23(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID23_SHIFT) & Self::PHYPARTID23_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID23` field.
    pub const fn set_phypartid23(&mut self, value: u16) {
        let offset = Self::PHYPARTID23_SHIFT;
        assert!(value & (Self::PHYPARTID23_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID23_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID23` field set to the given value.
    pub const fn with_phypartid23(mut self, value: u16) -> Self {
        self.set_phypartid23(value);
        self
    }
}

bitflags! {
    /// `MPAMVPM6_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm6El2: u64 {
    }
}

impl Mpamvpm6El2 {
    /// Offset of the `PhyPARTID24` field.
    pub const PHYPARTID24_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID24` field.
    pub const PHYPARTID24_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID25` field.
    pub const PHYPARTID25_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID25` field.
    pub const PHYPARTID25_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID26` field.
    pub const PHYPARTID26_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID26` field.
    pub const PHYPARTID26_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID27` field.
    pub const PHYPARTID27_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID27` field.
    pub const PHYPARTID27_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID24` field.
    pub const fn phypartid24(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID24_SHIFT) & Self::PHYPARTID24_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID24` field.
    pub const fn set_phypartid24(&mut self, value: u16) {
        let offset = Self::PHYPARTID24_SHIFT;
        assert!(value & (Self::PHYPARTID24_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID24_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID24` field set to the given value.
    pub const fn with_phypartid24(mut self, value: u16) -> Self {
        self.set_phypartid24(value);
        self
    }

    /// Returns the value of the `PhyPARTID25` field.
    pub const fn phypartid25(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID25_SHIFT) & Self::PHYPARTID25_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID25` field.
    pub const fn set_phypartid25(&mut self, value: u16) {
        let offset = Self::PHYPARTID25_SHIFT;
        assert!(value & (Self::PHYPARTID25_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID25_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID25` field set to the given value.
    pub const fn with_phypartid25(mut self, value: u16) -> Self {
        self.set_phypartid25(value);
        self
    }

    /// Returns the value of the `PhyPARTID26` field.
    pub const fn phypartid26(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID26_SHIFT) & Self::PHYPARTID26_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID26` field.
    pub const fn set_phypartid26(&mut self, value: u16) {
        let offset = Self::PHYPARTID26_SHIFT;
        assert!(value & (Self::PHYPARTID26_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID26_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID26` field set to the given value.
    pub const fn with_phypartid26(mut self, value: u16) -> Self {
        self.set_phypartid26(value);
        self
    }

    /// Returns the value of the `PhyPARTID27` field.
    pub const fn phypartid27(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID27_SHIFT) & Self::PHYPARTID27_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID27` field.
    pub const fn set_phypartid27(&mut self, value: u16) {
        let offset = Self::PHYPARTID27_SHIFT;
        assert!(value & (Self::PHYPARTID27_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID27_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID27` field set to the given value.
    pub const fn with_phypartid27(mut self, value: u16) -> Self {
        self.set_phypartid27(value);
        self
    }
}

bitflags! {
    /// `MPAMVPM7_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm7El2: u64 {
    }
}

impl Mpamvpm7El2 {
    /// Offset of the `PhyPARTID28` field.
    pub const PHYPARTID28_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID28` field.
    pub const PHYPARTID28_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID29` field.
    pub const PHYPARTID29_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID29` field.
    pub const PHYPARTID29_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID30` field.
    pub const PHYPARTID30_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID30` field.
    pub const PHYPARTID30_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `PhyPARTID31` field.
    pub const PHYPARTID31_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID31` field.
    pub const PHYPARTID31_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `PhyPARTID28` field.
    pub const fn phypartid28(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID28_SHIFT) & Self::PHYPARTID28_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID28` field.
    pub const fn set_phypartid28(&mut self, value: u16) {
        let offset = Self::PHYPARTID28_SHIFT;
        assert!(value & (Self::PHYPARTID28_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID28_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID28` field set to the given value.
    pub const fn with_phypartid28(mut self, value: u16) -> Self {
        self.set_phypartid28(value);
        self
    }

    /// Returns the value of the `PhyPARTID29` field.
    pub const fn phypartid29(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID29_SHIFT) & Self::PHYPARTID29_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID29` field.
    pub const fn set_phypartid29(&mut self, value: u16) {
        let offset = Self::PHYPARTID29_SHIFT;
        assert!(value & (Self::PHYPARTID29_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID29_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID29` field set to the given value.
    pub const fn with_phypartid29(mut self, value: u16) -> Self {
        self.set_phypartid29(value);
        self
    }

    /// Returns the value of the `PhyPARTID30` field.
    pub const fn phypartid30(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID30_SHIFT) & Self::PHYPARTID30_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID30` field.
    pub const fn set_phypartid30(&mut self, value: u16) {
        let offset = Self::PHYPARTID30_SHIFT;
        assert!(value & (Self::PHYPARTID30_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID30_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID30` field set to the given value.
    pub const fn with_phypartid30(mut self, value: u16) -> Self {
        self.set_phypartid30(value);
        self
    }

    /// Returns the value of the `PhyPARTID31` field.
    pub const fn phypartid31(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID31_SHIFT) & Self::PHYPARTID31_MASK) as u16
    }

    /// Sets the value of the `PhyPARTID31` field.
    pub const fn set_phypartid31(&mut self, value: u16) {
        let offset = Self::PHYPARTID31_SHIFT;
        assert!(value & (Self::PHYPARTID31_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID31_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PhyPARTID31` field set to the given value.
    pub const fn with_phypartid31(mut self, value: u16) -> Self {
        self.set_phypartid31(value);
        self
    }
}

bitflags! {
    /// `MPAMVPMV_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl MpamvpmvEl2 {
    /// Offset of the `VPM_V<m>` field.
    pub const VPM_V_SHIFT: u32 = 0;
}

bitflags! {
    /// `PFAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PfarEl2: u64 {
        /// `NSE2` bit.
        const NSE2 = 1 << 61;
        /// `NSE` bit.
        const NSE = 1 << 62;
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

impl PfarEl2 {
    /// Offset of the `PA` field.
    pub const PA_SHIFT: u32 = 0;
    /// Mask for the `PA` field.
    pub const PA_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `PA[51:48]` field.
    pub const PA_51_48_SHIFT: u32 = 48;
    /// Mask for the `PA[51:48]` field.
    pub const PA_51_48_MASK: u64 = 0b1111;
    /// Offset of the `PA[55:52]` field.
    pub const PA_55_52_SHIFT: u32 = 52;
    /// Mask for the `PA[55:52]` field.
    pub const PA_55_52_MASK: u64 = 0b1111;
    /// Offset of the `NSE2` field.
    pub const NSE2_SHIFT: u32 = 61;
    /// Offset of the `NSE` field.
    pub const NSE_SHIFT: u32 = 62;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 63;

    /// Returns the value of the `PA` field.
    pub const fn pa(self) -> u64 {
        (self.bits() >> Self::PA_SHIFT) & Self::PA_MASK
    }

    /// Sets the value of the `PA` field.
    pub const fn set_pa(&mut self, value: u64) {
        let offset = Self::PA_SHIFT;
        assert!(value & Self::PA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::PA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `PA` field set to the given value.
    pub const fn with_pa(mut self, value: u64) -> Self {
        self.set_pa(value);
        self
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        ((self.bits() >> Self::PA_51_48_SHIFT) & Self::PA_51_48_MASK) as u8
    }

    /// Sets the value of the `PA[51:48]` field.
    pub const fn set_pa_51_48(&mut self, value: u8) {
        let offset = Self::PA_51_48_SHIFT;
        assert!(value & (Self::PA_51_48_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_51_48_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PA[51:48]` field set to the given value.
    pub const fn with_pa_51_48(mut self, value: u8) -> Self {
        self.set_pa_51_48(value);
        self
    }

    /// Returns the value of the `PA[55:52]` field.
    pub const fn pa_55_52(self) -> u8 {
        ((self.bits() >> Self::PA_55_52_SHIFT) & Self::PA_55_52_MASK) as u8
    }

    /// Sets the value of the `PA[55:52]` field.
    pub const fn set_pa_55_52(&mut self, value: u8) {
        let offset = Self::PA_55_52_SHIFT;
        assert!(value & (Self::PA_55_52_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_55_52_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PA[55:52]` field set to the given value.
    pub const fn with_pa_55_52(mut self, value: u8) -> Self {
        self.set_pa_55_52(value);
        self
    }
}

bitflags! {
    /// `PIRE0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pire0El2: u64 {
    }
}

impl Pire0El2 {
    /// Offset of the `Perm<m>` field.
    pub const PERM_SHIFT: u32 = 0;
    /// Mask for the `Perm<m>` field.
    pub const PERM_MASK: u64 = 0b1111;

    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        ((self.bits() >> (Self::PERM_SHIFT + m * 4)) & Self::PERM_MASK) as u8
    }

    /// Sets the value of the `Perm<m>` field.
    pub const fn set_perm(&mut self, m: u32, value: u8) {
        assert!(m < 16);
        let offset = Self::PERM_SHIFT + m * 4;
        assert!(value & (Self::PERM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PERM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Perm<m>` field set to the given value.
    pub const fn with_perm(mut self, m: u32, value: u8) -> Self {
        self.set_perm(m, value);
        self
    }
}

/// `PIR_EL2` system register value.
pub type PirEl2 = Pire0El2;

/// `POR_EL2` system register value.
pub type PorEl2 = Pire0El2;

/// `S2PIR_EL2` system register value.
pub type S2pirEl2 = Pire0El2;

bitflags! {
    /// `SCTLR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr2El2: u64 {
        /// `EMEC` bit.
        const EMEC = 1 << 1;
        /// `NMEA` bit.
        const NMEA = 1 << 2;
        /// `EnADERR` bit.
        const ENADERR = 1 << 3;
        /// `EnANERR` bit.
        const ENANERR = 1 << 4;
        /// `EASE` bit.
        const EASE = 1 << 5;
        /// `EnIDCP128` bit.
        const ENIDCP128 = 1 << 6;
        /// `EnPACM` bit.
        const ENPACM = 1 << 7;
        /// `EnPACM0` bit.
        const ENPACM0 = 1 << 8;
        /// `CPTA` bit.
        const CPTA = 1 << 9;
        /// `CPTA0` bit.
        const CPTA0 = 1 << 10;
        /// `CPTM` bit.
        const CPTM = 1 << 11;
        /// `CPTM0` bit.
        const CPTM0 = 1 << 12;
        /// `DTZ` bit.
        const DTZ = 1 << 14;
        /// `TEIS` bit.
        const TEIS = 1 << 15;
        /// `TEOS` bit.
        const TEOS = 1 << 16;
        /// `VT` bit.
        const VT = 1 << 17;
        /// `EnDB2` bit.
        const ENDB2 = 1 << 19;
        /// `EnDA2` bit.
        const ENDA2 = 1 << 20;
        /// `EnIB2` bit.
        const ENIB2 = 1 << 21;
        /// `EnIA2` bit.
        const ENIA2 = 1 << 22;
        /// `BTD0` bit.
        const BTD0 = 1 << 23;
        /// `BTD` bit.
        const BTD = 1 << 24;
        /// `FDIT` bit.
        const FDIT = 1 << 25;
        /// `TLBOSNIS` bit.
        const TLBOSNIS = 1 << 26;
        /// `EnTP3` bit.
        const ENTP3 = 1 << 28;
    }
}

impl Sctlr2El2 {
    /// Offset of the `EMEC` field.
    pub const EMEC_SHIFT: u32 = 1;
    /// Offset of the `NMEA` field.
    pub const NMEA_SHIFT: u32 = 2;
    /// Offset of the `EnADERR` field.
    pub const ENADERR_SHIFT: u32 = 3;
    /// Offset of the `EnANERR` field.
    pub const ENANERR_SHIFT: u32 = 4;
    /// Offset of the `EASE` field.
    pub const EASE_SHIFT: u32 = 5;
    /// Offset of the `EnIDCP128` field.
    pub const ENIDCP128_SHIFT: u32 = 6;
    /// Offset of the `EnPACM` field.
    pub const ENPACM_SHIFT: u32 = 7;
    /// Offset of the `EnPACM0` field.
    pub const ENPACM0_SHIFT: u32 = 8;
    /// Offset of the `CPTA` field.
    pub const CPTA_SHIFT: u32 = 9;
    /// Offset of the `CPTA0` field.
    pub const CPTA0_SHIFT: u32 = 10;
    /// Offset of the `CPTM` field.
    pub const CPTM_SHIFT: u32 = 11;
    /// Offset of the `CPTM0` field.
    pub const CPTM0_SHIFT: u32 = 12;
    /// Offset of the `DTZ` field.
    pub const DTZ_SHIFT: u32 = 14;
    /// Offset of the `TEIS` field.
    pub const TEIS_SHIFT: u32 = 15;
    /// Offset of the `TEOS` field.
    pub const TEOS_SHIFT: u32 = 16;
    /// Offset of the `VT` field.
    pub const VT_SHIFT: u32 = 17;
    /// Offset of the `EnDB2` field.
    pub const ENDB2_SHIFT: u32 = 19;
    /// Offset of the `EnDA2` field.
    pub const ENDA2_SHIFT: u32 = 20;
    /// Offset of the `EnIB2` field.
    pub const ENIB2_SHIFT: u32 = 21;
    /// Offset of the `EnIA2` field.
    pub const ENIA2_SHIFT: u32 = 22;
    /// Offset of the `BTD0` field.
    pub const BTD0_SHIFT: u32 = 23;
    /// Offset of the `BTD` field.
    pub const BTD_SHIFT: u32 = 24;
    /// Offset of the `FDIT` field.
    pub const FDIT_SHIFT: u32 = 25;
    /// Offset of the `TLBOSNIS` field.
    pub const TLBOSNIS_SHIFT: u32 = 26;
    /// Offset of the `EnTP3` field.
    pub const ENTP3_SHIFT: u32 = 28;
}

bitflags! {
    /// `SCTLR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `SA` field.
    pub const SA_SHIFT: u32 = 3;
    /// Offset of the `SA0` field.
    pub const SA0_SHIFT: u32 = 4;
    /// Offset of the `CP15BEN` field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the `nAA` field.
    pub const NAA_SHIFT: u32 = 6;
    /// Offset of the `SED` field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the `UMA` field.
    pub const UMA_SHIFT: u32 = 9;
    /// Offset of the `EnRCTX` field.
    pub const ENRCTX_SHIFT: u32 = 10;
    /// Offset of the `EOS` field.
    pub const EOS_SHIFT: u32 = 11;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the `EnDB` field.
    pub const ENDB_SHIFT: u32 = 13;
    /// Offset of the `DZE` field.
    pub const DZE_SHIFT: u32 = 14;
    /// Offset of the `UCT` field.
    pub const UCT_SHIFT: u32 = 15;
    /// Offset of the `nTWI` field.
    pub const NTWI_SHIFT: u32 = 16;
    /// Offset of the `nTWE` field.
    pub const NTWE_SHIFT: u32 = 18;
    /// Offset of the `WXN` field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the `IESB` field.
    pub const IESB_SHIFT: u32 = 21;
    /// Offset of the `EIS` field.
    pub const EIS_SHIFT: u32 = 22;
    /// Offset of the `SPAN` field.
    pub const SPAN_SHIFT: u32 = 23;
    /// Offset of the `UCI` field.
    pub const UCI_SHIFT: u32 = 26;
    /// Offset of the `EnDA` field.
    pub const ENDA_SHIFT: u32 = 27;
    /// Offset of the `nTLSMD` field.
    pub const NTLSMD_SHIFT: u32 = 28;
    /// Offset of the `LSMAOE` field.
    pub const LSMAOE_SHIFT: u32 = 29;
    /// Offset of the `EnIB` field.
    pub const ENIB_SHIFT: u32 = 30;
    /// Offset of the `EnIA` field.
    pub const ENIA_SHIFT: u32 = 31;
    /// Offset of the `CMOW` field.
    pub const CMOW_SHIFT: u32 = 32;
    /// Offset of the `MSCEn` field.
    pub const MSCEN_SHIFT: u32 = 33;
    /// Offset of the `EnFPM` field.
    pub const ENFPM_SHIFT: u32 = 34;
    /// Offset of the `BT0` field.
    pub const BT0_SHIFT: u32 = 35;
    /// Offset of the `BT` field.
    pub const BT_SHIFT: u32 = 36;
    /// Offset of the `ITFSB` field.
    pub const ITFSB_SHIFT: u32 = 37;
    /// Offset of the `TCF0` field.
    pub const TCF0_SHIFT: u32 = 38;
    /// Mask for the `TCF0` field.
    pub const TCF0_MASK: u64 = 0b11;
    /// Offset of the `TCF` field.
    pub const TCF_SHIFT: u32 = 40;
    /// Mask for the `TCF` field.
    pub const TCF_MASK: u64 = 0b11;
    /// Offset of the `ATA0` field.
    pub const ATA0_SHIFT: u32 = 42;
    /// Offset of the `ATA` field.
    pub const ATA_SHIFT: u32 = 43;
    /// Offset of the `DSSBS` field.
    pub const DSSBS_SHIFT: u32 = 44;
    /// Offset of the `TWEDEn` field.
    pub const TWEDEN_SHIFT: u32 = 45;
    /// Offset of the `TWEDEL` field.
    pub const TWEDEL_SHIFT: u32 = 46;
    /// Mask for the `TWEDEL` field.
    pub const TWEDEL_MASK: u64 = 0b1111;
    /// Offset of the `EnASR` field.
    pub const ENASR_SHIFT: u32 = 54;
    /// Offset of the `EnAS0` field.
    pub const ENAS0_SHIFT: u32 = 55;
    /// Offset of the `EnALS` field.
    pub const ENALS_SHIFT: u32 = 56;
    /// Offset of the `EPAN` field.
    pub const EPAN_SHIFT: u32 = 57;
    /// Offset of the `TCSO0` field.
    pub const TCSO0_SHIFT: u32 = 58;
    /// Offset of the `TCSO` field.
    pub const TCSO_SHIFT: u32 = 59;
    /// Offset of the `EnTP2` field.
    pub const ENTP2_SHIFT: u32 = 60;
    /// Offset of the `NMI` field.
    pub const NMI_SHIFT: u32 = 61;
    /// Offset of the `SPINTMASK` field.
    pub const SPINTMASK_SHIFT: u32 = 62;
    /// Offset of the `TIDCP` field.
    pub const TIDCP_SHIFT: u32 = 63;

    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        ((self.bits() >> Self::TCF0_SHIFT) & Self::TCF0_MASK) as u8
    }

    /// Sets the value of the `TCF0` field.
    pub const fn set_tcf0(&mut self, value: u8) {
        let offset = Self::TCF0_SHIFT;
        assert!(value & (Self::TCF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TCF0` field set to the given value.
    pub const fn with_tcf0(mut self, value: u8) -> Self {
        self.set_tcf0(value);
        self
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        ((self.bits() >> Self::TCF_SHIFT) & Self::TCF_MASK) as u8
    }

    /// Sets the value of the `TCF` field.
    pub const fn set_tcf(&mut self, value: u8) {
        let offset = Self::TCF_SHIFT;
        assert!(value & (Self::TCF_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TCF` field set to the given value.
    pub const fn with_tcf(mut self, value: u8) -> Self {
        self.set_tcf(value);
        self
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> Self::TWEDEL_SHIFT) & Self::TWEDEL_MASK) as u8
    }

    /// Sets the value of the `TWEDEL` field.
    pub const fn set_twedel(&mut self, value: u8) {
        let offset = Self::TWEDEL_SHIFT;
        assert!(value & (Self::TWEDEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWEDEL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TWEDEL` field set to the given value.
    pub const fn with_twedel(mut self, value: u8) -> Self {
        self.set_twedel(value);
        self
    }
}

bitflags! {
    /// `SPSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        /// `EXLOCK` bit.
        const EXLOCK = 1 << 34;
        /// `PACM` bit.
        const PACM = 1 << 35;
        /// `UINJ` bit.
        const UINJ = 1 << 36;
    }
}

impl SpsrEl2 {
    /// Offset of the `M[3:0]` field.
    pub const M_3_0_SHIFT: u32 = 0;
    /// Mask for the `M[3:0]` field.
    pub const M_3_0_MASK: u64 = 0b1111;
    /// Offset of the `M[4]` field.
    pub const M_4_SHIFT: u32 = 4;
    /// Offset of the `T` field.
    pub const T_SHIFT: u32 = 5;
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 9;
    /// Offset of the `E` field.
    pub const E_SHIFT: u32 = 9;
    /// Offset of the `BTYPE` field.
    pub const BTYPE_SHIFT: u32 = 10;
    /// Mask for the `BTYPE` field.
    pub const BTYPE_MASK: u64 = 0b11;
    /// Offset of the `ALLINT` field.
    pub const ALLINT_SHIFT: u32 = 13;
    /// Offset of the `BTYPE2` field.
    pub const BTYPE2_SHIFT: u32 = 14;
    /// Offset of the `GE` field.
    pub const GE_SHIFT: u32 = 16;
    /// Mask for the `GE` field.
    pub const GE_MASK: u64 = 0b1111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 20;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 21;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 22;
    /// Offset of the `UAO` field.
    pub const UAO_SHIFT: u32 = 23;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
    /// Offset of the `TCO` field.
    pub const TCO_SHIFT: u32 = 25;
    /// Offset of the `Q` field.
    pub const Q_SHIFT: u32 = 27;
    /// Offset of the `V` field.
    pub const V_SHIFT: u32 = 28;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 29;
    /// Offset of the `Z` field.
    pub const Z_SHIFT: u32 = 30;
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 31;
    /// Offset of the `PM` field.
    pub const PM_SHIFT: u32 = 32;
    /// Offset of the `EXLOCK` field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the `PACM` field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> Self::M_3_0_SHIFT) & Self::M_3_0_MASK) as u8
    }

    /// Sets the value of the `M[3:0]` field.
    pub const fn set_m_3_0(&mut self, value: u8) {
        let offset = Self::M_3_0_SHIFT;
        assert!(value & (Self::M_3_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::M_3_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `M[3:0]` field set to the given value.
    pub const fn with_m_3_0(mut self, value: u8) -> Self {
        self.set_m_3_0(value);
        self
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> Self::BTYPE_SHIFT) & Self::BTYPE_MASK) as u8
    }

    /// Sets the value of the `BTYPE` field.
    pub const fn set_btype(&mut self, value: u8) {
        let offset = Self::BTYPE_SHIFT;
        assert!(value & (Self::BTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BTYPE` field set to the given value.
    pub const fn with_btype(mut self, value: u8) -> Self {
        self.set_btype(value);
        self
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> Self::GE_SHIFT) & Self::GE_MASK) as u8
    }

    /// Sets the value of the `GE` field.
    pub const fn set_ge(&mut self, value: u8) {
        let offset = Self::GE_SHIFT;
        assert!(value & (Self::GE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `GE` field set to the given value.
    pub const fn with_ge(mut self, value: u8) -> Self {
        self.set_ge(value);
        self
    }
}

bitflags! {
    /// `SP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl2: u64 {
    }
}

impl SpEl2 {
    /// Offset of the `StackPointer` field.
    pub const STACKPOINTER_SHIFT: u32 = 0;
    /// Mask for the `StackPointer` field.
    pub const STACKPOINTER_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        (self.bits() >> Self::STACKPOINTER_SHIFT) & Self::STACKPOINTER_MASK
    }

    /// Sets the value of the `StackPointer` field.
    pub const fn set_stackpointer(&mut self, value: u64) {
        let offset = Self::STACKPOINTER_SHIFT;
        assert!(value & Self::STACKPOINTER_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STACKPOINTER_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `StackPointer` field set to the given value.
    pub const fn with_stackpointer(mut self, value: u64) -> Self {
        self.set_stackpointer(value);
        self
    }
}

bitflags! {
    /// `TCR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `PnCH` field.
    pub const PNCH_SHIFT: u32 = 0;
    /// Offset of the `PIE` field.
    pub const PIE_SHIFT: u32 = 1;
    /// Offset of the `E0POE` field.
    pub const E0POE_SHIFT: u32 = 2;
    /// Offset of the `POE` field.
    pub const POE_SHIFT: u32 = 3;
    /// Offset of the `AIE` field.
    pub const AIE_SHIFT: u32 = 4;
    /// Offset of the `D128` field.
    pub const D128_SHIFT: u32 = 5;
    /// Offset of the `PTTWI` field.
    pub const PTTWI_SHIFT: u32 = 10;
    /// Offset of the `HAFT` field.
    pub const HAFT_SHIFT: u32 = 11;
    /// Offset of the `AMEC0` field.
    pub const AMEC0_SHIFT: u32 = 12;
    /// Offset of the `AMEC1` field.
    pub const AMEC1_SHIFT: u32 = 13;
    /// Offset of the `DisCH0` field.
    pub const DISCH0_SHIFT: u32 = 14;
    /// Offset of the `DisCH1` field.
    pub const DISCH1_SHIFT: u32 = 15;
    /// Offset of the `A2` field.
    pub const A2_SHIFT: u32 = 16;
    /// Offset of the `FNG0` field.
    pub const FNG0_SHIFT: u32 = 17;
    /// Offset of the `FNG1` field.
    pub const FNG1_SHIFT: u32 = 18;
    /// Offset of the `POE2F` field.
    pub const POE2F_SHIFT: u32 = 19;
    /// Offset of the `POIW` field.
    pub const POIW_SHIFT: u32 = 22;
    /// Mask for the `POIW` field.
    pub const POIW_MASK: u64 = 0b111;
    /// Offset of the `VTB0` field.
    pub const VTB0_SHIFT: u32 = 25;
    /// Mask for the `VTB0` field.
    pub const VTB0_MASK: u64 = 0b1_1111;
    /// Offset of the `VTB1` field.
    pub const VTB1_SHIFT: u32 = 30;
    /// Mask for the `VTB1` field.
    pub const VTB1_MASK: u64 = 0b1_1111;
    /// Offset of the `TVAD0` field.
    pub const TVAD0_SHIFT: u32 = 35;
    /// Offset of the `TVAD1` field.
    pub const TVAD1_SHIFT: u32 = 36;

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> Self::POIW_SHIFT) & Self::POIW_MASK) as u8
    }

    /// Sets the value of the `POIW` field.
    pub const fn set_poiw(&mut self, value: u8) {
        let offset = Self::POIW_SHIFT;
        assert!(value & (Self::POIW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::POIW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `POIW` field set to the given value.
    pub const fn with_poiw(mut self, value: u8) -> Self {
        self.set_poiw(value);
        self
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        ((self.bits() >> Self::VTB0_SHIFT) & Self::VTB0_MASK) as u8
    }

    /// Sets the value of the `VTB0` field.
    pub const fn set_vtb0(&mut self, value: u8) {
        let offset = Self::VTB0_SHIFT;
        assert!(value & (Self::VTB0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VTB0` field set to the given value.
    pub const fn with_vtb0(mut self, value: u8) -> Self {
        self.set_vtb0(value);
        self
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        ((self.bits() >> Self::VTB1_SHIFT) & Self::VTB1_MASK) as u8
    }

    /// Sets the value of the `VTB1` field.
    pub const fn set_vtb1(&mut self, value: u8) {
        let offset = Self::VTB1_SHIFT;
        assert!(value & (Self::VTB1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VTB1` field set to the given value.
    pub const fn with_vtb1(mut self, value: u8) -> Self {
        self.set_vtb1(value);
        self
    }
}

bitflags! {
    /// `TCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrEl2: u64 {
        /// RES1 bits in the `TCR_EL2` register.
        const RES1 = 0b1000_0000_1000_0000_0000_0000_0000_0000;
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
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b11_1111;
    /// Offset of the `EPD0` field.
    pub const EPD0_SHIFT: u32 = 7;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u64 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u64 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u64 = 0b11;
    /// Offset of the `TG0` field.
    pub const TG0_SHIFT: u32 = 14;
    /// Mask for the `TG0` field.
    pub const TG0_MASK: u64 = 0b11;
    /// Offset of the `PS` field.
    pub const PS_SHIFT: u32 = 16;
    /// Mask for the `PS` field.
    pub const PS_MASK: u64 = 0b111;
    /// Offset of the `T1SZ` field.
    pub const T1SZ_SHIFT: u32 = 16;
    /// Mask for the `T1SZ` field.
    pub const T1SZ_MASK: u64 = 0b11_1111;
    /// Offset of the `TBI` field.
    pub const TBI_SHIFT: u32 = 20;
    /// Offset of the `A1` field.
    pub const A1_SHIFT: u32 = 22;
    /// Offset of the `EPD1` field.
    pub const EPD1_SHIFT: u32 = 23;
    /// Offset of the `HPD` field.
    pub const HPD_SHIFT: u32 = 24;
    /// Offset of the `IRGN1` field.
    pub const IRGN1_SHIFT: u32 = 24;
    /// Mask for the `IRGN1` field.
    pub const IRGN1_MASK: u64 = 0b11;
    /// Offset of the `HWU59` field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the `HWU60` field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the `ORGN1` field.
    pub const ORGN1_SHIFT: u32 = 26;
    /// Mask for the `ORGN1` field.
    pub const ORGN1_MASK: u64 = 0b11;
    /// Offset of the `HWU61` field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the `HWU62` field.
    pub const HWU62_SHIFT: u32 = 28;
    /// Offset of the `SH1` field.
    pub const SH1_SHIFT: u32 = 28;
    /// Mask for the `SH1` field.
    pub const SH1_MASK: u64 = 0b11;
    /// Offset of the `TBID` field.
    pub const TBID_SHIFT: u32 = 29;
    /// Offset of the `TCMA` field.
    pub const TCMA_SHIFT: u32 = 30;
    /// Offset of the `TG1` field.
    pub const TG1_SHIFT: u32 = 30;
    /// Mask for the `TG1` field.
    pub const TG1_MASK: u64 = 0b11;
    /// Offset of the `IPS` field.
    pub const IPS_SHIFT: u32 = 32;
    /// Mask for the `IPS` field.
    pub const IPS_MASK: u64 = 0b111;
    /// Offset of the `MTX` field.
    pub const MTX_SHIFT: u32 = 33;
    /// Offset of the `AS` field.
    pub const AS_SHIFT: u32 = 36;
    /// Offset of the `TBI0` field.
    pub const TBI0_SHIFT: u32 = 37;
    /// Offset of the `TBI1` field.
    pub const TBI1_SHIFT: u32 = 38;
    /// Offset of the `HPD0` field.
    pub const HPD0_SHIFT: u32 = 41;
    /// Offset of the `HPD1` field.
    pub const HPD1_SHIFT: u32 = 42;
    /// Offset of the `HWU059` field.
    pub const HWU059_SHIFT: u32 = 43;
    /// Offset of the `HWU060` field.
    pub const HWU060_SHIFT: u32 = 44;
    /// Offset of the `HWU061` field.
    pub const HWU061_SHIFT: u32 = 45;
    /// Offset of the `HWU062` field.
    pub const HWU062_SHIFT: u32 = 46;
    /// Offset of the `HWU159` field.
    pub const HWU159_SHIFT: u32 = 47;
    /// Offset of the `HWU160` field.
    pub const HWU160_SHIFT: u32 = 48;
    /// Offset of the `HWU161` field.
    pub const HWU161_SHIFT: u32 = 49;
    /// Offset of the `HWU162` field.
    pub const HWU162_SHIFT: u32 = 50;
    /// Offset of the `TBID0` field.
    pub const TBID0_SHIFT: u32 = 51;
    /// Offset of the `TBID1` field.
    pub const TBID1_SHIFT: u32 = 52;
    /// Offset of the `NFD0` field.
    pub const NFD0_SHIFT: u32 = 53;
    /// Offset of the `NFD1` field.
    pub const NFD1_SHIFT: u32 = 54;
    /// Offset of the `E0PD0` field.
    pub const E0PD0_SHIFT: u32 = 55;
    /// Offset of the `E0PD1` field.
    pub const E0PD1_SHIFT: u32 = 56;
    /// Offset of the `TCMA0` field.
    pub const TCMA0_SHIFT: u32 = 57;
    /// Offset of the `TCMA1` field.
    pub const TCMA1_SHIFT: u32 = 58;
    /// Offset of the `MTX0` field.
    pub const MTX0_SHIFT: u32 = 60;
    /// Offset of the `MTX1` field.
    pub const MTX1_SHIFT: u32 = 61;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & Self::T0SZ_MASK) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `T0SZ` field set to the given value.
    pub const fn with_t0sz(mut self, value: u8) -> Self {
        self.set_t0sz(value);
        self
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & Self::IRGN0_MASK) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IRGN0` field set to the given value.
    pub const fn with_irgn0(mut self, value: u8) -> Self {
        self.set_irgn0(value);
        self
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & Self::ORGN0_MASK) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ORGN0` field set to the given value.
    pub const fn with_orgn0(mut self, value: u8) -> Self {
        self.set_orgn0(value);
        self
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & Self::SH0_MASK) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SH0` field set to the given value.
    pub const fn with_sh0(mut self, value: u8) -> Self {
        self.set_sh0(value);
        self
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> Self::TG0_SHIFT) & Self::TG0_MASK) as u8
    }

    /// Sets the value of the `TG0` field.
    pub const fn set_tg0(&mut self, value: u8) {
        let offset = Self::TG0_SHIFT;
        assert!(value & (Self::TG0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TG0` field set to the given value.
    pub const fn with_tg0(mut self, value: u8) -> Self {
        self.set_tg0(value);
        self
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> Self::PS_SHIFT) & Self::PS_MASK) as u8
    }

    /// Sets the value of the `PS` field.
    pub const fn set_ps(&mut self, value: u8) {
        let offset = Self::PS_SHIFT;
        assert!(value & (Self::PS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PS` field set to the given value.
    pub const fn with_ps(mut self, value: u8) -> Self {
        self.set_ps(value);
        self
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        ((self.bits() >> Self::T1SZ_SHIFT) & Self::T1SZ_MASK) as u8
    }

    /// Sets the value of the `T1SZ` field.
    pub const fn set_t1sz(&mut self, value: u8) {
        let offset = Self::T1SZ_SHIFT;
        assert!(value & (Self::T1SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T1SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `T1SZ` field set to the given value.
    pub const fn with_t1sz(mut self, value: u8) -> Self {
        self.set_t1sz(value);
        self
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        ((self.bits() >> Self::IRGN1_SHIFT) & Self::IRGN1_MASK) as u8
    }

    /// Sets the value of the `IRGN1` field.
    pub const fn set_irgn1(&mut self, value: u8) {
        let offset = Self::IRGN1_SHIFT;
        assert!(value & (Self::IRGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IRGN1` field set to the given value.
    pub const fn with_irgn1(mut self, value: u8) -> Self {
        self.set_irgn1(value);
        self
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        ((self.bits() >> Self::ORGN1_SHIFT) & Self::ORGN1_MASK) as u8
    }

    /// Sets the value of the `ORGN1` field.
    pub const fn set_orgn1(&mut self, value: u8) {
        let offset = Self::ORGN1_SHIFT;
        assert!(value & (Self::ORGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ORGN1` field set to the given value.
    pub const fn with_orgn1(mut self, value: u8) -> Self {
        self.set_orgn1(value);
        self
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        ((self.bits() >> Self::SH1_SHIFT) & Self::SH1_MASK) as u8
    }

    /// Sets the value of the `SH1` field.
    pub const fn set_sh1(&mut self, value: u8) {
        let offset = Self::SH1_SHIFT;
        assert!(value & (Self::SH1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SH1` field set to the given value.
    pub const fn with_sh1(mut self, value: u8) -> Self {
        self.set_sh1(value);
        self
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        ((self.bits() >> Self::TG1_SHIFT) & Self::TG1_MASK) as u8
    }

    /// Sets the value of the `TG1` field.
    pub const fn set_tg1(&mut self, value: u8) {
        let offset = Self::TG1_SHIFT;
        assert!(value & (Self::TG1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TG1` field set to the given value.
    pub const fn with_tg1(mut self, value: u8) -> Self {
        self.set_tg1(value);
        self
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        ((self.bits() >> Self::IPS_SHIFT) & Self::IPS_MASK) as u8
    }

    /// Sets the value of the `IPS` field.
    pub const fn set_ips(&mut self, value: u8) {
        let offset = Self::IPS_SHIFT;
        assert!(value & (Self::IPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IPS` field set to the given value.
    pub const fn with_ips(mut self, value: u8) -> Self {
        self.set_ips(value);
        self
    }
}

bitflags! {
    /// `TFSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TfsrEl2: u64 {
        /// `TF0` bit.
        const TF0 = 1 << 0;
        /// `TF1` bit.
        const TF1 = 1 << 1;
    }
}

impl TfsrEl2 {
    /// Offset of the `TF0` field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the `TF1` field.
    pub const TF1_SHIFT: u32 = 1;
}

bitflags! {
    /// `TPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl2: u64 {
    }
}

impl TpidrEl2 {
    /// Offset of the `ThreadID` field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the `ThreadID` field.
    pub const THREADID_MASK: u64 =
        0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> Self::THREADID_SHIFT) & Self::THREADID_MASK
    }

    /// Sets the value of the `ThreadID` field.
    pub const fn set_threadid(&mut self, value: u64) {
        let offset = Self::THREADID_SHIFT;
        assert!(value & Self::THREADID_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THREADID_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `ThreadID` field set to the given value.
    pub const fn with_threadid(mut self, value: u64) -> Self {
        self.set_threadid(value);
        self
    }
}

bitflags! {
    /// `TTBR0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr0El2 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR[47:1]` field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the `BADDR[47:1]` field.
    pub const BADDR_47_1_MASK: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> Self::BADDR_47_1_SHIFT) & Self::BADDR_47_1_MASK
    }

    /// Sets the value of the `BADDR[47:1]` field.
    pub const fn set_baddr_47_1(&mut self, value: u64) {
        let offset = Self::BADDR_47_1_SHIFT;
        assert!(value & Self::BADDR_47_1_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_47_1_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `BADDR[47:1]` field set to the given value.
    pub const fn with_baddr_47_1(mut self, value: u64) -> Self {
        self.set_baddr_47_1(value);
        self
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & Self::SKL_MASK) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SKL` field set to the given value.
    pub const fn with_skl(mut self, value: u8) -> Self {
        self.set_skl(value);
        self
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> Self::ASID_SHIFT) & Self::ASID_MASK) as u16
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u16) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ASID` field set to the given value.
    pub const fn with_asid(mut self, value: u16) -> Self {
        self.set_asid(value);
        self
    }
}

/// `TTBR1_EL2` system register value.
pub type Ttbr1El2 = Ttbr0El2;

bitflags! {
    /// `VBAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl2: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

impl VbarEl2 {
    /// Offset of the `UT` field.
    pub const UT_SHIFT: u32 = 0;
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 11;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u64 = 0b1_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        (self.bits() >> Self::VBA_SHIFT) & Self::VBA_MASK
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u64) {
        let offset = Self::VBA_SHIFT;
        assert!(value & Self::VBA_MASK == value);
        *self =
            Self::from_bits_retain((self.bits() & !(Self::VBA_MASK << offset)) | (value << offset));
    }

    /// Returns a copy with the `VBA` field set to the given value.
    pub const fn with_vba(mut self, value: u64) -> Self {
        self.set_vba(value);
        self
    }
}

bitflags! {
    /// `VDISR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b1111_1111_1111_1111_1111_1111;
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u64 = 0b11_1111;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u64 = 0b11;
    /// Offset of the `IDS` field.
    pub const IDS_SHIFT: u32 = 24;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & Self::ISS_MASK) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ISS` field set to the given value.
    pub const fn with_iss(mut self, value: u32) -> Self {
        self.set_iss(value);
        self
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & Self::STATUS_MASK) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `STATUS` field set to the given value.
    pub const fn with_status(mut self, value: u8) -> Self {
        self.set_status(value);
        self
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & Self::AET_MASK) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AET` field set to the given value.
    pub const fn with_aet(mut self, value: u8) -> Self {
        self.set_aet(value);
        self
    }
}

bitflags! {
    /// `VMPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VmpidrEl2: u64 {
        /// RES1 bits in the `VMPIDR_EL2` register.
        const RES1 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
    }
}

impl VmpidrEl2 {
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u64 = 0b1111_1111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u64 = 0b1111_1111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u64 = 0b1111_1111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `Aff3` field.
    pub const AFF3_SHIFT: u32 = 32;
    /// Mask for the `Aff3` field.
    pub const AFF3_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & Self::AFF0_MASK) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff0` field set to the given value.
    pub const fn with_aff0(mut self, value: u8) -> Self {
        self.set_aff0(value);
        self
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> Self::AFF1_SHIFT) & Self::AFF1_MASK) as u8
    }

    /// Sets the value of the `Aff1` field.
    pub const fn set_aff1(&mut self, value: u8) {
        let offset = Self::AFF1_SHIFT;
        assert!(value & (Self::AFF1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff1` field set to the given value.
    pub const fn with_aff1(mut self, value: u8) -> Self {
        self.set_aff1(value);
        self
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> Self::AFF2_SHIFT) & Self::AFF2_MASK) as u8
    }

    /// Sets the value of the `Aff2` field.
    pub const fn set_aff2(&mut self, value: u8) {
        let offset = Self::AFF2_SHIFT;
        assert!(value & (Self::AFF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff2` field set to the given value.
    pub const fn with_aff2(mut self, value: u8) -> Self {
        self.set_aff2(value);
        self
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        ((self.bits() >> Self::AFF3_SHIFT) & Self::AFF3_MASK) as u8
    }

    /// Sets the value of the `Aff3` field.
    pub const fn set_aff3(&mut self, value: u8) {
        let offset = Self::AFF3_SHIFT;
        assert!(value & (Self::AFF3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Aff3` field set to the given value.
    pub const fn with_aff3(mut self, value: u8) -> Self {
        self.set_aff3(value);
        self
    }
}

bitflags! {
    /// `VPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VpidrEl2: u64 {
    }
}

impl VpidrEl2 {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u64 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u64 = 0b1111_1111_1111;
    /// Offset of the `Architecture` field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the `Architecture` field.
    pub const ARCHITECTURE_MASK: u64 = 0b1111;
    /// Offset of the `Variant` field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the `Variant` field.
    pub const VARIANT_MASK: u64 = 0b1111;
    /// Offset of the `Implementer` field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the `Implementer` field.
    pub const IMPLEMENTER_MASK: u64 = 0b1111_1111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & Self::REVISION_MASK) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Revision` field set to the given value.
    pub const fn with_revision(mut self, value: u8) -> Self {
        self.set_revision(value);
        self
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> Self::PARTNUM_SHIFT) & Self::PARTNUM_MASK) as u16
    }

    /// Sets the value of the `PartNum` field.
    pub const fn set_partnum(&mut self, value: u16) {
        let offset = Self::PARTNUM_SHIFT;
        assert!(value & (Self::PARTNUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PartNum` field set to the given value.
    pub const fn with_partnum(mut self, value: u16) -> Self {
        self.set_partnum(value);
        self
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> Self::ARCHITECTURE_SHIFT) & Self::ARCHITECTURE_MASK) as u8
    }

    /// Sets the value of the `Architecture` field.
    pub const fn set_architecture(&mut self, value: u8) {
        let offset = Self::ARCHITECTURE_SHIFT;
        assert!(value & (Self::ARCHITECTURE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Architecture` field set to the given value.
    pub const fn with_architecture(mut self, value: u8) -> Self {
        self.set_architecture(value);
        self
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> Self::VARIANT_SHIFT) & Self::VARIANT_MASK) as u8
    }

    /// Sets the value of the `Variant` field.
    pub const fn set_variant(&mut self, value: u8) {
        let offset = Self::VARIANT_SHIFT;
        assert!(value & (Self::VARIANT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Variant` field set to the given value.
    pub const fn with_variant(mut self, value: u8) -> Self {
        self.set_variant(value);
        self
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> Self::IMPLEMENTER_SHIFT) & Self::IMPLEMENTER_MASK) as u8
    }

    /// Sets the value of the `Implementer` field.
    pub const fn set_implementer(&mut self, value: u8) {
        let offset = Self::IMPLEMENTER_SHIFT;
        assert!(value & (Self::IMPLEMENTER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `Implementer` field set to the given value.
    pub const fn with_implementer(mut self, value: u8) -> Self {
        self.set_implementer(value);
        self
    }
}

bitflags! {
    /// `VSESR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VsesrEl2: u64 {
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `IDS` bit.
        const IDS = 1 << 24;
    }
}

impl VsesrEl2 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b1111_1111_1111_1111_1111_1111;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u64 = 0b11;
    /// Offset of the `IDS` field.
    pub const IDS_SHIFT: u32 = 24;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & Self::ISS_MASK) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ISS` field set to the given value.
    pub const fn with_iss(mut self, value: u32) -> Self {
        self.set_iss(value);
        self
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & Self::AET_MASK) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `AET` field set to the given value.
    pub const fn with_aet(mut self, value: u8) -> Self {
        self.set_aet(value);
        self
    }
}

bitflags! {
    /// `VTCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VtcrEl2: u64 {
        /// RES1 bits in the `VTCR_EL2` register.
        const RES1 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
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
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b11_1111;
    /// Offset of the `SL0` field.
    pub const SL0_SHIFT: u32 = 6;
    /// Mask for the `SL0` field.
    pub const SL0_MASK: u64 = 0b11;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u64 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u64 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u64 = 0b11;
    /// Offset of the `TG0` field.
    pub const TG0_SHIFT: u32 = 14;
    /// Mask for the `TG0` field.
    pub const TG0_MASK: u64 = 0b11;
    /// Offset of the `PS` field.
    pub const PS_SHIFT: u32 = 16;
    /// Mask for the `PS` field.
    pub const PS_MASK: u64 = 0b111;
    /// Offset of the `VS` field.
    pub const VS_SHIFT: u32 = 19;
    /// Offset of the `HA` field.
    pub const HA_SHIFT: u32 = 21;
    /// Offset of the `HD` field.
    pub const HD_SHIFT: u32 = 22;
    /// Offset of the `HWU59` field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the `HWU60` field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the `HWU61` field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the `HWU62` field.
    pub const HWU62_SHIFT: u32 = 28;
    /// Offset of the `NSW` field.
    pub const NSW_SHIFT: u32 = 29;
    /// Offset of the `NSA` field.
    pub const NSA_SHIFT: u32 = 30;
    /// Offset of the `DS` field.
    pub const DS_SHIFT: u32 = 32;
    /// Offset of the `SL2` field.
    pub const SL2_SHIFT: u32 = 33;
    /// Offset of the `AssuredOnly` field.
    pub const ASSUREDONLY_SHIFT: u32 = 34;
    /// Offset of the `TL1` field.
    pub const TL1_SHIFT: u32 = 35;
    /// Offset of the `S2PIE` field.
    pub const S2PIE_SHIFT: u32 = 36;
    /// Offset of the `S2POE` field.
    pub const S2POE_SHIFT: u32 = 37;
    /// Offset of the `D128` field.
    pub const D128_SHIFT: u32 = 38;
    /// Offset of the `GCSH` field.
    pub const GCSH_SHIFT: u32 = 40;
    /// Offset of the `TL0` field.
    pub const TL0_SHIFT: u32 = 41;
    /// Offset of the `HAFT` field.
    pub const HAFT_SHIFT: u32 = 44;
    /// Offset of the `HDBSS` field.
    pub const HDBSS_SHIFT: u32 = 45;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & Self::T0SZ_MASK) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `T0SZ` field set to the given value.
    pub const fn with_t0sz(mut self, value: u8) -> Self {
        self.set_t0sz(value);
        self
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        ((self.bits() >> Self::SL0_SHIFT) & Self::SL0_MASK) as u8
    }

    /// Sets the value of the `SL0` field.
    pub const fn set_sl0(&mut self, value: u8) {
        let offset = Self::SL0_SHIFT;
        assert!(value & (Self::SL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SL0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SL0` field set to the given value.
    pub const fn with_sl0(mut self, value: u8) -> Self {
        self.set_sl0(value);
        self
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & Self::IRGN0_MASK) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IRGN0` field set to the given value.
    pub const fn with_irgn0(mut self, value: u8) -> Self {
        self.set_irgn0(value);
        self
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & Self::ORGN0_MASK) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ORGN0` field set to the given value.
    pub const fn with_orgn0(mut self, value: u8) -> Self {
        self.set_orgn0(value);
        self
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & Self::SH0_MASK) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SH0` field set to the given value.
    pub const fn with_sh0(mut self, value: u8) -> Self {
        self.set_sh0(value);
        self
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> Self::TG0_SHIFT) & Self::TG0_MASK) as u8
    }

    /// Sets the value of the `TG0` field.
    pub const fn set_tg0(&mut self, value: u8) {
        let offset = Self::TG0_SHIFT;
        assert!(value & (Self::TG0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TG0` field set to the given value.
    pub const fn with_tg0(mut self, value: u8) -> Self {
        self.set_tg0(value);
        self
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> Self::PS_SHIFT) & Self::PS_MASK) as u8
    }

    /// Sets the value of the `PS` field.
    pub const fn set_ps(&mut self, value: u8) {
        let offset = Self::PS_SHIFT;
        assert!(value & (Self::PS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PS` field set to the given value.
    pub const fn with_ps(mut self, value: u8) -> Self {
        self.set_ps(value);
        self
    }
}

bitflags! {
    /// `VTTBR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VttbrEl2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl VttbrEl2 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `VMID` field.
    pub const VMID_SHIFT: u32 = 48;
    /// Mask for the `VMID` field.
    pub const VMID_MASK: u64 = 0b1111_1111_1111_1111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> Self::BADDR_SHIFT) & Self::BADDR_MASK
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & Self::BADDR_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | (value << offset),
        );
    }

    /// Returns a copy with the `BADDR` field set to the given value.
    pub const fn with_baddr(mut self, value: u64) -> Self {
        self.set_baddr(value);
        self
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & Self::SKL_MASK) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SKL` field set to the given value.
    pub const fn with_skl(mut self, value: u8) -> Self {
        self.set_skl(value);
        self
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u16 {
        ((self.bits() >> Self::VMID_SHIFT) & Self::VMID_MASK) as u16
    }

    /// Sets the value of the `VMID` field.
    pub const fn set_vmid(&mut self, value: u16) {
        let offset = Self::VMID_SHIFT;
        assert!(value & (Self::VMID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VMID` field set to the given value.
    pub const fn with_vmid(mut self, value: u16) -> Self {
        self.set_vmid(value);
        self
    }
}
