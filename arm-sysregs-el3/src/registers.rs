// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Arm CPU system registers.

// This file is generated, do not edit manually.

use bitflags::bitflags;

bitflags! {
    /// `CPTR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl CptrEl3 {
    /// Offset of the `EZ` field.
    pub const EZ_SHIFT: u32 = 8;
    /// Offset of the `TFP` field.
    pub const TFP_SHIFT: u32 = 10;
    /// Offset of the `ESM` field.
    pub const ESM_SHIFT: u32 = 12;
    /// Offset of the `TTA` field.
    pub const TTA_SHIFT: u32 = 20;
    /// Offset of the `TAM` field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the `TCPAC` field.
    pub const TCPAC_SHIFT: u32 = 31;
}

bitflags! {
    /// `ESR_EL3` system register value.
    #[derive(Clone, Copy, Eq, Default, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl3: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

impl EsrEl3 {
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
    /// `FGWTE3_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fgwte3El3: u64 {
        /// `ACTLR_EL3` bit.
        const ACTLR_EL3 = 1 << 0;
        /// `AFSR0_EL3` bit.
        const AFSR0_EL3 = 1 << 1;
        /// `AFSR1_EL3` bit.
        const AFSR1_EL3 = 1 << 2;
        /// `AMAIR_EL3` bit.
        const AMAIR_EL3 = 1 << 3;
        /// `AMAIR2_EL3` bit.
        const AMAIR2_EL3 = 1 << 4;
        /// `GCSCR_EL3` bit.
        const GCSCR_EL3 = 1 << 5;
        /// `GCSPR_EL3` bit.
        const GCSPR_EL3 = 1 << 6;
        /// `GPCCR_EL3` bit.
        const GPCCR_EL3 = 1 << 7;
        /// `GPTBR_EL3` bit.
        const GPTBR_EL3 = 1 << 8;
        /// `MAIR_EL3` bit.
        const MAIR_EL3 = 1 << 9;
        /// `MAIR2_EL3` bit.
        const MAIR2_EL3 = 1 << 10;
        /// `MDCR_EL3` bit.
        const MDCR_EL3 = 1 << 11;
        /// `MECID_RL_A_EL3` bit.
        const MECID_RL_A_EL3 = 1 << 12;
        /// `MPAM3_EL3` bit.
        const MPAM3_EL3 = 1 << 13;
        /// `PIR_EL3` bit.
        const PIR_EL3 = 1 << 14;
        /// `SCTLR_EL3` bit.
        const SCTLR_EL3 = 1 << 15;
        /// `SCTLR2_EL3` bit.
        const SCTLR2_EL3 = 1 << 16;
        /// `SPMROOTCR_EL3` bit.
        const SPMROOTCR_EL3 = 1 << 17;
        /// `TCR_EL3` bit.
        const TCR_EL3 = 1 << 18;
        /// `TPIDR_EL3` bit.
        const TPIDR_EL3 = 1 << 19;
        /// `TTBR0_EL3` bit.
        const TTBR0_EL3 = 1 << 20;
        /// `VBAR_EL3` bit.
        const VBAR_EL3 = 1 << 21;
        /// `GPCBW_EL3` bit.
        const GPCBW_EL3 = 1 << 22;
    }
}

impl Fgwte3El3 {
    /// Offset of the `ACTLR_EL3` field.
    pub const ACTLR_EL3_SHIFT: u32 = 0;
    /// Offset of the `AFSR0_EL3` field.
    pub const AFSR0_EL3_SHIFT: u32 = 1;
    /// Offset of the `AFSR1_EL3` field.
    pub const AFSR1_EL3_SHIFT: u32 = 2;
    /// Offset of the `AMAIR_EL3` field.
    pub const AMAIR_EL3_SHIFT: u32 = 3;
    /// Offset of the `AMAIR2_EL3` field.
    pub const AMAIR2_EL3_SHIFT: u32 = 4;
    /// Offset of the `GCSCR_EL3` field.
    pub const GCSCR_EL3_SHIFT: u32 = 5;
    /// Offset of the `GCSPR_EL3` field.
    pub const GCSPR_EL3_SHIFT: u32 = 6;
    /// Offset of the `GPCCR_EL3` field.
    pub const GPCCR_EL3_SHIFT: u32 = 7;
    /// Offset of the `GPTBR_EL3` field.
    pub const GPTBR_EL3_SHIFT: u32 = 8;
    /// Offset of the `MAIR_EL3` field.
    pub const MAIR_EL3_SHIFT: u32 = 9;
    /// Offset of the `MAIR2_EL3` field.
    pub const MAIR2_EL3_SHIFT: u32 = 10;
    /// Offset of the `MDCR_EL3` field.
    pub const MDCR_EL3_SHIFT: u32 = 11;
    /// Offset of the `MECID_RL_A_EL3` field.
    pub const MECID_RL_A_EL3_SHIFT: u32 = 12;
    /// Offset of the `MPAM3_EL3` field.
    pub const MPAM3_EL3_SHIFT: u32 = 13;
    /// Offset of the `PIR_EL3` field.
    pub const PIR_EL3_SHIFT: u32 = 14;
    /// Offset of the `SCTLR_EL3` field.
    pub const SCTLR_EL3_SHIFT: u32 = 15;
    /// Offset of the `SCTLR2_EL3` field.
    pub const SCTLR2_EL3_SHIFT: u32 = 16;
    /// Offset of the `SPMROOTCR_EL3` field.
    pub const SPMROOTCR_EL3_SHIFT: u32 = 17;
    /// Offset of the `TCR_EL3` field.
    pub const TCR_EL3_SHIFT: u32 = 18;
    /// Offset of the `TPIDR_EL3` field.
    pub const TPIDR_EL3_SHIFT: u32 = 19;
    /// Offset of the `TTBR0_EL3` field.
    pub const TTBR0_EL3_SHIFT: u32 = 20;
    /// Offset of the `VBAR_EL3` field.
    pub const VBAR_EL3_SHIFT: u32 = 21;
    /// Offset of the `GPCBW_EL3` field.
    pub const GPCBW_EL3_SHIFT: u32 = 22;
}

bitflags! {
    /// `GPCCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GpccrEl3: u64 {
        /// `PPS3` bit.
        const PPS3 = 1 << 3;
        /// `RLPAD` bit.
        const RLPAD = 1 << 5;
        /// `NSPAD` bit.
        const NSPAD = 1 << 6;
        /// `SPAD` bit.
        const SPAD = 1 << 7;
        /// `GPC` bit.
        const GPC = 1 << 16;
        /// `GPCP` bit.
        const GPCP = 1 << 17;
        /// `TBGPCD` bit.
        const TBGPCD = 1 << 18;
        /// `NSO` bit.
        const NSO = 1 << 19;
        /// `APPSAA` bit.
        const APPSAA = 1 << 24;
        /// `SA` bit.
        const SA = 1 << 25;
        /// `NSP` bit.
        const NSP = 1 << 26;
        /// `NA6` bit.
        const NA6 = 1 << 27;
        /// `NA7` bit.
        const NA7 = 1 << 28;
        /// `GPCBW` bit.
        const GPCBW = 1 << 29;
    }
}

impl GpccrEl3 {
    /// Offset of the `PPS` field.
    pub const PPS_SHIFT: u32 = 0;
    /// Mask for the `PPS` field.
    pub const PPS_MASK: u64 = 0b111;
    /// Offset of the `PPS3` field.
    pub const PPS3_SHIFT: u32 = 3;
    /// Offset of the `RLPAD` field.
    pub const RLPAD_SHIFT: u32 = 5;
    /// Offset of the `NSPAD` field.
    pub const NSPAD_SHIFT: u32 = 6;
    /// Offset of the `SPAD` field.
    pub const SPAD_SHIFT: u32 = 7;
    /// Offset of the `IRGN` field.
    pub const IRGN_SHIFT: u32 = 8;
    /// Mask for the `IRGN` field.
    pub const IRGN_MASK: u64 = 0b11;
    /// Offset of the `ORGN` field.
    pub const ORGN_SHIFT: u32 = 10;
    /// Mask for the `ORGN` field.
    pub const ORGN_MASK: u64 = 0b11;
    /// Offset of the `SH` field.
    pub const SH_SHIFT: u32 = 12;
    /// Mask for the `SH` field.
    pub const SH_MASK: u64 = 0b11;
    /// Offset of the `PGS` field.
    pub const PGS_SHIFT: u32 = 14;
    /// Mask for the `PGS` field.
    pub const PGS_MASK: u64 = 0b11;
    /// Offset of the `GPC` field.
    pub const GPC_SHIFT: u32 = 16;
    /// Offset of the `GPCP` field.
    pub const GPCP_SHIFT: u32 = 17;
    /// Offset of the `TBGPCD` field.
    pub const TBGPCD_SHIFT: u32 = 18;
    /// Offset of the `NSO` field.
    pub const NSO_SHIFT: u32 = 19;
    /// Offset of the `L0GPTSZ` field.
    pub const L0GPTSZ_SHIFT: u32 = 20;
    /// Mask for the `L0GPTSZ` field.
    pub const L0GPTSZ_MASK: u64 = 0b1111;
    /// Offset of the `APPSAA` field.
    pub const APPSAA_SHIFT: u32 = 24;
    /// Offset of the `SA` field.
    pub const SA_SHIFT: u32 = 25;
    /// Offset of the `NSP` field.
    pub const NSP_SHIFT: u32 = 26;
    /// Offset of the `NA6` field.
    pub const NA6_SHIFT: u32 = 27;
    /// Offset of the `NA7` field.
    pub const NA7_SHIFT: u32 = 28;
    /// Offset of the `GPCBW` field.
    pub const GPCBW_SHIFT: u32 = 29;

    /// Returns the value of the `PPS` field.
    pub const fn pps(self) -> u8 {
        ((self.bits() >> Self::PPS_SHIFT) & Self::PPS_MASK) as u8
    }

    /// Sets the value of the `PPS` field.
    pub const fn set_pps(&mut self, value: u8) {
        let offset = Self::PPS_SHIFT;
        assert!(value & (Self::PPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PPS` field set to the given value.
    pub const fn with_pps(mut self, value: u8) -> Self {
        self.set_pps(value);
        self
    }

    /// Returns the value of the `IRGN` field.
    pub fn irgn(self) -> arm_sysregs_common::types::Cacheability {
        arm_sysregs_common::types::Cacheability::try_from(
            ((self.bits() >> Self::IRGN_SHIFT) & Self::IRGN_MASK) as u8,
        )
        .unwrap()
    }

    /// Sets the value of the `IRGN` field.
    pub fn set_irgn(&mut self, value: arm_sysregs_common::types::Cacheability) {
        let offset = Self::IRGN_SHIFT;
        let value: u8 = value.into();
        assert!(value & (Self::IRGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IRGN` field set to the given value.
    pub fn with_irgn(mut self, value: arm_sysregs_common::types::Cacheability) -> Self {
        self.set_irgn(value);
        self
    }

    /// Returns the value of the `ORGN` field.
    pub fn orgn(self) -> arm_sysregs_common::types::Cacheability {
        arm_sysregs_common::types::Cacheability::try_from(
            ((self.bits() >> Self::ORGN_SHIFT) & Self::ORGN_MASK) as u8,
        )
        .unwrap()
    }

    /// Sets the value of the `ORGN` field.
    pub fn set_orgn(&mut self, value: arm_sysregs_common::types::Cacheability) {
        let offset = Self::ORGN_SHIFT;
        let value: u8 = value.into();
        assert!(value & (Self::ORGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ORGN` field set to the given value.
    pub fn with_orgn(mut self, value: arm_sysregs_common::types::Cacheability) -> Self {
        self.set_orgn(value);
        self
    }

    /// Returns the value of the `SH` field.
    pub fn sh(self) -> arm_sysregs_common::types::Shareability {
        arm_sysregs_common::types::Shareability::try_from(
            ((self.bits() >> Self::SH_SHIFT) & Self::SH_MASK) as u8,
        )
        .unwrap()
    }

    /// Sets the value of the `SH` field.
    pub fn set_sh(&mut self, value: arm_sysregs_common::types::Shareability) {
        let offset = Self::SH_SHIFT;
        let value: u8 = value.into();
        assert!(value & (Self::SH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SH` field set to the given value.
    pub fn with_sh(mut self, value: arm_sysregs_common::types::Shareability) -> Self {
        self.set_sh(value);
        self
    }

    /// Returns the value of the `PGS` field.
    pub const fn pgs(self) -> u8 {
        ((self.bits() >> Self::PGS_SHIFT) & Self::PGS_MASK) as u8
    }

    /// Sets the value of the `PGS` field.
    pub const fn set_pgs(&mut self, value: u8) {
        let offset = Self::PGS_SHIFT;
        assert!(value & (Self::PGS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PGS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PGS` field set to the given value.
    pub const fn with_pgs(mut self, value: u8) -> Self {
        self.set_pgs(value);
        self
    }

    /// Returns the value of the `L0GPTSZ` field.
    pub const fn l0gptsz(self) -> u8 {
        ((self.bits() >> Self::L0GPTSZ_SHIFT) & Self::L0GPTSZ_MASK) as u8
    }

    /// Sets the value of the `L0GPTSZ` field.
    pub const fn set_l0gptsz(&mut self, value: u8) {
        let offset = Self::L0GPTSZ_SHIFT;
        assert!(value & (Self::L0GPTSZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L0GPTSZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `L0GPTSZ` field set to the given value.
    pub const fn with_l0gptsz(mut self, value: u8) -> Self {
        self.set_l0gptsz(value);
        self
    }
}

bitflags! {
    /// `GPTBR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GptbrEl3: u64 {
    }
}

impl GptbrEl3 {
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 0;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Offset of the `BADDR[43:40]` field.
    pub const BADDR_43_40_SHIFT: u32 = 40;
    /// Mask for the `BADDR[43:40]` field.
    pub const BADDR_43_40_MASK: u64 = 0b1111;

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

    /// Returns the value of the `BADDR[43:40]` field.
    pub const fn baddr_43_40(self) -> u8 {
        ((self.bits() >> Self::BADDR_43_40_SHIFT) & Self::BADDR_43_40_MASK) as u8
    }

    /// Sets the value of the `BADDR[43:40]` field.
    pub const fn set_baddr_43_40(&mut self, value: u8) {
        let offset = Self::BADDR_43_40_SHIFT;
        assert!(value & (Self::BADDR_43_40_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_43_40_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `BADDR[43:40]` field set to the given value.
    pub const fn with_baddr_43_40(mut self, value: u8) -> Self {
        self.set_baddr_43_40(value);
        self
    }
}

bitflags! {
    /// `ICC_CTLR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccCtlrEl3: u64 {
        /// `CBPR_EL1S` bit.
        const CBPR_EL1S = 1 << 0;
        /// `CBPR_EL1NS` bit.
        const CBPR_EL1NS = 1 << 1;
        /// `EOImode_EL3` bit.
        const EOIMODE_EL3 = 1 << 2;
        /// `EOImode_EL1S` bit.
        const EOIMODE_EL1S = 1 << 3;
        /// `EOImode_EL1NS` bit.
        const EOIMODE_EL1NS = 1 << 4;
        /// `RM` bit.
        const RM = 1 << 5;
        /// `PMHE` bit.
        const PMHE = 1 << 6;
        /// `SEIS` bit.
        const SEIS = 1 << 14;
        /// `A3V` bit.
        const A3V = 1 << 15;
        /// `nDS` bit.
        const NDS = 1 << 17;
        /// `RSS` bit.
        const RSS = 1 << 18;
        /// `ExtRange` bit.
        const EXTRANGE = 1 << 19;
    }
}

impl IccCtlrEl3 {
    /// Offset of the `CBPR_EL1S` field.
    pub const CBPR_EL1S_SHIFT: u32 = 0;
    /// Offset of the `CBPR_EL1NS` field.
    pub const CBPR_EL1NS_SHIFT: u32 = 1;
    /// Offset of the `EOImode_EL3` field.
    pub const EOIMODE_EL3_SHIFT: u32 = 2;
    /// Offset of the `EOImode_EL1S` field.
    pub const EOIMODE_EL1S_SHIFT: u32 = 3;
    /// Offset of the `EOImode_EL1NS` field.
    pub const EOIMODE_EL1NS_SHIFT: u32 = 4;
    /// Offset of the `RM` field.
    pub const RM_SHIFT: u32 = 5;
    /// Offset of the `PMHE` field.
    pub const PMHE_SHIFT: u32 = 6;
    /// Offset of the `PRIbits` field.
    pub const PRIBITS_SHIFT: u32 = 8;
    /// Mask for the `PRIbits` field.
    pub const PRIBITS_MASK: u64 = 0b111;
    /// Offset of the `IDbits` field.
    pub const IDBITS_SHIFT: u32 = 11;
    /// Mask for the `IDbits` field.
    pub const IDBITS_MASK: u64 = 0b111;
    /// Offset of the `SEIS` field.
    pub const SEIS_SHIFT: u32 = 14;
    /// Offset of the `A3V` field.
    pub const A3V_SHIFT: u32 = 15;
    /// Offset of the `nDS` field.
    pub const NDS_SHIFT: u32 = 17;
    /// Offset of the `RSS` field.
    pub const RSS_SHIFT: u32 = 18;
    /// Offset of the `ExtRange` field.
    pub const EXTRANGE_SHIFT: u32 = 19;

    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        ((self.bits() >> Self::PRIBITS_SHIFT) & Self::PRIBITS_MASK) as u8
    }

    /// Sets the value of the `PRIbits` field.
    pub const fn set_pribits(&mut self, value: u8) {
        let offset = Self::PRIBITS_SHIFT;
        assert!(value & (Self::PRIBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRIBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PRIbits` field set to the given value.
    pub const fn with_pribits(mut self, value: u8) -> Self {
        self.set_pribits(value);
        self
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        ((self.bits() >> Self::IDBITS_SHIFT) & Self::IDBITS_MASK) as u8
    }

    /// Sets the value of the `IDbits` field.
    pub const fn set_idbits(&mut self, value: u8) {
        let offset = Self::IDBITS_SHIFT;
        assert!(value & (Self::IDBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IDBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `IDbits` field set to the given value.
    pub const fn with_idbits(mut self, value: u8) -> Self {
        self.set_idbits(value);
        self
    }
}

bitflags! {
    /// `ICC_IGRPEN1_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen1El3: u64 {
        /// `EnableGrp1NS` bit.
        const ENABLEGRP1NS = 1 << 0;
        /// `EnableGrp1S` bit.
        const ENABLEGRP1S = 1 << 1;
    }
}

impl IccIgrpen1El3 {
    /// Offset of the `EnableGrp1NS` field.
    pub const ENABLEGRP1NS_SHIFT: u32 = 0;
    /// Offset of the `EnableGrp1S` field.
    pub const ENABLEGRP1S_SHIFT: u32 = 1;
}

bitflags! {
    /// `ICC_SRE_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl IccSreEl3 {
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
    /// `MAIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl3: u64 {
    }
}

impl MairEl3 {
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
    /// `MDCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `RLTE` field.
    pub const RLTE_SHIFT: u32 = 0;
    /// Offset of the `EPMADE` field.
    pub const EPMADE_SHIFT: u32 = 2;
    /// Offset of the `ETADE` field.
    pub const ETADE_SHIFT: u32 = 3;
    /// Offset of the `EDADE` field.
    pub const EDADE_SHIFT: u32 = 4;
    /// Offset of the `TPM` field.
    pub const TPM_SHIFT: u32 = 6;
    /// Offset of the `EnPM2` field.
    pub const ENPM2_SHIFT: u32 = 7;
    /// Offset of the `TDA` field.
    pub const TDA_SHIFT: u32 = 9;
    /// Offset of the `TDOSA` field.
    pub const TDOSA_SHIFT: u32 = 10;
    /// Offset of the `NSPBE` field.
    pub const NSPBE_SHIFT: u32 = 11;
    /// Offset of the `NSPB` field.
    pub const NSPB_SHIFT: u32 = 12;
    /// Mask for the `NSPB` field.
    pub const NSPB_MASK: u64 = 0b11;
    /// Offset of the `SPD32` field.
    pub const SPD32_SHIFT: u32 = 14;
    /// Mask for the `SPD32` field.
    pub const SPD32_MASK: u64 = 0b11;
    /// Offset of the `SDD` field.
    pub const SDD_SHIFT: u32 = 16;
    /// Offset of the `SPME` field.
    pub const SPME_SHIFT: u32 = 17;
    /// Offset of the `STE` field.
    pub const STE_SHIFT: u32 = 18;
    /// Offset of the `TTRF` field.
    pub const TTRF_SHIFT: u32 = 19;
    /// Offset of the `EDAD` field.
    pub const EDAD_SHIFT: u32 = 20;
    /// Offset of the `EPMAD` field.
    pub const EPMAD_SHIFT: u32 = 21;
    /// Offset of the `ETAD` field.
    pub const ETAD_SHIFT: u32 = 22;
    /// Offset of the `SCCD` field.
    pub const SCCD_SHIFT: u32 = 23;
    /// Offset of the `NSTB` field.
    pub const NSTB_SHIFT: u32 = 24;
    /// Mask for the `NSTB` field.
    pub const NSTB_MASK: u64 = 0b11;
    /// Offset of the `NSTBE` field.
    pub const NSTBE_SHIFT: u32 = 26;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the `MTPME` field.
    pub const MTPME_SHIFT: u32 = 28;
    /// Offset of the `PMSSE` field.
    pub const PMSSE_SHIFT: u32 = 30;
    /// Mask for the `PMSSE` field.
    pub const PMSSE_MASK: u64 = 0b11;
    /// Offset of the `SBRBE` field.
    pub const SBRBE_SHIFT: u32 = 32;
    /// Mask for the `SBRBE` field.
    pub const SBRBE_MASK: u64 = 0b11;
    /// Offset of the `MCCD` field.
    pub const MCCD_SHIFT: u32 = 34;
    /// Offset of the `MPMX` field.
    pub const MPMX_SHIFT: u32 = 35;
    /// Offset of the `EnPMSN` field.
    pub const ENPMSN_SHIFT: u32 = 36;
    /// Offset of the `E3BREW` field.
    pub const E3BREW_SHIFT: u32 = 37;
    /// Offset of the `E3BREC` field.
    pub const E3BREC_SHIFT: u32 = 38;
    /// Offset of the `EnTB2` field.
    pub const ENTB2_SHIFT: u32 = 39;
    /// Offset of the `PMEE` field.
    pub const PMEE_SHIFT: u32 = 40;
    /// Mask for the `PMEE` field.
    pub const PMEE_MASK: u64 = 0b11;
    /// Offset of the `EnPMS3` field.
    pub const ENPMS3_SHIFT: u32 = 42;
    /// Offset of the `EBWE` field.
    pub const EBWE_SHIFT: u32 = 43;
    /// Offset of the `EnPMSS` field.
    pub const ENPMSS_SHIFT: u32 = 44;
    /// Offset of the `EPMSSAD` field.
    pub const EPMSSAD_SHIFT: u32 = 45;
    /// Mask for the `EPMSSAD` field.
    pub const EPMSSAD_MASK: u64 = 0b11;
    /// Offset of the `EnITE` field.
    pub const ENITE_SHIFT: u32 = 47;
    /// Offset of the `ETBAD` field.
    pub const ETBAD_SHIFT: u32 = 48;
    /// Mask for the `ETBAD` field.
    pub const ETBAD_MASK: u64 = 0b11;
    /// Offset of the `EnSTEPOP` field.
    pub const ENSTEPOP_SHIFT: u32 = 50;
    /// Offset of the `PMSEE` field.
    pub const PMSEE_SHIFT: u32 = 51;
    /// Mask for the `PMSEE` field.
    pub const PMSEE_MASK: u64 = 0b11;
    /// Offset of the `TRBEE` field.
    pub const TRBEE_SHIFT: u32 = 53;
    /// Mask for the `TRBEE` field.
    pub const TRBEE_MASK: u64 = 0b11;
    /// Offset of the `EnPMS4` field.
    pub const ENPMS4_SHIFT: u32 = 55;

    /// Returns the value of the `NSPB` field.
    pub const fn nspb(self) -> u8 {
        ((self.bits() >> Self::NSPB_SHIFT) & Self::NSPB_MASK) as u8
    }

    /// Sets the value of the `NSPB` field.
    pub const fn set_nspb(&mut self, value: u8) {
        let offset = Self::NSPB_SHIFT;
        assert!(value & (Self::NSPB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSPB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `NSPB` field set to the given value.
    pub const fn with_nspb(mut self, value: u8) -> Self {
        self.set_nspb(value);
        self
    }

    /// Returns the value of the `SPD32` field.
    pub const fn spd32(self) -> u8 {
        ((self.bits() >> Self::SPD32_SHIFT) & Self::SPD32_MASK) as u8
    }

    /// Sets the value of the `SPD32` field.
    pub const fn set_spd32(&mut self, value: u8) {
        let offset = Self::SPD32_SHIFT;
        assert!(value & (Self::SPD32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPD32_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SPD32` field set to the given value.
    pub const fn with_spd32(mut self, value: u8) -> Self {
        self.set_spd32(value);
        self
    }

    /// Returns the value of the `NSTB` field.
    pub const fn nstb(self) -> u8 {
        ((self.bits() >> Self::NSTB_SHIFT) & Self::NSTB_MASK) as u8
    }

    /// Sets the value of the `NSTB` field.
    pub const fn set_nstb(&mut self, value: u8) {
        let offset = Self::NSTB_SHIFT;
        assert!(value & (Self::NSTB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSTB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `NSTB` field set to the given value.
    pub const fn with_nstb(mut self, value: u8) -> Self {
        self.set_nstb(value);
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

    /// Returns the value of the `SBRBE` field.
    pub const fn sbrbe(self) -> u8 {
        ((self.bits() >> Self::SBRBE_SHIFT) & Self::SBRBE_MASK) as u8
    }

    /// Sets the value of the `SBRBE` field.
    pub const fn set_sbrbe(&mut self, value: u8) {
        let offset = Self::SBRBE_SHIFT;
        assert!(value & (Self::SBRBE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SBRBE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `SBRBE` field set to the given value.
    pub const fn with_sbrbe(mut self, value: u8) -> Self {
        self.set_sbrbe(value);
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

    /// Returns the value of the `EPMSSAD` field.
    pub const fn epmssad(self) -> u8 {
        ((self.bits() >> Self::EPMSSAD_SHIFT) & Self::EPMSSAD_MASK) as u8
    }

    /// Sets the value of the `EPMSSAD` field.
    pub const fn set_epmssad(&mut self, value: u8) {
        let offset = Self::EPMSSAD_SHIFT;
        assert!(value & (Self::EPMSSAD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EPMSSAD_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `EPMSSAD` field set to the given value.
    pub const fn with_epmssad(mut self, value: u8) -> Self {
        self.set_epmssad(value);
        self
    }

    /// Returns the value of the `ETBAD` field.
    pub const fn etbad(self) -> u8 {
        ((self.bits() >> Self::ETBAD_SHIFT) & Self::ETBAD_MASK) as u8
    }

    /// Sets the value of the `ETBAD` field.
    pub const fn set_etbad(&mut self, value: u8) {
        let offset = Self::ETBAD_SHIFT;
        assert!(value & (Self::ETBAD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ETBAD_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `ETBAD` field set to the given value.
    pub const fn with_etbad(mut self, value: u8) -> Self {
        self.set_etbad(value);
        self
    }

    /// Returns the value of the `PMSEE` field.
    pub const fn pmsee(self) -> u8 {
        ((self.bits() >> Self::PMSEE_SHIFT) & Self::PMSEE_MASK) as u8
    }

    /// Sets the value of the `PMSEE` field.
    pub const fn set_pmsee(&mut self, value: u8) {
        let offset = Self::PMSEE_SHIFT;
        assert!(value & (Self::PMSEE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSEE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `PMSEE` field set to the given value.
    pub const fn with_pmsee(mut self, value: u8) -> Self {
        self.set_pmsee(value);
        self
    }

    /// Returns the value of the `TRBEE` field.
    pub const fn trbee(self) -> u8 {
        ((self.bits() >> Self::TRBEE_SHIFT) & Self::TRBEE_MASK) as u8
    }

    /// Sets the value of the `TRBEE` field.
    pub const fn set_trbee(&mut self, value: u8) {
        let offset = Self::TRBEE_SHIFT;
        assert!(value & (Self::TRBEE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRBEE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `TRBEE` field set to the given value.
    pub const fn with_trbee(mut self, value: u8) -> Self {
        self.set_trbee(value);
        self
    }
}

bitflags! {
    /// `MPAM3_EL3` system register value.
    ///
    /// Holds information to generate MPAM labels for memory requests when executing at EL3.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `altPMG` field.
    pub const ALTPMG_SHIFT: u32 = 48;
    /// Mask for the `altPMG` field.
    pub const ALTPMG_MASK: u64 = 0b1111_1111_1111_1111;
    /// Offset of the `RT_ALTSP_NS` field.
    pub const RT_ALTSP_NS_SHIFT: u32 = 52;
    /// Offset of the `ALTSP_EL3` field.
    pub const ALTSP_EL3_SHIFT: u32 = 55;
    /// Offset of the `ALTSP_HFC` field.
    pub const ALTSP_HFC_SHIFT: u32 = 56;
    /// Offset of the `ALTSP_HEN` field.
    pub const ALTSP_HEN_SHIFT: u32 = 57;
    /// Offset of the `FORCE_NS` field.
    pub const FORCE_NS_SHIFT: u32 = 60;
    /// Offset of the `SDEFLT` field.
    pub const SDEFLT_SHIFT: u32 = 61;
    /// Offset of the `TRAPLOWER` field.
    pub const TRAPLOWER_SHIFT: u32 = 62;
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
    /// `PIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PirEl3: u64 {
    }
}

impl PirEl3 {
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

/// `POR_EL3` system register value.
pub type PorEl3 = PirEl3;

bitflags! {
    /// `SCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ScrEl3: u64 {
        /// RES1 bits in the `SCR_EL3` register.
        const RES1 = 0b11_0000;
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
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 0;
    /// Offset of the `IRQ` field.
    pub const IRQ_SHIFT: u32 = 1;
    /// Offset of the `FIQ` field.
    pub const FIQ_SHIFT: u32 = 2;
    /// Offset of the `EA` field.
    pub const EA_SHIFT: u32 = 3;
    /// Offset of the `SMD` field.
    pub const SMD_SHIFT: u32 = 7;
    /// Offset of the `HCE` field.
    pub const HCE_SHIFT: u32 = 8;
    /// Offset of the `SIF` field.
    pub const SIF_SHIFT: u32 = 9;
    /// Offset of the `RW` field.
    pub const RW_SHIFT: u32 = 10;
    /// Offset of the `ST` field.
    pub const ST_SHIFT: u32 = 11;
    /// Offset of the `TWI` field.
    pub const TWI_SHIFT: u32 = 12;
    /// Offset of the `TWE` field.
    pub const TWE_SHIFT: u32 = 13;
    /// Offset of the `TLOR` field.
    pub const TLOR_SHIFT: u32 = 14;
    /// Offset of the `TERR` field.
    pub const TERR_SHIFT: u32 = 15;
    /// Offset of the `APK` field.
    pub const APK_SHIFT: u32 = 16;
    /// Offset of the `API` field.
    pub const API_SHIFT: u32 = 17;
    /// Offset of the `EEL2` field.
    pub const EEL2_SHIFT: u32 = 18;
    /// Offset of the `EASE` field.
    pub const EASE_SHIFT: u32 = 19;
    /// Offset of the `NMEA` field.
    pub const NMEA_SHIFT: u32 = 20;
    /// Offset of the `FIEN` field.
    pub const FIEN_SHIFT: u32 = 21;
    /// Offset of the `TID3` field.
    pub const TID3_SHIFT: u32 = 22;
    /// Offset of the `TID5` field.
    pub const TID5_SHIFT: u32 = 23;
    /// Offset of the `POE2En` field.
    pub const POE2EN_SHIFT: u32 = 24;
    /// Offset of the `EnSCXT` field.
    pub const ENSCXT_SHIFT: u32 = 25;
    /// Offset of the `ATA` field.
    pub const ATA_SHIFT: u32 = 26;
    /// Offset of the `FGTEn` field.
    pub const FGTEN_SHIFT: u32 = 27;
    /// Offset of the `ECVEn` field.
    pub const ECVEN_SHIFT: u32 = 28;
    /// Offset of the `TWEDEn` field.
    pub const TWEDEN_SHIFT: u32 = 29;
    /// Offset of the `TWEDEL` field.
    pub const TWEDEL_SHIFT: u32 = 30;
    /// Mask for the `TWEDEL` field.
    pub const TWEDEL_MASK: u64 = 0b1111;
    /// Offset of the `AMVOFFEN` field.
    pub const AMVOFFEN_SHIFT: u32 = 35;
    /// Offset of the `EnAS0` field.
    pub const ENAS0_SHIFT: u32 = 36;
    /// Offset of the `ADEn` field.
    pub const ADEN_SHIFT: u32 = 37;
    /// Offset of the `HXEn` field.
    pub const HXEN_SHIFT: u32 = 38;
    /// Offset of the `GCSEn` field.
    pub const GCSEN_SHIFT: u32 = 39;
    /// Offset of the `TRNDR` field.
    pub const TRNDR_SHIFT: u32 = 40;
    /// Offset of the `EnTP2` field.
    pub const ENTP2_SHIFT: u32 = 41;
    /// Offset of the `RCWMASKEn` field.
    pub const RCWMASKEN_SHIFT: u32 = 42;
    /// Offset of the `TCR2En` field.
    pub const TCR2EN_SHIFT: u32 = 43;
    /// Offset of the `SCTLR2En` field.
    pub const SCTLR2EN_SHIFT: u32 = 44;
    /// Offset of the `PIEn` field.
    pub const PIEN_SHIFT: u32 = 45;
    /// Offset of the `AIEn` field.
    pub const AIEN_SHIFT: u32 = 46;
    /// Offset of the `D128En` field.
    pub const D128EN_SHIFT: u32 = 47;
    /// Offset of the `GPF` field.
    pub const GPF_SHIFT: u32 = 48;
    /// Offset of the `MECEn` field.
    pub const MECEN_SHIFT: u32 = 49;
    /// Offset of the `EnFPM` field.
    pub const ENFPM_SHIFT: u32 = 50;
    /// Offset of the `TMEA` field.
    pub const TMEA_SHIFT: u32 = 51;
    /// Offset of the `TWERR` field.
    pub const TWERR_SHIFT: u32 = 52;
    /// Offset of the `PFAREn` field.
    pub const PFAREN_SHIFT: u32 = 53;
    /// Offset of the `SRMASKEn` field.
    pub const SRMASKEN_SHIFT: u32 = 54;
    /// Offset of the `EnIDCP128` field.
    pub const ENIDCP128_SHIFT: u32 = 55;
    /// Offset of the `VTLBIDEn` field.
    pub const VTLBIDEN_SHIFT: u32 = 56;
    /// Offset of the `DSE` field.
    pub const DSE_SHIFT: u32 = 57;
    /// Offset of the `EnDSE` field.
    pub const ENDSE_SHIFT: u32 = 58;
    /// Offset of the `FGTEn2` field.
    pub const FGTEN2_SHIFT: u32 = 59;
    /// Offset of the `HDBSSEn` field.
    pub const HDBSSEN_SHIFT: u32 = 60;
    /// Offset of the `HACDBSEn` field.
    pub const HACDBSEN_SHIFT: u32 = 61;
    /// Offset of the `NSE` field.
    pub const NSE_SHIFT: u32 = 62;
    /// Offset of the `TPLIMEn` field.
    pub const TPLIMEN_SHIFT: u32 = 63;

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
    /// `SCTLR2_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr2El3: u64 {
        /// `EMEC` bit.
        const EMEC = 1 << 1;
        /// `EnADERR` bit.
        const ENADERR = 1 << 3;
        /// `EnANERR` bit.
        const ENANERR = 1 << 4;
        /// `EnPACM` bit.
        const ENPACM = 1 << 7;
        /// `CPTA` bit.
        const CPTA = 1 << 9;
        /// `CPTM` bit.
        const CPTM = 1 << 11;
        /// `DTZ` bit.
        const DTZ = 1 << 14;
        /// `TEIS` bit.
        const TEIS = 1 << 15;
        /// `TEOS` bit.
        const TEOS = 1 << 16;
        /// `VT` bit.
        const VT = 1 << 17;
        /// `BTD` bit.
        const BTD = 1 << 24;
    }
}

impl Sctlr2El3 {
    /// Offset of the `EMEC` field.
    pub const EMEC_SHIFT: u32 = 1;
    /// Offset of the `EnADERR` field.
    pub const ENADERR_SHIFT: u32 = 3;
    /// Offset of the `EnANERR` field.
    pub const ENANERR_SHIFT: u32 = 4;
    /// Offset of the `EnPACM` field.
    pub const ENPACM_SHIFT: u32 = 7;
    /// Offset of the `CPTA` field.
    pub const CPTA_SHIFT: u32 = 9;
    /// Offset of the `CPTM` field.
    pub const CPTM_SHIFT: u32 = 11;
    /// Offset of the `DTZ` field.
    pub const DTZ_SHIFT: u32 = 14;
    /// Offset of the `TEIS` field.
    pub const TEIS_SHIFT: u32 = 15;
    /// Offset of the `TEOS` field.
    pub const TEOS_SHIFT: u32 = 16;
    /// Offset of the `VT` field.
    pub const VT_SHIFT: u32 = 17;
    /// Offset of the `BTD` field.
    pub const BTD_SHIFT: u32 = 24;
}

bitflags! {
    /// `SCTLR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SctlrEl3: u64 {
        /// RES1 bits in the `SCTLR_EL3` register.
        const RES1 = 0b11_0000_1000_0101_0000_0000_0011_0000;
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
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `SA` field.
    pub const SA_SHIFT: u32 = 3;
    /// Offset of the `nAA` field.
    pub const NAA_SHIFT: u32 = 6;
    /// Offset of the `EOS` field.
    pub const EOS_SHIFT: u32 = 11;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the `EnDB` field.
    pub const ENDB_SHIFT: u32 = 13;
    /// Offset of the `WXN` field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the `IESB` field.
    pub const IESB_SHIFT: u32 = 21;
    /// Offset of the `EIS` field.
    pub const EIS_SHIFT: u32 = 22;
    /// Offset of the `EnDA` field.
    pub const ENDA_SHIFT: u32 = 27;
    /// Offset of the `EnIB` field.
    pub const ENIB_SHIFT: u32 = 30;
    /// Offset of the `EnIA` field.
    pub const ENIA_SHIFT: u32 = 31;
    /// Offset of the `BT` field.
    pub const BT_SHIFT: u32 = 36;
    /// Offset of the `ITFSB` field.
    pub const ITFSB_SHIFT: u32 = 37;
    /// Offset of the `TCF` field.
    pub const TCF_SHIFT: u32 = 40;
    /// Mask for the `TCF` field.
    pub const TCF_MASK: u64 = 0b11;
    /// Offset of the `ATA` field.
    pub const ATA_SHIFT: u32 = 43;
    /// Offset of the `DSSBS` field.
    pub const DSSBS_SHIFT: u32 = 44;
    /// Offset of the `TCSO` field.
    pub const TCSO_SHIFT: u32 = 59;
    /// Offset of the `NMI` field.
    pub const NMI_SHIFT: u32 = 61;
    /// Offset of the `SPINTMASK` field.
    pub const SPINTMASK_SHIFT: u32 = 62;

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
}

bitflags! {
    /// `SMCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SmcrEl3: u64 {
        /// `EZT0` bit.
        const EZT0 = 1 << 30;
        /// `FA64` bit.
        const FA64 = 1 << 31;
    }
}

impl SmcrEl3 {
    /// Offset of the `LEN` field.
    pub const LEN_SHIFT: u32 = 0;
    /// Mask for the `LEN` field.
    pub const LEN_MASK: u64 = 0b1111;
    /// Offset of the `EZT0` field.
    pub const EZT0_SHIFT: u32 = 30;
    /// Offset of the `FA64` field.
    pub const FA64_SHIFT: u32 = 31;

    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        ((self.bits() >> Self::LEN_SHIFT) & Self::LEN_MASK) as u8
    }

    /// Sets the value of the `LEN` field.
    pub const fn set_len(&mut self, value: u8) {
        let offset = Self::LEN_SHIFT;
        assert!(value & (Self::LEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LEN` field set to the given value.
    pub const fn with_len(mut self, value: u8) -> Self {
        self.set_len(value);
        self
    }
}

bitflags! {
    /// `SPSR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        /// `EXLOCK` bit.
        const EXLOCK = 1 << 34;
        /// `PACM` bit.
        const PACM = 1 << 35;
        /// `UINJ` bit.
        const UINJ = 1 << 36;
    }
}

impl SpsrEl3 {
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
    /// `TCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrEl3: u64 {
        /// RES1 bits in the `TCR_EL3` register.
        const RES1 = 0b1000_0000_1000_0000_0000_0000_0000_0000;
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
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b11_1111;
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
    /// Offset of the `TBI` field.
    pub const TBI_SHIFT: u32 = 20;
    /// Offset of the `HA` field.
    pub const HA_SHIFT: u32 = 21;
    /// Offset of the `HD` field.
    pub const HD_SHIFT: u32 = 22;
    /// Offset of the `HPD` field.
    pub const HPD_SHIFT: u32 = 24;
    /// Offset of the `HWU59` field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the `HWU60` field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the `HWU61` field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the `HWU62` field.
    pub const HWU62_SHIFT: u32 = 28;
    /// Offset of the `TBID` field.
    pub const TBID_SHIFT: u32 = 29;
    /// Offset of the `TCMA` field.
    pub const TCMA_SHIFT: u32 = 30;
    /// Offset of the `DS` field.
    pub const DS_SHIFT: u32 = 32;
    /// Offset of the `MTX` field.
    pub const MTX_SHIFT: u32 = 33;
    /// Offset of the `PnCH` field.
    pub const PNCH_SHIFT: u32 = 34;
    /// Offset of the `PIE` field.
    pub const PIE_SHIFT: u32 = 35;
    /// Offset of the `POE` field.
    pub const POE_SHIFT: u32 = 36;
    /// Offset of the `AIE` field.
    pub const AIE_SHIFT: u32 = 37;
    /// Offset of the `D128` field.
    pub const D128_SHIFT: u32 = 38;
    /// Offset of the `PTTWI` field.
    pub const PTTWI_SHIFT: u32 = 41;
    /// Offset of the `HAFT` field.
    pub const HAFT_SHIFT: u32 = 42;
    /// Offset of the `DisCH0` field.
    pub const DISCH0_SHIFT: u32 = 43;
    /// Offset of the `POE2F` field.
    pub const POE2F_SHIFT: u32 = 44;
    /// Offset of the `POIW` field.
    pub const POIW_SHIFT: u32 = 45;
    /// Mask for the `POIW` field.
    pub const POIW_MASK: u64 = 0b111;
    /// Offset of the `VTB` field.
    pub const VTB_SHIFT: u32 = 48;
    /// Mask for the `VTB` field.
    pub const VTB_MASK: u64 = 0b1_1111;
    /// Offset of the `TVAD` field.
    pub const TVAD_SHIFT: u32 = 53;

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

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        ((self.bits() >> Self::VTB_SHIFT) & Self::VTB_MASK) as u8
    }

    /// Sets the value of the `VTB` field.
    pub const fn set_vtb(&mut self, value: u8) {
        let offset = Self::VTB_SHIFT;
        assert!(value & (Self::VTB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `VTB` field set to the given value.
    pub const fn with_vtb(mut self, value: u8) -> Self {
        self.set_vtb(value);
        self
    }
}

bitflags! {
    /// `TPIDR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl3: u64 {
    }
}

impl TpidrEl3 {
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
    /// `TTBR0_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El3: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Ttbr0El3 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;

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
}

bitflags! {
    /// `ZCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ZcrEl3: u64 {
    }
}

impl ZcrEl3 {
    /// Offset of the `LEN` field.
    pub const LEN_SHIFT: u32 = 0;
    /// Mask for the `LEN` field.
    pub const LEN_MASK: u64 = 0b1111;

    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        ((self.bits() >> Self::LEN_SHIFT) & Self::LEN_MASK) as u8
    }

    /// Sets the value of the `LEN` field.
    pub const fn set_len(&mut self, value: u8) {
        let offset = Self::LEN_SHIFT;
        assert!(value & (Self::LEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns a copy with the `LEN` field set to the given value.
    pub const fn with_len(mut self, value: u8) -> Self {
        self.set_len(value);
        self
    }
}
