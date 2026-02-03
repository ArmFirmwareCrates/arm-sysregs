// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

#![cfg_attr(not(any(test, feature = "fakes")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(not(any(test, feature = "fakes")), target_arch = "arm"))]
mod aarch32;
#[cfg(all(not(any(test, feature = "fakes")), target_arch = "aarch64"))]
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
    /// `AMCFGR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcfgr: u32 {
        /// `HDBG` bit.
        const HDBG = 1 << 24;
    }
}

impl Amcfgr {
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 0;
    /// Mask for the `N` field.
    pub const N_MASK: u32 = 0b11111111;
    /// Offset of the `SIZE` field.
    pub const SIZE_SHIFT: u32 = 8;
    /// Mask for the `SIZE` field.
    pub const SIZE_MASK: u32 = 0b111111;
    /// Offset of the `HDBG` field.
    pub const HDBG_SHIFT: u32 = 24;
    /// Offset of the `NCG` field.
    pub const NCG_SHIFT: u32 = 28;
    /// Mask for the `NCG` field.
    pub const NCG_MASK: u32 = 0b1111;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SIZE` field.
    pub const fn size(self) -> u8 {
        ((self.bits() >> Self::SIZE_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `SIZE` field.
    pub const fn set_size(&mut self, value: u8) {
        let offset = Self::SIZE_SHIFT;
        assert!(value & (Self::SIZE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SIZE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `NCG` field.
    pub const fn ncg(self) -> u8 {
        ((self.bits() >> Self::NCG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `NCG` field.
    pub const fn set_ncg(&mut self, value: u8) {
        let offset = Self::NCG_SHIFT;
        assert!(value & (Self::NCG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NCG_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `AMCGCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcgcr: u32 {
    }
}

impl Amcgcr {
    /// Offset of the `CG0NC` field.
    pub const CG0NC_SHIFT: u32 = 0;
    /// Mask for the `CG0NC` field.
    pub const CG0NC_MASK: u32 = 0b11111111;
    /// Offset of the `CG1NC` field.
    pub const CG1NC_SHIFT: u32 = 8;
    /// Mask for the `CG1NC` field.
    pub const CG1NC_MASK: u32 = 0b11111111;

    /// Returns the value of the `CG0NC` field.
    pub const fn cg0nc(self) -> u8 {
        ((self.bits() >> Self::CG0NC_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `CG0NC` field.
    pub const fn set_cg0nc(&mut self, value: u8) {
        let offset = Self::CG0NC_SHIFT;
        assert!(value & (Self::CG0NC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CG0NC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CG1NC` field.
    pub const fn cg1nc(self) -> u8 {
        ((self.bits() >> Self::CG1NC_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `CG1NC` field.
    pub const fn set_cg1nc(&mut self, value: u8) {
        let offset = Self::CG1NC_SHIFT;
        assert!(value & (Self::CG1NC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CG1NC_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `AMCNTENCLR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr0: u32 {
        /// `P<n>` bit 0.
        const P0 = 1 << 0;
        /// `P<n>` bit 1.
        const P1 = 1 << 1;
        /// `P<n>` bit 2.
        const P2 = 1 << 2;
        /// `P<n>` bit 3.
        const P3 = 1 << 3;
    }
}

impl Amcntenclr0 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCNTENCLR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr1: u32 {
        /// `P<n>` bit 0.
        const P0 = 1 << 0;
        /// `P<n>` bit 1.
        const P1 = 1 << 1;
        /// `P<n>` bit 2.
        const P2 = 1 << 2;
        /// `P<n>` bit 3.
        const P3 = 1 << 3;
        /// `P<n>` bit 4.
        const P4 = 1 << 4;
        /// `P<n>` bit 5.
        const P5 = 1 << 5;
        /// `P<n>` bit 6.
        const P6 = 1 << 6;
        /// `P<n>` bit 7.
        const P7 = 1 << 7;
        /// `P<n>` bit 8.
        const P8 = 1 << 8;
        /// `P<n>` bit 9.
        const P9 = 1 << 9;
        /// `P<n>` bit 10.
        const P10 = 1 << 10;
        /// `P<n>` bit 11.
        const P11 = 1 << 11;
        /// `P<n>` bit 12.
        const P12 = 1 << 12;
        /// `P<n>` bit 13.
        const P13 = 1 << 13;
        /// `P<n>` bit 14.
        const P14 = 1 << 14;
        /// `P<n>` bit 15.
        const P15 = 1 << 15;
    }
}

impl Amcntenclr1 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCNTENSET0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenset0: u32 {
        /// `P<n>` bit 0.
        const P0 = 1 << 0;
        /// `P<n>` bit 1.
        const P1 = 1 << 1;
        /// `P<n>` bit 2.
        const P2 = 1 << 2;
        /// `P<n>` bit 3.
        const P3 = 1 << 3;
    }
}

impl Amcntenset0 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCNTENSET1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenset1: u32 {
        /// `P<n>` bit 0.
        const P0 = 1 << 0;
        /// `P<n>` bit 1.
        const P1 = 1 << 1;
        /// `P<n>` bit 2.
        const P2 = 1 << 2;
        /// `P<n>` bit 3.
        const P3 = 1 << 3;
        /// `P<n>` bit 4.
        const P4 = 1 << 4;
        /// `P<n>` bit 5.
        const P5 = 1 << 5;
        /// `P<n>` bit 6.
        const P6 = 1 << 6;
        /// `P<n>` bit 7.
        const P7 = 1 << 7;
        /// `P<n>` bit 8.
        const P8 = 1 << 8;
        /// `P<n>` bit 9.
        const P9 = 1 << 9;
        /// `P<n>` bit 10.
        const P10 = 1 << 10;
        /// `P<n>` bit 11.
        const P11 = 1 << 11;
        /// `P<n>` bit 12.
        const P12 = 1 << 12;
        /// `P<n>` bit 13.
        const P13 = 1 << 13;
        /// `P<n>` bit 14.
        const P14 = 1 << 14;
        /// `P<n>` bit 15.
        const P15 = 1 << 15;
    }
}

impl Amcntenset1 {
    /// Offset of the `P<n>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `AMCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcr: u32 {
        /// `HDBG` bit.
        const HDBG = 1 << 10;
        /// `CG1RZ` bit.
        const CG1RZ = 1 << 17;
    }
}

impl Amcr {
    /// Offset of the `HDBG` field.
    pub const HDBG_SHIFT: u32 = 10;
    /// Offset of the `CG1RZ` field.
    pub const CG1RZ_SHIFT: u32 = 17;
}

bitflags! {
    /// `AMUSERENR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amuserenr: u32 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

impl Amuserenr {
    /// Offset of the `EN` field.
    pub const EN_SHIFT: u32 = 0;
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APIAKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyhiEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApiakeyhiEl1 {
    /// Offset of the `APIAKeyHi` field.
    pub const APIAKEYHI_SHIFT: u32 = 0;
    /// Mask for the `APIAKeyHi` field.
    pub const APIAKEYHI_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `APIAKeyHi` field.
    pub const fn apiakeyhi(self) -> u64 {
        ((self.bits() >> Self::APIAKEYHI_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `APIAKeyHi` field.
    pub const fn set_apiakeyhi(&mut self, value: u64) {
        let offset = Self::APIAKEYHI_SHIFT;
        assert!(value & (Self::APIAKEYHI_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APIAKEYHI_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APIAKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApiakeyloEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApiakeyloEl1 {
    /// Offset of the `APIAKeyLo` field.
    pub const APIAKEYLO_SHIFT: u32 = 0;
    /// Mask for the `APIAKeyLo` field.
    pub const APIAKEYLO_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `APIAKeyLo` field.
    pub const fn apiakeylo(self) -> u64 {
        ((self.bits() >> Self::APIAKEYLO_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `APIAKeyLo` field.
    pub const fn set_apiakeylo(&mut self, value: u64) {
        let offset = Self::APIAKEYLO_SHIFT;
        assert!(value & (Self::APIAKEYLO_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APIAKEYLO_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CCSIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ccsidr: u32 {
    }
}

impl Ccsidr {
    /// Offset of the `LineSize` field.
    pub const LINESIZE_SHIFT: u32 = 0;
    /// Mask for the `LineSize` field.
    pub const LINESIZE_MASK: u32 = 0b111;
    /// Offset of the `NumSets` field.
    pub const NUMSETS_SHIFT: u32 = 13;
    /// Mask for the `NumSets` field.
    pub const NUMSETS_MASK: u32 = 0b111111111111111;

    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        ((self.bits() >> Self::LINESIZE_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LineSize` field.
    pub const fn set_linesize(&mut self, value: u8) {
        let offset = Self::LINESIZE_SHIFT;
        assert!(value & (Self::LINESIZE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LINESIZE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `NumSets` field.
    pub const fn numsets(self) -> u16 {
        ((self.bits() >> Self::NUMSETS_SHIFT) & 0b111111111111111) as u16
    }

    /// Sets the value of the `NumSets` field.
    pub const fn set_numsets(&mut self, value: u16) {
        let offset = Self::NUMSETS_SHIFT;
        assert!(value & (Self::NUMSETS_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NUMSETS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CCSIDR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ccsidr2: u32 {
    }
}

impl Ccsidr2 {
    /// Offset of the `NumSets` field.
    pub const NUMSETS_SHIFT: u32 = 0;
    /// Mask for the `NumSets` field.
    pub const NUMSETS_MASK: u32 = 0b111111111111111111111111;

    /// Returns the value of the `NumSets` field.
    pub const fn numsets(self) -> u32 {
        ((self.bits() >> Self::NUMSETS_SHIFT) & 0b111111111111111111111111) as u32
    }

    /// Sets the value of the `NumSets` field.
    pub const fn set_numsets(&mut self, value: u32) {
        let offset = Self::NUMSETS_SHIFT;
        assert!(value & (Self::NUMSETS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NUMSETS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CCSIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CcsidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl CcsidrEl1 {
    /// Offset of the `LineSize` field.
    pub const LINESIZE_SHIFT: u32 = 0;
    /// Mask for the `LineSize` field.
    pub const LINESIZE_MASK: u64 = 0b111;

    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        ((self.bits() >> Self::LINESIZE_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LineSize` field.
    pub const fn set_linesize(&mut self, value: u8) {
        let offset = Self::LINESIZE_SHIFT;
        assert!(value & (Self::LINESIZE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LINESIZE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CLIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Clidr: u32 {
    }
}

impl Clidr {
    /// Offset of the `Ctype<n>` field.
    pub const CTYPE_SHIFT: u32 = 0;
    /// Mask for the `Ctype<n>` field.
    pub const CTYPE_MASK: u32 = 0b111;
    /// Offset of the `LoUIS` field.
    pub const LOUIS_SHIFT: u32 = 21;
    /// Mask for the `LoUIS` field.
    pub const LOUIS_MASK: u32 = 0b111;
    /// Offset of the `LoC` field.
    pub const LOC_SHIFT: u32 = 24;
    /// Mask for the `LoC` field.
    pub const LOC_MASK: u32 = 0b111;
    /// Offset of the `LoUU` field.
    pub const LOUU_SHIFT: u32 = 27;
    /// Mask for the `LoUU` field.
    pub const LOUU_MASK: u32 = 0b111;
    /// Offset of the `ICB` field.
    pub const ICB_SHIFT: u32 = 30;
    /// Mask for the `ICB` field.
    pub const ICB_MASK: u32 = 0b11;

    /// Returns the value of the given `Ctype<n>` field.
    pub const fn ctype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        ((self.bits() >> (Self::CTYPE_SHIFT + (n - 1) * 3)) & 0b111) as u8
    }

    /// Sets the value of the `Ctype<n>` field.
    pub const fn set_ctype(&mut self, n: u32, value: u8) {
        assert!(n >= 1 && n < 8);
        let offset = Self::CTYPE_SHIFT + (n - 1) * 3;
        assert!(value & (Self::CTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTYPE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `LoUIS` field.
    pub const fn louis(self) -> u8 {
        ((self.bits() >> Self::LOUIS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LoUIS` field.
    pub const fn set_louis(&mut self, value: u8) {
        let offset = Self::LOUIS_SHIFT;
        assert!(value & (Self::LOUIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUIS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `LoC` field.
    pub const fn loc(self) -> u8 {
        ((self.bits() >> Self::LOC_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LoC` field.
    pub const fn set_loc(&mut self, value: u8) {
        let offset = Self::LOC_SHIFT;
        assert!(value & (Self::LOC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `LoUU` field.
    pub const fn louu(self) -> u8 {
        ((self.bits() >> Self::LOUU_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LoUU` field.
    pub const fn set_louu(&mut self, value: u8) {
        let offset = Self::LOUU_SHIFT;
        assert!(value & (Self::LOUU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `ICB` field.
    pub const fn icb(self) -> u8 {
        ((self.bits() >> Self::ICB_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ICB` field.
    pub const fn set_icb(&mut self, value: u8) {
        let offset = Self::ICB_SHIFT;
        assert!(value & (Self::ICB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ICB_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CLIDR_EL1` system register value.
    ///
    /// Cache Level ID.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ClidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ClidrEl1 {
    /// Offset of the `Ctype<n>` field.
    pub const CTYPE_SHIFT: u32 = 0;
    /// Mask for the `Ctype<n>` field.
    pub const CTYPE_MASK: u64 = 0b111;
    /// Offset of the `LoUIS` field.
    pub const LOUIS_SHIFT: u32 = 21;
    /// Mask for the `LoUIS` field.
    pub const LOUIS_MASK: u64 = 0b111;
    /// Offset of the `LoC` field.
    pub const LOC_SHIFT: u32 = 24;
    /// Mask for the `LoC` field.
    pub const LOC_MASK: u64 = 0b111;
    /// Offset of the `LoUU` field.
    pub const LOUU_SHIFT: u32 = 27;
    /// Mask for the `LoUU` field.
    pub const LOUU_MASK: u64 = 0b111;
    /// Offset of the `ICB` field.
    pub const ICB_SHIFT: u32 = 30;
    /// Mask for the `ICB` field.
    pub const ICB_MASK: u64 = 0b111;
    /// Offset of the `Ttype<n>` field.
    pub const TTYPE_SHIFT: u32 = 33;
    /// Mask for the `Ttype<n>` field.
    pub const TTYPE_MASK: u64 = 0b11;

    /// Returns the value of the given `Ctype<n>` field.
    pub fn ctype(self, n: u32) -> crate::manual::CacheType {
        assert!(n >= 1 && n < 8);
        crate::manual::CacheType::try_from(
            ((self.bits() >> (Self::CTYPE_SHIFT + (n - 1) * 3)) & 0b111) as u8,
        )
        .unwrap()
    }

    /// Sets the value of the `Ctype<n>` field.
    pub fn set_ctype(&mut self, n: u32, value: crate::manual::CacheType) {
        assert!(n >= 1 && n < 8);
        let offset = Self::CTYPE_SHIFT + (n - 1) * 3;
        let value: u8 = value.into();
        assert!(value & (Self::CTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LoUIS` field.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn louis(self) -> u8 {
        ((self.bits() >> Self::LOUIS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LoUIS` field.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn set_louis(&mut self, value: u8) {
        let offset = Self::LOUIS_SHIFT;
        assert!(value & (Self::LOUIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUIS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LoC` field.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn loc(self) -> u8 {
        ((self.bits() >> Self::LOC_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LoC` field.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn set_loc(&mut self, value: u8) {
        let offset = Self::LOC_SHIFT;
        assert!(value & (Self::LOC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LoUU` field.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn louu(self) -> u8 {
        ((self.bits() >> Self::LOUU_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `LoUU` field.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn set_louu(&mut self, value: u8) {
        let offset = Self::LOUU_SHIFT;
        assert!(value & (Self::LOUU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOUU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ICB` field.
    ///
    /// Inner cache boundary level.
    pub const fn icb(self) -> u8 {
        ((self.bits() >> Self::ICB_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `ICB` field.
    ///
    /// Inner cache boundary level.
    pub const fn set_icb(&mut self, value: u8) {
        let offset = Self::ICB_SHIFT;
        assert!(value & (Self::ICB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ICB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the given `Ttype<n>` field.
    pub const fn ttype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        ((self.bits() >> (Self::TTYPE_SHIFT + (n - 1) * 2)) & 0b11) as u8
    }

    /// Sets the value of the `Ttype<n>` field.
    pub const fn set_ttype(&mut self, n: u32, value: u8) {
        assert!(n >= 1 && n < 8);
        let offset = Self::TTYPE_SHIFT + (n - 1) * 2;
        assert!(value & (Self::TTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTFRQ` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntfrq: u32 {
    }
}

impl Cntfrq {
    /// Offset of the `ClockFreq` field.
    pub const CLOCKFREQ_SHIFT: u32 = 0;
    /// Mask for the `ClockFreq` field.
    pub const CLOCKFREQ_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ClockFreq` field.
    pub const fn clockfreq(self) -> u32 {
        ((self.bits() >> Self::CLOCKFREQ_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ClockFreq` field.
    pub const fn set_clockfreq(&mut self, value: u32) {
        let offset = Self::CLOCKFREQ_SHIFT;
        assert!(value & (Self::CLOCKFREQ_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CLOCKFREQ_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CNTFRQ_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntfrqEl0: u64 {
    }
}

impl CntfrqEl0 {
    /// Offset of the `ClockFreq` field.
    pub const CLOCKFREQ_SHIFT: u32 = 0;
    /// Mask for the `ClockFreq` field.
    pub const CLOCKFREQ_MASK: u64 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ClockFreq` field.
    pub const fn clockfreq(self) -> u32 {
        ((self.bits() >> Self::CLOCKFREQ_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ClockFreq` field.
    pub const fn set_clockfreq(&mut self, value: u32) {
        let offset = Self::CLOCKFREQ_SHIFT;
        assert!(value & (Self::CLOCKFREQ_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CLOCKFREQ_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTHCTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cnthctl: u32 {
        /// `PL1PCTEN` bit.
        const PL1PCTEN = 1 << 0;
        /// `PL1PCEN` bit.
        const PL1PCEN = 1 << 1;
        /// `EVNTEN` bit.
        const EVNTEN = 1 << 2;
        /// `EVNTDIR` bit.
        const EVNTDIR = 1 << 3;
        /// `EVNTIS` bit.
        const EVNTIS = 1 << 17;
    }
}

impl Cnthctl {
    /// Offset of the `PL1PCTEN` field.
    pub const PL1PCTEN_SHIFT: u32 = 0;
    /// Offset of the `PL1PCEN` field.
    pub const PL1PCEN_SHIFT: u32 = 1;
    /// Offset of the `EVNTEN` field.
    pub const EVNTEN_SHIFT: u32 = 2;
    /// Offset of the `EVNTDIR` field.
    pub const EVNTDIR_SHIFT: u32 = 3;
    /// Offset of the `EVNTI` field.
    pub const EVNTI_SHIFT: u32 = 4;
    /// Mask for the `EVNTI` field.
    pub const EVNTI_MASK: u32 = 0b1111;
    /// Offset of the `EVNTIS` field.
    pub const EVNTIS_SHIFT: u32 = 17;

    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        ((self.bits() >> Self::EVNTI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EVNTI` field.
    pub const fn set_evnti(&mut self, value: u8) {
        let offset = Self::EVNTI_SHIFT;
        assert!(value & (Self::EVNTI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVNTI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHCTL_EL2` system register value.
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

#[cfg(feature = "el2")]
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
        ((self.bits() >> Self::EVNTI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EVNTI` field.
    pub const fn set_evnti(&mut self, value: u8) {
        let offset = Self::EVNTI_SHIFT;
        assert!(value & (Self::EVNTI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVNTI_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTHPS_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CnthpsCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTHPS_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsCval: u64 {
    }
}

impl CnthpsCval {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        ((self.bits() >> Self::COMPAREVALUE_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & (Self::COMPAREVALUE_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTHPS_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsTval: u32 {
    }
}

impl CnthpsTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CNTHP_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CnthpCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTHP_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpCval: u64 {
    }
}

impl CnthpCval {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        ((self.bits() >> Self::COMPAREVALUE_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & (Self::COMPAREVALUE_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTHP_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpTval: u32 {
    }
}

impl CnthpTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CNTHVS_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CnthvsCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTHVS_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsCval: u64 {
    }
}

impl CnthvsCval {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        ((self.bits() >> Self::COMPAREVALUE_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & (Self::COMPAREVALUE_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTHVS_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsTval: u32 {
    }
}

impl CnthvsTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CNTHV_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CnthvCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTHV_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvCval: u64 {
    }
}

impl CnthvCval {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        ((self.bits() >> Self::COMPAREVALUE_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & (Self::COMPAREVALUE_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTHV_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvTval: u32 {
    }
}

impl CnthvTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CNTKCTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntkctl: u32 {
        /// `PL0PCTEN` bit.
        const PL0PCTEN = 1 << 0;
        /// `PL0VCTEN` bit.
        const PL0VCTEN = 1 << 1;
        /// `EVNTEN` bit.
        const EVNTEN = 1 << 2;
        /// `EVNTDIR` bit.
        const EVNTDIR = 1 << 3;
        /// `PL0VTEN` bit.
        const PL0VTEN = 1 << 8;
        /// `PL0PTEN` bit.
        const PL0PTEN = 1 << 9;
        /// `EVNTIS` bit.
        const EVNTIS = 1 << 17;
    }
}

impl Cntkctl {
    /// Offset of the `PL0PCTEN` field.
    pub const PL0PCTEN_SHIFT: u32 = 0;
    /// Offset of the `PL0VCTEN` field.
    pub const PL0VCTEN_SHIFT: u32 = 1;
    /// Offset of the `EVNTEN` field.
    pub const EVNTEN_SHIFT: u32 = 2;
    /// Offset of the `EVNTDIR` field.
    pub const EVNTDIR_SHIFT: u32 = 3;
    /// Offset of the `EVNTI` field.
    pub const EVNTI_SHIFT: u32 = 4;
    /// Mask for the `EVNTI` field.
    pub const EVNTI_MASK: u32 = 0b1111;
    /// Offset of the `PL0VTEN` field.
    pub const PL0VTEN_SHIFT: u32 = 8;
    /// Offset of the `PL0PTEN` field.
    pub const PL0PTEN_SHIFT: u32 = 9;
    /// Offset of the `EVNTIS` field.
    pub const EVNTIS_SHIFT: u32 = 17;

    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        ((self.bits() >> Self::EVNTI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EVNTI` field.
    pub const fn set_evnti(&mut self, value: u8) {
        let offset = Self::EVNTI_SHIFT;
        assert!(value & (Self::EVNTI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVNTI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CNTPCT` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntpct: u64 {
    }
}

impl Cntpct {
    /// Offset of the `PhysicalCount` field.
    pub const PHYSICALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `PhysicalCount` field.
    pub const PHYSICALCOUNT_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `PhysicalCount` field.
    pub const fn physicalcount(self) -> u64 {
        ((self.bits() >> Self::PHYSICALCOUNT_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `PhysicalCount` field.
    pub const fn set_physicalcount(&mut self, value: u64) {
        let offset = Self::PHYSICALCOUNT_SHIFT;
        assert!(value & (Self::PHYSICALCOUNT_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYSICALCOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTPCTSS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntpctss: u64 {
    }
}

impl Cntpctss {
    /// Offset of the `SSPhysicalCount` field.
    pub const SSPHYSICALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `SSPhysicalCount` field.
    pub const SSPHYSICALCOUNT_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `SSPhysicalCount` field.
    pub const fn ssphysicalcount(self) -> u64 {
        ((self.bits() >> Self::SSPHYSICALCOUNT_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `SSPhysicalCount` field.
    pub const fn set_ssphysicalcount(&mut self, value: u64) {
        let offset = Self::SSPHYSICALCOUNT_SHIFT;
        assert!(value & (Self::SSPHYSICALCOUNT_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SSPHYSICALCOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTPCT_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpctEl0: u64 {
    }
}

impl CntpctEl0 {
    /// Offset of the `PhysicalCount` field.
    pub const PHYSICALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `PhysicalCount` field.
    pub const PHYSICALCOUNT_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `PhysicalCount` field.
    pub const fn physicalcount(self) -> u64 {
        ((self.bits() >> Self::PHYSICALCOUNT_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `PhysicalCount` field.
    pub const fn set_physicalcount(&mut self, value: u64) {
        let offset = Self::PHYSICALCOUNT_SHIFT;
        assert!(value & (Self::PHYSICALCOUNT_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYSICALCOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTP_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CntpCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTP_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpCval: u64 {
    }
}

impl CntpCval {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        ((self.bits() >> Self::COMPAREVALUE_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & (Self::COMPAREVALUE_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTP_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpTval: u32 {
    }
}

impl CntpTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CNTVCT` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntvct: u64 {
    }
}

impl Cntvct {
    /// Offset of the `VirtualCount` field.
    pub const VIRTUALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `VirtualCount` field.
    pub const VIRTUALCOUNT_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VirtualCount` field.
    pub const fn virtualcount(self) -> u64 {
        ((self.bits() >> Self::VIRTUALCOUNT_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `VirtualCount` field.
    pub const fn set_virtualcount(&mut self, value: u64) {
        let offset = Self::VIRTUALCOUNT_SHIFT;
        assert!(value & (Self::VIRTUALCOUNT_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRTUALCOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTVCTSS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntvctss: u64 {
    }
}

impl Cntvctss {
    /// Offset of the `SSVirtualCount` field.
    pub const SSVIRTUALCOUNT_SHIFT: u32 = 0;
    /// Mask for the `SSVirtualCount` field.
    pub const SSVIRTUALCOUNT_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `SSVirtualCount` field.
    pub const fn ssvirtualcount(self) -> u64 {
        ((self.bits() >> Self::SSVIRTUALCOUNT_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `SSVirtualCount` field.
    pub const fn set_ssvirtualcount(&mut self, value: u64) {
        let offset = Self::SSVIRTUALCOUNT_SHIFT;
        assert!(value & (Self::SSVIRTUALCOUNT_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SSVIRTUALCOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTVOFF` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntvoff: u64 {
    }
}

impl Cntvoff {
    /// Offset of the `VOffset` field.
    pub const VOFFSET_SHIFT: u32 = 0;
    /// Mask for the `VOffset` field.
    pub const VOFFSET_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        ((self.bits() >> Self::VOFFSET_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `VOffset` field.
    pub const fn set_voffset(&mut self, value: u64) {
        let offset = Self::VOFFSET_SHIFT;
        assert!(value & (Self::VOFFSET_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VOFFSET_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTVOFF_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvoffEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CntvoffEl2 {
    /// Offset of the `VOffset` field.
    pub const VOFFSET_SHIFT: u32 = 0;
    /// Mask for the `VOffset` field.
    pub const VOFFSET_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        ((self.bits() >> Self::VOFFSET_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `VOffset` field.
    pub const fn set_voffset(&mut self, value: u64) {
        let offset = Self::VOFFSET_SHIFT;
        assert!(value & (Self::VOFFSET_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VOFFSET_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTV_CTL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvCtl: u32 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

impl CntvCtl {
    /// Offset of the `ENABLE` field.
    pub const ENABLE_SHIFT: u32 = 0;
    /// Offset of the `IMASK` field.
    pub const IMASK_SHIFT: u32 = 1;
    /// Offset of the `ISTATUS` field.
    pub const ISTATUS_SHIFT: u32 = 2;
}

bitflags! {
    /// `CNTV_CVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvCval: u64 {
    }
}

impl CntvCval {
    /// Offset of the `CompareValue` field.
    pub const COMPAREVALUE_SHIFT: u32 = 0;
    /// Mask for the `CompareValue` field.
    pub const COMPAREVALUE_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        ((self.bits() >> Self::COMPAREVALUE_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `CompareValue` field.
    pub const fn set_comparevalue(&mut self, value: u64) {
        let offset = Self::COMPAREVALUE_SHIFT;
        assert!(value & (Self::COMPAREVALUE_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COMPAREVALUE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CNTV_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvTval: u32 {
    }
}

impl CntvTval {
    /// Offset of the `TimerValue` field.
    pub const TIMERVALUE_SHIFT: u32 = 0;
    /// Mask for the `TimerValue` field.
    pub const TIMERVALUE_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        ((self.bits() >> Self::TIMERVALUE_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TimerValue` field.
    pub const fn set_timervalue(&mut self, value: u32) {
        let offset = Self::TIMERVALUE_SHIFT;
        assert!(value & (Self::TIMERVALUE_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIMERVALUE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CONTEXTIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Contextidr: u32 {
    }
}

impl Contextidr {
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 0;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u32 = 0b11111111;

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        ((self.bits() >> Self::ASID_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u8) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CONTEXTIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ContextidrEl1 {
    /// Offset of the `PROCID` field.
    pub const PROCID_SHIFT: u32 = 0;
    /// Mask for the `PROCID` field.
    pub const PROCID_MASK: u64 = 0b11111111111111111111111111111111;

    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        ((self.bits() >> Self::PROCID_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `PROCID` field.
    pub const fn set_procid(&mut self, value: u32) {
        let offset = Self::PROCID_SHIFT;
        assert!(value & (Self::PROCID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PROCID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CONTEXTIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ContextidrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl ContextidrEl2 {
    /// Offset of the `PROCID` field.
    pub const PROCID_SHIFT: u32 = 0;
    /// Mask for the `PROCID` field.
    pub const PROCID_MASK: u64 = 0b11111111111111111111111111111111;

    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        ((self.bits() >> Self::PROCID_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `PROCID` field.
    pub const fn set_procid(&mut self, value: u32) {
        let offset = Self::PROCID_SHIFT;
        assert!(value & (Self::PROCID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PROCID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CPACR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cpacr: u32 {
        /// `TRCDIS` bit.
        const TRCDIS = 1 << 28;
        /// `ASEDIS` bit.
        const ASEDIS = 1 << 31;
    }
}

impl Cpacr {
    /// Offset of the `cp10` field.
    pub const CP10_SHIFT: u32 = 20;
    /// Mask for the `cp10` field.
    pub const CP10_MASK: u32 = 0b11;
    /// Offset of the `cp11` field.
    pub const CP11_SHIFT: u32 = 22;
    /// Mask for the `cp11` field.
    pub const CP11_MASK: u32 = 0b11;
    /// Offset of the `TRCDIS` field.
    pub const TRCDIS_SHIFT: u32 = 28;
    /// Offset of the `ASEDIS` field.
    pub const ASEDIS_SHIFT: u32 = 31;

    /// Returns the value of the `cp10` field.
    pub const fn cp10(self) -> u8 {
        ((self.bits() >> Self::CP10_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `cp10` field.
    pub const fn set_cp10(&mut self, value: u8) {
        let offset = Self::CP10_SHIFT;
        assert!(value & (Self::CP10_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CP10_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `cp11` field.
    pub const fn cp11(self) -> u8 {
        ((self.bits() >> Self::CP11_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `cp11` field.
    pub const fn set_cp11(&mut self, value: u8) {
        let offset = Self::CP11_SHIFT;
        assert!(value & (Self::CP11_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CP11_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CPACR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `TTA` field.
    pub const TTA_SHIFT: u32 = 28;
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
        ((self.bits() >> Self::ZEN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ZEN` field.
    pub const fn set_zen(&mut self, value: u8) {
        let offset = Self::ZEN_SHIFT;
        assert!(value & (Self::ZEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ZEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        ((self.bits() >> Self::FPEN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `FPEN` field.
    pub const fn set_fpen(&mut self, value: u8) {
        let offset = Self::FPEN_SHIFT;
        assert!(value & (Self::FPEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FPEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        ((self.bits() >> Self::SMEN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SMEN` field.
    pub const fn set_smen(&mut self, value: u8) {
        let offset = Self::SMEN_SHIFT;
        assert!(value & (Self::SMEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMEN_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CPTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        ((self.bits() >> Self::ZEN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ZEN` field.
    pub const fn set_zen(&mut self, value: u8) {
        let offset = Self::ZEN_SHIFT;
        assert!(value & (Self::ZEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ZEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        ((self.bits() >> Self::FPEN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `FPEN` field.
    pub const fn set_fpen(&mut self, value: u8) {
        let offset = Self::FPEN_SHIFT;
        assert!(value & (Self::FPEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FPEN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        ((self.bits() >> Self::SMEN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SMEN` field.
    pub const fn set_smen(&mut self, value: u8) {
        let offset = Self::SMEN_SHIFT;
        assert!(value & (Self::SMEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMEN_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
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
    /// `CSSELR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Csselr: u32 {
        /// `InD` bit.
        const IND = 1 << 0;
    }
}

impl Csselr {
    /// Offset of the `InD` field.
    pub const IND_SHIFT: u32 = 0;
    /// Offset of the `Level` field.
    pub const LEVEL_SHIFT: u32 = 1;
    /// Mask for the `Level` field.
    pub const LEVEL_MASK: u32 = 0b111;

    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        ((self.bits() >> Self::LEVEL_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `Level` field.
    pub const fn set_level(&mut self, value: u8) {
        let offset = Self::LEVEL_SHIFT;
        assert!(value & (Self::LEVEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEVEL_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CSSELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `InD` field.
    pub const IND_SHIFT: u32 = 0;
    /// Offset of the `Level` field.
    pub const LEVEL_SHIFT: u32 = 1;
    /// Mask for the `Level` field.
    pub const LEVEL_MASK: u64 = 0b111;
    /// Offset of the `TnD` field.
    pub const TND_SHIFT: u32 = 4;

    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        ((self.bits() >> Self::LEVEL_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `Level` field.
    pub const fn set_level(&mut self, value: u8) {
        let offset = Self::LEVEL_SHIFT;
        assert!(value & (Self::LEVEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEVEL_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ctr: u32 {
        /// RES1 bits in the `CTR` register.
        const RES1 = 0b10000000000000000000000000000000;
        /// `IDC` bit.
        const IDC = 1 << 28;
        /// `DIC` bit.
        const DIC = 1 << 29;
    }
}

impl Ctr {
    /// Offset of the `IminLine` field.
    pub const IMINLINE_SHIFT: u32 = 0;
    /// Mask for the `IminLine` field.
    pub const IMINLINE_MASK: u32 = 0b1111;
    /// Offset of the `L1Ip` field.
    pub const L1IP_SHIFT: u32 = 14;
    /// Mask for the `L1Ip` field.
    pub const L1IP_MASK: u32 = 0b11;
    /// Offset of the `DminLine` field.
    pub const DMINLINE_SHIFT: u32 = 16;
    /// Mask for the `DminLine` field.
    pub const DMINLINE_MASK: u32 = 0b1111;
    /// Offset of the `ERG` field.
    pub const ERG_SHIFT: u32 = 20;
    /// Mask for the `ERG` field.
    pub const ERG_MASK: u32 = 0b1111;
    /// Offset of the `CWG` field.
    pub const CWG_SHIFT: u32 = 24;
    /// Mask for the `CWG` field.
    pub const CWG_MASK: u32 = 0b1111;
    /// Offset of the `IDC` field.
    pub const IDC_SHIFT: u32 = 28;
    /// Offset of the `DIC` field.
    pub const DIC_SHIFT: u32 = 29;

    /// Returns the value of the `IminLine` field.
    pub const fn iminline(self) -> u8 {
        ((self.bits() >> Self::IMINLINE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `IminLine` field.
    pub const fn set_iminline(&mut self, value: u8) {
        let offset = Self::IMINLINE_SHIFT;
        assert!(value & (Self::IMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMINLINE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1Ip` field.
    pub const fn l1ip(self) -> u8 {
        ((self.bits() >> Self::L1IP_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `L1Ip` field.
    pub const fn set_l1ip(&mut self, value: u8) {
        let offset = Self::L1IP_SHIFT;
        assert!(value & (Self::L1IP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1IP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `DminLine` field.
    pub const fn dminline(self) -> u8 {
        ((self.bits() >> Self::DMINLINE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DminLine` field.
    pub const fn set_dminline(&mut self, value: u8) {
        let offset = Self::DMINLINE_SHIFT;
        assert!(value & (Self::DMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DMINLINE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `ERG` field.
    pub const fn erg(self) -> u8 {
        ((self.bits() >> Self::ERG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ERG` field.
    pub const fn set_erg(&mut self, value: u8) {
        let offset = Self::ERG_SHIFT;
        assert!(value & (Self::ERG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CWG` field.
    pub const fn cwg(self) -> u8 {
        ((self.bits() >> Self::CWG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CWG` field.
    pub const fn set_cwg(&mut self, value: u8) {
        let offset = Self::CWG_SHIFT;
        assert!(value & (Self::CWG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CWG_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `CTR_EL0` system register value.
    ///
    /// Cache Type Register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `IminLine` field.
    pub const IMINLINE_SHIFT: u32 = 0;
    /// Mask for the `IminLine` field.
    pub const IMINLINE_MASK: u64 = 0b1111;
    /// Offset of the `L1Ip` field.
    pub const L1IP_SHIFT: u32 = 14;
    /// Mask for the `L1Ip` field.
    pub const L1IP_MASK: u64 = 0b11;
    /// Offset of the `DminLine` field.
    pub const DMINLINE_SHIFT: u32 = 16;
    /// Mask for the `DminLine` field.
    pub const DMINLINE_MASK: u64 = 0b1111;
    /// Offset of the `ERG` field.
    pub const ERG_SHIFT: u32 = 20;
    /// Mask for the `ERG` field.
    pub const ERG_MASK: u64 = 0b1111;
    /// Offset of the `CWG` field.
    pub const CWG_SHIFT: u32 = 24;
    /// Mask for the `CWG` field.
    pub const CWG_MASK: u64 = 0b1111;
    /// Offset of the `IDC` field.
    pub const IDC_SHIFT: u32 = 28;
    /// Offset of the `DIC` field.
    pub const DIC_SHIFT: u32 = 29;
    /// Offset of the `TminLine` field.
    pub const TMINLINE_SHIFT: u32 = 32;
    /// Mask for the `TminLine` field.
    pub const TMINLINE_MASK: u64 = 0b111111;

    /// Returns the value of the `IminLine` field.
    pub const fn iminline(self) -> u8 {
        ((self.bits() >> Self::IMINLINE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `IminLine` field.
    pub const fn set_iminline(&mut self, value: u8) {
        let offset = Self::IMINLINE_SHIFT;
        assert!(value & (Self::IMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMINLINE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `L1Ip` field.
    pub const fn l1ip(self) -> u8 {
        ((self.bits() >> Self::L1IP_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `L1Ip` field.
    pub const fn set_l1ip(&mut self, value: u8) {
        let offset = Self::L1IP_SHIFT;
        assert!(value & (Self::L1IP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1IP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `DminLine` field.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn dminline(self) -> u8 {
        ((self.bits() >> Self::DMINLINE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DminLine` field.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn set_dminline(&mut self, value: u8) {
        let offset = Self::DMINLINE_SHIFT;
        assert!(value & (Self::DMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DMINLINE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ERG` field.
    pub const fn erg(self) -> u8 {
        ((self.bits() >> Self::ERG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ERG` field.
    pub const fn set_erg(&mut self, value: u8) {
        let offset = Self::ERG_SHIFT;
        assert!(value & (Self::ERG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CWG` field.
    pub const fn cwg(self) -> u8 {
        ((self.bits() >> Self::CWG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CWG` field.
    pub const fn set_cwg(&mut self, value: u8) {
        let offset = Self::CWG_SHIFT;
        assert!(value & (Self::CWG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CWG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TminLine` field.
    pub const fn tminline(self) -> u8 {
        ((self.bits() >> Self::TMINLINE_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `TminLine` field.
    pub const fn set_tminline(&mut self, value: u8) {
        let offset = Self::TMINLINE_SHIFT;
        assert!(value & (Self::TMINLINE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TMINLINE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `CurrentEL` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Currentel: u64 {
    }
}

impl Currentel {
    /// Offset of the `EL` field.
    pub const EL_SHIFT: u32 = 2;
    /// Mask for the `EL` field.
    pub const EL_MASK: u64 = 0b11;

    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        ((self.bits() >> Self::EL_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `EL` field.
    pub const fn set_el(&mut self, value: u8) {
        let offset = Self::EL_SHIFT;
        assert!(value & (Self::EL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `DACR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dacr: u32 {
    }
}

impl Dacr {
    /// Offset of the `D<n>` field.
    pub const D_SHIFT: u32 = 0;
    /// Mask for the `D<n>` field.
    pub const D_MASK: u32 = 0b11;

    /// Returns the value of the given `D<n>` field.
    pub const fn d(self, n: u32) -> u8 {
        assert!(n < 16);
        ((self.bits() >> (Self::D_SHIFT + (n - 0) * 2)) & 0b11) as u8
    }

    /// Sets the value of the `D<n>` field.
    pub const fn set_d(&mut self, n: u32, value: u8) {
        assert!(n < 16);
        let offset = Self::D_SHIFT + (n - 0) * 2;
        assert!(value & (Self::D_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::D_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGAUTHSTATUS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgauthstatus: u32 {
    }
}

impl Dbgauthstatus {
    /// Offset of the `NSID` field.
    pub const NSID_SHIFT: u32 = 0;
    /// Mask for the `NSID` field.
    pub const NSID_MASK: u32 = 0b11;
    /// Offset of the `NSNID` field.
    pub const NSNID_SHIFT: u32 = 2;
    /// Mask for the `NSNID` field.
    pub const NSNID_MASK: u32 = 0b11;
    /// Offset of the `SID` field.
    pub const SID_SHIFT: u32 = 4;
    /// Mask for the `SID` field.
    pub const SID_MASK: u32 = 0b11;
    /// Offset of the `SNID` field.
    pub const SNID_SHIFT: u32 = 6;
    /// Mask for the `SNID` field.
    pub const SNID_MASK: u32 = 0b11;

    /// Returns the value of the `NSID` field.
    pub const fn nsid(self) -> u8 {
        ((self.bits() >> Self::NSID_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `NSID` field.
    pub const fn set_nsid(&mut self, value: u8) {
        let offset = Self::NSID_SHIFT;
        assert!(value & (Self::NSID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `NSNID` field.
    pub const fn nsnid(self) -> u8 {
        ((self.bits() >> Self::NSNID_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `NSNID` field.
    pub const fn set_nsnid(&mut self, value: u8) {
        let offset = Self::NSNID_SHIFT;
        assert!(value & (Self::NSNID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSNID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SID` field.
    pub const fn sid(self) -> u8 {
        ((self.bits() >> Self::SID_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SID` field.
    pub const fn set_sid(&mut self, value: u8) {
        let offset = Self::SID_SHIFT;
        assert!(value & (Self::SID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SID_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SNID` field.
    pub const fn snid(self) -> u8 {
        ((self.bits() >> Self::SNID_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SNID` field.
    pub const fn set_snid(&mut self, value: u8) {
        let offset = Self::SNID_SHIFT;
        assert!(value & (Self::SNID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SNID_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGCLAIMCLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgclaimclr: u32 {
        /// `CLAIM<m>` bit 0.
        const CLAIM0 = 1 << 0;
        /// `CLAIM<m>` bit 1.
        const CLAIM1 = 1 << 1;
        /// `CLAIM<m>` bit 2.
        const CLAIM2 = 1 << 2;
        /// `CLAIM<m>` bit 3.
        const CLAIM3 = 1 << 3;
        /// `CLAIM<m>` bit 4.
        const CLAIM4 = 1 << 4;
        /// `CLAIM<m>` bit 5.
        const CLAIM5 = 1 << 5;
        /// `CLAIM<m>` bit 6.
        const CLAIM6 = 1 << 6;
        /// `CLAIM<m>` bit 7.
        const CLAIM7 = 1 << 7;
    }
}

impl Dbgclaimclr {
    /// Offset of the `CLAIM<m>` field.
    pub const CLAIM_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGCLAIMSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgclaimset: u32 {
        /// `CLAIM<m>` bit 0.
        const CLAIM0 = 1 << 0;
        /// `CLAIM<m>` bit 1.
        const CLAIM1 = 1 << 1;
        /// `CLAIM<m>` bit 2.
        const CLAIM2 = 1 << 2;
        /// `CLAIM<m>` bit 3.
        const CLAIM3 = 1 << 3;
        /// `CLAIM<m>` bit 4.
        const CLAIM4 = 1 << 4;
        /// `CLAIM<m>` bit 5.
        const CLAIM5 = 1 << 5;
        /// `CLAIM<m>` bit 6.
        const CLAIM6 = 1 << 6;
        /// `CLAIM<m>` bit 7.
        const CLAIM7 = 1 << 7;
    }
}

impl Dbgclaimset {
    /// Offset of the `CLAIM<m>` field.
    pub const CLAIM_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGDCCINT` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdccint: u32 {
        /// `TX` bit.
        const TX = 1 << 29;
        /// `RX` bit.
        const RX = 1 << 30;
    }
}

impl Dbgdccint {
    /// Offset of the `TX` field.
    pub const TX_SHIFT: u32 = 29;
    /// Offset of the `RX` field.
    pub const RX_SHIFT: u32 = 30;
}

bitflags! {
    /// `DBGDEVID` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdevid: u32 {
    }
}

impl Dbgdevid {
    /// Offset of the `PCSample` field.
    pub const PCSAMPLE_SHIFT: u32 = 0;
    /// Mask for the `PCSample` field.
    pub const PCSAMPLE_MASK: u32 = 0b1111;
    /// Offset of the `WPAddrMask` field.
    pub const WPADDRMASK_SHIFT: u32 = 4;
    /// Mask for the `WPAddrMask` field.
    pub const WPADDRMASK_MASK: u32 = 0b1111;
    /// Offset of the `BPAddrMask` field.
    pub const BPADDRMASK_SHIFT: u32 = 8;
    /// Mask for the `BPAddrMask` field.
    pub const BPADDRMASK_MASK: u32 = 0b1111;
    /// Offset of the `VectorCatch` field.
    pub const VECTORCATCH_SHIFT: u32 = 12;
    /// Mask for the `VectorCatch` field.
    pub const VECTORCATCH_MASK: u32 = 0b1111;
    /// Offset of the `VirtExtns` field.
    pub const VIRTEXTNS_SHIFT: u32 = 16;
    /// Mask for the `VirtExtns` field.
    pub const VIRTEXTNS_MASK: u32 = 0b1111;
    /// Offset of the `DoubleLock` field.
    pub const DOUBLELOCK_SHIFT: u32 = 20;
    /// Mask for the `DoubleLock` field.
    pub const DOUBLELOCK_MASK: u32 = 0b1111;
    /// Offset of the `AuxRegs` field.
    pub const AUXREGS_SHIFT: u32 = 24;
    /// Mask for the `AuxRegs` field.
    pub const AUXREGS_MASK: u32 = 0b1111;
    /// Offset of the `CIDMask` field.
    pub const CIDMASK_SHIFT: u32 = 28;
    /// Mask for the `CIDMask` field.
    pub const CIDMASK_MASK: u32 = 0b1111;

    /// Returns the value of the `PCSample` field.
    pub const fn pcsample(self) -> u8 {
        ((self.bits() >> Self::PCSAMPLE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PCSample` field.
    pub const fn set_pcsample(&mut self, value: u8) {
        let offset = Self::PCSAMPLE_SHIFT;
        assert!(value & (Self::PCSAMPLE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PCSAMPLE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `WPAddrMask` field.
    pub const fn wpaddrmask(self) -> u8 {
        ((self.bits() >> Self::WPADDRMASK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `WPAddrMask` field.
    pub const fn set_wpaddrmask(&mut self, value: u8) {
        let offset = Self::WPADDRMASK_SHIFT;
        assert!(value & (Self::WPADDRMASK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WPADDRMASK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BPAddrMask` field.
    pub const fn bpaddrmask(self) -> u8 {
        ((self.bits() >> Self::BPADDRMASK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BPAddrMask` field.
    pub const fn set_bpaddrmask(&mut self, value: u8) {
        let offset = Self::BPADDRMASK_SHIFT;
        assert!(value & (Self::BPADDRMASK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BPADDRMASK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `VectorCatch` field.
    pub const fn vectorcatch(self) -> u8 {
        ((self.bits() >> Self::VECTORCATCH_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `VectorCatch` field.
    pub const fn set_vectorcatch(&mut self, value: u8) {
        let offset = Self::VECTORCATCH_SHIFT;
        assert!(value & (Self::VECTORCATCH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VECTORCATCH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `VirtExtns` field.
    pub const fn virtextns(self) -> u8 {
        ((self.bits() >> Self::VIRTEXTNS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `VirtExtns` field.
    pub const fn set_virtextns(&mut self, value: u8) {
        let offset = Self::VIRTEXTNS_SHIFT;
        assert!(value & (Self::VIRTEXTNS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRTEXTNS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `DoubleLock` field.
    pub const fn doublelock(self) -> u8 {
        ((self.bits() >> Self::DOUBLELOCK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DoubleLock` field.
    pub const fn set_doublelock(&mut self, value: u8) {
        let offset = Self::DOUBLELOCK_SHIFT;
        assert!(value & (Self::DOUBLELOCK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DOUBLELOCK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `AuxRegs` field.
    pub const fn auxregs(self) -> u8 {
        ((self.bits() >> Self::AUXREGS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AuxRegs` field.
    pub const fn set_auxregs(&mut self, value: u8) {
        let offset = Self::AUXREGS_SHIFT;
        assert!(value & (Self::AUXREGS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AUXREGS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CIDMask` field.
    pub const fn cidmask(self) -> u8 {
        ((self.bits() >> Self::CIDMASK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CIDMask` field.
    pub const fn set_cidmask(&mut self, value: u8) {
        let offset = Self::CIDMASK_SHIFT;
        assert!(value & (Self::CIDMASK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CIDMASK_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDEVID1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdevid1: u32 {
    }
}

impl Dbgdevid1 {
    /// Offset of the `PCSROffset` field.
    pub const PCSROFFSET_SHIFT: u32 = 0;
    /// Mask for the `PCSROffset` field.
    pub const PCSROFFSET_MASK: u32 = 0b1111;

    /// Returns the value of the `PCSROffset` field.
    pub const fn pcsroffset(self) -> u8 {
        ((self.bits() >> Self::PCSROFFSET_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PCSROffset` field.
    pub const fn set_pcsroffset(&mut self, value: u8) {
        let offset = Self::PCSROFFSET_SHIFT;
        assert!(value & (Self::PCSROFFSET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PCSROFFSET_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdidr: u32 {
        /// RES1 bits in the `DBGDIDR` register.
        const RES1 = 0b1000000000000000;
        /// `SE_imp` bit.
        const SE_IMP = 1 << 12;
        /// `nSUHD_imp` bit.
        const NSUHD_IMP = 1 << 14;
    }
}

impl Dbgdidr {
    /// Offset of the `SE_imp` field.
    pub const SE_IMP_SHIFT: u32 = 12;
    /// Offset of the `nSUHD_imp` field.
    pub const NSUHD_IMP_SHIFT: u32 = 14;
    /// Offset of the `Version` field.
    pub const VERSION_SHIFT: u32 = 16;
    /// Mask for the `Version` field.
    pub const VERSION_MASK: u32 = 0b1111;
    /// Offset of the `CTX_CMPs` field.
    pub const CTX_CMPS_SHIFT: u32 = 20;
    /// Mask for the `CTX_CMPs` field.
    pub const CTX_CMPS_MASK: u32 = 0b1111;
    /// Offset of the `BRPs` field.
    pub const BRPS_SHIFT: u32 = 24;
    /// Mask for the `BRPs` field.
    pub const BRPS_MASK: u32 = 0b1111;
    /// Offset of the `WRPs` field.
    pub const WRPS_SHIFT: u32 = 28;
    /// Mask for the `WRPs` field.
    pub const WRPS_MASK: u32 = 0b1111;

    /// Returns the value of the `Version` field.
    pub const fn version(self) -> u8 {
        ((self.bits() >> Self::VERSION_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Version` field.
    pub const fn set_version(&mut self, value: u8) {
        let offset = Self::VERSION_SHIFT;
        assert!(value & (Self::VERSION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VERSION_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        ((self.bits() >> Self::CTX_CMPS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CTX_CMPs` field.
    pub const fn set_ctx_cmps(&mut self, value: u8) {
        let offset = Self::CTX_CMPS_SHIFT;
        assert!(value & (Self::CTX_CMPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTX_CMPS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        ((self.bits() >> Self::BRPS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BRPs` field.
    pub const fn set_brps(&mut self, value: u8) {
        let offset = Self::BRPS_SHIFT;
        assert!(value & (Self::BRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BRPS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        ((self.bits() >> Self::WRPS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `WRPs` field.
    pub const fn set_wrps(&mut self, value: u8) {
        let offset = Self::WRPS_SHIFT;
        assert!(value & (Self::WRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WRPS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDRAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdrar: u64 {
    }
}

impl Dbgdrar {
    /// Offset of the `Valid` field.
    pub const VALID_SHIFT: u32 = 0;
    /// Mask for the `Valid` field.
    pub const VALID_MASK: u64 = 0b11;
    /// Offset of the `ROMADDR[47:12]` field.
    pub const ROMADDR_47_12_SHIFT: u32 = 12;
    /// Mask for the `ROMADDR[47:12]` field.
    pub const ROMADDR_47_12_MASK: u64 = 0b111111111111111111111111111111111111;

    /// Returns the value of the `Valid` field.
    pub const fn valid(self) -> u8 {
        ((self.bits() >> Self::VALID_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `Valid` field.
    pub const fn set_valid(&mut self, value: u8) {
        let offset = Self::VALID_SHIFT;
        assert!(value & (Self::VALID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VALID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ROMADDR[47:12]` field.
    pub const fn romaddr_47_12(self) -> u64 {
        ((self.bits() >> Self::ROMADDR_47_12_SHIFT) & 0b111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `ROMADDR[47:12]` field.
    pub const fn set_romaddr_47_12(&mut self, value: u64) {
        let offset = Self::ROMADDR_47_12_SHIFT;
        assert!(value & (Self::ROMADDR_47_12_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ROMADDR_47_12_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `DBGDSCRext` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdscrext: u32 {
        /// `ERR` bit.
        const ERR = 1 << 6;
        /// `UDCCdis` bit.
        const UDCCDIS = 1 << 12;
        /// `HDE` bit.
        const HDE = 1 << 14;
        /// `MDBGen` bit.
        const MDBGEN = 1 << 15;
        /// `SPIDdis` bit.
        const SPIDDIS = 1 << 16;
        /// `SPNIDdis` bit.
        const SPNIDDIS = 1 << 17;
        /// `NS` bit.
        const NS = 1 << 18;
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
    }
}

impl Dbgdscrext {
    /// Offset of the `MOE` field.
    pub const MOE_SHIFT: u32 = 2;
    /// Mask for the `MOE` field.
    pub const MOE_MASK: u32 = 0b1111;
    /// Offset of the `ERR` field.
    pub const ERR_SHIFT: u32 = 6;
    /// Offset of the `UDCCdis` field.
    pub const UDCCDIS_SHIFT: u32 = 12;
    /// Offset of the `HDE` field.
    pub const HDE_SHIFT: u32 = 14;
    /// Offset of the `MDBGen` field.
    pub const MDBGEN_SHIFT: u32 = 15;
    /// Offset of the `SPIDdis` field.
    pub const SPIDDIS_SHIFT: u32 = 16;
    /// Offset of the `SPNIDdis` field.
    pub const SPNIDDIS_SHIFT: u32 = 17;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 18;
    /// Offset of the `SC2` field.
    pub const SC2_SHIFT: u32 = 19;
    /// Offset of the `TDA` field.
    pub const TDA_SHIFT: u32 = 21;
    /// Offset of the `INTdis` field.
    pub const INTDIS_SHIFT: u32 = 22;
    /// Mask for the `INTdis` field.
    pub const INTDIS_MASK: u32 = 0b11;
    /// Offset of the `TXU` field.
    pub const TXU_SHIFT: u32 = 26;
    /// Offset of the `RXO` field.
    pub const RXO_SHIFT: u32 = 27;
    /// Offset of the `TXfull` field.
    pub const TXFULL_SHIFT: u32 = 29;
    /// Offset of the `RXfull` field.
    pub const RXFULL_SHIFT: u32 = 30;
    /// Offset of the `TFO` field.
    pub const TFO_SHIFT: u32 = 31;

    /// Returns the value of the `MOE` field.
    pub const fn moe(self) -> u8 {
        ((self.bits() >> Self::MOE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MOE` field.
    pub const fn set_moe(&mut self, value: u8) {
        let offset = Self::MOE_SHIFT;
        assert!(value & (Self::MOE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MOE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        ((self.bits() >> Self::INTDIS_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `INTdis` field.
    pub const fn set_intdis(&mut self, value: u8) {
        let offset = Self::INTDIS_SHIFT;
        assert!(value & (Self::INTDIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTDIS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDSCRint` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdscrint: u32 {
        /// `UDCCdis` bit.
        const UDCCDIS = 1 << 12;
        /// `MDBGen` bit.
        const MDBGEN = 1 << 15;
        /// `SPIDdis` bit.
        const SPIDDIS = 1 << 16;
        /// `SPNIDdis` bit.
        const SPNIDDIS = 1 << 17;
        /// `NS` bit.
        const NS = 1 << 18;
        /// `TXfull` bit.
        const TXFULL = 1 << 29;
        /// `RXfull` bit.
        const RXFULL = 1 << 30;
    }
}

impl Dbgdscrint {
    /// Offset of the `MOE` field.
    pub const MOE_SHIFT: u32 = 2;
    /// Mask for the `MOE` field.
    pub const MOE_MASK: u32 = 0b1111;
    /// Offset of the `UDCCdis` field.
    pub const UDCCDIS_SHIFT: u32 = 12;
    /// Offset of the `MDBGen` field.
    pub const MDBGEN_SHIFT: u32 = 15;
    /// Offset of the `SPIDdis` field.
    pub const SPIDDIS_SHIFT: u32 = 16;
    /// Offset of the `SPNIDdis` field.
    pub const SPNIDDIS_SHIFT: u32 = 17;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 18;
    /// Offset of the `TXfull` field.
    pub const TXFULL_SHIFT: u32 = 29;
    /// Offset of the `RXfull` field.
    pub const RXFULL_SHIFT: u32 = 30;

    /// Returns the value of the `MOE` field.
    pub const fn moe(self) -> u8 {
        ((self.bits() >> Self::MOE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MOE` field.
    pub const fn set_moe(&mut self, value: u8) {
        let offset = Self::MOE_SHIFT;
        assert!(value & (Self::MOE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MOE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDTRRXext` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrrxext: u32 {
    }
}

impl Dbgdtrrxext {
    /// Offset of the `DTRRX` field.
    pub const DTRRX_SHIFT: u32 = 0;
    /// Mask for the `DTRRX` field.
    pub const DTRRX_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        ((self.bits() >> Self::DTRRX_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `DTRRX` field.
    pub const fn set_dtrrx(&mut self, value: u32) {
        let offset = Self::DTRRX_SHIFT;
        assert!(value & (Self::DTRRX_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRRX_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDTRRXint` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrrxint: u32 {
    }
}

impl Dbgdtrrxint {
    /// Offset of the `DTRRX` field.
    pub const DTRRX_SHIFT: u32 = 0;
    /// Mask for the `DTRRX` field.
    pub const DTRRX_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        ((self.bits() >> Self::DTRRX_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `DTRRX` field.
    pub const fn set_dtrrx(&mut self, value: u32) {
        let offset = Self::DTRRX_SHIFT;
        assert!(value & (Self::DTRRX_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRRX_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDTRTXext` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrtxext: u32 {
    }
}

impl Dbgdtrtxext {
    /// Offset of the `DTRTX` field.
    pub const DTRTX_SHIFT: u32 = 0;
    /// Mask for the `DTRTX` field.
    pub const DTRTX_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        ((self.bits() >> Self::DTRTX_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `DTRTX` field.
    pub const fn set_dtrtx(&mut self, value: u32) {
        let offset = Self::DTRTX_SHIFT;
        assert!(value & (Self::DTRTX_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRTX_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGDTRTXint` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrtxint: u32 {
    }
}

impl Dbgdtrtxint {
    /// Offset of the `DTRTX` field.
    pub const DTRTX_SHIFT: u32 = 0;
    /// Mask for the `DTRTX` field.
    pub const DTRTX_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        ((self.bits() >> Self::DTRTX_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `DTRTX` field.
    pub const fn set_dtrtx(&mut self, value: u32) {
        let offset = Self::DTRTX_SHIFT;
        assert!(value & (Self::DTRTX_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DTRTX_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGOSDLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgosdlr: u32 {
        /// `DLK` bit.
        const DLK = 1 << 0;
    }
}

impl Dbgosdlr {
    /// Offset of the `DLK` field.
    pub const DLK_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGOSECCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoseccr: u32 {
    }
}

impl Dbgoseccr {
    /// Offset of the `EDECCR` field.
    pub const EDECCR_SHIFT: u32 = 0;
    /// Mask for the `EDECCR` field.
    pub const EDECCR_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `EDECCR` field.
    pub const fn edeccr(self) -> u32 {
        ((self.bits() >> Self::EDECCR_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `EDECCR` field.
    pub const fn set_edeccr(&mut self, value: u32) {
        let offset = Self::EDECCR_SHIFT;
        assert!(value & (Self::EDECCR_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EDECCR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGOSLAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoslar: u32 {
    }
}

impl Dbgoslar {
    /// Offset of the `OSLA` field.
    pub const OSLA_SHIFT: u32 = 0;
    /// Mask for the `OSLA` field.
    pub const OSLA_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `OSLA` field.
    pub const fn osla(self) -> u32 {
        ((self.bits() >> Self::OSLA_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `OSLA` field.
    pub const fn set_osla(&mut self, value: u32) {
        let offset = Self::OSLA_SHIFT;
        assert!(value & (Self::OSLA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OSLA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DBGOSLSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoslsr: u32 {
        /// `OSLK` bit.
        const OSLK = 1 << 1;
        /// `nTT` bit.
        const NTT = 1 << 2;
    }
}

impl Dbgoslsr {
    /// Offset of the `OSLK` field.
    pub const OSLK_SHIFT: u32 = 1;
    /// Offset of the `nTT` field.
    pub const NTT_SHIFT: u32 = 2;
}

bitflags! {
    /// `DBGPRCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgprcr: u32 {
        /// `CORENPDRQ` bit.
        const CORENPDRQ = 1 << 0;
    }
}

impl Dbgprcr {
    /// Offset of the `CORENPDRQ` field.
    pub const CORENPDRQ_SHIFT: u32 = 0;
}

bitflags! {
    /// `DBGVCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgvcr: u32 {
        /// `SU` bit.
        const SU = 1 << 1;
        /// `U` bit.
        const U = 1 << 1;
        /// `S` bit.
        const S = 1 << 2;
        /// `SS` bit.
        const SS = 1 << 2;
        /// `P` bit.
        const P = 1 << 3;
        /// `SP` bit.
        const SP = 1 << 3;
        /// `D` bit.
        const D = 1 << 4;
        /// `SD` bit.
        const SD = 1 << 4;
        /// `I` bit.
        const I = 1 << 6;
        /// `SI` bit.
        const SI = 1 << 6;
        /// `F` bit.
        const F = 1 << 7;
        /// `SF` bit.
        const SF = 1 << 7;
        /// `MS` bit.
        const MS = 1 << 10;
        /// `MP` bit.
        const MP = 1 << 11;
        /// `MD` bit.
        const MD = 1 << 12;
        /// `MI` bit.
        const MI = 1 << 14;
        /// `MF` bit.
        const MF = 1 << 15;
        /// `NSU` bit.
        const NSU = 1 << 25;
        /// `NSS` bit.
        const NSS = 1 << 26;
        /// `NSP` bit.
        const NSP = 1 << 27;
        /// `NSD` bit.
        const NSD = 1 << 28;
        /// `NSI` bit.
        const NSI = 1 << 30;
        /// `NSF` bit.
        const NSF = 1 << 31;
    }
}

impl Dbgvcr {
    /// Offset of the `SU` field.
    pub const SU_SHIFT: u32 = 1;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 1;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 2;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 2;
    /// Offset of the `P` field.
    pub const P_SHIFT: u32 = 3;
    /// Offset of the `SP` field.
    pub const SP_SHIFT: u32 = 3;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 4;
    /// Offset of the `SD` field.
    pub const SD_SHIFT: u32 = 4;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 6;
    /// Offset of the `SI` field.
    pub const SI_SHIFT: u32 = 6;
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 7;
    /// Offset of the `SF` field.
    pub const SF_SHIFT: u32 = 7;
    /// Offset of the `MS` field.
    pub const MS_SHIFT: u32 = 10;
    /// Offset of the `MP` field.
    pub const MP_SHIFT: u32 = 11;
    /// Offset of the `MD` field.
    pub const MD_SHIFT: u32 = 12;
    /// Offset of the `MI` field.
    pub const MI_SHIFT: u32 = 14;
    /// Offset of the `MF` field.
    pub const MF_SHIFT: u32 = 15;
    /// Offset of the `NSU` field.
    pub const NSU_SHIFT: u32 = 25;
    /// Offset of the `NSS` field.
    pub const NSS_SHIFT: u32 = 26;
    /// Offset of the `NSP` field.
    pub const NSP_SHIFT: u32 = 27;
    /// Offset of the `NSD` field.
    pub const NSD_SHIFT: u32 = 28;
    /// Offset of the `NSI` field.
    pub const NSI_SHIFT: u32 = 30;
    /// Offset of the `NSF` field.
    pub const NSF_SHIFT: u32 = 31;
}

bitflags! {
    /// `DFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dfar: u32 {
    }
}

impl Dfar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        ((self.bits() >> Self::VA_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & (Self::VA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DFSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dfsr: u32 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `WnR` bit.
        const WNR = 1 << 11;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `CM` bit.
        const CM = 1 << 13;
        /// `FnV` bit.
        const FNV = 1 << 16;
    }
}

impl Dfsr {
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b111111;
    /// Offset of the `Domain` field.
    pub const DOMAIN_SHIFT: u32 = 4;
    /// Mask for the `Domain` field.
    pub const DOMAIN_MASK: u32 = 0b1111;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `WnR` field.
    pub const WNR_SHIFT: u32 = 11;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `CM` field.
    pub const CM_SHIFT: u32 = 13;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u32 = 0b11;
    /// Offset of the `FnV` field.
    pub const FNV_SHIFT: u32 = 16;

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Domain` field.
    pub const fn domain(self) -> u8 {
        ((self.bits() >> Self::DOMAIN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Domain` field.
    pub const fn set_domain(&mut self, value: u8) {
        let offset = Self::DOMAIN_SHIFT;
        assert!(value & (Self::DOMAIN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DOMAIN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DISR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Disr: u32 {
        /// `EA` bit.
        const EA = 1 << 9;
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `A` bit.
        const A = 1 << 31;
    }
}

impl Disr {
    /// Offset of the `DFSC` field.
    pub const DFSC_SHIFT: u32 = 0;
    /// Mask for the `DFSC` field.
    pub const DFSC_MASK: u32 = 0b111111;
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b111111;
    /// Offset of the `EA` field.
    pub const EA_SHIFT: u32 = 9;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `DFSC` field.
    pub const fn dfsc(self) -> u8 {
        ((self.bits() >> Self::DFSC_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `DFSC` field.
    pub const fn set_dfsc(&mut self, value: u8) {
        let offset = Self::DFSC_SHIFT;
        assert!(value & (Self::DFSC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DFSC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `DISR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `DFSC` field.
    pub const DFSC_SHIFT: u32 = 0;
    /// Mask for the `DFSC` field.
    pub const DFSC_MASK: u64 = 0b111111;
    /// Offset of the `WnR` field.
    pub const WNR_SHIFT: u32 = 6;
    /// Offset of the `WnRV` field.
    pub const WNRV_SHIFT: u32 = 7;
    /// Offset of the `EA` field.
    pub const EA_SHIFT: u32 = 9;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 10;
    /// Mask for the `AET` field.
    pub const AET_MASK: u64 = 0b111;
    /// Offset of the `WU` field.
    pub const WU_SHIFT: u32 = 16;
    /// Mask for the `WU` field.
    pub const WU_MASK: u64 = 0b11;
    /// Offset of the `IDS` field.
    pub const IDS_SHIFT: u32 = 24;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `DFSC` field.
    pub const fn dfsc(self) -> u8 {
        ((self.bits() >> Self::DFSC_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `DFSC` field.
    pub const fn set_dfsc(&mut self, value: u8) {
        let offset = Self::DFSC_SHIFT;
        assert!(value & (Self::DFSC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DFSC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `WU` field.
    pub const fn wu(self) -> u8 {
        ((self.bits() >> Self::WU_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `WU` field.
    pub const fn set_wu(&mut self, value: u8) {
        let offset = Self::WU_SHIFT;
        assert!(value & (Self::WU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WU_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `DIT` system register value.
    ///
    /// Data Independent Timing.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dit: u64 {
        /// Enable data independent timing.
        const DIT = 1 << 24;
    }
}

impl Dit {
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
}

bitflags! {
    /// `DLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dlr: u32 {
    }
}

impl Dlr {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u32 {
        ((self.bits() >> Self::ADDR_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u32) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & (Self::ADDR_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DSPSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dspsr: u32 {
        /// `T` bit.
        const T = 1 << 5;
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `E` bit.
        const E = 1 << 9;
        /// `IL` bit.
        const IL = 1 << 20;
        /// `SS` bit.
        const SS = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `SSBS` bit.
        const SSBS = 1 << 23;
        /// `DIT` bit.
        const DIT = 1 << 24;
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
    }
}

impl Dspsr {
    /// Offset of the `M[4:0]` field.
    pub const M_4_0_SHIFT: u32 = 0;
    /// Mask for the `M[4:0]` field.
    pub const M_4_0_MASK: u32 = 0b11111;
    /// Offset of the `T` field.
    pub const T_SHIFT: u32 = 5;
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the `E` field.
    pub const E_SHIFT: u32 = 9;
    /// Offset of the `GE` field.
    pub const GE_SHIFT: u32 = 16;
    /// Mask for the `GE` field.
    pub const GE_MASK: u32 = 0b1111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 20;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 21;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 22;
    /// Offset of the `SSBS` field.
    pub const SSBS_SHIFT: u32 = 23;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
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

    /// Returns the value of the `M[4:0]` field.
    pub const fn m_4_0(self) -> u8 {
        ((self.bits() >> Self::M_4_0_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `M[4:0]` field.
    pub const fn set_m_4_0(&mut self, value: u8) {
        let offset = Self::M_4_0_SHIFT;
        assert!(value & (Self::M_4_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::M_4_0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> Self::GE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GE` field.
    pub const fn set_ge(&mut self, value: u8) {
        let offset = Self::GE_SHIFT;
        assert!(value & (Self::GE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `DSPSR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dspsr2: u32 {
        /// `PPEND` bit.
        const PPEND = 1 << 1;
        /// `UINJ` bit.
        const UINJ = 1 << 4;
    }
}

impl Dspsr2 {
    /// Offset of the `PPEND` field.
    pub const PPEND_SHIFT: u32 = 1;
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 4;
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ElrEl1 {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        ((self.bits() >> Self::ADDR_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u64) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & (Self::ADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ELR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl ElrEl2 {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        ((self.bits() >> Self::ADDR_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u64) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & (Self::ADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ELR_hyp` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrHyp: u32 {
    }
}

#[cfg(feature = "el2")]
impl ElrHyp {
    /// Offset of the `ADDR` field.
    pub const ADDR_SHIFT: u32 = 0;
    /// Mask for the `ADDR` field.
    pub const ADDR_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u32 {
        ((self.bits() >> Self::ADDR_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ADDR` field.
    pub const fn set_addr(&mut self, value: u32) {
        let offset = Self::ADDR_SHIFT;
        assert!(value & (Self::ADDR_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADDR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERRIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erridr: u32 {
    }
}

impl Erridr {
    /// Offset of the `NUM` field.
    pub const NUM_SHIFT: u32 = 0;
    /// Mask for the `NUM` field.
    pub const NUM_MASK: u32 = 0b1111111111111111;

    /// Returns the value of the `NUM` field.
    pub const fn num(self) -> u16 {
        ((self.bits() >> Self::NUM_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `NUM` field.
    pub const fn set_num(&mut self, value: u16) {
        let offset = Self::NUM_SHIFT;
        assert!(value & (Self::NUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NUM_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERRSELR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Errselr: u32 {
    }
}

impl Errselr {
    /// Offset of the `SEL` field.
    pub const SEL_SHIFT: u32 = 0;
    /// Mask for the `SEL` field.
    pub const SEL_MASK: u32 = 0b1111111111111111;

    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u16 {
        ((self.bits() >> Self::SEL_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `SEL` field.
    pub const fn set_sel(&mut self, value: u16) {
        let offset = Self::SEL_SHIFT;
        assert!(value & (Self::SEL_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEL_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXADDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxaddr: u32 {
    }
}

impl Erxaddr {
    /// Offset of the `ERRnADDRlo` field.
    pub const ERRNADDRLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnADDRlo` field.
    pub const ERRNADDRLO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnADDRlo` field.
    pub const fn errnaddrlo(self) -> u32 {
        ((self.bits() >> Self::ERRNADDRLO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnADDRlo` field.
    pub const fn set_errnaddrlo(&mut self, value: u32) {
        let offset = Self::ERRNADDRLO_SHIFT;
        assert!(value & (Self::ERRNADDRLO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNADDRLO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXADDR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxaddr2: u32 {
    }
}

impl Erxaddr2 {
    /// Offset of the `ERRnADDRhi` field.
    pub const ERRNADDRHI_SHIFT: u32 = 0;
    /// Mask for the `ERRnADDRhi` field.
    pub const ERRNADDRHI_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnADDRhi` field.
    pub const fn errnaddrhi(self) -> u32 {
        ((self.bits() >> Self::ERRNADDRHI_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnADDRhi` field.
    pub const fn set_errnaddrhi(&mut self, value: u32) {
        let offset = Self::ERRNADDRHI_SHIFT;
        assert!(value & (Self::ERRNADDRHI_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNADDRHI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXCTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxctlr: u32 {
    }
}

impl Erxctlr {
    /// Offset of the `ERRnCTLRlo` field.
    pub const ERRNCTLRLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnCTLRlo` field.
    pub const ERRNCTLRLO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnCTLRlo` field.
    pub const fn errnctlrlo(self) -> u32 {
        ((self.bits() >> Self::ERRNCTLRLO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnCTLRlo` field.
    pub const fn set_errnctlrlo(&mut self, value: u32) {
        let offset = Self::ERRNCTLRLO_SHIFT;
        assert!(value & (Self::ERRNCTLRLO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNCTLRLO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXCTLR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxctlr2: u32 {
    }
}

impl Erxctlr2 {
    /// Offset of the `ERRnCTLRhi` field.
    pub const ERRNCTLRHI_SHIFT: u32 = 0;
    /// Mask for the `ERRnCTLRhi` field.
    pub const ERRNCTLRHI_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnCTLRhi` field.
    pub const fn errnctlrhi(self) -> u32 {
        ((self.bits() >> Self::ERRNCTLRHI_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnCTLRhi` field.
    pub const fn set_errnctlrhi(&mut self, value: u32) {
        let offset = Self::ERRNCTLRHI_SHIFT;
        assert!(value & (Self::ERRNCTLRHI_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNCTLRHI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXFR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxfr: u32 {
    }
}

impl Erxfr {
    /// Offset of the `ERRnFRlo` field.
    pub const ERRNFRLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnFRlo` field.
    pub const ERRNFRLO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnFRlo` field.
    pub const fn errnfrlo(self) -> u32 {
        ((self.bits() >> Self::ERRNFRLO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnFRlo` field.
    pub const fn set_errnfrlo(&mut self, value: u32) {
        let offset = Self::ERRNFRLO_SHIFT;
        assert!(value & (Self::ERRNFRLO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNFRLO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXFR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxfr2: u32 {
    }
}

impl Erxfr2 {
    /// Offset of the `ERRnFRhi` field.
    pub const ERRNFRHI_SHIFT: u32 = 0;
    /// Mask for the `ERRnFRhi` field.
    pub const ERRNFRHI_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnFRhi` field.
    pub const fn errnfrhi(self) -> u32 {
        ((self.bits() >> Self::ERRNFRHI_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnFRhi` field.
    pub const fn set_errnfrhi(&mut self, value: u32) {
        let offset = Self::ERRNFRHI_SHIFT;
        assert!(value & (Self::ERRNFRHI_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNFRHI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc0: u32 {
    }
}

impl Erxmisc0 {
    /// Offset of the `ERRnMISC0lo` field.
    pub const ERRNMISC0LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC0lo` field.
    pub const ERRNMISC0LO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC0lo` field.
    pub const fn errnmisc0lo(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC0LO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC0lo` field.
    pub const fn set_errnmisc0lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC0LO_SHIFT;
        assert!(value & (Self::ERRNMISC0LO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC0LO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc1: u32 {
    }
}

impl Erxmisc1 {
    /// Offset of the `ERRnMISC0hi` field.
    pub const ERRNMISC0HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC0hi` field.
    pub const ERRNMISC0HI_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC0hi` field.
    pub const fn errnmisc0hi(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC0HI_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC0hi` field.
    pub const fn set_errnmisc0hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC0HI_SHIFT;
        assert!(value & (Self::ERRNMISC0HI_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC0HI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc2: u32 {
    }
}

impl Erxmisc2 {
    /// Offset of the `ERRnMISC1lo` field.
    pub const ERRNMISC1LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC1lo` field.
    pub const ERRNMISC1LO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC1lo` field.
    pub const fn errnmisc1lo(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC1LO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC1lo` field.
    pub const fn set_errnmisc1lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC1LO_SHIFT;
        assert!(value & (Self::ERRNMISC1LO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC1LO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc3: u32 {
    }
}

impl Erxmisc3 {
    /// Offset of the `ERRnMISC1hi` field.
    pub const ERRNMISC1HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC1hi` field.
    pub const ERRNMISC1HI_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC1hi` field.
    pub const fn errnmisc1hi(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC1HI_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC1hi` field.
    pub const fn set_errnmisc1hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC1HI_SHIFT;
        assert!(value & (Self::ERRNMISC1HI_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC1HI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC4` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc4: u32 {
    }
}

impl Erxmisc4 {
    /// Offset of the `ERRnMISC2lo` field.
    pub const ERRNMISC2LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC2lo` field.
    pub const ERRNMISC2LO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC2lo` field.
    pub const fn errnmisc2lo(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC2LO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC2lo` field.
    pub const fn set_errnmisc2lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC2LO_SHIFT;
        assert!(value & (Self::ERRNMISC2LO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC2LO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC5` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc5: u32 {
    }
}

impl Erxmisc5 {
    /// Offset of the `ERRnMISC2hi` field.
    pub const ERRNMISC2HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC2hi` field.
    pub const ERRNMISC2HI_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC2hi` field.
    pub const fn errnmisc2hi(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC2HI_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC2hi` field.
    pub const fn set_errnmisc2hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC2HI_SHIFT;
        assert!(value & (Self::ERRNMISC2HI_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC2HI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC6` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc6: u32 {
    }
}

impl Erxmisc6 {
    /// Offset of the `ERRnMISC3lo` field.
    pub const ERRNMISC3LO_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC3lo` field.
    pub const ERRNMISC3LO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC3lo` field.
    pub const fn errnmisc3lo(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC3LO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC3lo` field.
    pub const fn set_errnmisc3lo(&mut self, value: u32) {
        let offset = Self::ERRNMISC3LO_SHIFT;
        assert!(value & (Self::ERRNMISC3LO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC3LO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXMISC7` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc7: u32 {
    }
}

impl Erxmisc7 {
    /// Offset of the `ERRnMISC3hi` field.
    pub const ERRNMISC3HI_SHIFT: u32 = 0;
    /// Mask for the `ERRnMISC3hi` field.
    pub const ERRNMISC3HI_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnMISC3hi` field.
    pub const fn errnmisc3hi(self) -> u32 {
        ((self.bits() >> Self::ERRNMISC3HI_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnMISC3hi` field.
    pub const fn set_errnmisc3hi(&mut self, value: u32) {
        let offset = Self::ERRNMISC3HI_SHIFT;
        assert!(value & (Self::ERRNMISC3HI_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNMISC3HI_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ERXSTATUS` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxstatus: u32 {
    }
}

impl Erxstatus {
    /// Offset of the `ERRnSTATUSlo` field.
    pub const ERRNSTATUSLO_SHIFT: u32 = 0;
    /// Mask for the `ERRnSTATUSlo` field.
    pub const ERRNSTATUSLO_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ERRnSTATUSlo` field.
    pub const fn errnstatuslo(self) -> u32 {
        ((self.bits() >> Self::ERRNSTATUSLO_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ERRnSTATUSlo` field.
    pub const fn set_errnstatuslo(&mut self, value: u32) {
        let offset = Self::ERRNSTATUSLO_SHIFT;
        assert!(value & (Self::ERRNSTATUSLO_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ERRNSTATUSLO_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ESR_EL1` system register value.
    #[derive(Clone, Copy, Eq, Default, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl1: u64 {
        /// `IL` bit.
        const IL = 1 << 25;
    }
}

#[cfg(feature = "el1")]
impl EsrEl1 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the `EC` field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the `EC` field.
    pub const EC_MASK: u64 = 0b111111;
    /// Offset of the `ISS2` field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the `ISS2` field.
    pub const ISS2_MASK: u64 = 0b111111111111111111111111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & 0b1111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> Self::EC_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `EC` field.
    pub const fn set_ec(&mut self, value: u8) {
        let offset = Self::EC_SHIFT;
        assert!(value & (Self::EC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> Self::ISS2_SHIFT) & 0b111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS2` field.
    pub const fn set_iss2(&mut self, value: u32) {
        let offset = Self::ISS2_SHIFT;
        assert!(value & (Self::ISS2_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS2_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ESR_EL2` system register value.
    #[derive(Clone, Copy, Eq, Default, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl2: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

#[cfg(feature = "el2")]
impl EsrEl2 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the `EC` field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the `EC` field.
    pub const EC_MASK: u64 = 0b111111;
    /// Offset of the `ISS2` field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the `ISS2` field.
    pub const ISS2_MASK: u64 = 0b111111111111111111111111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & 0b1111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> Self::EC_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `EC` field.
    pub const fn set_ec(&mut self, value: u8) {
        let offset = Self::EC_SHIFT;
        assert!(value & (Self::EC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> Self::ISS2_SHIFT) & 0b111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS2` field.
    pub const fn set_iss2(&mut self, value: u32) {
        let offset = Self::ISS2_SHIFT;
        assert!(value & (Self::ISS2_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS2_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ESR_EL3` system register value.
    #[derive(Clone, Copy, Eq, Default, PartialEq)]
    #[repr(transparent)]
    pub struct EsrEl3: u64 {
        /// 32-bit instruction length.
        const IL = 1 << 25;
    }
}

#[cfg(feature = "el3")]
impl EsrEl3 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the `EC` field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the `EC` field.
    pub const EC_MASK: u64 = 0b111111;
    /// Offset of the `ISS2` field.
    pub const ISS2_SHIFT: u32 = 32;
    /// Mask for the `ISS2` field.
    pub const ISS2_MASK: u64 = 0b111111111111111111111111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & 0b1111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> Self::EC_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `EC` field.
    pub const fn set_ec(&mut self, value: u8) {
        let offset = Self::EC_SHIFT;
        assert!(value & (Self::EC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        ((self.bits() >> Self::ISS2_SHIFT) & 0b111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS2` field.
    pub const fn set_iss2(&mut self, value: u32) {
        let offset = Self::ISS2_SHIFT;
        assert!(value & (Self::ISS2_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS2_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `FAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl FarEl1 {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        ((self.bits() >> Self::VA_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u64) {
        let offset = Self::VA_SHIFT;
        assert!(value & (Self::VA_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VA_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `FAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl FarEl2 {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        ((self.bits() >> Self::VA_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u64) {
        let offset = Self::VA_SHIFT;
        assert!(value & (Self::VA_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VA_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `GCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcrEl1: u64 {
        /// `RRND` bit.
        const RRND = 1 << 16;
    }
}

#[cfg(feature = "el1")]
impl GcrEl1 {
    /// Offset of the `Exclude` field.
    pub const EXCLUDE_SHIFT: u32 = 0;
    /// Mask for the `Exclude` field.
    pub const EXCLUDE_MASK: u64 = 0b1111111111111111;
    /// Offset of the `RRND` field.
    pub const RRND_SHIFT: u32 = 16;

    /// Returns the value of the `Exclude` field.
    pub const fn exclude(self) -> u16 {
        ((self.bits() >> Self::EXCLUDE_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `Exclude` field.
    pub const fn set_exclude(&mut self, value: u16) {
        let offset = Self::EXCLUDE_SHIFT;
        assert!(value & (Self::EXCLUDE_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXCLUDE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `GCSCR_EL1` system register value.
    ///
    /// Guarded Control Stack Control register.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
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
        ((self.bits() >> Self::PPS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `PPS` field.
    pub const fn set_pps(&mut self, value: u8) {
        let offset = Self::PPS_SHIFT;
        assert!(value & (Self::PPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IRGN` field.
    pub fn irgn(self) -> crate::manual::Cacheability {
        crate::manual::Cacheability::try_from(((self.bits() >> Self::IRGN_SHIFT) & 0b11) as u8)
            .unwrap()
    }

    /// Sets the value of the `IRGN` field.
    pub fn set_irgn(&mut self, value: crate::manual::Cacheability) {
        let offset = Self::IRGN_SHIFT;
        let value: u8 = value.into();
        assert!(value & (Self::IRGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ORGN` field.
    pub fn orgn(self) -> crate::manual::Cacheability {
        crate::manual::Cacheability::try_from(((self.bits() >> Self::ORGN_SHIFT) & 0b11) as u8)
            .unwrap()
    }

    /// Sets the value of the `ORGN` field.
    pub fn set_orgn(&mut self, value: crate::manual::Cacheability) {
        let offset = Self::ORGN_SHIFT;
        let value: u8 = value.into();
        assert!(value & (Self::ORGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH` field.
    pub fn sh(self) -> crate::manual::Shareability {
        crate::manual::Shareability::try_from(((self.bits() >> Self::SH_SHIFT) & 0b11) as u8)
            .unwrap()
    }

    /// Sets the value of the `SH` field.
    pub fn set_sh(&mut self, value: crate::manual::Shareability) {
        let offset = Self::SH_SHIFT;
        let value: u8 = value.into();
        assert!(value & (Self::SH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PGS` field.
    pub const fn pgs(self) -> u8 {
        ((self.bits() >> Self::PGS_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `PGS` field.
    pub const fn set_pgs(&mut self, value: u8) {
        let offset = Self::PGS_SHIFT;
        assert!(value & (Self::PGS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PGS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `L0GPTSZ` field.
    pub const fn l0gptsz(self) -> u8 {
        ((self.bits() >> Self::L0GPTSZ_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L0GPTSZ` field.
    pub const fn set_l0gptsz(&mut self, value: u8) {
        let offset = Self::L0GPTSZ_SHIFT;
        assert!(value & (Self::L0GPTSZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L0GPTSZ_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `GPTBR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GptbrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl GptbrEl3 {
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 0;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b1111111111111111111111111111111111111111;
    /// Offset of the `BADDR[43:40]` field.
    pub const BADDR_43_40_SHIFT: u32 = 40;
    /// Mask for the `BADDR[43:40]` field.
    pub const BADDR_43_40_MASK: u64 = 0b1111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        ((self.bits() >> Self::BADDR_SHIFT) & 0b1111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & (Self::BADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BADDR[43:40]` field.
    pub const fn baddr_43_40(self) -> u8 {
        ((self.bits() >> Self::BADDR_43_40_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BADDR[43:40]` field.
    pub const fn set_baddr_43_40(&mut self, value: u8) {
        let offset = Self::BADDR_43_40_SHIFT;
        assert!(value & (Self::BADDR_43_40_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_43_40_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl HafgrtrEl2 {
    /// Offset of the `AMEVCNTR0<x>_EL0` field.
    pub const AMEVCNTR0_EL0_SHIFT: u32 = 1;
}

bitflags! {
    /// `HCPTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hcptr: u32 {
        /// RES1 bits in the `HCPTR` register.
        const RES1 = 0b11001111111111;
        /// `TCP10` bit.
        const TCP10 = 1 << 10;
        /// `TCP11` bit.
        const TCP11 = 1 << 11;
        /// `TASE` bit.
        const TASE = 1 << 15;
        /// `TTA` bit.
        const TTA = 1 << 20;
        /// `TAM` bit.
        const TAM = 1 << 30;
        /// `TCPAC` bit.
        const TCPAC = 1 << 31;
    }
}

impl Hcptr {
    /// Offset of the `TCP10` field.
    pub const TCP10_SHIFT: u32 = 10;
    /// Offset of the `TCP11` field.
    pub const TCP11_SHIFT: u32 = 11;
    /// Offset of the `TASE` field.
    pub const TASE_SHIFT: u32 = 15;
    /// Offset of the `TTA` field.
    pub const TTA_SHIFT: u32 = 20;
    /// Offset of the `TAM` field.
    pub const TAM_SHIFT: u32 = 30;
    /// Offset of the `TCPAC` field.
    pub const TCPAC_SHIFT: u32 = 31;
}

bitflags! {
    /// `HCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hcr: u32 {
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
        /// `VA` bit.
        const VA = 1 << 8;
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
        /// `TAC` bit.
        const TAC = 1 << 21;
        /// `TSW` bit.
        const TSW = 1 << 22;
        /// `TPC` bit.
        const TPC = 1 << 23;
        /// `TPU` bit.
        const TPU = 1 << 24;
        /// `TTLB` bit.
        const TTLB = 1 << 25;
        /// `TVM` bit.
        const TVM = 1 << 26;
        /// `TGE` bit.
        const TGE = 1 << 27;
        /// `HCD` bit.
        const HCD = 1 << 29;
        /// `TRVM` bit.
        const TRVM = 1 << 30;
    }
}

impl Hcr {
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
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 8;
    /// Offset of the `FB` field.
    pub const FB_SHIFT: u32 = 9;
    /// Offset of the `BSU` field.
    pub const BSU_SHIFT: u32 = 10;
    /// Mask for the `BSU` field.
    pub const BSU_MASK: u32 = 0b11;
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
    /// Offset of the `TAC` field.
    pub const TAC_SHIFT: u32 = 21;
    /// Offset of the `TSW` field.
    pub const TSW_SHIFT: u32 = 22;
    /// Offset of the `TPC` field.
    pub const TPC_SHIFT: u32 = 23;
    /// Offset of the `TPU` field.
    pub const TPU_SHIFT: u32 = 24;
    /// Offset of the `TTLB` field.
    pub const TTLB_SHIFT: u32 = 25;
    /// Offset of the `TVM` field.
    pub const TVM_SHIFT: u32 = 26;
    /// Offset of the `TGE` field.
    pub const TGE_SHIFT: u32 = 27;
    /// Offset of the `HCD` field.
    pub const HCD_SHIFT: u32 = 29;
    /// Offset of the `TRVM` field.
    pub const TRVM_SHIFT: u32 = 30;

    /// Returns the value of the `BSU` field.
    pub const fn bsu(self) -> u8 {
        ((self.bits() >> Self::BSU_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `BSU` field.
    pub const fn set_bsu(&mut self, value: u8) {
        let offset = Self::BSU_SHIFT;
        assert!(value & (Self::BSU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BSU_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HCR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hcr2: u32 {
        /// `CD` bit.
        const CD = 1 << 0;
        /// `ID` bit.
        const ID = 1 << 1;
        /// `TERR` bit.
        const TERR = 1 << 4;
        /// `TEA` bit.
        const TEA = 1 << 5;
        /// `TID4` bit.
        const TID4 = 1 << 17;
        /// `TICAB` bit.
        const TICAB = 1 << 18;
        /// `TOCU` bit.
        const TOCU = 1 << 20;
        /// `TTLBIS` bit.
        const TTLBIS = 1 << 22;
    }
}

impl Hcr2 {
    /// Offset of the `CD` field.
    pub const CD_SHIFT: u32 = 0;
    /// Offset of the `ID` field.
    pub const ID_SHIFT: u32 = 1;
    /// Offset of the `TERR` field.
    pub const TERR_SHIFT: u32 = 4;
    /// Offset of the `TEA` field.
    pub const TEA_SHIFT: u32 = 5;
    /// Offset of the `TID4` field.
    pub const TID4_SHIFT: u32 = 17;
    /// Offset of the `TICAB` field.
    pub const TICAB_SHIFT: u32 = 18;
    /// Offset of the `TOCU` field.
    pub const TOCU_SHIFT: u32 = 20;
    /// Offset of the `TTLBIS` field.
    pub const TTLBIS_SHIFT: u32 = 22;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
        ((self.bits() >> Self::BSU_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `BSU` field.
    pub const fn set_bsu(&mut self, value: u8) {
        let offset = Self::BSU_SHIFT;
        assert!(value & (Self::BSU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BSU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> Self::TWEDEL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TWEDEL` field.
    pub const fn set_twedel(&mut self, value: u8) {
        let offset = Self::TWEDEL_SHIFT;
        assert!(value & (Self::TWEDEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWEDEL_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `HDCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdcr: u32 {
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
    }
}

impl Hdcr {
    /// Offset of the `HPMN` field.
    pub const HPMN_SHIFT: u32 = 0;
    /// Mask for the `HPMN` field.
    pub const HPMN_MASK: u32 = 0b11111;
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
    /// Offset of the `HPMD` field.
    pub const HPMD_SHIFT: u32 = 17;
    /// Offset of the `TTRF` field.
    pub const TTRF_SHIFT: u32 = 19;
    /// Offset of the `HCCD` field.
    pub const HCCD_SHIFT: u32 = 23;
    /// Offset of the `HLP` field.
    pub const HLP_SHIFT: u32 = 26;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the `MTPME` field.
    pub const MTPME_SHIFT: u32 = 28;
    /// Offset of the `HPMFZO` field.
    pub const HPMFZO_SHIFT: u32 = 29;

    /// Returns the value of the `HPMN` field.
    pub const fn hpmn(self) -> u8 {
        ((self.bits() >> Self::HPMN_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `HPMN` field.
    pub const fn set_hpmn(&mut self, value: u8) {
        let offset = Self::HPMN_SHIFT;
        assert!(value & (Self::HPMN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPMN_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HDFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdfar: u32 {
    }
}

impl Hdfar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        ((self.bits() >> Self::VA_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & (Self::VA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HDFGRTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `nPMECR_EL1` field.
    pub const NPMECR_EL1_SHIFT: u32 = 0;
    /// Offset of the `nPMIAR_EL1` field.
    pub const NPMIAR_EL1_SHIFT: u32 = 1;
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `HDFGWTR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `nPMECR_EL1` field.
    pub const NPMECR_EL1_SHIFT: u32 = 0;
    /// Offset of the `nPMIAR_EL1` field.
    pub const NPMIAR_EL1_SHIFT: u32 = 1;
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
    }
}

#[cfg(feature = "el2")]
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

    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NFGDTN_EL1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `nFGDTn_EL1` field.
    pub const fn set_nfgdtn_el1(&mut self, value: u8) {
        let offset = Self::NFGDTN_EL1_SHIFT;
        assert!(value & (Self::NFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NAFGDTN_EL1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `nAFGDTn_EL1` field.
    pub const fn set_nafgdtn_el1(&mut self, value: u8) {
        let offset = Self::NAFGDTN_EL1_SHIFT;
        assert!(value & (Self::NAFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NAFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
    }
}

#[cfg(feature = "el2")]
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

    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NFGDTN_EL1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `nFGDTn_EL1` field.
    pub const fn set_nfgdtn_el1(&mut self, value: u8) {
        let offset = Self::NFGDTN_EL1_SHIFT;
        assert!(value & (Self::NFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        ((self.bits() >> Self::NAFGDTN_EL1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `nAFGDTn_EL1` field.
    pub const fn set_nafgdtn_el1(&mut self, value: u8) {
        let offset = Self::NAFGDTN_EL1_SHIFT;
        assert!(value & (Self::NAFGDTN_EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NAFGDTN_EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
    /// `HIFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hifar: u32 {
    }
}

impl Hifar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        ((self.bits() >> Self::VA_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & (Self::VA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HMAIR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hmair0: u32 {
    }
}

impl Hmair0 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 4);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) & 0b11111111) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 4);
        let offset = Self::ATTR_SHIFT + (n - 0) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HMAIR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hmair1: u32 {
    }
}

impl Hmair1 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n >= 4 && n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 4) * 8)) & 0b11111111) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n >= 4 && n < 8);
        let offset = Self::ATTR_SHIFT + (n - 4) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HPFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hpfar: u32 {
    }
}

impl Hpfar {
    /// Offset of the `FIPA[39:12]` field.
    pub const FIPA_39_12_SHIFT: u32 = 4;
    /// Mask for the `FIPA[39:12]` field.
    pub const FIPA_39_12_MASK: u32 = 0b1111111111111111111111111111;

    /// Returns the value of the `FIPA[39:12]` field.
    pub const fn fipa_39_12(self) -> u32 {
        ((self.bits() >> Self::FIPA_39_12_SHIFT) & 0b1111111111111111111111111111) as u32
    }

    /// Sets the value of the `FIPA[39:12]` field.
    pub const fn set_fipa_39_12(&mut self, value: u32) {
        let offset = Self::FIPA_39_12_SHIFT;
        assert!(value & (Self::FIPA_39_12_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FIPA_39_12_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HPFAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HpfarEl2: u64 {
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

#[cfg(feature = "el2")]
impl HpfarEl2 {
    /// Offset of the `FIPA` field.
    pub const FIPA_SHIFT: u32 = 4;
    /// Mask for the `FIPA` field.
    pub const FIPA_MASK: u64 = 0b11111111111111111111111111111111111111111111;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 63;

    /// Returns the value of the `FIPA` field.
    pub const fn fipa(self) -> u64 {
        ((self.bits() >> Self::FIPA_SHIFT) & 0b11111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `FIPA` field.
    pub const fn set_fipa(&mut self, value: u64) {
        let offset = Self::FIPA_SHIFT;
        assert!(value & (Self::FIPA_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FIPA_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `HRMR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hrmr: u32 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

impl Hrmr {
    /// Offset of the `AA64` field.
    pub const AA64_SHIFT: u32 = 0;
    /// Offset of the `RR` field.
    pub const RR_SHIFT: u32 = 1;
}

bitflags! {
    /// `HSCTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hsctlr: u32 {
        /// RES1 bits in the `HSCTLR` register.
        const RES1 = 0b110000110001010000100000000000;
        /// `M` bit.
        const M = 1 << 0;
        /// `A` bit.
        const A = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 3;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 4;
        /// `CP15BEN` bit.
        const CP15BEN = 1 << 5;
        /// `ITD` bit.
        const ITD = 1 << 7;
        /// `SED` bit.
        const SED = 1 << 8;
        /// `I` bit.
        const I = 1 << 12;
        /// `WXN` bit.
        const WXN = 1 << 19;
        /// `TE` bit.
        const TE = 1 << 30;
        /// `DSSBS` bit.
        const DSSBS = 1 << 31;
    }
}

impl Hsctlr {
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `nTLSMD` field.
    pub const NTLSMD_SHIFT: u32 = 3;
    /// Offset of the `LSMAOE` field.
    pub const LSMAOE_SHIFT: u32 = 4;
    /// Offset of the `CP15BEN` field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the `ITD` field.
    pub const ITD_SHIFT: u32 = 7;
    /// Offset of the `SED` field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the `WXN` field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the `TE` field.
    pub const TE_SHIFT: u32 = 30;
    /// Offset of the `DSSBS` field.
    pub const DSSBS_SHIFT: u32 = 31;
}

bitflags! {
    /// `HSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hsr: u32 {
        /// `IL` bit.
        const IL = 1 << 25;
    }
}

impl Hsr {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u32 = 0b1111111111111111111111111;
    /// Offset of the `IL` field.
    pub const IL_SHIFT: u32 = 25;
    /// Offset of the `EC` field.
    pub const EC_SHIFT: u32 = 26;
    /// Mask for the `EC` field.
    pub const EC_MASK: u32 = 0b111111;

    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        ((self.bits() >> Self::ISS_SHIFT) & 0b1111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        ((self.bits() >> Self::EC_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `EC` field.
    pub const fn set_ec(&mut self, value: u8) {
        let offset = Self::EC_SHIFT;
        assert!(value & (Self::EC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EC_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HTCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Htcr: u32 {
        /// RES1 bits in the `HTCR` register.
        const RES1 = 0b10000000100000000000000000000000;
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
    }
}

impl Htcr {
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u32 = 0b111;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u32 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u32 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u32 = 0b11;
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

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HTPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Htpidr: u32 {
    }
}

impl Htpidr {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        ((self.bits() >> Self::TID_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & (Self::TID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TID_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HTRFCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Htrfcr: u32 {
        /// `E0HTRE` bit.
        const E0HTRE = 1 << 0;
        /// `E2TRE` bit.
        const E2TRE = 1 << 1;
        /// `CX` bit.
        const CX = 1 << 3;
    }
}

impl Htrfcr {
    /// Offset of the `E0HTRE` field.
    pub const E0HTRE_SHIFT: u32 = 0;
    /// Offset of the `E2TRE` field.
    pub const E2TRE_SHIFT: u32 = 1;
    /// Offset of the `CX` field.
    pub const CX_SHIFT: u32 = 3;
    /// Offset of the `TS` field.
    pub const TS_SHIFT: u32 = 5;
    /// Mask for the `TS` field.
    pub const TS_MASK: u32 = 0b11;

    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        ((self.bits() >> Self::TS_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TS` field.
    pub const fn set_ts(&mut self, value: u8) {
        let offset = Self::TS_SHIFT;
        assert!(value & (Self::TS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `HTTBR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Httbr: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Httbr {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b11111111111111111111111111111111111111111111111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        ((self.bits() >> Self::BADDR_SHIFT) & 0b11111111111111111111111111111111111111111111111)
            as u64
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & (Self::BADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `HVBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hvbar: u32 {
    }
}

impl Hvbar {
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 5;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u32 = 0b111111111111111111111111111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        ((self.bits() >> Self::VBA_SHIFT) & 0b111111111111111111111111111) as u32
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u32) {
        let offset = Self::VBA_SHIFT;
        assert!(value & (Self::VBA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_SRE_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `SRE` field.
    pub const SRE_SHIFT: u32 = 0;
    /// Offset of the `DFB` field.
    pub const DFB_SHIFT: u32 = 1;
    /// Offset of the `DIB` field.
    pub const DIB_SHIFT: u32 = 2;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
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

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
    pub const EOICOUNT_MASK: u64 = 0b11111;

    /// Returns the value of the `EOIcount` field.
    pub const fn eoicount(self) -> u8 {
        ((self.bits() >> Self::EOICOUNT_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `EOIcount` field.
    pub const fn set_eoicount(&mut self, value: u8) {
        let offset = Self::EOICOUNT_SHIFT;
        assert!(value & (Self::EOICOUNT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EOICOUNT_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
        ((self.bits() >> Self::VBPR1_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `VBPR1` field.
    pub const fn set_vbpr1(&mut self, value: u8) {
        let offset = Self::VBPR1_SHIFT;
        assert!(value & (Self::VBPR1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBPR1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VBPR0` field.
    pub const fn vbpr0(self) -> u8 {
        ((self.bits() >> Self::VBPR0_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `VBPR0` field.
    pub const fn set_vbpr0(&mut self, value: u8) {
        let offset = Self::VBPR0_SHIFT;
        assert!(value & (Self::VBPR0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBPR0_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64DFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64dfr0El1 {
    /// Offset of the `DebugVer` field.
    pub const DEBUGVER_SHIFT: u32 = 0;
    /// Mask for the `DebugVer` field.
    pub const DEBUGVER_MASK: u64 = 0b1111;
    /// Offset of the `TraceVer` field.
    pub const TRACEVER_SHIFT: u32 = 4;
    /// Mask for the `TraceVer` field.
    pub const TRACEVER_MASK: u64 = 0b1111;
    /// Offset of the `PMUVer` field.
    pub const PMUVER_SHIFT: u32 = 8;
    /// Mask for the `PMUVer` field.
    pub const PMUVER_MASK: u64 = 0b1111;
    /// Offset of the `BRPs` field.
    pub const BRPS_SHIFT: u32 = 12;
    /// Mask for the `BRPs` field.
    pub const BRPS_MASK: u64 = 0b1111;
    /// Offset of the `PMSS` field.
    pub const PMSS_SHIFT: u32 = 16;
    /// Mask for the `PMSS` field.
    pub const PMSS_MASK: u64 = 0b1111;
    /// Offset of the `WRPs` field.
    pub const WRPS_SHIFT: u32 = 20;
    /// Mask for the `WRPs` field.
    pub const WRPS_MASK: u64 = 0b1111;
    /// Offset of the `SEBEP` field.
    pub const SEBEP_SHIFT: u32 = 24;
    /// Mask for the `SEBEP` field.
    pub const SEBEP_MASK: u64 = 0b1111;
    /// Offset of the `CTX_CMPs` field.
    pub const CTX_CMPS_SHIFT: u32 = 28;
    /// Mask for the `CTX_CMPs` field.
    pub const CTX_CMPS_MASK: u64 = 0b1111;
    /// Offset of the `PMSVer` field.
    pub const PMSVER_SHIFT: u32 = 32;
    /// Mask for the `PMSVer` field.
    pub const PMSVER_MASK: u64 = 0b1111;
    /// Offset of the `DoubleLock` field.
    pub const DOUBLELOCK_SHIFT: u32 = 36;
    /// Mask for the `DoubleLock` field.
    pub const DOUBLELOCK_MASK: u64 = 0b1111;
    /// Offset of the `TraceFilt` field.
    pub const TRACEFILT_SHIFT: u32 = 40;
    /// Mask for the `TraceFilt` field.
    pub const TRACEFILT_MASK: u64 = 0b1111;
    /// Offset of the `TraceBuffer` field.
    pub const TRACEBUFFER_SHIFT: u32 = 44;
    /// Mask for the `TraceBuffer` field.
    pub const TRACEBUFFER_MASK: u64 = 0b1111;
    /// Offset of the `MTPMU` field.
    pub const MTPMU_SHIFT: u32 = 48;
    /// Mask for the `MTPMU` field.
    pub const MTPMU_MASK: u64 = 0b1111;
    /// Offset of the `BRBE` field.
    pub const BRBE_SHIFT: u32 = 52;
    /// Mask for the `BRBE` field.
    pub const BRBE_MASK: u64 = 0b1111;
    /// Offset of the `ExtTrcBuff` field.
    pub const EXTTRCBUFF_SHIFT: u32 = 56;
    /// Mask for the `ExtTrcBuff` field.
    pub const EXTTRCBUFF_MASK: u64 = 0b1111;
    /// Offset of the `HPMN0` field.
    pub const HPMN0_SHIFT: u32 = 60;
    /// Mask for the `HPMN0` field.
    pub const HPMN0_MASK: u64 = 0b1111;

    /// Returns the value of the `DebugVer` field.
    pub const fn debugver(self) -> u8 {
        ((self.bits() >> Self::DEBUGVER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DebugVer` field.
    pub const fn set_debugver(&mut self, value: u8) {
        let offset = Self::DEBUGVER_SHIFT;
        assert!(value & (Self::DEBUGVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DEBUGVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TraceVer` field.
    pub const fn tracever(self) -> u8 {
        ((self.bits() >> Self::TRACEVER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TraceVer` field.
    pub const fn set_tracever(&mut self, value: u8) {
        let offset = Self::TRACEVER_SHIFT;
        assert!(value & (Self::TRACEVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRACEVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMUVer` field.
    pub const fn pmuver(self) -> u8 {
        ((self.bits() >> Self::PMUVER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PMUVer` field.
    pub const fn set_pmuver(&mut self, value: u8) {
        let offset = Self::PMUVER_SHIFT;
        assert!(value & (Self::PMUVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMUVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        ((self.bits() >> Self::BRPS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BRPs` field.
    pub const fn set_brps(&mut self, value: u8) {
        let offset = Self::BRPS_SHIFT;
        assert!(value & (Self::BRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMSS` field.
    pub const fn pmss(self) -> u8 {
        ((self.bits() >> Self::PMSS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PMSS` field.
    pub const fn set_pmss(&mut self, value: u8) {
        let offset = Self::PMSS_SHIFT;
        assert!(value & (Self::PMSS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        ((self.bits() >> Self::WRPS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `WRPs` field.
    pub const fn set_wrps(&mut self, value: u8) {
        let offset = Self::WRPS_SHIFT;
        assert!(value & (Self::WRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SEBEP` field.
    pub const fn sebep(self) -> u8 {
        ((self.bits() >> Self::SEBEP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SEBEP` field.
    pub const fn set_sebep(&mut self, value: u8) {
        let offset = Self::SEBEP_SHIFT;
        assert!(value & (Self::SEBEP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEBEP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        ((self.bits() >> Self::CTX_CMPS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CTX_CMPs` field.
    pub const fn set_ctx_cmps(&mut self, value: u8) {
        let offset = Self::CTX_CMPS_SHIFT;
        assert!(value & (Self::CTX_CMPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTX_CMPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMSVer` field.
    pub const fn pmsver(self) -> u8 {
        ((self.bits() >> Self::PMSVER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PMSVer` field.
    pub const fn set_pmsver(&mut self, value: u8) {
        let offset = Self::PMSVER_SHIFT;
        assert!(value & (Self::PMSVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSVER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `DoubleLock` field.
    pub const fn doublelock(self) -> u8 {
        ((self.bits() >> Self::DOUBLELOCK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DoubleLock` field.
    pub const fn set_doublelock(&mut self, value: u8) {
        let offset = Self::DOUBLELOCK_SHIFT;
        assert!(value & (Self::DOUBLELOCK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DOUBLELOCK_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        ((self.bits() >> Self::TRACEFILT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TraceFilt` field.
    pub const fn set_tracefilt(&mut self, value: u8) {
        let offset = Self::TRACEFILT_SHIFT;
        assert!(value & (Self::TRACEFILT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRACEFILT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TraceBuffer` field.
    pub const fn tracebuffer(self) -> u8 {
        ((self.bits() >> Self::TRACEBUFFER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TraceBuffer` field.
    pub const fn set_tracebuffer(&mut self, value: u8) {
        let offset = Self::TRACEBUFFER_SHIFT;
        assert!(value & (Self::TRACEBUFFER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRACEBUFFER_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        ((self.bits() >> Self::MTPMU_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MTPMU` field.
    pub const fn set_mtpmu(&mut self, value: u8) {
        let offset = Self::MTPMU_SHIFT;
        assert!(value & (Self::MTPMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTPMU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BRBE` field.
    pub const fn brbe(self) -> u8 {
        ((self.bits() >> Self::BRBE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BRBE` field.
    pub const fn set_brbe(&mut self, value: u8) {
        let offset = Self::BRBE_SHIFT;
        assert!(value & (Self::BRBE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BRBE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ExtTrcBuff` field.
    pub const fn exttrcbuff(self) -> u8 {
        ((self.bits() >> Self::EXTTRCBUFF_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ExtTrcBuff` field.
    pub const fn set_exttrcbuff(&mut self, value: u8) {
        let offset = Self::EXTTRCBUFF_SHIFT;
        assert!(value & (Self::EXTTRCBUFF_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXTTRCBUFF_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        ((self.bits() >> Self::HPMN0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HPMN0` field.
    pub const fn set_hpmn0(&mut self, value: u8) {
        let offset = Self::HPMN0_SHIFT;
        assert!(value & (Self::HPMN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPMN0_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64DFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64dfr1El1 {
    /// Offset of the `SYSPMUID` field.
    pub const SYSPMUID_SHIFT: u32 = 0;
    /// Mask for the `SYSPMUID` field.
    pub const SYSPMUID_MASK: u64 = 0b11111111;
    /// Offset of the `BRPs` field.
    pub const BRPS_SHIFT: u32 = 8;
    /// Mask for the `BRPs` field.
    pub const BRPS_MASK: u64 = 0b11111111;
    /// Offset of the `WRPs` field.
    pub const WRPS_SHIFT: u32 = 16;
    /// Mask for the `WRPs` field.
    pub const WRPS_MASK: u64 = 0b11111111;
    /// Offset of the `CTX_CMPs` field.
    pub const CTX_CMPS_SHIFT: u32 = 24;
    /// Mask for the `CTX_CMPs` field.
    pub const CTX_CMPS_MASK: u64 = 0b11111111;
    /// Offset of the `SPMU` field.
    pub const SPMU_SHIFT: u32 = 32;
    /// Mask for the `SPMU` field.
    pub const SPMU_MASK: u64 = 0b1111;
    /// Offset of the `PMICNTR` field.
    pub const PMICNTR_SHIFT: u32 = 36;
    /// Mask for the `PMICNTR` field.
    pub const PMICNTR_MASK: u64 = 0b1111;
    /// Offset of the `ABLE` field.
    pub const ABLE_SHIFT: u32 = 40;
    /// Mask for the `ABLE` field.
    pub const ABLE_MASK: u64 = 0b1111;
    /// Offset of the `ITE` field.
    pub const ITE_SHIFT: u32 = 44;
    /// Mask for the `ITE` field.
    pub const ITE_MASK: u64 = 0b1111;
    /// Offset of the `EBEP` field.
    pub const EBEP_SHIFT: u32 = 48;
    /// Mask for the `EBEP` field.
    pub const EBEP_MASK: u64 = 0b1111;
    /// Offset of the `DPFZS` field.
    pub const DPFZS_SHIFT: u32 = 52;
    /// Mask for the `DPFZS` field.
    pub const DPFZS_MASK: u64 = 0b1111;
    /// Offset of the `ABL_CMPs` field.
    pub const ABL_CMPS_SHIFT: u32 = 56;
    /// Mask for the `ABL_CMPs` field.
    pub const ABL_CMPS_MASK: u64 = 0b11111111;

    /// Returns the value of the `SYSPMUID` field.
    pub const fn syspmuid(self) -> u8 {
        ((self.bits() >> Self::SYSPMUID_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `SYSPMUID` field.
    pub const fn set_syspmuid(&mut self, value: u8) {
        let offset = Self::SYSPMUID_SHIFT;
        assert!(value & (Self::SYSPMUID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYSPMUID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        ((self.bits() >> Self::BRPS_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `BRPs` field.
    pub const fn set_brps(&mut self, value: u8) {
        let offset = Self::BRPS_SHIFT;
        assert!(value & (Self::BRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        ((self.bits() >> Self::WRPS_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `WRPs` field.
    pub const fn set_wrps(&mut self, value: u8) {
        let offset = Self::WRPS_SHIFT;
        assert!(value & (Self::WRPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WRPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        ((self.bits() >> Self::CTX_CMPS_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `CTX_CMPs` field.
    pub const fn set_ctx_cmps(&mut self, value: u8) {
        let offset = Self::CTX_CMPS_SHIFT;
        assert!(value & (Self::CTX_CMPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CTX_CMPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SPMU` field.
    pub const fn spmu(self) -> u8 {
        ((self.bits() >> Self::SPMU_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SPMU` field.
    pub const fn set_spmu(&mut self, value: u8) {
        let offset = Self::SPMU_SHIFT;
        assert!(value & (Self::SPMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPMU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMICNTR` field.
    pub const fn pmicntr(self) -> u8 {
        ((self.bits() >> Self::PMICNTR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PMICNTR` field.
    pub const fn set_pmicntr(&mut self, value: u8) {
        let offset = Self::PMICNTR_SHIFT;
        assert!(value & (Self::PMICNTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMICNTR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ABLE` field.
    pub const fn able(self) -> u8 {
        ((self.bits() >> Self::ABLE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ABLE` field.
    pub const fn set_able(&mut self, value: u8) {
        let offset = Self::ABLE_SHIFT;
        assert!(value & (Self::ABLE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ABLE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ITE` field.
    pub const fn ite(self) -> u8 {
        ((self.bits() >> Self::ITE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ITE` field.
    pub const fn set_ite(&mut self, value: u8) {
        let offset = Self::ITE_SHIFT;
        assert!(value & (Self::ITE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ITE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EBEP` field.
    pub const fn ebep(self) -> u8 {
        ((self.bits() >> Self::EBEP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EBEP` field.
    pub const fn set_ebep(&mut self, value: u8) {
        let offset = Self::EBEP_SHIFT;
        assert!(value & (Self::EBEP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EBEP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `DPFZS` field.
    pub const fn dpfzs(self) -> u8 {
        ((self.bits() >> Self::DPFZS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DPFZS` field.
    pub const fn set_dpfzs(&mut self, value: u8) {
        let offset = Self::DPFZS_SHIFT;
        assert!(value & (Self::DPFZS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DPFZS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ABL_CMPs` field.
    pub const fn abl_cmps(self) -> u8 {
        ((self.bits() >> Self::ABL_CMPS_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `ABL_CMPs` field.
    pub const fn set_abl_cmps(&mut self, value: u8) {
        let offset = Self::ABL_CMPS_SHIFT;
        assert!(value & (Self::ABL_CMPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ABL_CMPS_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64ISAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64isar1El1 {
    /// Offset of the `DPB` field.
    pub const DPB_SHIFT: u32 = 0;
    /// Mask for the `DPB` field.
    pub const DPB_MASK: u64 = 0b1111;
    /// Offset of the `APA` field.
    pub const APA_SHIFT: u32 = 4;
    /// Mask for the `APA` field.
    pub const APA_MASK: u64 = 0b1111;
    /// Offset of the `API` field.
    pub const API_SHIFT: u32 = 8;
    /// Mask for the `API` field.
    pub const API_MASK: u64 = 0b1111;
    /// Offset of the `JSCVT` field.
    pub const JSCVT_SHIFT: u32 = 12;
    /// Mask for the `JSCVT` field.
    pub const JSCVT_MASK: u64 = 0b1111;
    /// Offset of the `FCMA` field.
    pub const FCMA_SHIFT: u32 = 16;
    /// Mask for the `FCMA` field.
    pub const FCMA_MASK: u64 = 0b1111;
    /// Offset of the `LRCPC` field.
    pub const LRCPC_SHIFT: u32 = 20;
    /// Mask for the `LRCPC` field.
    pub const LRCPC_MASK: u64 = 0b1111;
    /// Offset of the `GPA` field.
    pub const GPA_SHIFT: u32 = 24;
    /// Mask for the `GPA` field.
    pub const GPA_MASK: u64 = 0b1111;
    /// Offset of the `GPI` field.
    pub const GPI_SHIFT: u32 = 28;
    /// Mask for the `GPI` field.
    pub const GPI_MASK: u64 = 0b1111;
    /// Offset of the `FRINTTS` field.
    pub const FRINTTS_SHIFT: u32 = 32;
    /// Mask for the `FRINTTS` field.
    pub const FRINTTS_MASK: u64 = 0b1111;
    /// Offset of the `SB` field.
    pub const SB_SHIFT: u32 = 36;
    /// Mask for the `SB` field.
    pub const SB_MASK: u64 = 0b1111;
    /// Offset of the `SPECRES` field.
    pub const SPECRES_SHIFT: u32 = 40;
    /// Mask for the `SPECRES` field.
    pub const SPECRES_MASK: u64 = 0b1111;
    /// Offset of the `BF16` field.
    pub const BF16_SHIFT: u32 = 44;
    /// Mask for the `BF16` field.
    pub const BF16_MASK: u64 = 0b1111;
    /// Offset of the `DGH` field.
    pub const DGH_SHIFT: u32 = 48;
    /// Mask for the `DGH` field.
    pub const DGH_MASK: u64 = 0b1111;
    /// Offset of the `I8MM` field.
    pub const I8MM_SHIFT: u32 = 52;
    /// Mask for the `I8MM` field.
    pub const I8MM_MASK: u64 = 0b1111;
    /// Offset of the `XS` field.
    pub const XS_SHIFT: u32 = 56;
    /// Mask for the `XS` field.
    pub const XS_MASK: u64 = 0b1111;
    /// Offset of the `LS64` field.
    pub const LS64_SHIFT: u32 = 60;
    /// Mask for the `LS64` field.
    pub const LS64_MASK: u64 = 0b1111;

    /// Returns the value of the `DPB` field.
    pub const fn dpb(self) -> u8 {
        ((self.bits() >> Self::DPB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DPB` field.
    pub const fn set_dpb(&mut self, value: u8) {
        let offset = Self::DPB_SHIFT;
        assert!(value & (Self::DPB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DPB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `APA` field.
    pub const fn apa(self) -> u8 {
        ((self.bits() >> Self::APA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `APA` field.
    pub const fn set_apa(&mut self, value: u8) {
        let offset = Self::APA_SHIFT;
        assert!(value & (Self::APA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `API` field.
    pub const fn api(self) -> u8 {
        ((self.bits() >> Self::API_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `API` field.
    pub const fn set_api(&mut self, value: u8) {
        let offset = Self::API_SHIFT;
        assert!(value & (Self::API_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::API_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `JSCVT` field.
    pub const fn jscvt(self) -> u8 {
        ((self.bits() >> Self::JSCVT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `JSCVT` field.
    pub const fn set_jscvt(&mut self, value: u8) {
        let offset = Self::JSCVT_SHIFT;
        assert!(value & (Self::JSCVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::JSCVT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FCMA` field.
    pub const fn fcma(self) -> u8 {
        ((self.bits() >> Self::FCMA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `FCMA` field.
    pub const fn set_fcma(&mut self, value: u8) {
        let offset = Self::FCMA_SHIFT;
        assert!(value & (Self::FCMA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FCMA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LRCPC` field.
    pub const fn lrcpc(self) -> u8 {
        ((self.bits() >> Self::LRCPC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LRCPC` field.
    pub const fn set_lrcpc(&mut self, value: u8) {
        let offset = Self::LRCPC_SHIFT;
        assert!(value & (Self::LRCPC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LRCPC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GPA` field.
    pub const fn gpa(self) -> u8 {
        ((self.bits() >> Self::GPA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GPA` field.
    pub const fn set_gpa(&mut self, value: u8) {
        let offset = Self::GPA_SHIFT;
        assert!(value & (Self::GPA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GPA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GPI` field.
    pub const fn gpi(self) -> u8 {
        ((self.bits() >> Self::GPI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GPI` field.
    pub const fn set_gpi(&mut self, value: u8) {
        let offset = Self::GPI_SHIFT;
        assert!(value & (Self::GPI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GPI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FRINTTS` field.
    pub const fn frintts(self) -> u8 {
        ((self.bits() >> Self::FRINTTS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `FRINTTS` field.
    pub const fn set_frintts(&mut self, value: u8) {
        let offset = Self::FRINTTS_SHIFT;
        assert!(value & (Self::FRINTTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FRINTTS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SB` field.
    pub const fn sb(self) -> u8 {
        ((self.bits() >> Self::SB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SB` field.
    pub const fn set_sb(&mut self, value: u8) {
        let offset = Self::SB_SHIFT;
        assert!(value & (Self::SB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SPECRES` field.
    pub const fn specres(self) -> u8 {
        ((self.bits() >> Self::SPECRES_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SPECRES` field.
    pub const fn set_specres(&mut self, value: u8) {
        let offset = Self::SPECRES_SHIFT;
        assert!(value & (Self::SPECRES_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPECRES_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BF16` field.
    pub const fn bf16(self) -> u8 {
        ((self.bits() >> Self::BF16_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BF16` field.
    pub const fn set_bf16(&mut self, value: u8) {
        let offset = Self::BF16_SHIFT;
        assert!(value & (Self::BF16_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BF16_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `DGH` field.
    pub const fn dgh(self) -> u8 {
        ((self.bits() >> Self::DGH_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DGH` field.
    pub const fn set_dgh(&mut self, value: u8) {
        let offset = Self::DGH_SHIFT;
        assert!(value & (Self::DGH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DGH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `I8MM` field.
    pub const fn i8mm(self) -> u8 {
        ((self.bits() >> Self::I8MM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `I8MM` field.
    pub const fn set_i8mm(&mut self, value: u8) {
        let offset = Self::I8MM_SHIFT;
        assert!(value & (Self::I8MM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I8MM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `XS` field.
    pub const fn xs(self) -> u8 {
        ((self.bits() >> Self::XS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `XS` field.
    pub const fn set_xs(&mut self, value: u8) {
        let offset = Self::XS_SHIFT;
        assert!(value & (Self::XS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::XS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LS64` field.
    pub const fn ls64(self) -> u8 {
        ((self.bits() >> Self::LS64_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LS64` field.
    pub const fn set_ls64(&mut self, value: u8) {
        let offset = Self::LS64_SHIFT;
        assert!(value & (Self::LS64_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LS64_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64ISAR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64isar2El1 {
    /// Offset of the `WFxT` field.
    pub const WFXT_SHIFT: u32 = 0;
    /// Mask for the `WFxT` field.
    pub const WFXT_MASK: u64 = 0b1111;
    /// Offset of the `RPRES` field.
    pub const RPRES_SHIFT: u32 = 4;
    /// Mask for the `RPRES` field.
    pub const RPRES_MASK: u64 = 0b1111;
    /// Offset of the `GPA3` field.
    pub const GPA3_SHIFT: u32 = 8;
    /// Mask for the `GPA3` field.
    pub const GPA3_MASK: u64 = 0b1111;
    /// Offset of the `APA3` field.
    pub const APA3_SHIFT: u32 = 12;
    /// Mask for the `APA3` field.
    pub const APA3_MASK: u64 = 0b1111;
    /// Offset of the `MOPS` field.
    pub const MOPS_SHIFT: u32 = 16;
    /// Mask for the `MOPS` field.
    pub const MOPS_MASK: u64 = 0b1111;
    /// Offset of the `BC` field.
    pub const BC_SHIFT: u32 = 20;
    /// Mask for the `BC` field.
    pub const BC_MASK: u64 = 0b1111;
    /// Offset of the `PAC_frac` field.
    pub const PAC_FRAC_SHIFT: u32 = 24;
    /// Mask for the `PAC_frac` field.
    pub const PAC_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `CLRBHB` field.
    pub const CLRBHB_SHIFT: u32 = 28;
    /// Mask for the `CLRBHB` field.
    pub const CLRBHB_MASK: u64 = 0b1111;
    /// Offset of the `SYSREG_128` field.
    pub const SYSREG_128_SHIFT: u32 = 32;
    /// Mask for the `SYSREG_128` field.
    pub const SYSREG_128_MASK: u64 = 0b1111;
    /// Offset of the `SYSINSTR_128` field.
    pub const SYSINSTR_128_SHIFT: u32 = 36;
    /// Mask for the `SYSINSTR_128` field.
    pub const SYSINSTR_128_MASK: u64 = 0b1111;
    /// Offset of the `PRFMSLC` field.
    pub const PRFMSLC_SHIFT: u32 = 40;
    /// Mask for the `PRFMSLC` field.
    pub const PRFMSLC_MASK: u64 = 0b1111;
    /// Offset of the `PCDPHINT` field.
    pub const PCDPHINT_SHIFT: u32 = 44;
    /// Mask for the `PCDPHINT` field.
    pub const PCDPHINT_MASK: u64 = 0b1111;
    /// Offset of the `RPRFM` field.
    pub const RPRFM_SHIFT: u32 = 48;
    /// Mask for the `RPRFM` field.
    pub const RPRFM_MASK: u64 = 0b1111;
    /// Offset of the `CSSC` field.
    pub const CSSC_SHIFT: u32 = 52;
    /// Mask for the `CSSC` field.
    pub const CSSC_MASK: u64 = 0b1111;
    /// Offset of the `LUT` field.
    pub const LUT_SHIFT: u32 = 56;
    /// Mask for the `LUT` field.
    pub const LUT_MASK: u64 = 0b1111;
    /// Offset of the `ATS1A` field.
    pub const ATS1A_SHIFT: u32 = 60;
    /// Mask for the `ATS1A` field.
    pub const ATS1A_MASK: u64 = 0b1111;

    /// Returns the value of the `WFxT` field.
    pub const fn wfxt(self) -> u8 {
        ((self.bits() >> Self::WFXT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `WFxT` field.
    pub const fn set_wfxt(&mut self, value: u8) {
        let offset = Self::WFXT_SHIFT;
        assert!(value & (Self::WFXT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WFXT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RPRES` field.
    pub const fn rpres(self) -> u8 {
        ((self.bits() >> Self::RPRES_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RPRES` field.
    pub const fn set_rpres(&mut self, value: u8) {
        let offset = Self::RPRES_SHIFT;
        assert!(value & (Self::RPRES_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RPRES_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GPA3` field.
    pub const fn gpa3(self) -> u8 {
        ((self.bits() >> Self::GPA3_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GPA3` field.
    pub const fn set_gpa3(&mut self, value: u8) {
        let offset = Self::GPA3_SHIFT;
        assert!(value & (Self::GPA3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GPA3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `APA3` field.
    pub const fn apa3(self) -> u8 {
        ((self.bits() >> Self::APA3_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `APA3` field.
    pub const fn set_apa3(&mut self, value: u8) {
        let offset = Self::APA3_SHIFT;
        assert!(value & (Self::APA3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::APA3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MOPS` field.
    pub const fn mops(self) -> u8 {
        ((self.bits() >> Self::MOPS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MOPS` field.
    pub const fn set_mops(&mut self, value: u8) {
        let offset = Self::MOPS_SHIFT;
        assert!(value & (Self::MOPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MOPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BC` field.
    pub const fn bc(self) -> u8 {
        ((self.bits() >> Self::BC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BC` field.
    pub const fn set_bc(&mut self, value: u8) {
        let offset = Self::BC_SHIFT;
        assert!(value & (Self::BC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PAC_frac` field.
    pub const fn pac_frac(self) -> u8 {
        ((self.bits() >> Self::PAC_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PAC_frac` field.
    pub const fn set_pac_frac(&mut self, value: u8) {
        let offset = Self::PAC_FRAC_SHIFT;
        assert!(value & (Self::PAC_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PAC_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CLRBHB` field.
    pub const fn clrbhb(self) -> u8 {
        ((self.bits() >> Self::CLRBHB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CLRBHB` field.
    pub const fn set_clrbhb(&mut self, value: u8) {
        let offset = Self::CLRBHB_SHIFT;
        assert!(value & (Self::CLRBHB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CLRBHB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SYSREG_128` field.
    pub const fn sysreg_128(self) -> u8 {
        ((self.bits() >> Self::SYSREG_128_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SYSREG_128` field.
    pub const fn set_sysreg_128(&mut self, value: u8) {
        let offset = Self::SYSREG_128_SHIFT;
        assert!(value & (Self::SYSREG_128_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYSREG_128_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SYSINSTR_128` field.
    pub const fn sysinstr_128(self) -> u8 {
        ((self.bits() >> Self::SYSINSTR_128_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SYSINSTR_128` field.
    pub const fn set_sysinstr_128(&mut self, value: u8) {
        let offset = Self::SYSINSTR_128_SHIFT;
        assert!(value & (Self::SYSINSTR_128_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYSINSTR_128_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PRFMSLC` field.
    pub const fn prfmslc(self) -> u8 {
        ((self.bits() >> Self::PRFMSLC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PRFMSLC` field.
    pub const fn set_prfmslc(&mut self, value: u8) {
        let offset = Self::PRFMSLC_SHIFT;
        assert!(value & (Self::PRFMSLC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PRFMSLC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PCDPHINT` field.
    pub const fn pcdphint(self) -> u8 {
        ((self.bits() >> Self::PCDPHINT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PCDPHINT` field.
    pub const fn set_pcdphint(&mut self, value: u8) {
        let offset = Self::PCDPHINT_SHIFT;
        assert!(value & (Self::PCDPHINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PCDPHINT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RPRFM` field.
    pub const fn rprfm(self) -> u8 {
        ((self.bits() >> Self::RPRFM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RPRFM` field.
    pub const fn set_rprfm(&mut self, value: u8) {
        let offset = Self::RPRFM_SHIFT;
        assert!(value & (Self::RPRFM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RPRFM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CSSC` field.
    pub const fn cssc(self) -> u8 {
        ((self.bits() >> Self::CSSC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CSSC` field.
    pub const fn set_cssc(&mut self, value: u8) {
        let offset = Self::CSSC_SHIFT;
        assert!(value & (Self::CSSC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSSC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LUT` field.
    pub const fn lut(self) -> u8 {
        ((self.bits() >> Self::LUT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LUT` field.
    pub const fn set_lut(&mut self, value: u8) {
        let offset = Self::LUT_SHIFT;
        assert!(value & (Self::LUT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LUT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ATS1A` field.
    pub const fn ats1a(self) -> u8 {
        ((self.bits() >> Self::ATS1A_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ATS1A` field.
    pub const fn set_ats1a(&mut self, value: u8) {
        let offset = Self::ATS1A_SHIFT;
        assert!(value & (Self::ATS1A_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATS1A_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr0El1 {
    /// Offset of the `PARange` field.
    pub const PARANGE_SHIFT: u32 = 0;
    /// Mask for the `PARange` field.
    pub const PARANGE_MASK: u64 = 0b1111;
    /// Offset of the `ASIDBits` field.
    pub const ASIDBITS_SHIFT: u32 = 4;
    /// Mask for the `ASIDBits` field.
    pub const ASIDBITS_MASK: u64 = 0b1111;
    /// Offset of the `BigEnd` field.
    pub const BIGEND_SHIFT: u32 = 8;
    /// Mask for the `BigEnd` field.
    pub const BIGEND_MASK: u64 = 0b1111;
    /// Offset of the `SNSMem` field.
    pub const SNSMEM_SHIFT: u32 = 12;
    /// Mask for the `SNSMem` field.
    pub const SNSMEM_MASK: u64 = 0b1111;
    /// Offset of the `BigEndEL0` field.
    pub const BIGENDEL0_SHIFT: u32 = 16;
    /// Mask for the `BigEndEL0` field.
    pub const BIGENDEL0_MASK: u64 = 0b1111;
    /// Offset of the `TGran16` field.
    pub const TGRAN16_SHIFT: u32 = 20;
    /// Mask for the `TGran16` field.
    pub const TGRAN16_MASK: u64 = 0b1111;
    /// Offset of the `TGran64` field.
    pub const TGRAN64_SHIFT: u32 = 24;
    /// Mask for the `TGran64` field.
    pub const TGRAN64_MASK: u64 = 0b1111;
    /// Offset of the `TGran4` field.
    pub const TGRAN4_SHIFT: u32 = 28;
    /// Mask for the `TGran4` field.
    pub const TGRAN4_MASK: u64 = 0b1111;
    /// Offset of the `TGran16_2` field.
    pub const TGRAN16_2_SHIFT: u32 = 32;
    /// Mask for the `TGran16_2` field.
    pub const TGRAN16_2_MASK: u64 = 0b1111;
    /// Offset of the `TGran64_2` field.
    pub const TGRAN64_2_SHIFT: u32 = 36;
    /// Mask for the `TGran64_2` field.
    pub const TGRAN64_2_MASK: u64 = 0b1111;
    /// Offset of the `TGran4_2` field.
    pub const TGRAN4_2_SHIFT: u32 = 40;
    /// Mask for the `TGran4_2` field.
    pub const TGRAN4_2_MASK: u64 = 0b1111;
    /// Offset of the `ExS` field.
    pub const EXS_SHIFT: u32 = 44;
    /// Mask for the `ExS` field.
    pub const EXS_MASK: u64 = 0b1111;
    /// Offset of the `FGT` field.
    pub const FGT_SHIFT: u32 = 56;
    /// Mask for the `FGT` field.
    pub const FGT_MASK: u64 = 0b1111;
    /// Offset of the `ECV` field.
    pub const ECV_SHIFT: u32 = 60;
    /// Mask for the `ECV` field.
    pub const ECV_MASK: u64 = 0b1111;

    /// Returns the value of the `PARange` field.
    pub const fn parange(self) -> u8 {
        ((self.bits() >> Self::PARANGE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PARange` field.
    pub const fn set_parange(&mut self, value: u8) {
        let offset = Self::PARANGE_SHIFT;
        assert!(value & (Self::PARANGE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARANGE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ASIDBits` field.
    pub const fn asidbits(self) -> u8 {
        ((self.bits() >> Self::ASIDBITS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ASIDBits` field.
    pub const fn set_asidbits(&mut self, value: u8) {
        let offset = Self::ASIDBITS_SHIFT;
        assert!(value & (Self::ASIDBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASIDBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BigEnd` field.
    pub const fn bigend(self) -> u8 {
        ((self.bits() >> Self::BIGEND_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BigEnd` field.
    pub const fn set_bigend(&mut self, value: u8) {
        let offset = Self::BIGEND_SHIFT;
        assert!(value & (Self::BIGEND_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BIGEND_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SNSMem` field.
    pub const fn snsmem(self) -> u8 {
        ((self.bits() >> Self::SNSMEM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SNSMem` field.
    pub const fn set_snsmem(&mut self, value: u8) {
        let offset = Self::SNSMEM_SHIFT;
        assert!(value & (Self::SNSMEM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SNSMEM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BigEndEL0` field.
    pub const fn bigendel0(self) -> u8 {
        ((self.bits() >> Self::BIGENDEL0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BigEndEL0` field.
    pub const fn set_bigendel0(&mut self, value: u8) {
        let offset = Self::BIGENDEL0_SHIFT;
        assert!(value & (Self::BIGENDEL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BIGENDEL0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TGran16` field.
    pub const fn tgran16(self) -> u8 {
        ((self.bits() >> Self::TGRAN16_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TGran16` field.
    pub const fn set_tgran16(&mut self, value: u8) {
        let offset = Self::TGRAN16_SHIFT;
        assert!(value & (Self::TGRAN16_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN16_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TGran64` field.
    pub const fn tgran64(self) -> u8 {
        ((self.bits() >> Self::TGRAN64_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TGran64` field.
    pub const fn set_tgran64(&mut self, value: u8) {
        let offset = Self::TGRAN64_SHIFT;
        assert!(value & (Self::TGRAN64_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN64_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TGran4` field.
    pub const fn tgran4(self) -> u8 {
        ((self.bits() >> Self::TGRAN4_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TGran4` field.
    pub const fn set_tgran4(&mut self, value: u8) {
        let offset = Self::TGRAN4_SHIFT;
        assert!(value & (Self::TGRAN4_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN4_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TGran16_2` field.
    pub const fn tgran16_2(self) -> u8 {
        ((self.bits() >> Self::TGRAN16_2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TGran16_2` field.
    pub const fn set_tgran16_2(&mut self, value: u8) {
        let offset = Self::TGRAN16_2_SHIFT;
        assert!(value & (Self::TGRAN16_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN16_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TGran64_2` field.
    pub const fn tgran64_2(self) -> u8 {
        ((self.bits() >> Self::TGRAN64_2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TGran64_2` field.
    pub const fn set_tgran64_2(&mut self, value: u8) {
        let offset = Self::TGRAN64_2_SHIFT;
        assert!(value & (Self::TGRAN64_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN64_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TGran4_2` field.
    pub const fn tgran4_2(self) -> u8 {
        ((self.bits() >> Self::TGRAN4_2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TGran4_2` field.
    pub const fn set_tgran4_2(&mut self, value: u8) {
        let offset = Self::TGRAN4_2_SHIFT;
        assert!(value & (Self::TGRAN4_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TGRAN4_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ExS` field.
    pub const fn exs(self) -> u8 {
        ((self.bits() >> Self::EXS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ExS` field.
    pub const fn set_exs(&mut self, value: u8) {
        let offset = Self::EXS_SHIFT;
        assert!(value & (Self::EXS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FGT` field.
    pub const fn fgt(self) -> u8 {
        ((self.bits() >> Self::FGT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `FGT` field.
    pub const fn set_fgt(&mut self, value: u8) {
        let offset = Self::FGT_SHIFT;
        assert!(value & (Self::FGT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FGT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ECV` field.
    pub const fn ecv(self) -> u8 {
        ((self.bits() >> Self::ECV_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ECV` field.
    pub const fn set_ecv(&mut self, value: u8) {
        let offset = Self::ECV_SHIFT;
        assert!(value & (Self::ECV_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ECV_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr1El1 {
    /// Offset of the `HAFDBS` field.
    pub const HAFDBS_SHIFT: u32 = 0;
    /// Mask for the `HAFDBS` field.
    pub const HAFDBS_MASK: u64 = 0b1111;
    /// Offset of the `VMIDBits` field.
    pub const VMIDBITS_SHIFT: u32 = 4;
    /// Mask for the `VMIDBits` field.
    pub const VMIDBITS_MASK: u64 = 0b1111;
    /// Offset of the `VH` field.
    pub const VH_SHIFT: u32 = 8;
    /// Mask for the `VH` field.
    pub const VH_MASK: u64 = 0b1111;
    /// Offset of the `HPDS` field.
    pub const HPDS_SHIFT: u32 = 12;
    /// Mask for the `HPDS` field.
    pub const HPDS_MASK: u64 = 0b1111;
    /// Offset of the `LO` field.
    pub const LO_SHIFT: u32 = 16;
    /// Mask for the `LO` field.
    pub const LO_MASK: u64 = 0b1111;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 20;
    /// Mask for the `PAN` field.
    pub const PAN_MASK: u64 = 0b1111;
    /// Offset of the `SpecSEI` field.
    pub const SPECSEI_SHIFT: u32 = 24;
    /// Mask for the `SpecSEI` field.
    pub const SPECSEI_MASK: u64 = 0b1111;
    /// Offset of the `XNX` field.
    pub const XNX_SHIFT: u32 = 28;
    /// Mask for the `XNX` field.
    pub const XNX_MASK: u64 = 0b1111;
    /// Offset of the `TWED` field.
    pub const TWED_SHIFT: u32 = 32;
    /// Mask for the `TWED` field.
    pub const TWED_MASK: u64 = 0b1111;
    /// Offset of the `ETS` field.
    pub const ETS_SHIFT: u32 = 36;
    /// Mask for the `ETS` field.
    pub const ETS_MASK: u64 = 0b1111;
    /// Offset of the `HCX` field.
    pub const HCX_SHIFT: u32 = 40;
    /// Mask for the `HCX` field.
    pub const HCX_MASK: u64 = 0b1111;
    /// Offset of the `AFP` field.
    pub const AFP_SHIFT: u32 = 44;
    /// Mask for the `AFP` field.
    pub const AFP_MASK: u64 = 0b1111;
    /// Offset of the `nTLBPA` field.
    pub const NTLBPA_SHIFT: u32 = 48;
    /// Mask for the `nTLBPA` field.
    pub const NTLBPA_MASK: u64 = 0b1111;
    /// Offset of the `TIDCP1` field.
    pub const TIDCP1_SHIFT: u32 = 52;
    /// Mask for the `TIDCP1` field.
    pub const TIDCP1_MASK: u64 = 0b1111;
    /// Offset of the `CMOW` field.
    pub const CMOW_SHIFT: u32 = 56;
    /// Mask for the `CMOW` field.
    pub const CMOW_MASK: u64 = 0b1111;
    /// Offset of the `ECBHB` field.
    pub const ECBHB_SHIFT: u32 = 60;
    /// Mask for the `ECBHB` field.
    pub const ECBHB_MASK: u64 = 0b1111;

    /// Returns the value of the `HAFDBS` field.
    pub const fn hafdbs(self) -> u8 {
        ((self.bits() >> Self::HAFDBS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HAFDBS` field.
    pub const fn set_hafdbs(&mut self, value: u8) {
        let offset = Self::HAFDBS_SHIFT;
        assert!(value & (Self::HAFDBS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HAFDBS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VMIDBits` field.
    pub const fn vmidbits(self) -> u8 {
        ((self.bits() >> Self::VMIDBITS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `VMIDBits` field.
    pub const fn set_vmidbits(&mut self, value: u8) {
        let offset = Self::VMIDBITS_SHIFT;
        assert!(value & (Self::VMIDBITS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMIDBITS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VH` field.
    pub const fn vh(self) -> u8 {
        ((self.bits() >> Self::VH_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `VH` field.
    pub const fn set_vh(&mut self, value: u8) {
        let offset = Self::VH_SHIFT;
        assert!(value & (Self::VH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        ((self.bits() >> Self::HPDS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HPDS` field.
    pub const fn set_hpds(&mut self, value: u8) {
        let offset = Self::HPDS_SHIFT;
        assert!(value & (Self::HPDS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPDS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LO` field.
    pub const fn lo(self) -> u8 {
        ((self.bits() >> Self::LO_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LO` field.
    pub const fn set_lo(&mut self, value: u8) {
        let offset = Self::LO_SHIFT;
        assert!(value & (Self::LO_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LO_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        ((self.bits() >> Self::PAN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PAN` field.
    pub const fn set_pan(&mut self, value: u8) {
        let offset = Self::PAN_SHIFT;
        assert!(value & (Self::PAN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PAN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        ((self.bits() >> Self::SPECSEI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SpecSEI` field.
    pub const fn set_specsei(&mut self, value: u8) {
        let offset = Self::SPECSEI_SHIFT;
        assert!(value & (Self::SPECSEI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPECSEI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        ((self.bits() >> Self::XNX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `XNX` field.
    pub const fn set_xnx(&mut self, value: u8) {
        let offset = Self::XNX_SHIFT;
        assert!(value & (Self::XNX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::XNX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TWED` field.
    pub const fn twed(self) -> u8 {
        ((self.bits() >> Self::TWED_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TWED` field.
    pub const fn set_twed(&mut self, value: u8) {
        let offset = Self::TWED_SHIFT;
        assert!(value & (Self::TWED_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWED_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        ((self.bits() >> Self::ETS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ETS` field.
    pub const fn set_ets(&mut self, value: u8) {
        let offset = Self::ETS_SHIFT;
        assert!(value & (Self::ETS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ETS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `HCX` field.
    pub const fn hcx(self) -> u8 {
        ((self.bits() >> Self::HCX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HCX` field.
    pub const fn set_hcx(&mut self, value: u8) {
        let offset = Self::HCX_SHIFT;
        assert!(value & (Self::HCX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HCX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AFP` field.
    pub const fn afp(self) -> u8 {
        ((self.bits() >> Self::AFP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AFP` field.
    pub const fn set_afp(&mut self, value: u8) {
        let offset = Self::AFP_SHIFT;
        assert!(value & (Self::AFP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        ((self.bits() >> Self::NTLBPA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `nTLBPA` field.
    pub const fn set_ntlbpa(&mut self, value: u8) {
        let offset = Self::NTLBPA_SHIFT;
        assert!(value & (Self::NTLBPA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NTLBPA_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TIDCP1` field.
    pub const fn tidcp1(self) -> u8 {
        ((self.bits() >> Self::TIDCP1_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TIDCP1` field.
    pub const fn set_tidcp1(&mut self, value: u8) {
        let offset = Self::TIDCP1_SHIFT;
        assert!(value & (Self::TIDCP1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TIDCP1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CMOW` field.
    pub const fn cmow(self) -> u8 {
        ((self.bits() >> Self::CMOW_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CMOW` field.
    pub const fn set_cmow(&mut self, value: u8) {
        let offset = Self::CMOW_SHIFT;
        assert!(value & (Self::CMOW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMOW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ECBHB` field.
    pub const fn ecbhb(self) -> u8 {
        ((self.bits() >> Self::ECBHB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ECBHB` field.
    pub const fn set_ecbhb(&mut self, value: u8) {
        let offset = Self::ECBHB_SHIFT;
        assert!(value & (Self::ECBHB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ECBHB_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr2El1 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Mask for the `CnP` field.
    pub const CNP_MASK: u64 = 0b1111;
    /// Offset of the `UAO` field.
    pub const UAO_SHIFT: u32 = 4;
    /// Mask for the `UAO` field.
    pub const UAO_MASK: u64 = 0b1111;
    /// Offset of the `LSM` field.
    pub const LSM_SHIFT: u32 = 8;
    /// Mask for the `LSM` field.
    pub const LSM_MASK: u64 = 0b1111;
    /// Offset of the `IESB` field.
    pub const IESB_SHIFT: u32 = 12;
    /// Mask for the `IESB` field.
    pub const IESB_MASK: u64 = 0b1111;
    /// Offset of the `VARange` field.
    pub const VARANGE_SHIFT: u32 = 16;
    /// Mask for the `VARange` field.
    pub const VARANGE_MASK: u64 = 0b1111;
    /// Offset of the `CCIDX` field.
    pub const CCIDX_SHIFT: u32 = 20;
    /// Mask for the `CCIDX` field.
    pub const CCIDX_MASK: u64 = 0b1111;
    /// Offset of the `NV` field.
    pub const NV_SHIFT: u32 = 24;
    /// Mask for the `NV` field.
    pub const NV_MASK: u64 = 0b1111;
    /// Offset of the `ST` field.
    pub const ST_SHIFT: u32 = 28;
    /// Mask for the `ST` field.
    pub const ST_MASK: u64 = 0b1111;
    /// Offset of the `AT` field.
    pub const AT_SHIFT: u32 = 32;
    /// Mask for the `AT` field.
    pub const AT_MASK: u64 = 0b1111;
    /// Offset of the `IDS` field.
    pub const IDS_SHIFT: u32 = 36;
    /// Mask for the `IDS` field.
    pub const IDS_MASK: u64 = 0b1111;
    /// Offset of the `FWB` field.
    pub const FWB_SHIFT: u32 = 40;
    /// Mask for the `FWB` field.
    pub const FWB_MASK: u64 = 0b1111;
    /// Offset of the `TTL` field.
    pub const TTL_SHIFT: u32 = 48;
    /// Mask for the `TTL` field.
    pub const TTL_MASK: u64 = 0b1111;
    /// Offset of the `BBM` field.
    pub const BBM_SHIFT: u32 = 52;
    /// Mask for the `BBM` field.
    pub const BBM_MASK: u64 = 0b1111;
    /// Offset of the `EVT` field.
    pub const EVT_SHIFT: u32 = 56;
    /// Mask for the `EVT` field.
    pub const EVT_MASK: u64 = 0b1111;
    /// Offset of the `E0PD` field.
    pub const E0PD_SHIFT: u32 = 60;
    /// Mask for the `E0PD` field.
    pub const E0PD_MASK: u64 = 0b1111;

    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        ((self.bits() >> Self::CNP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CnP` field.
    pub const fn set_cnp(&mut self, value: u8) {
        let offset = Self::CNP_SHIFT;
        assert!(value & (Self::CNP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CNP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `UAO` field.
    pub const fn uao(self) -> u8 {
        ((self.bits() >> Self::UAO_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `UAO` field.
    pub const fn set_uao(&mut self, value: u8) {
        let offset = Self::UAO_SHIFT;
        assert!(value & (Self::UAO_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::UAO_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        ((self.bits() >> Self::LSM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LSM` field.
    pub const fn set_lsm(&mut self, value: u8) {
        let offset = Self::LSM_SHIFT;
        assert!(value & (Self::LSM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IESB` field.
    pub const fn iesb(self) -> u8 {
        ((self.bits() >> Self::IESB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `IESB` field.
    pub const fn set_iesb(&mut self, value: u8) {
        let offset = Self::IESB_SHIFT;
        assert!(value & (Self::IESB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IESB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VARange` field.
    pub const fn varange(self) -> u8 {
        ((self.bits() >> Self::VARANGE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `VARange` field.
    pub const fn set_varange(&mut self, value: u8) {
        let offset = Self::VARANGE_SHIFT;
        assert!(value & (Self::VARANGE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARANGE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        ((self.bits() >> Self::CCIDX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CCIDX` field.
    pub const fn set_ccidx(&mut self, value: u8) {
        let offset = Self::CCIDX_SHIFT;
        assert!(value & (Self::CCIDX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CCIDX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `NV` field.
    pub const fn nv(self) -> u8 {
        ((self.bits() >> Self::NV_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `NV` field.
    pub const fn set_nv(&mut self, value: u8) {
        let offset = Self::NV_SHIFT;
        assert!(value & (Self::NV_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NV_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ST` field.
    pub const fn st(self) -> u8 {
        ((self.bits() >> Self::ST_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ST` field.
    pub const fn set_st(&mut self, value: u8) {
        let offset = Self::ST_SHIFT;
        assert!(value & (Self::ST_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ST_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AT` field.
    pub const fn at(self) -> u8 {
        ((self.bits() >> Self::AT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AT` field.
    pub const fn set_at(&mut self, value: u8) {
        let offset = Self::AT_SHIFT;
        assert!(value & (Self::AT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IDS` field.
    pub const fn ids(self) -> u8 {
        ((self.bits() >> Self::IDS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `IDS` field.
    pub const fn set_ids(&mut self, value: u8) {
        let offset = Self::IDS_SHIFT;
        assert!(value & (Self::IDS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IDS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FWB` field.
    pub const fn fwb(self) -> u8 {
        ((self.bits() >> Self::FWB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `FWB` field.
    pub const fn set_fwb(&mut self, value: u8) {
        let offset = Self::FWB_SHIFT;
        assert!(value & (Self::FWB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FWB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TTL` field.
    pub const fn ttl(self) -> u8 {
        ((self.bits() >> Self::TTL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TTL` field.
    pub const fn set_ttl(&mut self, value: u8) {
        let offset = Self::TTL_SHIFT;
        assert!(value & (Self::TTL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BBM` field.
    pub const fn bbm(self) -> u8 {
        ((self.bits() >> Self::BBM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BBM` field.
    pub const fn set_bbm(&mut self, value: u8) {
        let offset = Self::BBM_SHIFT;
        assert!(value & (Self::BBM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BBM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        ((self.bits() >> Self::EVT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EVT` field.
    pub const fn set_evt(&mut self, value: u8) {
        let offset = Self::EVT_SHIFT;
        assert!(value & (Self::EVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `E0PD` field.
    pub const fn e0pd(self) -> u8 {
        ((self.bits() >> Self::E0PD_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `E0PD` field.
    pub const fn set_e0pd(&mut self, value: u8) {
        let offset = Self::E0PD_SHIFT;
        assert!(value & (Self::E0PD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E0PD_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr3El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr3El1 {
    /// Offset of the `TCRX` field.
    pub const TCRX_SHIFT: u32 = 0;
    /// Mask for the `TCRX` field.
    pub const TCRX_MASK: u64 = 0b1111;
    /// Offset of the `SCTLRX` field.
    pub const SCTLRX_SHIFT: u32 = 4;
    /// Mask for the `SCTLRX` field.
    pub const SCTLRX_MASK: u64 = 0b1111;
    /// Offset of the `S1PIE` field.
    pub const S1PIE_SHIFT: u32 = 8;
    /// Mask for the `S1PIE` field.
    pub const S1PIE_MASK: u64 = 0b1111;
    /// Offset of the `S2PIE` field.
    pub const S2PIE_SHIFT: u32 = 12;
    /// Mask for the `S2PIE` field.
    pub const S2PIE_MASK: u64 = 0b1111;
    /// Offset of the `S1POE` field.
    pub const S1POE_SHIFT: u32 = 16;
    /// Mask for the `S1POE` field.
    pub const S1POE_MASK: u64 = 0b1111;
    /// Offset of the `S2POE` field.
    pub const S2POE_SHIFT: u32 = 20;
    /// Mask for the `S2POE` field.
    pub const S2POE_MASK: u64 = 0b1111;
    /// Offset of the `AIE` field.
    pub const AIE_SHIFT: u32 = 24;
    /// Mask for the `AIE` field.
    pub const AIE_MASK: u64 = 0b1111;
    /// Offset of the `MEC` field.
    pub const MEC_SHIFT: u32 = 28;
    /// Mask for the `MEC` field.
    pub const MEC_MASK: u64 = 0b1111;
    /// Offset of the `D128` field.
    pub const D128_SHIFT: u32 = 32;
    /// Mask for the `D128` field.
    pub const D128_MASK: u64 = 0b1111;
    /// Offset of the `D128_2` field.
    pub const D128_2_SHIFT: u32 = 36;
    /// Mask for the `D128_2` field.
    pub const D128_2_MASK: u64 = 0b1111;
    /// Offset of the `SNERR` field.
    pub const SNERR_SHIFT: u32 = 40;
    /// Mask for the `SNERR` field.
    pub const SNERR_MASK: u64 = 0b1111;
    /// Offset of the `ANERR` field.
    pub const ANERR_SHIFT: u32 = 44;
    /// Mask for the `ANERR` field.
    pub const ANERR_MASK: u64 = 0b1111;
    /// Offset of the `SDERR` field.
    pub const SDERR_SHIFT: u32 = 52;
    /// Mask for the `SDERR` field.
    pub const SDERR_MASK: u64 = 0b1111;
    /// Offset of the `ADERR` field.
    pub const ADERR_SHIFT: u32 = 56;
    /// Mask for the `ADERR` field.
    pub const ADERR_MASK: u64 = 0b1111;
    /// Offset of the `Spec_FPACC` field.
    pub const SPEC_FPACC_SHIFT: u32 = 60;
    /// Mask for the `Spec_FPACC` field.
    pub const SPEC_FPACC_MASK: u64 = 0b1111;

    /// Returns the value of the `TCRX` field.
    pub const fn tcrx(self) -> u8 {
        ((self.bits() >> Self::TCRX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TCRX` field.
    pub const fn set_tcrx(&mut self, value: u8) {
        let offset = Self::TCRX_SHIFT;
        assert!(value & (Self::TCRX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCRX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SCTLRX` field.
    pub const fn sctlrx(self) -> u8 {
        ((self.bits() >> Self::SCTLRX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SCTLRX` field.
    pub const fn set_sctlrx(&mut self, value: u8) {
        let offset = Self::SCTLRX_SHIFT;
        assert!(value & (Self::SCTLRX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SCTLRX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `S1PIE` field.
    pub const fn s1pie(self) -> u8 {
        ((self.bits() >> Self::S1PIE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `S1PIE` field.
    pub const fn set_s1pie(&mut self, value: u8) {
        let offset = Self::S1PIE_SHIFT;
        assert!(value & (Self::S1PIE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S1PIE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `S2PIE` field.
    pub const fn s2pie(self) -> u8 {
        ((self.bits() >> Self::S2PIE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `S2PIE` field.
    pub const fn set_s2pie(&mut self, value: u8) {
        let offset = Self::S2PIE_SHIFT;
        assert!(value & (Self::S2PIE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S2PIE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `S1POE` field.
    pub const fn s1poe(self) -> u8 {
        ((self.bits() >> Self::S1POE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `S1POE` field.
    pub const fn set_s1poe(&mut self, value: u8) {
        let offset = Self::S1POE_SHIFT;
        assert!(value & (Self::S1POE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S1POE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `S2POE` field.
    pub const fn s2poe(self) -> u8 {
        ((self.bits() >> Self::S2POE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `S2POE` field.
    pub const fn set_s2poe(&mut self, value: u8) {
        let offset = Self::S2POE_SHIFT;
        assert!(value & (Self::S2POE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::S2POE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AIE` field.
    pub const fn aie(self) -> u8 {
        ((self.bits() >> Self::AIE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AIE` field.
    pub const fn set_aie(&mut self, value: u8) {
        let offset = Self::AIE_SHIFT;
        assert!(value & (Self::AIE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AIE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MEC` field.
    pub const fn mec(self) -> u8 {
        ((self.bits() >> Self::MEC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MEC` field.
    pub const fn set_mec(&mut self, value: u8) {
        let offset = Self::MEC_SHIFT;
        assert!(value & (Self::MEC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MEC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `D128` field.
    pub const fn d128(self) -> u8 {
        ((self.bits() >> Self::D128_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `D128` field.
    pub const fn set_d128(&mut self, value: u8) {
        let offset = Self::D128_SHIFT;
        assert!(value & (Self::D128_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::D128_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `D128_2` field.
    pub const fn d128_2(self) -> u8 {
        ((self.bits() >> Self::D128_2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `D128_2` field.
    pub const fn set_d128_2(&mut self, value: u8) {
        let offset = Self::D128_2_SHIFT;
        assert!(value & (Self::D128_2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::D128_2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SNERR` field.
    pub const fn snerr(self) -> u8 {
        ((self.bits() >> Self::SNERR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SNERR` field.
    pub const fn set_snerr(&mut self, value: u8) {
        let offset = Self::SNERR_SHIFT;
        assert!(value & (Self::SNERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SNERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ANERR` field.
    pub const fn anerr(self) -> u8 {
        ((self.bits() >> Self::ANERR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ANERR` field.
    pub const fn set_anerr(&mut self, value: u8) {
        let offset = Self::ANERR_SHIFT;
        assert!(value & (Self::ANERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ANERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SDERR` field.
    pub const fn sderr(self) -> u8 {
        ((self.bits() >> Self::SDERR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SDERR` field.
    pub const fn set_sderr(&mut self, value: u8) {
        let offset = Self::SDERR_SHIFT;
        assert!(value & (Self::SDERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SDERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ADERR` field.
    pub const fn aderr(self) -> u8 {
        ((self.bits() >> Self::ADERR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ADERR` field.
    pub const fn set_aderr(&mut self, value: u8) {
        let offset = Self::ADERR_SHIFT;
        assert!(value & (Self::ADERR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADERR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Spec_FPACC` field.
    pub const fn spec_fpacc(self) -> u8 {
        ((self.bits() >> Self::SPEC_FPACC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Spec_FPACC` field.
    pub const fn set_spec_fpacc(&mut self, value: u8) {
        let offset = Self::SPEC_FPACC_SHIFT;
        assert!(value & (Self::SPEC_FPACC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPEC_FPACC_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64PFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64pfr0El1 {
    /// Offset of the `EL0` field.
    pub const EL0_SHIFT: u32 = 0;
    /// Mask for the `EL0` field.
    pub const EL0_MASK: u64 = 0b1111;
    /// Offset of the `EL1` field.
    pub const EL1_SHIFT: u32 = 4;
    /// Mask for the `EL1` field.
    pub const EL1_MASK: u64 = 0b1111;
    /// Offset of the `EL2` field.
    pub const EL2_SHIFT: u32 = 8;
    /// Mask for the `EL2` field.
    pub const EL2_MASK: u64 = 0b1111;
    /// Offset of the `EL3` field.
    pub const EL3_SHIFT: u32 = 12;
    /// Mask for the `EL3` field.
    pub const EL3_MASK: u64 = 0b1111;
    /// Offset of the `FP` field.
    pub const FP_SHIFT: u32 = 16;
    /// Mask for the `FP` field.
    pub const FP_MASK: u64 = 0b1111;
    /// Offset of the `AdvSIMD` field.
    pub const ADVSIMD_SHIFT: u32 = 20;
    /// Mask for the `AdvSIMD` field.
    pub const ADVSIMD_MASK: u64 = 0b1111;
    /// Offset of the `GIC` field.
    pub const GIC_SHIFT: u32 = 24;
    /// Mask for the `GIC` field.
    pub const GIC_MASK: u64 = 0b1111;
    /// Offset of the `RAS` field.
    pub const RAS_SHIFT: u32 = 28;
    /// Mask for the `RAS` field.
    pub const RAS_MASK: u64 = 0b1111;
    /// Offset of the `SVE` field.
    pub const SVE_SHIFT: u32 = 32;
    /// Mask for the `SVE` field.
    pub const SVE_MASK: u64 = 0b1111;
    /// Offset of the `SEL2` field.
    pub const SEL2_SHIFT: u32 = 36;
    /// Mask for the `SEL2` field.
    pub const SEL2_MASK: u64 = 0b1111;
    /// Offset of the `MPAM` field.
    pub const MPAM_SHIFT: u32 = 40;
    /// Mask for the `MPAM` field.
    pub const MPAM_MASK: u64 = 0b1111;
    /// Offset of the `AMU` field.
    pub const AMU_SHIFT: u32 = 44;
    /// Mask for the `AMU` field.
    pub const AMU_MASK: u64 = 0b1111;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 48;
    /// Mask for the `DIT` field.
    pub const DIT_MASK: u64 = 0b1111;
    /// Offset of the `RME` field.
    pub const RME_SHIFT: u32 = 52;
    /// Mask for the `RME` field.
    pub const RME_MASK: u64 = 0b1111;
    /// Offset of the `CSV2` field.
    pub const CSV2_SHIFT: u32 = 56;
    /// Mask for the `CSV2` field.
    pub const CSV2_MASK: u64 = 0b1111;
    /// Offset of the `CSV3` field.
    pub const CSV3_SHIFT: u32 = 60;
    /// Mask for the `CSV3` field.
    pub const CSV3_MASK: u64 = 0b1111;

    /// Returns the value of the `EL0` field.
    pub const fn el0(self) -> u8 {
        ((self.bits() >> Self::EL0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EL0` field.
    pub const fn set_el0(&mut self, value: u8) {
        let offset = Self::EL0_SHIFT;
        assert!(value & (Self::EL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EL1` field.
    pub const fn el1(self) -> u8 {
        ((self.bits() >> Self::EL1_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EL1` field.
    pub const fn set_el1(&mut self, value: u8) {
        let offset = Self::EL1_SHIFT;
        assert!(value & (Self::EL1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EL2` field.
    pub const fn el2(self) -> u8 {
        ((self.bits() >> Self::EL2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EL2` field.
    pub const fn set_el2(&mut self, value: u8) {
        let offset = Self::EL2_SHIFT;
        assert!(value & (Self::EL2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EL3` field.
    pub const fn el3(self) -> u8 {
        ((self.bits() >> Self::EL3_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EL3` field.
    pub const fn set_el3(&mut self, value: u8) {
        let offset = Self::EL3_SHIFT;
        assert!(value & (Self::EL3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EL3_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FP` field.
    pub const fn fp(self) -> u8 {
        ((self.bits() >> Self::FP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `FP` field.
    pub const fn set_fp(&mut self, value: u8) {
        let offset = Self::FP_SHIFT;
        assert!(value & (Self::FP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AdvSIMD` field.
    pub const fn advsimd(self) -> u8 {
        ((self.bits() >> Self::ADVSIMD_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AdvSIMD` field.
    pub const fn set_advsimd(&mut self, value: u8) {
        let offset = Self::ADVSIMD_SHIFT;
        assert!(value & (Self::ADVSIMD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ADVSIMD_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        ((self.bits() >> Self::GIC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GIC` field.
    pub const fn set_gic(&mut self, value: u8) {
        let offset = Self::GIC_SHIFT;
        assert!(value & (Self::GIC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GIC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        ((self.bits() >> Self::RAS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RAS` field.
    pub const fn set_ras(&mut self, value: u8) {
        let offset = Self::RAS_SHIFT;
        assert!(value & (Self::RAS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RAS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SVE` field.
    pub const fn sve(self) -> u8 {
        ((self.bits() >> Self::SVE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SVE` field.
    pub const fn set_sve(&mut self, value: u8) {
        let offset = Self::SVE_SHIFT;
        assert!(value & (Self::SVE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SVE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SEL2` field.
    pub const fn sel2(self) -> u8 {
        ((self.bits() >> Self::SEL2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SEL2` field.
    pub const fn set_sel2(&mut self, value: u8) {
        let offset = Self::SEL2_SHIFT;
        assert!(value & (Self::SEL2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEL2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MPAM` field.
    pub const fn mpam(self) -> u8 {
        ((self.bits() >> Self::MPAM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MPAM` field.
    pub const fn set_mpam(&mut self, value: u8) {
        let offset = Self::MPAM_SHIFT;
        assert!(value & (Self::MPAM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPAM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        ((self.bits() >> Self::AMU_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AMU` field.
    pub const fn set_amu(&mut self, value: u8) {
        let offset = Self::AMU_SHIFT;
        assert!(value & (Self::AMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AMU_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        ((self.bits() >> Self::DIT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DIT` field.
    pub const fn set_dit(&mut self, value: u8) {
        let offset = Self::DIT_SHIFT;
        assert!(value & (Self::DIT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DIT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RME` field.
    pub const fn rme(self) -> u8 {
        ((self.bits() >> Self::RME_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RME` field.
    pub const fn set_rme(&mut self, value: u8) {
        let offset = Self::RME_SHIFT;
        assert!(value & (Self::RME_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RME_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        ((self.bits() >> Self::CSV2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CSV2` field.
    pub const fn set_csv2(&mut self, value: u8) {
        let offset = Self::CSV2_SHIFT;
        assert!(value & (Self::CSV2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        ((self.bits() >> Self::CSV3_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CSV3` field.
    pub const fn set_csv3(&mut self, value: u8) {
        let offset = Self::CSV3_SHIFT;
        assert!(value & (Self::CSV3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV3_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64PFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64pfr1El1 {
    /// Offset of the `BT` field.
    pub const BT_SHIFT: u32 = 0;
    /// Mask for the `BT` field.
    pub const BT_MASK: u64 = 0b1111;
    /// Offset of the `SSBS` field.
    pub const SSBS_SHIFT: u32 = 4;
    /// Mask for the `SSBS` field.
    pub const SSBS_MASK: u64 = 0b1111;
    /// Offset of the `MTE` field.
    pub const MTE_SHIFT: u32 = 8;
    /// Mask for the `MTE` field.
    pub const MTE_MASK: u64 = 0b1111;
    /// Offset of the `RAS_frac` field.
    pub const RAS_FRAC_SHIFT: u32 = 12;
    /// Mask for the `RAS_frac` field.
    pub const RAS_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `MPAM_frac` field.
    pub const MPAM_FRAC_SHIFT: u32 = 16;
    /// Mask for the `MPAM_frac` field.
    pub const MPAM_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `SME` field.
    pub const SME_SHIFT: u32 = 24;
    /// Mask for the `SME` field.
    pub const SME_MASK: u64 = 0b1111;
    /// Offset of the `RNDR_trap` field.
    pub const RNDR_TRAP_SHIFT: u32 = 28;
    /// Mask for the `RNDR_trap` field.
    pub const RNDR_TRAP_MASK: u64 = 0b1111;
    /// Offset of the `CSV2_frac` field.
    pub const CSV2_FRAC_SHIFT: u32 = 32;
    /// Mask for the `CSV2_frac` field.
    pub const CSV2_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `NMI` field.
    pub const NMI_SHIFT: u32 = 36;
    /// Mask for the `NMI` field.
    pub const NMI_MASK: u64 = 0b1111;
    /// Offset of the `MTE_frac` field.
    pub const MTE_FRAC_SHIFT: u32 = 40;
    /// Mask for the `MTE_frac` field.
    pub const MTE_FRAC_MASK: u64 = 0b1111;
    /// Offset of the `GCS` field.
    pub const GCS_SHIFT: u32 = 44;
    /// Mask for the `GCS` field.
    pub const GCS_MASK: u64 = 0b1111;
    /// Offset of the `THE` field.
    pub const THE_SHIFT: u32 = 48;
    /// Mask for the `THE` field.
    pub const THE_MASK: u64 = 0b1111;
    /// Offset of the `MTEX` field.
    pub const MTEX_SHIFT: u32 = 52;
    /// Mask for the `MTEX` field.
    pub const MTEX_MASK: u64 = 0b1111;
    /// Offset of the `DF2` field.
    pub const DF2_SHIFT: u32 = 56;
    /// Mask for the `DF2` field.
    pub const DF2_MASK: u64 = 0b1111;
    /// Offset of the `PFAR` field.
    pub const PFAR_SHIFT: u32 = 60;
    /// Mask for the `PFAR` field.
    pub const PFAR_MASK: u64 = 0b1111;

    /// Returns the value of the `BT` field.
    pub const fn bt(self) -> u8 {
        ((self.bits() >> Self::BT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BT` field.
    pub const fn set_bt(&mut self, value: u8) {
        let offset = Self::BT_SHIFT;
        assert!(value & (Self::BT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        ((self.bits() >> Self::SSBS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SSBS` field.
    pub const fn set_ssbs(&mut self, value: u8) {
        let offset = Self::SSBS_SHIFT;
        assert!(value & (Self::SSBS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SSBS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MTE` field.
    pub const fn mte(self) -> u8 {
        ((self.bits() >> Self::MTE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MTE` field.
    pub const fn set_mte(&mut self, value: u8) {
        let offset = Self::MTE_SHIFT;
        assert!(value & (Self::MTE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        ((self.bits() >> Self::RAS_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RAS_frac` field.
    pub const fn set_ras_frac(&mut self, value: u8) {
        let offset = Self::RAS_FRAC_SHIFT;
        assert!(value & (Self::RAS_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RAS_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MPAM_frac` field.
    pub const fn mpam_frac(self) -> u8 {
        ((self.bits() >> Self::MPAM_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MPAM_frac` field.
    pub const fn set_mpam_frac(&mut self, value: u8) {
        let offset = Self::MPAM_FRAC_SHIFT;
        assert!(value & (Self::MPAM_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPAM_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SME` field.
    pub const fn sme(self) -> u8 {
        ((self.bits() >> Self::SME_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SME` field.
    pub const fn set_sme(&mut self, value: u8) {
        let offset = Self::SME_SHIFT;
        assert!(value & (Self::SME_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SME_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RNDR_trap` field.
    pub const fn rndr_trap(self) -> u8 {
        ((self.bits() >> Self::RNDR_TRAP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RNDR_trap` field.
    pub const fn set_rndr_trap(&mut self, value: u8) {
        let offset = Self::RNDR_TRAP_SHIFT;
        assert!(value & (Self::RNDR_TRAP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RNDR_TRAP_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `CSV2_frac` field.
    pub const fn csv2_frac(self) -> u8 {
        ((self.bits() >> Self::CSV2_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CSV2_frac` field.
    pub const fn set_csv2_frac(&mut self, value: u8) {
        let offset = Self::CSV2_FRAC_SHIFT;
        assert!(value & (Self::CSV2_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV2_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `NMI` field.
    pub const fn nmi(self) -> u8 {
        ((self.bits() >> Self::NMI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `NMI` field.
    pub const fn set_nmi(&mut self, value: u8) {
        let offset = Self::NMI_SHIFT;
        assert!(value & (Self::NMI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NMI_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MTE_frac` field.
    pub const fn mte_frac(self) -> u8 {
        ((self.bits() >> Self::MTE_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MTE_frac` field.
    pub const fn set_mte_frac(&mut self, value: u8) {
        let offset = Self::MTE_FRAC_SHIFT;
        assert!(value & (Self::MTE_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTE_FRAC_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GCS` field.
    pub const fn gcs(self) -> u8 {
        ((self.bits() >> Self::GCS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GCS` field.
    pub const fn set_gcs(&mut self, value: u8) {
        let offset = Self::GCS_SHIFT;
        assert!(value & (Self::GCS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GCS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `THE` field.
    pub const fn the(self) -> u8 {
        ((self.bits() >> Self::THE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `THE` field.
    pub const fn set_the(&mut self, value: u8) {
        let offset = Self::THE_SHIFT;
        assert!(value & (Self::THE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `MTEX` field.
    pub const fn mtex(self) -> u8 {
        ((self.bits() >> Self::MTEX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MTEX` field.
    pub const fn set_mtex(&mut self, value: u8) {
        let offset = Self::MTEX_SHIFT;
        assert!(value & (Self::MTEX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTEX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `DF2` field.
    pub const fn df2(self) -> u8 {
        ((self.bits() >> Self::DF2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DF2` field.
    pub const fn set_df2(&mut self, value: u8) {
        let offset = Self::DF2_SHIFT;
        assert!(value & (Self::DF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DF2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PFAR` field.
    pub const fn pfar(self) -> u8 {
        ((self.bits() >> Self::PFAR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PFAR` field.
    pub const fn set_pfar(&mut self, value: u8) {
        let offset = Self::PFAR_SHIFT;
        assert!(value & (Self::PFAR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PFAR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64SMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `SMOP4` field.
    pub const SMOP4_SHIFT: u32 = 0;
    /// Offset of the `STMOP` field.
    pub const STMOP_SHIFT: u32 = 16;
    /// Offset of the `SFEXPA` field.
    pub const SFEXPA_SHIFT: u32 = 23;
    /// Offset of the `AES` field.
    pub const AES_SHIFT: u32 = 24;
    /// Offset of the `SBitPerm` field.
    pub const SBITPERM_SHIFT: u32 = 25;
    /// Offset of the `SF8DP2` field.
    pub const SF8DP2_SHIFT: u32 = 28;
    /// Offset of the `SF8DP4` field.
    pub const SF8DP4_SHIFT: u32 = 29;
    /// Offset of the `SF8FMA` field.
    pub const SF8FMA_SHIFT: u32 = 30;
    /// Offset of the `F32F32` field.
    pub const F32F32_SHIFT: u32 = 32;
    /// Offset of the `BI32I32` field.
    pub const BI32I32_SHIFT: u32 = 33;
    /// Offset of the `B16F32` field.
    pub const B16F32_SHIFT: u32 = 34;
    /// Offset of the `F16F32` field.
    pub const F16F32_SHIFT: u32 = 35;
    /// Offset of the `I8I32` field.
    pub const I8I32_SHIFT: u32 = 36;
    /// Mask for the `I8I32` field.
    pub const I8I32_MASK: u64 = 0b1111;
    /// Offset of the `F8F32` field.
    pub const F8F32_SHIFT: u32 = 40;
    /// Offset of the `F8F16` field.
    pub const F8F16_SHIFT: u32 = 41;
    /// Offset of the `F16F16` field.
    pub const F16F16_SHIFT: u32 = 42;
    /// Offset of the `B16B16` field.
    pub const B16B16_SHIFT: u32 = 43;
    /// Offset of the `I16I32` field.
    pub const I16I32_SHIFT: u32 = 44;
    /// Mask for the `I16I32` field.
    pub const I16I32_MASK: u64 = 0b1111;
    /// Offset of the `F64F64` field.
    pub const F64F64_SHIFT: u32 = 48;
    /// Offset of the `I16I64` field.
    pub const I16I64_SHIFT: u32 = 52;
    /// Mask for the `I16I64` field.
    pub const I16I64_MASK: u64 = 0b1111;
    /// Offset of the `SMEver` field.
    pub const SMEVER_SHIFT: u32 = 56;
    /// Mask for the `SMEver` field.
    pub const SMEVER_MASK: u64 = 0b1111;
    /// Offset of the `LUTv2` field.
    pub const LUTV2_SHIFT: u32 = 60;
    /// Offset of the `LUT6` field.
    pub const LUT6_SHIFT: u32 = 61;
    /// Offset of the `FA64` field.
    pub const FA64_SHIFT: u32 = 63;

    /// Returns the value of the `I8I32` field.
    pub const fn i8i32(self) -> u8 {
        ((self.bits() >> Self::I8I32_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `I8I32` field.
    pub const fn set_i8i32(&mut self, value: u8) {
        let offset = Self::I8I32_SHIFT;
        assert!(value & (Self::I8I32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I8I32_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `I16I32` field.
    pub const fn i16i32(self) -> u8 {
        ((self.bits() >> Self::I16I32_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `I16I32` field.
    pub const fn set_i16i32(&mut self, value: u8) {
        let offset = Self::I16I32_SHIFT;
        assert!(value & (Self::I16I32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I16I32_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `I16I64` field.
    pub const fn i16i64(self) -> u8 {
        ((self.bits() >> Self::I16I64_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `I16I64` field.
    pub const fn set_i16i64(&mut self, value: u8) {
        let offset = Self::I16I64_SHIFT;
        assert!(value & (Self::I16I64_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I16I64_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SMEver` field.
    pub const fn smever(self) -> u8 {
        ((self.bits() >> Self::SMEVER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SMEver` field.
    pub const fn set_smever(&mut self, value: u8) {
        let offset = Self::SMEVER_SHIFT;
        assert!(value & (Self::SMEVER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMEVER_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `ID_DFR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr0: u32 {
    }
}

impl IdDfr0 {
    /// Offset of the `CopDbg` field.
    pub const COPDBG_SHIFT: u32 = 0;
    /// Mask for the `CopDbg` field.
    pub const COPDBG_MASK: u32 = 0b1111;
    /// Offset of the `CopSDbg` field.
    pub const COPSDBG_SHIFT: u32 = 4;
    /// Mask for the `CopSDbg` field.
    pub const COPSDBG_MASK: u32 = 0b1111;
    /// Offset of the `MMapDbg` field.
    pub const MMAPDBG_SHIFT: u32 = 8;
    /// Mask for the `MMapDbg` field.
    pub const MMAPDBG_MASK: u32 = 0b1111;
    /// Offset of the `CopTrc` field.
    pub const COPTRC_SHIFT: u32 = 12;
    /// Mask for the `CopTrc` field.
    pub const COPTRC_MASK: u32 = 0b1111;
    /// Offset of the `MMapTrc` field.
    pub const MMAPTRC_SHIFT: u32 = 16;
    /// Mask for the `MMapTrc` field.
    pub const MMAPTRC_MASK: u32 = 0b1111;
    /// Offset of the `MProfDbg` field.
    pub const MPROFDBG_SHIFT: u32 = 20;
    /// Mask for the `MProfDbg` field.
    pub const MPROFDBG_MASK: u32 = 0b1111;
    /// Offset of the `PerfMon` field.
    pub const PERFMON_SHIFT: u32 = 24;
    /// Mask for the `PerfMon` field.
    pub const PERFMON_MASK: u32 = 0b1111;
    /// Offset of the `TraceFilt` field.
    pub const TRACEFILT_SHIFT: u32 = 28;
    /// Mask for the `TraceFilt` field.
    pub const TRACEFILT_MASK: u32 = 0b1111;

    /// Returns the value of the `CopDbg` field.
    pub const fn copdbg(self) -> u8 {
        ((self.bits() >> Self::COPDBG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CopDbg` field.
    pub const fn set_copdbg(&mut self, value: u8) {
        let offset = Self::COPDBG_SHIFT;
        assert!(value & (Self::COPDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CopSDbg` field.
    pub const fn copsdbg(self) -> u8 {
        ((self.bits() >> Self::COPSDBG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CopSDbg` field.
    pub const fn set_copsdbg(&mut self, value: u8) {
        let offset = Self::COPSDBG_SHIFT;
        assert!(value & (Self::COPSDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPSDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MMapDbg` field.
    pub const fn mmapdbg(self) -> u8 {
        ((self.bits() >> Self::MMAPDBG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MMapDbg` field.
    pub const fn set_mmapdbg(&mut self, value: u8) {
        let offset = Self::MMAPDBG_SHIFT;
        assert!(value & (Self::MMAPDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MMAPDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CopTrc` field.
    pub const fn coptrc(self) -> u8 {
        ((self.bits() >> Self::COPTRC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CopTrc` field.
    pub const fn set_coptrc(&mut self, value: u8) {
        let offset = Self::COPTRC_SHIFT;
        assert!(value & (Self::COPTRC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPTRC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MMapTrc` field.
    pub const fn mmaptrc(self) -> u8 {
        ((self.bits() >> Self::MMAPTRC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MMapTrc` field.
    pub const fn set_mmaptrc(&mut self, value: u8) {
        let offset = Self::MMAPTRC_SHIFT;
        assert!(value & (Self::MMAPTRC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MMAPTRC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MProfDbg` field.
    pub const fn mprofdbg(self) -> u8 {
        ((self.bits() >> Self::MPROFDBG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MProfDbg` field.
    pub const fn set_mprofdbg(&mut self, value: u8) {
        let offset = Self::MPROFDBG_SHIFT;
        assert!(value & (Self::MPROFDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPROFDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `PerfMon` field.
    pub const fn perfmon(self) -> u8 {
        ((self.bits() >> Self::PERFMON_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PerfMon` field.
    pub const fn set_perfmon(&mut self, value: u8) {
        let offset = Self::PERFMON_SHIFT;
        assert!(value & (Self::PERFMON_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PERFMON_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        ((self.bits() >> Self::TRACEFILT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TraceFilt` field.
    pub const fn set_tracefilt(&mut self, value: u8) {
        let offset = Self::TRACEFILT_SHIFT;
        assert!(value & (Self::TRACEFILT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRACEFILT_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_DFR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr1: u32 {
    }
}

impl IdDfr1 {
    /// Offset of the `MTPMU` field.
    pub const MTPMU_SHIFT: u32 = 0;
    /// Mask for the `MTPMU` field.
    pub const MTPMU_MASK: u32 = 0b1111;
    /// Offset of the `HPMN0` field.
    pub const HPMN0_SHIFT: u32 = 4;
    /// Mask for the `HPMN0` field.
    pub const HPMN0_MASK: u32 = 0b1111;

    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        ((self.bits() >> Self::MTPMU_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MTPMU` field.
    pub const fn set_mtpmu(&mut self, value: u8) {
        let offset = Self::MTPMU_SHIFT;
        assert!(value & (Self::MTPMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MTPMU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        ((self.bits() >> Self::HPMN0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HPMN0` field.
    pub const fn set_hpmn0(&mut self, value: u8) {
        let offset = Self::HPMN0_SHIFT;
        assert!(value & (Self::HPMN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPMN0_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_ISAR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar0: u32 {
    }
}

impl IdIsar0 {
    /// Offset of the `Swap` field.
    pub const SWAP_SHIFT: u32 = 0;
    /// Mask for the `Swap` field.
    pub const SWAP_MASK: u32 = 0b1111;
    /// Offset of the `BitCount` field.
    pub const BITCOUNT_SHIFT: u32 = 4;
    /// Mask for the `BitCount` field.
    pub const BITCOUNT_MASK: u32 = 0b1111;
    /// Offset of the `BitField` field.
    pub const BITFIELD_SHIFT: u32 = 8;
    /// Mask for the `BitField` field.
    pub const BITFIELD_MASK: u32 = 0b1111;
    /// Offset of the `CmpBranch` field.
    pub const CMPBRANCH_SHIFT: u32 = 12;
    /// Mask for the `CmpBranch` field.
    pub const CMPBRANCH_MASK: u32 = 0b1111;
    /// Offset of the `Coproc` field.
    pub const COPROC_SHIFT: u32 = 16;
    /// Mask for the `Coproc` field.
    pub const COPROC_MASK: u32 = 0b1111;
    /// Offset of the `Debug` field.
    pub const DEBUG_SHIFT: u32 = 20;
    /// Mask for the `Debug` field.
    pub const DEBUG_MASK: u32 = 0b1111;
    /// Offset of the `Divide` field.
    pub const DIVIDE_SHIFT: u32 = 24;
    /// Mask for the `Divide` field.
    pub const DIVIDE_MASK: u32 = 0b1111;

    /// Returns the value of the `Swap` field.
    pub const fn swap(self) -> u8 {
        ((self.bits() >> Self::SWAP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Swap` field.
    pub const fn set_swap(&mut self, value: u8) {
        let offset = Self::SWAP_SHIFT;
        assert!(value & (Self::SWAP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SWAP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BitCount` field.
    pub const fn bitcount(self) -> u8 {
        ((self.bits() >> Self::BITCOUNT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BitCount` field.
    pub const fn set_bitcount(&mut self, value: u8) {
        let offset = Self::BITCOUNT_SHIFT;
        assert!(value & (Self::BITCOUNT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BITCOUNT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BitField` field.
    pub const fn bitfield(self) -> u8 {
        ((self.bits() >> Self::BITFIELD_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BitField` field.
    pub const fn set_bitfield(&mut self, value: u8) {
        let offset = Self::BITFIELD_SHIFT;
        assert!(value & (Self::BITFIELD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BITFIELD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CmpBranch` field.
    pub const fn cmpbranch(self) -> u8 {
        ((self.bits() >> Self::CMPBRANCH_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CmpBranch` field.
    pub const fn set_cmpbranch(&mut self, value: u8) {
        let offset = Self::CMPBRANCH_SHIFT;
        assert!(value & (Self::CMPBRANCH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMPBRANCH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Coproc` field.
    pub const fn coproc(self) -> u8 {
        ((self.bits() >> Self::COPROC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Coproc` field.
    pub const fn set_coproc(&mut self, value: u8) {
        let offset = Self::COPROC_SHIFT;
        assert!(value & (Self::COPROC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COPROC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Debug` field.
    pub const fn debug(self) -> u8 {
        ((self.bits() >> Self::DEBUG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Debug` field.
    pub const fn set_debug(&mut self, value: u8) {
        let offset = Self::DEBUG_SHIFT;
        assert!(value & (Self::DEBUG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DEBUG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Divide` field.
    pub const fn divide(self) -> u8 {
        ((self.bits() >> Self::DIVIDE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Divide` field.
    pub const fn set_divide(&mut self, value: u8) {
        let offset = Self::DIVIDE_SHIFT;
        assert!(value & (Self::DIVIDE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DIVIDE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_ISAR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar1: u32 {
    }
}

impl IdIsar1 {
    /// Offset of the `Endian` field.
    pub const ENDIAN_SHIFT: u32 = 0;
    /// Mask for the `Endian` field.
    pub const ENDIAN_MASK: u32 = 0b1111;
    /// Offset of the `Except` field.
    pub const EXCEPT_SHIFT: u32 = 4;
    /// Mask for the `Except` field.
    pub const EXCEPT_MASK: u32 = 0b1111;
    /// Offset of the `Except_AR` field.
    pub const EXCEPT_AR_SHIFT: u32 = 8;
    /// Mask for the `Except_AR` field.
    pub const EXCEPT_AR_MASK: u32 = 0b1111;
    /// Offset of the `Extend` field.
    pub const EXTEND_SHIFT: u32 = 12;
    /// Mask for the `Extend` field.
    pub const EXTEND_MASK: u32 = 0b1111;
    /// Offset of the `IfThen` field.
    pub const IFTHEN_SHIFT: u32 = 16;
    /// Mask for the `IfThen` field.
    pub const IFTHEN_MASK: u32 = 0b1111;
    /// Offset of the `Immediate` field.
    pub const IMMEDIATE_SHIFT: u32 = 20;
    /// Mask for the `Immediate` field.
    pub const IMMEDIATE_MASK: u32 = 0b1111;
    /// Offset of the `Interwork` field.
    pub const INTERWORK_SHIFT: u32 = 24;
    /// Mask for the `Interwork` field.
    pub const INTERWORK_MASK: u32 = 0b1111;
    /// Offset of the `Jazelle` field.
    pub const JAZELLE_SHIFT: u32 = 28;
    /// Mask for the `Jazelle` field.
    pub const JAZELLE_MASK: u32 = 0b1111;

    /// Returns the value of the `Endian` field.
    pub const fn endian(self) -> u8 {
        ((self.bits() >> Self::ENDIAN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Endian` field.
    pub const fn set_endian(&mut self, value: u8) {
        let offset = Self::ENDIAN_SHIFT;
        assert!(value & (Self::ENDIAN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ENDIAN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Except` field.
    pub const fn except(self) -> u8 {
        ((self.bits() >> Self::EXCEPT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Except` field.
    pub const fn set_except(&mut self, value: u8) {
        let offset = Self::EXCEPT_SHIFT;
        assert!(value & (Self::EXCEPT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXCEPT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Except_AR` field.
    pub const fn except_ar(self) -> u8 {
        ((self.bits() >> Self::EXCEPT_AR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Except_AR` field.
    pub const fn set_except_ar(&mut self, value: u8) {
        let offset = Self::EXCEPT_AR_SHIFT;
        assert!(value & (Self::EXCEPT_AR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXCEPT_AR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Extend` field.
    pub const fn extend_(self) -> u8 {
        ((self.bits() >> Self::EXTEND_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Extend` field.
    pub const fn set_extend_(&mut self, value: u8) {
        let offset = Self::EXTEND_SHIFT;
        assert!(value & (Self::EXTEND_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EXTEND_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `IfThen` field.
    pub const fn ifthen(self) -> u8 {
        ((self.bits() >> Self::IFTHEN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `IfThen` field.
    pub const fn set_ifthen(&mut self, value: u8) {
        let offset = Self::IFTHEN_SHIFT;
        assert!(value & (Self::IFTHEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IFTHEN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Immediate` field.
    pub const fn immediate(self) -> u8 {
        ((self.bits() >> Self::IMMEDIATE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Immediate` field.
    pub const fn set_immediate(&mut self, value: u8) {
        let offset = Self::IMMEDIATE_SHIFT;
        assert!(value & (Self::IMMEDIATE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMMEDIATE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Interwork` field.
    pub const fn interwork(self) -> u8 {
        ((self.bits() >> Self::INTERWORK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Interwork` field.
    pub const fn set_interwork(&mut self, value: u8) {
        let offset = Self::INTERWORK_SHIFT;
        assert!(value & (Self::INTERWORK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTERWORK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Jazelle` field.
    pub const fn jazelle(self) -> u8 {
        ((self.bits() >> Self::JAZELLE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Jazelle` field.
    pub const fn set_jazelle(&mut self, value: u8) {
        let offset = Self::JAZELLE_SHIFT;
        assert!(value & (Self::JAZELLE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::JAZELLE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_ISAR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar2: u32 {
    }
}

impl IdIsar2 {
    /// Offset of the `LoadStore` field.
    pub const LOADSTORE_SHIFT: u32 = 0;
    /// Mask for the `LoadStore` field.
    pub const LOADSTORE_MASK: u32 = 0b1111;
    /// Offset of the `MemHint` field.
    pub const MEMHINT_SHIFT: u32 = 4;
    /// Mask for the `MemHint` field.
    pub const MEMHINT_MASK: u32 = 0b1111;
    /// Offset of the `MultiAccessInt` field.
    pub const MULTIACCESSINT_SHIFT: u32 = 8;
    /// Mask for the `MultiAccessInt` field.
    pub const MULTIACCESSINT_MASK: u32 = 0b1111;
    /// Offset of the `Mult` field.
    pub const MULT_SHIFT: u32 = 12;
    /// Mask for the `Mult` field.
    pub const MULT_MASK: u32 = 0b1111;
    /// Offset of the `MultS` field.
    pub const MULTS_SHIFT: u32 = 16;
    /// Mask for the `MultS` field.
    pub const MULTS_MASK: u32 = 0b1111;
    /// Offset of the `MultU` field.
    pub const MULTU_SHIFT: u32 = 20;
    /// Mask for the `MultU` field.
    pub const MULTU_MASK: u32 = 0b1111;
    /// Offset of the `PSR_AR` field.
    pub const PSR_AR_SHIFT: u32 = 24;
    /// Mask for the `PSR_AR` field.
    pub const PSR_AR_MASK: u32 = 0b1111;
    /// Offset of the `Reversal` field.
    pub const REVERSAL_SHIFT: u32 = 28;
    /// Mask for the `Reversal` field.
    pub const REVERSAL_MASK: u32 = 0b1111;

    /// Returns the value of the `LoadStore` field.
    pub const fn loadstore(self) -> u8 {
        ((self.bits() >> Self::LOADSTORE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LoadStore` field.
    pub const fn set_loadstore(&mut self, value: u8) {
        let offset = Self::LOADSTORE_SHIFT;
        assert!(value & (Self::LOADSTORE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LOADSTORE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MemHint` field.
    pub const fn memhint(self) -> u8 {
        ((self.bits() >> Self::MEMHINT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MemHint` field.
    pub const fn set_memhint(&mut self, value: u8) {
        let offset = Self::MEMHINT_SHIFT;
        assert!(value & (Self::MEMHINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MEMHINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MultiAccessInt` field.
    pub const fn multiaccessint(self) -> u8 {
        ((self.bits() >> Self::MULTIACCESSINT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MultiAccessInt` field.
    pub const fn set_multiaccessint(&mut self, value: u8) {
        let offset = Self::MULTIACCESSINT_SHIFT;
        assert!(value & (Self::MULTIACCESSINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULTIACCESSINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Mult` field.
    pub const fn mult(self) -> u8 {
        ((self.bits() >> Self::MULT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Mult` field.
    pub const fn set_mult(&mut self, value: u8) {
        let offset = Self::MULT_SHIFT;
        assert!(value & (Self::MULT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MultS` field.
    pub const fn mults(self) -> u8 {
        ((self.bits() >> Self::MULTS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MultS` field.
    pub const fn set_mults(&mut self, value: u8) {
        let offset = Self::MULTS_SHIFT;
        assert!(value & (Self::MULTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MultU` field.
    pub const fn multu(self) -> u8 {
        ((self.bits() >> Self::MULTU_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MultU` field.
    pub const fn set_multu(&mut self, value: u8) {
        let offset = Self::MULTU_SHIFT;
        assert!(value & (Self::MULTU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MULTU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `PSR_AR` field.
    pub const fn psr_ar(self) -> u8 {
        ((self.bits() >> Self::PSR_AR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PSR_AR` field.
    pub const fn set_psr_ar(&mut self, value: u8) {
        let offset = Self::PSR_AR_SHIFT;
        assert!(value & (Self::PSR_AR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PSR_AR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Reversal` field.
    pub const fn reversal(self) -> u8 {
        ((self.bits() >> Self::REVERSAL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Reversal` field.
    pub const fn set_reversal(&mut self, value: u8) {
        let offset = Self::REVERSAL_SHIFT;
        assert!(value & (Self::REVERSAL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVERSAL_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_ISAR3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar3: u32 {
    }
}

impl IdIsar3 {
    /// Offset of the `Saturate` field.
    pub const SATURATE_SHIFT: u32 = 0;
    /// Mask for the `Saturate` field.
    pub const SATURATE_MASK: u32 = 0b1111;
    /// Offset of the `SIMD` field.
    pub const SIMD_SHIFT: u32 = 4;
    /// Mask for the `SIMD` field.
    pub const SIMD_MASK: u32 = 0b1111;
    /// Offset of the `SVC` field.
    pub const SVC_SHIFT: u32 = 8;
    /// Mask for the `SVC` field.
    pub const SVC_MASK: u32 = 0b1111;
    /// Offset of the `SynchPrim` field.
    pub const SYNCHPRIM_SHIFT: u32 = 12;
    /// Mask for the `SynchPrim` field.
    pub const SYNCHPRIM_MASK: u32 = 0b1111;
    /// Offset of the `TabBranch` field.
    pub const TABBRANCH_SHIFT: u32 = 16;
    /// Mask for the `TabBranch` field.
    pub const TABBRANCH_MASK: u32 = 0b1111;
    /// Offset of the `T32Copy` field.
    pub const T32COPY_SHIFT: u32 = 20;
    /// Mask for the `T32Copy` field.
    pub const T32COPY_MASK: u32 = 0b1111;
    /// Offset of the `TrueNOP` field.
    pub const TRUENOP_SHIFT: u32 = 24;
    /// Mask for the `TrueNOP` field.
    pub const TRUENOP_MASK: u32 = 0b1111;
    /// Offset of the `T32EE` field.
    pub const T32EE_SHIFT: u32 = 28;
    /// Mask for the `T32EE` field.
    pub const T32EE_MASK: u32 = 0b1111;

    /// Returns the value of the `Saturate` field.
    pub const fn saturate(self) -> u8 {
        ((self.bits() >> Self::SATURATE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Saturate` field.
    pub const fn set_saturate(&mut self, value: u8) {
        let offset = Self::SATURATE_SHIFT;
        assert!(value & (Self::SATURATE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SATURATE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SIMD` field.
    pub const fn simd(self) -> u8 {
        ((self.bits() >> Self::SIMD_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SIMD` field.
    pub const fn set_simd(&mut self, value: u8) {
        let offset = Self::SIMD_SHIFT;
        assert!(value & (Self::SIMD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SIMD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SVC` field.
    pub const fn svc(self) -> u8 {
        ((self.bits() >> Self::SVC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SVC` field.
    pub const fn set_svc(&mut self, value: u8) {
        let offset = Self::SVC_SHIFT;
        assert!(value & (Self::SVC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SVC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SynchPrim` field.
    pub const fn synchprim(self) -> u8 {
        ((self.bits() >> Self::SYNCHPRIM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SynchPrim` field.
    pub const fn set_synchprim(&mut self, value: u8) {
        let offset = Self::SYNCHPRIM_SHIFT;
        assert!(value & (Self::SYNCHPRIM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYNCHPRIM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `TabBranch` field.
    pub const fn tabbranch(self) -> u8 {
        ((self.bits() >> Self::TABBRANCH_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TabBranch` field.
    pub const fn set_tabbranch(&mut self, value: u8) {
        let offset = Self::TABBRANCH_SHIFT;
        assert!(value & (Self::TABBRANCH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TABBRANCH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `T32Copy` field.
    pub const fn t32copy(self) -> u8 {
        ((self.bits() >> Self::T32COPY_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `T32Copy` field.
    pub const fn set_t32copy(&mut self, value: u8) {
        let offset = Self::T32COPY_SHIFT;
        assert!(value & (Self::T32COPY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T32COPY_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `TrueNOP` field.
    pub const fn truenop(self) -> u8 {
        ((self.bits() >> Self::TRUENOP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TrueNOP` field.
    pub const fn set_truenop(&mut self, value: u8) {
        let offset = Self::TRUENOP_SHIFT;
        assert!(value & (Self::TRUENOP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRUENOP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `T32EE` field.
    pub const fn t32ee(self) -> u8 {
        ((self.bits() >> Self::T32EE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `T32EE` field.
    pub const fn set_t32ee(&mut self, value: u8) {
        let offset = Self::T32EE_SHIFT;
        assert!(value & (Self::T32EE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T32EE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_ISAR4` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar4: u32 {
    }
}

impl IdIsar4 {
    /// Offset of the `Unpriv` field.
    pub const UNPRIV_SHIFT: u32 = 0;
    /// Mask for the `Unpriv` field.
    pub const UNPRIV_MASK: u32 = 0b1111;
    /// Offset of the `WithShifts` field.
    pub const WITHSHIFTS_SHIFT: u32 = 4;
    /// Mask for the `WithShifts` field.
    pub const WITHSHIFTS_MASK: u32 = 0b1111;
    /// Offset of the `Writeback` field.
    pub const WRITEBACK_SHIFT: u32 = 8;
    /// Mask for the `Writeback` field.
    pub const WRITEBACK_MASK: u32 = 0b1111;
    /// Offset of the `SMC` field.
    pub const SMC_SHIFT: u32 = 12;
    /// Mask for the `SMC` field.
    pub const SMC_MASK: u32 = 0b1111;
    /// Offset of the `Barrier` field.
    pub const BARRIER_SHIFT: u32 = 16;
    /// Mask for the `Barrier` field.
    pub const BARRIER_MASK: u32 = 0b1111;
    /// Offset of the `SynchPrim_frac` field.
    pub const SYNCHPRIM_FRAC_SHIFT: u32 = 20;
    /// Mask for the `SynchPrim_frac` field.
    pub const SYNCHPRIM_FRAC_MASK: u32 = 0b1111;
    /// Offset of the `PSR_M` field.
    pub const PSR_M_SHIFT: u32 = 24;
    /// Mask for the `PSR_M` field.
    pub const PSR_M_MASK: u32 = 0b1111;
    /// Offset of the `SWP_frac` field.
    pub const SWP_FRAC_SHIFT: u32 = 28;
    /// Mask for the `SWP_frac` field.
    pub const SWP_FRAC_MASK: u32 = 0b1111;

    /// Returns the value of the `Unpriv` field.
    pub const fn unpriv(self) -> u8 {
        ((self.bits() >> Self::UNPRIV_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Unpriv` field.
    pub const fn set_unpriv(&mut self, value: u8) {
        let offset = Self::UNPRIV_SHIFT;
        assert!(value & (Self::UNPRIV_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::UNPRIV_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `WithShifts` field.
    pub const fn withshifts(self) -> u8 {
        ((self.bits() >> Self::WITHSHIFTS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `WithShifts` field.
    pub const fn set_withshifts(&mut self, value: u8) {
        let offset = Self::WITHSHIFTS_SHIFT;
        assert!(value & (Self::WITHSHIFTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WITHSHIFTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Writeback` field.
    pub const fn writeback(self) -> u8 {
        ((self.bits() >> Self::WRITEBACK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Writeback` field.
    pub const fn set_writeback(&mut self, value: u8) {
        let offset = Self::WRITEBACK_SHIFT;
        assert!(value & (Self::WRITEBACK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WRITEBACK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SMC` field.
    pub const fn smc(self) -> u8 {
        ((self.bits() >> Self::SMC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SMC` field.
    pub const fn set_smc(&mut self, value: u8) {
        let offset = Self::SMC_SHIFT;
        assert!(value & (Self::SMC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SMC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Barrier` field.
    pub const fn barrier(self) -> u8 {
        ((self.bits() >> Self::BARRIER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Barrier` field.
    pub const fn set_barrier(&mut self, value: u8) {
        let offset = Self::BARRIER_SHIFT;
        assert!(value & (Self::BARRIER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BARRIER_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SynchPrim_frac` field.
    pub const fn synchprim_frac(self) -> u8 {
        ((self.bits() >> Self::SYNCHPRIM_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SynchPrim_frac` field.
    pub const fn set_synchprim_frac(&mut self, value: u8) {
        let offset = Self::SYNCHPRIM_FRAC_SHIFT;
        assert!(value & (Self::SYNCHPRIM_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SYNCHPRIM_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `PSR_M` field.
    pub const fn psr_m(self) -> u8 {
        ((self.bits() >> Self::PSR_M_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PSR_M` field.
    pub const fn set_psr_m(&mut self, value: u8) {
        let offset = Self::PSR_M_SHIFT;
        assert!(value & (Self::PSR_M_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PSR_M_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SWP_frac` field.
    pub const fn swp_frac(self) -> u8 {
        ((self.bits() >> Self::SWP_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SWP_frac` field.
    pub const fn set_swp_frac(&mut self, value: u8) {
        let offset = Self::SWP_FRAC_SHIFT;
        assert!(value & (Self::SWP_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SWP_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_ISAR5` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar5: u32 {
    }
}

impl IdIsar5 {
    /// Offset of the `SEVL` field.
    pub const SEVL_SHIFT: u32 = 0;
    /// Mask for the `SEVL` field.
    pub const SEVL_MASK: u32 = 0b1111;
    /// Offset of the `AES` field.
    pub const AES_SHIFT: u32 = 4;
    /// Mask for the `AES` field.
    pub const AES_MASK: u32 = 0b1111;
    /// Offset of the `SHA1` field.
    pub const SHA1_SHIFT: u32 = 8;
    /// Mask for the `SHA1` field.
    pub const SHA1_MASK: u32 = 0b1111;
    /// Offset of the `SHA2` field.
    pub const SHA2_SHIFT: u32 = 12;
    /// Mask for the `SHA2` field.
    pub const SHA2_MASK: u32 = 0b1111;
    /// Offset of the `CRC32` field.
    pub const CRC32_SHIFT: u32 = 16;
    /// Mask for the `CRC32` field.
    pub const CRC32_MASK: u32 = 0b1111;
    /// Offset of the `RDM` field.
    pub const RDM_SHIFT: u32 = 24;
    /// Mask for the `RDM` field.
    pub const RDM_MASK: u32 = 0b1111;
    /// Offset of the `VCMA` field.
    pub const VCMA_SHIFT: u32 = 28;
    /// Mask for the `VCMA` field.
    pub const VCMA_MASK: u32 = 0b1111;

    /// Returns the value of the `SEVL` field.
    pub const fn sevl(self) -> u8 {
        ((self.bits() >> Self::SEVL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SEVL` field.
    pub const fn set_sevl(&mut self, value: u8) {
        let offset = Self::SEVL_SHIFT;
        assert!(value & (Self::SEVL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEVL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `AES` field.
    pub const fn aes(self) -> u8 {
        ((self.bits() >> Self::AES_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AES` field.
    pub const fn set_aes(&mut self, value: u8) {
        let offset = Self::AES_SHIFT;
        assert!(value & (Self::AES_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AES_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SHA1` field.
    pub const fn sha1(self) -> u8 {
        ((self.bits() >> Self::SHA1_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SHA1` field.
    pub const fn set_sha1(&mut self, value: u8) {
        let offset = Self::SHA1_SHIFT;
        assert!(value & (Self::SHA1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SHA1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SHA2` field.
    pub const fn sha2(self) -> u8 {
        ((self.bits() >> Self::SHA2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SHA2` field.
    pub const fn set_sha2(&mut self, value: u8) {
        let offset = Self::SHA2_SHIFT;
        assert!(value & (Self::SHA2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SHA2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CRC32` field.
    pub const fn crc32(self) -> u8 {
        ((self.bits() >> Self::CRC32_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CRC32` field.
    pub const fn set_crc32(&mut self, value: u8) {
        let offset = Self::CRC32_SHIFT;
        assert!(value & (Self::CRC32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CRC32_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `RDM` field.
    pub const fn rdm(self) -> u8 {
        ((self.bits() >> Self::RDM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RDM` field.
    pub const fn set_rdm(&mut self, value: u8) {
        let offset = Self::RDM_SHIFT;
        assert!(value & (Self::RDM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RDM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `VCMA` field.
    pub const fn vcma(self) -> u8 {
        ((self.bits() >> Self::VCMA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `VCMA` field.
    pub const fn set_vcma(&mut self, value: u8) {
        let offset = Self::VCMA_SHIFT;
        assert!(value & (Self::VCMA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VCMA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_ISAR6` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar6: u32 {
    }
}

impl IdIsar6 {
    /// Offset of the `JSCVT` field.
    pub const JSCVT_SHIFT: u32 = 0;
    /// Mask for the `JSCVT` field.
    pub const JSCVT_MASK: u32 = 0b1111;
    /// Offset of the `DP` field.
    pub const DP_SHIFT: u32 = 4;
    /// Mask for the `DP` field.
    pub const DP_MASK: u32 = 0b1111;
    /// Offset of the `FHM` field.
    pub const FHM_SHIFT: u32 = 8;
    /// Mask for the `FHM` field.
    pub const FHM_MASK: u32 = 0b1111;
    /// Offset of the `SB` field.
    pub const SB_SHIFT: u32 = 12;
    /// Mask for the `SB` field.
    pub const SB_MASK: u32 = 0b1111;
    /// Offset of the `SPECRES` field.
    pub const SPECRES_SHIFT: u32 = 16;
    /// Mask for the `SPECRES` field.
    pub const SPECRES_MASK: u32 = 0b1111;
    /// Offset of the `BF16` field.
    pub const BF16_SHIFT: u32 = 20;
    /// Mask for the `BF16` field.
    pub const BF16_MASK: u32 = 0b1111;
    /// Offset of the `I8MM` field.
    pub const I8MM_SHIFT: u32 = 24;
    /// Mask for the `I8MM` field.
    pub const I8MM_MASK: u32 = 0b1111;
    /// Offset of the `CLRBHB` field.
    pub const CLRBHB_SHIFT: u32 = 28;
    /// Mask for the `CLRBHB` field.
    pub const CLRBHB_MASK: u32 = 0b1111;

    /// Returns the value of the `JSCVT` field.
    pub const fn jscvt(self) -> u8 {
        ((self.bits() >> Self::JSCVT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `JSCVT` field.
    pub const fn set_jscvt(&mut self, value: u8) {
        let offset = Self::JSCVT_SHIFT;
        assert!(value & (Self::JSCVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::JSCVT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `DP` field.
    pub const fn dp(self) -> u8 {
        ((self.bits() >> Self::DP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DP` field.
    pub const fn set_dp(&mut self, value: u8) {
        let offset = Self::DP_SHIFT;
        assert!(value & (Self::DP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `FHM` field.
    pub const fn fhm(self) -> u8 {
        ((self.bits() >> Self::FHM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `FHM` field.
    pub const fn set_fhm(&mut self, value: u8) {
        let offset = Self::FHM_SHIFT;
        assert!(value & (Self::FHM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FHM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SB` field.
    pub const fn sb(self) -> u8 {
        ((self.bits() >> Self::SB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SB` field.
    pub const fn set_sb(&mut self, value: u8) {
        let offset = Self::SB_SHIFT;
        assert!(value & (Self::SB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SB_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SPECRES` field.
    pub const fn specres(self) -> u8 {
        ((self.bits() >> Self::SPECRES_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SPECRES` field.
    pub const fn set_specres(&mut self, value: u8) {
        let offset = Self::SPECRES_SHIFT;
        assert!(value & (Self::SPECRES_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPECRES_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BF16` field.
    pub const fn bf16(self) -> u8 {
        ((self.bits() >> Self::BF16_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BF16` field.
    pub const fn set_bf16(&mut self, value: u8) {
        let offset = Self::BF16_SHIFT;
        assert!(value & (Self::BF16_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BF16_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `I8MM` field.
    pub const fn i8mm(self) -> u8 {
        ((self.bits() >> Self::I8MM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `I8MM` field.
    pub const fn set_i8mm(&mut self, value: u8) {
        let offset = Self::I8MM_SHIFT;
        assert!(value & (Self::I8MM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::I8MM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CLRBHB` field.
    pub const fn clrbhb(self) -> u8 {
        ((self.bits() >> Self::CLRBHB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CLRBHB` field.
    pub const fn set_clrbhb(&mut self, value: u8) {
        let offset = Self::CLRBHB_SHIFT;
        assert!(value & (Self::CLRBHB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CLRBHB_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_MMFR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr0: u32 {
    }
}

impl IdMmfr0 {
    /// Offset of the `VMSA` field.
    pub const VMSA_SHIFT: u32 = 0;
    /// Mask for the `VMSA` field.
    pub const VMSA_MASK: u32 = 0b1111;
    /// Offset of the `PMSA` field.
    pub const PMSA_SHIFT: u32 = 4;
    /// Mask for the `PMSA` field.
    pub const PMSA_MASK: u32 = 0b1111;
    /// Offset of the `OuterShr` field.
    pub const OUTERSHR_SHIFT: u32 = 8;
    /// Mask for the `OuterShr` field.
    pub const OUTERSHR_MASK: u32 = 0b1111;
    /// Offset of the `ShareLvl` field.
    pub const SHARELVL_SHIFT: u32 = 12;
    /// Mask for the `ShareLvl` field.
    pub const SHARELVL_MASK: u32 = 0b1111;
    /// Offset of the `TCM` field.
    pub const TCM_SHIFT: u32 = 16;
    /// Mask for the `TCM` field.
    pub const TCM_MASK: u32 = 0b1111;
    /// Offset of the `AuxReg` field.
    pub const AUXREG_SHIFT: u32 = 20;
    /// Mask for the `AuxReg` field.
    pub const AUXREG_MASK: u32 = 0b1111;
    /// Offset of the `FCSE` field.
    pub const FCSE_SHIFT: u32 = 24;
    /// Mask for the `FCSE` field.
    pub const FCSE_MASK: u32 = 0b1111;
    /// Offset of the `InnerShr` field.
    pub const INNERSHR_SHIFT: u32 = 28;
    /// Mask for the `InnerShr` field.
    pub const INNERSHR_MASK: u32 = 0b1111;

    /// Returns the value of the `VMSA` field.
    pub const fn vmsa(self) -> u8 {
        ((self.bits() >> Self::VMSA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `VMSA` field.
    pub const fn set_vmsa(&mut self, value: u8) {
        let offset = Self::VMSA_SHIFT;
        assert!(value & (Self::VMSA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMSA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `PMSA` field.
    pub const fn pmsa(self) -> u8 {
        ((self.bits() >> Self::PMSA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PMSA` field.
    pub const fn set_pmsa(&mut self, value: u8) {
        let offset = Self::PMSA_SHIFT;
        assert!(value & (Self::PMSA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `OuterShr` field.
    pub const fn outershr(self) -> u8 {
        ((self.bits() >> Self::OUTERSHR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `OuterShr` field.
    pub const fn set_outershr(&mut self, value: u8) {
        let offset = Self::OUTERSHR_SHIFT;
        assert!(value & (Self::OUTERSHR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OUTERSHR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `ShareLvl` field.
    pub const fn sharelvl(self) -> u8 {
        ((self.bits() >> Self::SHARELVL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ShareLvl` field.
    pub const fn set_sharelvl(&mut self, value: u8) {
        let offset = Self::SHARELVL_SHIFT;
        assert!(value & (Self::SHARELVL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SHARELVL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `TCM` field.
    pub const fn tcm(self) -> u8 {
        ((self.bits() >> Self::TCM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TCM` field.
    pub const fn set_tcm(&mut self, value: u8) {
        let offset = Self::TCM_SHIFT;
        assert!(value & (Self::TCM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `AuxReg` field.
    pub const fn auxreg(self) -> u8 {
        ((self.bits() >> Self::AUXREG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AuxReg` field.
    pub const fn set_auxreg(&mut self, value: u8) {
        let offset = Self::AUXREG_SHIFT;
        assert!(value & (Self::AUXREG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AUXREG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `FCSE` field.
    pub const fn fcse(self) -> u8 {
        ((self.bits() >> Self::FCSE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `FCSE` field.
    pub const fn set_fcse(&mut self, value: u8) {
        let offset = Self::FCSE_SHIFT;
        assert!(value & (Self::FCSE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FCSE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `InnerShr` field.
    pub const fn innershr(self) -> u8 {
        ((self.bits() >> Self::INNERSHR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `InnerShr` field.
    pub const fn set_innershr(&mut self, value: u8) {
        let offset = Self::INNERSHR_SHIFT;
        assert!(value & (Self::INNERSHR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INNERSHR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_MMFR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr1: u32 {
    }
}

impl IdMmfr1 {
    /// Offset of the `L1HvdVA` field.
    pub const L1HVDVA_SHIFT: u32 = 0;
    /// Mask for the `L1HvdVA` field.
    pub const L1HVDVA_MASK: u32 = 0b1111;
    /// Offset of the `L1UniVA` field.
    pub const L1UNIVA_SHIFT: u32 = 4;
    /// Mask for the `L1UniVA` field.
    pub const L1UNIVA_MASK: u32 = 0b1111;
    /// Offset of the `L1HvdSW` field.
    pub const L1HVDSW_SHIFT: u32 = 8;
    /// Mask for the `L1HvdSW` field.
    pub const L1HVDSW_MASK: u32 = 0b1111;
    /// Offset of the `L1UniSW` field.
    pub const L1UNISW_SHIFT: u32 = 12;
    /// Mask for the `L1UniSW` field.
    pub const L1UNISW_MASK: u32 = 0b1111;
    /// Offset of the `L1Hvd` field.
    pub const L1HVD_SHIFT: u32 = 16;
    /// Mask for the `L1Hvd` field.
    pub const L1HVD_MASK: u32 = 0b1111;
    /// Offset of the `L1Uni` field.
    pub const L1UNI_SHIFT: u32 = 20;
    /// Mask for the `L1Uni` field.
    pub const L1UNI_MASK: u32 = 0b1111;
    /// Offset of the `L1TstCln` field.
    pub const L1TSTCLN_SHIFT: u32 = 24;
    /// Mask for the `L1TstCln` field.
    pub const L1TSTCLN_MASK: u32 = 0b1111;
    /// Offset of the `BPred` field.
    pub const BPRED_SHIFT: u32 = 28;
    /// Mask for the `BPred` field.
    pub const BPRED_MASK: u32 = 0b1111;

    /// Returns the value of the `L1HvdVA` field.
    pub const fn l1hvdva(self) -> u8 {
        ((self.bits() >> Self::L1HVDVA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1HvdVA` field.
    pub const fn set_l1hvdva(&mut self, value: u8) {
        let offset = Self::L1HVDVA_SHIFT;
        assert!(value & (Self::L1HVDVA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDVA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1UniVA` field.
    pub const fn l1univa(self) -> u8 {
        ((self.bits() >> Self::L1UNIVA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1UniVA` field.
    pub const fn set_l1univa(&mut self, value: u8) {
        let offset = Self::L1UNIVA_SHIFT;
        assert!(value & (Self::L1UNIVA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1UNIVA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1HvdSW` field.
    pub const fn l1hvdsw(self) -> u8 {
        ((self.bits() >> Self::L1HVDSW_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1HvdSW` field.
    pub const fn set_l1hvdsw(&mut self, value: u8) {
        let offset = Self::L1HVDSW_SHIFT;
        assert!(value & (Self::L1HVDSW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDSW_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1UniSW` field.
    pub const fn l1unisw(self) -> u8 {
        ((self.bits() >> Self::L1UNISW_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1UniSW` field.
    pub const fn set_l1unisw(&mut self, value: u8) {
        let offset = Self::L1UNISW_SHIFT;
        assert!(value & (Self::L1UNISW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1UNISW_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1Hvd` field.
    pub const fn l1hvd(self) -> u8 {
        ((self.bits() >> Self::L1HVD_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1Hvd` field.
    pub const fn set_l1hvd(&mut self, value: u8) {
        let offset = Self::L1HVD_SHIFT;
        assert!(value & (Self::L1HVD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1Uni` field.
    pub const fn l1uni(self) -> u8 {
        ((self.bits() >> Self::L1UNI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1Uni` field.
    pub const fn set_l1uni(&mut self, value: u8) {
        let offset = Self::L1UNI_SHIFT;
        assert!(value & (Self::L1UNI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1UNI_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1TstCln` field.
    pub const fn l1tstcln(self) -> u8 {
        ((self.bits() >> Self::L1TSTCLN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1TstCln` field.
    pub const fn set_l1tstcln(&mut self, value: u8) {
        let offset = Self::L1TSTCLN_SHIFT;
        assert!(value & (Self::L1TSTCLN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1TSTCLN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BPred` field.
    pub const fn bpred(self) -> u8 {
        ((self.bits() >> Self::BPRED_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BPred` field.
    pub const fn set_bpred(&mut self, value: u8) {
        let offset = Self::BPRED_SHIFT;
        assert!(value & (Self::BPRED_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BPRED_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_MMFR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr2: u32 {
    }
}

impl IdMmfr2 {
    /// Offset of the `L1HvdFG` field.
    pub const L1HVDFG_SHIFT: u32 = 0;
    /// Mask for the `L1HvdFG` field.
    pub const L1HVDFG_MASK: u32 = 0b1111;
    /// Offset of the `L1HvdBG` field.
    pub const L1HVDBG_SHIFT: u32 = 4;
    /// Mask for the `L1HvdBG` field.
    pub const L1HVDBG_MASK: u32 = 0b1111;
    /// Offset of the `L1HvdRng` field.
    pub const L1HVDRNG_SHIFT: u32 = 8;
    /// Mask for the `L1HvdRng` field.
    pub const L1HVDRNG_MASK: u32 = 0b1111;
    /// Offset of the `HvdTLB` field.
    pub const HVDTLB_SHIFT: u32 = 12;
    /// Mask for the `HvdTLB` field.
    pub const HVDTLB_MASK: u32 = 0b1111;
    /// Offset of the `UniTLB` field.
    pub const UNITLB_SHIFT: u32 = 16;
    /// Mask for the `UniTLB` field.
    pub const UNITLB_MASK: u32 = 0b1111;
    /// Offset of the `MemBarr` field.
    pub const MEMBARR_SHIFT: u32 = 20;
    /// Mask for the `MemBarr` field.
    pub const MEMBARR_MASK: u32 = 0b1111;
    /// Offset of the `WFIStall` field.
    pub const WFISTALL_SHIFT: u32 = 24;
    /// Mask for the `WFIStall` field.
    pub const WFISTALL_MASK: u32 = 0b1111;
    /// Offset of the `HWAccFlg` field.
    pub const HWACCFLG_SHIFT: u32 = 28;
    /// Mask for the `HWAccFlg` field.
    pub const HWACCFLG_MASK: u32 = 0b1111;

    /// Returns the value of the `L1HvdFG` field.
    pub const fn l1hvdfg(self) -> u8 {
        ((self.bits() >> Self::L1HVDFG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1HvdFG` field.
    pub const fn set_l1hvdfg(&mut self, value: u8) {
        let offset = Self::L1HVDFG_SHIFT;
        assert!(value & (Self::L1HVDFG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDFG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1HvdBG` field.
    pub const fn l1hvdbg(self) -> u8 {
        ((self.bits() >> Self::L1HVDBG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1HvdBG` field.
    pub const fn set_l1hvdbg(&mut self, value: u8) {
        let offset = Self::L1HVDBG_SHIFT;
        assert!(value & (Self::L1HVDBG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDBG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `L1HvdRng` field.
    pub const fn l1hvdrng(self) -> u8 {
        ((self.bits() >> Self::L1HVDRNG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `L1HvdRng` field.
    pub const fn set_l1hvdrng(&mut self, value: u8) {
        let offset = Self::L1HVDRNG_SHIFT;
        assert!(value & (Self::L1HVDRNG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L1HVDRNG_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `HvdTLB` field.
    pub const fn hvdtlb(self) -> u8 {
        ((self.bits() >> Self::HVDTLB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HvdTLB` field.
    pub const fn set_hvdtlb(&mut self, value: u8) {
        let offset = Self::HVDTLB_SHIFT;
        assert!(value & (Self::HVDTLB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HVDTLB_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `UniTLB` field.
    pub const fn unitlb(self) -> u8 {
        ((self.bits() >> Self::UNITLB_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `UniTLB` field.
    pub const fn set_unitlb(&mut self, value: u8) {
        let offset = Self::UNITLB_SHIFT;
        assert!(value & (Self::UNITLB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::UNITLB_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MemBarr` field.
    pub const fn membarr(self) -> u8 {
        ((self.bits() >> Self::MEMBARR_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MemBarr` field.
    pub const fn set_membarr(&mut self, value: u8) {
        let offset = Self::MEMBARR_SHIFT;
        assert!(value & (Self::MEMBARR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MEMBARR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `WFIStall` field.
    pub const fn wfistall(self) -> u8 {
        ((self.bits() >> Self::WFISTALL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `WFIStall` field.
    pub const fn set_wfistall(&mut self, value: u8) {
        let offset = Self::WFISTALL_SHIFT;
        assert!(value & (Self::WFISTALL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::WFISTALL_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `HWAccFlg` field.
    pub const fn hwaccflg(self) -> u8 {
        ((self.bits() >> Self::HWACCFLG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HWAccFlg` field.
    pub const fn set_hwaccflg(&mut self, value: u8) {
        let offset = Self::HWACCFLG_SHIFT;
        assert!(value & (Self::HWACCFLG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HWACCFLG_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_MMFR3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr3: u32 {
    }
}

impl IdMmfr3 {
    /// Offset of the `CMaintVA` field.
    pub const CMAINTVA_SHIFT: u32 = 0;
    /// Mask for the `CMaintVA` field.
    pub const CMAINTVA_MASK: u32 = 0b1111;
    /// Offset of the `CMaintSW` field.
    pub const CMAINTSW_SHIFT: u32 = 4;
    /// Mask for the `CMaintSW` field.
    pub const CMAINTSW_MASK: u32 = 0b1111;
    /// Offset of the `BPMaint` field.
    pub const BPMAINT_SHIFT: u32 = 8;
    /// Mask for the `BPMaint` field.
    pub const BPMAINT_MASK: u32 = 0b1111;
    /// Offset of the `MaintBcst` field.
    pub const MAINTBCST_SHIFT: u32 = 12;
    /// Mask for the `MaintBcst` field.
    pub const MAINTBCST_MASK: u32 = 0b1111;
    /// Offset of the `PAN` field.
    pub const PAN_SHIFT: u32 = 16;
    /// Mask for the `PAN` field.
    pub const PAN_MASK: u32 = 0b1111;
    /// Offset of the `CohWalk` field.
    pub const COHWALK_SHIFT: u32 = 20;
    /// Mask for the `CohWalk` field.
    pub const COHWALK_MASK: u32 = 0b1111;
    /// Offset of the `CMemSz` field.
    pub const CMEMSZ_SHIFT: u32 = 24;
    /// Mask for the `CMemSz` field.
    pub const CMEMSZ_MASK: u32 = 0b1111;
    /// Offset of the `Supersec` field.
    pub const SUPERSEC_SHIFT: u32 = 28;
    /// Mask for the `Supersec` field.
    pub const SUPERSEC_MASK: u32 = 0b1111;

    /// Returns the value of the `CMaintVA` field.
    pub const fn cmaintva(self) -> u8 {
        ((self.bits() >> Self::CMAINTVA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CMaintVA` field.
    pub const fn set_cmaintva(&mut self, value: u8) {
        let offset = Self::CMAINTVA_SHIFT;
        assert!(value & (Self::CMAINTVA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMAINTVA_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CMaintSW` field.
    pub const fn cmaintsw(self) -> u8 {
        ((self.bits() >> Self::CMAINTSW_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CMaintSW` field.
    pub const fn set_cmaintsw(&mut self, value: u8) {
        let offset = Self::CMAINTSW_SHIFT;
        assert!(value & (Self::CMAINTSW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMAINTSW_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BPMaint` field.
    pub const fn bpmaint(self) -> u8 {
        ((self.bits() >> Self::BPMAINT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BPMaint` field.
    pub const fn set_bpmaint(&mut self, value: u8) {
        let offset = Self::BPMAINT_SHIFT;
        assert!(value & (Self::BPMAINT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BPMAINT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MaintBcst` field.
    pub const fn maintbcst(self) -> u8 {
        ((self.bits() >> Self::MAINTBCST_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MaintBcst` field.
    pub const fn set_maintbcst(&mut self, value: u8) {
        let offset = Self::MAINTBCST_SHIFT;
        assert!(value & (Self::MAINTBCST_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MAINTBCST_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        ((self.bits() >> Self::PAN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PAN` field.
    pub const fn set_pan(&mut self, value: u8) {
        let offset = Self::PAN_SHIFT;
        assert!(value & (Self::PAN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PAN_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CohWalk` field.
    pub const fn cohwalk(self) -> u8 {
        ((self.bits() >> Self::COHWALK_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CohWalk` field.
    pub const fn set_cohwalk(&mut self, value: u8) {
        let offset = Self::COHWALK_SHIFT;
        assert!(value & (Self::COHWALK_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::COHWALK_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CMemSz` field.
    pub const fn cmemsz(self) -> u8 {
        ((self.bits() >> Self::CMEMSZ_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CMemSz` field.
    pub const fn set_cmemsz(&mut self, value: u8) {
        let offset = Self::CMEMSZ_SHIFT;
        assert!(value & (Self::CMEMSZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CMEMSZ_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Supersec` field.
    pub const fn supersec(self) -> u8 {
        ((self.bits() >> Self::SUPERSEC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Supersec` field.
    pub const fn set_supersec(&mut self, value: u8) {
        let offset = Self::SUPERSEC_SHIFT;
        assert!(value & (Self::SUPERSEC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SUPERSEC_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_MMFR4` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr4: u32 {
    }
}

impl IdMmfr4 {
    /// Offset of the `SpecSEI` field.
    pub const SPECSEI_SHIFT: u32 = 0;
    /// Mask for the `SpecSEI` field.
    pub const SPECSEI_MASK: u32 = 0b1111;
    /// Offset of the `AC2` field.
    pub const AC2_SHIFT: u32 = 4;
    /// Mask for the `AC2` field.
    pub const AC2_MASK: u32 = 0b1111;
    /// Offset of the `XNX` field.
    pub const XNX_SHIFT: u32 = 8;
    /// Mask for the `XNX` field.
    pub const XNX_MASK: u32 = 0b1111;
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 12;
    /// Mask for the `CnP` field.
    pub const CNP_MASK: u32 = 0b1111;
    /// Offset of the `HPDS` field.
    pub const HPDS_SHIFT: u32 = 16;
    /// Mask for the `HPDS` field.
    pub const HPDS_MASK: u32 = 0b1111;
    /// Offset of the `LSM` field.
    pub const LSM_SHIFT: u32 = 20;
    /// Mask for the `LSM` field.
    pub const LSM_MASK: u32 = 0b1111;
    /// Offset of the `CCIDX` field.
    pub const CCIDX_SHIFT: u32 = 24;
    /// Mask for the `CCIDX` field.
    pub const CCIDX_MASK: u32 = 0b1111;
    /// Offset of the `EVT` field.
    pub const EVT_SHIFT: u32 = 28;
    /// Mask for the `EVT` field.
    pub const EVT_MASK: u32 = 0b1111;

    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        ((self.bits() >> Self::SPECSEI_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SpecSEI` field.
    pub const fn set_specsei(&mut self, value: u8) {
        let offset = Self::SPECSEI_SHIFT;
        assert!(value & (Self::SPECSEI_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPECSEI_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `AC2` field.
    pub const fn ac2(self) -> u8 {
        ((self.bits() >> Self::AC2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AC2` field.
    pub const fn set_ac2(&mut self, value: u8) {
        let offset = Self::AC2_SHIFT;
        assert!(value & (Self::AC2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AC2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        ((self.bits() >> Self::XNX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `XNX` field.
    pub const fn set_xnx(&mut self, value: u8) {
        let offset = Self::XNX_SHIFT;
        assert!(value & (Self::XNX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::XNX_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        ((self.bits() >> Self::CNP_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CnP` field.
    pub const fn set_cnp(&mut self, value: u8) {
        let offset = Self::CNP_SHIFT;
        assert!(value & (Self::CNP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CNP_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        ((self.bits() >> Self::HPDS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `HPDS` field.
    pub const fn set_hpds(&mut self, value: u8) {
        let offset = Self::HPDS_SHIFT;
        assert!(value & (Self::HPDS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPDS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        ((self.bits() >> Self::LSM_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LSM` field.
    pub const fn set_lsm(&mut self, value: u8) {
        let offset = Self::LSM_SHIFT;
        assert!(value & (Self::LSM_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LSM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        ((self.bits() >> Self::CCIDX_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CCIDX` field.
    pub const fn set_ccidx(&mut self, value: u8) {
        let offset = Self::CCIDX_SHIFT;
        assert!(value & (Self::CCIDX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CCIDX_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        ((self.bits() >> Self::EVT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EVT` field.
    pub const fn set_evt(&mut self, value: u8) {
        let offset = Self::EVT_SHIFT;
        assert!(value & (Self::EVT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EVT_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_MMFR5` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr5: u32 {
    }
}

impl IdMmfr5 {
    /// Offset of the `ETS` field.
    pub const ETS_SHIFT: u32 = 0;
    /// Mask for the `ETS` field.
    pub const ETS_MASK: u32 = 0b1111;
    /// Offset of the `nTLBPA` field.
    pub const NTLBPA_SHIFT: u32 = 4;
    /// Mask for the `nTLBPA` field.
    pub const NTLBPA_MASK: u32 = 0b1111;

    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        ((self.bits() >> Self::ETS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ETS` field.
    pub const fn set_ets(&mut self, value: u8) {
        let offset = Self::ETS_SHIFT;
        assert!(value & (Self::ETS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ETS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        ((self.bits() >> Self::NTLBPA_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `nTLBPA` field.
    pub const fn set_ntlbpa(&mut self, value: u8) {
        let offset = Self::NTLBPA_SHIFT;
        assert!(value & (Self::NTLBPA_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NTLBPA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_PFR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr0: u32 {
    }
}

impl IdPfr0 {
    /// Offset of the `State0` field.
    pub const STATE0_SHIFT: u32 = 0;
    /// Mask for the `State0` field.
    pub const STATE0_MASK: u32 = 0b1111;
    /// Offset of the `State1` field.
    pub const STATE1_SHIFT: u32 = 4;
    /// Mask for the `State1` field.
    pub const STATE1_MASK: u32 = 0b1111;
    /// Offset of the `State2` field.
    pub const STATE2_SHIFT: u32 = 8;
    /// Mask for the `State2` field.
    pub const STATE2_MASK: u32 = 0b1111;
    /// Offset of the `State3` field.
    pub const STATE3_SHIFT: u32 = 12;
    /// Mask for the `State3` field.
    pub const STATE3_MASK: u32 = 0b1111;
    /// Offset of the `CSV2` field.
    pub const CSV2_SHIFT: u32 = 16;
    /// Mask for the `CSV2` field.
    pub const CSV2_MASK: u32 = 0b1111;
    /// Offset of the `AMU` field.
    pub const AMU_SHIFT: u32 = 20;
    /// Mask for the `AMU` field.
    pub const AMU_MASK: u32 = 0b1111;
    /// Offset of the `DIT` field.
    pub const DIT_SHIFT: u32 = 24;
    /// Mask for the `DIT` field.
    pub const DIT_MASK: u32 = 0b1111;
    /// Offset of the `RAS` field.
    pub const RAS_SHIFT: u32 = 28;
    /// Mask for the `RAS` field.
    pub const RAS_MASK: u32 = 0b1111;

    /// Returns the value of the `State0` field.
    pub const fn state0(self) -> u8 {
        ((self.bits() >> Self::STATE0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `State0` field.
    pub const fn set_state0(&mut self, value: u8) {
        let offset = Self::STATE0_SHIFT;
        assert!(value & (Self::STATE0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `State1` field.
    pub const fn state1(self) -> u8 {
        ((self.bits() >> Self::STATE1_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `State1` field.
    pub const fn set_state1(&mut self, value: u8) {
        let offset = Self::STATE1_SHIFT;
        assert!(value & (Self::STATE1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `State2` field.
    pub const fn state2(self) -> u8 {
        ((self.bits() >> Self::STATE2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `State2` field.
    pub const fn set_state2(&mut self, value: u8) {
        let offset = Self::STATE2_SHIFT;
        assert!(value & (Self::STATE2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `State3` field.
    pub const fn state3(self) -> u8 {
        ((self.bits() >> Self::STATE3_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `State3` field.
    pub const fn set_state3(&mut self, value: u8) {
        let offset = Self::STATE3_SHIFT;
        assert!(value & (Self::STATE3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATE3_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        ((self.bits() >> Self::CSV2_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CSV2` field.
    pub const fn set_csv2(&mut self, value: u8) {
        let offset = Self::CSV2_SHIFT;
        assert!(value & (Self::CSV2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV2_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        ((self.bits() >> Self::AMU_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `AMU` field.
    pub const fn set_amu(&mut self, value: u8) {
        let offset = Self::AMU_SHIFT;
        assert!(value & (Self::AMU_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AMU_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        ((self.bits() >> Self::DIT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `DIT` field.
    pub const fn set_dit(&mut self, value: u8) {
        let offset = Self::DIT_SHIFT;
        assert!(value & (Self::DIT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::DIT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        ((self.bits() >> Self::RAS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RAS` field.
    pub const fn set_ras(&mut self, value: u8) {
        let offset = Self::RAS_SHIFT;
        assert!(value & (Self::RAS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RAS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_PFR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr1: u32 {
    }
}

impl IdPfr1 {
    /// Offset of the `ProgMod` field.
    pub const PROGMOD_SHIFT: u32 = 0;
    /// Mask for the `ProgMod` field.
    pub const PROGMOD_MASK: u32 = 0b1111;
    /// Offset of the `Security` field.
    pub const SECURITY_SHIFT: u32 = 4;
    /// Mask for the `Security` field.
    pub const SECURITY_MASK: u32 = 0b1111;
    /// Offset of the `MProgMod` field.
    pub const MPROGMOD_SHIFT: u32 = 8;
    /// Mask for the `MProgMod` field.
    pub const MPROGMOD_MASK: u32 = 0b1111;
    /// Offset of the `Virtualization` field.
    pub const VIRTUALIZATION_SHIFT: u32 = 12;
    /// Mask for the `Virtualization` field.
    pub const VIRTUALIZATION_MASK: u32 = 0b1111;
    /// Offset of the `GenTimer` field.
    pub const GENTIMER_SHIFT: u32 = 16;
    /// Mask for the `GenTimer` field.
    pub const GENTIMER_MASK: u32 = 0b1111;
    /// Offset of the `Sec_frac` field.
    pub const SEC_FRAC_SHIFT: u32 = 20;
    /// Mask for the `Sec_frac` field.
    pub const SEC_FRAC_MASK: u32 = 0b1111;
    /// Offset of the `Virt_frac` field.
    pub const VIRT_FRAC_SHIFT: u32 = 24;
    /// Mask for the `Virt_frac` field.
    pub const VIRT_FRAC_MASK: u32 = 0b1111;
    /// Offset of the `GIC` field.
    pub const GIC_SHIFT: u32 = 28;
    /// Mask for the `GIC` field.
    pub const GIC_MASK: u32 = 0b1111;

    /// Returns the value of the `ProgMod` field.
    pub const fn progmod(self) -> u8 {
        ((self.bits() >> Self::PROGMOD_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `ProgMod` field.
    pub const fn set_progmod(&mut self, value: u8) {
        let offset = Self::PROGMOD_SHIFT;
        assert!(value & (Self::PROGMOD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PROGMOD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Security` field.
    pub const fn security(self) -> u8 {
        ((self.bits() >> Self::SECURITY_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Security` field.
    pub const fn set_security(&mut self, value: u8) {
        let offset = Self::SECURITY_SHIFT;
        assert!(value & (Self::SECURITY_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SECURITY_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `MProgMod` field.
    pub const fn mprogmod(self) -> u8 {
        ((self.bits() >> Self::MPROGMOD_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `MProgMod` field.
    pub const fn set_mprogmod(&mut self, value: u8) {
        let offset = Self::MPROGMOD_SHIFT;
        assert!(value & (Self::MPROGMOD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MPROGMOD_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Virtualization` field.
    pub const fn virtualization(self) -> u8 {
        ((self.bits() >> Self::VIRTUALIZATION_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Virtualization` field.
    pub const fn set_virtualization(&mut self, value: u8) {
        let offset = Self::VIRTUALIZATION_SHIFT;
        assert!(value & (Self::VIRTUALIZATION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRTUALIZATION_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `GenTimer` field.
    pub const fn gentimer(self) -> u8 {
        ((self.bits() >> Self::GENTIMER_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GenTimer` field.
    pub const fn set_gentimer(&mut self, value: u8) {
        let offset = Self::GENTIMER_SHIFT;
        assert!(value & (Self::GENTIMER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GENTIMER_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Sec_frac` field.
    pub const fn sec_frac(self) -> u8 {
        ((self.bits() >> Self::SEC_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Sec_frac` field.
    pub const fn set_sec_frac(&mut self, value: u8) {
        let offset = Self::SEC_FRAC_SHIFT;
        assert!(value & (Self::SEC_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEC_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Virt_frac` field.
    pub const fn virt_frac(self) -> u8 {
        ((self.bits() >> Self::VIRT_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Virt_frac` field.
    pub const fn set_virt_frac(&mut self, value: u8) {
        let offset = Self::VIRT_FRAC_SHIFT;
        assert!(value & (Self::VIRT_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VIRT_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        ((self.bits() >> Self::GIC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GIC` field.
    pub const fn set_gic(&mut self, value: u8) {
        let offset = Self::GIC_SHIFT;
        assert!(value & (Self::GIC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GIC_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ID_PFR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr2: u32 {
    }
}

impl IdPfr2 {
    /// Offset of the `CSV3` field.
    pub const CSV3_SHIFT: u32 = 0;
    /// Mask for the `CSV3` field.
    pub const CSV3_MASK: u32 = 0b1111;
    /// Offset of the `SSBS` field.
    pub const SSBS_SHIFT: u32 = 4;
    /// Mask for the `SSBS` field.
    pub const SSBS_MASK: u32 = 0b1111;
    /// Offset of the `RAS_frac` field.
    pub const RAS_FRAC_SHIFT: u32 = 8;
    /// Mask for the `RAS_frac` field.
    pub const RAS_FRAC_MASK: u32 = 0b1111;

    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        ((self.bits() >> Self::CSV3_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `CSV3` field.
    pub const fn set_csv3(&mut self, value: u8) {
        let offset = Self::CSV3_SHIFT;
        assert!(value & (Self::CSV3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CSV3_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        ((self.bits() >> Self::SSBS_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `SSBS` field.
    pub const fn set_ssbs(&mut self, value: u8) {
        let offset = Self::SSBS_SHIFT;
        assert!(value & (Self::SSBS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SSBS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        ((self.bits() >> Self::RAS_FRAC_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `RAS_frac` field.
    pub const fn set_ras_frac(&mut self, value: u8) {
        let offset = Self::RAS_FRAC_SHIFT;
        assert!(value & (Self::RAS_FRAC_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RAS_FRAC_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `IFAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ifar: u32 {
    }
}

impl Ifar {
    /// Offset of the `VA` field.
    pub const VA_SHIFT: u32 = 0;
    /// Mask for the `VA` field.
    pub const VA_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        ((self.bits() >> Self::VA_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `VA` field.
    pub const fn set_va(&mut self, value: u32) {
        let offset = Self::VA_SHIFT;
        assert!(value & (Self::VA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `IFSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ifsr: u32 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `FnV` bit.
        const FNV = 1 << 16;
    }
}

impl Ifsr {
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b111111;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `FnV` field.
    pub const FNV_SHIFT: u32 = 16;

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `ISR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Isr: u32 {
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
    }
}

impl Isr {
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ISR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 6;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 7;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 8;
    /// Offset of the `FS` field.
    pub const FS_SHIFT: u32 = 9;
    /// Offset of the `IS` field.
    pub const IS_SHIFT: u32 = 10;
}

bitflags! {
    /// `MAIR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair0: u32 {
    }
}

impl Mair0 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 4);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) & 0b11111111) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 4);
        let offset = Self::ATTR_SHIFT + (n - 0) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `MAIR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair1: u32 {
    }
}

impl Mair1 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u32 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n >= 4 && n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 4) * 8)) & 0b11111111) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n >= 4 && n < 8);
        let offset = Self::ATTR_SHIFT + (n - 4) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MAIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MairEl1 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) & 0b11111111) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::ATTR_SHIFT + (n - 0) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MAIR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MairEl2 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) & 0b11111111) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::ATTR_SHIFT + (n - 0) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `MAIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MairEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl MairEl3 {
    /// Offset of the `Attr<n>` field.
    pub const ATTR_SHIFT: u32 = 0;
    /// Mask for the `Attr<n>` field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::ATTR_SHIFT + (n - 0) * 8)) & 0b11111111) as u8
    }

    /// Sets the value of the `Attr<n>` field.
    pub const fn set_attr(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::ATTR_SHIFT + (n - 0) * 8;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MDCCINT_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `TX` field.
    pub const TX_SHIFT: u32 = 29;
    /// Offset of the `RX` field.
    pub const RX_SHIFT: u32 = 30;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl MdcrEl2 {
    /// Offset of the `HPMN` field.
    pub const HPMN_SHIFT: u32 = 0;
    /// Mask for the `HPMN` field.
    pub const HPMN_MASK: u64 = 0b11111;
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
        ((self.bits() >> Self::HPMN_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `HPMN` field.
    pub const fn set_hpmn(&mut self, value: u8) {
        let offset = Self::HPMN_SHIFT;
        assert!(value & (Self::HPMN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::HPMN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `E2PB` field.
    pub const fn e2pb(self) -> u8 {
        ((self.bits() >> Self::E2PB_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `E2PB` field.
    pub const fn set_e2pb(&mut self, value: u8) {
        let offset = Self::E2PB_SHIFT;
        assert!(value & (Self::E2PB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E2PB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `E2TB` field.
    pub const fn e2tb(self) -> u8 {
        ((self.bits() >> Self::E2TB_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `E2TB` field.
    pub const fn set_e2tb(&mut self, value: u8) {
        let offset = Self::E2TB_SHIFT;
        assert!(value & (Self::E2TB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::E2TB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        ((self.bits() >> Self::PMSSE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `PMSSE` field.
    pub const fn set_pmsse(&mut self, value: u8) {
        let offset = Self::PMSSE_SHIFT;
        assert!(value & (Self::PMSSE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSSE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        ((self.bits() >> Self::PMEE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `PMEE` field.
    pub const fn set_pmee(&mut self, value: u8) {
        let offset = Self::PMEE_SHIFT;
        assert!(value & (Self::PMEE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMEE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
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
        ((self.bits() >> Self::NSPB_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `NSPB` field.
    pub const fn set_nspb(&mut self, value: u8) {
        let offset = Self::NSPB_SHIFT;
        assert!(value & (Self::NSPB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSPB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SPD32` field.
    pub const fn spd32(self) -> u8 {
        ((self.bits() >> Self::SPD32_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SPD32` field.
    pub const fn set_spd32(&mut self, value: u8) {
        let offset = Self::SPD32_SHIFT;
        assert!(value & (Self::SPD32_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPD32_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `NSTB` field.
    pub const fn nstb(self) -> u8 {
        ((self.bits() >> Self::NSTB_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `NSTB` field.
    pub const fn set_nstb(&mut self, value: u8) {
        let offset = Self::NSTB_SHIFT;
        assert!(value & (Self::NSTB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::NSTB_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        ((self.bits() >> Self::PMSSE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `PMSSE` field.
    pub const fn set_pmsse(&mut self, value: u8) {
        let offset = Self::PMSSE_SHIFT;
        assert!(value & (Self::PMSSE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSSE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SBRBE` field.
    pub const fn sbrbe(self) -> u8 {
        ((self.bits() >> Self::SBRBE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SBRBE` field.
    pub const fn set_sbrbe(&mut self, value: u8) {
        let offset = Self::SBRBE_SHIFT;
        assert!(value & (Self::SBRBE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SBRBE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        ((self.bits() >> Self::PMEE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `PMEE` field.
    pub const fn set_pmee(&mut self, value: u8) {
        let offset = Self::PMEE_SHIFT;
        assert!(value & (Self::PMEE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMEE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `EPMSSAD` field.
    pub const fn epmssad(self) -> u8 {
        ((self.bits() >> Self::EPMSSAD_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `EPMSSAD` field.
    pub const fn set_epmssad(&mut self, value: u8) {
        let offset = Self::EPMSSAD_SHIFT;
        assert!(value & (Self::EPMSSAD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EPMSSAD_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ETBAD` field.
    pub const fn etbad(self) -> u8 {
        ((self.bits() >> Self::ETBAD_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ETBAD` field.
    pub const fn set_etbad(&mut self, value: u8) {
        let offset = Self::ETBAD_SHIFT;
        assert!(value & (Self::ETBAD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ETBAD_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMSEE` field.
    pub const fn pmsee(self) -> u8 {
        ((self.bits() >> Self::PMSEE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `PMSEE` field.
    pub const fn set_pmsee(&mut self, value: u8) {
        let offset = Self::PMSEE_SHIFT;
        assert!(value & (Self::PMSEE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMSEE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TRBEE` field.
    pub const fn trbee(self) -> u8 {
        ((self.bits() >> Self::TRBEE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TRBEE` field.
    pub const fn set_trbee(&mut self, value: u8) {
        let offset = Self::TRBEE_SHIFT;
        assert!(value & (Self::TRBEE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TRBEE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MDSCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 0;
    /// Offset of the `ERR` field.
    pub const ERR_SHIFT: u32 = 6;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 12;
    /// Offset of the `KDE` field.
    pub const KDE_SHIFT: u32 = 13;
    /// Offset of the `HDE` field.
    pub const HDE_SHIFT: u32 = 14;
    /// Offset of the `MDE` field.
    pub const MDE_SHIFT: u32 = 15;
    /// Offset of the `SC2` field.
    pub const SC2_SHIFT: u32 = 19;
    /// Offset of the `TDA` field.
    pub const TDA_SHIFT: u32 = 21;
    /// Offset of the `INTdis` field.
    pub const INTDIS_SHIFT: u32 = 22;
    /// Mask for the `INTdis` field.
    pub const INTDIS_MASK: u64 = 0b11;
    /// Offset of the `TXU` field.
    pub const TXU_SHIFT: u32 = 26;
    /// Offset of the `RXO` field.
    pub const RXO_SHIFT: u32 = 27;
    /// Offset of the `TXfull` field.
    pub const TXFULL_SHIFT: u32 = 29;
    /// Offset of the `RXfull` field.
    pub const RXFULL_SHIFT: u32 = 30;
    /// Offset of the `TFO` field.
    pub const TFO_SHIFT: u32 = 31;
    /// Offset of the `EMBWE` field.
    pub const EMBWE_SHIFT: u32 = 32;
    /// Offset of the `TTA` field.
    pub const TTA_SHIFT: u32 = 33;
    /// Offset of the `EnSPM` field.
    pub const ENSPM_SHIFT: u32 = 34;
    /// Offset of the `EHBWE` field.
    pub const EHBWE_SHIFT: u32 = 35;
    /// Offset of the `EnSTEPOP` field.
    pub const ENSTEPOP_SHIFT: u32 = 50;

    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        ((self.bits() >> Self::INTDIS_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `INTdis` field.
    pub const fn set_intdis(&mut self, value: u8) {
        let offset = Self::INTDIS_SHIFT;
        assert!(value & (Self::INTDIS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INTDIS_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `MIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Midr: u32 {
    }
}

impl Midr {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u32 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u32 = 0b111111111111;
    /// Offset of the `Architecture` field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the `Architecture` field.
    pub const ARCHITECTURE_MASK: u32 = 0b1111;
    /// Offset of the `Variant` field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the `Variant` field.
    pub const VARIANT_MASK: u32 = 0b1111;
    /// Offset of the `Implementer` field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the `Implementer` field.
    pub const IMPLEMENTER_MASK: u32 = 0b11111111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> Self::PARTNUM_SHIFT) & 0b111111111111) as u16
    }

    /// Sets the value of the `PartNum` field.
    pub const fn set_partnum(&mut self, value: u16) {
        let offset = Self::PARTNUM_SHIFT;
        assert!(value & (Self::PARTNUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> Self::ARCHITECTURE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Architecture` field.
    pub const fn set_architecture(&mut self, value: u8) {
        let offset = Self::ARCHITECTURE_SHIFT;
        assert!(value & (Self::ARCHITECTURE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> Self::VARIANT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Variant` field.
    pub const fn set_variant(&mut self, value: u8) {
        let offset = Self::VARIANT_SHIFT;
        assert!(value & (Self::VARIANT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> Self::IMPLEMENTER_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Implementer` field.
    pub const fn set_implementer(&mut self, value: u8) {
        let offset = Self::IMPLEMENTER_SHIFT;
        assert!(value & (Self::IMPLEMENTER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MidrEl1 {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u64 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u64 = 0b111111111111;
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
    pub const IMPLEMENTER_MASK: u64 = 0b11111111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> Self::PARTNUM_SHIFT) & 0b111111111111) as u16
    }

    /// Sets the value of the `PartNum` field.
    pub const fn set_partnum(&mut self, value: u16) {
        let offset = Self::PARTNUM_SHIFT;
        assert!(value & (Self::PARTNUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> Self::ARCHITECTURE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Architecture` field.
    pub const fn set_architecture(&mut self, value: u8) {
        let offset = Self::ARCHITECTURE_SHIFT;
        assert!(value & (Self::ARCHITECTURE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> Self::VARIANT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Variant` field.
    pub const fn set_variant(&mut self, value: u8) {
        let offset = Self::VARIANT_SHIFT;
        assert!(value & (Self::VARIANT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> Self::IMPLEMENTER_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Implementer` field.
    pub const fn set_implementer(&mut self, value: u8) {
        let offset = Self::IMPLEMENTER_SHIFT;
        assert!(value & (Self::IMPLEMENTER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl Mpam2El2 {
    /// Offset of the `PARTID` field.
    pub const PARTID_SHIFT: u32 = 0;
    /// Mask for the `PARTID` field.
    pub const PARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PARTID_I` field.
    pub const PARTID_I_SHIFT: u32 = 0;
    /// Mask for the `PARTID_I` field.
    pub const PARTID_I_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PARTID_D` field.
    pub const PARTID_D_SHIFT: u32 = 16;
    /// Mask for the `PARTID_D` field.
    pub const PARTID_D_MASK: u64 = 0b1111111111111111;
    /// Offset of the `altPARTID` field.
    pub const ALTPARTID_SHIFT: u32 = 16;
    /// Mask for the `altPARTID` field.
    pub const ALTPARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PMG` field.
    pub const PMG_SHIFT: u32 = 32;
    /// Mask for the `PMG` field.
    pub const PMG_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PMG_I` field.
    pub const PMG_I_SHIFT: u32 = 32;
    /// Mask for the `PMG_I` field.
    pub const PMG_I_MASK: u64 = 0b11111111;
    /// Offset of the `PMG_D` field.
    pub const PMG_D_SHIFT: u32 = 40;
    /// Mask for the `PMG_D` field.
    pub const PMG_D_MASK: u64 = 0b11111111;
    /// Offset of the `TRAPMPAM1EL1` field.
    pub const TRAPMPAM1EL1_SHIFT: u32 = 48;
    /// Offset of the `altPMG` field.
    pub const ALTPMG_SHIFT: u32 = 48;
    /// Mask for the `altPMG` field.
    pub const ALTPMG_MASK: u64 = 0b1111111111111111;
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
        ((self.bits() >> Self::PARTID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PARTID` field.
    pub const fn set_partid(&mut self, value: u16) {
        let offset = Self::PARTID_SHIFT;
        assert!(value & (Self::PARTID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        ((self.bits() >> Self::PARTID_I_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PARTID_I` field.
    pub const fn set_partid_i(&mut self, value: u16) {
        let offset = Self::PARTID_I_SHIFT;
        assert!(value & (Self::PARTID_I_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_I_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        ((self.bits() >> Self::PARTID_D_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PARTID_D` field.
    pub const fn set_partid_d(&mut self, value: u16) {
        let offset = Self::PARTID_D_SHIFT;
        assert!(value & (Self::PARTID_D_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_D_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        ((self.bits() >> Self::ALTPARTID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `altPARTID` field.
    pub const fn set_altpartid(&mut self, value: u16) {
        let offset = Self::ALTPARTID_SHIFT;
        assert!(value & (Self::ALTPARTID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ALTPARTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        ((self.bits() >> Self::PMG_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PMG` field.
    pub const fn set_pmg(&mut self, value: u16) {
        let offset = Self::PMG_SHIFT;
        assert!(value & (Self::PMG_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        ((self.bits() >> Self::PMG_I_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `PMG_I` field.
    pub const fn set_pmg_i(&mut self, value: u8) {
        let offset = Self::PMG_I_SHIFT;
        assert!(value & (Self::PMG_I_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_I_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        ((self.bits() >> Self::PMG_D_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `PMG_D` field.
    pub const fn set_pmg_d(&mut self, value: u8) {
        let offset = Self::PMG_D_SHIFT;
        assert!(value & (Self::PMG_D_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_D_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        ((self.bits() >> Self::ALTPMG_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `altPMG` field.
    pub const fn set_altpmg(&mut self, value: u16) {
        let offset = Self::ALTPMG_SHIFT;
        assert!(value & (Self::ALTPMG_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ALTPMG_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
impl Mpam3El3 {
    /// Offset of the `PARTID` field.
    pub const PARTID_SHIFT: u32 = 0;
    /// Mask for the `PARTID` field.
    pub const PARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PARTID_I` field.
    pub const PARTID_I_SHIFT: u32 = 0;
    /// Mask for the `PARTID_I` field.
    pub const PARTID_I_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PARTID_D` field.
    pub const PARTID_D_SHIFT: u32 = 16;
    /// Mask for the `PARTID_D` field.
    pub const PARTID_D_MASK: u64 = 0b1111111111111111;
    /// Offset of the `altPARTID` field.
    pub const ALTPARTID_SHIFT: u32 = 16;
    /// Mask for the `altPARTID` field.
    pub const ALTPARTID_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PMG` field.
    pub const PMG_SHIFT: u32 = 32;
    /// Mask for the `PMG` field.
    pub const PMG_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PMG_I` field.
    pub const PMG_I_SHIFT: u32 = 32;
    /// Mask for the `PMG_I` field.
    pub const PMG_I_MASK: u64 = 0b11111111;
    /// Offset of the `PMG_D` field.
    pub const PMG_D_SHIFT: u32 = 40;
    /// Mask for the `PMG_D` field.
    pub const PMG_D_MASK: u64 = 0b11111111;
    /// Offset of the `altPMG` field.
    pub const ALTPMG_SHIFT: u32 = 48;
    /// Mask for the `altPMG` field.
    pub const ALTPMG_MASK: u64 = 0b1111111111111111;
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
        ((self.bits() >> Self::PARTID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PARTID` field.
    pub const fn set_partid(&mut self, value: u16) {
        let offset = Self::PARTID_SHIFT;
        assert!(value & (Self::PARTID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        ((self.bits() >> Self::PARTID_I_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PARTID_I` field.
    pub const fn set_partid_i(&mut self, value: u16) {
        let offset = Self::PARTID_I_SHIFT;
        assert!(value & (Self::PARTID_I_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_I_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        ((self.bits() >> Self::PARTID_D_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PARTID_D` field.
    pub const fn set_partid_d(&mut self, value: u16) {
        let offset = Self::PARTID_D_SHIFT;
        assert!(value & (Self::PARTID_D_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_D_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        ((self.bits() >> Self::ALTPARTID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `altPARTID` field.
    pub const fn set_altpartid(&mut self, value: u16) {
        let offset = Self::ALTPARTID_SHIFT;
        assert!(value & (Self::ALTPARTID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ALTPARTID_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        ((self.bits() >> Self::PMG_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PMG` field.
    pub const fn set_pmg(&mut self, value: u16) {
        let offset = Self::PMG_SHIFT;
        assert!(value & (Self::PMG_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        ((self.bits() >> Self::PMG_I_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `PMG_I` field.
    pub const fn set_pmg_i(&mut self, value: u8) {
        let offset = Self::PMG_I_SHIFT;
        assert!(value & (Self::PMG_I_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_I_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        ((self.bits() >> Self::PMG_D_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `PMG_D` field.
    pub const fn set_pmg_d(&mut self, value: u8) {
        let offset = Self::PMG_D_SHIFT;
        assert!(value & (Self::PMG_D_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PMG_D_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        ((self.bits() >> Self::ALTPMG_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `altPMG` field.
    pub const fn set_altpmg(&mut self, value: u16) {
        let offset = Self::ALTPMG_SHIFT;
        assert!(value & (Self::ALTPMG_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ALTPMG_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMHCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `EL0_VPMEN` field.
    pub const EL0_VPMEN_SHIFT: u32 = 0;
    /// Offset of the `EL1_VPMEN` field.
    pub const EL1_VPMEN_SHIFT: u32 = 1;
    /// Offset of the `VPMEN` field.
    pub const VPMEN_SHIFT: u32 = 2;
    /// Offset of the `VMMEN` field.
    pub const VMMEN_SHIFT: u32 = 3;
    /// Offset of the `SMVPMEN` field.
    pub const SMVPMEN_SHIFT: u32 = 4;
    /// Offset of the `SMVMMEN` field.
    pub const SMVMMEN_SHIFT: u32 = 5;
    /// Offset of the `GSTAPP_PLK` field.
    pub const GSTAPP_PLK_SHIFT: u32 = 8;
    /// Offset of the `TRAP_MPAMIDR_EL1` field.
    pub const TRAP_MPAMIDR_EL1_SHIFT: u32 = 31;
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAMIDR_EL1` system register value.
    ///
    /// Indicates the maximum PARTID and PMG values supported in the implementation and the support for other optional features.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `PARTID_MAX` field.
    pub const PARTID_MAX_SHIFT: u32 = 0;
    /// Mask for the `PARTID_MAX` field.
    pub const PARTID_MAX_MASK: u64 = 0b1111111111111111;
    /// Offset of the `HAS_HCR` field.
    pub const HAS_HCR_SHIFT: u32 = 17;
    /// Offset of the `VPMR_MAX` field.
    pub const VPMR_MAX_SHIFT: u32 = 18;
    /// Mask for the `VPMR_MAX` field.
    pub const VPMR_MAX_MASK: u64 = 0b111;
    /// Offset of the `HAS_ALT_ID` field.
    pub const HAS_ALT_ID_SHIFT: u32 = 21;
    /// Offset of the `HAS_INSTR_ALT_ID` field.
    pub const HAS_INSTR_ALT_ID_SHIFT: u32 = 22;
    /// Offset of the `HAS_BW_CTRL` field.
    pub const HAS_BW_CTRL_SHIFT: u32 = 56;
    /// Offset of the `HAS_ALTSP` field.
    pub const HAS_ALTSP_SHIFT: u32 = 57;
    /// Offset of the `HAS_TIDR` field.
    pub const HAS_TIDR_SHIFT: u32 = 58;
    /// Offset of the `SP4` field.
    pub const SP4_SHIFT: u32 = 59;
    /// Offset of the `HAS_FORCE_NS` field.
    pub const HAS_FORCE_NS_SHIFT: u32 = 60;
    /// Offset of the `HAS_SDEFLT` field.
    pub const HAS_SDEFLT_SHIFT: u32 = 61;

    /// Returns the value of the `PARTID_MAX` field.
    pub const fn partid_max(self) -> u16 {
        ((self.bits() >> Self::PARTID_MAX_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PARTID_MAX` field.
    pub const fn set_partid_max(&mut self, value: u16) {
        let offset = Self::PARTID_MAX_SHIFT;
        assert!(value & (Self::PARTID_MAX_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTID_MAX_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VPMR_MAX` field.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn vpmr_max(self) -> u8 {
        ((self.bits() >> Self::VPMR_MAX_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `VPMR_MAX` field.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn set_vpmr_max(&mut self, value: u8) {
        let offset = Self::VPMR_MAX_SHIFT;
        assert!(value & (Self::VPMR_MAX_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VPMR_MAX_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm0El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm0El2 {
    /// Offset of the `PhyPARTID0` field.
    pub const PHYPARTID0_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID0` field.
    pub const PHYPARTID0_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID1` field.
    pub const PHYPARTID1_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID1` field.
    pub const PHYPARTID1_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID2` field.
    pub const PHYPARTID2_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID2` field.
    pub const PHYPARTID2_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID3` field.
    pub const PHYPARTID3_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID3` field.
    pub const PHYPARTID3_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID0` field.
    pub const fn phypartid0(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID0_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID0` field.
    pub const fn set_phypartid0(&mut self, value: u16) {
        let offset = Self::PHYPARTID0_SHIFT;
        assert!(value & (Self::PHYPARTID0_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID1` field.
    pub const fn phypartid1(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID1_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID1` field.
    pub const fn set_phypartid1(&mut self, value: u16) {
        let offset = Self::PHYPARTID1_SHIFT;
        assert!(value & (Self::PHYPARTID1_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID2` field.
    pub const fn phypartid2(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID2_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID2` field.
    pub const fn set_phypartid2(&mut self, value: u16) {
        let offset = Self::PHYPARTID2_SHIFT;
        assert!(value & (Self::PHYPARTID2_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID3` field.
    pub const fn phypartid3(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID3_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID3` field.
    pub const fn set_phypartid3(&mut self, value: u16) {
        let offset = Self::PHYPARTID3_SHIFT;
        assert!(value & (Self::PHYPARTID3_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID3_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm1El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm1El2 {
    /// Offset of the `PhyPARTID4` field.
    pub const PHYPARTID4_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID4` field.
    pub const PHYPARTID4_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID5` field.
    pub const PHYPARTID5_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID5` field.
    pub const PHYPARTID5_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID6` field.
    pub const PHYPARTID6_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID6` field.
    pub const PHYPARTID6_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID7` field.
    pub const PHYPARTID7_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID7` field.
    pub const PHYPARTID7_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID4` field.
    pub const fn phypartid4(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID4_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID4` field.
    pub const fn set_phypartid4(&mut self, value: u16) {
        let offset = Self::PHYPARTID4_SHIFT;
        assert!(value & (Self::PHYPARTID4_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID4_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID5` field.
    pub const fn phypartid5(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID5_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID5` field.
    pub const fn set_phypartid5(&mut self, value: u16) {
        let offset = Self::PHYPARTID5_SHIFT;
        assert!(value & (Self::PHYPARTID5_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID5_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID6` field.
    pub const fn phypartid6(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID6_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID6` field.
    pub const fn set_phypartid6(&mut self, value: u16) {
        let offset = Self::PHYPARTID6_SHIFT;
        assert!(value & (Self::PHYPARTID6_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID6_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID7` field.
    pub const fn phypartid7(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID7_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID7` field.
    pub const fn set_phypartid7(&mut self, value: u16) {
        let offset = Self::PHYPARTID7_SHIFT;
        assert!(value & (Self::PHYPARTID7_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID7_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm2El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm2El2 {
    /// Offset of the `PhyPARTID8` field.
    pub const PHYPARTID8_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID8` field.
    pub const PHYPARTID8_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID9` field.
    pub const PHYPARTID9_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID9` field.
    pub const PHYPARTID9_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID10` field.
    pub const PHYPARTID10_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID10` field.
    pub const PHYPARTID10_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID11` field.
    pub const PHYPARTID11_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID11` field.
    pub const PHYPARTID11_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID8` field.
    pub const fn phypartid8(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID8_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID8` field.
    pub const fn set_phypartid8(&mut self, value: u16) {
        let offset = Self::PHYPARTID8_SHIFT;
        assert!(value & (Self::PHYPARTID8_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID8_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID9` field.
    pub const fn phypartid9(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID9_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID9` field.
    pub const fn set_phypartid9(&mut self, value: u16) {
        let offset = Self::PHYPARTID9_SHIFT;
        assert!(value & (Self::PHYPARTID9_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID9_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID10` field.
    pub const fn phypartid10(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID10_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID10` field.
    pub const fn set_phypartid10(&mut self, value: u16) {
        let offset = Self::PHYPARTID10_SHIFT;
        assert!(value & (Self::PHYPARTID10_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID10_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID11` field.
    pub const fn phypartid11(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID11_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID11` field.
    pub const fn set_phypartid11(&mut self, value: u16) {
        let offset = Self::PHYPARTID11_SHIFT;
        assert!(value & (Self::PHYPARTID11_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID11_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM3_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm3El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm3El2 {
    /// Offset of the `PhyPARTID12` field.
    pub const PHYPARTID12_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID12` field.
    pub const PHYPARTID12_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID13` field.
    pub const PHYPARTID13_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID13` field.
    pub const PHYPARTID13_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID14` field.
    pub const PHYPARTID14_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID14` field.
    pub const PHYPARTID14_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID15` field.
    pub const PHYPARTID15_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID15` field.
    pub const PHYPARTID15_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID12` field.
    pub const fn phypartid12(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID12_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID12` field.
    pub const fn set_phypartid12(&mut self, value: u16) {
        let offset = Self::PHYPARTID12_SHIFT;
        assert!(value & (Self::PHYPARTID12_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID12_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID13` field.
    pub const fn phypartid13(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID13_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID13` field.
    pub const fn set_phypartid13(&mut self, value: u16) {
        let offset = Self::PHYPARTID13_SHIFT;
        assert!(value & (Self::PHYPARTID13_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID13_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID14` field.
    pub const fn phypartid14(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID14_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID14` field.
    pub const fn set_phypartid14(&mut self, value: u16) {
        let offset = Self::PHYPARTID14_SHIFT;
        assert!(value & (Self::PHYPARTID14_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID14_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID15` field.
    pub const fn phypartid15(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID15_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID15` field.
    pub const fn set_phypartid15(&mut self, value: u16) {
        let offset = Self::PHYPARTID15_SHIFT;
        assert!(value & (Self::PHYPARTID15_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID15_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM4_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm4El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm4El2 {
    /// Offset of the `PhyPARTID16` field.
    pub const PHYPARTID16_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID16` field.
    pub const PHYPARTID16_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID17` field.
    pub const PHYPARTID17_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID17` field.
    pub const PHYPARTID17_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID18` field.
    pub const PHYPARTID18_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID18` field.
    pub const PHYPARTID18_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID19` field.
    pub const PHYPARTID19_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID19` field.
    pub const PHYPARTID19_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID16` field.
    pub const fn phypartid16(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID16_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID16` field.
    pub const fn set_phypartid16(&mut self, value: u16) {
        let offset = Self::PHYPARTID16_SHIFT;
        assert!(value & (Self::PHYPARTID16_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID16_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID17` field.
    pub const fn phypartid17(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID17_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID17` field.
    pub const fn set_phypartid17(&mut self, value: u16) {
        let offset = Self::PHYPARTID17_SHIFT;
        assert!(value & (Self::PHYPARTID17_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID17_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID18` field.
    pub const fn phypartid18(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID18_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID18` field.
    pub const fn set_phypartid18(&mut self, value: u16) {
        let offset = Self::PHYPARTID18_SHIFT;
        assert!(value & (Self::PHYPARTID18_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID18_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID19` field.
    pub const fn phypartid19(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID19_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID19` field.
    pub const fn set_phypartid19(&mut self, value: u16) {
        let offset = Self::PHYPARTID19_SHIFT;
        assert!(value & (Self::PHYPARTID19_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID19_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM5_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm5El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm5El2 {
    /// Offset of the `PhyPARTID20` field.
    pub const PHYPARTID20_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID20` field.
    pub const PHYPARTID20_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID21` field.
    pub const PHYPARTID21_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID21` field.
    pub const PHYPARTID21_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID22` field.
    pub const PHYPARTID22_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID22` field.
    pub const PHYPARTID22_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID23` field.
    pub const PHYPARTID23_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID23` field.
    pub const PHYPARTID23_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID20` field.
    pub const fn phypartid20(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID20_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID20` field.
    pub const fn set_phypartid20(&mut self, value: u16) {
        let offset = Self::PHYPARTID20_SHIFT;
        assert!(value & (Self::PHYPARTID20_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID20_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID21` field.
    pub const fn phypartid21(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID21_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID21` field.
    pub const fn set_phypartid21(&mut self, value: u16) {
        let offset = Self::PHYPARTID21_SHIFT;
        assert!(value & (Self::PHYPARTID21_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID21_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID22` field.
    pub const fn phypartid22(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID22_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID22` field.
    pub const fn set_phypartid22(&mut self, value: u16) {
        let offset = Self::PHYPARTID22_SHIFT;
        assert!(value & (Self::PHYPARTID22_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID22_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID23` field.
    pub const fn phypartid23(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID23_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID23` field.
    pub const fn set_phypartid23(&mut self, value: u16) {
        let offset = Self::PHYPARTID23_SHIFT;
        assert!(value & (Self::PHYPARTID23_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID23_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM6_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm6El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm6El2 {
    /// Offset of the `PhyPARTID24` field.
    pub const PHYPARTID24_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID24` field.
    pub const PHYPARTID24_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID25` field.
    pub const PHYPARTID25_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID25` field.
    pub const PHYPARTID25_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID26` field.
    pub const PHYPARTID26_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID26` field.
    pub const PHYPARTID26_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID27` field.
    pub const PHYPARTID27_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID27` field.
    pub const PHYPARTID27_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID24` field.
    pub const fn phypartid24(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID24_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID24` field.
    pub const fn set_phypartid24(&mut self, value: u16) {
        let offset = Self::PHYPARTID24_SHIFT;
        assert!(value & (Self::PHYPARTID24_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID24_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID25` field.
    pub const fn phypartid25(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID25_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID25` field.
    pub const fn set_phypartid25(&mut self, value: u16) {
        let offset = Self::PHYPARTID25_SHIFT;
        assert!(value & (Self::PHYPARTID25_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID25_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID26` field.
    pub const fn phypartid26(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID26_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID26` field.
    pub const fn set_phypartid26(&mut self, value: u16) {
        let offset = Self::PHYPARTID26_SHIFT;
        assert!(value & (Self::PHYPARTID26_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID26_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID27` field.
    pub const fn phypartid27(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID27_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID27` field.
    pub const fn set_phypartid27(&mut self, value: u16) {
        let offset = Self::PHYPARTID27_SHIFT;
        assert!(value & (Self::PHYPARTID27_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID27_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVPM7_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpamvpm7El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mpamvpm7El2 {
    /// Offset of the `PhyPARTID28` field.
    pub const PHYPARTID28_SHIFT: u32 = 0;
    /// Mask for the `PhyPARTID28` field.
    pub const PHYPARTID28_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID29` field.
    pub const PHYPARTID29_SHIFT: u32 = 16;
    /// Mask for the `PhyPARTID29` field.
    pub const PHYPARTID29_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID30` field.
    pub const PHYPARTID30_SHIFT: u32 = 32;
    /// Mask for the `PhyPARTID30` field.
    pub const PHYPARTID30_MASK: u64 = 0b1111111111111111;
    /// Offset of the `PhyPARTID31` field.
    pub const PHYPARTID31_SHIFT: u32 = 48;
    /// Mask for the `PhyPARTID31` field.
    pub const PHYPARTID31_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `PhyPARTID28` field.
    pub const fn phypartid28(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID28_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID28` field.
    pub const fn set_phypartid28(&mut self, value: u16) {
        let offset = Self::PHYPARTID28_SHIFT;
        assert!(value & (Self::PHYPARTID28_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID28_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID29` field.
    pub const fn phypartid29(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID29_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID29` field.
    pub const fn set_phypartid29(&mut self, value: u16) {
        let offset = Self::PHYPARTID29_SHIFT;
        assert!(value & (Self::PHYPARTID29_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID29_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID30` field.
    pub const fn phypartid30(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID30_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID30` field.
    pub const fn set_phypartid30(&mut self, value: u16) {
        let offset = Self::PHYPARTID30_SHIFT;
        assert!(value & (Self::PHYPARTID30_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID30_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PhyPARTID31` field.
    pub const fn phypartid31(self) -> u16 {
        ((self.bits() >> Self::PHYPARTID31_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `PhyPARTID31` field.
    pub const fn set_phypartid31(&mut self, value: u16) {
        let offset = Self::PHYPARTID31_SHIFT;
        assert!(value & (Self::PHYPARTID31_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PHYPARTID31_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl MpamvpmvEl2 {
    /// Offset of the `VPM_V<m>` field.
    pub const VPM_V_SHIFT: u32 = 0;
}

bitflags! {
    /// `MPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpidr: u32 {
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
        /// `M` bit.
        const M = 1 << 31;
    }
}

impl Mpidr {
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u32 = 0b11111111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u32 = 0b11111111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u32 = 0b11111111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 31;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> Self::AFF1_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff1` field.
    pub const fn set_aff1(&mut self, value: u8) {
        let offset = Self::AFF1_SHIFT;
        assert!(value & (Self::AFF1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> Self::AFF2_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff2` field.
    pub const fn set_aff2(&mut self, value: u8) {
        let offset = Self::AFF2_SHIFT;
        assert!(value & (Self::AFF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u64 = 0b11111111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u64 = 0b11111111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u64 = 0b11111111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `Aff3` field.
    pub const AFF3_SHIFT: u32 = 32;
    /// Mask for the `Aff3` field.
    pub const AFF3_MASK: u64 = 0b11111111;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> Self::AFF1_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff1` field.
    pub const fn set_aff1(&mut self, value: u8) {
        let offset = Self::AFF1_SHIFT;
        assert!(value & (Self::AFF1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> Self::AFF2_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff2` field.
    pub const fn set_aff2(&mut self, value: u8) {
        let offset = Self::AFF2_SHIFT;
        assert!(value & (Self::AFF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        ((self.bits() >> Self::AFF3_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff3` field.
    pub const fn set_aff3(&mut self, value: u8) {
        let offset = Self::AFF3_SHIFT;
        assert!(value & (Self::AFF3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF3_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `MVBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mvbar: u32 {
    }
}

impl Mvbar {
    /// Offset of the `Reserved` field.
    pub const RESERVED_SHIFT: u32 = 0;
    /// Mask for the `Reserved` field.
    pub const RESERVED_MASK: u32 = 0b11111;
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 5;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u32 = 0b111111111111111111111111111;

    /// Returns the value of the `Reserved` field.
    pub const fn reserved(self) -> u8 {
        ((self.bits() >> Self::RESERVED_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `Reserved` field.
    pub const fn set_reserved(&mut self, value: u8) {
        let offset = Self::RESERVED_SHIFT;
        assert!(value & (Self::RESERVED_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RESERVED_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        ((self.bits() >> Self::VBA_SHIFT) & 0b111111111111111111111111111) as u32
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u32) {
        let offset = Self::VBA_SHIFT;
        assert!(value & (Self::VBA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `NMRR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Nmrr: u32 {
    }
}

impl Nmrr {
    /// Offset of the `IR<n>` field.
    pub const IR_SHIFT: u32 = 0;
    /// Mask for the `IR<n>` field.
    pub const IR_MASK: u32 = 0b11;
    /// Offset of the `OR<n>` field.
    pub const OR_SHIFT: u32 = 16;
    /// Mask for the `OR<n>` field.
    pub const OR_MASK: u32 = 0b11;

    /// Returns the value of the given `IR<n>` field.
    pub const fn ir(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::IR_SHIFT + (n - 0) * 2)) & 0b11) as u8
    }

    /// Sets the value of the `IR<n>` field.
    pub const fn set_ir(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::IR_SHIFT + (n - 0) * 2;
        assert!(value & (Self::IR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IR_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the given `OR<n>` field.
    pub const fn or(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::OR_SHIFT + (n - 0) * 2)) & 0b11) as u8
    }

    /// Sets the value of the `OR<n>` field.
    pub const fn set_or(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::OR_SHIFT + (n - 0) * 2;
        assert!(value & (Self::OR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `NSACR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Nsacr: u32 {
        /// `cp10` bit.
        const CP10 = 1 << 10;
        /// `cp11` bit.
        const CP11 = 1 << 11;
        /// `NSASEDIS` bit.
        const NSASEDIS = 1 << 15;
        /// `NSTRCDIS` bit.
        const NSTRCDIS = 1 << 20;
    }
}

impl Nsacr {
    /// Offset of the `cp10` field.
    pub const CP10_SHIFT: u32 = 10;
    /// Offset of the `cp11` field.
    pub const CP11_SHIFT: u32 = 11;
    /// Offset of the `NSASEDIS` field.
    pub const NSASEDIS_SHIFT: u32 = 15;
    /// Offset of the `NSTRCDIS` field.
    pub const NSTRCDIS_SHIFT: u32 = 20;
}

bitflags! {
    /// `PAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Par: u64 {
        /// `F` bit.
        const F = 1 << 0;
        /// `SS` bit.
        const SS = 1 << 1;
        /// `FS[5]` bit.
        const FS_5 = 1 << 6;
        /// `S2WLK` bit.
        const S2WLK = 1 << 8;
        /// `FSTAGE` bit.
        const FSTAGE = 1 << 9;
        /// `NS` bit.
        const NS = 1 << 9;
        /// `NOS` bit.
        const NOS = 1 << 10;
        /// `LPAE` bit.
        const LPAE = 1 << 11;
    }
}

impl Par {
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 0;
    /// Offset of the `FST` field.
    pub const FST_SHIFT: u32 = 1;
    /// Mask for the `FST` field.
    pub const FST_MASK: u64 = 0b111111;
    /// Offset of the `FS[4:0]` field.
    pub const FS_4_0_SHIFT: u32 = 1;
    /// Mask for the `FS[4:0]` field.
    pub const FS_4_0_MASK: u64 = 0b11111;
    /// Offset of the `SS` field.
    pub const SS_SHIFT: u32 = 1;
    /// Offset of the `Outer[1:0]` field.
    pub const OUTER_1_0_SHIFT: u32 = 2;
    /// Mask for the `Outer[1:0]` field.
    pub const OUTER_1_0_MASK: u64 = 0b11;
    /// Offset of the `Inner[2:0]` field.
    pub const INNER_2_0_SHIFT: u32 = 4;
    /// Mask for the `Inner[2:0]` field.
    pub const INNER_2_0_MASK: u64 = 0b111;
    /// Offset of the `FS[5]` field.
    pub const FS_5_SHIFT: u32 = 6;
    /// Offset of the `S2WLK` field.
    pub const S2WLK_SHIFT: u32 = 8;
    /// Offset of the `FSTAGE` field.
    pub const FSTAGE_SHIFT: u32 = 9;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 9;
    /// Offset of the `NOS` field.
    pub const NOS_SHIFT: u32 = 10;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 11;
    /// Offset of the `ATTR` field.
    pub const ATTR_SHIFT: u32 = 56;
    /// Mask for the `ATTR` field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the `FST` field.
    pub const fn fst(self) -> u8 {
        ((self.bits() >> Self::FST_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `FST` field.
    pub const fn set_fst(&mut self, value: u8) {
        let offset = Self::FST_SHIFT;
        assert!(value & (Self::FST_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FST_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `FS[4:0]` field.
    pub const fn fs_4_0(self) -> u8 {
        ((self.bits() >> Self::FS_4_0_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `FS[4:0]` field.
    pub const fn set_fs_4_0(&mut self, value: u8) {
        let offset = Self::FS_4_0_SHIFT;
        assert!(value & (Self::FS_4_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FS_4_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Outer[1:0]` field.
    pub const fn outer_1_0(self) -> u8 {
        ((self.bits() >> Self::OUTER_1_0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `Outer[1:0]` field.
    pub const fn set_outer_1_0(&mut self, value: u8) {
        let offset = Self::OUTER_1_0_SHIFT;
        assert!(value & (Self::OUTER_1_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::OUTER_1_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Inner[2:0]` field.
    pub const fn inner_2_0(self) -> u8 {
        ((self.bits() >> Self::INNER_2_0_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `Inner[2:0]` field.
    pub const fn set_inner_2_0(&mut self, value: u8) {
        let offset = Self::INNER_2_0_SHIFT;
        assert!(value & (Self::INNER_2_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::INNER_2_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ATTR` field.
    pub const fn attr(self) -> u8 {
        ((self.bits() >> Self::ATTR_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `ATTR` field.
    pub const fn set_attr(&mut self, value: u8) {
        let offset = Self::ATTR_SHIFT;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `F` field.
    pub const F_SHIFT: u32 = 0;
    /// Offset of the `FST` field.
    pub const FST_SHIFT: u32 = 1;
    /// Mask for the `FST` field.
    pub const FST_MASK: u64 = 0b111111;
    /// Offset of the `SH` field.
    pub const SH_SHIFT: u32 = 7;
    /// Mask for the `SH` field.
    pub const SH_MASK: u64 = 0b11;
    /// Offset of the `PTW` field.
    pub const PTW_SHIFT: u32 = 8;
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 9;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 9;
    /// Offset of the `NSE` field.
    pub const NSE_SHIFT: u32 = 11;
    /// Offset of the `AssuredOnly` field.
    pub const ASSUREDONLY_SHIFT: u32 = 12;
    /// Offset of the `PA[47:12]` field.
    pub const PA_47_12_SHIFT: u32 = 12;
    /// Mask for the `PA[47:12]` field.
    pub const PA_47_12_MASK: u64 = 0b111111111111111111111111111111111111;
    /// Offset of the `TopLevel` field.
    pub const TOPLEVEL_SHIFT: u32 = 13;
    /// Offset of the `Overlay` field.
    pub const OVERLAY_SHIFT: u32 = 14;
    /// Offset of the `DirtyBit` field.
    pub const DIRTYBIT_SHIFT: u32 = 15;
    /// Offset of the `PA[51:48]` field.
    pub const PA_51_48_SHIFT: u32 = 48;
    /// Mask for the `PA[51:48]` field.
    pub const PA_51_48_MASK: u64 = 0b1111;
    /// Offset of the `ATTR` field.
    pub const ATTR_SHIFT: u32 = 56;
    /// Mask for the `ATTR` field.
    pub const ATTR_MASK: u64 = 0b11111111;

    /// Returns the value of the `FST` field.
    pub const fn fst(self) -> u8 {
        ((self.bits() >> Self::FST_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `FST` field.
    pub const fn set_fst(&mut self, value: u8) {
        let offset = Self::FST_SHIFT;
        assert!(value & (Self::FST_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::FST_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        ((self.bits() >> Self::SH_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH` field.
    pub const fn set_sh(&mut self, value: u8) {
        let offset = Self::SH_SHIFT;
        assert!(value & (Self::SH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PA[47:12]` field.
    pub const fn pa_47_12(self) -> u64 {
        ((self.bits() >> Self::PA_47_12_SHIFT) & 0b111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `PA[47:12]` field.
    pub const fn set_pa_47_12(&mut self, value: u64) {
        let offset = Self::PA_47_12_SHIFT;
        assert!(value & (Self::PA_47_12_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_47_12_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        ((self.bits() >> Self::PA_51_48_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `PA[51:48]` field.
    pub const fn set_pa_51_48(&mut self, value: u8) {
        let offset = Self::PA_51_48_SHIFT;
        assert!(value & (Self::PA_51_48_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PA_51_48_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ATTR` field.
    pub const fn attr(self) -> u8 {
        ((self.bits() >> Self::ATTR_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `ATTR` field.
    pub const fn set_attr(&mut self, value: u8) {
        let offset = Self::ATTR_SHIFT;
        assert!(value & (Self::ATTR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ATTR_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `PMCCFILTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmccfiltr: u32 {
        /// `RLU` bit.
        const RLU = 1 << 21;
        /// `NSH` bit.
        const NSH = 1 << 27;
        /// `NSU` bit.
        const NSU = 1 << 28;
        /// `NSK` bit.
        const NSK = 1 << 29;
        /// `U` bit.
        const U = 1 << 30;
        /// `P` bit.
        const P = 1 << 31;
    }
}

impl Pmccfiltr {
    /// Offset of the `RLU` field.
    pub const RLU_SHIFT: u32 = 21;
    /// Offset of the `NSH` field.
    pub const NSH_SHIFT: u32 = 27;
    /// Offset of the `NSU` field.
    pub const NSU_SHIFT: u32 = 28;
    /// Offset of the `NSK` field.
    pub const NSK_SHIFT: u32 = 29;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `P` field.
    pub const P_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMCCNTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmccntr: u64 {
    }
}

impl Pmccntr {
    /// Offset of the `CCNT` field.
    pub const CCNT_SHIFT: u32 = 0;
    /// Mask for the `CCNT` field.
    pub const CCNT_MASK: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `CCNT` field.
    pub const fn ccnt(self) -> u64 {
        ((self.bits() >> Self::CCNT_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `CCNT` field.
    pub const fn set_ccnt(&mut self, value: u64) {
        let offset = Self::CCNT_SHIFT;
        assert!(value & (Self::CCNT_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::CCNT_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `PMCEID0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid0: u32 {
        /// `ID<n>` bit 0.
        const ID0 = 1 << 0;
        /// `ID<n>` bit 1.
        const ID1 = 1 << 1;
        /// `ID<n>` bit 2.
        const ID2 = 1 << 2;
        /// `ID<n>` bit 3.
        const ID3 = 1 << 3;
        /// `ID<n>` bit 4.
        const ID4 = 1 << 4;
        /// `ID<n>` bit 5.
        const ID5 = 1 << 5;
        /// `ID<n>` bit 6.
        const ID6 = 1 << 6;
        /// `ID<n>` bit 7.
        const ID7 = 1 << 7;
        /// `ID<n>` bit 8.
        const ID8 = 1 << 8;
        /// `ID<n>` bit 9.
        const ID9 = 1 << 9;
        /// `ID<n>` bit 10.
        const ID10 = 1 << 10;
        /// `ID<n>` bit 11.
        const ID11 = 1 << 11;
        /// `ID<n>` bit 12.
        const ID12 = 1 << 12;
        /// `ID<n>` bit 13.
        const ID13 = 1 << 13;
        /// `ID<n>` bit 14.
        const ID14 = 1 << 14;
        /// `ID<n>` bit 15.
        const ID15 = 1 << 15;
        /// `ID<n>` bit 16.
        const ID16 = 1 << 16;
        /// `ID<n>` bit 17.
        const ID17 = 1 << 17;
        /// `ID<n>` bit 18.
        const ID18 = 1 << 18;
        /// `ID<n>` bit 19.
        const ID19 = 1 << 19;
        /// `ID<n>` bit 20.
        const ID20 = 1 << 20;
        /// `ID<n>` bit 21.
        const ID21 = 1 << 21;
        /// `ID<n>` bit 22.
        const ID22 = 1 << 22;
        /// `ID<n>` bit 23.
        const ID23 = 1 << 23;
        /// `ID<n>` bit 24.
        const ID24 = 1 << 24;
        /// `ID<n>` bit 25.
        const ID25 = 1 << 25;
        /// `ID<n>` bit 26.
        const ID26 = 1 << 26;
        /// `ID<n>` bit 27.
        const ID27 = 1 << 27;
        /// `ID<n>` bit 28.
        const ID28 = 1 << 28;
        /// `ID<n>` bit 29.
        const ID29 = 1 << 29;
        /// `ID<n>` bit 30.
        const ID30 = 1 << 30;
        /// `ID<n>` bit 31.
        const ID31 = 1 << 31;
    }
}

impl Pmceid0 {
    /// Offset of the `ID<n>` field.
    pub const ID_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCEID1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid1: u32 {
        /// `ID<n>` bit 0.
        const ID0 = 1 << 0;
        /// `ID<n>` bit 1.
        const ID1 = 1 << 1;
        /// `ID<n>` bit 2.
        const ID2 = 1 << 2;
        /// `ID<n>` bit 3.
        const ID3 = 1 << 3;
        /// `ID<n>` bit 4.
        const ID4 = 1 << 4;
        /// `ID<n>` bit 5.
        const ID5 = 1 << 5;
        /// `ID<n>` bit 6.
        const ID6 = 1 << 6;
        /// `ID<n>` bit 7.
        const ID7 = 1 << 7;
        /// `ID<n>` bit 8.
        const ID8 = 1 << 8;
        /// `ID<n>` bit 9.
        const ID9 = 1 << 9;
        /// `ID<n>` bit 10.
        const ID10 = 1 << 10;
        /// `ID<n>` bit 11.
        const ID11 = 1 << 11;
        /// `ID<n>` bit 12.
        const ID12 = 1 << 12;
        /// `ID<n>` bit 13.
        const ID13 = 1 << 13;
        /// `ID<n>` bit 14.
        const ID14 = 1 << 14;
        /// `ID<n>` bit 15.
        const ID15 = 1 << 15;
        /// `ID<n>` bit 16.
        const ID16 = 1 << 16;
        /// `ID<n>` bit 17.
        const ID17 = 1 << 17;
        /// `ID<n>` bit 18.
        const ID18 = 1 << 18;
        /// `ID<n>` bit 19.
        const ID19 = 1 << 19;
        /// `ID<n>` bit 20.
        const ID20 = 1 << 20;
        /// `ID<n>` bit 21.
        const ID21 = 1 << 21;
        /// `ID<n>` bit 22.
        const ID22 = 1 << 22;
        /// `ID<n>` bit 23.
        const ID23 = 1 << 23;
        /// `ID<n>` bit 24.
        const ID24 = 1 << 24;
        /// `ID<n>` bit 25.
        const ID25 = 1 << 25;
        /// `ID<n>` bit 26.
        const ID26 = 1 << 26;
        /// `ID<n>` bit 27.
        const ID27 = 1 << 27;
        /// `ID<n>` bit 28.
        const ID28 = 1 << 28;
        /// `ID<n>` bit 29.
        const ID29 = 1 << 29;
        /// `ID<n>` bit 30.
        const ID30 = 1 << 30;
        /// `ID<n>` bit 31.
        const ID31 = 1 << 31;
    }
}

impl Pmceid1 {
    /// Offset of the `ID<n>` field.
    pub const ID_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCEID2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid2: u32 {
        /// `IDhi<n>` bit 0.
        const IDHI0 = 1 << 0;
        /// `IDhi<n>` bit 1.
        const IDHI1 = 1 << 1;
        /// `IDhi<n>` bit 2.
        const IDHI2 = 1 << 2;
        /// `IDhi<n>` bit 3.
        const IDHI3 = 1 << 3;
        /// `IDhi<n>` bit 4.
        const IDHI4 = 1 << 4;
        /// `IDhi<n>` bit 5.
        const IDHI5 = 1 << 5;
        /// `IDhi<n>` bit 6.
        const IDHI6 = 1 << 6;
        /// `IDhi<n>` bit 7.
        const IDHI7 = 1 << 7;
        /// `IDhi<n>` bit 8.
        const IDHI8 = 1 << 8;
        /// `IDhi<n>` bit 9.
        const IDHI9 = 1 << 9;
        /// `IDhi<n>` bit 10.
        const IDHI10 = 1 << 10;
        /// `IDhi<n>` bit 11.
        const IDHI11 = 1 << 11;
        /// `IDhi<n>` bit 12.
        const IDHI12 = 1 << 12;
        /// `IDhi<n>` bit 13.
        const IDHI13 = 1 << 13;
        /// `IDhi<n>` bit 14.
        const IDHI14 = 1 << 14;
        /// `IDhi<n>` bit 15.
        const IDHI15 = 1 << 15;
        /// `IDhi<n>` bit 16.
        const IDHI16 = 1 << 16;
        /// `IDhi<n>` bit 17.
        const IDHI17 = 1 << 17;
        /// `IDhi<n>` bit 18.
        const IDHI18 = 1 << 18;
        /// `IDhi<n>` bit 19.
        const IDHI19 = 1 << 19;
        /// `IDhi<n>` bit 20.
        const IDHI20 = 1 << 20;
        /// `IDhi<n>` bit 21.
        const IDHI21 = 1 << 21;
        /// `IDhi<n>` bit 22.
        const IDHI22 = 1 << 22;
        /// `IDhi<n>` bit 23.
        const IDHI23 = 1 << 23;
        /// `IDhi<n>` bit 24.
        const IDHI24 = 1 << 24;
        /// `IDhi<n>` bit 25.
        const IDHI25 = 1 << 25;
        /// `IDhi<n>` bit 26.
        const IDHI26 = 1 << 26;
        /// `IDhi<n>` bit 27.
        const IDHI27 = 1 << 27;
        /// `IDhi<n>` bit 28.
        const IDHI28 = 1 << 28;
        /// `IDhi<n>` bit 29.
        const IDHI29 = 1 << 29;
        /// `IDhi<n>` bit 30.
        const IDHI30 = 1 << 30;
        /// `IDhi<n>` bit 31.
        const IDHI31 = 1 << 31;
    }
}

impl Pmceid2 {
    /// Offset of the `IDhi<n>` field.
    pub const IDHI_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCEID3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid3: u32 {
        /// `IDhi<n>` bit 0.
        const IDHI0 = 1 << 0;
        /// `IDhi<n>` bit 1.
        const IDHI1 = 1 << 1;
        /// `IDhi<n>` bit 2.
        const IDHI2 = 1 << 2;
        /// `IDhi<n>` bit 3.
        const IDHI3 = 1 << 3;
        /// `IDhi<n>` bit 4.
        const IDHI4 = 1 << 4;
        /// `IDhi<n>` bit 5.
        const IDHI5 = 1 << 5;
        /// `IDhi<n>` bit 6.
        const IDHI6 = 1 << 6;
        /// `IDhi<n>` bit 7.
        const IDHI7 = 1 << 7;
        /// `IDhi<n>` bit 8.
        const IDHI8 = 1 << 8;
        /// `IDhi<n>` bit 9.
        const IDHI9 = 1 << 9;
        /// `IDhi<n>` bit 10.
        const IDHI10 = 1 << 10;
        /// `IDhi<n>` bit 11.
        const IDHI11 = 1 << 11;
        /// `IDhi<n>` bit 12.
        const IDHI12 = 1 << 12;
        /// `IDhi<n>` bit 13.
        const IDHI13 = 1 << 13;
        /// `IDhi<n>` bit 14.
        const IDHI14 = 1 << 14;
        /// `IDhi<n>` bit 15.
        const IDHI15 = 1 << 15;
        /// `IDhi<n>` bit 16.
        const IDHI16 = 1 << 16;
        /// `IDhi<n>` bit 17.
        const IDHI17 = 1 << 17;
        /// `IDhi<n>` bit 18.
        const IDHI18 = 1 << 18;
        /// `IDhi<n>` bit 19.
        const IDHI19 = 1 << 19;
        /// `IDhi<n>` bit 20.
        const IDHI20 = 1 << 20;
        /// `IDhi<n>` bit 21.
        const IDHI21 = 1 << 21;
        /// `IDhi<n>` bit 22.
        const IDHI22 = 1 << 22;
        /// `IDhi<n>` bit 23.
        const IDHI23 = 1 << 23;
        /// `IDhi<n>` bit 24.
        const IDHI24 = 1 << 24;
        /// `IDhi<n>` bit 25.
        const IDHI25 = 1 << 25;
        /// `IDhi<n>` bit 26.
        const IDHI26 = 1 << 26;
        /// `IDhi<n>` bit 27.
        const IDHI27 = 1 << 27;
        /// `IDhi<n>` bit 28.
        const IDHI28 = 1 << 28;
        /// `IDhi<n>` bit 29.
        const IDHI29 = 1 << 29;
        /// `IDhi<n>` bit 30.
        const IDHI30 = 1 << 30;
        /// `IDhi<n>` bit 31.
        const IDHI31 = 1 << 31;
    }
}

impl Pmceid3 {
    /// Offset of the `IDhi<n>` field.
    pub const IDHI_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMCNTENCLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmcntenclr: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmcntenclr {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMCNTENSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmcntenset: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmcntenset {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmcr: u32 {
        /// `E` bit.
        const E = 1 << 0;
        /// `P` bit.
        const P = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `D` bit.
        const D = 1 << 3;
        /// `X` bit.
        const X = 1 << 4;
        /// `DP` bit.
        const DP = 1 << 5;
        /// `LC` bit.
        const LC = 1 << 6;
        /// `LP` bit.
        const LP = 1 << 7;
        /// `FZO` bit.
        const FZO = 1 << 9;
    }
}

impl Pmcr {
    /// Offset of the `E` field.
    pub const E_SHIFT: u32 = 0;
    /// Offset of the `P` field.
    pub const P_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 3;
    /// Offset of the `X` field.
    pub const X_SHIFT: u32 = 4;
    /// Offset of the `DP` field.
    pub const DP_SHIFT: u32 = 5;
    /// Offset of the `LC` field.
    pub const LC_SHIFT: u32 = 6;
    /// Offset of the `LP` field.
    pub const LP_SHIFT: u32 = 7;
    /// Offset of the `FZO` field.
    pub const FZO_SHIFT: u32 = 9;
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 11;
    /// Mask for the `N` field.
    pub const N_MASK: u32 = 0b11111;
    /// Offset of the `IDCODE` field.
    pub const IDCODE_SHIFT: u32 = 16;
    /// Mask for the `IDCODE` field.
    pub const IDCODE_MASK: u32 = 0b11111111;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 24;
    /// Mask for the `IMP` field.
    pub const IMP_MASK: u32 = 0b11111111;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `IDCODE` field.
    pub const fn idcode(self) -> u8 {
        ((self.bits() >> Self::IDCODE_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `IDCODE` field.
    pub const fn set_idcode(&mut self, value: u8) {
        let offset = Self::IDCODE_SHIFT;
        assert!(value & (Self::IDCODE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IDCODE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `IMP` field.
    pub const fn imp(self) -> u8 {
        ((self.bits() >> Self::IMP_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `IMP` field.
    pub const fn set_imp(&mut self, value: u8) {
        let offset = Self::IMP_SHIFT;
        assert!(value & (Self::IMP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMP_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `PMCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `E` field.
    pub const E_SHIFT: u32 = 0;
    /// Offset of the `P` field.
    pub const P_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `D` field.
    pub const D_SHIFT: u32 = 3;
    /// Offset of the `X` field.
    pub const X_SHIFT: u32 = 4;
    /// Offset of the `DP` field.
    pub const DP_SHIFT: u32 = 5;
    /// Offset of the `LC` field.
    pub const LC_SHIFT: u32 = 6;
    /// Offset of the `LP` field.
    pub const LP_SHIFT: u32 = 7;
    /// Offset of the `FZO` field.
    pub const FZO_SHIFT: u32 = 9;
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 11;
    /// Mask for the `N` field.
    pub const N_MASK: u64 = 0b11111;
    /// Offset of the `IDCODE` field.
    pub const IDCODE_SHIFT: u32 = 16;
    /// Mask for the `IDCODE` field.
    pub const IDCODE_MASK: u64 = 0b11111111;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 24;
    /// Mask for the `IMP` field.
    pub const IMP_MASK: u64 = 0b11111111;
    /// Offset of the `FZS` field.
    pub const FZS_SHIFT: u32 = 32;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IDCODE` field.
    pub const fn idcode(self) -> u8 {
        ((self.bits() >> Self::IDCODE_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `IDCODE` field.
    pub const fn set_idcode(&mut self, value: u8) {
        let offset = Self::IDCODE_SHIFT;
        assert!(value & (Self::IDCODE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IDCODE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IMP` field.
    pub const fn imp(self) -> u8 {
        ((self.bits() >> Self::IMP_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `IMP` field.
    pub const fn set_imp(&mut self, value: u8) {
        let offset = Self::IMP_SHIFT;
        assert!(value & (Self::IMP_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMP_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `PMINTENCLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmintenclr: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmintenclr {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMINTENSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmintenset: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmintenset {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMMIR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmmir: u32 {
    }
}

impl Pmmir {
    /// Offset of the `SLOTS` field.
    pub const SLOTS_SHIFT: u32 = 0;
    /// Mask for the `SLOTS` field.
    pub const SLOTS_MASK: u32 = 0b11111111;
    /// Offset of the `BUS_SLOTS` field.
    pub const BUS_SLOTS_SHIFT: u32 = 8;
    /// Mask for the `BUS_SLOTS` field.
    pub const BUS_SLOTS_MASK: u32 = 0b11111111;
    /// Offset of the `BUS_WIDTH` field.
    pub const BUS_WIDTH_SHIFT: u32 = 16;
    /// Mask for the `BUS_WIDTH` field.
    pub const BUS_WIDTH_MASK: u32 = 0b1111;
    /// Offset of the `THWIDTH` field.
    pub const THWIDTH_SHIFT: u32 = 20;
    /// Mask for the `THWIDTH` field.
    pub const THWIDTH_MASK: u32 = 0b1111;
    /// Offset of the `EDGE` field.
    pub const EDGE_SHIFT: u32 = 24;
    /// Mask for the `EDGE` field.
    pub const EDGE_MASK: u32 = 0b1111;

    /// Returns the value of the `SLOTS` field.
    pub const fn slots(self) -> u8 {
        ((self.bits() >> Self::SLOTS_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `SLOTS` field.
    pub const fn set_slots(&mut self, value: u8) {
        let offset = Self::SLOTS_SHIFT;
        assert!(value & (Self::SLOTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SLOTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BUS_SLOTS` field.
    pub const fn bus_slots(self) -> u8 {
        ((self.bits() >> Self::BUS_SLOTS_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `BUS_SLOTS` field.
    pub const fn set_bus_slots(&mut self, value: u8) {
        let offset = Self::BUS_SLOTS_SHIFT;
        assert!(value & (Self::BUS_SLOTS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BUS_SLOTS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `BUS_WIDTH` field.
    pub const fn bus_width(self) -> u8 {
        ((self.bits() >> Self::BUS_WIDTH_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `BUS_WIDTH` field.
    pub const fn set_bus_width(&mut self, value: u8) {
        let offset = Self::BUS_WIDTH_SHIFT;
        assert!(value & (Self::BUS_WIDTH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BUS_WIDTH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `THWIDTH` field.
    pub const fn thwidth(self) -> u8 {
        ((self.bits() >> Self::THWIDTH_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `THWIDTH` field.
    pub const fn set_thwidth(&mut self, value: u8) {
        let offset = Self::THWIDTH_SHIFT;
        assert!(value & (Self::THWIDTH_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THWIDTH_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `EDGE` field.
    pub const fn edge(self) -> u8 {
        ((self.bits() >> Self::EDGE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `EDGE` field.
    pub const fn set_edge(&mut self, value: u8) {
        let offset = Self::EDGE_SHIFT;
        assert!(value & (Self::EDGE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::EDGE_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `PMOVSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmovsr: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmovsr {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMOVSSET` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmovsset: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
        /// `C` bit.
        const C = 1 << 31;
    }
}

impl Pmovsset {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 31;
}

bitflags! {
    /// `PMSELR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmselr: u32 {
    }
}

impl Pmselr {
    /// Offset of the `SEL` field.
    pub const SEL_SHIFT: u32 = 0;
    /// Mask for the `SEL` field.
    pub const SEL_MASK: u32 = 0b11111;

    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u8 {
        ((self.bits() >> Self::SEL_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `SEL` field.
    pub const fn set_sel(&mut self, value: u8) {
        let offset = Self::SEL_SHIFT;
        assert!(value & (Self::SEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEL_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `PMSWINC` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmswinc: u32 {
        /// `P<m>` bit 0.
        const P0 = 1 << 0;
        /// `P<m>` bit 1.
        const P1 = 1 << 1;
        /// `P<m>` bit 2.
        const P2 = 1 << 2;
        /// `P<m>` bit 3.
        const P3 = 1 << 3;
        /// `P<m>` bit 4.
        const P4 = 1 << 4;
        /// `P<m>` bit 5.
        const P5 = 1 << 5;
        /// `P<m>` bit 6.
        const P6 = 1 << 6;
        /// `P<m>` bit 7.
        const P7 = 1 << 7;
        /// `P<m>` bit 8.
        const P8 = 1 << 8;
        /// `P<m>` bit 9.
        const P9 = 1 << 9;
        /// `P<m>` bit 10.
        const P10 = 1 << 10;
        /// `P<m>` bit 11.
        const P11 = 1 << 11;
        /// `P<m>` bit 12.
        const P12 = 1 << 12;
        /// `P<m>` bit 13.
        const P13 = 1 << 13;
        /// `P<m>` bit 14.
        const P14 = 1 << 14;
        /// `P<m>` bit 15.
        const P15 = 1 << 15;
        /// `P<m>` bit 16.
        const P16 = 1 << 16;
        /// `P<m>` bit 17.
        const P17 = 1 << 17;
        /// `P<m>` bit 18.
        const P18 = 1 << 18;
        /// `P<m>` bit 19.
        const P19 = 1 << 19;
        /// `P<m>` bit 20.
        const P20 = 1 << 20;
        /// `P<m>` bit 21.
        const P21 = 1 << 21;
        /// `P<m>` bit 22.
        const P22 = 1 << 22;
        /// `P<m>` bit 23.
        const P23 = 1 << 23;
        /// `P<m>` bit 24.
        const P24 = 1 << 24;
        /// `P<m>` bit 25.
        const P25 = 1 << 25;
        /// `P<m>` bit 26.
        const P26 = 1 << 26;
        /// `P<m>` bit 27.
        const P27 = 1 << 27;
        /// `P<m>` bit 28.
        const P28 = 1 << 28;
        /// `P<m>` bit 29.
        const P29 = 1 << 29;
        /// `P<m>` bit 30.
        const P30 = 1 << 30;
    }
}

impl Pmswinc {
    /// Offset of the `P<m>` field.
    pub const P_SHIFT: u32 = 0;
}

bitflags! {
    /// `PMUSERENR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmuserenr: u32 {
        /// `EN` bit.
        const EN = 1 << 0;
        /// `SW` bit.
        const SW = 1 << 1;
        /// `CR` bit.
        const CR = 1 << 2;
        /// `ER` bit.
        const ER = 1 << 3;
        /// `TID` bit.
        const TID = 1 << 6;
    }
}

impl Pmuserenr {
    /// Offset of the `EN` field.
    pub const EN_SHIFT: u32 = 0;
    /// Offset of the `SW` field.
    pub const SW_SHIFT: u32 = 1;
    /// Offset of the `CR` field.
    pub const CR_SHIFT: u32 = 2;
    /// Offset of the `ER` field.
    pub const ER_SHIFT: u32 = 3;
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 6;
}

bitflags! {
    /// `PMXEVTYPER` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmxevtyper: u32 {
    }
}

impl Pmxevtyper {
    /// Offset of the `ETR` field.
    pub const ETR_SHIFT: u32 = 0;
    /// Mask for the `ETR` field.
    pub const ETR_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `ETR` field.
    pub const fn etr(self) -> u32 {
        ((self.bits() >> Self::ETR_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ETR` field.
    pub const fn set_etr(&mut self, value: u32) {
        let offset = Self::ETR_SHIFT;
        assert!(value & (Self::ETR_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ETR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `PRRR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Prrr: u32 {
        /// `DS0` bit.
        const DS0 = 1 << 16;
        /// `DS1` bit.
        const DS1 = 1 << 17;
        /// `NS0` bit.
        const NS0 = 1 << 18;
        /// `NS1` bit.
        const NS1 = 1 << 19;
        /// `NOS<n>` bit 0.
        const NOS0 = 1 << 24;
        /// `NOS<n>` bit 1.
        const NOS1 = 1 << 25;
        /// `NOS<n>` bit 2.
        const NOS2 = 1 << 26;
        /// `NOS<n>` bit 3.
        const NOS3 = 1 << 27;
        /// `NOS<n>` bit 4.
        const NOS4 = 1 << 28;
        /// `NOS<n>` bit 5.
        const NOS5 = 1 << 29;
        /// `NOS<n>` bit 6.
        const NOS6 = 1 << 30;
        /// `NOS<n>` bit 7.
        const NOS7 = 1 << 31;
    }
}

impl Prrr {
    /// Offset of the `TR<n>` field.
    pub const TR_SHIFT: u32 = 0;
    /// Mask for the `TR<n>` field.
    pub const TR_MASK: u32 = 0b11;
    /// Offset of the `DS0` field.
    pub const DS0_SHIFT: u32 = 16;
    /// Offset of the `DS1` field.
    pub const DS1_SHIFT: u32 = 17;
    /// Offset of the `NS0` field.
    pub const NS0_SHIFT: u32 = 18;
    /// Offset of the `NS1` field.
    pub const NS1_SHIFT: u32 = 19;
    /// Offset of the `NOS<n>` field.
    pub const NOS_SHIFT: u32 = 24;

    /// Returns the value of the given `TR<n>` field.
    pub const fn tr(self, n: u32) -> u8 {
        assert!(n < 8);
        ((self.bits() >> (Self::TR_SHIFT + (n - 0) * 2)) & 0b11) as u8
    }

    /// Sets the value of the `TR<n>` field.
    pub const fn set_tr(&mut self, n: u32, value: u8) {
        assert!(n < 8);
        let offset = Self::TR_SHIFT + (n - 0) * 2;
        assert!(value & (Self::TR_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TR_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `RGSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RgsrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl RgsrEl1 {
    /// Offset of the `TAG` field.
    pub const TAG_SHIFT: u32 = 0;
    /// Mask for the `TAG` field.
    pub const TAG_MASK: u64 = 0b1111;
    /// Offset of the `SEED` field.
    pub const SEED_SHIFT: u32 = 8;
    /// Mask for the `SEED` field.
    pub const SEED_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `TAG` field.
    pub const fn tag(self) -> u8 {
        ((self.bits() >> Self::TAG_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TAG` field.
    pub const fn set_tag(&mut self, value: u8) {
        let offset = Self::TAG_SHIFT;
        assert!(value & (Self::TAG_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TAG_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SEED` field.
    pub const fn seed(self) -> u16 {
        ((self.bits() >> Self::SEED_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `SEED` field.
    pub const fn set_seed(&mut self, value: u16) {
        let offset = Self::SEED_SHIFT;
        assert!(value & (Self::SEED_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SEED_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `RMR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rmr: u32 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

impl Rmr {
    /// Offset of the `AA64` field.
    pub const AA64_SHIFT: u32 = 0;
    /// Offset of the `RR` field.
    pub const RR_SHIFT: u32 = 1;
}

bitflags! {
    /// `RVBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rvbar: u32 {
        /// RES1 bits in the `RVBAR` register.
        const RES1 = 0b1;
    }
}

impl Rvbar {
    /// Offset of the `ResetAddress` field.
    pub const RESETADDRESS_SHIFT: u32 = 1;
    /// Mask for the `ResetAddress` field.
    pub const RESETADDRESS_MASK: u32 = 0b1111111111111111111111111111111;

    /// Returns the value of the `ResetAddress` field.
    pub const fn resetaddress(self) -> u32 {
        ((self.bits() >> Self::RESETADDRESS_SHIFT) & 0b1111111111111111111111111111111) as u32
    }

    /// Sets the value of the `ResetAddress` field.
    pub const fn set_resetaddress(&mut self, value: u32) {
        let offset = Self::RESETADDRESS_SHIFT;
        assert!(value & (Self::RESETADDRESS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RESETADDRESS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `SCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Scr: u32 {
        /// `NS` bit.
        const NS = 1 << 0;
        /// `IRQ` bit.
        const IRQ = 1 << 1;
        /// `FIQ` bit.
        const FIQ = 1 << 2;
        /// `EA` bit.
        const EA = 1 << 3;
        /// `FW` bit.
        const FW = 1 << 4;
        /// `AW` bit.
        const AW = 1 << 5;
        /// `nET` bit.
        const NET = 1 << 6;
        /// `SCD` bit.
        const SCD = 1 << 7;
        /// `HCE` bit.
        const HCE = 1 << 8;
        /// `SIF` bit.
        const SIF = 1 << 9;
        /// `TWI` bit.
        const TWI = 1 << 12;
        /// `TWE` bit.
        const TWE = 1 << 13;
        /// `TERR` bit.
        const TERR = 1 << 15;
    }
}

impl Scr {
    /// Offset of the `NS` field.
    pub const NS_SHIFT: u32 = 0;
    /// Offset of the `IRQ` field.
    pub const IRQ_SHIFT: u32 = 1;
    /// Offset of the `FIQ` field.
    pub const FIQ_SHIFT: u32 = 2;
    /// Offset of the `EA` field.
    pub const EA_SHIFT: u32 = 3;
    /// Offset of the `FW` field.
    pub const FW_SHIFT: u32 = 4;
    /// Offset of the `AW` field.
    pub const AW_SHIFT: u32 = 5;
    /// Offset of the `nET` field.
    pub const NET_SHIFT: u32 = 6;
    /// Offset of the `SCD` field.
    pub const SCD_SHIFT: u32 = 7;
    /// Offset of the `HCE` field.
    pub const HCE_SHIFT: u32 = 8;
    /// Offset of the `SIF` field.
    pub const SIF_SHIFT: u32 = 9;
    /// Offset of the `TWI` field.
    pub const TWI_SHIFT: u32 = 12;
    /// Offset of the `TWE` field.
    pub const TWE_SHIFT: u32 = 13;
    /// Offset of the `TERR` field.
    pub const TERR_SHIFT: u32 = 15;
}

#[cfg(feature = "el3")]
bitflags! {
    /// `SCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        ((self.bits() >> Self::TWEDEL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TWEDEL` field.
    pub const fn set_twedel(&mut self, value: u8) {
        let offset = Self::TWEDEL_SHIFT;
        assert!(value & (Self::TWEDEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWEDEL_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `SCTLR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr: u32 {
        /// RES1 bits in the `SCTLR` register.
        const RES1 = 0b10000000000100000000000;
        /// `M` bit.
        const M = 1 << 0;
        /// `A` bit.
        const A = 1 << 1;
        /// `C` bit.
        const C = 1 << 2;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 3;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 4;
        /// `CP15BEN` bit.
        const CP15BEN = 1 << 5;
        /// `UNK` bit.
        const UNK = 1 << 6;
        /// `ITD` bit.
        const ITD = 1 << 7;
        /// `SED` bit.
        const SED = 1 << 8;
        /// `EnRCTX` bit.
        const ENRCTX = 1 << 10;
        /// `I` bit.
        const I = 1 << 12;
        /// `V` bit.
        const V = 1 << 13;
        /// `nTWI` bit.
        const NTWI = 1 << 16;
        /// `nTWE` bit.
        const NTWE = 1 << 18;
        /// `WXN` bit.
        const WXN = 1 << 19;
        /// `UWXN` bit.
        const UWXN = 1 << 20;
        /// `SPAN` bit.
        const SPAN = 1 << 23;
        /// `EE` bit.
        const EE = 1 << 25;
        /// `TRE` bit.
        const TRE = 1 << 28;
        /// `AFE` bit.
        const AFE = 1 << 29;
        /// `TE` bit.
        const TE = 1 << 30;
        /// `DSSBS` bit.
        const DSSBS = 1 << 31;
    }
}

impl Sctlr {
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 0;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 1;
    /// Offset of the `C` field.
    pub const C_SHIFT: u32 = 2;
    /// Offset of the `nTLSMD` field.
    pub const NTLSMD_SHIFT: u32 = 3;
    /// Offset of the `LSMAOE` field.
    pub const LSMAOE_SHIFT: u32 = 4;
    /// Offset of the `CP15BEN` field.
    pub const CP15BEN_SHIFT: u32 = 5;
    /// Offset of the `UNK` field.
    pub const UNK_SHIFT: u32 = 6;
    /// Offset of the `ITD` field.
    pub const ITD_SHIFT: u32 = 7;
    /// Offset of the `SED` field.
    pub const SED_SHIFT: u32 = 8;
    /// Offset of the `EnRCTX` field.
    pub const ENRCTX_SHIFT: u32 = 10;
    /// Offset of the `I` field.
    pub const I_SHIFT: u32 = 12;
    /// Offset of the `V` field.
    pub const V_SHIFT: u32 = 13;
    /// Offset of the `nTWI` field.
    pub const NTWI_SHIFT: u32 = 16;
    /// Offset of the `nTWE` field.
    pub const NTWE_SHIFT: u32 = 18;
    /// Offset of the `WXN` field.
    pub const WXN_SHIFT: u32 = 19;
    /// Offset of the `UWXN` field.
    pub const UWXN_SHIFT: u32 = 20;
    /// Offset of the `SPAN` field.
    pub const SPAN_SHIFT: u32 = 23;
    /// Offset of the `EE` field.
    pub const EE_SHIFT: u32 = 25;
    /// Offset of the `TRE` field.
    pub const TRE_SHIFT: u32 = 28;
    /// Offset of the `AFE` field.
    pub const AFE_SHIFT: u32 = 29;
    /// Offset of the `TE` field.
    pub const TE_SHIFT: u32 = 30;
    /// Offset of the `DSSBS` field.
    pub const DSSBS_SHIFT: u32 = 31;
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
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

#[cfg(feature = "el1")]
bitflags! {
    /// `SCTLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `ITD` field.
    pub const ITD_SHIFT: u32 = 7;
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
    /// Offset of the `TSCXT` field.
    pub const TSCXT_SHIFT: u32 = 20;
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
    /// Offset of the `BT1` field.
    pub const BT1_SHIFT: u32 = 36;
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
        ((self.bits() >> Self::TCF0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TCF0` field.
    pub const fn set_tcf0(&mut self, value: u8) {
        let offset = Self::TCF0_SHIFT;
        assert!(value & (Self::TCF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        ((self.bits() >> Self::TCF_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TCF` field.
    pub const fn set_tcf(&mut self, value: u8) {
        let offset = Self::TCF_SHIFT;
        assert!(value & (Self::TCF_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> Self::TWEDEL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TWEDEL` field.
    pub const fn set_twedel(&mut self, value: u8) {
        let offset = Self::TWEDEL_SHIFT;
        assert!(value & (Self::TWEDEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWEDEL_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
        ((self.bits() >> Self::TCF0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TCF0` field.
    pub const fn set_tcf0(&mut self, value: u8) {
        let offset = Self::TCF0_SHIFT;
        assert!(value & (Self::TCF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        ((self.bits() >> Self::TCF_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TCF` field.
    pub const fn set_tcf(&mut self, value: u8) {
        let offset = Self::TCF_SHIFT;
        assert!(value & (Self::TCF_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        ((self.bits() >> Self::TWEDEL_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `TWEDEL` field.
    pub const fn set_twedel(&mut self, value: u8) {
        let offset = Self::TWEDEL_SHIFT;
        assert!(value & (Self::TWEDEL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TWEDEL_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `SCTLR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
        ((self.bits() >> Self::TCF_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TCF` field.
    pub const fn set_tcf(&mut self, value: u8) {
        let offset = Self::TCF_SHIFT;
        assert!(value & (Self::TCF_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TCF_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `SDCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sdcr: u32 {
        /// `SPME` bit.
        const SPME = 1 << 17;
        /// `STE` bit.
        const STE = 1 << 18;
        /// `TTRF` bit.
        const TTRF = 1 << 19;
        /// `EDAD` bit.
        const EDAD = 1 << 20;
        /// `EPMAD` bit.
        const EPMAD = 1 << 21;
        /// `SCCD` bit.
        const SCCD = 1 << 23;
        /// `TDCC` bit.
        const TDCC = 1 << 27;
        /// `MTPME` bit.
        const MTPME = 1 << 28;
    }
}

impl Sdcr {
    /// Offset of the `SPD` field.
    pub const SPD_SHIFT: u32 = 14;
    /// Mask for the `SPD` field.
    pub const SPD_MASK: u32 = 0b11;
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
    /// Offset of the `SCCD` field.
    pub const SCCD_SHIFT: u32 = 23;
    /// Offset of the `TDCC` field.
    pub const TDCC_SHIFT: u32 = 27;
    /// Offset of the `MTPME` field.
    pub const MTPME_SHIFT: u32 = 28;

    /// Returns the value of the `SPD` field.
    pub const fn spd(self) -> u8 {
        ((self.bits() >> Self::SPD_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SPD` field.
    pub const fn set_spd(&mut self, value: u8) {
        let offset = Self::SPD_SHIFT;
        assert!(value & (Self::SPD_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SPD_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `SDER` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sder: u32 {
        /// `SUIDEN` bit.
        const SUIDEN = 1 << 0;
        /// `SUNIDEN` bit.
        const SUNIDEN = 1 << 1;
    }
}

impl Sder {
    /// Offset of the `SUIDEN` field.
    pub const SUIDEN_SHIFT: u32 = 0;
    /// Offset of the `SUNIDEN` field.
    pub const SUNIDEN_SHIFT: u32 = 1;
}

#[cfg(feature = "el3")]
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

#[cfg(feature = "el3")]
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
        ((self.bits() >> Self::LEN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LEN` field.
    pub const fn set_len(&mut self, value: u8) {
        let offset = Self::LEN_SHIFT;
        assert!(value & (Self::LEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEN_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `PPEND` field.
    pub const PPEND_SHIFT: u32 = 33;
    /// Offset of the `EXLOCK` field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the `PACM` field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> Self::M_3_0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `M[3:0]` field.
    pub const fn set_m_3_0(&mut self, value: u8) {
        let offset = Self::M_3_0_SHIFT;
        assert!(value & (Self::M_3_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::M_3_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> Self::BTYPE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `BTYPE` field.
    pub const fn set_btype(&mut self, value: u8) {
        let offset = Self::BTYPE_SHIFT;
        assert!(value & (Self::BTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> Self::GE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GE` field.
    pub const fn set_ge(&mut self, value: u8) {
        let offset = Self::GE_SHIFT;
        assert!(value & (Self::GE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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
    /// Offset of the `PPEND` field.
    pub const PPEND_SHIFT: u32 = 33;
    /// Offset of the `EXLOCK` field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the `PACM` field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> Self::M_3_0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `M[3:0]` field.
    pub const fn set_m_3_0(&mut self, value: u8) {
        let offset = Self::M_3_0_SHIFT;
        assert!(value & (Self::M_3_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::M_3_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> Self::BTYPE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `BTYPE` field.
    pub const fn set_btype(&mut self, value: u8) {
        let offset = Self::BTYPE_SHIFT;
        assert!(value & (Self::BTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> Self::GE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GE` field.
    pub const fn set_ge(&mut self, value: u8) {
        let offset = Self::GE_SHIFT;
        assert!(value & (Self::GE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
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
    /// Offset of the `PPEND` field.
    pub const PPEND_SHIFT: u32 = 33;
    /// Offset of the `EXLOCK` field.
    pub const EXLOCK_SHIFT: u32 = 34;
    /// Offset of the `PACM` field.
    pub const PACM_SHIFT: u32 = 35;
    /// Offset of the `UINJ` field.
    pub const UINJ_SHIFT: u32 = 36;

    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        ((self.bits() >> Self::M_3_0_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `M[3:0]` field.
    pub const fn set_m_3_0(&mut self, value: u8) {
        let offset = Self::M_3_0_SHIFT;
        assert!(value & (Self::M_3_0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::M_3_0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        ((self.bits() >> Self::BTYPE_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `BTYPE` field.
    pub const fn set_btype(&mut self, value: u8) {
        let offset = Self::BTYPE_SHIFT;
        assert!(value & (Self::BTYPE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BTYPE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        ((self.bits() >> Self::GE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `GE` field.
    pub const fn set_ge(&mut self, value: u8) {
        let offset = Self::GE_SHIFT;
        assert!(value & (Self::GE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::GE_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SP_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl SpEl1 {
    /// Offset of the `StackPointer` field.
    pub const STACKPOINTER_SHIFT: u32 = 0;
    /// Mask for the `StackPointer` field.
    pub const STACKPOINTER_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        ((self.bits() >> Self::STACKPOINTER_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `StackPointer` field.
    pub const fn set_stackpointer(&mut self, value: u64) {
        let offset = Self::STACKPOINTER_SHIFT;
        assert!(value & (Self::STACKPOINTER_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STACKPOINTER_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl SpEl2 {
    /// Offset of the `StackPointer` field.
    pub const STACKPOINTER_SHIFT: u32 = 0;
    /// Mask for the `StackPointer` field.
    pub const STACKPOINTER_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        ((self.bits() >> Self::STACKPOINTER_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `StackPointer` field.
    pub const fn set_stackpointer(&mut self, value: u64) {
        let offset = Self::STACKPOINTER_SHIFT;
        assert!(value & (Self::STACKPOINTER_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STACKPOINTER_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TCR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `FNGNA0` field.
    pub const FNGNA0_SHIFT: u32 = 20;
    /// Offset of the `FNGNA1` field.
    pub const FNGNA1_SHIFT: u32 = 21;
    /// Offset of the `POIW` field.
    pub const POIW_SHIFT: u32 = 22;
    /// Mask for the `POIW` field.
    pub const POIW_MASK: u64 = 0b111;
    /// Offset of the `VTB0` field.
    pub const VTB0_SHIFT: u32 = 25;
    /// Mask for the `VTB0` field.
    pub const VTB0_MASK: u64 = 0b11111;
    /// Offset of the `VTB1` field.
    pub const VTB1_SHIFT: u32 = 30;
    /// Mask for the `VTB1` field.
    pub const VTB1_MASK: u64 = 0b11111;
    /// Offset of the `TVAD0` field.
    pub const TVAD0_SHIFT: u32 = 35;
    /// Offset of the `TVAD1` field.
    pub const TVAD1_SHIFT: u32 = 36;

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> Self::POIW_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `POIW` field.
    pub const fn set_poiw(&mut self, value: u8) {
        let offset = Self::POIW_SHIFT;
        assert!(value & (Self::POIW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::POIW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        ((self.bits() >> Self::VTB0_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `VTB0` field.
    pub const fn set_vtb0(&mut self, value: u8) {
        let offset = Self::VTB0_SHIFT;
        assert!(value & (Self::VTB0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        ((self.bits() >> Self::VTB1_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `VTB1` field.
    pub const fn set_vtb1(&mut self, value: u8) {
        let offset = Self::VTB1_SHIFT;
        assert!(value & (Self::VTB1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB1_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
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
    pub const VTB0_MASK: u64 = 0b11111;
    /// Offset of the `VTB1` field.
    pub const VTB1_SHIFT: u32 = 30;
    /// Mask for the `VTB1` field.
    pub const VTB1_MASK: u64 = 0b11111;
    /// Offset of the `TVAD0` field.
    pub const TVAD0_SHIFT: u32 = 35;
    /// Offset of the `TVAD1` field.
    pub const TVAD1_SHIFT: u32 = 36;

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> Self::POIW_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `POIW` field.
    pub const fn set_poiw(&mut self, value: u8) {
        let offset = Self::POIW_SHIFT;
        assert!(value & (Self::POIW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::POIW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        ((self.bits() >> Self::VTB0_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `VTB0` field.
    pub const fn set_vtb0(&mut self, value: u8) {
        let offset = Self::VTB0_SHIFT;
        assert!(value & (Self::VTB0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        ((self.bits() >> Self::VTB1_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `VTB1` field.
    pub const fn set_vtb1(&mut self, value: u8) {
        let offset = Self::VTB1_SHIFT;
        assert!(value & (Self::VTB1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB1_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b111111;
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
    /// Offset of the `T1SZ` field.
    pub const T1SZ_SHIFT: u32 = 16;
    /// Mask for the `T1SZ` field.
    pub const T1SZ_MASK: u64 = 0b111111;
    /// Offset of the `A1` field.
    pub const A1_SHIFT: u32 = 22;
    /// Offset of the `EPD1` field.
    pub const EPD1_SHIFT: u32 = 23;
    /// Offset of the `IRGN1` field.
    pub const IRGN1_SHIFT: u32 = 24;
    /// Mask for the `IRGN1` field.
    pub const IRGN1_MASK: u64 = 0b11;
    /// Offset of the `ORGN1` field.
    pub const ORGN1_SHIFT: u32 = 26;
    /// Mask for the `ORGN1` field.
    pub const ORGN1_MASK: u64 = 0b11;
    /// Offset of the `SH1` field.
    pub const SH1_SHIFT: u32 = 28;
    /// Mask for the `SH1` field.
    pub const SH1_MASK: u64 = 0b11;
    /// Offset of the `TG1` field.
    pub const TG1_SHIFT: u32 = 30;
    /// Mask for the `TG1` field.
    pub const TG1_MASK: u64 = 0b11;
    /// Offset of the `IPS` field.
    pub const IPS_SHIFT: u32 = 32;
    /// Mask for the `IPS` field.
    pub const IPS_MASK: u64 = 0b111;
    /// Offset of the `AS` field.
    pub const AS_SHIFT: u32 = 36;
    /// Offset of the `TBI0` field.
    pub const TBI0_SHIFT: u32 = 37;
    /// Offset of the `TBI1` field.
    pub const TBI1_SHIFT: u32 = 38;
    /// Offset of the `HA` field.
    pub const HA_SHIFT: u32 = 39;
    /// Offset of the `HD` field.
    pub const HD_SHIFT: u32 = 40;
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
    /// Offset of the `DS` field.
    pub const DS_SHIFT: u32 = 59;
    /// Offset of the `MTX0` field.
    pub const MTX0_SHIFT: u32 = 60;
    /// Offset of the `MTX1` field.
    pub const MTX1_SHIFT: u32 = 61;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> Self::TG0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TG0` field.
    pub const fn set_tg0(&mut self, value: u8) {
        let offset = Self::TG0_SHIFT;
        assert!(value & (Self::TG0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        ((self.bits() >> Self::T1SZ_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `T1SZ` field.
    pub const fn set_t1sz(&mut self, value: u8) {
        let offset = Self::T1SZ_SHIFT;
        assert!(value & (Self::T1SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T1SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        ((self.bits() >> Self::IRGN1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN1` field.
    pub const fn set_irgn1(&mut self, value: u8) {
        let offset = Self::IRGN1_SHIFT;
        assert!(value & (Self::IRGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        ((self.bits() >> Self::ORGN1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN1` field.
    pub const fn set_orgn1(&mut self, value: u8) {
        let offset = Self::ORGN1_SHIFT;
        assert!(value & (Self::ORGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        ((self.bits() >> Self::SH1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH1` field.
    pub const fn set_sh1(&mut self, value: u8) {
        let offset = Self::SH1_SHIFT;
        assert!(value & (Self::SH1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        ((self.bits() >> Self::TG1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TG1` field.
    pub const fn set_tg1(&mut self, value: u8) {
        let offset = Self::TG1_SHIFT;
        assert!(value & (Self::TG1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        ((self.bits() >> Self::IPS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `IPS` field.
    pub const fn set_ips(&mut self, value: u8) {
        let offset = Self::IPS_SHIFT;
        assert!(value & (Self::IPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IPS_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b111111;
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
    pub const T1SZ_MASK: u64 = 0b111111;
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
    /// Offset of the `VTB` field.
    pub const VTB_SHIFT: u32 = 48;
    /// Mask for the `VTB` field.
    pub const VTB_MASK: u64 = 0b11111;
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
    /// Offset of the `TVAD` field.
    pub const TVAD_SHIFT: u32 = 53;
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
        ((self.bits() >> Self::T0SZ_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> Self::TG0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TG0` field.
    pub const fn set_tg0(&mut self, value: u8) {
        let offset = Self::TG0_SHIFT;
        assert!(value & (Self::TG0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> Self::PS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `PS` field.
    pub const fn set_ps(&mut self, value: u8) {
        let offset = Self::PS_SHIFT;
        assert!(value & (Self::PS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        ((self.bits() >> Self::T1SZ_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `T1SZ` field.
    pub const fn set_t1sz(&mut self, value: u8) {
        let offset = Self::T1SZ_SHIFT;
        assert!(value & (Self::T1SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T1SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        ((self.bits() >> Self::IRGN1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN1` field.
    pub const fn set_irgn1(&mut self, value: u8) {
        let offset = Self::IRGN1_SHIFT;
        assert!(value & (Self::IRGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        ((self.bits() >> Self::ORGN1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN1` field.
    pub const fn set_orgn1(&mut self, value: u8) {
        let offset = Self::ORGN1_SHIFT;
        assert!(value & (Self::ORGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        ((self.bits() >> Self::SH1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH1` field.
    pub const fn set_sh1(&mut self, value: u8) {
        let offset = Self::SH1_SHIFT;
        assert!(value & (Self::SH1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        ((self.bits() >> Self::TG1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TG1` field.
    pub const fn set_tg1(&mut self, value: u8) {
        let offset = Self::TG1_SHIFT;
        assert!(value & (Self::TG1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        ((self.bits() >> Self::IPS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `IPS` field.
    pub const fn set_ips(&mut self, value: u8) {
        let offset = Self::IPS_SHIFT;
        assert!(value & (Self::IPS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IPS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        ((self.bits() >> Self::VTB_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `VTB` field.
    pub const fn set_vtb(&mut self, value: u8) {
        let offset = Self::VTB_SHIFT;
        assert!(value & (Self::VTB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `TCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b111111;
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
    pub const VTB_MASK: u64 = 0b11111;
    /// Offset of the `TVAD` field.
    pub const TVAD_SHIFT: u32 = 53;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> Self::TG0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TG0` field.
    pub const fn set_tg0(&mut self, value: u8) {
        let offset = Self::TG0_SHIFT;
        assert!(value & (Self::TG0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> Self::PS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `PS` field.
    pub const fn set_ps(&mut self, value: u8) {
        let offset = Self::PS_SHIFT;
        assert!(value & (Self::PS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        ((self.bits() >> Self::POIW_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `POIW` field.
    pub const fn set_poiw(&mut self, value: u8) {
        let offset = Self::POIW_SHIFT;
        assert!(value & (Self::POIW_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::POIW_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        ((self.bits() >> Self::VTB_SHIFT) & 0b11111) as u8
    }

    /// Sets the value of the `VTB` field.
    pub const fn set_vtb(&mut self, value: u8) {
        let offset = Self::VTB_SHIFT;
        assert!(value & (Self::VTB_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VTB_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TFSRE0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `TF0` field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the `TF1` field.
    pub const TF1_SHIFT: u32 = 1;
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TFSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `TF0` field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the `TF1` field.
    pub const TF1_SHIFT: u32 = 1;
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl TfsrEl2 {
    /// Offset of the `TF0` field.
    pub const TF0_SHIFT: u32 = 0;
    /// Offset of the `TF1` field.
    pub const TF1_SHIFT: u32 = 1;
}

bitflags! {
    /// `TLBTR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbtr: u32 {
        /// `nU` bit.
        const NU = 1 << 0;
    }
}

impl Tlbtr {
    /// Offset of the `nU` field.
    pub const NU_SHIFT: u32 = 0;
}

bitflags! {
    /// `TPIDRPRW` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidrprw: u32 {
    }
}

impl Tpidrprw {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        ((self.bits() >> Self::TID_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & (Self::TID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TID_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `TPIDRRO_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrroEl0: u64 {
    }
}

impl TpidrroEl0 {
    /// Offset of the `ThreadID` field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the `ThreadID` field.
    pub const THREADID_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> Self::THREADID_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `ThreadID` field.
    pub const fn set_threadid(&mut self, value: u64) {
        let offset = Self::THREADID_SHIFT;
        assert!(value & (Self::THREADID_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THREADID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `TPIDRURO` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidruro: u32 {
    }
}

impl Tpidruro {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        ((self.bits() >> Self::TID_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & (Self::TID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TID_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `TPIDRURW` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidrurw: u32 {
    }
}

impl Tpidrurw {
    /// Offset of the `TID` field.
    pub const TID_SHIFT: u32 = 0;
    /// Mask for the `TID` field.
    pub const TID_MASK: u32 = 0b11111111111111111111111111111111;

    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        ((self.bits() >> Self::TID_SHIFT) & 0b11111111111111111111111111111111) as u32
    }

    /// Sets the value of the `TID` field.
    pub const fn set_tid(&mut self, value: u32) {
        let offset = Self::TID_SHIFT;
        assert!(value & (Self::TID_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TID_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `TPIDR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl0: u64 {
    }
}

impl TpidrEl0 {
    /// Offset of the `ThreadID` field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the `ThreadID` field.
    pub const THREADID_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> Self::THREADID_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `ThreadID` field.
    pub const fn set_threadid(&mut self, value: u64) {
        let offset = Self::THREADID_SHIFT;
        assert!(value & (Self::THREADID_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THREADID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TPIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl TpidrEl1 {
    /// Offset of the `ThreadID` field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the `ThreadID` field.
    pub const THREADID_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> Self::THREADID_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `ThreadID` field.
    pub const fn set_threadid(&mut self, value: u64) {
        let offset = Self::THREADID_SHIFT;
        assert!(value & (Self::THREADID_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THREADID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl TpidrEl2 {
    /// Offset of the `ThreadID` field.
    pub const THREADID_SHIFT: u32 = 0;
    /// Mask for the `ThreadID` field.
    pub const THREADID_MASK: u64 =
        0b1111111111111111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        ((self.bits() >> Self::THREADID_SHIFT)
            & 0b1111111111111111111111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `ThreadID` field.
    pub const fn set_threadid(&mut self, value: u64) {
        let offset = Self::THREADID_SHIFT;
        assert!(value & (Self::THREADID_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::THREADID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `TRFCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trfcr: u32 {
        /// `E0TRE` bit.
        const E0TRE = 1 << 0;
        /// `E1TRE` bit.
        const E1TRE = 1 << 1;
    }
}

impl Trfcr {
    /// Offset of the `E0TRE` field.
    pub const E0TRE_SHIFT: u32 = 0;
    /// Offset of the `E1TRE` field.
    pub const E1TRE_SHIFT: u32 = 1;
    /// Offset of the `TS` field.
    pub const TS_SHIFT: u32 = 5;
    /// Mask for the `TS` field.
    pub const TS_MASK: u32 = 0b11;

    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        ((self.bits() >> Self::TS_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TS` field.
    pub const fn set_ts(&mut self, value: u8) {
        let offset = Self::TS_SHIFT;
        assert!(value & (Self::TS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TS_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `TTBCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbcr: u32 {
        /// `PD0` bit.
        const PD0 = 1 << 4;
        /// `PD1` bit.
        const PD1 = 1 << 5;
        /// `T2E` bit.
        const T2E = 1 << 6;
        /// `EPD0` bit.
        const EPD0 = 1 << 7;
        /// `A1` bit.
        const A1 = 1 << 22;
        /// `EPD1` bit.
        const EPD1 = 1 << 23;
        /// `EAE` bit.
        const EAE = 1 << 31;
    }
}

impl Ttbcr {
    /// Offset of the `N` field.
    pub const N_SHIFT: u32 = 0;
    /// Mask for the `N` field.
    pub const N_MASK: u32 = 0b111;
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u32 = 0b111;
    /// Offset of the `PD0` field.
    pub const PD0_SHIFT: u32 = 4;
    /// Offset of the `PD1` field.
    pub const PD1_SHIFT: u32 = 5;
    /// Offset of the `T2E` field.
    pub const T2E_SHIFT: u32 = 6;
    /// Offset of the `EPD0` field.
    pub const EPD0_SHIFT: u32 = 7;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u32 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u32 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u32 = 0b11;
    /// Offset of the `T1SZ` field.
    pub const T1SZ_SHIFT: u32 = 16;
    /// Mask for the `T1SZ` field.
    pub const T1SZ_MASK: u32 = 0b111;
    /// Offset of the `A1` field.
    pub const A1_SHIFT: u32 = 22;
    /// Offset of the `EPD1` field.
    pub const EPD1_SHIFT: u32 = 23;
    /// Offset of the `IRGN1` field.
    pub const IRGN1_SHIFT: u32 = 24;
    /// Mask for the `IRGN1` field.
    pub const IRGN1_MASK: u32 = 0b11;
    /// Offset of the `ORGN1` field.
    pub const ORGN1_SHIFT: u32 = 26;
    /// Mask for the `ORGN1` field.
    pub const ORGN1_MASK: u32 = 0b11;
    /// Offset of the `SH1` field.
    pub const SH1_SHIFT: u32 = 28;
    /// Mask for the `SH1` field.
    pub const SH1_MASK: u32 = 0b11;
    /// Offset of the `EAE` field.
    pub const EAE_SHIFT: u32 = 31;

    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        ((self.bits() >> Self::N_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `N` field.
    pub const fn set_n(&mut self, value: u8) {
        let offset = Self::N_SHIFT;
        assert!(value & (Self::N_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::N_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        ((self.bits() >> Self::T1SZ_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `T1SZ` field.
    pub const fn set_t1sz(&mut self, value: u8) {
        let offset = Self::T1SZ_SHIFT;
        assert!(value & (Self::T1SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T1SZ_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        ((self.bits() >> Self::IRGN1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN1` field.
    pub const fn set_irgn1(&mut self, value: u8) {
        let offset = Self::IRGN1_SHIFT;
        assert!(value & (Self::IRGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        ((self.bits() >> Self::ORGN1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN1` field.
    pub const fn set_orgn1(&mut self, value: u8) {
        let offset = Self::ORGN1_SHIFT;
        assert!(value & (Self::ORGN1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        ((self.bits() >> Self::SH1_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH1` field.
    pub const fn set_sh1(&mut self, value: u8) {
        let offset = Self::SH1_SHIFT;
        assert!(value & (Self::SH1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH1_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `TTBCR2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbcr2: u32 {
        /// `HPD0` bit.
        const HPD0 = 1 << 9;
        /// `HPD1` bit.
        const HPD1 = 1 << 10;
        /// `HWU059` bit.
        const HWU059 = 1 << 11;
        /// `HWU060` bit.
        const HWU060 = 1 << 12;
        /// `HWU061` bit.
        const HWU061 = 1 << 13;
        /// `HWU062` bit.
        const HWU062 = 1 << 14;
        /// `HWU159` bit.
        const HWU159 = 1 << 15;
        /// `HWU160` bit.
        const HWU160 = 1 << 16;
        /// `HWU161` bit.
        const HWU161 = 1 << 17;
        /// `HWU162` bit.
        const HWU162 = 1 << 18;
    }
}

impl Ttbcr2 {
    /// Offset of the `HPD0` field.
    pub const HPD0_SHIFT: u32 = 9;
    /// Offset of the `HPD1` field.
    pub const HPD1_SHIFT: u32 = 10;
    /// Offset of the `HWU059` field.
    pub const HWU059_SHIFT: u32 = 11;
    /// Offset of the `HWU060` field.
    pub const HWU060_SHIFT: u32 = 12;
    /// Offset of the `HWU061` field.
    pub const HWU061_SHIFT: u32 = 13;
    /// Offset of the `HWU062` field.
    pub const HWU062_SHIFT: u32 = 14;
    /// Offset of the `HWU159` field.
    pub const HWU159_SHIFT: u32 = 15;
    /// Offset of the `HWU160` field.
    pub const HWU160_SHIFT: u32 = 16;
    /// Offset of the `HWU161` field.
    pub const HWU161_SHIFT: u32 = 17;
    /// Offset of the `HWU162` field.
    pub const HWU162_SHIFT: u32 = 18;
}

bitflags! {
    /// `TTBR0` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
        /// `S` bit.
        const S = 1 << 1;
        /// `IMP` bit.
        const IMP = 1 << 2;
        /// `NOS` bit.
        const NOS = 1 << 5;
    }
}

impl Ttbr0 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 1;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 2;
    /// Offset of the `RGN` field.
    pub const RGN_SHIFT: u32 = 3;
    /// Mask for the `RGN` field.
    pub const RGN_MASK: u64 = 0b11;
    /// Offset of the `NOS` field.
    pub const NOS_SHIFT: u32 = 5;
    /// Offset of the `TTB0` field.
    pub const TTB0_SHIFT: u32 = 7;
    /// Mask for the `TTB0` field.
    pub const TTB0_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b11111111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        ((self.bits() >> Self::BADDR_SHIFT) & 0b11111111111111111111111111111111111111111111111)
            as u64
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & (Self::BADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RGN` field.
    pub const fn rgn(self) -> u8 {
        ((self.bits() >> Self::RGN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `RGN` field.
    pub const fn set_rgn(&mut self, value: u8) {
        let offset = Self::RGN_SHIFT;
        assert!(value & (Self::RGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TTB0` field.
    pub const fn ttb0(self) -> u32 {
        ((self.bits() >> Self::TTB0_SHIFT) & 0b1111111111111111111111111) as u32
    }

    /// Sets the value of the `TTB0` field.
    pub const fn set_ttb0(&mut self, value: u32) {
        let offset = Self::TTB0_SHIFT;
        assert!(value & (Self::TTB0_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTB0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        ((self.bits() >> Self::ASID_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u8) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TTBR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl Ttbr0El1 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR[47:1]` field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the `BADDR[47:1]` field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> Self::BADDR_47_1_SHIFT)
            & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `BADDR[47:1]` field.
    pub const fn set_baddr_47_1(&mut self, value: u64) {
        let offset = Self::BADDR_47_1_SHIFT;
        assert!(value & (Self::BADDR_47_1_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_47_1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> Self::ASID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u16) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TTBR0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl Ttbr0El2 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR[47:1]` field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the `BADDR[47:1]` field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> Self::BADDR_47_1_SHIFT)
            & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `BADDR[47:1]` field.
    pub const fn set_baddr_47_1(&mut self, value: u64) {
        let offset = Self::BADDR_47_1_SHIFT;
        assert!(value & (Self::BADDR_47_1_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_47_1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> Self::ASID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u16) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `TTBR0_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0El3: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el3")]
impl Ttbr0El3 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `TTBR1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
        /// `S` bit.
        const S = 1 << 1;
        /// `IMP` bit.
        const IMP = 1 << 2;
        /// `NOS` bit.
        const NOS = 1 << 5;
    }
}

impl Ttbr1 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 1;
    /// Offset of the `IMP` field.
    pub const IMP_SHIFT: u32 = 2;
    /// Offset of the `RGN` field.
    pub const RGN_SHIFT: u32 = 3;
    /// Mask for the `RGN` field.
    pub const RGN_MASK: u64 = 0b11;
    /// Offset of the `NOS` field.
    pub const NOS_SHIFT: u32 = 5;
    /// Offset of the `TTB1` field.
    pub const TTB1_SHIFT: u32 = 7;
    /// Mask for the `TTB1` field.
    pub const TTB1_MASK: u64 = 0b1111111111111111111111111;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b11111111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        ((self.bits() >> Self::BADDR_SHIFT) & 0b11111111111111111111111111111111111111111111111)
            as u64
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & (Self::BADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `RGN` field.
    pub const fn rgn(self) -> u8 {
        ((self.bits() >> Self::RGN_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `RGN` field.
    pub const fn set_rgn(&mut self, value: u8) {
        let offset = Self::RGN_SHIFT;
        assert!(value & (Self::RGN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RGN_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TTB1` field.
    pub const fn ttb1(self) -> u32 {
        ((self.bits() >> Self::TTB1_SHIFT) & 0b1111111111111111111111111) as u32
    }

    /// Sets the value of the `TTB1` field.
    pub const fn set_ttb1(&mut self, value: u32) {
        let offset = Self::TTB1_SHIFT;
        assert!(value & (Self::TTB1_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TTB1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        ((self.bits() >> Self::ASID_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u8) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TTBR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1El1: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl Ttbr1El1 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR[47:1]` field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the `BADDR[47:1]` field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> Self::BADDR_47_1_SHIFT)
            & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `BADDR[47:1]` field.
    pub const fn set_baddr_47_1(&mut self, value: u64) {
        let offset = Self::BADDR_47_1_SHIFT;
        assert!(value & (Self::BADDR_47_1_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_47_1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> Self::ASID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u16) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TTBR1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1El2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl Ttbr1El2 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR[47:1]` field.
    pub const BADDR_47_1_SHIFT: u32 = 1;
    /// Mask for the `BADDR[47:1]` field.
    pub const BADDR_47_1_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `ASID` field.
    pub const ASID_SHIFT: u32 = 48;
    /// Mask for the `ASID` field.
    pub const ASID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        ((self.bits() >> Self::BADDR_47_1_SHIFT)
            & 0b11111111111111111111111111111111111111111111111) as u64
    }

    /// Sets the value of the `BADDR[47:1]` field.
    pub const fn set_baddr_47_1(&mut self, value: u64) {
        let offset = Self::BADDR_47_1_SHIFT;
        assert!(value & (Self::BADDR_47_1_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_47_1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        ((self.bits() >> Self::ASID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `ASID` field.
    pub const fn set_asid(&mut self, value: u16) {
        let offset = Self::ASID_SHIFT;
        assert!(value & (Self::ASID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ASID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `VBAR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vbar: u32 {
    }
}

impl Vbar {
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 5;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u32 = 0b111111111111111111111111111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        ((self.bits() >> Self::VBA_SHIFT) & 0b111111111111111111111111111) as u32
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u32) {
        let offset = Self::VBA_SHIFT;
        assert!(value & (Self::VBA_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBA_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `VBAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl1: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl VbarEl1 {
    /// Offset of the `UT` field.
    pub const UT_SHIFT: u32 = 0;
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 11;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u64 = 0b11111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        ((self.bits() >> Self::VBA_SHIFT) & 0b11111111111111111111111111111111111111111111111111111)
            as u64
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u64) {
        let offset = Self::VBA_SHIFT;
        assert!(value & (Self::VBA_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBA_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VBAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl2: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl VbarEl2 {
    /// Offset of the `UT` field.
    pub const UT_SHIFT: u32 = 0;
    /// Offset of the `VBA` field.
    pub const VBA_SHIFT: u32 = 11;
    /// Mask for the `VBA` field.
    pub const VBA_MASK: u64 = 0b11111111111111111111111111111111111111111111111111111;

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        ((self.bits() >> Self::VBA_SHIFT) & 0b11111111111111111111111111111111111111111111111111111)
            as u64
    }

    /// Sets the value of the `VBA` field.
    pub const fn set_vba(&mut self, value: u64) {
        let offset = Self::VBA_SHIFT;
        assert!(value & (Self::VBA_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VBA_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `VDFSR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vdfsr: u32 {
        /// `ExT` bit.
        const EXT = 1 << 12;
    }
}

impl Vdfsr {
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u32 = 0b11;

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

bitflags! {
    /// `VDISR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vdisr: u32 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `A` bit.
        const A = 1 << 31;
    }
}

impl Vdisr {
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u32 = 0b111111;
    /// Offset of the `LPAE` field.
    pub const LPAE_SHIFT: u32 = 9;
    /// Offset of the `ExT` field.
    pub const EXT_SHIFT: u32 = 12;
    /// Offset of the `AET` field.
    pub const AET_SHIFT: u32 = 14;
    /// Mask for the `AET` field.
    pub const AET_MASK: u32 = 0b11;
    /// Offset of the `A` field.
    pub const A_SHIFT: u32 = 31;

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl VdisrEl2 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b111111111111111111111111;
    /// Offset of the `STATUS` field.
    pub const STATUS_SHIFT: u32 = 0;
    /// Mask for the `STATUS` field.
    pub const STATUS_MASK: u64 = 0b111111;
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
        ((self.bits() >> Self::ISS_SHIFT) & 0b111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        ((self.bits() >> Self::STATUS_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `STATUS` field.
    pub const fn set_status(&mut self, value: u8) {
        let offset = Self::STATUS_SHIFT;
        assert!(value & (Self::STATUS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::STATUS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `VMPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vmpidr: u32 {
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
        /// `M` bit.
        const M = 1 << 31;
    }
}

impl Vmpidr {
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u32 = 0b11111111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u32 = 0b11111111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u32 = 0b11111111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `M` field.
    pub const M_SHIFT: u32 = 31;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> Self::AFF1_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff1` field.
    pub const fn set_aff1(&mut self, value: u8) {
        let offset = Self::AFF1_SHIFT;
        assert!(value & (Self::AFF1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> Self::AFF2_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff2` field.
    pub const fn set_aff2(&mut self, value: u8) {
        let offset = Self::AFF2_SHIFT;
        assert!(value & (Self::AFF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VMPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `Aff0` field.
    pub const AFF0_SHIFT: u32 = 0;
    /// Mask for the `Aff0` field.
    pub const AFF0_MASK: u64 = 0b11111111;
    /// Offset of the `Aff1` field.
    pub const AFF1_SHIFT: u32 = 8;
    /// Mask for the `Aff1` field.
    pub const AFF1_MASK: u64 = 0b11111111;
    /// Offset of the `Aff2` field.
    pub const AFF2_SHIFT: u32 = 16;
    /// Mask for the `Aff2` field.
    pub const AFF2_MASK: u64 = 0b11111111;
    /// Offset of the `MT` field.
    pub const MT_SHIFT: u32 = 24;
    /// Offset of the `U` field.
    pub const U_SHIFT: u32 = 30;
    /// Offset of the `Aff3` field.
    pub const AFF3_SHIFT: u32 = 32;
    /// Mask for the `Aff3` field.
    pub const AFF3_MASK: u64 = 0b11111111;

    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        ((self.bits() >> Self::AFF0_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff0` field.
    pub const fn set_aff0(&mut self, value: u8) {
        let offset = Self::AFF0_SHIFT;
        assert!(value & (Self::AFF0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        ((self.bits() >> Self::AFF1_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff1` field.
    pub const fn set_aff1(&mut self, value: u8) {
        let offset = Self::AFF1_SHIFT;
        assert!(value & (Self::AFF1_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF1_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        ((self.bits() >> Self::AFF2_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff2` field.
    pub const fn set_aff2(&mut self, value: u8) {
        let offset = Self::AFF2_SHIFT;
        assert!(value & (Self::AFF2_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF2_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        ((self.bits() >> Self::AFF3_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Aff3` field.
    pub const fn set_aff3(&mut self, value: u8) {
        let offset = Self::AFF3_SHIFT;
        assert!(value & (Self::AFF3_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AFF3_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `VPIDR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vpidr: u32 {
    }
}

impl Vpidr {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u32 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u32 = 0b111111111111;
    /// Offset of the `Architecture` field.
    pub const ARCHITECTURE_SHIFT: u32 = 16;
    /// Mask for the `Architecture` field.
    pub const ARCHITECTURE_MASK: u32 = 0b1111;
    /// Offset of the `Variant` field.
    pub const VARIANT_SHIFT: u32 = 20;
    /// Mask for the `Variant` field.
    pub const VARIANT_MASK: u32 = 0b1111;
    /// Offset of the `Implementer` field.
    pub const IMPLEMENTER_SHIFT: u32 = 24;
    /// Mask for the `Implementer` field.
    pub const IMPLEMENTER_MASK: u32 = 0b11111111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> Self::PARTNUM_SHIFT) & 0b111111111111) as u16
    }

    /// Sets the value of the `PartNum` field.
    pub const fn set_partnum(&mut self, value: u16) {
        let offset = Self::PARTNUM_SHIFT;
        assert!(value & (Self::PARTNUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> Self::ARCHITECTURE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Architecture` field.
    pub const fn set_architecture(&mut self, value: u8) {
        let offset = Self::ARCHITECTURE_SHIFT;
        assert!(value & (Self::ARCHITECTURE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> Self::VARIANT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Variant` field.
    pub const fn set_variant(&mut self, value: u8) {
        let offset = Self::VARIANT_SHIFT;
        assert!(value & (Self::VARIANT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> Self::IMPLEMENTER_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Implementer` field.
    pub const fn set_implementer(&mut self, value: u8) {
        let offset = Self::IMPLEMENTER_SHIFT;
        assert!(value & (Self::IMPLEMENTER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VPIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VpidrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl VpidrEl2 {
    /// Offset of the `Revision` field.
    pub const REVISION_SHIFT: u32 = 0;
    /// Mask for the `Revision` field.
    pub const REVISION_MASK: u64 = 0b1111;
    /// Offset of the `PartNum` field.
    pub const PARTNUM_SHIFT: u32 = 4;
    /// Mask for the `PartNum` field.
    pub const PARTNUM_MASK: u64 = 0b111111111111;
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
    pub const IMPLEMENTER_MASK: u64 = 0b11111111;

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        ((self.bits() >> Self::REVISION_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Revision` field.
    pub const fn set_revision(&mut self, value: u8) {
        let offset = Self::REVISION_SHIFT;
        assert!(value & (Self::REVISION_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::REVISION_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        ((self.bits() >> Self::PARTNUM_SHIFT) & 0b111111111111) as u16
    }

    /// Sets the value of the `PartNum` field.
    pub const fn set_partnum(&mut self, value: u16) {
        let offset = Self::PARTNUM_SHIFT;
        assert!(value & (Self::PARTNUM_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PARTNUM_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        ((self.bits() >> Self::ARCHITECTURE_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Architecture` field.
    pub const fn set_architecture(&mut self, value: u8) {
        let offset = Self::ARCHITECTURE_SHIFT;
        assert!(value & (Self::ARCHITECTURE_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ARCHITECTURE_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        ((self.bits() >> Self::VARIANT_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `Variant` field.
    pub const fn set_variant(&mut self, value: u8) {
        let offset = Self::VARIANT_SHIFT;
        assert!(value & (Self::VARIANT_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VARIANT_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        ((self.bits() >> Self::IMPLEMENTER_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `Implementer` field.
    pub const fn set_implementer(&mut self, value: u8) {
        let offset = Self::IMPLEMENTER_SHIFT;
        assert!(value & (Self::IMPLEMENTER_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IMPLEMENTER_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
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

#[cfg(feature = "el2")]
impl VsesrEl2 {
    /// Offset of the `ISS` field.
    pub const ISS_SHIFT: u32 = 0;
    /// Mask for the `ISS` field.
    pub const ISS_MASK: u64 = 0b111111111111111111111111;
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
        ((self.bits() >> Self::ISS_SHIFT) & 0b111111111111111111111111) as u32
    }

    /// Sets the value of the `ISS` field.
    pub const fn set_iss(&mut self, value: u32) {
        let offset = Self::ISS_SHIFT;
        assert!(value & (Self::ISS_MASK as u32) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ISS_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        ((self.bits() >> Self::AET_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `AET` field.
    pub const fn set_aet(&mut self, value: u8) {
        let offset = Self::AET_SHIFT;
        assert!(value & (Self::AET_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::AET_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `VTCR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vtcr: u32 {
        /// RES1 bits in the `VTCR` register.
        const RES1 = 0b10000000000000000000000000000000;
        /// `S` bit.
        const S = 1 << 4;
        /// `HWU59` bit.
        const HWU59 = 1 << 25;
        /// `HWU60` bit.
        const HWU60 = 1 << 26;
        /// `HWU61` bit.
        const HWU61 = 1 << 27;
        /// `HWU62` bit.
        const HWU62 = 1 << 28;
    }
}

impl Vtcr {
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u32 = 0b1111;
    /// Offset of the `S` field.
    pub const S_SHIFT: u32 = 4;
    /// Offset of the `SL0` field.
    pub const SL0_SHIFT: u32 = 6;
    /// Mask for the `SL0` field.
    pub const SL0_MASK: u32 = 0b11;
    /// Offset of the `IRGN0` field.
    pub const IRGN0_SHIFT: u32 = 8;
    /// Mask for the `IRGN0` field.
    pub const IRGN0_MASK: u32 = 0b11;
    /// Offset of the `ORGN0` field.
    pub const ORGN0_SHIFT: u32 = 10;
    /// Mask for the `ORGN0` field.
    pub const ORGN0_MASK: u32 = 0b11;
    /// Offset of the `SH0` field.
    pub const SH0_SHIFT: u32 = 12;
    /// Mask for the `SH0` field.
    pub const SH0_MASK: u32 = 0b11;
    /// Offset of the `HWU59` field.
    pub const HWU59_SHIFT: u32 = 25;
    /// Offset of the `HWU60` field.
    pub const HWU60_SHIFT: u32 = 26;
    /// Offset of the `HWU61` field.
    pub const HWU61_SHIFT: u32 = 27;
    /// Offset of the `HWU62` field.
    pub const HWU62_SHIFT: u32 = 28;

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        ((self.bits() >> Self::T0SZ_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        ((self.bits() >> Self::SL0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SL0` field.
    pub const fn set_sl0(&mut self, value: u8) {
        let offset = Self::SL0_SHIFT;
        assert!(value & (Self::SL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SL0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u32) << offset),
        );
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u32) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VTCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    /// Offset of the `T0SZ` field.
    pub const T0SZ_SHIFT: u32 = 0;
    /// Mask for the `T0SZ` field.
    pub const T0SZ_MASK: u64 = 0b111111;
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
        ((self.bits() >> Self::T0SZ_SHIFT) & 0b111111) as u8
    }

    /// Sets the value of the `T0SZ` field.
    pub const fn set_t0sz(&mut self, value: u8) {
        let offset = Self::T0SZ_SHIFT;
        assert!(value & (Self::T0SZ_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::T0SZ_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        ((self.bits() >> Self::SL0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SL0` field.
    pub const fn set_sl0(&mut self, value: u8) {
        let offset = Self::SL0_SHIFT;
        assert!(value & (Self::SL0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SL0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        ((self.bits() >> Self::IRGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `IRGN0` field.
    pub const fn set_irgn0(&mut self, value: u8) {
        let offset = Self::IRGN0_SHIFT;
        assert!(value & (Self::IRGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::IRGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        ((self.bits() >> Self::ORGN0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `ORGN0` field.
    pub const fn set_orgn0(&mut self, value: u8) {
        let offset = Self::ORGN0_SHIFT;
        assert!(value & (Self::ORGN0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::ORGN0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        ((self.bits() >> Self::SH0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SH0` field.
    pub const fn set_sh0(&mut self, value: u8) {
        let offset = Self::SH0_SHIFT;
        assert!(value & (Self::SH0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        ((self.bits() >> Self::TG0_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `TG0` field.
    pub const fn set_tg0(&mut self, value: u8) {
        let offset = Self::TG0_SHIFT;
        assert!(value & (Self::TG0_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::TG0_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        ((self.bits() >> Self::PS_SHIFT) & 0b111) as u8
    }

    /// Sets the value of the `PS` field.
    pub const fn set_ps(&mut self, value: u8) {
        let offset = Self::PS_SHIFT;
        assert!(value & (Self::PS_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PS_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

bitflags! {
    /// `VTTBR` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vttbr: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

impl Vttbr {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `VMID` field.
    pub const VMID_SHIFT: u32 = 48;
    /// Mask for the `VMID` field.
    pub const VMID_MASK: u64 = 0b11111111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        ((self.bits() >> Self::BADDR_SHIFT) & 0b11111111111111111111111111111111111111111111111)
            as u64
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & (Self::BADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u8 {
        ((self.bits() >> Self::VMID_SHIFT) & 0b11111111) as u8
    }

    /// Sets the value of the `VMID` field.
    pub const fn set_vmid(&mut self, value: u8) {
        let offset = Self::VMID_SHIFT;
        assert!(value & (Self::VMID_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VTTBR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VttbrEl2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl VttbrEl2 {
    /// Offset of the `CnP` field.
    pub const CNP_SHIFT: u32 = 0;
    /// Offset of the `BADDR` field.
    pub const BADDR_SHIFT: u32 = 1;
    /// Mask for the `BADDR` field.
    pub const BADDR_MASK: u64 = 0b11111111111111111111111111111111111111111111111;
    /// Offset of the `SKL` field.
    pub const SKL_SHIFT: u32 = 1;
    /// Mask for the `SKL` field.
    pub const SKL_MASK: u64 = 0b11;
    /// Offset of the `VMID` field.
    pub const VMID_SHIFT: u32 = 48;
    /// Mask for the `VMID` field.
    pub const VMID_MASK: u64 = 0b1111111111111111;

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        ((self.bits() >> Self::BADDR_SHIFT) & 0b11111111111111111111111111111111111111111111111)
            as u64
    }

    /// Sets the value of the `BADDR` field.
    pub const fn set_baddr(&mut self, value: u64) {
        let offset = Self::BADDR_SHIFT;
        assert!(value & (Self::BADDR_MASK as u64) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::BADDR_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        ((self.bits() >> Self::SKL_SHIFT) & 0b11) as u8
    }

    /// Sets the value of the `SKL` field.
    pub const fn set_skl(&mut self, value: u8) {
        let offset = Self::SKL_SHIFT;
        assert!(value & (Self::SKL_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SKL_MASK << offset)) | ((value as u64) << offset),
        );
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u16 {
        ((self.bits() >> Self::VMID_SHIFT) & 0b1111111111111111) as u16
    }

    /// Sets the value of the `VMID` field.
    pub const fn set_vmid(&mut self, value: u16) {
        let offset = Self::VMID_SHIFT;
        assert!(value & (Self::VMID_MASK as u16) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::VMID_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ZCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ZcrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl ZcrEl3 {
    /// Offset of the `LEN` field.
    pub const LEN_SHIFT: u32 = 0;
    /// Mask for the `LEN` field.
    pub const LEN_MASK: u64 = 0b1111;

    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        ((self.bits() >> Self::LEN_SHIFT) & 0b1111) as u8
    }

    /// Sets the value of the `LEN` field.
    pub const fn set_len(&mut self, value: u8) {
        let offset = Self::LEN_SHIFT;
        assert!(value & (Self::LEN_MASK as u8) == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::LEN_MASK << offset)) | ((value as u64) << offset),
        );
    }
}

#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(actlr: (p15, 0, c0, c1, 1), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(actlr2: (p15, 0, c0, c1, 3), u32, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(actlr_el1, u64, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(actlr_el2, u64, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(adfsr: (p15, 0, c1, c5, 0), u32, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(afsr0_el1, u64, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(afsr0_el2, u64, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(afsr1_el1, u64, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(afsr1_el2, u64, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(aidr: (p15, 1, c0, c0, 7), u32, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(aifsr: (p15, 0, c1, c5, 1), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amair0: (p15, 0, c3, c10, 0), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amair1: (p15, 0, c3, c10, 1), u32, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(amair_el1, u64, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amair_el2, u64, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amcfgr: (p15, 0, c2, c13, 1), u32: Amcfgr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amcgcr: (p15, 0, c2, c13, 2), u32: Amcgcr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenclr0: (p15, 0, c2, c13, 4), u32: Amcntenclr0, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenclr1: (p15, 0, c3, c13, 0), u32: Amcntenclr1, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenset0: (p15, 0, c2, c13, 5), u32: Amcntenset0, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenset1: (p15, 0, c3, c13, 1), u32: Amcntenset1, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcr: (p15, 0, c2, c13, 0), u32: Amcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amuserenr: (p15, 0, c2, c13, 3), u32: Amuserenr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(apiakeyhi_el1: s3_0_c2_c1_1, u64: ApiakeyhiEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(apiakeylo_el1: s3_0_c2_c1_0, u64: ApiakeyloEl1, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(ccsidr: (p15, 1, c0, c0, 0), u32: Ccsidr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(ccsidr2: (p15, 1, c0, c0, 2), u32: Ccsidr2, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(ccsidr_el1, u64: CcsidrEl1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(clidr: (p15, 1, c0, c0, 1), u32: Clidr, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(clidr_el1, u64: ClidrEl1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntfrq: (p15, 0, c0, c14, 0), u32: Cntfrq, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntfrq_el0, u64: CntfrqEl0, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthctl: (p15, 4, c1, c14, 0), u32: Cnthctl, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthctl_el2, u64: CnthctlEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthps_ctl: (p15, 0, c2, c14, 1), u32: CnthpsCtl, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthps_cval: (p15, 2, c14), u64: CnthpsCval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthps_tval: (p15, 0, c2, c14, 0), u32: CnthpsTval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthp_ctl: (p15, 0, c2, c14, 1), u32: CnthpCtl, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthp_cval: (p15, 2, c14), u64: CnthpCval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthp_tval: (p15, 0, c2, c14, 0), u32: CnthpTval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthvs_ctl: (p15, 0, c3, c14, 1), u32: CnthvsCtl, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthvs_cval: (p15, 3, c14), u64: CnthvsCval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthvs_tval: (p15, 0, c3, c14, 0), u32: CnthvsTval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthv_ctl: (p15, 0, c3, c14, 1), u32: CnthvCtl, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthv_cval: (p15, 3, c14), u64: CnthvCval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthv_tval: (p15, 0, c3, c14, 0), u32: CnthvTval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntkctl: (p15, 0, c1, c14, 0), u32: Cntkctl, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntpct: (p15, 0, c14), u64: Cntpct, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntpctss: (p15, 8, c14), u64: Cntpctss, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(cntpct_el0, u64: CntpctEl0, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntp_ctl: (p15, 0, c2, c14, 1), u32: CntpCtl, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntp_cval: (p15, 2, c14), u64: CntpCval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntp_tval: (p15, 0, c2, c14, 0), u32: CntpTval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntvct: (p15, 1, c14), u64: Cntvct, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntvctss: (p15, 9, c14), u64: Cntvctss, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntvoff: (p15, 4, c14), u64: Cntvoff, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cntvoff_el2, u64: CntvoffEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntv_ctl: (p15, 0, c3, c14, 1), u32: CntvCtl, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntv_cval: (p15, 3, c14), u64: CntvCval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntv_tval: (p15, 0, c3, c14, 0), u32: CntvTval, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(contextidr: (p15, 0, c0, c13, 1), u32: Contextidr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(contextidr_el1, u64: ContextidrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(contextidr_el2: s3_4_c13_c0_1, u64: ContextidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cpacr: (p15, 0, c0, c1, 2), u32: Cpacr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(cpacr_el1, u64: CpacrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cptr_el2, u64: CptrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(cptr_el3, u64: CptrEl3, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(csselr: (p15, 2, c0, c0, 0), u32: Csselr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(csselr_el1, u64: CsselrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(ctr: (p15, 0, c0, c0, 1), u32: Ctr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(ctr_el0, u64: CtrEl0, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(currentel, u64: Currentel, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dacr: (p15, 0, c0, c3, 0), u32: Dacr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgauthstatus: (p14, 0, c14, c7, 6), u32: Dbgauthstatus, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgclaimclr: (p14, 0, c9, c7, 6), u32: Dbgclaimclr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgclaimset: (p14, 0, c8, c7, 6), u32: Dbgclaimset, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdccint: (p14, 0, c2, c0, 0), u32: Dbgdccint, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdevid: (p14, 0, c2, c7, 7), u32: Dbgdevid, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdevid1: (p14, 0, c1, c7, 7), u32: Dbgdevid1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdevid2: (p14, 0, c0, c7, 7), u32, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdidr: (p14, 0, c0, c0, 0), u32: Dbgdidr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdrar: (p14, 0, c1), u64: Dbgdrar, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdsar: (p14, 0, c2), u64, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdscrext: (p14, 0, c2, c0, 2), u32: Dbgdscrext, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdscrint: (p14, 0, c1, c0, 0), u32: Dbgdscrint, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdtrrxext: (p14, 0, c0, c0, 2), u32: Dbgdtrrxext, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdtrrxint: (p14, 0, c5, c0, 0), u32: Dbgdtrrxint, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdtrtxext: (p14, 0, c3, c0, 2), u32: Dbgdtrtxext, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(dbgdtrtxint: (p14, 0, c5, c0, 0), u32: Dbgdtrtxint, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgosdlr: (p14, 0, c3, c1, 4), u32: Dbgosdlr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgoseccr: (p14, 0, c6, c0, 2), u32: Dbgoseccr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(dbgoslar: (p14, 0, c0, c1, 4), u32: Dbgoslar, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgoslsr: (p14, 0, c1, c1, 4), u32: Dbgoslsr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgprcr: (p14, 0, c4, c1, 4), u32: Dbgprcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgvcr: (p14, 0, c7, c0, 0), u32: Dbgvcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgwfar: (p14, 0, c6, c0, 0), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dfar: (p15, 0, c0, c6, 0), u32: Dfar, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dfsr: (p15, 0, c0, c5, 0), u32: Dfsr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(disr: (p15, 0, c1, c12, 1), u32: Disr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(disr_el1: s3_0_c12_c1_1, u64: DisrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(dit: s3_3_c4_c2_5, u64: Dit, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dlr: (p15, 3, c5, c4, 1), u32: Dlr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dspsr: (p15, 3, c5, c4, 0), u32: Dspsr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dspsr2: (p15, 3, c5, c4, 2), u32: Dspsr2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(elr_el1, u64: ElrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(elr_el2, u64: ElrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "arm"), feature = "el2"))]
read_write_sysreg!(elr_hyp, u32: ElrHyp, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(erridr: (p15, 0, c3, c5, 0), u32: Erridr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(errselr: (p15, 0, c3, c5, 1), u32: Errselr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxaddr: (p15, 0, c4, c5, 3), u32: Erxaddr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxaddr2: (p15, 0, c4, c5, 7), u32: Erxaddr2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxctlr: (p15, 0, c4, c5, 1), u32: Erxctlr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxctlr2: (p15, 0, c4, c5, 5), u32: Erxctlr2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(erxfr: (p15, 0, c4, c5, 0), u32: Erxfr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(erxfr2: (p15, 0, c4, c5, 4), u32: Erxfr2, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc0: (p15, 0, c5, c5, 0), u32: Erxmisc0, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc1: (p15, 0, c5, c5, 1), u32: Erxmisc1, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc2: (p15, 0, c5, c5, 4), u32: Erxmisc2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc3: (p15, 0, c5, c5, 5), u32: Erxmisc3, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc4: (p15, 0, c5, c5, 2), u32: Erxmisc4, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc5: (p15, 0, c5, c5, 3), u32: Erxmisc5, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc6: (p15, 0, c5, c5, 6), u32: Erxmisc6, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc7: (p15, 0, c5, c5, 7), u32: Erxmisc7, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxstatus: (p15, 0, c4, c5, 2), u32: Erxstatus, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(esr_el1, u64: EsrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(esr_el2, u64: EsrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(esr_el3, u64: EsrEl3, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(far_el1, u64: FarEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(far_el2, u64: FarEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(fcseidr: (p15, 0, c0, c13, 0), u32, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(gcr_el1: s3_0_c1_c0_6, u64: GcrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(gcscr_el1: s3_0_c2_c5_0, u64: GcscrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(gcscr_el2: s3_4_c2_c5_0, u64: GcscrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(gpccr_el3: s3_6_c2_c1_6, u64: GpccrEl3, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(gptbr_el3: s3_6_c2_c1_4, u64: GptbrEl3, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hacr: (p15, 4, c1, c1, 7), u32, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hacr_el2, u64, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hactlr: (p15, 4, c0, c1, 1), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hactlr2: (p15, 4, c0, c1, 3), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hadfsr: (p15, 4, c1, c5, 0), u32, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hafgrtr_el2: s3_4_c3_c1_6, u64: HafgrtrEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(haifsr: (p15, 4, c1, c5, 1), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hamair0: (p15, 4, c3, c10, 0), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hamair1: (p15, 4, c3, c10, 1), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hcptr: (p15, 4, c1, c1, 2), u32: Hcptr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hcr: (p15, 4, c1, c1, 0), u32: Hcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hcr2: (p15, 4, c1, c1, 4), u32: Hcr2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hcrx_el2: s3_4_c1_c2_2, u64: HcrxEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hcr_el2, u64: HcrEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hdcr: (p15, 4, c1, c1, 1), u32: Hdcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hdfar: (p15, 4, c0, c6, 0), u32: Hdfar, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgrtr2_el2: s3_4_c3_c1_0, u64: Hdfgrtr2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgrtr_el2: s3_4_c3_c1_4, u64: HdfgrtrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgwtr2_el2: s3_4_c3_c1_1, u64: Hdfgwtr2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgwtr_el2: s3_4_c3_c1_5, u64: HdfgwtrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgitr2_el2: s3_4_c3_c1_7, u64: Hfgitr2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgitr_el2: s3_4_c1_c1_6, u64: HfgitrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgrtr2_el2: s3_4_c3_c1_2, u64: Hfgrtr2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgrtr_el2: s3_4_c1_c1_4, u64: HfgrtrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgwtr2_el2: s3_4_c3_c1_3, u64: Hfgwtr2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgwtr_el2: s3_4_c1_c1_5, u64: HfgwtrEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hifar: (p15, 4, c0, c6, 2), u32: Hifar, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hmair0: (p15, 4, c2, c10, 0), u32: Hmair0, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hmair1: (p15, 4, c2, c10, 1), u32: Hmair1, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hpfar: (p15, 4, c0, c6, 4), u32: Hpfar, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hpfar_el2, u64: HpfarEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hrmr: (p15, 4, c0, c12, 2), u32: Hrmr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hsctlr: (p15, 4, c0, c1, 0), u32: Hsctlr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hsr: (p15, 4, c2, c5, 0), u32: Hsr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hstr: (p15, 4, c1, c1, 3), u32, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hstr_el2, u64, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(htcr: (p15, 4, c0, c2, 2), u32: Htcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(htpidr: (p15, 4, c0, c13, 2), u32: Htpidr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(htrfcr: (p15, 4, c2, c1, 1), u32: Htrfcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(httbr: (p15, 4, c2), u64: Httbr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hvbar: (p15, 4, c0, c12, 0), u32: Hvbar, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_sre_el1: s3_0_c12_c12_5, u64: IccSreEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(icc_sre_el2: s3_4_c12_c9_5, u64: IccSreEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The SRE bit of `icc_sre_el3` must not be changed from 1 to 0, as this can result in unpredictable behaviour.
    icc_sre_el3: s3_6_c12_c12_5, u64: IccSreEl3, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(ich_hcr_el2: s3_4_c12_c11_0, u64: IchHcrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(ich_vmcr_el2: s3_4_c12_c11_7, u64: IchVmcrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64dfr0_el1, u64: IdAa64dfr0El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64dfr1_el1, u64: IdAa64dfr1El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64isar1_el1, u64: IdAa64isar1El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64isar2_el1, u64: IdAa64isar2El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr0_el1, u64: IdAa64mmfr0El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr1_el1, u64: IdAa64mmfr1El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr2_el1, u64: IdAa64mmfr2El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr3_el1, u64: IdAa64mmfr3El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64pfr0_el1, u64: IdAa64pfr0El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64pfr1_el1, u64: IdAa64pfr1El1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64smfr0_el1: s3_0_c0_c4_5, u64: IdAa64smfr0El1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_afr0: (p15, 0, c1, c0, 3), u32, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_dfr0: (p15, 0, c1, c0, 2), u32: IdDfr0, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_dfr1: (p15, 0, c3, c0, 5), u32: IdDfr1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar0: (p15, 0, c2, c0, 0), u32: IdIsar0, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar1: (p15, 0, c2, c0, 1), u32: IdIsar1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar2: (p15, 0, c2, c0, 2), u32: IdIsar2, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar3: (p15, 0, c2, c0, 3), u32: IdIsar3, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar4: (p15, 0, c2, c0, 4), u32: IdIsar4, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar5: (p15, 0, c2, c0, 5), u32: IdIsar5, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar6: (p15, 0, c2, c0, 7), u32: IdIsar6, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr0: (p15, 0, c1, c0, 4), u32: IdMmfr0, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr1: (p15, 0, c1, c0, 5), u32: IdMmfr1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr2: (p15, 0, c1, c0, 6), u32: IdMmfr2, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr3: (p15, 0, c1, c0, 7), u32: IdMmfr3, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr4: (p15, 0, c2, c0, 6), u32: IdMmfr4, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr5: (p15, 0, c3, c0, 6), u32: IdMmfr5, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_pfr0: (p15, 0, c1, c0, 0), u32: IdPfr0, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_pfr1: (p15, 0, c1, c0, 1), u32: IdPfr1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_pfr2: (p15, 0, c3, c0, 4), u32: IdPfr2, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ifar: (p15, 0, c0, c6, 2), u32: Ifar, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ifsr: (p15, 0, c0, c5, 1), u32: Ifsr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(isr: (p15, 0, c1, c12, 0), u32: Isr, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(isr_el1, u64: IsrEl1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(jidr: (p14, 7, c0, c0, 0), u32, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(jmcr: (p14, 7, c0, c2, 0), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(joscr: (p14, 7, c0, c1, 0), u32, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(mair0: (p15, 0, c2, c10, 0), u32: Mair0, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(mair1: (p15, 0, c2, c10, 1), u32: Mair1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(mair_el1, u64: MairEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mair_el2, u64: MairEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 memory attribute indirection register.
    mair_el3, u64: MairEl3, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(mdccint_el1, u64: MdccintEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mdcr_el2, u64: MdcrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(mdcr_el3, u64: MdcrEl3, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(mdscr_el1, u64: MdscrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(midr: (p15, 0, c0, c0, 0), u32: Midr, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(midr_el1, u64: MidrEl1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpam2_el2: s3_4_c10_c5_0, u64: Mpam2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(mpam3_el3: s3_6_c10_c5_0, u64: Mpam3El3, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamhcr_el2: s3_4_c10_c4_0, u64: MpamhcrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(mpamidr_el1: s3_0_c10_c4_4, u64: MpamidrEl1, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm0_el2: s3_4_c10_c6_0, u64: Mpamvpm0El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm1_el2: s3_4_c10_c6_1, u64: Mpamvpm1El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm2_el2: s3_4_c10_c6_2, u64: Mpamvpm2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm3_el2: s3_4_c10_c6_3, u64: Mpamvpm3El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm4_el2: s3_4_c10_c6_4, u64: Mpamvpm4El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm5_el2: s3_4_c10_c6_5, u64: Mpamvpm5El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm6_el2: s3_4_c10_c6_6, u64: Mpamvpm6El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm7_el2: s3_4_c10_c6_7, u64: Mpamvpm7El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpmv_el2: s3_4_c10_c4_1, u64: MpamvpmvEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(mpidr: (p15, 0, c0, c0, 5), u32: Mpidr, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(mpidr_el1, u64: MpidrEl1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(mvbar: (p15, 0, c0, c12, 1), u32: Mvbar, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(nmrr: (p15, 0, c2, c10, 1), u32: Nmrr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(nsacr: (p15, 0, c1, c1, 2), u32: Nsacr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(par: (p15, 0, c7), u64: Par, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(par_el1, u64: ParEl1, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmccfiltr: (p15, 0, c15, c14, 7), u32: Pmccfiltr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmccntr: (p15, 0, c9), u64: Pmccntr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid0: (p15, 0, c12, c9, 6), u32: Pmceid0, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid1: (p15, 0, c12, c9, 7), u32: Pmceid1, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid2: (p15, 0, c14, c9, 4), u32: Pmceid2, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid3: (p15, 0, c14, c9, 5), u32: Pmceid3, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmcntenclr: (p15, 0, c12, c9, 2), u32: Pmcntenclr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmcntenset: (p15, 0, c12, c9, 1), u32: Pmcntenset, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmcr: (p15, 0, c12, c9, 0), u32: Pmcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(pmcr_el0: s3_3_c9_c12_0, u64: PmcrEl0, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmintenclr: (p15, 0, c14, c9, 2), u32: Pmintenclr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmintenset: (p15, 0, c14, c9, 1), u32: Pmintenset, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmmir: (p15, 0, c14, c9, 6), u32: Pmmir, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmovsr: (p15, 0, c12, c9, 3), u32: Pmovsr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmovsset: (p15, 0, c14, c9, 3), u32: Pmovsset, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmselr: (p15, 0, c12, c9, 5), u32: Pmselr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(pmswinc: (p15, 0, c12, c9, 4), u32: Pmswinc, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmuserenr: (p15, 0, c14, c9, 0), u32: Pmuserenr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmxevtyper: (p15, 0, c13, c9, 1), u32: Pmxevtyper, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(prrr: (p15, 0, c2, c10, 0), u32: Prrr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(revidr: (p15, 0, c0, c0, 6), u32, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(rgsr_el1: s3_0_c1_c0_5, u64: RgsrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(rmr: (p15, 0, c0, c12, 2), u32: Rmr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(rvbar: (p15, 0, c0, c12, 1), u32: Rvbar, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(scr: (p15, 0, c1, c1, 0), u32: Scr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(scr_el3, u64: ScrEl3, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(sctlr: (p15, 0, c0, c1, 0), u32: Sctlr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(sctlr2_el3: s3_6_c1_c0_3, u64: Sctlr2El3, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(sctlr_el1, u64: SctlrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(sctlr_el2, u64: SctlrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 system control register.
    sctlr_el3, u64: SctlrEl3, safe_read, fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(sdcr: (p15, 0, c3, c1, 1), u32: Sdcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(sder: (p15, 0, c1, c1, 1), u32: Sder, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(smcr_el3: s3_6_c1_c2_6, u64: SmcrEl3, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(spsr_el1, u64: SpsrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(spsr_el2, u64: SpsrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(spsr_el3, u64: SpsrEl3, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(sp_el1, u64: SpEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(sp_el2, u64: SpEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(tcmtr: (p15, 0, c0, c0, 2), u32, safe, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tcr2_el1: s3_0_c2_c0_3, u64: Tcr2El1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tcr2_el2: s3_4_c2_c0_3, u64: Tcr2El2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tcr_el1, u64: TcrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tcr_el2, u64: TcrEl2, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 translation control register.
    tcr_el3, u64: TcrEl3, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tfsre0_el1: s3_0_c5_c6_1, u64: Tfsre0El1, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tfsr_el1: s3_0_c5_c6_0, u64: TfsrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tfsr_el2: s3_4_c5_c6_0, u64: TfsrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(tlbtr: (p15, 0, c0, c0, 3), u32: Tlbtr, safe, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(tpidrprw: (p15, 0, c0, c13, 4), u32: Tpidrprw, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(tpidrro_el0, u64: TpidrroEl0, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(tpidruro: (p15, 0, c0, c13, 3), u32: Tpidruro, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(tpidrurw: (p15, 0, c0, c13, 2), u32: Tpidrurw, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(tpidr_el0, u64: TpidrEl0, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tpidr_el1, u64: TpidrEl1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tpidr_el2, u64: TpidrEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(trfcr: (p15, 0, c2, c1, 1), u32: Trfcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbcr: (p15, 0, c0, c2, 2), u32: Ttbcr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbcr2: (p15, 0, c0, c2, 3), u32: Ttbcr2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbr0: (p15, 0, c2), u64: Ttbr0, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el1, u64: Ttbr0El1, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el2, u64: Ttbr0El2, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el3, u64: Ttbr0El3, safe_read, fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbr1: (p15, 1, c2), u64: Ttbr1, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el1, u64: Ttbr1El1, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el2: s3_4_c2_c0_1, u64: Ttbr1El2, safe_read, fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vbar: (p15, 0, c0, c12, 0), u32: Vbar, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el1, u64: VbarEl1, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el2, u64: VbarEl2, safe_read, fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vdfsr: (p15, 4, c2, c5, 3), u32: Vdfsr, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vdisr: (p15, 0, c1, c12, 1), u32: Vdisr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vdisr_el2: s3_4_c12_c1_1, u64: VdisrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vmpidr: (p15, 0, c0, c0, 5), u32: Vmpidr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vmpidr_el2, u64: VmpidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vpidr: (p15, 0, c0, c0, 0), u32: Vpidr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vpidr_el2, u64: VpidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vsesr_el2: s3_4_c5_c2_3, u64: VsesrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vtcr: (p15, 4, c1, c2, 2), u32: Vtcr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vtcr_el2, u64: VtcrEl2, safe_read, fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vttbr: (p15, 6, c2), u64: Vttbr, safe_read, fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned stage 2 translation table.
    vttbr_el2, u64: VttbrEl2, safe_read, fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(zcr_el3: s3_6_c1_c2_0, u64: ZcrEl3, safe_read, fake::SYSREGS);
