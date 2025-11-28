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
    /// `ACCDATA_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AccdataEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl AccdataEl1 {
    /// Returns the value of the `ACCDATA` field.
    pub const fn accdata(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ALLINT` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Allint: u64 {
        /// `ALLINT` bit.
        const ALLINT = 1 << 13;
    }
}

bitflags! {
    /// `AMCFGR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcfgr: u32 {
        /// `HDBG` bit.
        const HDBG = 1 << 24;
    }
}

impl Amcfgr {
    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `SIZE` field.
    pub const fn size(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111111
    }

    /// Returns the value of the `NCG` field.
    pub const fn ncg(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `AMCFGR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmcfgrEl0: u64 {
        /// `HDBG` bit.
        const HDBG = 1 << 24;
    }
}

impl AmcfgrEl0 {
    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `SIZE` field.
    pub const fn size(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111111
    }

    /// Returns the value of the `NCG` field.
    pub const fn ncg(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `AMCG1IDR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcg1idrEl0: u64 {
        /// `AMEVCNTR1<n>_EL0` bit 0.
        const AMEVCNTR10_EL0 = 1 << 0;
        /// `AMEVCNTR1<n>_EL0` bit 1.
        const AMEVCNTR11_EL0 = 1 << 1;
        /// `AMEVCNTR1<n>_EL0` bit 2.
        const AMEVCNTR12_EL0 = 1 << 2;
        /// `AMEVCNTR1<n>_EL0` bit 3.
        const AMEVCNTR13_EL0 = 1 << 3;
        /// `AMEVCNTR1<n>_EL0` bit 4.
        const AMEVCNTR14_EL0 = 1 << 4;
        /// `AMEVCNTR1<n>_EL0` bit 5.
        const AMEVCNTR15_EL0 = 1 << 5;
        /// `AMEVCNTR1<n>_EL0` bit 6.
        const AMEVCNTR16_EL0 = 1 << 6;
        /// `AMEVCNTR1<n>_EL0` bit 7.
        const AMEVCNTR17_EL0 = 1 << 7;
        /// `AMEVCNTR1<n>_EL0` bit 8.
        const AMEVCNTR18_EL0 = 1 << 8;
        /// `AMEVCNTR1<n>_EL0` bit 9.
        const AMEVCNTR19_EL0 = 1 << 9;
        /// `AMEVCNTR1<n>_EL0` bit 10.
        const AMEVCNTR110_EL0 = 1 << 10;
        /// `AMEVCNTR1<n>_EL0` bit 11.
        const AMEVCNTR111_EL0 = 1 << 11;
        /// `AMEVCNTR1<n>_EL0` bit 12.
        const AMEVCNTR112_EL0 = 1 << 12;
        /// `AMEVCNTR1<n>_EL0` bit 13.
        const AMEVCNTR113_EL0 = 1 << 13;
        /// `AMEVCNTR1<n>_EL0` bit 14.
        const AMEVCNTR114_EL0 = 1 << 14;
        /// `AMEVCNTR1<n>_EL0` bit 15.
        const AMEVCNTR115_EL0 = 1 << 15;
        /// `AMEVCNTOFF1<n>_EL2` bit 0.
        const AMEVCNTOFF10_EL2 = 1 << 16;
        /// `AMEVCNTOFF1<n>_EL2` bit 1.
        const AMEVCNTOFF11_EL2 = 1 << 17;
        /// `AMEVCNTOFF1<n>_EL2` bit 2.
        const AMEVCNTOFF12_EL2 = 1 << 18;
        /// `AMEVCNTOFF1<n>_EL2` bit 3.
        const AMEVCNTOFF13_EL2 = 1 << 19;
        /// `AMEVCNTOFF1<n>_EL2` bit 4.
        const AMEVCNTOFF14_EL2 = 1 << 20;
        /// `AMEVCNTOFF1<n>_EL2` bit 5.
        const AMEVCNTOFF15_EL2 = 1 << 21;
        /// `AMEVCNTOFF1<n>_EL2` bit 6.
        const AMEVCNTOFF16_EL2 = 1 << 22;
        /// `AMEVCNTOFF1<n>_EL2` bit 7.
        const AMEVCNTOFF17_EL2 = 1 << 23;
        /// `AMEVCNTOFF1<n>_EL2` bit 8.
        const AMEVCNTOFF18_EL2 = 1 << 24;
        /// `AMEVCNTOFF1<n>_EL2` bit 9.
        const AMEVCNTOFF19_EL2 = 1 << 25;
        /// `AMEVCNTOFF1<n>_EL2` bit 10.
        const AMEVCNTOFF110_EL2 = 1 << 26;
        /// `AMEVCNTOFF1<n>_EL2` bit 11.
        const AMEVCNTOFF111_EL2 = 1 << 27;
        /// `AMEVCNTOFF1<n>_EL2` bit 12.
        const AMEVCNTOFF112_EL2 = 1 << 28;
        /// `AMEVCNTOFF1<n>_EL2` bit 13.
        const AMEVCNTOFF113_EL2 = 1 << 29;
        /// `AMEVCNTOFF1<n>_EL2` bit 14.
        const AMEVCNTOFF114_EL2 = 1 << 30;
        /// `AMEVCNTOFF1<n>_EL2` bit 15.
        const AMEVCNTOFF115_EL2 = 1 << 31;
    }
}

bitflags! {
    /// `AMCGCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcgcr: u32 {
    }
}

impl Amcgcr {
    /// Returns the value of the `CG0NC` field.
    pub const fn cg0nc(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `CG1NC` field.
    pub const fn cg1nc(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }
}

bitflags! {
    /// `AMCGCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmcgcrEl0: u64 {
    }
}

impl AmcgcrEl0 {
    /// Returns the value of the `CG0NC` field.
    pub const fn cg0nc(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `CG1NC` field.
    pub const fn cg1nc(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }
}

bitflags! {
    /// `AMCNTENCLR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `AMCNTENCLR0_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr0El0: u64 {
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

bitflags! {
    /// `AMCNTENCLR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `AMCNTENCLR1_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenclr1El0: u64 {
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

bitflags! {
    /// `AMCNTENSET0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `AMCNTENSET0_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenset0El0: u64 {
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

bitflags! {
    /// `AMCNTENSET1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `AMCNTENSET1_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcntenset1El0: u64 {
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

bitflags! {
    /// `AMCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amcr: u32 {
        /// `HDBG` bit.
        const HDBG = 1 << 10;
        /// `CG1RZ` bit.
        const CG1RZ = 1 << 17;
    }
}

bitflags! {
    /// `AMCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmcrEl0: u64 {
        /// `HDBG` bit.
        const HDBG = 1 << 10;
        /// `CG1RZ` bit.
        const CG1RZ = 1 << 17;
    }
}

bitflags! {
    /// `AMUSERENR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Amuserenr: u32 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

bitflags! {
    /// `AMUSERENR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct AmuserenrEl0: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APDAKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApdakeyhiEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApdakeyhiEl1 {
    /// Returns the value of the `APDAKeyHi` field.
    pub const fn apdakeyhi(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APDAKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApdakeyloEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApdakeyloEl1 {
    /// Returns the value of the `APDAKeyLo` field.
    pub const fn apdakeylo(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APDBKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApdbkeyhiEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApdbkeyhiEl1 {
    /// Returns the value of the `APDBKeyHi` field.
    pub const fn apdbkeyhi(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APDBKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApdbkeyloEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApdbkeyloEl1 {
    /// Returns the value of the `APDBKeyLo` field.
    pub const fn apdbkeylo(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APGAKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApgakeyhiEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApgakeyhiEl1 {
    /// Returns the value of the `APGAKeyHi` field.
    pub const fn apgakeyhi(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APGAKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApgakeyloEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApgakeyloEl1 {
    /// Returns the value of the `APGAKeyLo` field.
    pub const fn apgakeylo(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

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
    /// Returns the value of the `APIAKeyHi` field.
    pub const fn apiakeyhi(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `APIAKeyLo` field.
    pub const fn apiakeylo(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APIBKeyHi_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApibkeyhiEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApibkeyhiEl1 {
    /// Returns the value of the `APIBKeyHi` field.
    pub const fn apibkeyhi(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `APIBKeyLo_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ApibkeyloEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ApibkeyloEl1 {
    /// Returns the value of the `APIBKeyLo` field.
    pub const fn apibkeylo(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS12NSOPR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats12nsopr: u32 {
    }
}

impl Ats12nsopr {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS12NSOPW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats12nsopw: u32 {
    }
}

impl Ats12nsopw {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS12NSOUR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats12nsour: u32 {
    }
}

impl Ats12nsour {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS12NSOUW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats12nsouw: u32 {
    }
}

impl Ats12nsouw {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1CPR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1cpr: u32 {
    }
}

impl Ats1cpr {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1CPRP` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1cprp: u32 {
    }
}

impl Ats1cprp {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1CPW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1cpw: u32 {
    }
}

impl Ats1cpw {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1CPWP` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1cpwp: u32 {
    }
}

impl Ats1cpwp {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1CUR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1cur: u32 {
    }
}

impl Ats1cur {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1CUW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1cuw: u32 {
    }
}

impl Ats1cuw {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1HR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1hr: u32 {
    }
}

impl Ats1hr {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ATS1HW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ats1hw: u32 {
    }
}

impl Ats1hw {
    /// Returns the value of the `IA` field.
    pub const fn ia(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `BPIMVA` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Bpimva: u32 {
    }
}

impl Bpimva {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `BRBCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct BrbcrEl1: u64 {
        /// `E0BRE` bit.
        const E0BRE = 1 << 0;
        /// `E1BRE` bit.
        const E1BRE = 1 << 1;
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

#[cfg(feature = "el1")]
impl BrbcrEl1 {
    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `BRBCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
impl BrbcrEl2 {
    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `BRBFCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct BrbfcrEl1: u64 {
        /// `PAUSED` bit.
        const PAUSED = 1 << 7;
        /// `EnI` bit.
        const ENI = 1 << 16;
        /// `DIRECT` bit.
        const DIRECT = 1 << 17;
        /// `INDIRECT` bit.
        const INDIRECT = 1 << 18;
        /// `RTN` bit.
        const RTN = 1 << 19;
        /// `INDCALL` bit.
        const INDCALL = 1 << 20;
        /// `DIRCALL` bit.
        const DIRCALL = 1 << 21;
        /// `CONDDIR` bit.
        const CONDDIR = 1 << 22;
    }
}

#[cfg(feature = "el1")]
impl BrbfcrEl1 {
    /// Returns the value of the `BANK` field.
    pub const fn bank(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `BRBIDR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Brbidr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Brbidr0El1 {
    /// Returns the value of the `NUMREC` field.
    pub const fn numrec(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `FORMAT` field.
    pub const fn format(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `CC` field.
    pub const fn cc(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `BRBINFINJ_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct BrbinfinjEl1: u64 {
        /// `MPRED` bit.
        const MPRED = 1 << 5;
        /// `CCU` bit.
        const CCU = 1 << 46;
    }
}

#[cfg(feature = "el1")]
impl BrbinfinjEl1 {
    /// Returns the value of the `VALID` field.
    pub const fn valid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `TYPE` field.
    pub const fn type_(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111111
    }

    /// Returns the value of the `CC` field.
    pub const fn cc(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b11111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `BRBSRCINJ_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct BrbsrcinjEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl BrbsrcinjEl1 {
    /// Returns the value of the `ADDRESS` field.
    pub const fn address(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `BRBTGTINJ_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct BrbtgtinjEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl BrbtgtinjEl1 {
    /// Returns the value of the `ADDRESS` field.
    pub const fn address(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `BRBTS_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct BrbtsEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl BrbtsEl1 {
    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CCSIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ccsidr: u32 {
    }
}

impl Ccsidr {
    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }

    /// Returns the value of the `NumSets` field.
    pub const fn numsets(self) -> u16 {
        (self.bits() >> 13) as u16 & 0b111111111111111
    }
}

bitflags! {
    /// `CCSIDR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ccsidr2: u32 {
    }
}

impl Ccsidr2 {
    /// Returns the value of the `NumSets` field.
    pub const fn numsets(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CCSIDR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ccsidr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Ccsidr2El1 {
    /// Returns the value of the `NumSets` field.
    pub const fn numsets(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
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
    /// Returns the value of the `LineSize` field.
    pub const fn linesize(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

bitflags! {
    /// `CFPRCTX` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cfprctx: u32 {
        /// `GASID` bit.
        const GASID = 1 << 8;
        /// `NS` bit.
        const NS = 1 << 26;
        /// `GVMID` bit.
        const GVMID = 1 << 27;
    }
}

impl Cfprctx {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }
}

bitflags! {
    /// `CLIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Clidr: u32 {
    }
}

impl Clidr {
    /// Returns the value of the given `Ctype<n>` field.
    pub const fn ctype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        (self.bits() >> (0 + (n - 1) * 3)) as u8 & 0b111
    }

    /// Returns the value of the `LoUIS` field.
    pub const fn louis(self) -> u8 {
        (self.bits() >> 21) as u8 & 0b111
    }

    /// Returns the value of the `LoC` field.
    pub const fn loc(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b111
    }

    /// Returns the value of the `LoUU` field.
    pub const fn louu(self) -> u8 {
        (self.bits() >> 27) as u8 & 0b111
    }

    /// Returns the value of the `ICB` field.
    pub const fn icb(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11
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
    /// Returns the value of the given `Ctype<n>` field.
    pub const fn ctype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        (self.bits() >> (0 + (n - 1) * 3)) as u8 & 0b111
    }

    /// Returns the value of the `LoUIS` field.
    ///
    /// Level of Unification Inner Shareable for the cache hierarchy.
    pub const fn louis(self) -> u8 {
        (self.bits() >> 21) as u8 & 0b111
    }

    /// Returns the value of the `LoC` field.
    ///
    /// Level of Coherence for the cache hierarchy.
    pub const fn loc(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b111
    }

    /// Returns the value of the `LoUU` field.
    ///
    /// Level of Unification Uniprocessor for the cache hierarchy.
    pub const fn louu(self) -> u8 {
        (self.bits() >> 27) as u8 & 0b111
    }

    /// Returns the value of the `ICB` field.
    ///
    /// Inner cache boundary level.
    pub const fn icb(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b111
    }

    /// Returns the value of the given `Ttype<n>` field.
    pub const fn ttype(self, n: u32) -> u8 {
        assert!(n >= 1 && n < 8);
        (self.bits() >> (33 + (n - 1) * 2)) as u8 & 0b11
    }
}

bitflags! {
    /// `CNTFRQ` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cntfrq: u32 {
    }
}

impl Cntfrq {
    /// Returns the value of the `ClockFreq` field.
    pub const fn clockfreq(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
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
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHCTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
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
    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

bitflags! {
    /// `CNTHPS_CTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHPS_CTL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHPS_CVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsCvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthpsCvalEl2 {
    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHPS_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsTval: u32 {
    }
}

impl CnthpsTval {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHPS_TVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpsTvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthpsTvalEl2 {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHP_CTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHP_CTL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpCtlEl2: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHP_CVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpCvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthpCvalEl2 {
    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHP_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpTval: u32 {
    }
}

impl CnthpTval {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHP_TVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthpTvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthpTvalEl2 {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHVS_CTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHVS_CTL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsCtlEl2: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHVS_CVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsCvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthvsCvalEl2 {
    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHVS_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsTval: u32 {
    }
}

impl CnthvsTval {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHVS_TVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvsTvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthvsTvalEl2 {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHV_CTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHV_CTL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvCtlEl2: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHV_CVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvCvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthvCvalEl2 {
    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTHV_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvTval: u32 {
    }
}

impl CnthvTval {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTHV_TVAL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CnthvTvalEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CnthvTvalEl2 {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTKCTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CNTKCTL_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntkctlEl1: u64 {
        /// `EL0PCTEN` bit.
        const EL0PCTEN = 1 << 0;
        /// `EL0VCTEN` bit.
        const EL0VCTEN = 1 << 1;
        /// `EVNTEN` bit.
        const EVNTEN = 1 << 2;
        /// `EVNTDIR` bit.
        const EVNTDIR = 1 << 3;
        /// `EL0VTEN` bit.
        const EL0VTEN = 1 << 8;
        /// `EL0PTEN` bit.
        const EL0PTEN = 1 << 9;
        /// `EL1PCTEN` bit.
        const EL1PCTEN = 1 << 10;
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

#[cfg(feature = "el1")]
impl CntkctlEl1 {
    /// Returns the value of the `EVNTI` field.
    pub const fn evnti(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

bitflags! {
    /// `CNTPCTSS_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpctssEl0: u64 {
    }
}

impl CntpctssEl0 {
    /// Returns the value of the `SSPhysicalCount` field.
    pub const fn ssphysicalcount(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTPCT_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpctEl0: u64 {
    }
}

impl CntpctEl0 {
    /// Returns the value of the `PhysicalCount` field.
    pub const fn physicalcount(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CNTPOFF_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpoffEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl CntpoffEl2 {
    /// Returns the value of the `PO` field.
    pub const fn po(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CNTPS_CTL_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpsCtlEl1: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CNTPS_CVAL_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpsCvalEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl CntpsCvalEl1 {
    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CNTPS_TVAL_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpsTvalEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl CntpsTvalEl1 {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTP_CTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `CNTP_CTL_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpCtlEl0: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

bitflags! {
    /// `CNTP_CVAL_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpCvalEl0: u64 {
    }
}

impl CntpCvalEl0 {
    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTP_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpTval: u32 {
    }
}

impl CntpTval {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTP_TVAL_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntpTvalEl0: u64 {
    }
}

impl CntpTvalEl0 {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTVCTSS_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvctssEl0: u64 {
    }
}

impl CntvctssEl0 {
    /// Returns the value of the `SSVirtualCount` field.
    pub const fn ssvirtualcount(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTVCT_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvctEl0: u64 {
    }
}

impl CntvctEl0 {
    /// Returns the value of the `VirtualCount` field.
    pub const fn virtualcount(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `VOffset` field.
    pub const fn voffset(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTV_CTL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `CNTV_CTL_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvCtlEl0: u64 {
        /// `ENABLE` bit.
        const ENABLE = 1 << 0;
        /// `IMASK` bit.
        const IMASK = 1 << 1;
        /// `ISTATUS` bit.
        const ISTATUS = 1 << 2;
    }
}

bitflags! {
    /// `CNTV_CVAL_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvCvalEl0: u64 {
    }
}

impl CntvCvalEl0 {
    /// Returns the value of the `CompareValue` field.
    pub const fn comparevalue(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTV_TVAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvTval: u32 {
    }
}

impl CntvTval {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CNTV_TVAL_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CntvTvalEl0: u64 {
    }
}

impl CntvTvalEl0 {
    /// Returns the value of the `TimerValue` field.
    pub const fn timervalue(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `CONTEXTIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Contextidr: u32 {
    }
}

impl Contextidr {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
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
    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
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
    /// Returns the value of the `PROCID` field.
    pub const fn procid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `COSPRCTX` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cosprctx: u32 {
        /// `GASID` bit.
        const GASID = 1 << 8;
        /// `NS` bit.
        const NS = 1 << 26;
        /// `GVMID` bit.
        const GVMID = 1 << 27;
    }
}

impl Cosprctx {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }
}

bitflags! {
    /// `CPACR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cpacr: u32 {
        /// `TRCDIS` bit.
        const TRCDIS = 1 << 28;
        /// `ASEDIS` bit.
        const ASEDIS = 1 << 31;
    }
}

impl Cpacr {
    /// Returns the value of the `cp10` field.
    pub const fn cp10(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b11
    }

    /// Returns the value of the `cp11` field.
    pub const fn cp11(self) -> u8 {
        (self.bits() >> 22) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `CPACRMASK_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CpacrmaskEl1: u64 {
        /// `ZEN` bit.
        const ZEN = 1 << 16;
        /// `FPEN` bit.
        const FPEN = 1 << 20;
        /// `SMEN` bit.
        const SMEN = 1 << 24;
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
    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b11
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }
}

bitflags! {
    /// `CPPRCTX` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Cpprctx: u32 {
        /// `GASID` bit.
        const GASID = 1 << 8;
        /// `NS` bit.
        const NS = 1 << 26;
        /// `GVMID` bit.
        const GVMID = 1 << 27;
    }
}

impl Cpprctx {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `CPTRMASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CptrmaskEl2: u64 {
        /// `TZ` bit.
        const TZ = 1 << 8;
        /// `TFP` bit.
        const TFP = 1 << 10;
        /// `TSM` bit.
        const TSM = 1 << 12;
        /// `ZEN` bit.
        const ZEN = 1 << 16;
        /// `FPEN` bit.
        const FPEN = 1 << 20;
        /// `SMEN` bit.
        const SMEN = 1 << 24;
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
    /// Returns the value of the `ZEN` field.
    pub const fn zen(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11
    }

    /// Returns the value of the `FPEN` field.
    pub const fn fpen(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b11
    }

    /// Returns the value of the `SMEN` field.
    pub const fn smen(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
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

bitflags! {
    /// `CSSELR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Csselr: u32 {
        /// `InD` bit.
        const IND = 1 << 0;
    }
}

impl Csselr {
    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }
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
    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }
}

bitflags! {
    /// `CTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `IminLine` field.
    pub const fn iminline(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `L1Ip` field.
    pub const fn l1ip(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `DminLine` field.
    pub const fn dminline(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `ERG` field.
    pub const fn erg(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `CWG` field.
    pub const fn cwg(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
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
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `L1Ip` field.
    pub const fn l1ip(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `DminLine` field.
    ///
    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the PE.
    pub const fn dminline(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `ERG` field.
    pub const fn erg(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `CWG` field.
    pub const fn cwg(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `TminLine` field.
    pub const fn tminline(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b111111
    }
}

bitflags! {
    /// `CurrentEL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Currentel: u64 {
    }
}

impl Currentel {
    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b11
    }
}

bitflags! {
    /// `DACR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dacr: u32 {
    }
}

impl Dacr {
    /// Returns the value of the given `D<n>` field.
    pub const fn d(self, n: u32) -> u8 {
        assert!(n < 16);
        (self.bits() >> (0 + (n - 0) * 2)) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `DACR32_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dacr32El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Dacr32El2 {
    /// Returns the value of the given `D<n>` field.
    pub const fn d(self, n: u32) -> u8 {
        assert!(n < 16);
        (self.bits() >> (0 + (n - 0) * 2)) as u8 & 0b11
    }
}

bitflags! {
    /// `DAIF` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Daif: u64 {
        /// `F` bit.
        const F = 1 << 6;
        /// `I` bit.
        const I = 1 << 7;
        /// `A` bit.
        const A = 1 << 8;
        /// `D` bit.
        const D = 1 << 9;
    }
}

bitflags! {
    /// `DBGAUTHSTATUS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgauthstatus: u32 {
    }
}

impl Dbgauthstatus {
    /// Returns the value of the `NSID` field.
    pub const fn nsid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `NSNID` field.
    pub const fn nsnid(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b11
    }

    /// Returns the value of the `SID` field.
    pub const fn sid(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b11
    }

    /// Returns the value of the `SNID` field.
    pub const fn snid(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `DBGAUTHSTATUS_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DbgauthstatusEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl DbgauthstatusEl1 {
    /// Returns the value of the `NSID` field.
    pub const fn nsid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `NSNID` field.
    pub const fn nsnid(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b11
    }

    /// Returns the value of the `SID` field.
    pub const fn sid(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b11
    }

    /// Returns the value of the `SNID` field.
    pub const fn snid(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `RLID` field.
    pub const fn rlid(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `RLNID` field.
    pub const fn rlnid(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `RTID` field.
    pub const fn rtid(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }

    /// Returns the value of the `RTNID` field.
    pub const fn rtnid(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b11
    }
}

bitflags! {
    /// `DBGCLAIMCLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el1")]
bitflags! {
    /// `DBGCLAIMCLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DbgclaimclrEl1: u64 {
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

bitflags! {
    /// `DBGCLAIMSET` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el1")]
bitflags! {
    /// `DBGCLAIMSET_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DbgclaimsetEl1: u64 {
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

bitflags! {
    /// `DBGDCCINT` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdccint: u32 {
        /// `TX` bit.
        const TX = 1 << 29;
        /// `RX` bit.
        const RX = 1 << 30;
    }
}

bitflags! {
    /// `DBGDEVID` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdevid: u32 {
    }
}

impl Dbgdevid {
    /// Returns the value of the `PCSample` field.
    pub const fn pcsample(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `WPAddrMask` field.
    pub const fn wpaddrmask(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `BPAddrMask` field.
    pub const fn bpaddrmask(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `VectorCatch` field.
    pub const fn vectorcatch(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `VirtExtns` field.
    pub const fn virtextns(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `DoubleLock` field.
    pub const fn doublelock(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `AuxRegs` field.
    pub const fn auxregs(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `CIDMask` field.
    pub const fn cidmask(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `DBGDEVID1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdevid1: u32 {
    }
}

impl Dbgdevid1 {
    /// Returns the value of the `PCSROffset` field.
    pub const fn pcsroffset(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

bitflags! {
    /// `DBGDIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `Version` field.
    pub const fn version(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `DBGDRAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdrar: u32 {
    }
}

impl Dbgdrar {
    /// Returns the value of the `Valid` field.
    pub const fn valid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `ROMADDR[47:12]` field.
    pub const fn romaddr_47_12(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b111111111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGDSCRext` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `MOE` field.
    pub const fn moe(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b1111
    }

    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        (self.bits() >> 22) as u8 & 0b11
    }
}

bitflags! {
    /// `DBGDSCRint` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `MOE` field.
    pub const fn moe(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b1111
    }
}

bitflags! {
    /// `DBGDTRRX_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DbgdtrrxEl0: u64 {
    }
}

impl DbgdtrrxEl0 {
    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGDTRRXext` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrrxext: u32 {
    }
}

impl Dbgdtrrxext {
    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGDTRRXint` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrrxint: u32 {
    }
}

impl Dbgdtrrxint {
    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGDTRTX_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DbgdtrtxEl0: u64 {
    }
}

impl DbgdtrtxEl0 {
    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGDTRTXext` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrtxext: u32 {
    }
}

impl Dbgdtrtxext {
    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGDTRTXint` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgdtrtxint: u32 {
    }
}

impl Dbgdtrtxint {
    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGDTR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DbgdtrEl0: u64 {
    }
}

impl DbgdtrEl0 {
    /// Returns the value of the `LowWord` field.
    pub const fn lowword(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }

    /// Returns the value of the `HighWord` field.
    pub const fn highword(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGOSDLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgosdlr: u32 {
        /// `DLK` bit.
        const DLK = 1 << 0;
    }
}

bitflags! {
    /// `DBGOSECCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoseccr: u32 {
    }
}

impl Dbgoseccr {
    /// Returns the value of the `EDECCR` field.
    pub const fn edeccr(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGOSLAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoslar: u32 {
    }
}

impl Dbgoslar {
    /// Returns the value of the `OSLA` field.
    pub const fn osla(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DBGOSLSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgoslsr: u32 {
        /// `OSLK` bit.
        const OSLK = 1 << 1;
        /// `nTT` bit.
        const NTT = 1 << 2;
    }
}

bitflags! {
    /// `DBGPRCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgprcr: u32 {
        /// `CORENPDRQ` bit.
        const CORENPDRQ = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `DBGPRCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DbgprcrEl1: u64 {
        /// `CORENPDRQ` bit.
        const CORENPDRQ = 1 << 0;
    }
}

bitflags! {
    /// `DBGVCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `DBGVCR32_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dbgvcr32El2: u64 {
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

bitflags! {
    /// `DCCIMVAC` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dccimvac: u32 {
    }
}

impl Dccimvac {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DCCISW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dccisw: u32 {
    }
}

impl Dccisw {
    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `SetWay` field.
    pub const fn setway(self) -> u32 {
        (self.bits() >> 4) as u32 & 0b1111111111111111111111111111
    }
}

bitflags! {
    /// `DCCMVAC` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dccmvac: u32 {
    }
}

impl Dccmvac {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DCCMVAU` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dccmvau: u32 {
    }
}

impl Dccmvau {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DCCSW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dccsw: u32 {
    }
}

impl Dccsw {
    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `SetWay` field.
    pub const fn setway(self) -> u32 {
        (self.bits() >> 4) as u32 & 0b1111111111111111111111111111
    }
}

bitflags! {
    /// `DCIMVAC` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dcimvac: u32 {
    }
}

impl Dcimvac {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DCISW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dcisw: u32 {
    }
}

impl Dcisw {
    /// Returns the value of the `Level` field.
    pub const fn level(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `SetWay` field.
    pub const fn setway(self) -> u32 {
        (self.bits() >> 4) as u32 & 0b1111111111111111111111111111
    }
}

bitflags! {
    /// `DCZID_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DczidEl0: u64 {
        /// `DZP` bit.
        const DZP = 1 << 4;
    }
}

impl DczidEl0 {
    /// Returns the value of the `BS` field.
    pub const fn bs(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `TBS` field.
    pub const fn tbs(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b1111
    }
}

bitflags! {
    /// `DFAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dfar: u32 {
    }
}

impl Dfar {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DFSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `Domain` field.
    pub const fn domain(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }
}

bitflags! {
    /// `DISR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `DFSC` field.
    pub const fn dfsc(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
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
    /// Returns the value of the `DFSC` field.
    pub const fn dfsc(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b111
    }

    /// Returns the value of the `WU` field.
    pub const fn wu(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11
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
    /// `DLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dlr: u32 {
    }
}

impl Dlr {
    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `DLR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DlrEl0: u64 {
    }
}

impl DlrEl0 {
    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `DPOCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DpocrEl0: u64 {
        /// `DPOE2` bit.
        const DPOE2 = 1 << 0;
    }
}

impl DpocrEl0 {
    /// Returns the value of the `FPOIndex` field.
    pub const fn fpoindex(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b1111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `DPOTBR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dpotbr0El1: u64 {
        /// `DPOTD` bit.
        const DPOTD = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 1;
    }
}

#[cfg(feature = "el1")]
impl Dpotbr0El1 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 9) as u64 & 0b1111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `DPOTBR0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dpotbr0El2: u64 {
        /// `DPOTD` bit.
        const DPOTD = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 1;
    }
}

#[cfg(feature = "el2")]
impl Dpotbr0El2 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 9) as u64 & 0b1111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `DPOTBR0_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dpotbr0El3: u64 {
        /// `DPOTD` bit.
        const DPOTD = 1 << 0;
    }
}

#[cfg(feature = "el3")]
impl Dpotbr0El3 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 9) as u64 & 0b1111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `DPOTBR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dpotbr1El1: u64 {
        /// `DPOTD` bit.
        const DPOTD = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 1;
    }
}

#[cfg(feature = "el1")]
impl Dpotbr1El1 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 9) as u64 & 0b1111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `DPOTBR1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dpotbr1El2: u64 {
        /// `DPOTD` bit.
        const DPOTD = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 1;
    }
}

#[cfg(feature = "el2")]
impl Dpotbr1El2 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 9) as u64 & 0b1111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `DSPSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `M[4:0]` field.
    pub const fn m_4_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }
}

bitflags! {
    /// `DSPSR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dspsr2: u32 {
        /// `PPEND` bit.
        const PPEND = 1 << 1;
        /// `UINJ` bit.
        const UINJ = 1 << 4;
    }
}

bitflags! {
    /// `DSPSR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct DspsrEl0: u64 {
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

impl DspsrEl0 {
    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }
}

bitflags! {
    /// `DTLBIASID` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dtlbiasid: u32 {
    }
}

impl Dtlbiasid {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

bitflags! {
    /// `DTLBIMVA` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dtlbimva: u32 {
    }
}

impl Dtlbimva {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `DVPRCTX` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Dvprctx: u32 {
        /// `GASID` bit.
        const GASID = 1 << 8;
        /// `NS` bit.
        const NS = 1 << 26;
        /// `GVMID` bit.
        const GVMID = 1 << 27;
    }
}

impl Dvprctx {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `EL` field.
    pub const fn el(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }
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
    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ELR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ElrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl ElrEl3 {
    /// Returns the value of the `ADDR` field.
    pub const fn addr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERRIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erridr: u32 {
    }
}

impl Erridr {
    /// Returns the value of the `NUM` field.
    pub const fn num(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERRIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErridrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErridrEl1 {
    /// Returns the value of the `NUM` field.
    pub const fn num(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

bitflags! {
    /// `ERRSELR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Errselr: u32 {
    }
}

impl Errselr {
    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERRSELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErrselrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErrselrEl1 {
    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

bitflags! {
    /// `ERXADDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxaddr: u32 {
    }
}

impl Erxaddr {
    /// Returns the value of the `ERRnADDRlo` field.
    pub const fn errnaddrlo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXADDR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxaddr2: u32 {
    }
}

impl Erxaddr2 {
    /// Returns the value of the `ERRnADDRhi` field.
    pub const fn errnaddrhi(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXADDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxaddrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErxaddrEl1 {
    /// Returns the value of the `ERRnADDR` field.
    pub const fn errnaddr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxctlr: u32 {
    }
}

impl Erxctlr {
    /// Returns the value of the `ERRnCTLRlo` field.
    pub const fn errnctlrlo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXCTLR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxctlr2: u32 {
    }
}

impl Erxctlr2 {
    /// Returns the value of the `ERRnCTLRhi` field.
    pub const fn errnctlrhi(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXCTLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxctlrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErxctlrEl1 {
    /// Returns the value of the `ERRnCTLR` field.
    pub const fn errnctlr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXFR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxfr: u32 {
    }
}

impl Erxfr {
    /// Returns the value of the `ERRnFRlo` field.
    pub const fn errnfrlo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXFR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxfr2: u32 {
    }
}

impl Erxfr2 {
    /// Returns the value of the `ERRnFRhi` field.
    pub const fn errnfrhi(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXFR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxfrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErxfrEl1 {
    /// Returns the value of the `ERRnFR` field.
    pub const fn errnfr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXGSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxgsrEl1: u64 {
        /// `S<q>` bit 0.
        const S0 = 1 << 0;
        /// `S<q>` bit 1.
        const S1 = 1 << 1;
        /// `S<q>` bit 2.
        const S2 = 1 << 2;
        /// `S<q>` bit 3.
        const S3 = 1 << 3;
        /// `S<q>` bit 4.
        const S4 = 1 << 4;
        /// `S<q>` bit 5.
        const S5 = 1 << 5;
        /// `S<q>` bit 6.
        const S6 = 1 << 6;
        /// `S<q>` bit 7.
        const S7 = 1 << 7;
        /// `S<q>` bit 8.
        const S8 = 1 << 8;
        /// `S<q>` bit 9.
        const S9 = 1 << 9;
        /// `S<q>` bit 10.
        const S10 = 1 << 10;
        /// `S<q>` bit 11.
        const S11 = 1 << 11;
        /// `S<q>` bit 12.
        const S12 = 1 << 12;
        /// `S<q>` bit 13.
        const S13 = 1 << 13;
        /// `S<q>` bit 14.
        const S14 = 1 << 14;
        /// `S<q>` bit 15.
        const S15 = 1 << 15;
        /// `S<q>` bit 16.
        const S16 = 1 << 16;
        /// `S<q>` bit 17.
        const S17 = 1 << 17;
        /// `S<q>` bit 18.
        const S18 = 1 << 18;
        /// `S<q>` bit 19.
        const S19 = 1 << 19;
        /// `S<q>` bit 20.
        const S20 = 1 << 20;
        /// `S<q>` bit 21.
        const S21 = 1 << 21;
        /// `S<q>` bit 22.
        const S22 = 1 << 22;
        /// `S<q>` bit 23.
        const S23 = 1 << 23;
        /// `S<q>` bit 24.
        const S24 = 1 << 24;
        /// `S<q>` bit 25.
        const S25 = 1 << 25;
        /// `S<q>` bit 26.
        const S26 = 1 << 26;
        /// `S<q>` bit 27.
        const S27 = 1 << 27;
        /// `S<q>` bit 28.
        const S28 = 1 << 28;
        /// `S<q>` bit 29.
        const S29 = 1 << 29;
        /// `S<q>` bit 30.
        const S30 = 1 << 30;
        /// `S<q>` bit 31.
        const S31 = 1 << 31;
        /// `S<q>` bit 32.
        const S32 = 1 << 32;
        /// `S<q>` bit 33.
        const S33 = 1 << 33;
        /// `S<q>` bit 34.
        const S34 = 1 << 34;
        /// `S<q>` bit 35.
        const S35 = 1 << 35;
        /// `S<q>` bit 36.
        const S36 = 1 << 36;
        /// `S<q>` bit 37.
        const S37 = 1 << 37;
        /// `S<q>` bit 38.
        const S38 = 1 << 38;
        /// `S<q>` bit 39.
        const S39 = 1 << 39;
        /// `S<q>` bit 40.
        const S40 = 1 << 40;
        /// `S<q>` bit 41.
        const S41 = 1 << 41;
        /// `S<q>` bit 42.
        const S42 = 1 << 42;
        /// `S<q>` bit 43.
        const S43 = 1 << 43;
        /// `S<q>` bit 44.
        const S44 = 1 << 44;
        /// `S<q>` bit 45.
        const S45 = 1 << 45;
        /// `S<q>` bit 46.
        const S46 = 1 << 46;
        /// `S<q>` bit 47.
        const S47 = 1 << 47;
        /// `S<q>` bit 48.
        const S48 = 1 << 48;
        /// `S<q>` bit 49.
        const S49 = 1 << 49;
        /// `S<q>` bit 50.
        const S50 = 1 << 50;
        /// `S<q>` bit 51.
        const S51 = 1 << 51;
        /// `S<q>` bit 52.
        const S52 = 1 << 52;
        /// `S<q>` bit 53.
        const S53 = 1 << 53;
        /// `S<q>` bit 54.
        const S54 = 1 << 54;
        /// `S<q>` bit 55.
        const S55 = 1 << 55;
        /// `S<q>` bit 56.
        const S56 = 1 << 56;
        /// `S<q>` bit 57.
        const S57 = 1 << 57;
        /// `S<q>` bit 58.
        const S58 = 1 << 58;
        /// `S<q>` bit 59.
        const S59 = 1 << 59;
        /// `S<q>` bit 60.
        const S60 = 1 << 60;
        /// `S<q>` bit 61.
        const S61 = 1 << 61;
        /// `S<q>` bit 62.
        const S62 = 1 << 62;
        /// `S<q>` bit 63.
        const S63 = 1 << 63;
    }
}

bitflags! {
    /// `ERXMISC0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc0: u32 {
    }
}

impl Erxmisc0 {
    /// Returns the value of the `ERRnMISC0lo` field.
    pub const fn errnmisc0lo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXMISC0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Erxmisc0El1 {
    /// Returns the value of the `ERRnMISC0` field.
    pub const fn errnmisc0(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXMISC1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc1: u32 {
    }
}

impl Erxmisc1 {
    /// Returns the value of the `ERRnMISC0hi` field.
    pub const fn errnmisc0hi(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXMISC1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Erxmisc1El1 {
    /// Returns the value of the `ERRnMISC1` field.
    pub const fn errnmisc1(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXMISC2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc2: u32 {
    }
}

impl Erxmisc2 {
    /// Returns the value of the `ERRnMISC1lo` field.
    pub const fn errnmisc1lo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXMISC2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Erxmisc2El1 {
    /// Returns the value of the `ERRnMISC2` field.
    pub const fn errnmisc2(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXMISC3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc3: u32 {
    }
}

impl Erxmisc3 {
    /// Returns the value of the `ERRnMISC1hi` field.
    pub const fn errnmisc1hi(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXMISC3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc3El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Erxmisc3El1 {
    /// Returns the value of the `ERRnMISC3` field.
    pub const fn errnmisc3(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXMISC4` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc4: u32 {
    }
}

impl Erxmisc4 {
    /// Returns the value of the `ERRnMISC2lo` field.
    pub const fn errnmisc2lo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXMISC5` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc5: u32 {
    }
}

impl Erxmisc5 {
    /// Returns the value of the `ERRnMISC2hi` field.
    pub const fn errnmisc2hi(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXMISC6` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc6: u32 {
    }
}

impl Erxmisc6 {
    /// Returns the value of the `ERRnMISC3lo` field.
    pub const fn errnmisc3lo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXMISC7` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxmisc7: u32 {
    }
}

impl Erxmisc7 {
    /// Returns the value of the `ERRnMISC3hi` field.
    pub const fn errnmisc3hi(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXPFGCDN_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxpfgcdnEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErxpfgcdnEl1 {
    /// Returns the value of the `ERRnPFGCDN` field.
    pub const fn errnpfgcdn(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXPFGCTL_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxpfgctlEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErxpfgctlEl1 {
    /// Returns the value of the `ERRnPFGCTL` field.
    pub const fn errnpfgctl(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXPFGF_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxpfgfEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErxpfgfEl1 {
    /// Returns the value of the `ERRnPFGF` field.
    pub const fn errnpfgf(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ERXSTATUS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Erxstatus: u32 {
    }
}

impl Erxstatus {
    /// Returns the value of the `ERRnSTATUSlo` field.
    pub const fn errnstatuslo(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ERXSTATUS_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ErxstatusEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ErxstatusEl1 {
    /// Returns the value of the `ERRnSTATUS` field.
    pub const fn errnstatus(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
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
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
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
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `ISS2` field.
    pub const fn iss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
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
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `FAR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct FarEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl FarEl3 {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `FGWTE3_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `FPCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fpcr: u64 {
        /// `FIZ` bit.
        const FIZ = 1 << 0;
        /// `AH` bit.
        const AH = 1 << 1;
        /// `NEP` bit.
        const NEP = 1 << 2;
        /// `IOE` bit.
        const IOE = 1 << 8;
        /// `DZE` bit.
        const DZE = 1 << 9;
        /// `OFE` bit.
        const OFE = 1 << 10;
        /// `UFE` bit.
        const UFE = 1 << 11;
        /// `IXE` bit.
        const IXE = 1 << 12;
        /// `EBF` bit.
        const EBF = 1 << 13;
        /// `IDE` bit.
        const IDE = 1 << 15;
        /// `FZ16` bit.
        const FZ16 = 1 << 19;
        /// `FZ` bit.
        const FZ = 1 << 24;
        /// `DN` bit.
        const DN = 1 << 25;
        /// `AHP` bit.
        const AHP = 1 << 26;
    }
}

impl Fpcr {
    /// Returns the value of the `Len` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111
    }

    /// Returns the value of the `Stride` field.
    pub const fn stride(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b11
    }

    /// Returns the value of the `RMode` field.
    pub const fn rmode(self) -> u8 {
        (self.bits() >> 22) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `FPEXC32_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fpexc32El2: u64 {
        /// `IOF` bit.
        const IOF = 1 << 0;
        /// `DZF` bit.
        const DZF = 1 << 1;
        /// `OFF` bit.
        const OFF = 1 << 2;
        /// `UFF` bit.
        const UFF = 1 << 3;
        /// `IXF` bit.
        const IXF = 1 << 4;
        /// `IDF` bit.
        const IDF = 1 << 7;
        /// `TFV` bit.
        const TFV = 1 << 26;
        /// `VV` bit.
        const VV = 1 << 27;
        /// `FP2V` bit.
        const FP2V = 1 << 28;
        /// `DEX` bit.
        const DEX = 1 << 29;
        /// `EN` bit.
        const EN = 1 << 30;
        /// `EX` bit.
        const EX = 1 << 31;
    }
}

#[cfg(feature = "el2")]
impl Fpexc32El2 {
    /// Returns the value of the `VECITR` field.
    pub const fn vecitr(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111
    }
}

bitflags! {
    /// `FPMR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fpmr: u64 {
        /// `OSM` bit.
        const OSM = 1 << 14;
        /// `OSC` bit.
        const OSC = 1 << 15;
    }
}

impl Fpmr {
    /// Returns the value of the `F8S1` field.
    pub const fn f8s1(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }

    /// Returns the value of the `F8S2` field.
    pub const fn f8s2(self) -> u8 {
        (self.bits() >> 3) as u8 & 0b111
    }

    /// Returns the value of the `F8D` field.
    pub const fn f8d(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b111
    }

    /// Returns the value of the `LSCALE` field.
    pub const fn lscale(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111111
    }

    /// Returns the value of the `NSCALE` field.
    pub const fn nscale(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
    }

    /// Returns the value of the `LSCALE2` field.
    pub const fn lscale2(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b111111
    }
}

bitflags! {
    /// `FPSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Fpsr: u64 {
        /// `IOC` bit.
        const IOC = 1 << 0;
        /// `DZC` bit.
        const DZC = 1 << 1;
        /// `OFC` bit.
        const OFC = 1 << 2;
        /// `UFC` bit.
        const UFC = 1 << 3;
        /// `IXC` bit.
        const IXC = 1 << 4;
        /// `IDC` bit.
        const IDC = 1 << 7;
        /// `QC` bit.
        const QC = 1 << 27;
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
    /// Returns the value of the `Exclude` field.
    pub const fn exclude(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `GCSCRE0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Gcscre0El1: u64 {
        /// `PCRSEL` bit.
        const PCRSEL = 1 << 0;
        /// `RVCHKEN` bit.
        const RVCHKEN = 1 << 5;
        /// `PUSHMEn` bit.
        const PUSHMEN = 1 << 8;
        /// `STREn` bit.
        const STREN = 1 << 9;
        /// `nTR` bit.
        const NTR = 1 << 10;
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

#[cfg(feature = "el3")]
bitflags! {
    /// `GCSCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcscrEl3: u64 {
        /// `PCRSEL` bit.
        const PCRSEL = 1 << 0;
        /// `RVCHKEN` bit.
        const RVCHKEN = 1 << 5;
        /// `EXLOCKEN` bit.
        const EXLOCKEN = 1 << 6;
        /// `PUSHMEn` bit.
        const PUSHMEN = 1 << 8;
        /// `STREn` bit.
        const STREN = 1 << 9;
    }
}

bitflags! {
    /// `GCSPR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcsprEl0: u64 {
    }
}

impl GcsprEl0 {
    /// Returns the value of the `PTR[63:3]` field.
    pub const fn ptr_63_3(self) -> u64 {
        (self.bits() >> 3) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `GCSPR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcsprEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl GcsprEl1 {
    /// Returns the value of the `PTR[63:3]` field.
    pub const fn ptr_63_3(self) -> u64 {
        (self.bits() >> 3) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `GCSPR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcsprEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl GcsprEl2 {
    /// Returns the value of the `PTR[63:3]` field.
    pub const fn ptr_63_3(self) -> u64 {
        (self.bits() >> 3) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `GCSPR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GcsprEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl GcsprEl3 {
    /// Returns the value of the `PTR[63:3]` field.
    pub const fn ptr_63_3(self) -> u64 {
        (self.bits() >> 3) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `GMID_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GmidEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl GmidEl1 {
    /// Returns the value of the `BS` field.
    pub const fn bs(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `GPCBW_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GpcbwEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl GpcbwEl3 {
    /// Returns the value of the `BWADDR` field.
    pub const fn bwaddr(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111
    }

    /// Returns the value of the `BWSTRIDE` field.
    pub const fn bwstride(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111
    }

    /// Returns the value of the `BWSIZE` field.
    pub const fn bwsize(self) -> u8 {
        (self.bits() >> 37) as u8 & 0b111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `GPCCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `PPS` field.
    pub const fn pps(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }

    /// Returns the value of the `IRGN` field.
    pub const fn irgn(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN` field.
    pub const fn orgn(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `PGS` field.
    pub const fn pgs(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `L0GPTSZ` field.
    pub const fn l0gptsz(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `GPTBR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct GptbrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl GptbrEl3 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111
    }

    /// Returns the value of the `BADDR[43:40]` field.
    pub const fn baddr_43_40(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HACDBSBR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HacdbsbrEl2: u64 {
        /// `EN` bit.
        const EN = 1 << 11;
    }
}

#[cfg(feature = "el2")]
impl HacdbsbrEl2 {
    /// Returns the value of the `SZ` field.
    pub const fn sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b11111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HACDBSCONS_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HacdbsconsEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl HacdbsconsEl2 {
    /// Returns the value of the `INDEX` field.
    pub const fn index(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111
    }

    /// Returns the value of the `ERR_REASON` field.
    pub const fn err_reason(self) -> u8 {
        (self.bits() >> 62) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HAFGRTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `HCPTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `HCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `BSU` field.
    pub const fn bsu(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }
}

bitflags! {
    /// `HCR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `HCRMASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HcrmaskEl2: u64 {
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
        /// `BSU` bit.
        const BSU = 1 << 10;
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
        /// `TGE` bit.
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
        /// `TWEDEL` bit.
        const TWEDEL = 1 << 60;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HCRXMASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HcrxmaskEl2: u64 {
        /// `EnAS0` bit.
        const ENAS0 = 1 << 0;
        /// `EnALS` bit.
        const ENALS = 1 << 1;
        /// `EnASR` bit.
        const ENASR = 1 << 2;
        /// `FnXS` bit.
        const FNXS = 1 << 3;
        /// `FGTnXS` bit.
        const FGTNXS = 1 << 4;
        /// `SMPME` bit.
        const SMPME = 1 << 5;
        /// `TALLINT` bit.
        const TALLINT = 1 << 6;
        /// `VINMI` bit.
        const VINMI = 1 << 7;
        /// `VFNMI` bit.
        const VFNMI = 1 << 8;
        /// `CMOW` bit.
        const CMOW = 1 << 9;
        /// `MCE2` bit.
        const MCE2 = 1 << 10;
        /// `MSCEn` bit.
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
    /// Returns the value of the `BSU` field.
    pub const fn bsu(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HDBSSBR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HdbssbrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl HdbssbrEl2 {
    /// Returns the value of the `SZ` field.
    pub const fn sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b11111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HDBSSPROD_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct HdbssprodEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl HdbssprodEl2 {
    /// Returns the value of the `INDEX` field.
    pub const fn index(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111
    }

    /// Returns the value of the `FSC` field.
    pub const fn fsc(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }
}

bitflags! {
    /// `HDCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `HPMN` field.
    pub const fn hpmn(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

bitflags! {
    /// `HDFAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hdfar: u32 {
    }
}

impl Hdfar {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
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
bitflags! {
    /// `HDFGRTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
bitflags! {
    /// `HDFGWTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
bitflags! {
    /// `HFGITR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        (self.bits() >> 18) as u8 & 0b11
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        (self.bits() >> 31) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `HFGRTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `nFGDTn_EL1` field.
    pub const fn nfgdtn_el1(self) -> u8 {
        (self.bits() >> 18) as u8 & 0b11
    }

    /// Returns the value of the `nAFGDTn_EL1` field.
    pub const fn nafgdtn_el1(self) -> u8 {
        (self.bits() >> 31) as u8 & 0b11
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

bitflags! {
    /// `HIFAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hifar: u32 {
    }
}

impl Hifar {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `HMAIR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hmair0: u32 {
    }
}

impl Hmair0 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 4);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
    }
}

bitflags! {
    /// `HMAIR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hmair1: u32 {
    }
}

impl Hmair1 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n >= 4 && n < 8);
        (self.bits() >> (0 + (n - 4) * 8)) as u8 & 0b11111111
    }
}

bitflags! {
    /// `HPFAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hpfar: u32 {
    }
}

impl Hpfar {
    /// Returns the value of the `FIPA[39:12]` field.
    pub const fn fipa_39_12(self) -> u32 {
        (self.bits() >> 4) as u32 & 0b1111111111111111111111111111
    }
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
    /// Returns the value of the `FIPA` field.
    pub const fn fipa(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b11111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `HRMR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hrmr: u32 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

bitflags! {
    /// `HSCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `HSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hsr: u32 {
        /// `IL` bit.
        const IL = 1 << 25;
    }
}

impl Hsr {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }
}

bitflags! {
    /// `HTCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }
}

bitflags! {
    /// `HTPIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Htpidr: u32 {
    }
}

impl Htpidr {
    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `HTRFCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b11
    }
}

bitflags! {
    /// `HVBAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Hvbar: u32 {
    }
}

impl Hvbar {
    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        (self.bits() >> 5) as u32 & 0b111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_APR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccAprEl1: u64 {
        /// `P<x>` bit 0.
        const P0 = 1 << 0;
        /// `P<x>` bit 1.
        const P1 = 1 << 1;
        /// `P<x>` bit 2.
        const P2 = 1 << 2;
        /// `P<x>` bit 3.
        const P3 = 1 << 3;
        /// `P<x>` bit 4.
        const P4 = 1 << 4;
        /// `P<x>` bit 5.
        const P5 = 1 << 5;
        /// `P<x>` bit 6.
        const P6 = 1 << 6;
        /// `P<x>` bit 7.
        const P7 = 1 << 7;
        /// `P<x>` bit 8.
        const P8 = 1 << 8;
        /// `P<x>` bit 9.
        const P9 = 1 << 9;
        /// `P<x>` bit 10.
        const P10 = 1 << 10;
        /// `P<x>` bit 11.
        const P11 = 1 << 11;
        /// `P<x>` bit 12.
        const P12 = 1 << 12;
        /// `P<x>` bit 13.
        const P13 = 1 << 13;
        /// `P<x>` bit 14.
        const P14 = 1 << 14;
        /// `P<x>` bit 15.
        const P15 = 1 << 15;
        /// `P<x>` bit 16.
        const P16 = 1 << 16;
        /// `P<x>` bit 17.
        const P17 = 1 << 17;
        /// `P<x>` bit 18.
        const P18 = 1 << 18;
        /// `P<x>` bit 19.
        const P19 = 1 << 19;
        /// `P<x>` bit 20.
        const P20 = 1 << 20;
        /// `P<x>` bit 21.
        const P21 = 1 << 21;
        /// `P<x>` bit 22.
        const P22 = 1 << 22;
        /// `P<x>` bit 23.
        const P23 = 1 << 23;
        /// `P<x>` bit 24.
        const P24 = 1 << 24;
        /// `P<x>` bit 25.
        const P25 = 1 << 25;
        /// `P<x>` bit 26.
        const P26 = 1 << 26;
        /// `P<x>` bit 27.
        const P27 = 1 << 27;
        /// `P<x>` bit 28.
        const P28 = 1 << 28;
        /// `P<x>` bit 29.
        const P29 = 1 << 29;
        /// `P<x>` bit 30.
        const P30 = 1 << 30;
        /// `P<x>` bit 31.
        const P31 = 1 << 31;
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ICC_APR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccAprEl3: u64 {
        /// `P<x>` bit 0.
        const P0 = 1 << 0;
        /// `P<x>` bit 1.
        const P1 = 1 << 1;
        /// `P<x>` bit 2.
        const P2 = 1 << 2;
        /// `P<x>` bit 3.
        const P3 = 1 << 3;
        /// `P<x>` bit 4.
        const P4 = 1 << 4;
        /// `P<x>` bit 5.
        const P5 = 1 << 5;
        /// `P<x>` bit 6.
        const P6 = 1 << 6;
        /// `P<x>` bit 7.
        const P7 = 1 << 7;
        /// `P<x>` bit 8.
        const P8 = 1 << 8;
        /// `P<x>` bit 9.
        const P9 = 1 << 9;
        /// `P<x>` bit 10.
        const P10 = 1 << 10;
        /// `P<x>` bit 11.
        const P11 = 1 << 11;
        /// `P<x>` bit 12.
        const P12 = 1 << 12;
        /// `P<x>` bit 13.
        const P13 = 1 << 13;
        /// `P<x>` bit 14.
        const P14 = 1 << 14;
        /// `P<x>` bit 15.
        const P15 = 1 << 15;
        /// `P<x>` bit 16.
        const P16 = 1 << 16;
        /// `P<x>` bit 17.
        const P17 = 1 << 17;
        /// `P<x>` bit 18.
        const P18 = 1 << 18;
        /// `P<x>` bit 19.
        const P19 = 1 << 19;
        /// `P<x>` bit 20.
        const P20 = 1 << 20;
        /// `P<x>` bit 21.
        const P21 = 1 << 21;
        /// `P<x>` bit 22.
        const P22 = 1 << 22;
        /// `P<x>` bit 23.
        const P23 = 1 << 23;
        /// `P<x>` bit 24.
        const P24 = 1 << 24;
        /// `P<x>` bit 25.
        const P25 = 1 << 25;
        /// `P<x>` bit 26.
        const P26 = 1 << 26;
        /// `P<x>` bit 27.
        const P27 = 1 << 27;
        /// `P<x>` bit 28.
        const P28 = 1 << 28;
        /// `P<x>` bit 29.
        const P29 = 1 << 29;
        /// `P<x>` bit 30.
        const P30 = 1 << 30;
        /// `P<x>` bit 31.
        const P31 = 1 << 31;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_ASGI1R_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccAsgi1rEl1: u64 {
        /// `IRM` bit.
        const IRM = 1 << 40;
    }
}

#[cfg(feature = "el1")]
impl IccAsgi1rEl1 {
    /// Returns the value of the `TargetList` field.
    pub const fn targetlist(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }

    /// Returns the value of the `RS` field.
    pub const fn rs(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111111
    }
}

bitflags! {
    /// `ICC_BPR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccBpr0: u32 {
    }
}

impl IccBpr0 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_BPR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccBpr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccBpr0El1 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

bitflags! {
    /// `ICC_BPR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccBpr1: u32 {
    }
}

impl IccBpr1 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_BPR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccBpr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccBpr1El1 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_CR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccCr0El1: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
        /// `LINK` bit.
        const LINK = 1 << 1;
        /// `LINK_IDLE` bit.
        const LINK_IDLE = 1 << 2;
        /// `PID` bit.
        const PID = 1 << 38;
    }
}

#[cfg(feature = "el1")]
impl IccCr0El1 {
    /// Returns the value of the `IPPT` field.
    pub const fn ippt(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ICC_CR0_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccCr0El3: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
        /// `LINK` bit.
        const LINK = 1 << 1;
        /// `LINK_IDLE` bit.
        const LINK_IDLE = 1 << 2;
    }
}

#[cfg(feature = "el3")]
impl IccCr0El3 {
    /// Returns the value of the `PID` field.
    pub const fn pid(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11
    }
}

bitflags! {
    /// `ICC_CTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccCtlr: u32 {
        /// `CBPR` bit.
        const CBPR = 1 << 0;
        /// `EOImode` bit.
        const EOIMODE = 1 << 1;
        /// `PMHE` bit.
        const PMHE = 1 << 6;
        /// `SEIS` bit.
        const SEIS = 1 << 14;
        /// `A3V` bit.
        const A3V = 1 << 15;
        /// `RSS` bit.
        const RSS = 1 << 18;
        /// `ExtRange` bit.
        const EXTRANGE = 1 << 19;
    }
}

impl IccCtlr {
    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_CTLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccCtlrEl1: u64 {
        /// `CBPR` bit.
        const CBPR = 1 << 0;
        /// `EOImode` bit.
        const EOIMODE = 1 << 1;
        /// `PMHE` bit.
        const PMHE = 1 << 6;
        /// `SEIS` bit.
        const SEIS = 1 << 14;
        /// `A3V` bit.
        const A3V = 1 << 15;
        /// `RSS` bit.
        const RSS = 1 << 18;
        /// `ExtRange` bit.
        const EXTRANGE = 1 << 19;
    }
}

#[cfg(feature = "el1")]
impl IccCtlrEl1 {
    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ICC_CTLR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el3")]
impl IccCtlrEl3 {
    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b111
    }
}

bitflags! {
    /// `ICC_DIR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccDir: u32 {
    }
}

impl IccDir {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_DIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccDirEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccDirEl1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ICC_DOMHPPIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccDomhppirEl3: u64 {
        /// `NS_HPPI` bit.
        const NS_HPPI = 1 << 0;
        /// `S_HPPI` bit.
        const S_HPPI = 1 << 1;
        /// `RL_HPPI` bit.
        const RL_HPPI = 1 << 2;
        /// `P_HPPI` bit.
        const P_HPPI = 1 << 3;
    }
}

bitflags! {
    /// `ICC_EOIR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccEoir0: u32 {
    }
}

impl IccEoir0 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_EOIR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccEoir0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccEoir0El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICC_EOIR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccEoir1: u32 {
    }
}

impl IccEoir1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_EOIR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccEoir1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccEoir1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_HAPR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHaprEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccHaprEl1 {
    /// Returns the value of the `PRIORITY` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

bitflags! {
    /// `ICC_HPPIR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppir0: u32 {
    }
}

impl IccHppir0 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_HPPIR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppir0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccHppir0El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICC_HPPIR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppir1: u32 {
    }
}

impl IccHppir1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_HPPIR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppir1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccHppir1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_HPPIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppirEl1: u64 {
        /// `HPPIV` bit.
        const HPPIV = 1 << 32;
    }
}

#[cfg(feature = "el1")]
impl IccHppirEl1 {
    /// Returns the value of the `ID` field.
    pub const fn id(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `TYPE` field.
    pub const fn type_(self) -> u8 {
        (self.bits() >> 29) as u8 & 0b111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ICC_HPPIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHppirEl3: u64 {
        /// `HPPIV` bit.
        const HPPIV = 1 << 32;
    }
}

#[cfg(feature = "el3")]
impl IccHppirEl3 {
    /// Returns the value of the `ID` field.
    pub const fn id(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `TYPE` field.
    pub const fn type_(self) -> u8 {
        (self.bits() >> 29) as u8 & 0b111
    }
}

bitflags! {
    /// `ICC_HSRE` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccHsre: u32 {
        /// `SRE` bit.
        const SRE = 1 << 0;
        /// `DFB` bit.
        const DFB = 1 << 1;
        /// `DIB` bit.
        const DIB = 1 << 2;
        /// `Enable` bit.
        const ENABLE = 1 << 3;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_IAFFIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIaffidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccIaffidrEl1 {
    /// Returns the value of the `IAFFID` field.
    pub const fn iaffid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

bitflags! {
    /// `ICC_IAR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIar0: u32 {
    }
}

impl IccIar0 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_IAR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIar0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccIar0El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICC_IAR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIar1: u32 {
    }
}

impl IccIar1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_IAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIar1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccIar1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_ICSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIcsrEl1: u64 {
        /// `F` bit.
        const F = 1 << 0;
        /// `Enabled` bit.
        const ENABLED = 1 << 1;
        /// `Pending` bit.
        const PENDING = 1 << 2;
        /// `IRM` bit.
        const IRM = 1 << 3;
        /// `Active` bit.
        const ACTIVE = 1 << 4;
        /// `HM` bit.
        const HM = 1 << 5;
    }
}

#[cfg(feature = "el1")]
impl IccIcsrEl1 {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b11111
    }

    /// Returns the value of the `IAFFID` field.
    pub const fn iaffid(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_IDR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIdr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccIdr0El1 {
    /// Returns the value of the `ID_BITS` field.
    pub const fn id_bits(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `PRI_BITS` field.
    pub const fn pri_bits(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `GCIE_LEGACY` field.
    pub const fn gcie_legacy(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }
}

bitflags! {
    /// `ICC_IGRPEN0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen0: u32 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_IGRPEN0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen0El1: u64 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

bitflags! {
    /// `ICC_IGRPEN1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen1: u32 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_IGRPEN1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen1El1: u64 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ICC_IGRPEN1_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccIgrpen1El3: u64 {
        /// `EnableGrp1NS` bit.
        const ENABLEGRP1NS = 1 << 0;
        /// `EnableGrp1S` bit.
        const ENABLEGRP1S = 1 << 1;
    }
}

bitflags! {
    /// `ICC_MCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccMctlr: u32 {
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

impl IccMctlr {
    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b111
    }
}

bitflags! {
    /// `ICC_MGRPEN1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccMgrpen1: u32 {
        /// `EnableGrp1NS` bit.
        const ENABLEGRP1NS = 1 << 0;
        /// `EnableGrp1S` bit.
        const ENABLEGRP1S = 1 << 1;
    }
}

bitflags! {
    /// `ICC_MSRE` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccMsre: u32 {
        /// `SRE` bit.
        const SRE = 1 << 0;
        /// `DFB` bit.
        const DFB = 1 << 1;
        /// `DIB` bit.
        const DIB = 1 << 2;
        /// `Enable` bit.
        const ENABLE = 1 << 3;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_NMIAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccNmiar1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccNmiar1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_PCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccPcrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccPcrEl1 {
    /// Returns the value of the `PRIORITY` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `ICC_PCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccPcrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl IccPcrEl3 {
    /// Returns the value of the `PRIORITY` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

bitflags! {
    /// `ICC_PMR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccPmr: u32 {
    }
}

impl IccPmr {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_PMR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccPmrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IccPmrEl1 {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

bitflags! {
    /// `ICC_RPR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccRpr: u32 {
    }
}

impl IccRpr {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_RPR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccRprEl1: u64 {
        /// `NMI_NS` bit.
        const NMI_NS = 1 << 62;
        /// `NMI` bit.
        const NMI = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl IccRprEl1 {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_SGI0R_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSgi0rEl1: u64 {
        /// `IRM` bit.
        const IRM = 1 << 40;
    }
}

#[cfg(feature = "el1")]
impl IccSgi0rEl1 {
    /// Returns the value of the `TargetList` field.
    pub const fn targetlist(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }

    /// Returns the value of the `RS` field.
    pub const fn rs(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICC_SGI1R_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSgi1rEl1: u64 {
        /// `IRM` bit.
        const IRM = 1 << 40;
    }
}

#[cfg(feature = "el1")]
impl IccSgi1rEl1 {
    /// Returns the value of the `TargetList` field.
    pub const fn targetlist(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }

    /// Returns the value of the `RS` field.
    pub const fn rs(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111111
    }
}

bitflags! {
    /// `ICC_SRE` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IccSre: u32 {
        /// `SRE` bit.
        const SRE = 1 << 0;
        /// `DFB` bit.
        const DFB = 1 << 1;
        /// `DIB` bit.
        const DIB = 1 << 2;
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

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_APR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchAprEl2: u64 {
        /// `P<x>` bit 0.
        const P0 = 1 << 0;
        /// `P<x>` bit 1.
        const P1 = 1 << 1;
        /// `P<x>` bit 2.
        const P2 = 1 << 2;
        /// `P<x>` bit 3.
        const P3 = 1 << 3;
        /// `P<x>` bit 4.
        const P4 = 1 << 4;
        /// `P<x>` bit 5.
        const P5 = 1 << 5;
        /// `P<x>` bit 6.
        const P6 = 1 << 6;
        /// `P<x>` bit 7.
        const P7 = 1 << 7;
        /// `P<x>` bit 8.
        const P8 = 1 << 8;
        /// `P<x>` bit 9.
        const P9 = 1 << 9;
        /// `P<x>` bit 10.
        const P10 = 1 << 10;
        /// `P<x>` bit 11.
        const P11 = 1 << 11;
        /// `P<x>` bit 12.
        const P12 = 1 << 12;
        /// `P<x>` bit 13.
        const P13 = 1 << 13;
        /// `P<x>` bit 14.
        const P14 = 1 << 14;
        /// `P<x>` bit 15.
        const P15 = 1 << 15;
        /// `P<x>` bit 16.
        const P16 = 1 << 16;
        /// `P<x>` bit 17.
        const P17 = 1 << 17;
        /// `P<x>` bit 18.
        const P18 = 1 << 18;
        /// `P<x>` bit 19.
        const P19 = 1 << 19;
        /// `P<x>` bit 20.
        const P20 = 1 << 20;
        /// `P<x>` bit 21.
        const P21 = 1 << 21;
        /// `P<x>` bit 22.
        const P22 = 1 << 22;
        /// `P<x>` bit 23.
        const P23 = 1 << 23;
        /// `P<x>` bit 24.
        const P24 = 1 << 24;
        /// `P<x>` bit 25.
        const P25 = 1 << 25;
        /// `P<x>` bit 26.
        const P26 = 1 << 26;
        /// `P<x>` bit 27.
        const P27 = 1 << 27;
        /// `P<x>` bit 28.
        const P28 = 1 << 28;
        /// `P<x>` bit 29.
        const P29 = 1 << 29;
        /// `P<x>` bit 30.
        const P30 = 1 << 30;
        /// `P<x>` bit 31.
        const P31 = 1 << 31;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_CONTEXTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchContextrEl2: u64 {
        /// `DB` bit.
        const DB = 1 << 60;
        /// `IRICHPPIDIS` bit.
        const IRICHPPIDIS = 1 << 61;
        /// `F` bit.
        const F = 1 << 62;
        /// `V` bit.
        const V = 1 << 63;
    }
}

#[cfg(feature = "el2")]
impl IchContextrEl2 {
    /// Returns the value of the `VM` field.
    pub const fn vm(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `VPE` field.
    pub const fn vpe(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `DBPM` field.
    pub const fn dbpm(self) -> u8 {
        (self.bits() >> 55) as u8 & 0b11111
    }
}

bitflags! {
    /// `ICH_EISR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchEisr: u32 {
        /// `Status<n>` bit 0.
        const STATUS0 = 1 << 0;
        /// `Status<n>` bit 1.
        const STATUS1 = 1 << 1;
        /// `Status<n>` bit 2.
        const STATUS2 = 1 << 2;
        /// `Status<n>` bit 3.
        const STATUS3 = 1 << 3;
        /// `Status<n>` bit 4.
        const STATUS4 = 1 << 4;
        /// `Status<n>` bit 5.
        const STATUS5 = 1 << 5;
        /// `Status<n>` bit 6.
        const STATUS6 = 1 << 6;
        /// `Status<n>` bit 7.
        const STATUS7 = 1 << 7;
        /// `Status<n>` bit 8.
        const STATUS8 = 1 << 8;
        /// `Status<n>` bit 9.
        const STATUS9 = 1 << 9;
        /// `Status<n>` bit 10.
        const STATUS10 = 1 << 10;
        /// `Status<n>` bit 11.
        const STATUS11 = 1 << 11;
        /// `Status<n>` bit 12.
        const STATUS12 = 1 << 12;
        /// `Status<n>` bit 13.
        const STATUS13 = 1 << 13;
        /// `Status<n>` bit 14.
        const STATUS14 = 1 << 14;
        /// `Status<n>` bit 15.
        const STATUS15 = 1 << 15;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_EISR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchEisrEl2: u64 {
        /// `Status<n>` bit 0.
        const STATUS0 = 1 << 0;
        /// `Status<n>` bit 1.
        const STATUS1 = 1 << 1;
        /// `Status<n>` bit 2.
        const STATUS2 = 1 << 2;
        /// `Status<n>` bit 3.
        const STATUS3 = 1 << 3;
        /// `Status<n>` bit 4.
        const STATUS4 = 1 << 4;
        /// `Status<n>` bit 5.
        const STATUS5 = 1 << 5;
        /// `Status<n>` bit 6.
        const STATUS6 = 1 << 6;
        /// `Status<n>` bit 7.
        const STATUS7 = 1 << 7;
        /// `Status<n>` bit 8.
        const STATUS8 = 1 << 8;
        /// `Status<n>` bit 9.
        const STATUS9 = 1 << 9;
        /// `Status<n>` bit 10.
        const STATUS10 = 1 << 10;
        /// `Status<n>` bit 11.
        const STATUS11 = 1 << 11;
        /// `Status<n>` bit 12.
        const STATUS12 = 1 << 12;
        /// `Status<n>` bit 13.
        const STATUS13 = 1 << 13;
        /// `Status<n>` bit 14.
        const STATUS14 = 1 << 14;
        /// `Status<n>` bit 15.
        const STATUS15 = 1 << 15;
    }
}

bitflags! {
    /// `ICH_ELRSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchElrsr: u32 {
        /// `Status<n>` bit 0.
        const STATUS0 = 1 << 0;
        /// `Status<n>` bit 1.
        const STATUS1 = 1 << 1;
        /// `Status<n>` bit 2.
        const STATUS2 = 1 << 2;
        /// `Status<n>` bit 3.
        const STATUS3 = 1 << 3;
        /// `Status<n>` bit 4.
        const STATUS4 = 1 << 4;
        /// `Status<n>` bit 5.
        const STATUS5 = 1 << 5;
        /// `Status<n>` bit 6.
        const STATUS6 = 1 << 6;
        /// `Status<n>` bit 7.
        const STATUS7 = 1 << 7;
        /// `Status<n>` bit 8.
        const STATUS8 = 1 << 8;
        /// `Status<n>` bit 9.
        const STATUS9 = 1 << 9;
        /// `Status<n>` bit 10.
        const STATUS10 = 1 << 10;
        /// `Status<n>` bit 11.
        const STATUS11 = 1 << 11;
        /// `Status<n>` bit 12.
        const STATUS12 = 1 << 12;
        /// `Status<n>` bit 13.
        const STATUS13 = 1 << 13;
        /// `Status<n>` bit 14.
        const STATUS14 = 1 << 14;
        /// `Status<n>` bit 15.
        const STATUS15 = 1 << 15;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_ELRSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchElrsrEl2: u64 {
        /// `Status<n>` bit 0.
        const STATUS0 = 1 << 0;
        /// `Status<n>` bit 1.
        const STATUS1 = 1 << 1;
        /// `Status<n>` bit 2.
        const STATUS2 = 1 << 2;
        /// `Status<n>` bit 3.
        const STATUS3 = 1 << 3;
        /// `Status<n>` bit 4.
        const STATUS4 = 1 << 4;
        /// `Status<n>` bit 5.
        const STATUS5 = 1 << 5;
        /// `Status<n>` bit 6.
        const STATUS6 = 1 << 6;
        /// `Status<n>` bit 7.
        const STATUS7 = 1 << 7;
        /// `Status<n>` bit 8.
        const STATUS8 = 1 << 8;
        /// `Status<n>` bit 9.
        const STATUS9 = 1 << 9;
        /// `Status<n>` bit 10.
        const STATUS10 = 1 << 10;
        /// `Status<n>` bit 11.
        const STATUS11 = 1 << 11;
        /// `Status<n>` bit 12.
        const STATUS12 = 1 << 12;
        /// `Status<n>` bit 13.
        const STATUS13 = 1 << 13;
        /// `Status<n>` bit 14.
        const STATUS14 = 1 << 14;
        /// `Status<n>` bit 15.
        const STATUS15 = 1 << 15;
    }
}

bitflags! {
    /// `ICH_HCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchHcr: u32 {
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
    }
}

impl IchHcr {
    /// Returns the value of the `EOIcount` field.
    pub const fn eoicount(self) -> u8 {
        (self.bits() >> 27) as u8 & 0b11111
    }
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
    /// Returns the value of the `EOIcount` field.
    pub const fn eoicount(self) -> u8 {
        (self.bits() >> 27) as u8 & 0b11111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_HFGITR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchHfgitrEl2: u64 {
        /// `GICCDEN` bit.
        const GICCDEN = 1 << 0;
        /// `GICCDDIS` bit.
        const GICCDDIS = 1 << 1;
        /// `GICCDPRI` bit.
        const GICCDPRI = 1 << 2;
        /// `GICCDAFF` bit.
        const GICCDAFF = 1 << 3;
        /// `GICCDPEND` bit.
        const GICCDPEND = 1 << 4;
        /// `GICCDRCFG` bit.
        const GICCDRCFG = 1 << 5;
        /// `GICCDHM` bit.
        const GICCDHM = 1 << 6;
        /// `GICCDEOI` bit.
        const GICCDEOI = 1 << 7;
        /// `GICCDDI` bit.
        const GICCDDI = 1 << 8;
        /// `GICRCDIA` bit.
        const GICRCDIA = 1 << 9;
        /// `GICRCDNMIA` bit.
        const GICRCDNMIA = 1 << 10;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_HFGRTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchHfgrtrEl2: u64 {
        /// `ICC_APR_EL1` bit.
        const ICC_APR_EL1 = 1 << 0;
        /// `ICC_IDRn_EL1` bit.
        const ICC_IDRN_EL1 = 1 << 1;
        /// `ICC_CR0_EL1` bit.
        const ICC_CR0_EL1 = 1 << 2;
        /// `ICC_HAPR_EL1` bit.
        const ICC_HAPR_EL1 = 1 << 3;
        /// `ICC_HPPIR_EL1` bit.
        const ICC_HPPIR_EL1 = 1 << 4;
        /// `ICC_PCR_EL1` bit.
        const ICC_PCR_EL1 = 1 << 5;
        /// `ICC_ICSR_EL1` bit.
        const ICC_ICSR_EL1 = 1 << 6;
        /// `ICC_IAFFIDR_EL1` bit.
        const ICC_IAFFIDR_EL1 = 1 << 7;
        /// `ICC_PPI_HMRn_EL1` bit.
        const ICC_PPI_HMRN_EL1 = 1 << 16;
        /// `ICC_PPI_ENABLERn_EL1` bit.
        const ICC_PPI_ENABLERN_EL1 = 1 << 17;
        /// `ICC_PPI_PENDRn_EL1` bit.
        const ICC_PPI_PENDRN_EL1 = 1 << 18;
        /// `ICC_PPI_PRIORITYRn_EL1` bit.
        const ICC_PPI_PRIORITYRN_EL1 = 1 << 19;
        /// `ICC_PPI_ACTIVERn_EL1` bit.
        const ICC_PPI_ACTIVERN_EL1 = 1 << 20;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_HFGWTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchHfgwtrEl2: u64 {
        /// `ICC_APR_EL1` bit.
        const ICC_APR_EL1 = 1 << 0;
        /// `ICC_CR0_EL1` bit.
        const ICC_CR0_EL1 = 1 << 2;
        /// `ICC_PCR_EL1` bit.
        const ICC_PCR_EL1 = 1 << 5;
        /// `ICC_ICSR_EL1` bit.
        const ICC_ICSR_EL1 = 1 << 6;
        /// `ICC_PPI_ENABLERn_EL1` bit.
        const ICC_PPI_ENABLERN_EL1 = 1 << 17;
        /// `ICC_PPI_PENDRn_EL1` bit.
        const ICC_PPI_PENDRN_EL1 = 1 << 18;
        /// `ICC_PPI_PRIORITYRn_EL1` bit.
        const ICC_PPI_PRIORITYRN_EL1 = 1 << 19;
        /// `ICC_PPI_ACTIVERn_EL1` bit.
        const ICC_PPI_ACTIVERN_EL1 = 1 << 20;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_HPPIR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchHppirEl2: u64 {
        /// `HPPIV` bit.
        const HPPIV = 1 << 32;
    }
}

#[cfg(feature = "el2")]
impl IchHppirEl2 {
    /// Returns the value of the `ID` field.
    pub const fn id(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `TYPE` field.
    pub const fn type_(self) -> u8 {
        (self.bits() >> 29) as u8 & 0b111
    }
}

bitflags! {
    /// `ICH_MISR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchMisr: u32 {
        /// `EOI` bit.
        const EOI = 1 << 0;
        /// `U` bit.
        const U = 1 << 1;
        /// `LRENP` bit.
        const LRENP = 1 << 2;
        /// `NP` bit.
        const NP = 1 << 3;
        /// `VGrp0E` bit.
        const VGRP0E = 1 << 4;
        /// `VGrp0D` bit.
        const VGRP0D = 1 << 5;
        /// `VGrp1E` bit.
        const VGRP1E = 1 << 6;
        /// `VGrp1D` bit.
        const VGRP1D = 1 << 7;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_MISR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchMisrEl2: u64 {
        /// `EOI` bit.
        const EOI = 1 << 0;
        /// `U` bit.
        const U = 1 << 1;
        /// `LRENP` bit.
        const LRENP = 1 << 2;
        /// `NP` bit.
        const NP = 1 << 3;
        /// `VGrp0E` bit.
        const VGRP0E = 1 << 4;
        /// `VGrp0D` bit.
        const VGRP0D = 1 << 5;
        /// `VGrp1E` bit.
        const VGRP1E = 1 << 6;
        /// `VGrp1D` bit.
        const VGRP1D = 1 << 7;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_VCTLR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchVctlrEl2: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
        /// `V3` bit.
        const V3 = 1 << 1;
    }
}

bitflags! {
    /// `ICH_VMCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchVmcr: u32 {
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

impl IchVmcr {
    /// Returns the value of the `VBPR1` field.
    pub const fn vbpr1(self) -> u8 {
        (self.bits() >> 18) as u8 & 0b111
    }

    /// Returns the value of the `VBPR0` field.
    pub const fn vbpr0(self) -> u8 {
        (self.bits() >> 21) as u8 & 0b111
    }

    /// Returns the value of the `VPMR` field.
    pub const fn vpmr(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
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
    /// Returns the value of the `VBPR1` field.
    pub const fn vbpr1(self) -> u8 {
        (self.bits() >> 18) as u8 & 0b111
    }

    /// Returns the value of the `VBPR0` field.
    pub const fn vbpr0(self) -> u8 {
        (self.bits() >> 21) as u8 & 0b111
    }
}

bitflags! {
    /// `ICH_VTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchVtr: u32 {
        /// `TDS` bit.
        const TDS = 1 << 19;
        /// `nV4` bit.
        const NV4 = 1 << 20;
        /// `A3V` bit.
        const A3V = 1 << 21;
        /// `SEIS` bit.
        const SEIS = 1 << 22;
    }
}

impl IchVtr {
    /// Returns the value of the `ListRegs` field.
    pub const fn listregs(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 23) as u8 & 0b111
    }

    /// Returns the value of the `PREbits` field.
    pub const fn prebits(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111
    }

    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 29) as u8 & 0b111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ICH_VTR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IchVtrEl2: u64 {
        /// `DVIM` bit.
        const DVIM = 1 << 18;
        /// `TDS` bit.
        const TDS = 1 << 19;
        /// `nV4` bit.
        const NV4 = 1 << 20;
        /// `A3V` bit.
        const A3V = 1 << 21;
        /// `SEIS` bit.
        const SEIS = 1 << 22;
    }
}

#[cfg(feature = "el2")]
impl IchVtrEl2 {
    /// Returns the value of the `ListRegs` field.
    pub const fn listregs(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 23) as u8 & 0b111
    }

    /// Returns the value of the `PREbits` field.
    pub const fn prebits(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111
    }

    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 29) as u8 & 0b111
    }
}

bitflags! {
    /// `ICIMVAU` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Icimvau: u32 {
    }
}

impl Icimvau {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_APR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvAprEl1: u64 {
        /// `P<x>` bit 0.
        const P0 = 1 << 0;
        /// `P<x>` bit 1.
        const P1 = 1 << 1;
        /// `P<x>` bit 2.
        const P2 = 1 << 2;
        /// `P<x>` bit 3.
        const P3 = 1 << 3;
        /// `P<x>` bit 4.
        const P4 = 1 << 4;
        /// `P<x>` bit 5.
        const P5 = 1 << 5;
        /// `P<x>` bit 6.
        const P6 = 1 << 6;
        /// `P<x>` bit 7.
        const P7 = 1 << 7;
        /// `P<x>` bit 8.
        const P8 = 1 << 8;
        /// `P<x>` bit 9.
        const P9 = 1 << 9;
        /// `P<x>` bit 10.
        const P10 = 1 << 10;
        /// `P<x>` bit 11.
        const P11 = 1 << 11;
        /// `P<x>` bit 12.
        const P12 = 1 << 12;
        /// `P<x>` bit 13.
        const P13 = 1 << 13;
        /// `P<x>` bit 14.
        const P14 = 1 << 14;
        /// `P<x>` bit 15.
        const P15 = 1 << 15;
        /// `P<x>` bit 16.
        const P16 = 1 << 16;
        /// `P<x>` bit 17.
        const P17 = 1 << 17;
        /// `P<x>` bit 18.
        const P18 = 1 << 18;
        /// `P<x>` bit 19.
        const P19 = 1 << 19;
        /// `P<x>` bit 20.
        const P20 = 1 << 20;
        /// `P<x>` bit 21.
        const P21 = 1 << 21;
        /// `P<x>` bit 22.
        const P22 = 1 << 22;
        /// `P<x>` bit 23.
        const P23 = 1 << 23;
        /// `P<x>` bit 24.
        const P24 = 1 << 24;
        /// `P<x>` bit 25.
        const P25 = 1 << 25;
        /// `P<x>` bit 26.
        const P26 = 1 << 26;
        /// `P<x>` bit 27.
        const P27 = 1 << 27;
        /// `P<x>` bit 28.
        const P28 = 1 << 28;
        /// `P<x>` bit 29.
        const P29 = 1 << 29;
        /// `P<x>` bit 30.
        const P30 = 1 << 30;
        /// `P<x>` bit 31.
        const P31 = 1 << 31;
    }
}

bitflags! {
    /// `ICV_BPR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvBpr0: u32 {
    }
}

impl IcvBpr0 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_BPR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvBpr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvBpr0El1 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

bitflags! {
    /// `ICV_BPR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvBpr1: u32 {
    }
}

impl IcvBpr1 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_BPR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvBpr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvBpr1El1 {
    /// Returns the value of the `BinaryPoint` field.
    pub const fn binarypoint(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_CR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvCr0El1: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

bitflags! {
    /// `ICV_CTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvCtlr: u32 {
        /// `CBPR` bit.
        const CBPR = 1 << 0;
        /// `EOImode` bit.
        const EOIMODE = 1 << 1;
        /// `SEIS` bit.
        const SEIS = 1 << 14;
        /// `A3V` bit.
        const A3V = 1 << 15;
        /// `RSS` bit.
        const RSS = 1 << 18;
        /// `ExtRange` bit.
        const EXTRANGE = 1 << 19;
    }
}

impl IcvCtlr {
    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_CTLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvCtlrEl1: u64 {
        /// `CBPR` bit.
        const CBPR = 1 << 0;
        /// `EOImode` bit.
        const EOIMODE = 1 << 1;
        /// `SEIS` bit.
        const SEIS = 1 << 14;
        /// `A3V` bit.
        const A3V = 1 << 15;
        /// `RSS` bit.
        const RSS = 1 << 18;
        /// `ExtRange` bit.
        const EXTRANGE = 1 << 19;
    }
}

#[cfg(feature = "el1")]
impl IcvCtlrEl1 {
    /// Returns the value of the `PRIbits` field.
    pub const fn pribits(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111
    }

    /// Returns the value of the `IDbits` field.
    pub const fn idbits(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b111
    }
}

bitflags! {
    /// `ICV_DIR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvDir: u32 {
    }
}

impl IcvDir {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_DIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvDirEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvDirEl1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICV_EOIR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvEoir0: u32 {
    }
}

impl IcvEoir0 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_EOIR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvEoir0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvEoir0El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICV_EOIR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvEoir1: u32 {
    }
}

impl IcvEoir1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_EOIR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvEoir1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvEoir1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_HAPR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvHaprEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvHaprEl1 {
    /// Returns the value of the `PRIORITY` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

bitflags! {
    /// `ICV_HPPIR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvHppir0: u32 {
    }
}

impl IcvHppir0 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_HPPIR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvHppir0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvHppir0El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICV_HPPIR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvHppir1: u32 {
    }
}

impl IcvHppir1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_HPPIR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvHppir1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvHppir1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_HPPIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvHppirEl1: u64 {
        /// `HPPIV` bit.
        const HPPIV = 1 << 32;
    }
}

#[cfg(feature = "el1")]
impl IcvHppirEl1 {
    /// Returns the value of the `ID` field.
    pub const fn id(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `TYPE` field.
    pub const fn type_(self) -> u8 {
        (self.bits() >> 29) as u8 & 0b111
    }
}

bitflags! {
    /// `ICV_IAR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIar0: u32 {
    }
}

impl IcvIar0 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_IAR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIar0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvIar0El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICV_IAR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIar1: u32 {
    }
}

impl IcvIar1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_IAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIar1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvIar1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `ICV_IGRPEN0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIgrpen0: u32 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_IGRPEN0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIgrpen0El1: u64 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

bitflags! {
    /// `ICV_IGRPEN1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIgrpen1: u32 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_IGRPEN1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvIgrpen1El1: u64 {
        /// `Enable` bit.
        const ENABLE = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_NMIAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvNmiar1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvNmiar1El1 {
    /// Returns the value of the `INTID` field.
    pub const fn intid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_PCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvPcrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvPcrEl1 {
    /// Returns the value of the `PRIORITY` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

bitflags! {
    /// `ICV_PMR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvPmr: u32 {
    }
}

impl IcvPmr {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_PMR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvPmrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IcvPmrEl1 {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

bitflags! {
    /// `ICV_RPR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvRpr: u32 {
    }
}

impl IcvRpr {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ICV_RPR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IcvRprEl1: u64 {
        /// `NMI` bit.
        const NMI = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl IcvRprEl1 {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
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
    /// Returns the value of the `DebugVer` field.
    pub const fn debugver(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `TraceVer` field.
    pub const fn tracever(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `PMUVer` field.
    pub const fn pmuver(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `PMSS` field.
    pub const fn pmss(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `SEBEP` field.
    pub const fn sebep(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `PMSVer` field.
    pub const fn pmsver(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `DoubleLock` field.
    pub const fn doublelock(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `TraceBuffer` field.
    pub const fn tracebuffer(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `BRBE` field.
    pub const fn brbe(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `ExtTrcBuff` field.
    pub const fn exttrcbuff(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
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
    /// Returns the value of the `SYSPMUID` field.
    pub const fn syspmuid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `BRPs` field.
    pub const fn brps(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `WRPs` field.
    pub const fn wrps(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `CTX_CMPs` field.
    pub const fn ctx_cmps(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
    }

    /// Returns the value of the `SPMU` field.
    pub const fn spmu(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `PMICNTR` field.
    pub const fn pmicntr(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `ABLE` field.
    pub const fn able(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `ITE` field.
    pub const fn ite(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `EBEP` field.
    pub const fn ebep(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `DPFZS` field.
    pub const fn dpfzs(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `ABL_CMPs` field.
    pub const fn abl_cmps(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64DFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64dfr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64dfr2El1 {
    /// Returns the value of the `STEP` field.
    pub const fn step(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BWE` field.
    pub const fn bwe(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `SPE_EXC` field.
    pub const fn spe_exc(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `SPE_nVM` field.
    pub const fn spe_nvm(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `TRBE_EXC` field.
    pub const fn trbe_exc(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64FPFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64fpfr0El1: u64 {
        /// `F8E5M2` bit.
        const F8E5M2 = 1 << 0;
        /// `F8E4M3` bit.
        const F8E4M3 = 1 << 1;
        /// `F16MM2` bit.
        const F16MM2 = 1 << 15;
        /// `F8MM4` bit.
        const F8MM4 = 1 << 26;
        /// `F8MM8` bit.
        const F8MM8 = 1 << 27;
        /// `F8DP2` bit.
        const F8DP2 = 1 << 28;
        /// `F8DP4` bit.
        const F8DP4 = 1 << 29;
        /// `F8FMA` bit.
        const F8FMA = 1 << 30;
        /// `F8CVT` bit.
        const F8CVT = 1 << 31;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64ISAR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64isar0El1 {
    /// Returns the value of the `AES` field.
    pub const fn aes(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `SHA1` field.
    pub const fn sha1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SHA2` field.
    pub const fn sha2(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `CRC32` field.
    pub const fn crc32(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Atomic` field.
    pub const fn atomic(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `RDM` field.
    pub const fn rdm(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `SHA3` field.
    pub const fn sha3(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `SM3` field.
    pub const fn sm3(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `SM4` field.
    pub const fn sm4(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `DP` field.
    pub const fn dp(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `FHM` field.
    pub const fn fhm(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `TLB` field.
    pub const fn tlb(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `RNDR` field.
    pub const fn rndr(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64ISAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64isar1El1 {
    /// Returns the value of the `DPB` field.
    pub const fn dpb(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `APA` field.
    pub const fn apa(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `API` field.
    pub const fn api(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `JSCVT` field.
    pub const fn jscvt(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `FCMA` field.
    pub const fn fcma(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `LRCPC` field.
    pub const fn lrcpc(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `GPA` field.
    pub const fn gpa(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `GPI` field.
    pub const fn gpi(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `FRINTTS` field.
    pub const fn frintts(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `SB` field.
    pub const fn sb(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `SPECRES` field.
    pub const fn specres(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `BF16` field.
    pub const fn bf16(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `DGH` field.
    pub const fn dgh(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `I8MM` field.
    pub const fn i8mm(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `XS` field.
    pub const fn xs(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `LS64` field.
    pub const fn ls64(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64ISAR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64isar2El1 {
    /// Returns the value of the `WFxT` field.
    pub const fn wfxt(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `RPRES` field.
    pub const fn rpres(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `GPA3` field.
    pub const fn gpa3(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `APA3` field.
    pub const fn apa3(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `MOPS` field.
    pub const fn mops(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `BC` field.
    pub const fn bc(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `PAC_frac` field.
    pub const fn pac_frac(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `CLRBHB` field.
    pub const fn clrbhb(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `SYSREG_128` field.
    pub const fn sysreg_128(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `SYSINSTR_128` field.
    pub const fn sysinstr_128(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `PRFMSLC` field.
    pub const fn prfmslc(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `PCDPHINT` field.
    pub const fn pcdphint(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `RPRFM` field.
    pub const fn rprfm(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `CSSC` field.
    pub const fn cssc(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `LUT` field.
    pub const fn lut(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `ATS1A` field.
    pub const fn ats1a(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64ISAR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64isar3El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64isar3El1 {
    /// Returns the value of the `CPA` field.
    pub const fn cpa(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `FAMINMAX` field.
    pub const fn faminmax(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `TLBIW` field.
    pub const fn tlbiw(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `PACM` field.
    pub const fn pacm(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `LSFE` field.
    pub const fn lsfe(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `OCCMO` field.
    pub const fn occmo(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `LSUI` field.
    pub const fn lsui(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `FPRCVT` field.
    pub const fn fprcvt(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `PAC_frac2` field.
    pub const fn pac_frac2(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `MTETC` field.
    pub const fn mtetc(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `LSCSHINT` field.
    pub const fn lscshint(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `LSCP` field.
    pub const fn lscp(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
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
    /// Returns the value of the `PARange` field.
    pub const fn parange(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `ASIDBits` field.
    pub const fn asidbits(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `BigEnd` field.
    pub const fn bigend(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SNSMem` field.
    pub const fn snsmem(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `BigEndEL0` field.
    pub const fn bigendel0(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `TGran16` field.
    pub const fn tgran16(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `TGran64` field.
    pub const fn tgran64(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `TGran4` field.
    pub const fn tgran4(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `TGran16_2` field.
    pub const fn tgran16_2(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `TGran64_2` field.
    pub const fn tgran64_2(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `TGran4_2` field.
    pub const fn tgran4_2(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `ExS` field.
    pub const fn exs(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `FGT` field.
    pub const fn fgt(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `ECV` field.
    pub const fn ecv(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
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
    /// Returns the value of the `HAFDBS` field.
    pub const fn hafdbs(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `VMIDBits` field.
    pub const fn vmidbits(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `VH` field.
    pub const fn vh(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `LO` field.
    pub const fn lo(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `TWED` field.
    pub const fn twed(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `HCX` field.
    pub const fn hcx(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `AFP` field.
    pub const fn afp(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `TIDCP1` field.
    pub const fn tidcp1(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `CMOW` field.
    pub const fn cmow(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `ECBHB` field.
    pub const fn ecbhb(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
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
    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `UAO` field.
    pub const fn uao(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `IESB` field.
    pub const fn iesb(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `VARange` field.
    pub const fn varange(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `NV` field.
    pub const fn nv(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `ST` field.
    pub const fn st(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `AT` field.
    pub const fn at(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `IDS` field.
    pub const fn ids(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `FWB` field.
    pub const fn fwb(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `TTL` field.
    pub const fn ttl(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `BBM` field.
    pub const fn bbm(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `E0PD` field.
    pub const fn e0pd(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
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
    /// Returns the value of the `TCRX` field.
    pub const fn tcrx(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SCTLRX` field.
    pub const fn sctlrx(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `S1PIE` field.
    pub const fn s1pie(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `S2PIE` field.
    pub const fn s2pie(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `S1POE` field.
    pub const fn s1poe(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `S2POE` field.
    pub const fn s2poe(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `AIE` field.
    pub const fn aie(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `MEC` field.
    pub const fn mec(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `D128` field.
    pub const fn d128(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `D128_2` field.
    pub const fn d128_2(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `SNERR` field.
    pub const fn snerr(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `ANERR` field.
    pub const fn anerr(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `SDERR` field.
    pub const fn sderr(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `ADERR` field.
    pub const fn aderr(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `Spec_FPACC` field.
    pub const fn spec_fpacc(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64MMFR4_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64mmfr4El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64mmfr4El1 {
    /// Returns the value of the `PoPS` field.
    pub const fn pops(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `EIESB` field.
    pub const fn eiesb(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `ASID2` field.
    pub const fn asid2(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `HACDBS` field.
    pub const fn hacdbs(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `FGWTE3` field.
    pub const fn fgwte3(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `NV_frac` field.
    pub const fn nv_frac(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `E2H0` field.
    pub const fn e2h0(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `RMEGDI` field.
    pub const fn rmegdi(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `EAESR` field.
    pub const fn eaesr(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `E3DSE` field.
    pub const fn e3dse(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `TLBID` field.
    pub const fn tlbid(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `SRMASK` field.
    pub const fn srmask(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `TPS` field.
    pub const fn tps(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `TEV` field.
    pub const fn tev(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `SCRX` field.
    pub const fn scrx(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `MTEFGT` field.
    pub const fn mtefgt(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
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
    /// Returns the value of the `EL0` field.
    pub const fn el0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `EL1` field.
    pub const fn el1(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `EL2` field.
    pub const fn el2(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `EL3` field.
    pub const fn el3(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `FP` field.
    pub const fn fp(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `AdvSIMD` field.
    pub const fn advsimd(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `SVE` field.
    pub const fn sve(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `SEL2` field.
    pub const fn sel2(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `MPAM` field.
    pub const fn mpam(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `RME` field.
    pub const fn rme(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
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
    /// Returns the value of the `BT` field.
    pub const fn bt(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MTE` field.
    pub const fn mte(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `MPAM_frac` field.
    pub const fn mpam_frac(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `SME` field.
    pub const fn sme(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `RNDR_trap` field.
    pub const fn rndr_trap(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `CSV2_frac` field.
    pub const fn csv2_frac(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `NMI` field.
    pub const fn nmi(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `MTE_frac` field.
    pub const fn mte_frac(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `GCS` field.
    pub const fn gcs(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `THE` field.
    pub const fn the(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `MTEX` field.
    pub const fn mtex(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `DF2` field.
    pub const fn df2(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `PFAR` field.
    pub const fn pfar(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64PFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64pfr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64pfr2El1 {
    /// Returns the value of the `MTEPERM` field.
    pub const fn mteperm(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `MTESTOREONLY` field.
    pub const fn mtestoreonly(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MTEFAR` field.
    pub const fn mtefar(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `GCIE` field.
    pub const fn gcie(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `UINJ` field.
    pub const fn uinj(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MTEEIRG` field.
    pub const fn mteeirg(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `FGDT` field.
    pub const fn fgdt(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `MPAM2` field.
    pub const fn mpam2(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `FPMR` field.
    pub const fn fpmr(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `VMTE` field.
    pub const fn vmte(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `VMTETC` field.
    pub const fn vmtetc(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `VMTETCL` field.
    pub const fn vmtetcl(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
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
    /// Returns the value of the `I8I32` field.
    pub const fn i8i32(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `I16I32` field.
    pub const fn i16i32(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `I16I64` field.
    pub const fn i16i64(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `SMEver` field.
    pub const fn smever(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_AA64ZFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdAa64zfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdAa64zfr0El1 {
    /// Returns the value of the `SVEver` field.
    pub const fn svever(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `AES` field.
    pub const fn aes(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `EltPerm` field.
    pub const fn eltperm(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `BitPerm` field.
    pub const fn bitperm(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `BF16` field.
    pub const fn bf16(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `B16B16` field.
    pub const fn b16b16(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `SHA3` field.
    pub const fn sha3(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `SM4` field.
    pub const fn sm4(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `I8MM` field.
    pub const fn i8mm(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `F16MM` field.
    pub const fn f16mm(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `F32MM` field.
    pub const fn f32mm(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `F64MM` field.
    pub const fn f64mm(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_DFR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr0: u32 {
    }
}

impl IdDfr0 {
    /// Returns the value of the `CopDbg` field.
    pub const fn copdbg(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `CopSDbg` field.
    pub const fn copsdbg(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MMapDbg` field.
    pub const fn mmapdbg(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `CopTrc` field.
    pub const fn coptrc(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `MMapTrc` field.
    pub const fn mmaptrc(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MProfDbg` field.
    pub const fn mprofdbg(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `PerfMon` field.
    pub const fn perfmon(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_DFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdDfr0El1 {
    /// Returns the value of the `CopDbg` field.
    pub const fn copdbg(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `CopSDbg` field.
    pub const fn copsdbg(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MMapDbg` field.
    pub const fn mmapdbg(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `CopTrc` field.
    pub const fn coptrc(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `MMapTrc` field.
    pub const fn mmaptrc(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MProfDbg` field.
    pub const fn mprofdbg(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `PerfMon` field.
    pub const fn perfmon(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `TraceFilt` field.
    pub const fn tracefilt(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_DFR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr1: u32 {
    }
}

impl IdDfr1 {
    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_DFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdDfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdDfr1El1 {
    /// Returns the value of the `MTPMU` field.
    pub const fn mtpmu(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `HPMN0` field.
    pub const fn hpmn0(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_ISAR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar0: u32 {
    }
}

impl IdIsar0 {
    /// Returns the value of the `Swap` field.
    pub const fn swap(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BitCount` field.
    pub const fn bitcount(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `BitField` field.
    pub const fn bitfield(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `CmpBranch` field.
    pub const fn cmpbranch(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `Coproc` field.
    pub const fn coproc(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Debug` field.
    pub const fn debug(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Divide` field.
    pub const fn divide(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_ISAR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdIsar0El1 {
    /// Returns the value of the `Swap` field.
    pub const fn swap(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BitCount` field.
    pub const fn bitcount(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `BitField` field.
    pub const fn bitfield(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `CmpBranch` field.
    pub const fn cmpbranch(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `Coproc` field.
    pub const fn coproc(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Debug` field.
    pub const fn debug(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Divide` field.
    pub const fn divide(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_ISAR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar1: u32 {
    }
}

impl IdIsar1 {
    /// Returns the value of the `Endian` field.
    pub const fn endian(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `Except` field.
    pub const fn except(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `Except_AR` field.
    pub const fn except_ar(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `Extend` field.
    pub const fn extend_(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `IfThen` field.
    pub const fn ifthen(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Immediate` field.
    pub const fn immediate(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Interwork` field.
    pub const fn interwork(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Jazelle` field.
    pub const fn jazelle(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_ISAR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdIsar1El1 {
    /// Returns the value of the `Endian` field.
    pub const fn endian(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `Except` field.
    pub const fn except(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `Except_AR` field.
    pub const fn except_ar(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `Extend` field.
    pub const fn extend_(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `IfThen` field.
    pub const fn ifthen(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Immediate` field.
    pub const fn immediate(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Interwork` field.
    pub const fn interwork(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Jazelle` field.
    pub const fn jazelle(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_ISAR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar2: u32 {
    }
}

impl IdIsar2 {
    /// Returns the value of the `LoadStore` field.
    pub const fn loadstore(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `MemHint` field.
    pub const fn memhint(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MultiAccessInt` field.
    pub const fn multiaccessint(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `Mult` field.
    pub const fn mult(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `MultS` field.
    pub const fn mults(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MultU` field.
    pub const fn multu(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `PSR_AR` field.
    pub const fn psr_ar(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Reversal` field.
    pub const fn reversal(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_ISAR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdIsar2El1 {
    /// Returns the value of the `LoadStore` field.
    pub const fn loadstore(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `MemHint` field.
    pub const fn memhint(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MultiAccessInt` field.
    pub const fn multiaccessint(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `Mult` field.
    pub const fn mult(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `MultS` field.
    pub const fn mults(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MultU` field.
    pub const fn multu(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `PSR_AR` field.
    pub const fn psr_ar(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Reversal` field.
    pub const fn reversal(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_ISAR3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar3: u32 {
    }
}

impl IdIsar3 {
    /// Returns the value of the `Saturate` field.
    pub const fn saturate(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SIMD` field.
    pub const fn simd(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `SVC` field.
    pub const fn svc(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SynchPrim` field.
    pub const fn synchprim(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `TabBranch` field.
    pub const fn tabbranch(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `T32Copy` field.
    pub const fn t32copy(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `TrueNOP` field.
    pub const fn truenop(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `T32EE` field.
    pub const fn t32ee(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_ISAR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar3El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdIsar3El1 {
    /// Returns the value of the `Saturate` field.
    pub const fn saturate(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SIMD` field.
    pub const fn simd(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `SVC` field.
    pub const fn svc(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SynchPrim` field.
    pub const fn synchprim(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `TabBranch` field.
    pub const fn tabbranch(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `T32Copy` field.
    pub const fn t32copy(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `TrueNOP` field.
    pub const fn truenop(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `T32EE` field.
    pub const fn t32ee(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_ISAR4` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar4: u32 {
    }
}

impl IdIsar4 {
    /// Returns the value of the `Unpriv` field.
    pub const fn unpriv(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `WithShifts` field.
    pub const fn withshifts(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `Writeback` field.
    pub const fn writeback(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SMC` field.
    pub const fn smc(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `Barrier` field.
    pub const fn barrier(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `SynchPrim_frac` field.
    pub const fn synchprim_frac(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `PSR_M` field.
    pub const fn psr_m(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `SWP_frac` field.
    pub const fn swp_frac(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_ISAR4_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar4El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdIsar4El1 {
    /// Returns the value of the `Unpriv` field.
    pub const fn unpriv(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `WithShifts` field.
    pub const fn withshifts(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `Writeback` field.
    pub const fn writeback(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SMC` field.
    pub const fn smc(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `Barrier` field.
    pub const fn barrier(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `SynchPrim_frac` field.
    pub const fn synchprim_frac(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `PSR_M` field.
    pub const fn psr_m(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `SWP_frac` field.
    pub const fn swp_frac(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_ISAR5` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar5: u32 {
    }
}

impl IdIsar5 {
    /// Returns the value of the `SEVL` field.
    pub const fn sevl(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `AES` field.
    pub const fn aes(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `SHA1` field.
    pub const fn sha1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SHA2` field.
    pub const fn sha2(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `CRC32` field.
    pub const fn crc32(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `RDM` field.
    pub const fn rdm(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `VCMA` field.
    pub const fn vcma(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_ISAR5_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar5El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdIsar5El1 {
    /// Returns the value of the `SEVL` field.
    pub const fn sevl(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `AES` field.
    pub const fn aes(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `SHA1` field.
    pub const fn sha1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SHA2` field.
    pub const fn sha2(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `CRC32` field.
    pub const fn crc32(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `RDM` field.
    pub const fn rdm(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `VCMA` field.
    pub const fn vcma(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_ISAR6` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar6: u32 {
    }
}

impl IdIsar6 {
    /// Returns the value of the `JSCVT` field.
    pub const fn jscvt(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `DP` field.
    pub const fn dp(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `FHM` field.
    pub const fn fhm(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SB` field.
    pub const fn sb(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `SPECRES` field.
    pub const fn specres(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `BF16` field.
    pub const fn bf16(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `I8MM` field.
    pub const fn i8mm(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `CLRBHB` field.
    pub const fn clrbhb(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_ISAR6_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdIsar6El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdIsar6El1 {
    /// Returns the value of the `JSCVT` field.
    pub const fn jscvt(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `DP` field.
    pub const fn dp(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `FHM` field.
    pub const fn fhm(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SB` field.
    pub const fn sb(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `SPECRES` field.
    pub const fn specres(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `BF16` field.
    pub const fn bf16(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `I8MM` field.
    pub const fn i8mm(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `CLRBHB` field.
    pub const fn clrbhb(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_MMFR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr0: u32 {
    }
}

impl IdMmfr0 {
    /// Returns the value of the `VMSA` field.
    pub const fn vmsa(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `PMSA` field.
    pub const fn pmsa(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `OuterShr` field.
    pub const fn outershr(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `ShareLvl` field.
    pub const fn sharelvl(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `TCM` field.
    pub const fn tcm(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `AuxReg` field.
    pub const fn auxreg(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `FCSE` field.
    pub const fn fcse(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `InnerShr` field.
    pub const fn innershr(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_MMFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdMmfr0El1 {
    /// Returns the value of the `VMSA` field.
    pub const fn vmsa(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `PMSA` field.
    pub const fn pmsa(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `OuterShr` field.
    pub const fn outershr(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `ShareLvl` field.
    pub const fn sharelvl(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `TCM` field.
    pub const fn tcm(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `AuxReg` field.
    pub const fn auxreg(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `FCSE` field.
    pub const fn fcse(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `InnerShr` field.
    pub const fn innershr(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_MMFR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr1: u32 {
    }
}

impl IdMmfr1 {
    /// Returns the value of the `L1HvdVA` field.
    pub const fn l1hvdva(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `L1UniVA` field.
    pub const fn l1univa(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `L1HvdSW` field.
    pub const fn l1hvdsw(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `L1UniSW` field.
    pub const fn l1unisw(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `L1Hvd` field.
    pub const fn l1hvd(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `L1Uni` field.
    pub const fn l1uni(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `L1TstCln` field.
    pub const fn l1tstcln(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `BPred` field.
    pub const fn bpred(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_MMFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdMmfr1El1 {
    /// Returns the value of the `L1HvdVA` field.
    pub const fn l1hvdva(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `L1UniVA` field.
    pub const fn l1univa(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `L1HvdSW` field.
    pub const fn l1hvdsw(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `L1UniSW` field.
    pub const fn l1unisw(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `L1Hvd` field.
    pub const fn l1hvd(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `L1Uni` field.
    pub const fn l1uni(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `L1TstCln` field.
    pub const fn l1tstcln(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `BPred` field.
    pub const fn bpred(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_MMFR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr2: u32 {
    }
}

impl IdMmfr2 {
    /// Returns the value of the `L1HvdFG` field.
    pub const fn l1hvdfg(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `L1HvdBG` field.
    pub const fn l1hvdbg(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `L1HvdRng` field.
    pub const fn l1hvdrng(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `HvdTLB` field.
    pub const fn hvdtlb(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `UniTLB` field.
    pub const fn unitlb(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MemBarr` field.
    pub const fn membarr(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `WFIStall` field.
    pub const fn wfistall(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `HWAccFlg` field.
    pub const fn hwaccflg(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_MMFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdMmfr2El1 {
    /// Returns the value of the `L1HvdFG` field.
    pub const fn l1hvdfg(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `L1HvdBG` field.
    pub const fn l1hvdbg(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `L1HvdRng` field.
    pub const fn l1hvdrng(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `HvdTLB` field.
    pub const fn hvdtlb(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `UniTLB` field.
    pub const fn unitlb(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MemBarr` field.
    pub const fn membarr(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `WFIStall` field.
    pub const fn wfistall(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `HWAccFlg` field.
    pub const fn hwaccflg(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_MMFR3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr3: u32 {
    }
}

impl IdMmfr3 {
    /// Returns the value of the `CMaintVA` field.
    pub const fn cmaintva(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `CMaintSW` field.
    pub const fn cmaintsw(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `BPMaint` field.
    pub const fn bpmaint(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `MaintBcst` field.
    pub const fn maintbcst(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `CohWalk` field.
    pub const fn cohwalk(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `CMemSz` field.
    pub const fn cmemsz(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Supersec` field.
    pub const fn supersec(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_MMFR3_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr3El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdMmfr3El1 {
    /// Returns the value of the `CMaintVA` field.
    pub const fn cmaintva(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `CMaintSW` field.
    pub const fn cmaintsw(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `BPMaint` field.
    pub const fn bpmaint(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `MaintBcst` field.
    pub const fn maintbcst(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `PAN` field.
    pub const fn pan(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `CohWalk` field.
    pub const fn cohwalk(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `CMemSz` field.
    pub const fn cmemsz(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `Supersec` field.
    pub const fn supersec(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_MMFR4` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr4: u32 {
    }
}

impl IdMmfr4 {
    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `AC2` field.
    pub const fn ac2(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_MMFR4_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr4El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdMmfr4El1 {
    /// Returns the value of the `SpecSEI` field.
    pub const fn specsei(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `AC2` field.
    pub const fn ac2(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `XNX` field.
    pub const fn xnx(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `CnP` field.
    pub const fn cnp(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `HPDS` field.
    pub const fn hpds(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `LSM` field.
    pub const fn lsm(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `CCIDX` field.
    pub const fn ccidx(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `EVT` field.
    pub const fn evt(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_MMFR5` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr5: u32 {
    }
}

impl IdMmfr5 {
    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_MMFR5_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdMmfr5El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdMmfr5El1 {
    /// Returns the value of the `ETS` field.
    pub const fn ets(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `nTLBPA` field.
    pub const fn ntlbpa(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_PFR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr0: u32 {
    }
}

impl IdPfr0 {
    /// Returns the value of the `State0` field.
    pub const fn state0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `State1` field.
    pub const fn state1(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `State2` field.
    pub const fn state2(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `State3` field.
    pub const fn state3(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_PFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdPfr0El1 {
    /// Returns the value of the `State0` field.
    pub const fn state0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `State1` field.
    pub const fn state1(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `State2` field.
    pub const fn state2(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `State3` field.
    pub const fn state3(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `CSV2` field.
    pub const fn csv2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `AMU` field.
    pub const fn amu(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `DIT` field.
    pub const fn dit(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `RAS` field.
    pub const fn ras(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_PFR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr1: u32 {
    }
}

impl IdPfr1 {
    /// Returns the value of the `ProgMod` field.
    pub const fn progmod(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `Security` field.
    pub const fn security(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MProgMod` field.
    pub const fn mprogmod(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `Virtualization` field.
    pub const fn virtualization(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `GenTimer` field.
    pub const fn gentimer(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Sec_frac` field.
    pub const fn sec_frac(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Virt_frac` field.
    pub const fn virt_frac(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_PFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdPfr1El1 {
    /// Returns the value of the `ProgMod` field.
    pub const fn progmod(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `Security` field.
    pub const fn security(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `MProgMod` field.
    pub const fn mprogmod(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `Virtualization` field.
    pub const fn virtualization(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `GenTimer` field.
    pub const fn gentimer(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Sec_frac` field.
    pub const fn sec_frac(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Virt_frac` field.
    pub const fn virt_frac(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `GIC` field.
    pub const fn gic(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `ID_PFR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr2: u32 {
    }
}

impl IdPfr2 {
    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ID_PFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IdPfr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl IdPfr2El1 {
    /// Returns the value of the `CSV3` field.
    pub const fn csv3(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SSBS` field.
    pub const fn ssbs(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `RAS_frac` field.
    pub const fn ras_frac(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }
}

bitflags! {
    /// `IFAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ifar: u32 {
    }
}

impl Ifar {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `IFSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `IFSR32_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ifsr32El2: u64 {
        /// `LPAE` bit.
        const LPAE = 1 << 9;
        /// `ExT` bit.
        const EXT = 1 << 12;
        /// `FnV` bit.
        const FNV = 1 << 16;
    }
}

#[cfg(feature = "el2")]
impl Ifsr32El2 {
    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `IRTBRP_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IrtbrpEl1: u64 {
        /// `POE2` bit.
        const POE2 = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 4;
        /// `ETCSVC` bit.
        const ETCSVC = 1 << 5;
    }
}

#[cfg(feature = "el1")]
impl IrtbrpEl1 {
    /// Returns the value of the `TIW` field.
    pub const fn tiw(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `IRTBRP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IrtbrpEl2: u64 {
        /// `POE2` bit.
        const POE2 = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 4;
        /// `ETCSVC` bit.
        const ETCSVC = 1 << 5;
    }
}

#[cfg(feature = "el2")]
impl IrtbrpEl2 {
    /// Returns the value of the `TIW` field.
    pub const fn tiw(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `IRTBRP_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IrtbrpEl3: u64 {
        /// `POE2` bit.
        const POE2 = 1 << 0;
        /// `ETCSVC` bit.
        const ETCSVC = 1 << 5;
    }
}

#[cfg(feature = "el3")]
impl IrtbrpEl3 {
    /// Returns the value of the `TIW` field.
    pub const fn tiw(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `IRTBRU_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IrtbruEl1: u64 {
        /// `POE2` bit.
        const POE2 = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 4;
        /// `ETCSVC` bit.
        const ETCSVC = 1 << 5;
    }
}

#[cfg(feature = "el1")]
impl IrtbruEl1 {
    /// Returns the value of the `TIW` field.
    pub const fn tiw(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `IRTBRU_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct IrtbruEl2: u64 {
        /// `POE2` bit.
        const POE2 = 1 << 0;
        /// `FNG` bit.
        const FNG = 1 << 4;
        /// `ETCSVC` bit.
        const ETCSVC = 1 << 5;
    }
}

#[cfg(feature = "el2")]
impl IrtbruEl2 {
    /// Returns the value of the `TIW` field.
    pub const fn tiw(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111
    }

    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `ISR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `ITLBIASID` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Itlbiasid: u32 {
    }
}

impl Itlbiasid {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

bitflags! {
    /// `ITLBIMVA` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Itlbimva: u32 {
    }
}

impl Itlbimva {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `LDSTT_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct LdsttEl1: u64 {
        /// `TTBA` bit.
        const TTBA = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl LdsttEl1 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b1111111
    }

    /// Returns the value of the `FPOIndex` field.
    pub const fn fpoindex(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `LDSTT_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct LdsttEl2: u64 {
        /// `TTBA` bit.
        const TTBA = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl LdsttEl2 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b1111111
    }

    /// Returns the value of the `FPOIndex` field.
    pub const fn fpoindex(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `LORC_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct LorcEl1: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl LorcEl1 {
    /// Returns the value of the `DS` field.
    pub const fn ds(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `LOREA_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct LoreaEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl LoreaEl1 {
    /// Returns the value of the `EA[47:16]` field.
    pub const fn ea_47_16(self) -> u32 {
        (self.bits() >> 16) as u32 & 0b11111111111111111111111111111111
    }

    /// Returns the value of the `EA[51:48]` field.
    pub const fn ea_51_48(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `EA[55:52]` field.
    pub const fn ea_55_52(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `LORID_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct LoridEl1: u64 {
        /// `RL` bit.
        const RL = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl LoridEl1 {
    /// Returns the value of the `LR` field.
    pub const fn lr(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `LD` field.
    pub const fn ld(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `LORN_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct LornEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl LornEl1 {
    /// Returns the value of the `Num` field.
    pub const fn num(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `LORSA_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct LorsaEl1: u64 {
        /// `Valid` bit.
        const VALID = 1 << 0;
        /// `RL` bit.
        const RL = 1 << 1;
    }
}

#[cfg(feature = "el1")]
impl LorsaEl1 {
    /// Returns the value of the `SA` field.
    pub const fn sa(self) -> u64 {
        (self.bits() >> 16) as u64 & 0b1111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `MAIR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair0: u32 {
    }
}

impl Mair0 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 4);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
    }
}

bitflags! {
    /// `MAIR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair1: u32 {
    }
}

impl Mair1 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n >= 4 && n < 8);
        (self.bits() >> (0 + (n - 4) * 8)) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MAIR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Mair2El1 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MAIR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair2El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Mair2El2 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `MAIR2_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mair2El3: u64 {
    }
}

#[cfg(feature = "el3")]
impl Mair2El3 {
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
    }
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
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
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
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
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
    /// Returns the value of the given `Attr<n>` field.
    pub const fn attr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 8)) as u8 & 0b11111111
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

bitflags! {
    /// `MDCCSR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdccsrEl0: u64 {
        /// `TXfull` bit.
        const TXFULL = 1 << 29;
        /// `RXfull` bit.
        const RXFULL = 1 << 30;
    }
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
    /// Returns the value of the `HPMN` field.
    pub const fn hpmn(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `E2PB` field.
    pub const fn e2pb(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `E2TB` field.
    pub const fn e2tb(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11
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
    /// Returns the value of the `NSPB` field.
    pub const fn nspb(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `SPD32` field.
    pub const fn spd32(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `NSTB` field.
    pub const fn nstb(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }

    /// Returns the value of the `PMSSE` field.
    pub const fn pmsse(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11
    }

    /// Returns the value of the `SBRBE` field.
    pub const fn sbrbe(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11
    }

    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11
    }

    /// Returns the value of the `EPMSSAD` field.
    pub const fn epmssad(self) -> u8 {
        (self.bits() >> 45) as u8 & 0b11
    }

    /// Returns the value of the `ETBAD` field.
    pub const fn etbad(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11
    }

    /// Returns the value of the `PMSEE` field.
    pub const fn pmsee(self) -> u8 {
        (self.bits() >> 51) as u8 & 0b11
    }

    /// Returns the value of the `TRBEE` field.
    pub const fn trbee(self) -> u8 {
        (self.bits() >> 53) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MDRAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdrarEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MdrarEl1 {
    /// Returns the value of the `Valid` field.
    pub const fn valid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `ROMADDR` field.
    pub const fn romaddr(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b11111111111111111111111111111111111111111111
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
    /// Returns the value of the `INTdis` field.
    pub const fn intdis(self) -> u8 {
        (self.bits() >> 22) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MDSELR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdselrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MdselrEl1 {
    /// Returns the value of the `BANK` field.
    pub const fn bank(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MDSTEPOP_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MdstepopEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MdstepopEl1 {
    /// Returns the value of the `OPCODE` field.
    pub const fn opcode(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MECIDR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MecidrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MecidrEl2 {
    /// Returns the value of the `MECIDWidthm1` field.
    pub const fn mecidwidthm1(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MECID_A0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MecidA0El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MecidA0El2 {
    /// Returns the value of the `MECID` field.
    pub const fn mecid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MECID_A1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MecidA1El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MecidA1El2 {
    /// Returns the value of the `MECID` field.
    pub const fn mecid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MECID_P0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MecidP0El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MecidP0El2 {
    /// Returns the value of the `MECID` field.
    pub const fn mecid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MECID_P1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MecidP1El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MecidP1El2 {
    /// Returns the value of the `MECID` field.
    pub const fn mecid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `MECID_RL_A_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MecidRlAEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl MecidRlAEl3 {
    /// Returns the value of the `MECID` field.
    pub const fn mecid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `MFAR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MfarEl3: u64 {
        /// `NSE2` bit.
        const NSE2 = 1 << 61;
        /// `NSE` bit.
        const NSE = 1 << 62;
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

#[cfg(feature = "el3")]
impl MfarEl3 {
    /// Returns the value of the `PA` field.
    pub const fn pa(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b111111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `FPA` field.
    pub const fn fpa(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b111111111111111111111111111111111111
    }

    /// Returns the value of the `FPA[51:48]` field.
    pub const fn fpa_51_48(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `FPA[55:52]` field.
    pub const fn fpa_55_52(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `PA[55:52]` field.
    pub const fn pa_55_52(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }
}

bitflags! {
    /// `MIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Midr: u32 {
    }
}

impl Midr {
    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        (self.bits() >> 4) as u16 & 0b111111111111
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
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
    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        (self.bits() >> 4) as u16 & 0b111111111111
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAM0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpam0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Mpam0El1 {
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11111111
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAM1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpam1El1: u64 {
        /// `ALTSP_FRCD` bit.
        const ALTSP_FRCD = 1 << 54;
        /// `FORCED_NS` bit.
        const FORCED_NS = 1 << 60;
        /// `MPAMEN` bit.
        const MPAMEN = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl Mpam1El1 {
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11111111
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11111111
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_I` field.
    pub const fn partid_i(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `altPARTID` field.
    pub const fn altpartid(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG_I` field.
    pub const fn pmg_i(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11111111
    }

    /// Returns the value of the `altPMG` field.
    pub const fn altpmg(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAMBW0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpambw0El1: u64 {
        /// `HARDLIM` bit.
        const HARDLIM = 1 << 61;
        /// `ENABLED` bit.
        const ENABLED = 1 << 62;
        /// `HW_SCALE_ENABLE` bit.
        const HW_SCALE_ENABLE = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl Mpambw0El1 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAMBW1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpambw1El1: u64 {
        /// `HARDLIM` bit.
        const HARDLIM = 1 << 61;
        /// `ENABLED` bit.
        const ENABLED = 1 << 62;
        /// `HW_SCALE_ENABLE` bit.
        const HW_SCALE_ENABLE = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl Mpambw1El1 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMBW2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpambw2El2: u64 {
        /// `nTRAP_MPAMBWSM_EL1` bit.
        const NTRAP_MPAMBWSM_EL1 = 1 << 49;
        /// `nTRAP_MPAMBW1_EL1` bit.
        const NTRAP_MPAMBW1_EL1 = 1 << 50;
        /// `nTRAP_MPAMBW0_EL1` bit.
        const NTRAP_MPAMBW0_EL1 = 1 << 51;
        /// `nTRAP_MPAMBWIDR_EL1` bit.
        const NTRAP_MPAMBWIDR_EL1 = 1 << 52;
        /// `HARDLIM` bit.
        const HARDLIM = 1 << 61;
        /// `ENABLED` bit.
        const ENABLED = 1 << 62;
        /// `HW_SCALE_ENABLE` bit.
        const HW_SCALE_ENABLE = 1 << 63;
    }
}

#[cfg(feature = "el2")]
impl Mpambw2El2 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `MPAMBW3_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mpambw3El3: u64 {
        /// `nTRAPLOWER` bit.
        const NTRAPLOWER = 1 << 49;
        /// `HARDLIM` bit.
        const HARDLIM = 1 << 61;
        /// `ENABLED` bit.
        const ENABLED = 1 << 62;
        /// `HW_SCALE_ENABLE` bit.
        const HW_SCALE_ENABLE = 1 << 63;
    }
}

#[cfg(feature = "el3")]
impl Mpambw3El3 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMBWCAP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpambwcapEl2: u64 {
        /// `ENABLED` bit.
        const ENABLED = 1 << 62;
        /// `HW_SCALE_ENABLE` bit.
        const HW_SCALE_ENABLE = 1 << 63;
    }
}

#[cfg(feature = "el2")]
impl MpambwcapEl2 {
    /// Returns the value of the `CAP` field.
    pub const fn cap(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAMBWIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpambwidrEl1: u64 {
        /// `HAS_HW_SCALE` bit.
        const HAS_HW_SCALE = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl MpambwidrEl1 {
    /// Returns the value of the `BWA_WD` field.
    pub const fn bwa_wd(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `MAX_LIM` field.
    pub const fn max_lim(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAMBWSM_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpambwsmEl1: u64 {
        /// `HARDLIM` bit.
        const HARDLIM = 1 << 61;
        /// `ENABLED` bit.
        const ENABLED = 1 << 62;
        /// `HW_SCALE_ENABLE` bit.
        const HW_SCALE_ENABLE = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl MpambwsmEl1 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAMCTL_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamctlEl1: u64 {
        /// `EN_ALT_IPARTID_EL0` bit.
        const EN_ALT_IPARTID_EL0 = 1 << 49;
        /// `EN_ALT_IPMG_EL0` bit.
        const EN_ALT_IPMG_EL0 = 1 << 50;
        /// `EN_ALT_IPARTID` bit.
        const EN_ALT_IPARTID = 1 << 51;
        /// `EN_ALT_IPMG` bit.
        const EN_ALT_IPMG = 1 << 52;
        /// `MPAMEN` bit.
        const MPAMEN = 1 << 63;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMCTL_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamctlEl2: u64 {
        /// `nTRAPMPAM1EL1` bit.
        const NTRAPMPAM1EL1 = 1 << 48;
        /// `nTRAPMPAM0EL1` bit.
        const NTRAPMPAM0EL1 = 1 << 49;
        /// `nTRAPMPAMSM` bit.
        const NTRAPMPAMSM = 1 << 50;
        /// `EN_ALT_IPARTID` bit.
        const EN_ALT_IPARTID = 1 << 51;
        /// `EN_ALT_IPMG` bit.
        const EN_ALT_IPMG = 1 << 52;
        /// `nTIDR` bit.
        const NTIDR = 1 << 58;
        /// `MPAMEN` bit.
        const MPAMEN = 1 << 63;
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `MPAMCTL_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamctlEl3: u64 {
        /// `EN_ALT_IPARTID` bit.
        const EN_ALT_IPARTID = 1 << 51;
        /// `EN_ALT_IPMG` bit.
        const EN_ALT_IPMG = 1 << 52;
        /// `nTRAPLOWER` bit.
        const NTRAPLOWER = 1 << 62;
        /// `MPAMEN` bit.
        const MPAMEN = 1 << 63;
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
    /// Returns the value of the `PARTID_MAX` field.
    pub const fn partid_max(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `VPMR_MAX` field.
    ///
    /// Indicates the maximum register index n for the `MPAMVPM<n>_EL2` registers.
    pub const fn vpmr_max(self) -> u8 {
        (self.bits() >> 18) as u8 & 0b111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MPAMSM_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamsmEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl MpamsmEl1 {
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PARTID_D` field.
    pub const fn partid_d(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG` field.
    pub const fn pmg(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PMG_D` field.
    pub const fn pmg_d(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVIDCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamvidcrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MpamvidcrEl2 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b11111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `MPAMVIDSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct MpamvidsrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl MpamvidsrEl2 {
    /// Returns the value of the `FSC` field.
    pub const fn fsc(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `FIR` field.
    pub const fn fir(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `FADDR` field.
    pub const fn faddr(self) -> u64 {
        (self.bits() >> 8) as u64 & 0b11111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `PhyPARTID0` field.
    pub const fn phypartid0(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID1` field.
    pub const fn phypartid1(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID2` field.
    pub const fn phypartid2(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID3` field.
    pub const fn phypartid3(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PhyPARTID4` field.
    pub const fn phypartid4(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID5` field.
    pub const fn phypartid5(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID6` field.
    pub const fn phypartid6(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID7` field.
    pub const fn phypartid7(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PhyPARTID8` field.
    pub const fn phypartid8(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID9` field.
    pub const fn phypartid9(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID10` field.
    pub const fn phypartid10(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID11` field.
    pub const fn phypartid11(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PhyPARTID12` field.
    pub const fn phypartid12(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID13` field.
    pub const fn phypartid13(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID14` field.
    pub const fn phypartid14(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID15` field.
    pub const fn phypartid15(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PhyPARTID16` field.
    pub const fn phypartid16(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID17` field.
    pub const fn phypartid17(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID18` field.
    pub const fn phypartid18(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID19` field.
    pub const fn phypartid19(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PhyPARTID20` field.
    pub const fn phypartid20(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID21` field.
    pub const fn phypartid21(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID22` field.
    pub const fn phypartid22(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID23` field.
    pub const fn phypartid23(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PhyPARTID24` field.
    pub const fn phypartid24(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID25` field.
    pub const fn phypartid25(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID26` field.
    pub const fn phypartid26(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID27` field.
    pub const fn phypartid27(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `PhyPARTID28` field.
    pub const fn phypartid28(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID29` field.
    pub const fn phypartid29(self) -> u16 {
        (self.bits() >> 16) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID30` field.
    pub const fn phypartid30(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `PhyPARTID31` field.
    pub const fn phypartid31(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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

bitflags! {
    /// `MPIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }
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
    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }
}

bitflags! {
    /// `MVBAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mvbar: u32 {
    }
}

impl Mvbar {
    /// Returns the value of the `Reserved` field.
    pub const fn reserved(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        (self.bits() >> 5) as u32 & 0b111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MVFR0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mvfr0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Mvfr0El1 {
    /// Returns the value of the `SIMDReg` field.
    pub const fn simdreg(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `FPSP` field.
    pub const fn fpsp(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `FPDP` field.
    pub const fn fpdp(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `FPTrap` field.
    pub const fn fptrap(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `FPDivide` field.
    pub const fn fpdivide(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `FPSqrt` field.
    pub const fn fpsqrt(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `FPShVec` field.
    pub const fn fpshvec(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `FPRound` field.
    pub const fn fpround(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MVFR1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mvfr1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Mvfr1El1 {
    /// Returns the value of the `FPFtZ` field.
    pub const fn fpftz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `FPDNaN` field.
    pub const fn fpdnan(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `SIMDLS` field.
    pub const fn simdls(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `SIMDInt` field.
    pub const fn simdint(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `SIMDSP` field.
    pub const fn simdsp(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `SIMDHP` field.
    pub const fn simdhp(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `FPHP` field.
    pub const fn fphp(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `SIMDFMAC` field.
    pub const fn simdfmac(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `MVFR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Mvfr2El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Mvfr2El1 {
    /// Returns the value of the `SIMDMisc` field.
    pub const fn simdmisc(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `FPMisc` field.
    pub const fn fpmisc(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

bitflags! {
    /// `NMRR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Nmrr: u32 {
    }
}

impl Nmrr {
    /// Returns the value of the given `IR<n>` field.
    pub const fn ir(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 2)) as u8 & 0b11
    }

    /// Returns the value of the given `OR<n>` field.
    pub const fn or(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (16 + (n - 0) * 2)) as u8 & 0b11
    }
}

bitflags! {
    /// `NSACR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
bitflags! {
    /// `NVHCRMASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NvhcrmaskEl2: u64 {
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
        /// `BSU` bit.
        const BSU = 1 << 10;
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
        /// `TGE` bit.
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
        /// `TWEDEL` bit.
        const TWEDEL = 1 << 60;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `NVHCRXMASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NvhcrxmaskEl2: u64 {
        /// `EnAS0` bit.
        const ENAS0 = 1 << 0;
        /// `EnALS` bit.
        const ENALS = 1 << 1;
        /// `EnASR` bit.
        const ENASR = 1 << 2;
        /// `FnXS` bit.
        const FNXS = 1 << 3;
        /// `FGTnXS` bit.
        const FGTNXS = 1 << 4;
        /// `SMPME` bit.
        const SMPME = 1 << 5;
        /// `TALLINT` bit.
        const TALLINT = 1 << 6;
        /// `VINMI` bit.
        const VINMI = 1 << 7;
        /// `VFNMI` bit.
        const VFNMI = 1 << 8;
        /// `CMOW` bit.
        const CMOW = 1 << 9;
        /// `MCE2` bit.
        const MCE2 = 1 << 10;
        /// `MSCEn` bit.
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
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `NVHCRX_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NvhcrxEl2: u64 {
        /// `EnAS0` bit.
        const ENAS0 = 1 << 0;
        /// `EnALS` bit.
        const ENALS = 1 << 1;
        /// `EnASR` bit.
        const ENASR = 1 << 2;
        /// `FnXS` bit.
        const FNXS = 1 << 3;
        /// `FGTnXS` bit.
        const FGTNXS = 1 << 4;
        /// `SMPME` bit.
        const SMPME = 1 << 5;
        /// `TALLINT` bit.
        const TALLINT = 1 << 6;
        /// `VINMI` bit.
        const VINMI = 1 << 7;
        /// `VFNMI` bit.
        const VFNMI = 1 << 8;
        /// `CMOW` bit.
        const CMOW = 1 << 9;
        /// `MCE2` bit.
        const MCE2 = 1 << 10;
        /// `MSCEn` bit.
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
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `NVHCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct NvhcrEl2: u64 {
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
        /// `BSU` bit.
        const BSU = 1 << 10;
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
        /// `TGE` bit.
        const TGE = 1 << 27;
        /// `TDZ` bit.
        const TDZ = 1 << 28;
        /// `HCD` bit.
        const HCD = 1 << 29;
        /// `TVRM` bit.
        const TVRM = 1 << 30;
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
        /// `TTTLBIS` bit.
        const TTTLBIS = 1 << 54;
        /// `TTTLBOS` bit.
        const TTTLBOS = 1 << 55;
        /// `ATA` bit.
        const ATA = 1 << 56;
        /// `DCT` bit.
        const DCT = 1 << 57;
        /// `TID5` bit.
        const TID5 = 1 << 58;
        /// `TWEDEn` bit.
        const TWEDEN = 1 << 59;
        /// `TWEDEL` bit.
        const TWEDEL = 1 << 60;
    }
}

bitflags! {
    /// `NZCV` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Nzcv: u64 {
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

#[cfg(feature = "el1")]
bitflags! {
    /// `OSDLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct OsdlrEl1: u64 {
        /// `DLK` bit.
        const DLK = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `OSDTRRX_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct OsdtrrxEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl OsdtrrxEl1 {
    /// Returns the value of the `DTRRX` field.
    pub const fn dtrrx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `OSDTRTX_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct OsdtrtxEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl OsdtrtxEl1 {
    /// Returns the value of the `DTRTX` field.
    pub const fn dtrtx(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `OSECCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct OseccrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl OseccrEl1 {
    /// Returns the value of the `EDECCR` field.
    pub const fn edeccr(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `OSLAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct OslarEl1: u64 {
        /// `OSLK` bit.
        const OSLK = 1 << 0;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `OSLSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct OslsrEl1: u64 {
        /// `OSLK` bit.
        const OSLK = 1 << 1;
        /// `nTT` bit.
        const NTT = 1 << 2;
    }
}

bitflags! {
    /// `PAN` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pan: u64 {
        /// `PAN` bit.
        const PAN = 1 << 22;
    }
}

bitflags! {
    /// `PAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Par: u32 {
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
    /// Returns the value of the `FST` field.
    pub const fn fst(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111111
    }

    /// Returns the value of the `FS[4:0]` field.
    pub const fn fs_4_0(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11111
    }

    /// Returns the value of the `Outer[1:0]` field.
    pub const fn outer_1_0(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b11
    }

    /// Returns the value of the `Inner[2:0]` field.
    pub const fn inner_2_0(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b111
    }

    /// Returns the value of the `ATTR` field.
    pub const fn attr(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b11111111
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
    /// Returns the value of the `FST` field.
    pub const fn fst(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b111111
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        (self.bits() >> 7) as u8 & 0b11
    }

    /// Returns the value of the `PA[47:12]` field.
    pub const fn pa_47_12(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b111111111111111111111111111111111111
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `ATTR` field.
    pub const fn attr(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PFAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PfarEl1: u64 {
        /// `NSE` bit.
        const NSE = 1 << 62;
        /// `NS` bit.
        const NS = 1 << 63;
    }
}

#[cfg(feature = "el1")]
impl PfarEl1 {
    /// Returns the value of the `PA` field.
    pub const fn pa(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b111111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `PA[55:52]` field.
    pub const fn pa_55_52(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `PFAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el2")]
impl PfarEl2 {
    /// Returns the value of the `PA` field.
    pub const fn pa(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b111111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `PA[51:48]` field.
    pub const fn pa_51_48(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `PA[55:52]` field.
    pub const fn pa_55_52(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PIRE0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pire0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Pire0El1 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `PIRE0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pire0El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Pire0El2 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PirEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PirEl1 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `PIR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PirEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl PirEl2 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `PIR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PirEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl PirEl3 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

bitflags! {
    /// `PM` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pm: u64 {
        /// `PM` bit.
        const PM = 1 << 32;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMBIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmbidrEl1: u64 {
        /// `P` bit.
        const P = 1 << 4;
        /// `F` bit.
        const F = 1 << 5;
    }
}

#[cfg(feature = "el1")]
impl PmbidrEl1 {
    /// Returns the value of the `Align` field.
    pub const fn align(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `AddrMode` field.
    pub const fn addrmode(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `EA` field.
    pub const fn ea(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `MaxBuffSize` field.
    pub const fn maxbuffsize(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMBLIMITR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmblimitrEl1: u64 {
        /// `E` bit.
        const E = 1 << 0;
        /// `PMFZ` bit.
        const PMFZ = 1 << 5;
        /// `nVM` bit.
        const NVM = 1 << 7;
    }
}

#[cfg(feature = "el1")]
impl PmblimitrEl1 {
    /// Returns the value of the `FM` field.
    pub const fn fm(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `LIMIT` field.
    pub const fn limit(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b1111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMBMAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmbmarEl1: u64 {
        /// `TTBA` bit.
        const TTBA = 1 << 32;
    }
}

#[cfg(feature = "el1")]
impl PmbmarEl1 {
    /// Returns the value of the `Attr` field.
    pub const fn attr(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 33) as u8 & 0b1111111
    }

    /// Returns the value of the `FPOIndex` field.
    pub const fn fpoindex(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMBPTR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmbptrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PmbptrEl1 {
    /// Returns the value of the `PTR` field.
    pub const fn ptr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMBSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmbsrEl1: u64 {
        /// `COLL` bit.
        const COLL = 1 << 16;
        /// `S` bit.
        const S = 1 << 17;
        /// `EA` bit.
        const EA = 1 << 18;
        /// `DL` bit.
        const DL = 1 << 19;
    }
}

#[cfg(feature = "el1")]
impl PmbsrEl1 {
    /// Returns the value of the `MSS` field.
    pub const fn mss(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `MSS2` field.
    pub const fn mss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `PMBSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmbsrEl2: u64 {
        /// `COLL` bit.
        const COLL = 1 << 16;
        /// `S` bit.
        const S = 1 << 17;
        /// `EA` bit.
        const EA = 1 << 18;
        /// `DL` bit.
        const DL = 1 << 19;
    }
}

#[cfg(feature = "el2")]
impl PmbsrEl2 {
    /// Returns the value of the `MSS` field.
    pub const fn mss(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `MSS2` field.
    pub const fn mss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `PMBSR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmbsrEl3: u64 {
        /// `COLL` bit.
        const COLL = 1 << 16;
        /// `S` bit.
        const S = 1 << 17;
        /// `EA` bit.
        const EA = 1 << 18;
        /// `DL` bit.
        const DL = 1 << 19;
    }
}

#[cfg(feature = "el3")]
impl PmbsrEl3 {
    /// Returns the value of the `MSS` field.
    pub const fn mss(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `MSS2` field.
    pub const fn mss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
    }
}

bitflags! {
    /// `PMCCFILTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMCCFILTR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmccfiltrEl0: u64 {
        /// `RLH` bit.
        const RLH = 1 << 20;
        /// `RLU` bit.
        const RLU = 1 << 21;
        /// `RLK` bit.
        const RLK = 1 << 22;
        /// `SH` bit.
        const SH = 1 << 24;
        /// `M` bit.
        const M = 1 << 26;
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

impl PmccfiltrEl0 {
    /// Returns the value of the `VS` field.
    pub const fn vs(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b11
    }
}

bitflags! {
    /// `PMCCNTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmccntr: u32 {
    }
}

impl Pmccntr {
    /// Returns the value of the `CCNT` field.
    pub const fn ccnt(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `PMCCNTR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmccntrEl0: u64 {
    }
}

impl PmccntrEl0 {
    /// Returns the value of the `CCNT` field.
    pub const fn ccnt(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMCCNTSVR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmccntsvrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PmccntsvrEl1 {
    /// Returns the value of the `CCNT` field.
    pub const fn ccnt(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `PMCEID0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMCEID0_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid0El0: u64 {
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
        /// `IDhi<n>` bit 0.
        const IDHI0 = 1 << 32;
        /// `IDhi<n>` bit 1.
        const IDHI1 = 1 << 33;
        /// `IDhi<n>` bit 2.
        const IDHI2 = 1 << 34;
        /// `IDhi<n>` bit 3.
        const IDHI3 = 1 << 35;
        /// `IDhi<n>` bit 4.
        const IDHI4 = 1 << 36;
        /// `IDhi<n>` bit 5.
        const IDHI5 = 1 << 37;
        /// `IDhi<n>` bit 6.
        const IDHI6 = 1 << 38;
        /// `IDhi<n>` bit 7.
        const IDHI7 = 1 << 39;
        /// `IDhi<n>` bit 8.
        const IDHI8 = 1 << 40;
        /// `IDhi<n>` bit 9.
        const IDHI9 = 1 << 41;
        /// `IDhi<n>` bit 10.
        const IDHI10 = 1 << 42;
        /// `IDhi<n>` bit 11.
        const IDHI11 = 1 << 43;
        /// `IDhi<n>` bit 12.
        const IDHI12 = 1 << 44;
        /// `IDhi<n>` bit 13.
        const IDHI13 = 1 << 45;
        /// `IDhi<n>` bit 14.
        const IDHI14 = 1 << 46;
        /// `IDhi<n>` bit 15.
        const IDHI15 = 1 << 47;
        /// `IDhi<n>` bit 16.
        const IDHI16 = 1 << 48;
        /// `IDhi<n>` bit 17.
        const IDHI17 = 1 << 49;
        /// `IDhi<n>` bit 18.
        const IDHI18 = 1 << 50;
        /// `IDhi<n>` bit 19.
        const IDHI19 = 1 << 51;
        /// `IDhi<n>` bit 20.
        const IDHI20 = 1 << 52;
        /// `IDhi<n>` bit 21.
        const IDHI21 = 1 << 53;
        /// `IDhi<n>` bit 22.
        const IDHI22 = 1 << 54;
        /// `IDhi<n>` bit 23.
        const IDHI23 = 1 << 55;
        /// `IDhi<n>` bit 24.
        const IDHI24 = 1 << 56;
        /// `IDhi<n>` bit 25.
        const IDHI25 = 1 << 57;
        /// `IDhi<n>` bit 26.
        const IDHI26 = 1 << 58;
        /// `IDhi<n>` bit 27.
        const IDHI27 = 1 << 59;
        /// `IDhi<n>` bit 28.
        const IDHI28 = 1 << 60;
        /// `IDhi<n>` bit 29.
        const IDHI29 = 1 << 61;
        /// `IDhi<n>` bit 30.
        const IDHI30 = 1 << 62;
        /// `IDhi<n>` bit 31.
        const IDHI31 = 1 << 63;
    }
}

bitflags! {
    /// `PMCEID1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMCEID1_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmceid1El0: u64 {
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
        /// `IDhi<n>` bit 0.
        const IDHI0 = 1 << 32;
        /// `IDhi<n>` bit 1.
        const IDHI1 = 1 << 33;
        /// `IDhi<n>` bit 2.
        const IDHI2 = 1 << 34;
        /// `IDhi<n>` bit 3.
        const IDHI3 = 1 << 35;
        /// `IDhi<n>` bit 4.
        const IDHI4 = 1 << 36;
        /// `IDhi<n>` bit 5.
        const IDHI5 = 1 << 37;
        /// `IDhi<n>` bit 6.
        const IDHI6 = 1 << 38;
        /// `IDhi<n>` bit 7.
        const IDHI7 = 1 << 39;
        /// `IDhi<n>` bit 8.
        const IDHI8 = 1 << 40;
        /// `IDhi<n>` bit 9.
        const IDHI9 = 1 << 41;
        /// `IDhi<n>` bit 10.
        const IDHI10 = 1 << 42;
        /// `IDhi<n>` bit 11.
        const IDHI11 = 1 << 43;
        /// `IDhi<n>` bit 12.
        const IDHI12 = 1 << 44;
        /// `IDhi<n>` bit 13.
        const IDHI13 = 1 << 45;
        /// `IDhi<n>` bit 14.
        const IDHI14 = 1 << 46;
        /// `IDhi<n>` bit 15.
        const IDHI15 = 1 << 47;
        /// `IDhi<n>` bit 16.
        const IDHI16 = 1 << 48;
        /// `IDhi<n>` bit 17.
        const IDHI17 = 1 << 49;
        /// `IDhi<n>` bit 18.
        const IDHI18 = 1 << 50;
        /// `IDhi<n>` bit 19.
        const IDHI19 = 1 << 51;
        /// `IDhi<n>` bit 20.
        const IDHI20 = 1 << 52;
        /// `IDhi<n>` bit 21.
        const IDHI21 = 1 << 53;
        /// `IDhi<n>` bit 22.
        const IDHI22 = 1 << 54;
        /// `IDhi<n>` bit 23.
        const IDHI23 = 1 << 55;
        /// `IDhi<n>` bit 24.
        const IDHI24 = 1 << 56;
        /// `IDhi<n>` bit 25.
        const IDHI25 = 1 << 57;
        /// `IDhi<n>` bit 26.
        const IDHI26 = 1 << 58;
        /// `IDhi<n>` bit 27.
        const IDHI27 = 1 << 59;
        /// `IDhi<n>` bit 28.
        const IDHI28 = 1 << 60;
        /// `IDhi<n>` bit 29.
        const IDHI29 = 1 << 61;
        /// `IDhi<n>` bit 30.
        const IDHI30 = 1 << 62;
        /// `IDhi<n>` bit 31.
        const IDHI31 = 1 << 63;
    }
}

bitflags! {
    /// `PMCEID2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMCEID3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMCNTENCLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMCNTENCLR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmcntenclrEl0: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

bitflags! {
    /// `PMCNTENSET` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMCNTENSET_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmcntensetEl0: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

bitflags! {
    /// `PMCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        (self.bits() >> 11) as u8 & 0b11111
    }

    /// Returns the value of the `IDCODE` field.
    pub const fn idcode(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `IMP` field.
    pub const fn imp(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
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
        (self.bits() >> 11) as u8 & 0b11111
    }

    /// Returns the value of the `IDCODE` field.
    pub const fn idcode(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `IMP` field.
    pub const fn imp(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMECR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmecrEl1: u64 {
        /// `KPME` bit.
        const KPME = 1 << 2;
    }
}

#[cfg(feature = "el1")]
impl PmecrEl1 {
    /// Returns the value of the `PMEE` field.
    pub const fn pmee(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `SSE` field.
    pub const fn sse(self) -> u8 {
        (self.bits() >> 3) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMIAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmiarEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PmiarEl1 {
    /// Returns the value of the `ADDRESS` field.
    pub const fn address(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `PMICFILTR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmicfiltrEl0: u64 {
        /// `RLH` bit.
        const RLH = 1 << 20;
        /// `RLU` bit.
        const RLU = 1 << 21;
        /// `RLK` bit.
        const RLK = 1 << 22;
        /// `SH` bit.
        const SH = 1 << 24;
        /// `M` bit.
        const M = 1 << 26;
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
        /// `SYNC` bit.
        const SYNC = 1 << 58;
    }
}

impl PmicfiltrEl0 {
    /// Returns the value of the `evtCount` field.
    pub const fn evtcount(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `VS` field.
    pub const fn vs(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b11
    }
}

bitflags! {
    /// `PMICNTR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmicntrEl0: u64 {
    }
}

impl PmicntrEl0 {
    /// Returns the value of the `ICNT` field.
    pub const fn icnt(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMICNTSVR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmicntsvrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PmicntsvrEl1 {
    /// Returns the value of the `ICNT` field.
    pub const fn icnt(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `PMINTENCLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el1")]
bitflags! {
    /// `PMINTENCLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmintenclrEl1: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

bitflags! {
    /// `PMINTENSET` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el1")]
bitflags! {
    /// `PMINTENSET_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmintensetEl1: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

bitflags! {
    /// `PMMIR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmmir: u32 {
    }
}

impl Pmmir {
    /// Returns the value of the `SLOTS` field.
    pub const fn slots(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `BUS_SLOTS` field.
    pub const fn bus_slots(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `BUS_WIDTH` field.
    pub const fn bus_width(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `THWIDTH` field.
    pub const fn thwidth(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `EDGE` field.
    pub const fn edge(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMMIR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmmirEl1: u64 {
        /// `SME` bit.
        const SME = 1 << 28;
    }
}

#[cfg(feature = "el1")]
impl PmmirEl1 {
    /// Returns the value of the `SLOTS` field.
    pub const fn slots(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `BUS_SLOTS` field.
    pub const fn bus_slots(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `BUS_WIDTH` field.
    pub const fn bus_width(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `THWIDTH` field.
    pub const fn thwidth(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `EDGE` field.
    pub const fn edge(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }
}

bitflags! {
    /// `PMOVSCLR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmovsclrEl0: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

bitflags! {
    /// `PMOVSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMOVSSET` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMOVSSET_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmovssetEl0: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmscrEl1: u64 {
        /// `E0SPE` bit.
        const E0SPE = 1 << 0;
        /// `E1SPE` bit.
        const E1SPE = 1 << 1;
        /// `CX` bit.
        const CX = 1 << 3;
        /// `PA` bit.
        const PA = 1 << 4;
        /// `TS` bit.
        const TS = 1 << 5;
        /// `KE` bit.
        const KE = 1 << 10;
        /// `EnVM` bit.
        const ENVM = 1 << 11;
    }
}

#[cfg(feature = "el1")]
impl PmscrEl1 {
    /// Returns the value of the `PCT` field.
    pub const fn pct(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `EE` field.
    pub const fn ee(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `PMSCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmscrEl2: u64 {
        /// `E0HSPE` bit.
        const E0HSPE = 1 << 0;
        /// `E2SPE` bit.
        const E2SPE = 1 << 1;
        /// `CX` bit.
        const CX = 1 << 3;
        /// `PA` bit.
        const PA = 1 << 4;
        /// `TS` bit.
        const TS = 1 << 5;
        /// `KE` bit.
        const KE = 1 << 10;
        /// `EnVM` bit.
        const ENVM = 1 << 11;
    }
}

#[cfg(feature = "el2")]
impl PmscrEl2 {
    /// Returns the value of the `PCT` field.
    pub const fn pct(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `EE` field.
    pub const fn ee(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSDSFR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsdsfrEl1: u64 {
        /// `S<m>` bit 0.
        const S0 = 1 << 0;
        /// `S<m>` bit 1.
        const S1 = 1 << 1;
        /// `S<m>` bit 2.
        const S2 = 1 << 2;
        /// `S<m>` bit 3.
        const S3 = 1 << 3;
        /// `S<m>` bit 4.
        const S4 = 1 << 4;
        /// `S<m>` bit 5.
        const S5 = 1 << 5;
        /// `S<m>` bit 6.
        const S6 = 1 << 6;
        /// `S<m>` bit 7.
        const S7 = 1 << 7;
        /// `S<m>` bit 8.
        const S8 = 1 << 8;
        /// `S<m>` bit 9.
        const S9 = 1 << 9;
        /// `S<m>` bit 10.
        const S10 = 1 << 10;
        /// `S<m>` bit 11.
        const S11 = 1 << 11;
        /// `S<m>` bit 12.
        const S12 = 1 << 12;
        /// `S<m>` bit 13.
        const S13 = 1 << 13;
        /// `S<m>` bit 14.
        const S14 = 1 << 14;
        /// `S<m>` bit 15.
        const S15 = 1 << 15;
        /// `S<m>` bit 16.
        const S16 = 1 << 16;
        /// `S<m>` bit 17.
        const S17 = 1 << 17;
        /// `S<m>` bit 18.
        const S18 = 1 << 18;
        /// `S<m>` bit 19.
        const S19 = 1 << 19;
        /// `S<m>` bit 20.
        const S20 = 1 << 20;
        /// `S<m>` bit 21.
        const S21 = 1 << 21;
        /// `S<m>` bit 22.
        const S22 = 1 << 22;
        /// `S<m>` bit 23.
        const S23 = 1 << 23;
        /// `S<m>` bit 24.
        const S24 = 1 << 24;
        /// `S<m>` bit 25.
        const S25 = 1 << 25;
        /// `S<m>` bit 26.
        const S26 = 1 << 26;
        /// `S<m>` bit 27.
        const S27 = 1 << 27;
        /// `S<m>` bit 28.
        const S28 = 1 << 28;
        /// `S<m>` bit 29.
        const S29 = 1 << 29;
        /// `S<m>` bit 30.
        const S30 = 1 << 30;
        /// `S<m>` bit 31.
        const S31 = 1 << 31;
        /// `S<m>` bit 32.
        const S32 = 1 << 32;
        /// `S<m>` bit 33.
        const S33 = 1 << 33;
        /// `S<m>` bit 34.
        const S34 = 1 << 34;
        /// `S<m>` bit 35.
        const S35 = 1 << 35;
        /// `S<m>` bit 36.
        const S36 = 1 << 36;
        /// `S<m>` bit 37.
        const S37 = 1 << 37;
        /// `S<m>` bit 38.
        const S38 = 1 << 38;
        /// `S<m>` bit 39.
        const S39 = 1 << 39;
        /// `S<m>` bit 40.
        const S40 = 1 << 40;
        /// `S<m>` bit 41.
        const S41 = 1 << 41;
        /// `S<m>` bit 42.
        const S42 = 1 << 42;
        /// `S<m>` bit 43.
        const S43 = 1 << 43;
        /// `S<m>` bit 44.
        const S44 = 1 << 44;
        /// `S<m>` bit 45.
        const S45 = 1 << 45;
        /// `S<m>` bit 46.
        const S46 = 1 << 46;
        /// `S<m>` bit 47.
        const S47 = 1 << 47;
        /// `S<m>` bit 48.
        const S48 = 1 << 48;
        /// `S<m>` bit 49.
        const S49 = 1 << 49;
        /// `S<m>` bit 50.
        const S50 = 1 << 50;
        /// `S<m>` bit 51.
        const S51 = 1 << 51;
        /// `S<m>` bit 52.
        const S52 = 1 << 52;
        /// `S<m>` bit 53.
        const S53 = 1 << 53;
        /// `S<m>` bit 54.
        const S54 = 1 << 54;
        /// `S<m>` bit 55.
        const S55 = 1 << 55;
        /// `S<m>` bit 56.
        const S56 = 1 << 56;
        /// `S<m>` bit 57.
        const S57 = 1 << 57;
        /// `S<m>` bit 58.
        const S58 = 1 << 58;
        /// `S<m>` bit 59.
        const S59 = 1 << 59;
        /// `S<m>` bit 60.
        const S60 = 1 << 60;
        /// `S<m>` bit 61.
        const S61 = 1 << 61;
        /// `S<m>` bit 62.
        const S62 = 1 << 62;
        /// `S<m>` bit 63.
        const S63 = 1 << 63;
    }
}

bitflags! {
    /// `PMSELR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmselr: u32 {
    }
}

impl Pmselr {
    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

bitflags! {
    /// `PMSELR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmselrEl0: u64 {
    }
}

impl PmselrEl0 {
    /// Returns the value of the `SEL` field.
    pub const fn sel(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSEVFR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsevfrEl1: u64 {
        /// `E[1]` bit.
        const E_1 = 1 << 1;
        /// `E[2]` bit.
        const E_2 = 1 << 2;
        /// `E[3]` bit.
        const E_3 = 1 << 3;
        /// `E[4]` bit.
        const E_4 = 1 << 4;
        /// `E[5]` bit.
        const E_5 = 1 << 5;
        /// `E[6]` bit.
        const E_6 = 1 << 6;
        /// `E[7]` bit.
        const E_7 = 1 << 7;
        /// `E[8]` bit.
        const E_8 = 1 << 8;
        /// `E[9]` bit.
        const E_9 = 1 << 9;
        /// `E[10]` bit.
        const E_10 = 1 << 10;
        /// `E[11]` bit.
        const E_11 = 1 << 11;
        /// `E[12]` bit.
        const E_12 = 1 << 12;
        /// `E[13]` bit.
        const E_13 = 1 << 13;
        /// `E[14]` bit.
        const E_14 = 1 << 14;
        /// `E[15]` bit.
        const E_15 = 1 << 15;
        /// `E[17]` bit.
        const E_17 = 1 << 17;
        /// `E[18]` bit.
        const E_18 = 1 << 18;
        /// `E[19]` bit.
        const E_19 = 1 << 19;
        /// `E[20]` bit.
        const E_20 = 1 << 20;
        /// `E[21]` bit.
        const E_21 = 1 << 21;
        /// `E[22]` bit.
        const E_22 = 1 << 22;
        /// `E[23]` bit.
        const E_23 = 1 << 23;
        /// `E[24]` bit.
        const E_24 = 1 << 24;
        /// `E[25]` bit.
        const E_25 = 1 << 25;
        /// `E[26]` bit.
        const E_26 = 1 << 26;
        /// `E[27]` bit.
        const E_27 = 1 << 27;
        /// `E[28]` bit.
        const E_28 = 1 << 28;
        /// `E[29]` bit.
        const E_29 = 1 << 29;
        /// `E[30]` bit.
        const E_30 = 1 << 30;
        /// `E[31]` bit.
        const E_31 = 1 << 31;
        /// `E[48]` bit.
        const E_48 = 1 << 48;
        /// `E[49]` bit.
        const E_49 = 1 << 49;
        /// `E[50]` bit.
        const E_50 = 1 << 50;
        /// `E[51]` bit.
        const E_51 = 1 << 51;
        /// `E[52]` bit.
        const E_52 = 1 << 52;
        /// `E[53]` bit.
        const E_53 = 1 << 53;
        /// `E[54]` bit.
        const E_54 = 1 << 54;
        /// `E[55]` bit.
        const E_55 = 1 << 55;
        /// `E[56]` bit.
        const E_56 = 1 << 56;
        /// `E[57]` bit.
        const E_57 = 1 << 57;
        /// `E[58]` bit.
        const E_58 = 1 << 58;
        /// `E[59]` bit.
        const E_59 = 1 << 59;
        /// `E[60]` bit.
        const E_60 = 1 << 60;
        /// `E[61]` bit.
        const E_61 = 1 << 61;
        /// `E[62]` bit.
        const E_62 = 1 << 62;
        /// `E[63]` bit.
        const E_63 = 1 << 63;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSFCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsfcrEl1: u64 {
        /// `FE` bit.
        const FE = 1 << 0;
        /// `FT` bit.
        const FT = 1 << 1;
        /// `FL` bit.
        const FL = 1 << 2;
        /// `FnE` bit.
        const FNE = 1 << 3;
        /// `FDS` bit.
        const FDS = 1 << 4;
    }
}

#[cfg(feature = "el1")]
impl PmsfcrEl1 {
    /// Returns the value of the `TYPE` field.
    pub const fn type_(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111
    }

    /// Returns the value of the `TYPEm` field.
    pub const fn typem(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSICR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsicrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PmsicrEl1 {
    /// Returns the value of the `COUNT` field.
    pub const fn count(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }

    /// Returns the value of the `ECOUNT` field.
    pub const fn ecount(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsidrEl1: u64 {
        /// `FE` bit.
        const FE = 1 << 0;
        /// `FT` bit.
        const FT = 1 << 1;
        /// `FL` bit.
        const FL = 1 << 2;
        /// `ArchInst` bit.
        const ARCHINST = 1 << 3;
        /// `LDS` bit.
        const LDS = 1 << 4;
        /// `ERnd` bit.
        const ERND = 1 << 5;
        /// `FnE` bit.
        const FNE = 1 << 6;
        /// `FDS` bit.
        const FDS = 1 << 7;
        /// `PBT` bit.
        const PBT = 1 << 24;
        /// `CRR` bit.
        const CRR = 1 << 25;
        /// `EFT` bit.
        const EFT = 1 << 26;
        /// `FPF` bit.
        const FPF = 1 << 27;
        /// `SME` bit.
        const SME = 1 << 32;
    }
}

#[cfg(feature = "el1")]
impl PmsidrEl1 {
    /// Returns the value of the `Interval` field.
    pub const fn interval(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `MaxSize` field.
    pub const fn maxsize(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `CountSize` field.
    pub const fn countsize(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Format` field.
    pub const fn format(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `ALTCLK` field.
    pub const fn altclk(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSIRR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsirrEl1: u64 {
        /// `RND` bit.
        const RND = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl PmsirrEl1 {
    /// Returns the value of the `INTERVAL` field.
    pub const fn interval(self) -> u32 {
        (self.bits() >> 8) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSLATFR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmslatfrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PmslatfrEl1 {
    /// Returns the value of the `MINLAT` field.
    pub const fn minlat(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSNEVFR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsnevfrEl1: u64 {
        /// `E[1]` bit.
        const E_1 = 1 << 1;
        /// `E[2]` bit.
        const E_2 = 1 << 2;
        /// `E[3]` bit.
        const E_3 = 1 << 3;
        /// `E[4]` bit.
        const E_4 = 1 << 4;
        /// `E[5]` bit.
        const E_5 = 1 << 5;
        /// `E[6]` bit.
        const E_6 = 1 << 6;
        /// `E[7]` bit.
        const E_7 = 1 << 7;
        /// `E[8]` bit.
        const E_8 = 1 << 8;
        /// `E[9]` bit.
        const E_9 = 1 << 9;
        /// `E[10]` bit.
        const E_10 = 1 << 10;
        /// `E[11]` bit.
        const E_11 = 1 << 11;
        /// `E[12]` bit.
        const E_12 = 1 << 12;
        /// `E[13]` bit.
        const E_13 = 1 << 13;
        /// `E[14]` bit.
        const E_14 = 1 << 14;
        /// `E[15]` bit.
        const E_15 = 1 << 15;
        /// `E[17]` bit.
        const E_17 = 1 << 17;
        /// `E[18]` bit.
        const E_18 = 1 << 18;
        /// `E[19]` bit.
        const E_19 = 1 << 19;
        /// `E[20]` bit.
        const E_20 = 1 << 20;
        /// `E[21]` bit.
        const E_21 = 1 << 21;
        /// `E[22]` bit.
        const E_22 = 1 << 22;
        /// `E[23]` bit.
        const E_23 = 1 << 23;
        /// `E[24]` bit.
        const E_24 = 1 << 24;
        /// `E[25]` bit.
        const E_25 = 1 << 25;
        /// `E[26]` bit.
        const E_26 = 1 << 26;
        /// `E[27]` bit.
        const E_27 = 1 << 27;
        /// `E[28]` bit.
        const E_28 = 1 << 28;
        /// `E[29]` bit.
        const E_29 = 1 << 29;
        /// `E[30]` bit.
        const E_30 = 1 << 30;
        /// `E[31]` bit.
        const E_31 = 1 << 31;
        /// `E[48]` bit.
        const E_48 = 1 << 48;
        /// `E[49]` bit.
        const E_49 = 1 << 49;
        /// `E[50]` bit.
        const E_50 = 1 << 50;
        /// `E[51]` bit.
        const E_51 = 1 << 51;
        /// `E[52]` bit.
        const E_52 = 1 << 52;
        /// `E[53]` bit.
        const E_53 = 1 << 53;
        /// `E[54]` bit.
        const E_54 = 1 << 54;
        /// `E[55]` bit.
        const E_55 = 1 << 55;
        /// `E[56]` bit.
        const E_56 = 1 << 56;
        /// `E[57]` bit.
        const E_57 = 1 << 57;
        /// `E[58]` bit.
        const E_58 = 1 << 58;
        /// `E[59]` bit.
        const E_59 = 1 << 59;
        /// `E[60]` bit.
        const E_60 = 1 << 60;
        /// `E[61]` bit.
        const E_61 = 1 << 61;
        /// `E[62]` bit.
        const E_62 = 1 << 62;
        /// `E[63]` bit.
        const E_63 = 1 << 63;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `PMSSCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmsscrEl1: u64 {
        /// `SS` bit.
        const SS = 1 << 0;
        /// `NC` bit.
        const NC = 1 << 32;
    }
}

bitflags! {
    /// `PMSWINC` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMSWINC_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmswincEl0: u64 {
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

#[cfg(feature = "el1")]
bitflags! {
    /// `PMUACR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmuacrEl1: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

bitflags! {
    /// `PMUSERENR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `PMUSERENR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmuserenrEl0: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
        /// `SW` bit.
        const SW = 1 << 1;
        /// `CR` bit.
        const CR = 1 << 2;
        /// `ER` bit.
        const ER = 1 << 3;
        /// `UEN` bit.
        const UEN = 1 << 4;
        /// `IR` bit.
        const IR = 1 << 5;
        /// `TID` bit.
        const TID = 1 << 6;
    }
}

bitflags! {
    /// `PMXEVCNTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmxevcntr: u32 {
    }
}

impl Pmxevcntr {
    /// Returns the value of the `PMEVCNTR<n>` field.
    pub const fn pmevcntr<n>(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `PMXEVTYPER` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Pmxevtyper: u32 {
    }
}

impl Pmxevtyper {
    /// Returns the value of the `ETR` field.
    pub const fn etr(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `PMXEVTYPER_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmxevtyperEl0: u64 {
    }
}

impl PmxevtyperEl0 {
    /// Returns the value of the `EVTYPERn` field.
    pub const fn evtypern(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `PMZR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PmzrEl0: u64 {
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
        /// `F0` bit.
        const F0 = 1 << 32;
    }
}

bitflags! {
    /// `POR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PorEl0: u64 {
    }
}

impl PorEl0 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `POR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PorEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl PorEl1 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `POR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PorEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl PorEl2 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `POR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct PorEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl PorEl3 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

bitflags! {
    /// `PRRR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the given `TR<n>` field.
    pub const fn tr(self, n: u32) -> u8 {
        assert!(n < 8);
        (self.bits() >> (0 + (n - 0) * 2)) as u8 & 0b11
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
    /// Returns the value of the `TAG` field.
    pub const fn tag(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SEED` field.
    pub const fn seed(self) -> u16 {
        (self.bits() >> 8) as u16 & 0b1111111111111111
    }
}

bitflags! {
    /// `RMR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rmr: u32 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `RMR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RmrEl1: u64 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `RMR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RmrEl2: u64 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `RMR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RmrEl3: u64 {
        /// `AA64` bit.
        const AA64 = 1 << 0;
        /// `RR` bit.
        const RR = 1 << 1;
    }
}

bitflags! {
    /// `RNDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rndr: u64 {
    }
}

impl Rndr {
    /// Returns the value of the `RNDR` field.
    pub const fn rndr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `RNDRRS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rndrrs: u64 {
    }
}

impl Rndrrs {
    /// Returns the value of the `RNDRRS` field.
    pub const fn rndrrs(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `RVBAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Rvbar: u32 {
        /// RES1 bits in the `RVBAR` register.
        const RES1 = 0b1;
    }
}

impl Rvbar {
    /// Returns the value of the `ResetAddress` field.
    pub const fn resetaddress(self) -> u32 {
        (self.bits() >> 1) as u32 & 0b1111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `RVBAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RvbarEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl RvbarEl1 {
    /// Returns the value of the `ResetAddress` field.
    pub const fn resetaddress(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `RVBAR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RvbarEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl RvbarEl2 {
    /// Returns the value of the `ResetAddress` field.
    pub const fn resetaddress(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `RVBAR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct RvbarEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl RvbarEl3 {
    /// Returns the value of the `ResetAddress` field.
    pub const fn resetaddress(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `S2PIR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct S2pirEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl S2pirEl2 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `S2POR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct S2porEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl S2porEl1 {
    /// Returns the value of the given `Perm<m>` field.
    pub const fn perm(self, m: u32) -> u8 {
        assert!(m < 16);
        (self.bits() >> (0 + (m - 0) * 4)) as u8 & 0b1111
    }
}

bitflags! {
    /// `SCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el3")]
bitflags! {
    /// `SCR2_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Scr2El3: u64 {
        /// `FDIT` bit.
        const FDIT = 1 << 0;
        /// `NV3En` bit.
        const NV3EN = 1 << 1;
        /// `SRMASK2En` bit.
        const SRMASK2EN = 1 << 2;
        /// `VTE` bit.
        const VTE = 1 << 3;
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
        /// Enable acivity monitors virtual offsets.
        const AMVOFFEN = 1 << 35;
        /// Enable ST64BV0 at lower ELs.
        const ENAS0 = 1 << 36;
        /// Enable ACCDATA_EL1 at lower ELs.
        const ADEN = 1 << 37;
        /// Enable HCRX_EL2.
        const HXEN = 1 << 38;
        /// Enable gaurded control stack.
        const GCSEN = 1 << 39;
        /// Trap RNDR and RNDRRS to EL3.
        const TRNDR = 1 << 40;
        /// Enable TPIDR2_EL0 at lower ELs.
        const ENTP2 = 1 << 41;
        /// Enable RCW and RCWS mask registers at lower ELs.
        const RCWMASKEN = 1 << 42;
        /// Enable TCR2_ELx registers at lower ELs.
        const TCR2EN = 1 << 43;
        /// Enable SCTLR2_ELx rogisters at lower ELs.
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
    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b1111
    }
}

bitflags! {
    /// `SCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el1")]
bitflags! {
    /// `SCTLR2MASK_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr2maskEl1: u64 {
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

#[cfg(feature = "el2")]
bitflags! {
    /// `SCTLR2MASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr2maskEl2: u64 {
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

#[cfg(feature = "el1")]
bitflags! {
    /// `SCTLR2_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sctlr2El1: u64 {
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

#[cfg(feature = "el2")]
bitflags! {
    /// `SCTLR2_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el3")]
bitflags! {
    /// `SCTLR2_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(feature = "el1")]
bitflags! {
    /// `SCTLRMASK_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SctlrmaskEl1: u64 {
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
        /// `SPAN` bit.
        const SPAN = 1 << 23;
        /// `E0E` bit.
        const E0E = 1 << 24;
        /// `EE` bit.
        const EE = 1 << 25;
        /// `UCI` bit.
        const UCI = 1 << 26;
        /// `EnDA` bit.
        const ENDA = 1 << 27;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 28;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 29;
        /// `EnIB` bit.
        const ENIB = 1 << 30;
        /// `EnIA` bit.
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
        /// `TCF0` bit.
        const TCF0 = 1 << 38;
        /// `TCF` bit.
        const TCF = 1 << 40;
        /// `ATA0` bit.
        const ATA0 = 1 << 42;
        /// `ATA` bit.
        const ATA = 1 << 43;
        /// `DSSBS` bit.
        const DSSBS = 1 << 44;
        /// `TWEDEn` bit.
        const TWEDEN = 1 << 45;
        /// `TWEDEL` bit.
        const TWEDEL = 1 << 46;
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
        /// `SPINTMASK` bit.
        const SPINTMASK = 1 << 62;
        /// `TIDCP` bit.
        const TIDCP = 1 << 63;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SCTLRMASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SctlrmaskEl2: u64 {
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
        /// `SPAN` bit.
        const SPAN = 1 << 23;
        /// `E0E` bit.
        const E0E = 1 << 24;
        /// `EE` bit.
        const EE = 1 << 25;
        /// `UCI` bit.
        const UCI = 1 << 26;
        /// `EnDA` bit.
        const ENDA = 1 << 27;
        /// `nTLSMD` bit.
        const NTLSMD = 1 << 28;
        /// `LSMAOE` bit.
        const LSMAOE = 1 << 29;
        /// `EnIB` bit.
        const ENIB = 1 << 30;
        /// `EnIA` bit.
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
        /// `TCF0` bit.
        const TCF0 = 1 << 38;
        /// `TCF` bit.
        const TCF = 1 << 40;
        /// `ATA0` bit.
        const ATA0 = 1 << 42;
        /// `ATA` bit.
        const ATA = 1 << 43;
        /// `DSSBS` bit.
        const DSSBS = 1 << 44;
        /// `TWEDEn` bit.
        const TWEDEN = 1 << 45;
        /// `TWEDEL` bit.
        const TWEDEL = 1 << 46;
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
        /// `SPINTMASK` bit.
        const SPINTMASK = 1 << 62;
        /// `TIDCP` bit.
        const TIDCP = 1 << 63;
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
    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        (self.bits() >> 38) as u8 & 0b11
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> 46) as u8 & 0b1111
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
    /// Returns the value of the `TCF0` field.
    pub const fn tcf0(self) -> u8 {
        (self.bits() >> 38) as u8 & 0b11
    }

    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11
    }

    /// Returns the value of the `TWEDEL` field.
    pub const fn twedel(self) -> u8 {
        (self.bits() >> 46) as u8 & 0b1111
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
    /// Returns the value of the `TCF` field.
    pub const fn tcf(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11
    }
}

bitflags! {
    /// `SCXTNUM_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ScxtnumEl0: u64 {
    }
}

impl ScxtnumEl0 {
    /// Returns the value of the `SCXTNUM` field.
    pub const fn scxtnum(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SCXTNUM_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ScxtnumEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ScxtnumEl1 {
    /// Returns the value of the `SCXTNUM` field.
    pub const fn scxtnum(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SCXTNUM_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ScxtnumEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl ScxtnumEl2 {
    /// Returns the value of the `SCXTNUM` field.
    pub const fn scxtnum(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `SCXTNUM_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ScxtnumEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl ScxtnumEl3 {
    /// Returns the value of the `SCXTNUM` field.
    pub const fn scxtnum(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `SDCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `SPD` field.
    pub const fn spd(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }
}

bitflags! {
    /// `SDER` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sder: u32 {
        /// `SUIDEN` bit.
        const SUIDEN = 1 << 0;
        /// `SUNIDEN` bit.
        const SUNIDEN = 1 << 1;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SDER32_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sder32El2: u64 {
        /// `SUIDEN` bit.
        const SUIDEN = 1 << 0;
        /// `SUNIDEN` bit.
        const SUNIDEN = 1 << 1;
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `SDER32_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Sder32El3: u64 {
        /// `SUIDEN` bit.
        const SUIDEN = 1 << 0;
        /// `SUNIDEN` bit.
        const SUNIDEN = 1 << 1;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SMCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SmcrEl1: u64 {
        /// `EZT0` bit.
        const EZT0 = 1 << 30;
        /// `FA64` bit.
        const FA64 = 1 << 31;
    }
}

#[cfg(feature = "el1")]
impl SmcrEl1 {
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SMCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SmcrEl2: u64 {
        /// `EZT0` bit.
        const EZT0 = 1 << 30;
        /// `FA64` bit.
        const FA64 = 1 << 31;
    }
}

#[cfg(feature = "el2")]
impl SmcrEl2 {
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
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
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SMIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SmidrEl1: u64 {
        /// `SMPS` bit.
        const SMPS = 1 << 15;
    }
}

#[cfg(feature = "el1")]
impl SmidrEl1 {
    /// Returns the value of the `Affinity` field.
    pub const fn affinity(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b111111111111
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        (self.bits() >> 13) as u8 & 0b11
    }

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
    }

    /// Returns the value of the `Affinity2` field.
    pub const fn affinity2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b11111111111111111111
    }

    /// Returns the value of the `HIP` field.
    pub const fn hip(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `NSMC` field.
    pub const fn nsmc(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SMPRIMAP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SmprimapEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl SmprimapEl2 {
    /// Returns the value of the `P0` field.
    pub const fn p0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `P1` field.
    pub const fn p1(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `P2` field.
    pub const fn p2(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `P3` field.
    pub const fn p3(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `P4` field.
    pub const fn p4(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `P5` field.
    pub const fn p5(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `P6` field.
    pub const fn p6(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `P7` field.
    pub const fn p7(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }

    /// Returns the value of the `P8` field.
    pub const fn p8(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b1111
    }

    /// Returns the value of the `P9` field.
    pub const fn p9(self) -> u8 {
        (self.bits() >> 36) as u8 & 0b1111
    }

    /// Returns the value of the `P10` field.
    pub const fn p10(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111
    }

    /// Returns the value of the `P11` field.
    pub const fn p11(self) -> u8 {
        (self.bits() >> 44) as u8 & 0b1111
    }

    /// Returns the value of the `P12` field.
    pub const fn p12(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b1111
    }

    /// Returns the value of the `P13` field.
    pub const fn p13(self) -> u8 {
        (self.bits() >> 52) as u8 & 0b1111
    }

    /// Returns the value of the `P14` field.
    pub const fn p14(self) -> u8 {
        (self.bits() >> 56) as u8 & 0b1111
    }

    /// Returns the value of the `P15` field.
    pub const fn p15(self) -> u8 {
        (self.bits() >> 60) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SMPRI_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SmpriEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl SmpriEl1 {
    /// Returns the value of the `Priority` field.
    pub const fn priority(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMACCESSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmaccessrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl SpmaccessrEl1 {
    /// Returns the value of the given `P<m>` field.
    pub const fn p(self, m: u32) -> u8 {
        assert!(m < 32);
        (self.bits() >> (0 + (m - 0) * 2)) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `SPMACCESSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmaccessrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl SpmaccessrEl2 {
    /// Returns the value of the given `P<m>` field.
    pub const fn p(self, m: u32) -> u8 {
        assert!(m < 32);
        (self.bits() >> (0 + (m - 0) * 2)) as u8 & 0b11
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `SPMACCESSR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmaccessrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl SpmaccessrEl3 {
    /// Returns the value of the given `P<m>` field.
    pub const fn p(self, m: u32) -> u8 {
        assert!(m < 32);
        (self.bits() >> (0 + (m - 0) * 2)) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMCFGR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmcfgrEl1: u64 {
        /// `EX` bit.
        const EX = 1 << 16;
        /// `NA` bit.
        const NA = 1 << 17;
        /// `MSI` bit.
        const MSI = 1 << 20;
        /// `FZO` bit.
        const FZO = 1 << 21;
        /// `SS` bit.
        const SS = 1 << 22;
        /// `TRO` bit.
        const TRO = 1 << 23;
        /// `HDBG` bit.
        const HDBG = 1 << 24;
    }
}

#[cfg(feature = "el1")]
impl SpmcfgrEl1 {
    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `SIZE` field.
    pub const fn size(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b111111
    }

    /// Returns the value of the `NCG` field.
    pub const fn ncg(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `SPMCNTENCLR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmcntenclrEl0: u64 {
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
        /// `P<m>` bit 31.
        const P31 = 1 << 31;
        /// `P<m>` bit 32.
        const P32 = 1 << 32;
        /// `P<m>` bit 33.
        const P33 = 1 << 33;
        /// `P<m>` bit 34.
        const P34 = 1 << 34;
        /// `P<m>` bit 35.
        const P35 = 1 << 35;
        /// `P<m>` bit 36.
        const P36 = 1 << 36;
        /// `P<m>` bit 37.
        const P37 = 1 << 37;
        /// `P<m>` bit 38.
        const P38 = 1 << 38;
        /// `P<m>` bit 39.
        const P39 = 1 << 39;
        /// `P<m>` bit 40.
        const P40 = 1 << 40;
        /// `P<m>` bit 41.
        const P41 = 1 << 41;
        /// `P<m>` bit 42.
        const P42 = 1 << 42;
        /// `P<m>` bit 43.
        const P43 = 1 << 43;
        /// `P<m>` bit 44.
        const P44 = 1 << 44;
        /// `P<m>` bit 45.
        const P45 = 1 << 45;
        /// `P<m>` bit 46.
        const P46 = 1 << 46;
        /// `P<m>` bit 47.
        const P47 = 1 << 47;
        /// `P<m>` bit 48.
        const P48 = 1 << 48;
        /// `P<m>` bit 49.
        const P49 = 1 << 49;
        /// `P<m>` bit 50.
        const P50 = 1 << 50;
        /// `P<m>` bit 51.
        const P51 = 1 << 51;
        /// `P<m>` bit 52.
        const P52 = 1 << 52;
        /// `P<m>` bit 53.
        const P53 = 1 << 53;
        /// `P<m>` bit 54.
        const P54 = 1 << 54;
        /// `P<m>` bit 55.
        const P55 = 1 << 55;
        /// `P<m>` bit 56.
        const P56 = 1 << 56;
        /// `P<m>` bit 57.
        const P57 = 1 << 57;
        /// `P<m>` bit 58.
        const P58 = 1 << 58;
        /// `P<m>` bit 59.
        const P59 = 1 << 59;
        /// `P<m>` bit 60.
        const P60 = 1 << 60;
        /// `P<m>` bit 61.
        const P61 = 1 << 61;
        /// `P<m>` bit 62.
        const P62 = 1 << 62;
        /// `P<m>` bit 63.
        const P63 = 1 << 63;
    }
}

bitflags! {
    /// `SPMCNTENSET_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmcntensetEl0: u64 {
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
        /// `P<m>` bit 31.
        const P31 = 1 << 31;
        /// `P<m>` bit 32.
        const P32 = 1 << 32;
        /// `P<m>` bit 33.
        const P33 = 1 << 33;
        /// `P<m>` bit 34.
        const P34 = 1 << 34;
        /// `P<m>` bit 35.
        const P35 = 1 << 35;
        /// `P<m>` bit 36.
        const P36 = 1 << 36;
        /// `P<m>` bit 37.
        const P37 = 1 << 37;
        /// `P<m>` bit 38.
        const P38 = 1 << 38;
        /// `P<m>` bit 39.
        const P39 = 1 << 39;
        /// `P<m>` bit 40.
        const P40 = 1 << 40;
        /// `P<m>` bit 41.
        const P41 = 1 << 41;
        /// `P<m>` bit 42.
        const P42 = 1 << 42;
        /// `P<m>` bit 43.
        const P43 = 1 << 43;
        /// `P<m>` bit 44.
        const P44 = 1 << 44;
        /// `P<m>` bit 45.
        const P45 = 1 << 45;
        /// `P<m>` bit 46.
        const P46 = 1 << 46;
        /// `P<m>` bit 47.
        const P47 = 1 << 47;
        /// `P<m>` bit 48.
        const P48 = 1 << 48;
        /// `P<m>` bit 49.
        const P49 = 1 << 49;
        /// `P<m>` bit 50.
        const P50 = 1 << 50;
        /// `P<m>` bit 51.
        const P51 = 1 << 51;
        /// `P<m>` bit 52.
        const P52 = 1 << 52;
        /// `P<m>` bit 53.
        const P53 = 1 << 53;
        /// `P<m>` bit 54.
        const P54 = 1 << 54;
        /// `P<m>` bit 55.
        const P55 = 1 << 55;
        /// `P<m>` bit 56.
        const P56 = 1 << 56;
        /// `P<m>` bit 57.
        const P57 = 1 << 57;
        /// `P<m>` bit 58.
        const P58 = 1 << 58;
        /// `P<m>` bit 59.
        const P59 = 1 << 59;
        /// `P<m>` bit 60.
        const P60 = 1 << 60;
        /// `P<m>` bit 61.
        const P61 = 1 << 61;
        /// `P<m>` bit 62.
        const P62 = 1 << 62;
        /// `P<m>` bit 63.
        const P63 = 1 << 63;
    }
}

bitflags! {
    /// `SPMCR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmcrEl0: u64 {
        /// `E` bit.
        const E = 1 << 0;
        /// `P` bit.
        const P = 1 << 1;
        /// `EX` bit.
        const EX = 1 << 4;
        /// `NA` bit.
        const NA = 1 << 8;
        /// `FZO` bit.
        const FZO = 1 << 9;
        /// `HDBG` bit.
        const HDBG = 1 << 10;
        /// `TRO` bit.
        const TRO = 1 << 11;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMDEVAFF_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmdevaffEl1: u64 {
        /// `MT` bit.
        const MT = 1 << 24;
        /// `U` bit.
        const U = 1 << 30;
        /// `F0V` bit.
        const F0V = 1 << 31;
    }
}

#[cfg(feature = "el1")]
impl SpmdevaffEl1 {
    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMDEVARCH_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmdevarchEl1: u64 {
        /// `PRESENT` bit.
        const PRESENT = 1 << 20;
    }
}

#[cfg(feature = "el1")]
impl SpmdevarchEl1 {
    /// Returns the value of the `ARCHPART` field.
    pub const fn archpart(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b111111111111
    }

    /// Returns the value of the `ARCHVER` field.
    pub const fn archver(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `REVISION` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `ARCHITECT` field.
    pub const fn architect(self) -> u16 {
        (self.bits() >> 21) as u16 & 0b11111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMIIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmiidrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl SpmiidrEl1 {
    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b111111111111
    }

    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `ProductID` field.
    pub const fn productid(self) -> u16 {
        (self.bits() >> 20) as u16 & 0b111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMINTENCLR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmintenclrEl1: u64 {
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
        /// `P<m>` bit 31.
        const P31 = 1 << 31;
        /// `P<m>` bit 32.
        const P32 = 1 << 32;
        /// `P<m>` bit 33.
        const P33 = 1 << 33;
        /// `P<m>` bit 34.
        const P34 = 1 << 34;
        /// `P<m>` bit 35.
        const P35 = 1 << 35;
        /// `P<m>` bit 36.
        const P36 = 1 << 36;
        /// `P<m>` bit 37.
        const P37 = 1 << 37;
        /// `P<m>` bit 38.
        const P38 = 1 << 38;
        /// `P<m>` bit 39.
        const P39 = 1 << 39;
        /// `P<m>` bit 40.
        const P40 = 1 << 40;
        /// `P<m>` bit 41.
        const P41 = 1 << 41;
        /// `P<m>` bit 42.
        const P42 = 1 << 42;
        /// `P<m>` bit 43.
        const P43 = 1 << 43;
        /// `P<m>` bit 44.
        const P44 = 1 << 44;
        /// `P<m>` bit 45.
        const P45 = 1 << 45;
        /// `P<m>` bit 46.
        const P46 = 1 << 46;
        /// `P<m>` bit 47.
        const P47 = 1 << 47;
        /// `P<m>` bit 48.
        const P48 = 1 << 48;
        /// `P<m>` bit 49.
        const P49 = 1 << 49;
        /// `P<m>` bit 50.
        const P50 = 1 << 50;
        /// `P<m>` bit 51.
        const P51 = 1 << 51;
        /// `P<m>` bit 52.
        const P52 = 1 << 52;
        /// `P<m>` bit 53.
        const P53 = 1 << 53;
        /// `P<m>` bit 54.
        const P54 = 1 << 54;
        /// `P<m>` bit 55.
        const P55 = 1 << 55;
        /// `P<m>` bit 56.
        const P56 = 1 << 56;
        /// `P<m>` bit 57.
        const P57 = 1 << 57;
        /// `P<m>` bit 58.
        const P58 = 1 << 58;
        /// `P<m>` bit 59.
        const P59 = 1 << 59;
        /// `P<m>` bit 60.
        const P60 = 1 << 60;
        /// `P<m>` bit 61.
        const P61 = 1 << 61;
        /// `P<m>` bit 62.
        const P62 = 1 << 62;
        /// `P<m>` bit 63.
        const P63 = 1 << 63;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMINTENSET_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmintensetEl1: u64 {
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
        /// `P<m>` bit 31.
        const P31 = 1 << 31;
        /// `P<m>` bit 32.
        const P32 = 1 << 32;
        /// `P<m>` bit 33.
        const P33 = 1 << 33;
        /// `P<m>` bit 34.
        const P34 = 1 << 34;
        /// `P<m>` bit 35.
        const P35 = 1 << 35;
        /// `P<m>` bit 36.
        const P36 = 1 << 36;
        /// `P<m>` bit 37.
        const P37 = 1 << 37;
        /// `P<m>` bit 38.
        const P38 = 1 << 38;
        /// `P<m>` bit 39.
        const P39 = 1 << 39;
        /// `P<m>` bit 40.
        const P40 = 1 << 40;
        /// `P<m>` bit 41.
        const P41 = 1 << 41;
        /// `P<m>` bit 42.
        const P42 = 1 << 42;
        /// `P<m>` bit 43.
        const P43 = 1 << 43;
        /// `P<m>` bit 44.
        const P44 = 1 << 44;
        /// `P<m>` bit 45.
        const P45 = 1 << 45;
        /// `P<m>` bit 46.
        const P46 = 1 << 46;
        /// `P<m>` bit 47.
        const P47 = 1 << 47;
        /// `P<m>` bit 48.
        const P48 = 1 << 48;
        /// `P<m>` bit 49.
        const P49 = 1 << 49;
        /// `P<m>` bit 50.
        const P50 = 1 << 50;
        /// `P<m>` bit 51.
        const P51 = 1 << 51;
        /// `P<m>` bit 52.
        const P52 = 1 << 52;
        /// `P<m>` bit 53.
        const P53 = 1 << 53;
        /// `P<m>` bit 54.
        const P54 = 1 << 54;
        /// `P<m>` bit 55.
        const P55 = 1 << 55;
        /// `P<m>` bit 56.
        const P56 = 1 << 56;
        /// `P<m>` bit 57.
        const P57 = 1 << 57;
        /// `P<m>` bit 58.
        const P58 = 1 << 58;
        /// `P<m>` bit 59.
        const P59 = 1 << 59;
        /// `P<m>` bit 60.
        const P60 = 1 << 60;
        /// `P<m>` bit 61.
        const P61 = 1 << 61;
        /// `P<m>` bit 62.
        const P62 = 1 << 62;
        /// `P<m>` bit 63.
        const P63 = 1 << 63;
    }
}

bitflags! {
    /// `SPMOVSCLR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmovsclrEl0: u64 {
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
        /// `P<m>` bit 31.
        const P31 = 1 << 31;
        /// `P<m>` bit 32.
        const P32 = 1 << 32;
        /// `P<m>` bit 33.
        const P33 = 1 << 33;
        /// `P<m>` bit 34.
        const P34 = 1 << 34;
        /// `P<m>` bit 35.
        const P35 = 1 << 35;
        /// `P<m>` bit 36.
        const P36 = 1 << 36;
        /// `P<m>` bit 37.
        const P37 = 1 << 37;
        /// `P<m>` bit 38.
        const P38 = 1 << 38;
        /// `P<m>` bit 39.
        const P39 = 1 << 39;
        /// `P<m>` bit 40.
        const P40 = 1 << 40;
        /// `P<m>` bit 41.
        const P41 = 1 << 41;
        /// `P<m>` bit 42.
        const P42 = 1 << 42;
        /// `P<m>` bit 43.
        const P43 = 1 << 43;
        /// `P<m>` bit 44.
        const P44 = 1 << 44;
        /// `P<m>` bit 45.
        const P45 = 1 << 45;
        /// `P<m>` bit 46.
        const P46 = 1 << 46;
        /// `P<m>` bit 47.
        const P47 = 1 << 47;
        /// `P<m>` bit 48.
        const P48 = 1 << 48;
        /// `P<m>` bit 49.
        const P49 = 1 << 49;
        /// `P<m>` bit 50.
        const P50 = 1 << 50;
        /// `P<m>` bit 51.
        const P51 = 1 << 51;
        /// `P<m>` bit 52.
        const P52 = 1 << 52;
        /// `P<m>` bit 53.
        const P53 = 1 << 53;
        /// `P<m>` bit 54.
        const P54 = 1 << 54;
        /// `P<m>` bit 55.
        const P55 = 1 << 55;
        /// `P<m>` bit 56.
        const P56 = 1 << 56;
        /// `P<m>` bit 57.
        const P57 = 1 << 57;
        /// `P<m>` bit 58.
        const P58 = 1 << 58;
        /// `P<m>` bit 59.
        const P59 = 1 << 59;
        /// `P<m>` bit 60.
        const P60 = 1 << 60;
        /// `P<m>` bit 61.
        const P61 = 1 << 61;
        /// `P<m>` bit 62.
        const P62 = 1 << 62;
        /// `P<m>` bit 63.
        const P63 = 1 << 63;
    }
}

bitflags! {
    /// `SPMOVSSET_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmovssetEl0: u64 {
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
        /// `P<m>` bit 31.
        const P31 = 1 << 31;
        /// `P<m>` bit 32.
        const P32 = 1 << 32;
        /// `P<m>` bit 33.
        const P33 = 1 << 33;
        /// `P<m>` bit 34.
        const P34 = 1 << 34;
        /// `P<m>` bit 35.
        const P35 = 1 << 35;
        /// `P<m>` bit 36.
        const P36 = 1 << 36;
        /// `P<m>` bit 37.
        const P37 = 1 << 37;
        /// `P<m>` bit 38.
        const P38 = 1 << 38;
        /// `P<m>` bit 39.
        const P39 = 1 << 39;
        /// `P<m>` bit 40.
        const P40 = 1 << 40;
        /// `P<m>` bit 41.
        const P41 = 1 << 41;
        /// `P<m>` bit 42.
        const P42 = 1 << 42;
        /// `P<m>` bit 43.
        const P43 = 1 << 43;
        /// `P<m>` bit 44.
        const P44 = 1 << 44;
        /// `P<m>` bit 45.
        const P45 = 1 << 45;
        /// `P<m>` bit 46.
        const P46 = 1 << 46;
        /// `P<m>` bit 47.
        const P47 = 1 << 47;
        /// `P<m>` bit 48.
        const P48 = 1 << 48;
        /// `P<m>` bit 49.
        const P49 = 1 << 49;
        /// `P<m>` bit 50.
        const P50 = 1 << 50;
        /// `P<m>` bit 51.
        const P51 = 1 << 51;
        /// `P<m>` bit 52.
        const P52 = 1 << 52;
        /// `P<m>` bit 53.
        const P53 = 1 << 53;
        /// `P<m>` bit 54.
        const P54 = 1 << 54;
        /// `P<m>` bit 55.
        const P55 = 1 << 55;
        /// `P<m>` bit 56.
        const P56 = 1 << 56;
        /// `P<m>` bit 57.
        const P57 = 1 << 57;
        /// `P<m>` bit 58.
        const P58 = 1 << 58;
        /// `P<m>` bit 59.
        const P59 = 1 << 59;
        /// `P<m>` bit 60.
        const P60 = 1 << 60;
        /// `P<m>` bit 61.
        const P61 = 1 << 61;
        /// `P<m>` bit 62.
        const P62 = 1 << 62;
        /// `P<m>` bit 63.
        const P63 = 1 << 63;
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `SPMROOTCR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmrootcrEl3: u64 {
        /// `RTO` bit.
        const RTO = 1 << 0;
        /// `RLO` bit.
        const RLO = 1 << 1;
        /// `NAO` bit.
        const NAO = 1 << 3;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `SPMSCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmscrEl1: u64 {
        /// `SO` bit.
        const SO = 1 << 0;
        /// `NAO` bit.
        const NAO = 1 << 4;
    }
}

bitflags! {
    /// `SPMSELR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmselrEl0: u64 {
    }
}

impl SpmselrEl0 {
    /// Returns the value of the `BANK` field.
    pub const fn bank(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `SYSPMUSEL` field.
    pub const fn syspmusel(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b111111
    }
}

bitflags! {
    /// `SPMZR_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpmzrEl0: u64 {
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
        /// `P<m>` bit 31.
        const P31 = 1 << 31;
        /// `P<m>` bit 32.
        const P32 = 1 << 32;
        /// `P<m>` bit 33.
        const P33 = 1 << 33;
        /// `P<m>` bit 34.
        const P34 = 1 << 34;
        /// `P<m>` bit 35.
        const P35 = 1 << 35;
        /// `P<m>` bit 36.
        const P36 = 1 << 36;
        /// `P<m>` bit 37.
        const P37 = 1 << 37;
        /// `P<m>` bit 38.
        const P38 = 1 << 38;
        /// `P<m>` bit 39.
        const P39 = 1 << 39;
        /// `P<m>` bit 40.
        const P40 = 1 << 40;
        /// `P<m>` bit 41.
        const P41 = 1 << 41;
        /// `P<m>` bit 42.
        const P42 = 1 << 42;
        /// `P<m>` bit 43.
        const P43 = 1 << 43;
        /// `P<m>` bit 44.
        const P44 = 1 << 44;
        /// `P<m>` bit 45.
        const P45 = 1 << 45;
        /// `P<m>` bit 46.
        const P46 = 1 << 46;
        /// `P<m>` bit 47.
        const P47 = 1 << 47;
        /// `P<m>` bit 48.
        const P48 = 1 << 48;
        /// `P<m>` bit 49.
        const P49 = 1 << 49;
        /// `P<m>` bit 50.
        const P50 = 1 << 50;
        /// `P<m>` bit 51.
        const P51 = 1 << 51;
        /// `P<m>` bit 52.
        const P52 = 1 << 52;
        /// `P<m>` bit 53.
        const P53 = 1 << 53;
        /// `P<m>` bit 54.
        const P54 = 1 << 54;
        /// `P<m>` bit 55.
        const P55 = 1 << 55;
        /// `P<m>` bit 56.
        const P56 = 1 << 56;
        /// `P<m>` bit 57.
        const P57 = 1 << 57;
        /// `P<m>` bit 58.
        const P58 = 1 << 58;
        /// `P<m>` bit 59.
        const P59 = 1 << 59;
        /// `P<m>` bit 60.
        const P60 = 1 << 60;
        /// `P<m>` bit 61.
        const P61 = 1 << 61;
        /// `P<m>` bit 62.
        const P62 = 1 << 62;
        /// `P<m>` bit 63.
        const P63 = 1 << 63;
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
    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
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
    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
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
    /// Returns the value of the `M[3:0]` field.
    pub const fn m_3_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `BTYPE` field.
    pub const fn btype(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }
}

bitflags! {
    /// `SPSR_abt` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrAbt: u64 {
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
        /// `DIT` bit.
        const DIT = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `SSBS` bit.
        const SSBS = 1 << 23;
        /// `J` bit.
        const J = 1 << 24;
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

impl SpsrAbt {
    /// Returns the value of the `M[4:0]` field.
    pub const fn m_4_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }
}

bitflags! {
    /// `SPSR_fiq` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrFiq: u64 {
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
        /// `DIT` bit.
        const DIT = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `SSBS` bit.
        const SSBS = 1 << 23;
        /// `J` bit.
        const J = 1 << 24;
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

impl SpsrFiq {
    /// Returns the value of the `M[4:0]` field.
    pub const fn m_4_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }
}

bitflags! {
    /// `SPSR_irq` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrIrq: u64 {
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
        /// `DIT` bit.
        const DIT = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `SSBS` bit.
        const SSBS = 1 << 23;
        /// `J` bit.
        const J = 1 << 24;
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

impl SpsrIrq {
    /// Returns the value of the `M[4:0]` field.
    pub const fn m_4_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }
}

bitflags! {
    /// `SPSR_und` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpsrUnd: u64 {
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
        /// `DIT` bit.
        const DIT = 1 << 21;
        /// `PAN` bit.
        const PAN = 1 << 22;
        /// `SSBS` bit.
        const SSBS = 1 << 23;
        /// `J` bit.
        const J = 1 << 24;
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

impl SpsrUnd {
    /// Returns the value of the `M[4:0]` field.
    pub const fn m_4_0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `GE` field.
    pub const fn ge(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }
}

bitflags! {
    /// `SPSel` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Spsel: u64 {
        /// `SP` bit.
        const SP = 1 << 0;
    }
}

bitflags! {
    /// `SP_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct SpEl0: u64 {
    }
}

impl SpEl0 {
    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `StackPointer` field.
    pub const fn stackpointer(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `SSBS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ssbs: u64 {
        /// `SSBS` bit.
        const SSBS = 1 << 12;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `STINDEX_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct StindexEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl StindexEl1 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `STINDEX_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct StindexEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl StindexEl2 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `STINDEX_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct StindexEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl StindexEl3 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

bitflags! {
    /// `SVCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Svcr: u64 {
        /// `SM` bit.
        const SM = 1 << 0;
        /// `ZA` bit.
        const ZA = 1 << 1;
    }
}

bitflags! {
    /// `TCO` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tco: u64 {
        /// `TCO` bit.
        const TCO = 1 << 25;
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TCR2MASK_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tcr2maskEl1: u64 {
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
        /// `POIW` bit.
        const POIW = 1 << 22;
        /// `VTB0` bit.
        const VTB0 = 1 << 25;
        /// `VTB1` bit.
        const VTB1 = 1 << 30;
        /// `TVAD0` bit.
        const TVAD0 = 1 << 35;
        /// `TVAD1` bit.
        const TVAD1 = 1 << 36;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TCR2MASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tcr2maskEl2: u64 {
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
        /// `POIW` bit.
        const POIW = 1 << 22;
        /// `VTB0` bit.
        const VTB0 = 1 << 25;
        /// `VTB1` bit.
        const VTB1 = 1 << 30;
        /// `TVAD0` bit.
        const TVAD0 = 1 << 35;
        /// `TVAD1` bit.
        const TVAD1 = 1 << 36;
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
    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        (self.bits() >> 22) as u8 & 0b111
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        (self.bits() >> 25) as u8 & 0b11111
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11111
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
    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        (self.bits() >> 22) as u8 & 0b111
    }

    /// Returns the value of the `VTB0` field.
    pub const fn vtb0(self) -> u8 {
        (self.bits() >> 25) as u8 & 0b11111
    }

    /// Returns the value of the `VTB1` field.
    pub const fn vtb1(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TCRMASK_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrmaskEl1: u64 {
        /// `T0SZ` bit.
        const T0SZ = 1 << 0;
        /// `EPD0` bit.
        const EPD0 = 1 << 7;
        /// `IRGN0` bit.
        const IRGN0 = 1 << 8;
        /// `ORGN0` bit.
        const ORGN0 = 1 << 10;
        /// `SH0` bit.
        const SH0 = 1 << 12;
        /// `TG0` bit.
        const TG0 = 1 << 14;
        /// `T1SZ` bit.
        const T1SZ = 1 << 16;
        /// `A1` bit.
        const A1 = 1 << 22;
        /// `EPD1` bit.
        const EPD1 = 1 << 23;
        /// `IRGN1` bit.
        const IRGN1 = 1 << 24;
        /// `ORGN1` bit.
        const ORGN1 = 1 << 26;
        /// `SH1` bit.
        const SH1 = 1 << 28;
        /// `TG1` bit.
        const TG1 = 1 << 30;
        /// `IPS` bit.
        const IPS = 1 << 32;
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

#[cfg(feature = "el2")]
bitflags! {
    /// `TCRMASK_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TcrmaskEl2: u64 {
        /// `T0SZ` bit.
        const T0SZ = 1 << 0;
        /// `EPD0` bit.
        const EPD0 = 1 << 7;
        /// `IRGN0` bit.
        const IRGN0 = 1 << 8;
        /// `ORGN0` bit.
        const ORGN0 = 1 << 10;
        /// `SH0` bit.
        const SH0 = 1 << 12;
        /// `TG0` bit.
        const TG0 = 1 << 14;
        /// `PS` bit.
        const PS = 1 << 16;
        /// `T1SZ` bit.
        const T1SZ = 1 << 16;
        /// `TBI` bit.
        const TBI = 1 << 20;
        /// `A1` bit.
        const A1 = 1 << 22;
        /// `EPD1` bit.
        const EPD1 = 1 << 23;
        /// `HPD` bit.
        const HPD = 1 << 24;
        /// `IRGN1` bit.
        const IRGN1 = 1 << 24;
        /// `HWU59` bit.
        const HWU59 = 1 << 25;
        /// `HWU60` bit.
        const HWU60 = 1 << 26;
        /// `ORGN1` bit.
        const ORGN1 = 1 << 26;
        /// `HWU61` bit.
        const HWU61 = 1 << 27;
        /// `HWU62` bit.
        const HWU62 = 1 << 28;
        /// `SH1` bit.
        const SH1 = 1 << 28;
        /// `TBID` bit.
        const TBID = 1 << 29;
        /// `TCMA` bit.
        const TCMA = 1 << 30;
        /// `TG1` bit.
        const TG1 = 1 << 30;
        /// `IPS` bit.
        const IPS = 1 << 32;
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
        /// `VTB` bit.
        const VTB = 1 << 48;
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
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b11
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b11
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b111
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
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b11
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b11
    }

    /// Returns the value of the `TG1` field.
    pub const fn tg1(self) -> u8 {
        (self.bits() >> 30) as u8 & 0b11
    }

    /// Returns the value of the `IPS` field.
    pub const fn ips(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b111
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111
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
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111
    }

    /// Returns the value of the `POIW` field.
    pub const fn poiw(self) -> u8 {
        (self.bits() >> 45) as u8 & 0b111
    }

    /// Returns the value of the `VTB` field.
    pub const fn vtb(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111
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

#[cfg(feature = "el3")]
bitflags! {
    /// `TFSR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TfsrEl3: u64 {
        /// `TF0` bit.
        const TF0 = 1 << 0;
    }
}

bitflags! {
    /// `TINDEX_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TindexEl0: u64 {
    }
}

impl TindexEl0 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TINDEX_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TindexEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl TindexEl1 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TINDEX_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TindexEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl TindexEl2 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `TINDEX_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TindexEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl TindexEl3 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

bitflags! {
    /// `TLBIASID` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbiasid: u32 {
    }
}

impl Tlbiasid {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

bitflags! {
    /// `TLBIASIDIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbiasidis: u32 {
    }
}

impl Tlbiasidis {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TLBIDIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TlbididrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl TlbididrEl1 {
    /// Returns the value of the `NIS` field.
    pub const fn nis(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `NVIS` field.
    pub const fn nvis(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111
    }

    /// Returns the value of the `NOS` field.
    pub const fn nos(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111
    }

    /// Returns the value of the `NVOS` field.
    pub const fn nvos(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b11111
    }
}

bitflags! {
    /// `TLBIIPAS2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbiipas2: u32 {
    }
}

impl Tlbiipas2 {
    /// Returns the value of the `IPA[39:12]` field.
    pub const fn ipa_39_12(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111111
    }
}

bitflags! {
    /// `TLBIIPAS2IS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbiipas2is: u32 {
    }
}

impl Tlbiipas2is {
    /// Returns the value of the `IPA[39:12]` field.
    pub const fn ipa_39_12(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111111
    }
}

bitflags! {
    /// `TLBIIPAS2L` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbiipas2l: u32 {
    }
}

impl Tlbiipas2l {
    /// Returns the value of the `IPA[39:12]` field.
    pub const fn ipa_39_12(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111111
    }
}

bitflags! {
    /// `TLBIIPAS2LIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbiipas2lis: u32 {
    }
}

impl Tlbiipas2lis {
    /// Returns the value of the `IPA[39:12]` field.
    pub const fn ipa_39_12(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b1111111111111111111111111111
    }
}

bitflags! {
    /// `TLBIMVA` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimva: u32 {
    }
}

impl Tlbimva {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAA` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvaa: u32 {
    }
}

impl Tlbimvaa {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAAIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvaais: u32 {
    }
}

impl Tlbimvaais {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvaal: u32 {
    }
}

impl Tlbimvaal {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAALIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvaalis: u32 {
    }
}

impl Tlbimvaalis {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAH` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvah: u32 {
    }
}

impl Tlbimvah {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAHIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvahis: u32 {
    }
}

impl Tlbimvahis {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvais: u32 {
    }
}

impl Tlbimvais {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVAL` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimval: u32 {
    }
}

impl Tlbimval {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVALH` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvalh: u32 {
    }
}

impl Tlbimvalh {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVALHIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvalhis: u32 {
    }
}

impl Tlbimvalhis {
    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBIMVALIS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbimvalis: u32 {
    }
}

impl Tlbimvalis {
    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `VA` field.
    pub const fn va(self) -> u32 {
        (self.bits() >> 12) as u32 & 0b11111111111111111111
    }
}

bitflags! {
    /// `TLBTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tlbtr: u32 {
        /// `nU` bit.
        const NU = 1 << 0;
    }
}

bitflags! {
    /// `TPIDR2_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidr2El0: u64 {
    }
}

impl Tpidr2El0 {
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `TPIDRPRW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidrprw: u32 {
    }
}

impl Tpidrprw {
    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
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
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `TPIDRURO` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidruro: u32 {
    }
}

impl Tpidruro {
    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TPIDRURW` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpidrurw: u32 {
    }
}

impl Tpidrurw {
    /// Returns the value of the `TID` field.
    pub const fn tid(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
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
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `TPIDR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TpidrEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl TpidrEl3 {
    /// Returns the value of the `ThreadID` field.
    pub const fn threadid(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `TPMAX0_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmax0El0: u64 {
    }
}

impl Tpmax0El0 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TPMAX0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmax0El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Tpmax0El1 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TPMAX0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmax0El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Tpmax0El2 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `TPMAX1_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmax1El0: u64 {
    }
}

impl Tpmax1El0 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TPMAX1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmax1El1: u64 {
    }
}

#[cfg(feature = "el1")]
impl Tpmax1El1 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TPMAX1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmax1El2: u64 {
    }
}

#[cfg(feature = "el2")]
impl Tpmax1El2 {
    /// Returns the value of the `MAX` field.
    pub const fn max(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `TPMIN0_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmin0El0: u64 {
        /// `TPLIM` bit.
        const TPLIM = 1 << 0;
        /// `TPRW` bit.
        const TPRW = 1 << 1;
    }
}

impl Tpmin0El0 {
    /// Returns the value of the `MIN` field.
    pub const fn min(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TPMIN0_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmin0El1: u64 {
        /// `TPLIM` bit.
        const TPLIM = 1 << 0;
        /// `TPRW` bit.
        const TPRW = 1 << 1;
    }
}

#[cfg(feature = "el1")]
impl Tpmin0El1 {
    /// Returns the value of the `MIN` field.
    pub const fn min(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TPMIN0_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmin0El2: u64 {
        /// `TPLIM` bit.
        const TPLIM = 1 << 0;
        /// `TPRW` bit.
        const TPRW = 1 << 1;
    }
}

#[cfg(feature = "el2")]
impl Tpmin0El2 {
    /// Returns the value of the `MIN` field.
    pub const fn min(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `TPMIN1_EL0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmin1El0: u64 {
        /// `TPLIM` bit.
        const TPLIM = 1 << 0;
    }
}

impl Tpmin1El0 {
    /// Returns the value of the `MIN` field.
    pub const fn min(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TPMIN1_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmin1El1: u64 {
        /// `TPLIM` bit.
        const TPLIM = 1 << 0;
    }
}

#[cfg(feature = "el1")]
impl Tpmin1El1 {
    /// Returns the value of the `MIN` field.
    pub const fn min(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TPMIN1_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Tpmin1El2: u64 {
        /// `TPLIM` bit.
        const TPLIM = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl Tpmin1El2 {
    /// Returns the value of the `MIN` field.
    pub const fn min(self) -> u64 {
        (self.bits() >> 4) as u64 & 0b111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBBASER_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbbaserEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl TrbbaserEl1 {
    /// Returns the value of the `BASE` field.
    pub const fn base(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b1111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBIDR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbidrEl1: u64 {
        /// `P` bit.
        const P = 1 << 4;
        /// `F` bit.
        const F = 1 << 5;
    }
}

#[cfg(feature = "el1")]
impl TrbidrEl1 {
    /// Returns the value of the `Align` field.
    pub const fn align(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `AddrMode` field.
    pub const fn addrmode(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `EA` field.
    pub const fn ea(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `MPAM` field.
    pub const fn mpam(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `MPAM2` field.
    pub const fn mpam2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `MaxBuffSize` field.
    pub const fn maxbuffsize(self) -> u16 {
        (self.bits() >> 32) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBLIMITR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrblimitrEl1: u64 {
        /// `E` bit.
        const E = 1 << 0;
        /// `nVM` bit.
        const NVM = 1 << 5;
        /// `XE` bit.
        const XE = 1 << 6;
    }
}

#[cfg(feature = "el1")]
impl TrblimitrEl1 {
    /// Returns the value of the `FM` field.
    pub const fn fm(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `TM` field.
    pub const fn tm(self) -> u8 {
        (self.bits() >> 3) as u8 & 0b11
    }

    /// Returns the value of the `LIMIT` field.
    pub const fn limit(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b1111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBMAR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbmarEl1: u64 {
        /// `TTBA` bit.
        const TTBA = 1 << 32;
    }
}

#[cfg(feature = "el1")]
impl TrbmarEl1 {
    /// Returns the value of the `Attr` field.
    pub const fn attr(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `SH` field.
    pub const fn sh(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `PAS` field.
    pub const fn pas(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 33) as u8 & 0b1111111
    }

    /// Returns the value of the `FPOIndex` field.
    pub const fn fpoindex(self) -> u8 {
        (self.bits() >> 40) as u8 & 0b1111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBMPAM_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbmpamEl1: u64 {
        /// `EN` bit.
        const EN = 1 << 26;
    }
}

#[cfg(feature = "el1")]
impl TrbmpamEl1 {
    /// Returns the value of the `PARTID` field.
    pub const fn partid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `MPAM_SP` field.
    pub const fn mpam_sp(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBPTR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbptrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl TrbptrEl1 {
    /// Returns the value of the `PTR` field.
    pub const fn ptr(self) -> u64 {
        (self.bits() >> 0) as u64 & 0b1111111111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBSR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbsrEl1: u64 {
        /// `S` bit.
        const S = 1 << 17;
        /// `EA` bit.
        const EA = 1 << 18;
        /// `WRAP` bit.
        const WRAP = 1 << 20;
        /// `TRG` bit.
        const TRG = 1 << 21;
        /// `IRQ` bit.
        const IRQ = 1 << 22;
    }
}

#[cfg(feature = "el1")]
impl TrbsrEl1 {
    /// Returns the value of the `MSS` field.
    pub const fn mss(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `MSS2` field.
    pub const fn mss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TRBSR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbsrEl2: u64 {
        /// `S` bit.
        const S = 1 << 17;
        /// `EA` bit.
        const EA = 1 << 18;
        /// `WRAP` bit.
        const WRAP = 1 << 20;
        /// `TRG` bit.
        const TRG = 1 << 21;
        /// `IRQ` bit.
        const IRQ = 1 << 22;
    }
}

#[cfg(feature = "el2")]
impl TrbsrEl2 {
    /// Returns the value of the `MSS` field.
    pub const fn mss(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `MSS2` field.
    pub const fn mss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `TRBSR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbsrEl3: u64 {
        /// `S` bit.
        const S = 1 << 17;
        /// `EA` bit.
        const EA = 1 << 18;
        /// `WRAP` bit.
        const WRAP = 1 << 20;
        /// `TRG` bit.
        const TRG = 1 << 21;
        /// `IRQ` bit.
        const IRQ = 1 << 22;
    }
}

#[cfg(feature = "el3")]
impl TrbsrEl3 {
    /// Returns the value of the `MSS` field.
    pub const fn mss(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }

    /// Returns the value of the `EC` field.
    pub const fn ec(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b111111
    }

    /// Returns the value of the `MSS2` field.
    pub const fn mss2(self) -> u32 {
        (self.bits() >> 32) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRBTRG_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrbtrgEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl TrbtrgEl1 {
    /// Returns the value of the `TRG` field.
    pub const fn trg(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TRCAUTHSTATUS` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcauthstatus: u64 {
    }
}

impl Trcauthstatus {
    /// Returns the value of the `NSID` field.
    pub const fn nsid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }

    /// Returns the value of the `NSNID` field.
    pub const fn nsnid(self) -> u8 {
        (self.bits() >> 2) as u8 & 0b11
    }

    /// Returns the value of the `SID` field.
    pub const fn sid(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b11
    }

    /// Returns the value of the `SNID` field.
    pub const fn snid(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `HID` field.
    pub const fn hid(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `HNID` field.
    pub const fn hnid(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `RLID` field.
    pub const fn rlid(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `RLNID` field.
    pub const fn rlnid(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `RTID` field.
    pub const fn rtid(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }

    /// Returns the value of the `RTNID` field.
    pub const fn rtnid(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b11
    }
}

bitflags! {
    /// `TRCBBCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcbbctlr: u64 {
        /// `RANGE[<m>]` bit 0.
        const RANGE_0 = 1 << 0;
        /// `RANGE[<m>]` bit 1.
        const RANGE_1 = 1 << 1;
        /// `RANGE[<m>]` bit 2.
        const RANGE_2 = 1 << 2;
        /// `RANGE[<m>]` bit 3.
        const RANGE_3 = 1 << 3;
        /// `RANGE[<m>]` bit 4.
        const RANGE_4 = 1 << 4;
        /// `RANGE[<m>]` bit 5.
        const RANGE_5 = 1 << 5;
        /// `RANGE[<m>]` bit 6.
        const RANGE_6 = 1 << 6;
        /// `RANGE[<m>]` bit 7.
        const RANGE_7 = 1 << 7;
        /// `MODE` bit.
        const MODE = 1 << 8;
    }
}

bitflags! {
    /// `TRCCCCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcccctlr: u64 {
    }
}

impl Trcccctlr {
    /// Returns the value of the `THRESHOLD` field.
    pub const fn threshold(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b111111111111
    }
}

bitflags! {
    /// `TRCCIDCCTLR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trccidcctlr0: u64 {
        /// `COMP0[<m>]` bit 0.
        const COMP0_0 = 1 << 0;
        /// `COMP0[<m>]` bit 1.
        const COMP0_1 = 1 << 1;
        /// `COMP0[<m>]` bit 2.
        const COMP0_2 = 1 << 2;
        /// `COMP0[<m>]` bit 3.
        const COMP0_3 = 1 << 3;
        /// `COMP0[<m>]` bit 4.
        const COMP0_4 = 1 << 4;
        /// `COMP0[<m>]` bit 5.
        const COMP0_5 = 1 << 5;
        /// `COMP0[<m>]` bit 6.
        const COMP0_6 = 1 << 6;
        /// `COMP0[<m>]` bit 7.
        const COMP0_7 = 1 << 7;
        /// `COMP1[<m>]` bit 0.
        const COMP1_0 = 1 << 8;
        /// `COMP1[<m>]` bit 1.
        const COMP1_1 = 1 << 9;
        /// `COMP1[<m>]` bit 2.
        const COMP1_2 = 1 << 10;
        /// `COMP1[<m>]` bit 3.
        const COMP1_3 = 1 << 11;
        /// `COMP1[<m>]` bit 4.
        const COMP1_4 = 1 << 12;
        /// `COMP1[<m>]` bit 5.
        const COMP1_5 = 1 << 13;
        /// `COMP1[<m>]` bit 6.
        const COMP1_6 = 1 << 14;
        /// `COMP1[<m>]` bit 7.
        const COMP1_7 = 1 << 15;
        /// `COMP2[<m>]` bit 0.
        const COMP2_0 = 1 << 16;
        /// `COMP2[<m>]` bit 1.
        const COMP2_1 = 1 << 17;
        /// `COMP2[<m>]` bit 2.
        const COMP2_2 = 1 << 18;
        /// `COMP2[<m>]` bit 3.
        const COMP2_3 = 1 << 19;
        /// `COMP2[<m>]` bit 4.
        const COMP2_4 = 1 << 20;
        /// `COMP2[<m>]` bit 5.
        const COMP2_5 = 1 << 21;
        /// `COMP2[<m>]` bit 6.
        const COMP2_6 = 1 << 22;
        /// `COMP2[<m>]` bit 7.
        const COMP2_7 = 1 << 23;
        /// `COMP3[<m>]` bit 0.
        const COMP3_0 = 1 << 24;
        /// `COMP3[<m>]` bit 1.
        const COMP3_1 = 1 << 25;
        /// `COMP3[<m>]` bit 2.
        const COMP3_2 = 1 << 26;
        /// `COMP3[<m>]` bit 3.
        const COMP3_3 = 1 << 27;
        /// `COMP3[<m>]` bit 4.
        const COMP3_4 = 1 << 28;
        /// `COMP3[<m>]` bit 5.
        const COMP3_5 = 1 << 29;
        /// `COMP3[<m>]` bit 6.
        const COMP3_6 = 1 << 30;
        /// `COMP3[<m>]` bit 7.
        const COMP3_7 = 1 << 31;
    }
}

bitflags! {
    /// `TRCCIDCCTLR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trccidcctlr1: u64 {
        /// `COMP4[<m>]` bit 0.
        const COMP4_0 = 1 << 0;
        /// `COMP4[<m>]` bit 1.
        const COMP4_1 = 1 << 1;
        /// `COMP4[<m>]` bit 2.
        const COMP4_2 = 1 << 2;
        /// `COMP4[<m>]` bit 3.
        const COMP4_3 = 1 << 3;
        /// `COMP4[<m>]` bit 4.
        const COMP4_4 = 1 << 4;
        /// `COMP4[<m>]` bit 5.
        const COMP4_5 = 1 << 5;
        /// `COMP4[<m>]` bit 6.
        const COMP4_6 = 1 << 6;
        /// `COMP4[<m>]` bit 7.
        const COMP4_7 = 1 << 7;
        /// `COMP5[<m>]` bit 0.
        const COMP5_0 = 1 << 8;
        /// `COMP5[<m>]` bit 1.
        const COMP5_1 = 1 << 9;
        /// `COMP5[<m>]` bit 2.
        const COMP5_2 = 1 << 10;
        /// `COMP5[<m>]` bit 3.
        const COMP5_3 = 1 << 11;
        /// `COMP5[<m>]` bit 4.
        const COMP5_4 = 1 << 12;
        /// `COMP5[<m>]` bit 5.
        const COMP5_5 = 1 << 13;
        /// `COMP5[<m>]` bit 6.
        const COMP5_6 = 1 << 14;
        /// `COMP5[<m>]` bit 7.
        const COMP5_7 = 1 << 15;
        /// `COMP6[<m>]` bit 0.
        const COMP6_0 = 1 << 16;
        /// `COMP6[<m>]` bit 1.
        const COMP6_1 = 1 << 17;
        /// `COMP6[<m>]` bit 2.
        const COMP6_2 = 1 << 18;
        /// `COMP6[<m>]` bit 3.
        const COMP6_3 = 1 << 19;
        /// `COMP6[<m>]` bit 4.
        const COMP6_4 = 1 << 20;
        /// `COMP6[<m>]` bit 5.
        const COMP6_5 = 1 << 21;
        /// `COMP6[<m>]` bit 6.
        const COMP6_6 = 1 << 22;
        /// `COMP6[<m>]` bit 7.
        const COMP6_7 = 1 << 23;
        /// `COMP7[<m>]` bit 0.
        const COMP7_0 = 1 << 24;
        /// `COMP7[<m>]` bit 1.
        const COMP7_1 = 1 << 25;
        /// `COMP7[<m>]` bit 2.
        const COMP7_2 = 1 << 26;
        /// `COMP7[<m>]` bit 3.
        const COMP7_3 = 1 << 27;
        /// `COMP7[<m>]` bit 4.
        const COMP7_4 = 1 << 28;
        /// `COMP7[<m>]` bit 5.
        const COMP7_5 = 1 << 29;
        /// `COMP7[<m>]` bit 6.
        const COMP7_6 = 1 << 30;
        /// `COMP7[<m>]` bit 7.
        const COMP7_7 = 1 << 31;
    }
}

bitflags! {
    /// `TRCCLAIMCLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcclaimclr: u64 {
        /// `CLR[<m>]` bit 0.
        const CLR_0 = 1 << 0;
        /// `CLR[<m>]` bit 1.
        const CLR_1 = 1 << 1;
        /// `CLR[<m>]` bit 2.
        const CLR_2 = 1 << 2;
        /// `CLR[<m>]` bit 3.
        const CLR_3 = 1 << 3;
        /// `CLR[<m>]` bit 4.
        const CLR_4 = 1 << 4;
        /// `CLR[<m>]` bit 5.
        const CLR_5 = 1 << 5;
        /// `CLR[<m>]` bit 6.
        const CLR_6 = 1 << 6;
        /// `CLR[<m>]` bit 7.
        const CLR_7 = 1 << 7;
        /// `CLR[<m>]` bit 8.
        const CLR_8 = 1 << 8;
        /// `CLR[<m>]` bit 9.
        const CLR_9 = 1 << 9;
        /// `CLR[<m>]` bit 10.
        const CLR_10 = 1 << 10;
        /// `CLR[<m>]` bit 11.
        const CLR_11 = 1 << 11;
        /// `CLR[<m>]` bit 12.
        const CLR_12 = 1 << 12;
        /// `CLR[<m>]` bit 13.
        const CLR_13 = 1 << 13;
        /// `CLR[<m>]` bit 14.
        const CLR_14 = 1 << 14;
        /// `CLR[<m>]` bit 15.
        const CLR_15 = 1 << 15;
        /// `CLR[<m>]` bit 16.
        const CLR_16 = 1 << 16;
        /// `CLR[<m>]` bit 17.
        const CLR_17 = 1 << 17;
        /// `CLR[<m>]` bit 18.
        const CLR_18 = 1 << 18;
        /// `CLR[<m>]` bit 19.
        const CLR_19 = 1 << 19;
        /// `CLR[<m>]` bit 20.
        const CLR_20 = 1 << 20;
        /// `CLR[<m>]` bit 21.
        const CLR_21 = 1 << 21;
        /// `CLR[<m>]` bit 22.
        const CLR_22 = 1 << 22;
        /// `CLR[<m>]` bit 23.
        const CLR_23 = 1 << 23;
        /// `CLR[<m>]` bit 24.
        const CLR_24 = 1 << 24;
        /// `CLR[<m>]` bit 25.
        const CLR_25 = 1 << 25;
        /// `CLR[<m>]` bit 26.
        const CLR_26 = 1 << 26;
        /// `CLR[<m>]` bit 27.
        const CLR_27 = 1 << 27;
        /// `CLR[<m>]` bit 28.
        const CLR_28 = 1 << 28;
        /// `CLR[<m>]` bit 29.
        const CLR_29 = 1 << 29;
        /// `CLR[<m>]` bit 30.
        const CLR_30 = 1 << 30;
        /// `CLR[<m>]` bit 31.
        const CLR_31 = 1 << 31;
    }
}

bitflags! {
    /// `TRCCLAIMSET` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcclaimset: u64 {
        /// `SET[<m>]` bit 0.
        const SET_0 = 1 << 0;
        /// `SET[<m>]` bit 1.
        const SET_1 = 1 << 1;
        /// `SET[<m>]` bit 2.
        const SET_2 = 1 << 2;
        /// `SET[<m>]` bit 3.
        const SET_3 = 1 << 3;
        /// `SET[<m>]` bit 4.
        const SET_4 = 1 << 4;
        /// `SET[<m>]` bit 5.
        const SET_5 = 1 << 5;
        /// `SET[<m>]` bit 6.
        const SET_6 = 1 << 6;
        /// `SET[<m>]` bit 7.
        const SET_7 = 1 << 7;
        /// `SET[<m>]` bit 8.
        const SET_8 = 1 << 8;
        /// `SET[<m>]` bit 9.
        const SET_9 = 1 << 9;
        /// `SET[<m>]` bit 10.
        const SET_10 = 1 << 10;
        /// `SET[<m>]` bit 11.
        const SET_11 = 1 << 11;
        /// `SET[<m>]` bit 12.
        const SET_12 = 1 << 12;
        /// `SET[<m>]` bit 13.
        const SET_13 = 1 << 13;
        /// `SET[<m>]` bit 14.
        const SET_14 = 1 << 14;
        /// `SET[<m>]` bit 15.
        const SET_15 = 1 << 15;
        /// `SET[<m>]` bit 16.
        const SET_16 = 1 << 16;
        /// `SET[<m>]` bit 17.
        const SET_17 = 1 << 17;
        /// `SET[<m>]` bit 18.
        const SET_18 = 1 << 18;
        /// `SET[<m>]` bit 19.
        const SET_19 = 1 << 19;
        /// `SET[<m>]` bit 20.
        const SET_20 = 1 << 20;
        /// `SET[<m>]` bit 21.
        const SET_21 = 1 << 21;
        /// `SET[<m>]` bit 22.
        const SET_22 = 1 << 22;
        /// `SET[<m>]` bit 23.
        const SET_23 = 1 << 23;
        /// `SET[<m>]` bit 24.
        const SET_24 = 1 << 24;
        /// `SET[<m>]` bit 25.
        const SET_25 = 1 << 25;
        /// `SET[<m>]` bit 26.
        const SET_26 = 1 << 26;
        /// `SET[<m>]` bit 27.
        const SET_27 = 1 << 27;
        /// `SET[<m>]` bit 28.
        const SET_28 = 1 << 28;
        /// `SET[<m>]` bit 29.
        const SET_29 = 1 << 29;
        /// `SET[<m>]` bit 30.
        const SET_30 = 1 << 30;
        /// `SET[<m>]` bit 31.
        const SET_31 = 1 << 31;
    }
}

bitflags! {
    /// `TRCCONFIGR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcconfigr: u64 {
        /// RES1 bits in the `TRCCONFIGR` register.
        const RES1 = 0b1;
        /// `BB` bit.
        const BB = 1 << 3;
        /// `CCI` bit.
        const CCI = 1 << 4;
        /// `CID` bit.
        const CID = 1 << 6;
        /// `VMID` bit.
        const VMID = 1 << 7;
        /// `TS` bit.
        const TS = 1 << 11;
        /// `RS` bit.
        const RS = 1 << 12;
        /// `ITO` bit.
        const ITO = 1 << 18;
    }
}

impl Trcconfigr {
    /// Returns the value of the `QE` field.
    pub const fn qe(self) -> u8 {
        (self.bits() >> 13) as u8 & 0b11
    }
}

bitflags! {
    /// `TRCDEVARCH` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcdevarch: u64 {
        /// `PRESENT` bit.
        const PRESENT = 1 << 20;
    }
}

impl Trcdevarch {
    /// Returns the value of the `ARCHPART` field.
    pub const fn archpart(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b111111111111
    }

    /// Returns the value of the `ARCHVER` field.
    pub const fn archver(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `REVISION` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `ARCHITECT` field.
    pub const fn architect(self) -> u16 {
        (self.bits() >> 21) as u16 & 0b11111111111
    }
}

bitflags! {
    /// `TRCEVENTCTL0R` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trceventctl0r: u64 {
        /// `EVENT0_TYPE` bit.
        const EVENT0_TYPE = 1 << 7;
        /// `EVENT1_TYPE` bit.
        const EVENT1_TYPE = 1 << 15;
        /// `EVENT2_TYPE` bit.
        const EVENT2_TYPE = 1 << 23;
        /// `EVENT3_TYPE` bit.
        const EVENT3_TYPE = 1 << 31;
    }
}

impl Trceventctl0r {
    /// Returns the value of the `EVENT0_SEL` field.
    pub const fn event0_sel(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `EVENT1_SEL` field.
    pub const fn event1_sel(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111
    }

    /// Returns the value of the `EVENT2_SEL` field.
    pub const fn event2_sel(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111
    }

    /// Returns the value of the `EVENT3_SEL` field.
    pub const fn event3_sel(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111
    }
}

bitflags! {
    /// `TRCEVENTCTL1R` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trceventctl1r: u64 {
        /// `INSTEN[<m>]` bit 0.
        const INSTEN_0 = 1 << 0;
        /// `INSTEN[<m>]` bit 1.
        const INSTEN_1 = 1 << 1;
        /// `INSTEN[<m>]` bit 2.
        const INSTEN_2 = 1 << 2;
        /// `INSTEN[<m>]` bit 3.
        const INSTEN_3 = 1 << 3;
        /// `ATB` bit.
        const ATB = 1 << 11;
        /// `LPOVERRIDE` bit.
        const LPOVERRIDE = 1 << 12;
        /// `OE` bit.
        const OE = 1 << 13;
    }
}

bitflags! {
    /// `TRCIDR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr0: u64 {
        /// RES1 bits in the `TRCIDR0` register.
        const RES1 = 0b1;
        /// `TRCBB` bit.
        const TRCBB = 1 << 5;
        /// `TRCCOND` bit.
        const TRCCOND = 1 << 6;
        /// `TRCCCI` bit.
        const TRCCCI = 1 << 7;
        /// `RETSTACK` bit.
        const RETSTACK = 1 << 9;
        /// `QFILT` bit.
        const QFILT = 1 << 14;
        /// `TRCEXDATA` bit.
        const TRCEXDATA = 1 << 17;
        /// `ITE` bit.
        const ITE = 1 << 22;
        /// `TSMARK` bit.
        const TSMARK = 1 << 23;
        /// `COMMOPT` bit.
        const COMMOPT = 1 << 29;
        /// `COMMTRANS` bit.
        const COMMTRANS = 1 << 30;
    }
}

impl Trcidr0 {
    /// Returns the value of the `INSTP0` field.
    pub const fn instp0(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `TRCDATA` field.
    pub const fn trcdata(self) -> u8 {
        (self.bits() >> 3) as u8 & 0b11
    }

    /// Returns the value of the `NUMEVENT` field.
    pub const fn numevent(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `CONDTYPE` field.
    pub const fn condtype(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `QSUPP` field.
    pub const fn qsupp(self) -> u8 {
        (self.bits() >> 15) as u8 & 0b11
    }

    /// Returns the value of the `TSSIZE` field.
    pub const fn tssize(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111
    }
}

bitflags! {
    /// `TRCIDR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr1: u64 {
        /// RES1 bits in the `TRCIDR1` register.
        const RES1 = 0b1111000000000000;
    }
}

impl Trcidr1 {
    /// Returns the value of the `REVISION` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `TRCARCHMIN` field.
    pub const fn trcarchmin(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `TRCARCHMAJ` field.
    pub const fn trcarchmaj(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111
    }

    /// Returns the value of the `DESIGNER` field.
    pub const fn designer(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
    }
}

bitflags! {
    /// `TRCIDR10` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr10: u64 {
    }
}

impl Trcidr10 {
    /// Returns the value of the `NUMP1KEY` field.
    pub const fn nump1key(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TRCIDR11` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr11: u64 {
    }
}

impl Trcidr11 {
    /// Returns the value of the `NUMP1SPC` field.
    pub const fn nump1spc(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TRCIDR12` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr12: u64 {
    }
}

impl Trcidr12 {
    /// Returns the value of the `NUMCONDKEY` field.
    pub const fn numcondkey(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TRCIDR13` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr13: u64 {
    }
}

impl Trcidr13 {
    /// Returns the value of the `NUMCONDSPC` field.
    pub const fn numcondspc(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TRCIDR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr2: u64 {
        /// `WFXMODE` bit.
        const WFXMODE = 1 << 31;
    }
}

impl Trcidr2 {
    /// Returns the value of the `IASIZE` field.
    pub const fn iasize(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }

    /// Returns the value of the `CIDSIZE` field.
    pub const fn cidsize(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b11111
    }

    /// Returns the value of the `VMIDSIZE` field.
    pub const fn vmidsize(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11111
    }

    /// Returns the value of the `DASIZE` field.
    pub const fn dasize(self) -> u8 {
        (self.bits() >> 15) as u8 & 0b11111
    }

    /// Returns the value of the `DVSIZE` field.
    pub const fn dvsize(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b11111
    }

    /// Returns the value of the `CCSIZE` field.
    pub const fn ccsize(self) -> u8 {
        (self.bits() >> 25) as u8 & 0b1111
    }

    /// Returns the value of the `VMIDOPT` field.
    pub const fn vmidopt(self) -> u8 {
        (self.bits() >> 29) as u8 & 0b11
    }
}

bitflags! {
    /// `TRCIDR3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr3: u64 {
        /// `EXLEVEL_S_EL0` bit.
        const EXLEVEL_S_EL0 = 1 << 16;
        /// `EXLEVEL_S_EL1` bit.
        const EXLEVEL_S_EL1 = 1 << 17;
        /// `EXLEVEL_S_EL2` bit.
        const EXLEVEL_S_EL2 = 1 << 18;
        /// `EXLEVEL_S_EL3` bit.
        const EXLEVEL_S_EL3 = 1 << 19;
        /// `EXLEVEL_NS_EL0` bit.
        const EXLEVEL_NS_EL0 = 1 << 20;
        /// `EXLEVEL_NS_EL1` bit.
        const EXLEVEL_NS_EL1 = 1 << 21;
        /// `EXLEVEL_NS_EL2` bit.
        const EXLEVEL_NS_EL2 = 1 << 22;
        /// `TRCERR` bit.
        const TRCERR = 1 << 24;
        /// `SYNCPR` bit.
        const SYNCPR = 1 << 25;
        /// `STALLCTL` bit.
        const STALLCTL = 1 << 26;
        /// `SYSSTALL` bit.
        const SYSSTALL = 1 << 27;
        /// `NOOVERFLOW` bit.
        const NOOVERFLOW = 1 << 31;
    }
}

impl Trcidr3 {
    /// Returns the value of the `CCITMIN` field.
    pub const fn ccitmin(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b111111111111
    }
}

bitflags! {
    /// `TRCIDR4` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr4: u64 {
        /// `SUPPDAC` bit.
        const SUPPDAC = 1 << 8;
    }
}

impl Trcidr4 {
    /// Returns the value of the `NUMACPAIRS` field.
    pub const fn numacpairs(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `NUMDVC` field.
    pub const fn numdvc(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }

    /// Returns the value of the `NUMPC` field.
    pub const fn numpc(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b1111
    }

    /// Returns the value of the `NUMRSPAIR` field.
    pub const fn numrspair(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `NUMSSCC` field.
    pub const fn numsscc(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `NUMCIDC` field.
    pub const fn numcidc(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b1111
    }

    /// Returns the value of the `NUMVMIDC` field.
    pub const fn numvmidc(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b1111
    }
}

bitflags! {
    /// `TRCIDR5` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr5: u64 {
        /// `ATBTRIG` bit.
        const ATBTRIG = 1 << 22;
        /// `LPOVERRIDE` bit.
        const LPOVERRIDE = 1 << 23;
        /// `OE` bit.
        const OE = 1 << 31;
    }
}

impl Trcidr5 {
    /// Returns the value of the `NUMEXTIN` field.
    pub const fn numextin(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b111111111
    }

    /// Returns the value of the `NUMEXTINSEL` field.
    pub const fn numextinsel(self) -> u8 {
        (self.bits() >> 9) as u8 & 0b111
    }

    /// Returns the value of the `TRACEIDSIZE` field.
    pub const fn traceidsize(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111111
    }

    /// Returns the value of the `NUMSEQSTATE` field.
    pub const fn numseqstate(self) -> u8 {
        (self.bits() >> 25) as u8 & 0b111
    }

    /// Returns the value of the `NUMCNTR` field.
    pub const fn numcntr(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b111
    }
}

bitflags! {
    /// `TRCIDR6` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr6: u64 {
        /// `EXLEVEL_RL_EL0` bit.
        const EXLEVEL_RL_EL0 = 1 << 0;
        /// `EXLEVEL_RL_EL1` bit.
        const EXLEVEL_RL_EL1 = 1 << 1;
        /// `EXLEVEL_RL_EL2` bit.
        const EXLEVEL_RL_EL2 = 1 << 2;
    }
}

bitflags! {
    /// `TRCIDR8` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr8: u64 {
    }
}

impl Trcidr8 {
    /// Returns the value of the `MAXSPEC` field.
    pub const fn maxspec(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TRCIDR9` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcidr9: u64 {
    }
}

impl Trcidr9 {
    /// Returns the value of the `NUMP0KEY` field.
    pub const fn nump0key(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b11111111111111111111111111111111
    }
}

bitflags! {
    /// `TRCIMSPEC0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcimspec0: u64 {
    }
}

impl Trcimspec0 {
    /// Returns the value of the `SUPPORT` field.
    pub const fn support(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `EN` field.
    pub const fn en(self) -> u8 {
        (self.bits() >> 4) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRCITECR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrcitecrEl1: u64 {
        /// `E0E` bit.
        const E0E = 1 << 0;
        /// `E1E` bit.
        const E1E = 1 << 1;
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TRCITECR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrcitecrEl2: u64 {
        /// `E0HE` bit.
        const E0HE = 1 << 0;
        /// `E2E` bit.
        const E2E = 1 << 1;
    }
}

bitflags! {
    /// `TRCITEEDCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trciteedcr: u64 {
        /// `E<m>` bit 0.
        const E0 = 1 << 0;
        /// `E<m>` bit 1.
        const E1 = 1 << 1;
        /// `E<m>` bit 2.
        const E2 = 1 << 2;
        /// `E3` bit.
        const E3 = 1 << 3;
        /// `NS` bit.
        const NS = 1 << 4;
        /// `S` bit.
        const S = 1 << 5;
        /// `RL` bit.
        const RL = 1 << 6;
    }
}

bitflags! {
    /// `TRCOSLSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcoslsr: u64 {
        /// `OSLK` bit.
        const OSLK = 1 << 1;
    }
}

bitflags! {
    /// `TRCPRGCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcprgctlr: u64 {
        /// `EN` bit.
        const EN = 1 << 0;
    }
}

bitflags! {
    /// `TRCQCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcqctlr: u64 {
        /// `RANGE[<m>]` bit 0.
        const RANGE_0 = 1 << 0;
        /// `RANGE[<m>]` bit 1.
        const RANGE_1 = 1 << 1;
        /// `RANGE[<m>]` bit 2.
        const RANGE_2 = 1 << 2;
        /// `RANGE[<m>]` bit 3.
        const RANGE_3 = 1 << 3;
        /// `RANGE[<m>]` bit 4.
        const RANGE_4 = 1 << 4;
        /// `RANGE[<m>]` bit 5.
        const RANGE_5 = 1 << 5;
        /// `RANGE[<m>]` bit 6.
        const RANGE_6 = 1 << 6;
        /// `RANGE[<m>]` bit 7.
        const RANGE_7 = 1 << 7;
        /// `MODE` bit.
        const MODE = 1 << 8;
    }
}

bitflags! {
    /// `TRCRSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcrsr: u64 {
        /// `EXTIN[<m>]` bit 0.
        const EXTIN_0 = 1 << 0;
        /// `EXTIN[<m>]` bit 1.
        const EXTIN_1 = 1 << 1;
        /// `EXTIN[<m>]` bit 2.
        const EXTIN_2 = 1 << 2;
        /// `EXTIN[<m>]` bit 3.
        const EXTIN_3 = 1 << 3;
        /// `EVENT[<m>]` bit 0.
        const EVENT_0 = 1 << 8;
        /// `EVENT[<m>]` bit 1.
        const EVENT_1 = 1 << 9;
        /// `EVENT[<m>]` bit 2.
        const EVENT_2 = 1 << 10;
        /// `EVENT[<m>]` bit 3.
        const EVENT_3 = 1 << 11;
        /// `TA` bit.
        const TA = 1 << 12;
    }
}

bitflags! {
    /// `TRCSEQRSTEVR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcseqrstevr: u64 {
        /// `RST_TYPE` bit.
        const RST_TYPE = 1 << 7;
    }
}

impl Trcseqrstevr {
    /// Returns the value of the `RST_SEL` field.
    pub const fn rst_sel(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

bitflags! {
    /// `TRCSEQSTR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcseqstr: u64 {
    }
}

impl Trcseqstr {
    /// Returns the value of the `STATE` field.
    pub const fn state(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11
    }
}

bitflags! {
    /// `TRCSTALLCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcstallctlr: u64 {
        /// `ISTALL` bit.
        const ISTALL = 1 << 8;
        /// `NOOVERFLOW` bit.
        const NOOVERFLOW = 1 << 13;
    }
}

impl Trcstallctlr {
    /// Returns the value of the `LEVEL` field.
    pub const fn level(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

bitflags! {
    /// `TRCSTATR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcstatr: u64 {
        /// `IDLE` bit.
        const IDLE = 1 << 0;
        /// `PMSTABLE` bit.
        const PMSTABLE = 1 << 1;
    }
}

bitflags! {
    /// `TRCSYNCPR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcsyncpr: u64 {
    }
}

impl Trcsyncpr {
    /// Returns the value of the `PERIOD` field.
    pub const fn period(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

bitflags! {
    /// `TRCTRACEIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trctraceidr: u64 {
    }
}

impl Trctraceidr {
    /// Returns the value of the `TRACEID` field.
    pub const fn traceid(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111111
    }
}

bitflags! {
    /// `TRCTSCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trctsctlr: u64 {
        /// `EVENT_TYPE` bit.
        const EVENT_TYPE = 1 << 7;
    }
}

impl Trctsctlr {
    /// Returns the value of the `EVENT_SEL` field.
    pub const fn event_sel(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111
    }
}

bitflags! {
    /// `TRCVICTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcvictlr: u64 {
        /// `EVENT_TYPE` bit.
        const EVENT_TYPE = 1 << 7;
        /// `SSSTATUS` bit.
        const SSSTATUS = 1 << 9;
        /// `TRCRESET` bit.
        const TRCRESET = 1 << 10;
        /// `TRCERR` bit.
        const TRCERR = 1 << 11;
        /// `EXLEVEL_S_EL0` bit.
        const EXLEVEL_S_EL0 = 1 << 16;
        /// `EXLEVEL_S_EL1` bit.
        const EXLEVEL_S_EL1 = 1 << 17;
        /// `EXLEVEL_S_EL2` bit.
        const EXLEVEL_S_EL2 = 1 << 18;
        /// `EXLEVEL_S_EL3` bit.
        const EXLEVEL_S_EL3 = 1 << 19;
        /// `EXLEVEL_NS_EL0` bit.
        const EXLEVEL_NS_EL0 = 1 << 20;
        /// `EXLEVEL_NS_EL1` bit.
        const EXLEVEL_NS_EL1 = 1 << 21;
        /// `EXLEVEL_NS_EL2` bit.
        const EXLEVEL_NS_EL2 = 1 << 22;
        /// `EXLEVEL_RL_EL0` bit.
        const EXLEVEL_RL_EL0 = 1 << 24;
        /// `EXLEVEL_RL_EL1` bit.
        const EXLEVEL_RL_EL1 = 1 << 25;
        /// `EXLEVEL_RL_EL2` bit.
        const EXLEVEL_RL_EL2 = 1 << 26;
    }
}

bitflags! {
    /// `TRCVIIECTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcviiectlr: u64 {
        /// `INCLUDE[<m>]` bit 0.
        const INCLUDE_0 = 1 << 0;
        /// `INCLUDE[<m>]` bit 1.
        const INCLUDE_1 = 1 << 1;
        /// `INCLUDE[<m>]` bit 2.
        const INCLUDE_2 = 1 << 2;
        /// `INCLUDE[<m>]` bit 3.
        const INCLUDE_3 = 1 << 3;
        /// `INCLUDE[<m>]` bit 4.
        const INCLUDE_4 = 1 << 4;
        /// `INCLUDE[<m>]` bit 5.
        const INCLUDE_5 = 1 << 5;
        /// `INCLUDE[<m>]` bit 6.
        const INCLUDE_6 = 1 << 6;
        /// `INCLUDE[<m>]` bit 7.
        const INCLUDE_7 = 1 << 7;
        /// `EXCLUDE[<m>]` bit 0.
        const EXCLUDE_0 = 1 << 16;
        /// `EXCLUDE[<m>]` bit 1.
        const EXCLUDE_1 = 1 << 17;
        /// `EXCLUDE[<m>]` bit 2.
        const EXCLUDE_2 = 1 << 18;
        /// `EXCLUDE[<m>]` bit 3.
        const EXCLUDE_3 = 1 << 19;
        /// `EXCLUDE[<m>]` bit 4.
        const EXCLUDE_4 = 1 << 20;
        /// `EXCLUDE[<m>]` bit 5.
        const EXCLUDE_5 = 1 << 21;
        /// `EXCLUDE[<m>]` bit 6.
        const EXCLUDE_6 = 1 << 22;
        /// `EXCLUDE[<m>]` bit 7.
        const EXCLUDE_7 = 1 << 23;
    }
}

bitflags! {
    /// `TRCVIPCSSCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcvipcssctlr: u64 {
        /// `START[<m>]` bit 0.
        const START_0 = 1 << 0;
        /// `START[<m>]` bit 1.
        const START_1 = 1 << 1;
        /// `START[<m>]` bit 2.
        const START_2 = 1 << 2;
        /// `START[<m>]` bit 3.
        const START_3 = 1 << 3;
        /// `START[<m>]` bit 4.
        const START_4 = 1 << 4;
        /// `START[<m>]` bit 5.
        const START_5 = 1 << 5;
        /// `START[<m>]` bit 6.
        const START_6 = 1 << 6;
        /// `START[<m>]` bit 7.
        const START_7 = 1 << 7;
        /// `STOP[<m>]` bit 0.
        const STOP_0 = 1 << 16;
        /// `STOP[<m>]` bit 1.
        const STOP_1 = 1 << 17;
        /// `STOP[<m>]` bit 2.
        const STOP_2 = 1 << 18;
        /// `STOP[<m>]` bit 3.
        const STOP_3 = 1 << 19;
        /// `STOP[<m>]` bit 4.
        const STOP_4 = 1 << 20;
        /// `STOP[<m>]` bit 5.
        const STOP_5 = 1 << 21;
        /// `STOP[<m>]` bit 6.
        const STOP_6 = 1 << 22;
        /// `STOP[<m>]` bit 7.
        const STOP_7 = 1 << 23;
    }
}

bitflags! {
    /// `TRCVISSCTLR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcvissctlr: u64 {
        /// `START[<m>]` bit 0.
        const START_0 = 1 << 0;
        /// `START[<m>]` bit 1.
        const START_1 = 1 << 1;
        /// `START[<m>]` bit 2.
        const START_2 = 1 << 2;
        /// `START[<m>]` bit 3.
        const START_3 = 1 << 3;
        /// `START[<m>]` bit 4.
        const START_4 = 1 << 4;
        /// `START[<m>]` bit 5.
        const START_5 = 1 << 5;
        /// `START[<m>]` bit 6.
        const START_6 = 1 << 6;
        /// `START[<m>]` bit 7.
        const START_7 = 1 << 7;
        /// `START[<m>]` bit 8.
        const START_8 = 1 << 8;
        /// `START[<m>]` bit 9.
        const START_9 = 1 << 9;
        /// `START[<m>]` bit 10.
        const START_10 = 1 << 10;
        /// `START[<m>]` bit 11.
        const START_11 = 1 << 11;
        /// `START[<m>]` bit 12.
        const START_12 = 1 << 12;
        /// `START[<m>]` bit 13.
        const START_13 = 1 << 13;
        /// `START[<m>]` bit 14.
        const START_14 = 1 << 14;
        /// `START[<m>]` bit 15.
        const START_15 = 1 << 15;
        /// `STOP[<m>]` bit 0.
        const STOP_0 = 1 << 16;
        /// `STOP[<m>]` bit 1.
        const STOP_1 = 1 << 17;
        /// `STOP[<m>]` bit 2.
        const STOP_2 = 1 << 18;
        /// `STOP[<m>]` bit 3.
        const STOP_3 = 1 << 19;
        /// `STOP[<m>]` bit 4.
        const STOP_4 = 1 << 20;
        /// `STOP[<m>]` bit 5.
        const STOP_5 = 1 << 21;
        /// `STOP[<m>]` bit 6.
        const STOP_6 = 1 << 22;
        /// `STOP[<m>]` bit 7.
        const STOP_7 = 1 << 23;
        /// `STOP[<m>]` bit 8.
        const STOP_8 = 1 << 24;
        /// `STOP[<m>]` bit 9.
        const STOP_9 = 1 << 25;
        /// `STOP[<m>]` bit 10.
        const STOP_10 = 1 << 26;
        /// `STOP[<m>]` bit 11.
        const STOP_11 = 1 << 27;
        /// `STOP[<m>]` bit 12.
        const STOP_12 = 1 << 28;
        /// `STOP[<m>]` bit 13.
        const STOP_13 = 1 << 29;
        /// `STOP[<m>]` bit 14.
        const STOP_14 = 1 << 30;
        /// `STOP[<m>]` bit 15.
        const STOP_15 = 1 << 31;
    }
}

bitflags! {
    /// `TRCVMIDCCTLR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcvmidcctlr0: u64 {
        /// `COMP0[<m>]` bit 0.
        const COMP0_0 = 1 << 0;
        /// `COMP0[<m>]` bit 1.
        const COMP0_1 = 1 << 1;
        /// `COMP0[<m>]` bit 2.
        const COMP0_2 = 1 << 2;
        /// `COMP0[<m>]` bit 3.
        const COMP0_3 = 1 << 3;
        /// `COMP0[<m>]` bit 4.
        const COMP0_4 = 1 << 4;
        /// `COMP0[<m>]` bit 5.
        const COMP0_5 = 1 << 5;
        /// `COMP0[<m>]` bit 6.
        const COMP0_6 = 1 << 6;
        /// `COMP0[<m>]` bit 7.
        const COMP0_7 = 1 << 7;
        /// `COMP1[<m>]` bit 0.
        const COMP1_0 = 1 << 8;
        /// `COMP1[<m>]` bit 1.
        const COMP1_1 = 1 << 9;
        /// `COMP1[<m>]` bit 2.
        const COMP1_2 = 1 << 10;
        /// `COMP1[<m>]` bit 3.
        const COMP1_3 = 1 << 11;
        /// `COMP1[<m>]` bit 4.
        const COMP1_4 = 1 << 12;
        /// `COMP1[<m>]` bit 5.
        const COMP1_5 = 1 << 13;
        /// `COMP1[<m>]` bit 6.
        const COMP1_6 = 1 << 14;
        /// `COMP1[<m>]` bit 7.
        const COMP1_7 = 1 << 15;
        /// `COMP2[<m>]` bit 0.
        const COMP2_0 = 1 << 16;
        /// `COMP2[<m>]` bit 1.
        const COMP2_1 = 1 << 17;
        /// `COMP2[<m>]` bit 2.
        const COMP2_2 = 1 << 18;
        /// `COMP2[<m>]` bit 3.
        const COMP2_3 = 1 << 19;
        /// `COMP2[<m>]` bit 4.
        const COMP2_4 = 1 << 20;
        /// `COMP2[<m>]` bit 5.
        const COMP2_5 = 1 << 21;
        /// `COMP2[<m>]` bit 6.
        const COMP2_6 = 1 << 22;
        /// `COMP2[<m>]` bit 7.
        const COMP2_7 = 1 << 23;
        /// `COMP3[<m>]` bit 0.
        const COMP3_0 = 1 << 24;
        /// `COMP3[<m>]` bit 1.
        const COMP3_1 = 1 << 25;
        /// `COMP3[<m>]` bit 2.
        const COMP3_2 = 1 << 26;
        /// `COMP3[<m>]` bit 3.
        const COMP3_3 = 1 << 27;
        /// `COMP3[<m>]` bit 4.
        const COMP3_4 = 1 << 28;
        /// `COMP3[<m>]` bit 5.
        const COMP3_5 = 1 << 29;
        /// `COMP3[<m>]` bit 6.
        const COMP3_6 = 1 << 30;
        /// `COMP3[<m>]` bit 7.
        const COMP3_7 = 1 << 31;
    }
}

bitflags! {
    /// `TRCVMIDCCTLR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trcvmidcctlr1: u64 {
        /// `COMP4[<m>]` bit 0.
        const COMP4_0 = 1 << 0;
        /// `COMP4[<m>]` bit 1.
        const COMP4_1 = 1 << 1;
        /// `COMP4[<m>]` bit 2.
        const COMP4_2 = 1 << 2;
        /// `COMP4[<m>]` bit 3.
        const COMP4_3 = 1 << 3;
        /// `COMP4[<m>]` bit 4.
        const COMP4_4 = 1 << 4;
        /// `COMP4[<m>]` bit 5.
        const COMP4_5 = 1 << 5;
        /// `COMP4[<m>]` bit 6.
        const COMP4_6 = 1 << 6;
        /// `COMP4[<m>]` bit 7.
        const COMP4_7 = 1 << 7;
        /// `COMP5[<m>]` bit 0.
        const COMP5_0 = 1 << 8;
        /// `COMP5[<m>]` bit 1.
        const COMP5_1 = 1 << 9;
        /// `COMP5[<m>]` bit 2.
        const COMP5_2 = 1 << 10;
        /// `COMP5[<m>]` bit 3.
        const COMP5_3 = 1 << 11;
        /// `COMP5[<m>]` bit 4.
        const COMP5_4 = 1 << 12;
        /// `COMP5[<m>]` bit 5.
        const COMP5_5 = 1 << 13;
        /// `COMP5[<m>]` bit 6.
        const COMP5_6 = 1 << 14;
        /// `COMP5[<m>]` bit 7.
        const COMP5_7 = 1 << 15;
        /// `COMP6[<m>]` bit 0.
        const COMP6_0 = 1 << 16;
        /// `COMP6[<m>]` bit 1.
        const COMP6_1 = 1 << 17;
        /// `COMP6[<m>]` bit 2.
        const COMP6_2 = 1 << 18;
        /// `COMP6[<m>]` bit 3.
        const COMP6_3 = 1 << 19;
        /// `COMP6[<m>]` bit 4.
        const COMP6_4 = 1 << 20;
        /// `COMP6[<m>]` bit 5.
        const COMP6_5 = 1 << 21;
        /// `COMP6[<m>]` bit 6.
        const COMP6_6 = 1 << 22;
        /// `COMP6[<m>]` bit 7.
        const COMP6_7 = 1 << 23;
        /// `COMP7[<m>]` bit 0.
        const COMP7_0 = 1 << 24;
        /// `COMP7[<m>]` bit 1.
        const COMP7_1 = 1 << 25;
        /// `COMP7[<m>]` bit 2.
        const COMP7_2 = 1 << 26;
        /// `COMP7[<m>]` bit 3.
        const COMP7_3 = 1 << 27;
        /// `COMP7[<m>]` bit 4.
        const COMP7_4 = 1 << 28;
        /// `COMP7[<m>]` bit 5.
        const COMP7_5 = 1 << 29;
        /// `COMP7[<m>]` bit 6.
        const COMP7_6 = 1 << 30;
        /// `COMP7[<m>]` bit 7.
        const COMP7_7 = 1 << 31;
    }
}

bitflags! {
    /// `TRFCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Trfcr: u32 {
        /// `E0TRE` bit.
        const E0TRE = 1 << 0;
        /// `E1TRE` bit.
        const E1TRE = 1 << 1;
    }
}

impl Trfcr {
    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b11
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TRFCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrfcrEl1: u64 {
        /// `E0TRE` bit.
        const E0TRE = 1 << 0;
        /// `E1TRE` bit.
        const E1TRE = 1 << 1;
        /// `CX` bit.
        const CX = 1 << 3;
        /// `KE` bit.
        const KE = 1 << 10;
        /// `DnVM` bit.
        const DNVM = 1 << 11;
    }
}

#[cfg(feature = "el1")]
impl TrfcrEl1 {
    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b11
    }

    /// Returns the value of the `EE` field.
    pub const fn ee(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TRFCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TrfcrEl2: u64 {
        /// `E0HTRE` bit.
        const E0HTRE = 1 << 0;
        /// `E2TRE` bit.
        const E2TRE = 1 << 1;
        /// `CX` bit.
        const CX = 1 << 3;
        /// `KE` bit.
        const KE = 1 << 10;
        /// `DnVM` bit.
        const DNVM = 1 << 11;
    }
}

#[cfg(feature = "el2")]
impl TrfcrEl2 {
    /// Returns the value of the `TS` field.
    pub const fn ts(self) -> u8 {
        (self.bits() >> 5) as u8 & 0b11
    }

    /// Returns the value of the `EE` field.
    pub const fn ee(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }
}

bitflags! {
    /// `TTBCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `N` field.
    pub const fn n(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }

    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `T1SZ` field.
    pub const fn t1sz(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111
    }

    /// Returns the value of the `IRGN1` field.
    pub const fn irgn1(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11
    }

    /// Returns the value of the `ORGN1` field.
    pub const fn orgn1(self) -> u8 {
        (self.bits() >> 26) as u8 & 0b11
    }

    /// Returns the value of the `SH1` field.
    pub const fn sh1(self) -> u8 {
        (self.bits() >> 28) as u8 & 0b11
    }
}

bitflags! {
    /// `TTBCR2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

bitflags! {
    /// `TTBR0` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr0: u32 {
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
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 1) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `RGN` field.
    pub const fn rgn(self) -> u8 {
        (self.bits() >> 3) as u8 & 0b11
    }

    /// Returns the value of the `TTB0` field.
    pub const fn ttb0(self) -> u32 {
        (self.bits() >> 7) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111111
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
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> 1) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> 1) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }
}

bitflags! {
    /// `TTBR1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Ttbr1: u32 {
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
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 1) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `RGN` field.
    pub const fn rgn(self) -> u8 {
        (self.bits() >> 3) as u8 & 0b11
    }

    /// Returns the value of the `TTB1` field.
    pub const fn ttb1(self) -> u32 {
        (self.bits() >> 7) as u32 & 0b1111111111111111111111111
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u8 {
        (self.bits() >> 48) as u8 & 0b11111111
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
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> 1) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
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
    /// Returns the value of the `BADDR[47:1]` field.
    pub const fn baddr_47_1(self) -> u64 {
        (self.bits() >> 1) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `ASID` field.
    pub const fn asid(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TTTBRP_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TttbrpEl1: u64 {
        /// `FNG` bit.
        const FNG = 1 << 5;
    }
}

#[cfg(feature = "el1")]
impl TttbrpEl1 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TTTBRP_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TttbrpEl2: u64 {
        /// `FNG` bit.
        const FNG = 1 << 5;
    }
}

#[cfg(feature = "el2")]
impl TttbrpEl2 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `TTTBRP_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TttbrpEl3: u64 {
    }
}

#[cfg(feature = "el3")]
impl TttbrpEl3 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `TTTBRU_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TttbruEl1: u64 {
        /// `FNG` bit.
        const FNG = 1 << 5;
    }
}

#[cfg(feature = "el1")]
impl TttbruEl1 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `TTTBRU_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct TttbruEl2: u64 {
        /// `FNG` bit.
        const FNG = 1 << 5;
    }
}

#[cfg(feature = "el2")]
impl TttbruEl2 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 6) as u64 & 0b1111111111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `UAO` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Uao: u64 {
        /// `UAO` bit.
        const UAO = 1 << 23;
    }
}

bitflags! {
    /// `VBAR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vbar: u32 {
    }
}

impl Vbar {
    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u32 {
        (self.bits() >> 5) as u32 & 0b111111111111111111111111111
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
    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        (self.bits() >> 11) as u64 & 0b11111111111111111111111111111111111111111111111111111
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
    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        (self.bits() >> 11) as u64 & 0b11111111111111111111111111111111111111111111111111111
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `VBAR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VbarEl3: u64 {
        /// `UT` bit.
        const UT = 1 << 0;
    }
}

#[cfg(feature = "el3")]
impl VbarEl3 {
    /// Returns the value of the `VBA` field.
    pub const fn vba(self) -> u64 {
        (self.bits() >> 11) as u64 & 0b11111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `VDFSR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vdfsr: u32 {
        /// `ExT` bit.
        const EXT = 1 << 12;
    }
}

impl Vdfsr {
    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }
}

bitflags! {
    /// `VDISR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
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
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `STATUS` field.
    pub const fn status(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `VDISR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VdisrEl3: u64 {
        /// `IDS` bit.
        const IDS = 1 << 24;
        /// `A` bit.
        const A = 1 << 31;
    }
}

#[cfg(feature = "el3")]
impl VdisrEl3 {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VMECID_A_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VmecidAEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl VmecidAEl2 {
    /// Returns the value of the `MECID` field.
    pub const fn mecid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VMECID_P_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VmecidPEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl VmecidPEl2 {
    /// Returns the value of the `MECID` field.
    pub const fn mecid(self) -> u16 {
        (self.bits() >> 0) as u16 & 0b1111111111111111
    }
}

bitflags! {
    /// `VMPIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
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
    /// Returns the value of the `Aff0` field.
    pub const fn aff0(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff1` field.
    pub const fn aff1(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff2` field.
    pub const fn aff2(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b11111111
    }

    /// Returns the value of the `Aff3` field.
    pub const fn aff3(self) -> u8 {
        (self.bits() >> 32) as u8 & 0b11111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VNCCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VnccrEl2: u64 {
        /// `TTBA` bit.
        const TTBA = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl VnccrEl2 {
    /// Returns the value of the `TIndex` field.
    pub const fn tindex(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b1111111
    }

    /// Returns the value of the `FPOIndex` field.
    pub const fn fpoindex(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b1111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VNCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VncrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl VncrEl2 {
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 12) as u64 & 0b1111111111111111111111111111111111111111111111111111
    }
}

bitflags! {
    /// `VPIDR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Vpidr: u32 {
    }
}

impl Vpidr {
    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        (self.bits() >> 4) as u16 & 0b111111111111
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
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
    /// Returns the value of the `Revision` field.
    pub const fn revision(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `PartNum` field.
    pub const fn partnum(self) -> u16 {
        (self.bits() >> 4) as u16 & 0b111111111111
    }

    /// Returns the value of the `Architecture` field.
    pub const fn architecture(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b1111
    }

    /// Returns the value of the `Variant` field.
    pub const fn variant(self) -> u8 {
        (self.bits() >> 20) as u8 & 0b1111
    }

    /// Returns the value of the `Implementer` field.
    pub const fn implementer(self) -> u8 {
        (self.bits() >> 24) as u8 & 0b11111111
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
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }

    /// Returns the value of the `AET` field.
    pub const fn aet(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }
}

#[cfg(feature = "el3")]
bitflags! {
    /// `VSESR_EL3` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VsesrEl3: u64 {
        /// `IDS` bit.
        const IDS = 1 << 24;
    }
}

#[cfg(feature = "el3")]
impl VsesrEl3 {
    /// Returns the value of the `ISS` field.
    pub const fn iss(self) -> u32 {
        (self.bits() >> 0) as u32 & 0b111111111111111111111111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VSTCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VstcrEl2: u64 {
        /// RES1 bits in the `VSTCR_EL2` register.
        const RES1 = 0b10000000000000000000000000000000;
        /// `SW` bit.
        const SW = 1 << 29;
        /// `SA` bit.
        const SA = 1 << 30;
        /// `SL2` bit.
        const SL2 = 1 << 33;
    }
}

#[cfg(feature = "el2")]
impl VstcrEl2 {
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `VSTTBR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct VsttbrEl2: u64 {
        /// `CnP` bit.
        const CNP = 1 << 0;
    }
}

#[cfg(feature = "el2")]
impl VsttbrEl2 {
    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }
}

bitflags! {
    /// `VTCR` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
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
    /// Returns the value of the `T0SZ` field.
    pub const fn t0sz(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b111111
    }

    /// Returns the value of the `SL0` field.
    pub const fn sl0(self) -> u8 {
        (self.bits() >> 6) as u8 & 0b11
    }

    /// Returns the value of the `IRGN0` field.
    pub const fn irgn0(self) -> u8 {
        (self.bits() >> 8) as u8 & 0b11
    }

    /// Returns the value of the `ORGN0` field.
    pub const fn orgn0(self) -> u8 {
        (self.bits() >> 10) as u8 & 0b11
    }

    /// Returns the value of the `SH0` field.
    pub const fn sh0(self) -> u8 {
        (self.bits() >> 12) as u8 & 0b11
    }

    /// Returns the value of the `TG0` field.
    pub const fn tg0(self) -> u8 {
        (self.bits() >> 14) as u8 & 0b11
    }

    /// Returns the value of the `PS` field.
    pub const fn ps(self) -> u8 {
        (self.bits() >> 16) as u8 & 0b111
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
    /// Returns the value of the `BADDR` field.
    pub const fn baddr(self) -> u64 {
        (self.bits() >> 1) as u64 & 0b11111111111111111111111111111111111111111111111
    }

    /// Returns the value of the `SKL` field.
    pub const fn skl(self) -> u8 {
        (self.bits() >> 1) as u8 & 0b11
    }

    /// Returns the value of the `VMID` field.
    pub const fn vmid(self) -> u16 {
        (self.bits() >> 48) as u16 & 0b1111111111111111
    }
}

#[cfg(feature = "el1")]
bitflags! {
    /// `ZCR_EL1` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ZcrEl1: u64 {
    }
}

#[cfg(feature = "el1")]
impl ZcrEl1 {
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

#[cfg(feature = "el2")]
bitflags! {
    /// `ZCR_EL2` system register value.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct ZcrEl2: u64 {
    }
}

#[cfg(feature = "el2")]
impl ZcrEl2 {
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
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
    /// Returns the value of the `LEN` field.
    pub const fn len(self) -> u8 {
        (self.bits() >> 0) as u8 & 0b1111
    }
}

#[cfg(feature = "el1")]
read_write_sysreg!(accdata_el1: s3_0_c13_c0_5, u64: AccdataEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(actlr, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(actlr2, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(actlrmask_el1: s3_0_c1_c4_1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(actlrmask_el2: s3_4_c1_c4_1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(actlr_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(actlr_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(actlr_el3: s3_6_c1_c0_1, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(adfsr, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(afsr0_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(afsr0_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(afsr0_el3: s3_6_c5_c1_0, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(afsr1_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(afsr1_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(afsr1_el3: s3_6_c5_c1_1, u64, safe_read, fake::SYSREGS);
read_sysreg!(aidr, u32, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(aidr_el1: s3_1_c0_c0_7, u64, safe, fake::SYSREGS);
read_write_sysreg!(aifsr, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(allint: s3_0_c4_c3_0, u64: Allint, safe_read, fake::SYSREGS);
read_write_sysreg!(amair0, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(amair1, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(amair2_el1: s3_0_c10_c3_1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(amair2_el2: s3_4_c10_c3_1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(amair2_el3: s3_6_c10_c3_1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(amair_el1, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(amair_el2, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(amair_el3: s3_6_c10_c3_0, u64, safe_read, fake::SYSREGS);
read_sysreg!(amcfgr, u32: Amcfgr, safe, fake::SYSREGS);
read_sysreg!(amcfgr_el0: s3_3_c13_c2_1, u64: AmcfgrEl0, safe, fake::SYSREGS);
read_sysreg!(amcg1idr_el0: s3_3_c13_c2_6, u64: Amcg1idrEl0, safe, fake::SYSREGS);
read_sysreg!(amcgcr, u32: Amcgcr, safe, fake::SYSREGS);
read_sysreg!(amcgcr_el0: s3_3_c13_c2_2, u64: AmcgcrEl0, safe, fake::SYSREGS);
read_write_sysreg!(amcntenclr0, u32: Amcntenclr0, safe_read, fake::SYSREGS);
read_write_sysreg!(amcntenclr0_el0: s3_3_c13_c2_4, u64: Amcntenclr0El0, safe_read, fake::SYSREGS);
read_write_sysreg!(amcntenclr1, u32: Amcntenclr1, safe_read, fake::SYSREGS);
read_write_sysreg!(amcntenclr1_el0: s3_3_c13_c3_0, u64: Amcntenclr1El0, safe_read, fake::SYSREGS);
read_write_sysreg!(amcntenset0, u32: Amcntenset0, safe_read, fake::SYSREGS);
read_write_sysreg!(amcntenset0_el0: s3_3_c13_c2_5, u64: Amcntenset0El0, safe_read, fake::SYSREGS);
read_write_sysreg!(amcntenset1, u32: Amcntenset1, safe_read, fake::SYSREGS);
read_write_sysreg!(amcntenset1_el0: s3_3_c13_c3_1, u64: Amcntenset1El0, safe_read, fake::SYSREGS);
read_write_sysreg!(amcr, u32: Amcr, safe_read, fake::SYSREGS);
read_write_sysreg!(amcr_el0: s3_3_c13_c2_0, u64: AmcrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(amuserenr, u32: Amuserenr, safe_read, fake::SYSREGS);
read_write_sysreg!(amuserenr_el0: s3_3_c13_c2_3, u64: AmuserenrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apdakeyhi_el1: s3_0_c2_c2_1, u64: ApdakeyhiEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apdakeylo_el1: s3_0_c2_c2_0, u64: ApdakeyloEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apdbkeyhi_el1: s3_0_c2_c2_3, u64: ApdbkeyhiEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apdbkeylo_el1: s3_0_c2_c2_2, u64: ApdbkeyloEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apgakeyhi_el1: s3_0_c2_c3_1, u64: ApgakeyhiEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apgakeylo_el1: s3_0_c2_c3_0, u64: ApgakeyloEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apiakeyhi_el1, u64: ApiakeyhiEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apiakeylo_el1, u64: ApiakeyloEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apibkeyhi_el1: s3_0_c2_c1_3, u64: ApibkeyhiEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(apibkeylo_el1: s3_0_c2_c1_2, u64: ApibkeyloEl1, safe_read, fake::SYSREGS);
write_sysreg!(ats12nsopr, u32: Ats12nsopr, fake::SYSREGS);
write_sysreg!(ats12nsopw, u32: Ats12nsopw, fake::SYSREGS);
write_sysreg!(ats12nsour, u32: Ats12nsour, fake::SYSREGS);
write_sysreg!(ats12nsouw, u32: Ats12nsouw, fake::SYSREGS);
write_sysreg!(ats1cpr, u32: Ats1cpr, fake::SYSREGS);
write_sysreg!(ats1cprp, u32: Ats1cprp, fake::SYSREGS);
write_sysreg!(ats1cpw, u32: Ats1cpw, fake::SYSREGS);
write_sysreg!(ats1cpwp, u32: Ats1cpwp, fake::SYSREGS);
write_sysreg!(ats1cur, u32: Ats1cur, fake::SYSREGS);
write_sysreg!(ats1cuw, u32: Ats1cuw, fake::SYSREGS);
write_sysreg!(ats1hr, u32: Ats1hr, fake::SYSREGS);
write_sysreg!(ats1hw, u32: Ats1hw, fake::SYSREGS);
write_sysreg!(bpiall, u32, fake::SYSREGS);
write_sysreg!(bpiallis, u32, fake::SYSREGS);
write_sysreg!(bpimva, u32: Bpimva, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(brbcr_el1: s2_1_c9_c0_0, u64: BrbcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(brbcr_el2: s2_1_c9_c0_0, u64: BrbcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(brbfcr_el1: s2_1_c9_c0_1, u64: BrbfcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(brbidr0_el1: s2_1_c9_c2_0, u64: Brbidr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(brbinfinj_el1: s2_1_c9_c1_0, u64: BrbinfinjEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(brbsrcinj_el1: s2_1_c9_c1_1, u64: BrbsrcinjEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(brbtgtinj_el1: s2_1_c9_c1_2, u64: BrbtgtinjEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(brbts_el1: s2_1_c9_c0_2, u64: BrbtsEl1, safe_read, fake::SYSREGS);
read_sysreg!(ccsidr, u32: Ccsidr, safe, fake::SYSREGS);
read_sysreg!(ccsidr2, u32: Ccsidr2, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(ccsidr2_el1: s3_1_c0_c0_2, u64: Ccsidr2El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(ccsidr_el1, u64: CcsidrEl1, safe, fake::SYSREGS);
write_sysreg!(cfprctx, u32: Cfprctx, fake::SYSREGS);
read_sysreg!(clidr, u32: Clidr, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(clidr_el1, u64: ClidrEl1, safe, fake::SYSREGS);
read_write_sysreg!(cntfrq, u32: Cntfrq, safe_read, fake::SYSREGS);
read_write_sysreg!(cntfrq_el0, u64: CntfrqEl0, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(cnthctl, u32: Cnthctl, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthctl_el2, u64: CnthctlEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(cnthps_ctl, u32: CnthpsCtl, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthps_ctl_el2: s3_4_c14_c5_1, u64: CnthpsCtlEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthps_cval_el2: s3_4_c14_c5_2, u64: CnthpsCvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cnthps_tval, u32: CnthpsTval, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthps_tval_el2: s3_4_c14_c5_0, u64: CnthpsTvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cnthp_ctl, u32: CnthpCtl, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthp_ctl_el2: s3_4_c14_c2_1, u64: CnthpCtlEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthp_cval_el2: s3_4_c14_c2_2, u64: CnthpCvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cnthp_tval, u32: CnthpTval, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthp_tval_el2: s3_4_c14_c2_0, u64: CnthpTvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cnthvs_ctl, u32: CnthvsCtl, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthvs_ctl_el2: s3_4_c14_c4_1, u64: CnthvsCtlEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthvs_cval_el2: s3_4_c14_c4_2, u64: CnthvsCvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cnthvs_tval, u32: CnthvsTval, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthvs_tval_el2: s3_4_c14_c4_0, u64: CnthvsTvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cnthv_ctl, u32: CnthvCtl, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthv_ctl_el2: s3_4_c14_c3_1, u64: CnthvCtlEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthv_cval_el2: s3_4_c14_c3_2, u64: CnthvCvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cnthv_tval, u32: CnthvTval, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cnthv_tval_el2: s3_4_c14_c3_0, u64: CnthvTvalEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(cntkctl, u32: Cntkctl, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(cntkctl_el1: s3_0_c14_c1_0, u64: CntkctlEl1, safe_read, fake::SYSREGS);
read_sysreg!(cntpctss_el0: s3_3_c14_c0_5, u64: CntpctssEl0, safe, fake::SYSREGS);
read_sysreg!(cntpct_el0: s3_3_c14_c0_1, u64: CntpctEl0, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cntpoff_el2: s3_4_c14_c0_6, u64: CntpoffEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(cntps_ctl_el1: s3_7_c14_c2_1, u64: CntpsCtlEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(cntps_cval_el1: s3_7_c14_c2_2, u64: CntpsCvalEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(cntps_tval_el1: s3_7_c14_c2_0, u64: CntpsTvalEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(cntp_ctl, u32: CntpCtl, safe_read, fake::SYSREGS);
read_write_sysreg!(cntp_ctl_el0: s3_3_c14_c2_1, u64: CntpCtlEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(cntp_cval_el0: s3_3_c14_c2_2, u64: CntpCvalEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(cntp_tval, u32: CntpTval, safe_read, fake::SYSREGS);
read_write_sysreg!(cntp_tval_el0: s3_3_c14_c2_0, u64: CntpTvalEl0, safe_read, fake::SYSREGS);
read_sysreg!(cntvctss_el0: s3_3_c14_c0_6, u64: CntvctssEl0, safe, fake::SYSREGS);
read_sysreg!(cntvct_el0: s3_3_c14_c0_2, u64: CntvctEl0, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cntvoff_el2, u64: CntvoffEl2, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(cntv_ctl, u32: CntvCtl, safe_read, fake::SYSREGS);
read_write_sysreg!(cntv_ctl_el0: s3_3_c14_c3_1, u64: CntvCtlEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(cntv_cval_el0: s3_3_c14_c3_2, u64: CntvCvalEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(cntv_tval, u32: CntvTval, safe_read, fake::SYSREGS);
read_write_sysreg!(cntv_tval_el0: s3_3_c14_c3_0, u64: CntvTvalEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(contextidr, u32: Contextidr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(contextidr_el1, u64: ContextidrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(contextidr_el2, u64: ContextidrEl2, safe_read, safe_write, fake::SYSREGS);
write_sysreg!(cosprctx, u32: Cosprctx, fake::SYSREGS);
write_sysreg!(cp15dmb, u32, fake::SYSREGS);
write_sysreg!(cp15dsb, u32, fake::SYSREGS);
write_sysreg!(cp15isb, u32, fake::SYSREGS);
read_write_sysreg!(cpacr, u32: Cpacr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(cpacrmask_el1: s3_0_c1_c4_2, u64: CpacrmaskEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(cpacr_el1, u64: CpacrEl1, safe_read, fake::SYSREGS);
write_sysreg!(cpprctx, u32: Cpprctx, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cptrmask_el2: s3_4_c1_c4_2, u64: CptrmaskEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(cptr_el2, u64: CptrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(cptr_el3, u64: CptrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(csselr, u32: Csselr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(csselr_el1, u64: CsselrEl1, safe_read, safe_write, fake::SYSREGS);
read_sysreg!(ctr, u32: Ctr, safe, fake::SYSREGS);
read_sysreg!(ctr_el0, u64: CtrEl0, safe, fake::SYSREGS);
read_sysreg!(currentel: s3_0_c4_c2_2, u64: Currentel, safe, fake::SYSREGS);
read_write_sysreg!(dacr, u32: Dacr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(dacr32_el2: s3_4_c3_c0_0, u64: Dacr32El2, safe_read, fake::SYSREGS);
read_write_sysreg!(daif: s3_3_c4_c2_1, u64: Daif, safe_read, fake::SYSREGS);
read_sysreg!(dbgauthstatus, u32: Dbgauthstatus, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(dbgauthstatus_el1: s2_0_c7_c14_6, u64: DbgauthstatusEl1, safe, fake::SYSREGS);
read_write_sysreg!(dbgclaimclr, u32: Dbgclaimclr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(dbgclaimclr_el1: s2_0_c7_c9_6, u64: DbgclaimclrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(dbgclaimset, u32: Dbgclaimset, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(dbgclaimset_el1: s2_0_c7_c8_6, u64: DbgclaimsetEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(dbgdccint, u32: Dbgdccint, safe_read, fake::SYSREGS);
read_sysreg!(dbgdevid, u32: Dbgdevid, safe, fake::SYSREGS);
read_sysreg!(dbgdevid1, u32: Dbgdevid1, safe, fake::SYSREGS);
read_sysreg!(dbgdevid2, u32, safe, fake::SYSREGS);
read_sysreg!(dbgdidr, u32: Dbgdidr, safe, fake::SYSREGS);
read_sysreg!(dbgdrar, u32: Dbgdrar, safe, fake::SYSREGS);
read_sysreg!(dbgdsar, u32, safe, fake::SYSREGS);
read_write_sysreg!(dbgdscrext, u32: Dbgdscrext, safe_read, fake::SYSREGS);
read_sysreg!(dbgdscrint, u32: Dbgdscrint, safe, fake::SYSREGS);
read_sysreg!(dbgdtrrx_el0: s2_3_c0_c5_0, u64: DbgdtrrxEl0, safe, fake::SYSREGS);
read_write_sysreg!(dbgdtrrxext, u32: Dbgdtrrxext, safe_read, fake::SYSREGS);
read_sysreg!(dbgdtrrxint, u32: Dbgdtrrxint, safe, fake::SYSREGS);
write_sysreg!(dbgdtrtx_el0: s2_3_c0_c5_0, u64: DbgdtrtxEl0, fake::SYSREGS);
read_write_sysreg!(dbgdtrtxext, u32: Dbgdtrtxext, safe_read, fake::SYSREGS);
write_sysreg!(dbgdtrtxint, u32: Dbgdtrtxint, fake::SYSREGS);
read_write_sysreg!(dbgdtr_el0: s2_3_c0_c4_0, u64: DbgdtrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(dbgosdlr, u32: Dbgosdlr, safe_read, fake::SYSREGS);
read_write_sysreg!(dbgoseccr, u32: Dbgoseccr, safe_read, fake::SYSREGS);
write_sysreg!(dbgoslar, u32: Dbgoslar, fake::SYSREGS);
read_sysreg!(dbgoslsr, u32: Dbgoslsr, safe, fake::SYSREGS);
read_write_sysreg!(dbgprcr, u32: Dbgprcr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(dbgprcr_el1: s2_0_c1_c4_4, u64: DbgprcrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(dbgvcr, u32: Dbgvcr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(dbgvcr32_el2: s2_4_c0_c7_0, u64: Dbgvcr32El2, safe_read, fake::SYSREGS);
read_write_sysreg!(dbgwfar, u32, safe_read, fake::SYSREGS);
write_sysreg!(dccimvac, u32: Dccimvac, fake::SYSREGS);
write_sysreg!(dccisw, u32: Dccisw, fake::SYSREGS);
write_sysreg!(dccmvac, u32: Dccmvac, fake::SYSREGS);
write_sysreg!(dccmvau, u32: Dccmvau, fake::SYSREGS);
write_sysreg!(dccsw, u32: Dccsw, fake::SYSREGS);
write_sysreg!(dcimvac, u32: Dcimvac, fake::SYSREGS);
write_sysreg!(dcisw, u32: Dcisw, fake::SYSREGS);
read_sysreg!(dczid_el0: s3_3_c0_c0_7, u64: DczidEl0, safe, fake::SYSREGS);
read_write_sysreg!(dfar, u32: Dfar, safe_read, fake::SYSREGS);
read_write_sysreg!(dfsr, u32: Dfsr, safe_read, fake::SYSREGS);
read_write_sysreg!(disr, u32: Disr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(disr_el1, u64: DisrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(dit, u64: Dit, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(dlr, u32: Dlr, safe_read, fake::SYSREGS);
read_write_sysreg!(dlr_el0: s3_3_c4_c5_1, u64: DlrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(dpocr_el0: s3_3_c4_c5_2, u64: DpocrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(dpotbr0_el1: s3_0_c2_c0_6, u64: Dpotbr0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(dpotbr0_el2: s3_4_c2_c0_6, u64: Dpotbr0El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(dpotbr0_el3: s3_6_c2_c0_6, u64: Dpotbr0El3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(dpotbr1_el1: s3_0_c2_c0_7, u64: Dpotbr1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(dpotbr1_el2: s3_4_c2_c0_7, u64: Dpotbr1El2, safe_read, fake::SYSREGS);
read_write_sysreg!(dspsr, u32: Dspsr, safe_read, fake::SYSREGS);
read_write_sysreg!(dspsr2, u32: Dspsr2, safe_read, fake::SYSREGS);
read_write_sysreg!(dspsr_el0: s3_3_c4_c5_0, u64: DspsrEl0, safe_read, fake::SYSREGS);
write_sysreg!(dtlbiall, u32, fake::SYSREGS);
write_sysreg!(dtlbiasid, u32: Dtlbiasid, fake::SYSREGS);
write_sysreg!(dtlbimva, u32: Dtlbimva, fake::SYSREGS);
write_sysreg!(dvprctx, u32: Dvprctx, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(elr_el1, u64: ElrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(elr_el2, u64: ElrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(elr_el3: s3_6_c4_c0_1, u64: ElrEl3, safe_read, fake::SYSREGS);
read_sysreg!(erridr, u32: Erridr, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(erridr_el1: s3_0_c5_c3_0, u64: ErridrEl1, safe, fake::SYSREGS);
read_write_sysreg!(errselr, u32: Errselr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(errselr_el1: s3_0_c5_c3_1, u64: ErrselrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(erxaddr, u32: Erxaddr, safe_read, fake::SYSREGS);
read_write_sysreg!(erxaddr2, u32: Erxaddr2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxaddr_el1: s3_0_c5_c4_3, u64: ErxaddrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(erxctlr, u32: Erxctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(erxctlr2, u32: Erxctlr2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxctlr_el1: s3_0_c5_c4_1, u64: ErxctlrEl1, safe_read, fake::SYSREGS);
read_sysreg!(erxfr, u32: Erxfr, safe, fake::SYSREGS);
read_sysreg!(erxfr2, u32: Erxfr2, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(erxfr_el1: s3_0_c5_c4_0, u64: ErxfrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(erxgsr_el1: s3_0_c5_c3_2, u64: ErxgsrEl1, safe, fake::SYSREGS);
read_write_sysreg!(erxmisc0, u32: Erxmisc0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxmisc0_el1: s3_0_c5_c5_0, u64: Erxmisc0El1, safe_read, fake::SYSREGS);
read_write_sysreg!(erxmisc1, u32: Erxmisc1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxmisc1_el1: s3_0_c5_c5_1, u64: Erxmisc1El1, safe_read, fake::SYSREGS);
read_write_sysreg!(erxmisc2, u32: Erxmisc2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxmisc2_el1: s3_0_c5_c5_2, u64: Erxmisc2El1, safe_read, fake::SYSREGS);
read_write_sysreg!(erxmisc3, u32: Erxmisc3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxmisc3_el1: s3_0_c5_c5_3, u64: Erxmisc3El1, safe_read, fake::SYSREGS);
read_write_sysreg!(erxmisc4, u32: Erxmisc4, safe_read, fake::SYSREGS);
read_write_sysreg!(erxmisc5, u32: Erxmisc5, safe_read, fake::SYSREGS);
read_write_sysreg!(erxmisc6, u32: Erxmisc6, safe_read, fake::SYSREGS);
read_write_sysreg!(erxmisc7, u32: Erxmisc7, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxpfgcdn_el1: s3_0_c5_c4_6, u64: ErxpfgcdnEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxpfgctl_el1: s3_0_c5_c4_5, u64: ErxpfgctlEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(erxpfgf_el1: s3_0_c5_c4_4, u64: ErxpfgfEl1, safe, fake::SYSREGS);
read_write_sysreg!(erxstatus, u32: Erxstatus, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(erxstatus_el1: s3_0_c5_c4_2, u64: ErxstatusEl1, safe_read, fake::SYSREGS);
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
#[cfg(feature = "el3")]
read_write_sysreg!(far_el3: s3_6_c6_c0_0, u64: FarEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(fcseidr, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(fgwte3_el3: s3_6_c1_c1_5, u64: Fgwte3El3, safe_read, fake::SYSREGS);
read_write_sysreg!(fpcr: s3_3_c4_c4_0, u64: Fpcr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(fpexc32_el2: s3_4_c5_c3_0, u64: Fpexc32El2, safe_read, fake::SYSREGS);
read_write_sysreg!(fpmr: s3_3_c4_c4_2, u64: Fpmr, safe_read, fake::SYSREGS);
read_write_sysreg!(fpsr: s3_3_c4_c4_1, u64: Fpsr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(gcr_el1, u64: GcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(gcscre0_el1: s3_0_c2_c5_2, u64: Gcscre0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(gcscr_el1, u64: GcscrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(gcscr_el2, u64: GcscrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(gcscr_el3: s3_6_c2_c5_0, u64: GcscrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(gcspr_el0: s3_3_c2_c5_1, u64: GcsprEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(gcspr_el1: s3_0_c2_c5_1, u64: GcsprEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(gcspr_el2: s3_4_c2_c5_1, u64: GcsprEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(gcspr_el3: s3_6_c2_c5_1, u64: GcsprEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(gmid_el1: s3_1_c0_c0_4, u64: GmidEl1, safe, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(gpcbw_el3: s3_6_c2_c1_5, u64: GpcbwEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(gpccr_el3: s3_6_c2_c1_6, u64: GpccrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(gptbr_el3: s3_6_c2_c1_4, u64: GptbrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hacdbsbr_el2: s3_4_c2_c3_4, u64: HacdbsbrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hacdbscons_el2: s3_4_c2_c3_5, u64: HacdbsconsEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hacr, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hacr_el2, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(hactlr, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(hactlr2, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(hadfsr, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hafgrtr_el2: s3_4_c3_c1_6, u64: HafgrtrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(haifsr, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(hamair0, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(hamair1, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(hcptr, u32: Hcptr, safe_read, fake::SYSREGS);
read_write_sysreg!(hcr, u32: Hcr, safe_read, fake::SYSREGS);
read_write_sysreg!(hcr2, u32: Hcr2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hcrmask_el2: s3_4_c1_c5_6, u64: HcrmaskEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hcrxmask_el2: s3_4_c1_c5_7, u64: HcrxmaskEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hcrx_el2, u64: HcrxEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hcr_el2, u64: HcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdbssbr_el2: s3_4_c2_c3_2, u64: HdbssbrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdbssprod_el2: s3_4_c2_c3_3, u64: HdbssprodEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hdcr, u32: Hdcr, safe_read, fake::SYSREGS);
read_write_sysreg!(hdfar, u32: Hdfar, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdfgrtr2_el2, u64: Hdfgrtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdfgrtr_el2: s3_4_c3_c1_4, u64: HdfgrtrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdfgwtr2_el2, u64: Hdfgwtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hdfgwtr_el2: s3_4_c3_c1_5, u64: HdfgwtrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgitr2_el2, u64: Hfgitr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgitr_el2: s3_4_c1_c1_6, u64: HfgitrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgrtr2_el2, u64: Hfgrtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgrtr_el2: s3_4_c1_c1_4, u64: HfgrtrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgwtr2_el2: s3_4_c3_c1_3, u64: Hfgwtr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hfgwtr_el2, u64: HfgwtrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hifar, u32: Hifar, safe_read, fake::SYSREGS);
read_write_sysreg!(hmair0, u32: Hmair0, safe_read, fake::SYSREGS);
read_write_sysreg!(hmair1, u32: Hmair1, safe_read, fake::SYSREGS);
read_write_sysreg!(hpfar, u32: Hpfar, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hpfar_el2, u64: HpfarEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(hrmr, u32: Hrmr, safe_read, fake::SYSREGS);
read_write_sysreg!(hsctlr, u32: Hsctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(hsr, u32: Hsr, safe_read, fake::SYSREGS);
read_write_sysreg!(hstr, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(hstr_el2, u64, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(htcr, u32: Htcr, safe_read, fake::SYSREGS);
read_write_sysreg!(htpidr, u32: Htpidr, safe_read, fake::SYSREGS);
read_write_sysreg!(htrfcr, u32: Htrfcr, safe_read, fake::SYSREGS);
read_write_sysreg!(hvbar, u32: Hvbar, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_apr_el1: s3_1_c12_c0_0, u64: IccAprEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(icc_apr_el3: s3_6_c12_c8_0, u64: IccAprEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icc_asgi1r_el1: s3_0_c12_c11_6, u64: IccAsgi1rEl1, fake::SYSREGS);
read_write_sysreg!(icc_bpr0, u32: IccBpr0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_bpr0_el1: s3_0_c12_c8_3, u64: IccBpr0El1, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_bpr1, u32: IccBpr1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_bpr1_el1: s3_0_c12_c12_3, u64: IccBpr1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_cr0_el1: s3_1_c12_c0_1, u64: IccCr0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(icc_cr0_el3: s3_6_c12_c9_0, u64: IccCr0El3, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_ctlr, u32: IccCtlr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_ctlr_el1: s3_0_c12_c12_4, u64: IccCtlrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(icc_ctlr_el3: s3_6_c12_c12_4, u64: IccCtlrEl3, safe_read, fake::SYSREGS);
write_sysreg!(icc_dir, u32: IccDir, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icc_dir_el1: s3_0_c12_c11_1, u64: IccDirEl1, fake::SYSREGS);
#[cfg(feature = "el3")]
read_sysreg!(icc_domhppir_el3: s3_6_c12_c8_2, u64: IccDomhppirEl3, safe, fake::SYSREGS);
write_sysreg!(icc_eoir0, u32: IccEoir0, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icc_eoir0_el1: s3_0_c12_c8_1, u64: IccEoir0El1, fake::SYSREGS);
write_sysreg!(icc_eoir1, u32: IccEoir1, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icc_eoir1_el1: s3_0_c12_c12_1, u64: IccEoir1El1, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_hapr_el1: s3_1_c12_c0_3, u64: IccHaprEl1, safe, fake::SYSREGS);
read_sysreg!(icc_hppir0, u32: IccHppir0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_hppir0_el1: s3_0_c12_c8_2, u64: IccHppir0El1, safe, fake::SYSREGS);
read_sysreg!(icc_hppir1, u32: IccHppir1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_hppir1_el1: s3_0_c12_c12_2, u64: IccHppir1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_hppir_el1: s3_0_c12_c10_3, u64: IccHppirEl1, safe, fake::SYSREGS);
#[cfg(feature = "el3")]
read_sysreg!(icc_hppir_el3: s3_6_c12_c9_1, u64: IccHppirEl3, safe, fake::SYSREGS);
read_write_sysreg!(icc_hsre, u32: IccHsre, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_iaffidr_el1: s3_0_c12_c10_5, u64: IccIaffidrEl1, safe, fake::SYSREGS);
read_sysreg!(icc_iar0, u32: IccIar0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_iar0_el1: s3_0_c12_c8_0, u64: IccIar0El1, safe, fake::SYSREGS);
read_sysreg!(icc_iar1, u32: IccIar1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_iar1_el1: s3_0_c12_c12_0, u64: IccIar1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_icsr_el1: s3_0_c12_c10_4, u64: IccIcsrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_idr0_el1: s3_0_c12_c10_2, u64: IccIdr0El1, safe, fake::SYSREGS);
read_write_sysreg!(icc_igrpen0, u32: IccIgrpen0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_igrpen0_el1: s3_0_c12_c12_6, u64: IccIgrpen0El1, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_igrpen1, u32: IccIgrpen1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_igrpen1_el1: s3_0_c12_c12_7, u64: IccIgrpen1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(icc_igrpen1_el3: s3_6_c12_c12_7, u64: IccIgrpen1El3, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_mctlr, u32: IccMctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_mgrpen1, u32: IccMgrpen1, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_msre, u32: IccMsre, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_nmiar1_el1: s3_0_c12_c9_5, u64: IccNmiar1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_pcr_el1: s3_1_c12_c0_2, u64: IccPcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(icc_pcr_el3: s3_6_c12_c8_1, u64: IccPcrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(icc_pmr, u32: IccPmr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icc_pmr_el1: s3_0_c4_c6_0, u64: IccPmrEl1, safe_read, fake::SYSREGS);
read_sysreg!(icc_rpr, u32: IccRpr, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icc_rpr_el1: s3_0_c12_c11_3, u64: IccRprEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icc_sgi0r_el1: s3_0_c12_c11_7, u64: IccSgi0rEl1, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icc_sgi1r_el1: s3_0_c12_c11_5, u64: IccSgi1rEl1, fake::SYSREGS);
read_write_sysreg!(icc_sre, u32: IccSre, safe_read, fake::SYSREGS);
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
read_write_sysreg!(ich_apr_el2: s3_4_c12_c8_4, u64: IchAprEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_contextr_el2: s3_4_c12_c11_6, u64: IchContextrEl2, safe_read, fake::SYSREGS);
read_sysreg!(ich_eisr, u32: IchEisr, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_sysreg!(ich_eisr_el2: s3_4_c12_c11_3, u64: IchEisrEl2, safe, fake::SYSREGS);
read_sysreg!(ich_elrsr, u32: IchElrsr, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_sysreg!(ich_elrsr_el2: s3_4_c12_c11_5, u64: IchElrsrEl2, safe, fake::SYSREGS);
read_write_sysreg!(ich_hcr, u32: IchHcr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_hcr_el2, u64: IchHcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_hfgitr_el2: s3_4_c12_c9_7, u64: IchHfgitrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_hfgrtr_el2: s3_4_c12_c9_4, u64: IchHfgrtrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_hfgwtr_el2: s3_4_c12_c9_6, u64: IchHfgwtrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_sysreg!(ich_hppir_el2: s3_4_c12_c8_5, u64: IchHppirEl2, safe, fake::SYSREGS);
read_sysreg!(ich_misr, u32: IchMisr, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_sysreg!(ich_misr_el2: s3_4_c12_c11_2, u64: IchMisrEl2, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_vctlr_el2: s3_4_c12_c11_4, u64: IchVctlrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(ich_vmcr, u32: IchVmcr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ich_vmcr_el2, u64: IchVmcrEl2, safe_read, safe_write, fake::SYSREGS);
read_sysreg!(ich_vtr, u32: IchVtr, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_sysreg!(ich_vtr_el2: s3_4_c12_c11_1, u64: IchVtrEl2, safe, fake::SYSREGS);
write_sysreg!(iciallu, u32, fake::SYSREGS);
write_sysreg!(icialluis, u32, fake::SYSREGS);
write_sysreg!(icimvau, u32: Icimvau, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_apr_el1: s3_1_c12_c0_0, u64: IcvAprEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(icv_bpr0, u32: IcvBpr0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_bpr0_el1: s3_0_c12_c8_3, u64: IcvBpr0El1, safe_read, fake::SYSREGS);
read_write_sysreg!(icv_bpr1, u32: IcvBpr1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_bpr1_el1: s3_0_c12_c12_3, u64: IcvBpr1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_cr0_el1: s3_1_c12_c0_1, u64: IcvCr0El1, safe_read, fake::SYSREGS);
read_write_sysreg!(icv_ctlr, u32: IcvCtlr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_ctlr_el1: s3_0_c12_c12_4, u64: IcvCtlrEl1, safe_read, fake::SYSREGS);
write_sysreg!(icv_dir, u32: IcvDir, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icv_dir_el1: s3_0_c12_c11_1, u64: IcvDirEl1, fake::SYSREGS);
write_sysreg!(icv_eoir0, u32: IcvEoir0, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icv_eoir0_el1: s3_0_c12_c8_1, u64: IcvEoir0El1, fake::SYSREGS);
write_sysreg!(icv_eoir1, u32: IcvEoir1, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(icv_eoir1_el1: s3_0_c12_c12_1, u64: IcvEoir1El1, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_hapr_el1: s3_1_c12_c0_3, u64: IcvHaprEl1, safe, fake::SYSREGS);
read_sysreg!(icv_hppir0, u32: IcvHppir0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_hppir0_el1: s3_0_c12_c8_2, u64: IcvHppir0El1, safe, fake::SYSREGS);
read_sysreg!(icv_hppir1, u32: IcvHppir1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_hppir1_el1: s3_0_c12_c12_2, u64: IcvHppir1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_hppir_el1: s3_0_c12_c10_3, u64: IcvHppirEl1, safe, fake::SYSREGS);
read_sysreg!(icv_iar0, u32: IcvIar0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_iar0_el1: s3_0_c12_c8_0, u64: IcvIar0El1, safe, fake::SYSREGS);
read_sysreg!(icv_iar1, u32: IcvIar1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_iar1_el1: s3_0_c12_c12_0, u64: IcvIar1El1, safe, fake::SYSREGS);
read_write_sysreg!(icv_igrpen0, u32: IcvIgrpen0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_igrpen0_el1: s3_0_c12_c12_6, u64: IcvIgrpen0El1, safe_read, fake::SYSREGS);
read_write_sysreg!(icv_igrpen1, u32: IcvIgrpen1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_igrpen1_el1: s3_0_c12_c12_7, u64: IcvIgrpen1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_nmiar1_el1: s3_0_c12_c9_5, u64: IcvNmiar1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_pcr_el1: s3_1_c12_c0_2, u64: IcvPcrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(icv_pmr, u32: IcvPmr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(icv_pmr_el1: s3_0_c4_c6_0, u64: IcvPmrEl1, safe_read, fake::SYSREGS);
read_sysreg!(icv_rpr, u32: IcvRpr, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(icv_rpr_el1: s3_0_c12_c11_3, u64: IcvRprEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64afr0_el1: s3_0_c0_c5_4, u64, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64afr1_el1: s3_0_c0_c5_5, u64, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64dfr0_el1, u64: IdAa64dfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64dfr1_el1, u64: IdAa64dfr1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64dfr2_el1: s3_0_c0_c5_2, u64: IdAa64dfr2El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64fpfr0_el1: s3_0_c0_c4_7, u64: IdAa64fpfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64isar0_el1: s3_0_c0_c6_0, u64: IdAa64isar0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64isar1_el1: s3_0_c0_c6_1, u64: IdAa64isar1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64isar2_el1: s3_0_c0_c6_2, u64: IdAa64isar2El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64isar3_el1: s3_0_c0_c6_3, u64: IdAa64isar3El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr0_el1, u64: IdAa64mmfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr1_el1, u64: IdAa64mmfr1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr2_el1, u64: IdAa64mmfr2El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr3_el1, u64: IdAa64mmfr3El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64mmfr4_el1: s3_0_c0_c7_4, u64: IdAa64mmfr4El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64pfr0_el1, u64: IdAa64pfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64pfr1_el1, u64: IdAa64pfr1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64pfr2_el1: s3_0_c0_c4_2, u64: IdAa64pfr2El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64smfr0_el1: s3_0_c0_c4_5, u64: IdAa64smfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_aa64zfr0_el1: s3_0_c0_c4_4, u64: IdAa64zfr0El1, safe, fake::SYSREGS);
read_sysreg!(id_afr0, u32, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_afr0_el1: s3_0_c0_c1_3, u64, safe, fake::SYSREGS);
read_sysreg!(id_dfr0, u32: IdDfr0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_dfr0_el1: s3_0_c0_c1_2, u64: IdDfr0El1, safe, fake::SYSREGS);
read_sysreg!(id_dfr1, u32: IdDfr1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_dfr1_el1: s3_0_c0_c3_5, u64: IdDfr1El1, safe, fake::SYSREGS);
read_sysreg!(id_isar0, u32: IdIsar0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_isar0_el1: s3_0_c0_c2_0, u64: IdIsar0El1, safe, fake::SYSREGS);
read_sysreg!(id_isar1, u32: IdIsar1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_isar1_el1: s3_0_c0_c2_1, u64: IdIsar1El1, safe, fake::SYSREGS);
read_sysreg!(id_isar2, u32: IdIsar2, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_isar2_el1: s3_0_c0_c2_2, u64: IdIsar2El1, safe, fake::SYSREGS);
read_sysreg!(id_isar3, u32: IdIsar3, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_isar3_el1: s3_0_c0_c2_3, u64: IdIsar3El1, safe, fake::SYSREGS);
read_sysreg!(id_isar4, u32: IdIsar4, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_isar4_el1: s3_0_c0_c2_4, u64: IdIsar4El1, safe, fake::SYSREGS);
read_sysreg!(id_isar5, u32: IdIsar5, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_isar5_el1: s3_0_c0_c2_5, u64: IdIsar5El1, safe, fake::SYSREGS);
read_sysreg!(id_isar6, u32: IdIsar6, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_isar6_el1: s3_0_c0_c2_7, u64: IdIsar6El1, safe, fake::SYSREGS);
read_sysreg!(id_mmfr0, u32: IdMmfr0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_mmfr0_el1: s3_0_c0_c1_4, u64: IdMmfr0El1, safe, fake::SYSREGS);
read_sysreg!(id_mmfr1, u32: IdMmfr1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_mmfr1_el1: s3_0_c0_c1_5, u64: IdMmfr1El1, safe, fake::SYSREGS);
read_sysreg!(id_mmfr2, u32: IdMmfr2, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_mmfr2_el1: s3_0_c0_c1_6, u64: IdMmfr2El1, safe, fake::SYSREGS);
read_sysreg!(id_mmfr3, u32: IdMmfr3, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_mmfr3_el1: s3_0_c0_c1_7, u64: IdMmfr3El1, safe, fake::SYSREGS);
read_sysreg!(id_mmfr4, u32: IdMmfr4, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_mmfr4_el1: s3_0_c0_c2_6, u64: IdMmfr4El1, safe, fake::SYSREGS);
read_sysreg!(id_mmfr5, u32: IdMmfr5, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_mmfr5_el1: s3_0_c0_c3_6, u64: IdMmfr5El1, safe, fake::SYSREGS);
read_sysreg!(id_pfr0, u32: IdPfr0, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_pfr0_el1: s3_0_c0_c1_0, u64: IdPfr0El1, safe, fake::SYSREGS);
read_sysreg!(id_pfr1, u32: IdPfr1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_pfr1_el1: s3_0_c0_c1_1, u64: IdPfr1El1, safe, fake::SYSREGS);
read_sysreg!(id_pfr2, u32: IdPfr2, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(id_pfr2_el1: s3_0_c0_c3_4, u64: IdPfr2El1, safe, fake::SYSREGS);
read_write_sysreg!(ifar, u32: Ifar, safe_read, fake::SYSREGS);
read_write_sysreg!(ifsr, u32: Ifsr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ifsr32_el2: s3_4_c5_c0_1, u64: Ifsr32El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(irtbrp_el1: s3_0_c2_c0_5, u64: IrtbrpEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(irtbrp_el2: s3_4_c2_c0_5, u64: IrtbrpEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(irtbrp_el3: s3_6_c2_c0_5, u64: IrtbrpEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(irtbru_el1: s3_0_c2_c0_4, u64: IrtbruEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(irtbru_el2: s3_4_c2_c0_4, u64: IrtbruEl2, safe_read, fake::SYSREGS);
read_sysreg!(isr, u32: Isr, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(isr_el1, u64: IsrEl1, safe, fake::SYSREGS);
write_sysreg!(itlbiall, u32, fake::SYSREGS);
write_sysreg!(itlbiasid, u32: Itlbiasid, fake::SYSREGS);
write_sysreg!(itlbimva, u32: Itlbimva, fake::SYSREGS);
read_sysreg!(jidr, u32, safe, fake::SYSREGS);
read_write_sysreg!(jmcr, u32, safe_read, fake::SYSREGS);
read_write_sysreg!(joscr, u32, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(ldstt_el1: s3_0_c2_c1_7, u64: LdsttEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(ldstt_el2: s3_4_c2_c1_7, u64: LdsttEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(lorc_el1: s3_0_c10_c4_3, u64: LorcEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(lorea_el1: s3_0_c10_c4_1, u64: LoreaEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(lorid_el1: s3_0_c10_c4_7, u64: LoridEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(lorn_el1: s3_0_c10_c4_2, u64: LornEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(lorsa_el1: s3_0_c10_c4_0, u64: LorsaEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(mair0, u32: Mair0, safe_read, fake::SYSREGS);
read_write_sysreg!(mair1, u32: Mair1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mair2_el1: s3_0_c10_c2_1, u64: Mair2El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mair2_el2: s3_4_c10_c1_1, u64: Mair2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mair2_el3: s3_6_c10_c1_1, u64: Mair2El3, safe_read, fake::SYSREGS);
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
read_sysreg!(mdccsr_el0: s2_3_c0_c1_0, u64: MdccsrEl0, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mdcr_el2, u64: MdcrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mdcr_el3, u64: MdcrEl3, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mdrar_el1: s2_0_c1_c0_0, u64: MdrarEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mdscr_el1, u64: MdscrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mdselr_el1: s2_0_c0_c4_2, u64: MdselrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mdstepop_el1: s2_0_c0_c5_2, u64: MdstepopEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_sysreg!(mecidr_el2: s3_4_c10_c8_7, u64: MecidrEl2, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mecid_a0_el2: s3_4_c10_c8_1, u64: MecidA0El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mecid_a1_el2: s3_4_c10_c8_3, u64: MecidA1El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mecid_p0_el2: s3_4_c10_c8_0, u64: MecidP0El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mecid_p1_el2: s3_4_c10_c8_2, u64: MecidP1El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mecid_rl_a_el3: s3_6_c10_c10_1, u64: MecidRlAEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mfar_el3: s3_6_c6_c0_5, u64: MfarEl3, safe_read, fake::SYSREGS);
read_sysreg!(midr, u32: Midr, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(midr_el1, u64: MidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mpam0_el1: s3_0_c10_c5_1, u64: Mpam0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mpam1_el1: s3_0_c10_c5_0, u64: Mpam1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpam2_el2, u64: Mpam2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mpam3_el3: s3_6_c10_c5_0, u64: Mpam3El3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mpambw0_el1: s3_0_c10_c5_5, u64: Mpambw0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mpambw1_el1: s3_0_c10_c5_4, u64: Mpambw1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpambw2_el2: s3_4_c10_c5_4, u64: Mpambw2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mpambw3_el3: s3_6_c10_c5_4, u64: Mpambw3El3, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpambwcap_el2: s3_4_c10_c5_6, u64: MpambwcapEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mpambwidr_el1: s3_0_c10_c4_5, u64: MpambwidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mpambwsm_el1: s3_0_c10_c5_7, u64: MpambwsmEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mpamctl_el1: s3_0_c10_c5_2, u64: MpamctlEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamctl_el2: s3_4_c10_c5_2, u64: MpamctlEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(mpamctl_el3: s3_6_c10_c5_2, u64: MpamctlEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamhcr_el2, u64: MpamhcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mpamidr_el1, u64: MpamidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(mpamsm_el1: s3_0_c10_c5_3, u64: MpamsmEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvidcr_el2: s3_4_c10_c7_0, u64: MpamvidcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvidsr_el2: s3_4_c10_c7_1, u64: MpamvidsrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm0_el2, u64: Mpamvpm0El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm1_el2, u64: Mpamvpm1El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm2_el2, u64: Mpamvpm2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm3_el2, u64: Mpamvpm3El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm4_el2, u64: Mpamvpm4El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm5_el2, u64: Mpamvpm5El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm6_el2, u64: Mpamvpm6El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpm7_el2, u64: Mpamvpm7El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(mpamvpmv_el2, u64: MpamvpmvEl2, safe_read, fake::SYSREGS);
read_sysreg!(mpidr, u32: Mpidr, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mpidr_el1, u64: MpidrEl1, safe, fake::SYSREGS);
read_write_sysreg!(mvbar, u32: Mvbar, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mvfr0_el1: s3_0_c0_c3_0, u64: Mvfr0El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mvfr1_el1: s3_0_c0_c3_1, u64: Mvfr1El1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(mvfr2_el1: s3_0_c0_c3_2, u64: Mvfr2El1, safe, fake::SYSREGS);
read_write_sysreg!(nmrr, u32: Nmrr, safe_read, fake::SYSREGS);
read_write_sysreg!(nsacr, u32: Nsacr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(nvhcrmask_el2: s3_4_c1_c5_4, u64: NvhcrmaskEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(nvhcrxmask_el2: s3_4_c1_c5_5, u64: NvhcrxmaskEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(nvhcrx_el2: s3_4_c1_c5_1, u64: NvhcrxEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(nvhcr_el2: s3_4_c1_c5_0, u64: NvhcrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(nzcv: s3_3_c4_c2_0, u64: Nzcv, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(osdlr_el1: s2_0_c1_c3_4, u64: OsdlrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(osdtrrx_el1: s2_0_c0_c0_2, u64: OsdtrrxEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(osdtrtx_el1: s2_0_c0_c3_2, u64: OsdtrtxEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(oseccr_el1: s2_0_c0_c6_2, u64: OseccrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
write_sysreg!(oslar_el1: s2_0_c1_c0_4, u64: OslarEl1, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(oslsr_el1: s2_0_c1_c1_4, u64: OslsrEl1, safe, fake::SYSREGS);
read_write_sysreg!(pan: s3_0_c4_c2_3, u64: Pan, safe_read, fake::SYSREGS);
read_write_sysreg!(par, u32: Par, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(par_el1, u64: ParEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pfar_el1: s3_0_c6_c0_5, u64: PfarEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(pfar_el2: s3_4_c6_c0_5, u64: PfarEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pire0_el1: s3_0_c10_c2_2, u64: Pire0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(pire0_el2: s3_4_c10_c2_2, u64: Pire0El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pir_el1: s3_0_c10_c2_3, u64: PirEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(pir_el2: s3_4_c10_c2_3, u64: PirEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(pir_el3: s3_6_c10_c2_3, u64: PirEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(pm: s3_0_c4_c3_1, u64: Pm, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(pmbidr_el1: s3_0_c9_c10_7, u64: PmbidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmblimitr_el1: s3_0_c9_c10_0, u64: PmblimitrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmbmar_el1: s3_0_c9_c10_5, u64: PmbmarEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmbptr_el1: s3_0_c9_c10_1, u64: PmbptrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmbsr_el1: s3_0_c9_c10_3, u64: PmbsrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(pmbsr_el2: s3_4_c9_c10_3, u64: PmbsrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(pmbsr_el3: s3_6_c9_c10_3, u64: PmbsrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(pmccfiltr, u32: Pmccfiltr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmccfiltr_el0: s3_3_c14_c15_7, u64: PmccfiltrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(pmccntr, u32: Pmccntr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmccntr_el0: s3_3_c9_c13_0, u64: PmccntrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(pmccntsvr_el1: s2_0_c14_c11_7, u64: PmccntsvrEl1, safe, fake::SYSREGS);
read_sysreg!(pmceid0, u32: Pmceid0, safe, fake::SYSREGS);
read_sysreg!(pmceid0_el0: s3_3_c9_c12_6, u64: Pmceid0El0, safe, fake::SYSREGS);
read_sysreg!(pmceid1, u32: Pmceid1, safe, fake::SYSREGS);
read_sysreg!(pmceid1_el0: s3_3_c9_c12_7, u64: Pmceid1El0, safe, fake::SYSREGS);
read_sysreg!(pmceid2, u32: Pmceid2, safe, fake::SYSREGS);
read_sysreg!(pmceid3, u32: Pmceid3, safe, fake::SYSREGS);
read_write_sysreg!(pmcntenclr, u32: Pmcntenclr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmcntenclr_el0: s3_3_c9_c12_2, u64: PmcntenclrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(pmcntenset, u32: Pmcntenset, safe_read, fake::SYSREGS);
read_write_sysreg!(pmcntenset_el0: s3_3_c9_c12_1, u64: PmcntensetEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(pmcr, u32: Pmcr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmcr_el0, u64: PmcrEl0, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmecr_el1: s3_0_c9_c14_5, u64: PmecrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmiar_el1: s3_0_c9_c14_7, u64: PmiarEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(pmicfiltr_el0: s3_3_c9_c6_0, u64: PmicfiltrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(pmicntr_el0: s3_3_c9_c4_0, u64: PmicntrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(pmicntsvr_el1: s2_0_c14_c12_0, u64: PmicntsvrEl1, safe, fake::SYSREGS);
read_write_sysreg!(pmintenclr, u32: Pmintenclr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmintenclr_el1: s3_0_c9_c14_2, u64: PmintenclrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(pmintenset, u32: Pmintenset, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmintenset_el1: s3_0_c9_c14_1, u64: PmintensetEl1, safe_read, fake::SYSREGS);
read_sysreg!(pmmir, u32: Pmmir, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(pmmir_el1: s3_0_c9_c14_6, u64: PmmirEl1, safe, fake::SYSREGS);
read_write_sysreg!(pmovsclr_el0: s3_3_c9_c12_3, u64: PmovsclrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(pmovsr, u32: Pmovsr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmovsset, u32: Pmovsset, safe_read, fake::SYSREGS);
read_write_sysreg!(pmovsset_el0: s3_3_c9_c14_3, u64: PmovssetEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmscr_el1: s3_0_c9_c9_0, u64: PmscrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(pmscr_el2: s3_4_c9_c9_0, u64: PmscrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmsdsfr_el1: s3_0_c9_c10_4, u64: PmsdsfrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(pmselr, u32: Pmselr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmselr_el0: s3_3_c9_c12_5, u64: PmselrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmsevfr_el1: s3_0_c9_c9_5, u64: PmsevfrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmsfcr_el1: s3_0_c9_c9_4, u64: PmsfcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmsicr_el1: s3_0_c9_c9_2, u64: PmsicrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(pmsidr_el1: s3_0_c9_c9_7, u64: PmsidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmsirr_el1: s3_0_c9_c9_3, u64: PmsirrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmslatfr_el1: s3_0_c9_c9_6, u64: PmslatfrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmsnevfr_el1: s3_0_c9_c9_1, u64: PmsnevfrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmsscr_el1: s3_0_c9_c13_3, u64: PmsscrEl1, safe_read, fake::SYSREGS);
write_sysreg!(pmswinc, u32: Pmswinc, fake::SYSREGS);
write_sysreg!(pmswinc_el0: s3_3_c9_c12_4, u64: PmswincEl0, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(pmuacr_el1: s3_0_c9_c14_4, u64: PmuacrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(pmuserenr, u32: Pmuserenr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmuserenr_el0: s3_3_c9_c14_0, u64: PmuserenrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(pmxevcntr, u32: Pmxevcntr, safe_read, fake::SYSREGS);
read_write_sysreg!(pmxevcntr_el0: s3_3_c9_c13_2, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(pmxevtyper, u32: Pmxevtyper, safe_read, fake::SYSREGS);
read_write_sysreg!(pmxevtyper_el0: s3_3_c9_c13_1, u64: PmxevtyperEl0, safe_read, fake::SYSREGS);
write_sysreg!(pmzr_el0: s3_3_c9_c13_4, u64: PmzrEl0, fake::SYSREGS);
read_write_sysreg!(por_el0: s3_3_c10_c2_4, u64: PorEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(por_el1: s3_0_c10_c2_4, u64: PorEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(por_el2: s3_4_c10_c2_4, u64: PorEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(por_el3: s3_6_c10_c2_4, u64: PorEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(prrr, u32: Prrr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(rcwmask_el1: s3_0_c13_c0_6, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(rcwsmask_el1: s3_0_c13_c0_3, u64, safe_read, fake::SYSREGS);
read_sysreg!(revidr, u32, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(revidr_el1: s3_0_c0_c0_6, u64, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(rgsr_el1, u64: RgsrEl1, safe_read, safe_write, fake::SYSREGS);
read_write_sysreg!(rmr, u32: Rmr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(rmr_el1: s3_0_c12_c0_2, u64: RmrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(rmr_el2: s3_4_c12_c0_2, u64: RmrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(rmr_el3: s3_6_c12_c0_2, u64: RmrEl3, safe_read, fake::SYSREGS);
read_sysreg!(rndr: s3_3_c2_c4_0, u64: Rndr, safe, fake::SYSREGS);
read_sysreg!(rndrrs: s3_3_c2_c4_1, u64: Rndrrs, safe, fake::SYSREGS);
read_sysreg!(rvbar, u32: Rvbar, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(rvbar_el1: s3_0_c12_c0_1, u64: RvbarEl1, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_sysreg!(rvbar_el2: s3_4_c12_c0_1, u64: RvbarEl2, safe, fake::SYSREGS);
#[cfg(feature = "el3")]
read_sysreg!(rvbar_el3: s3_6_c12_c0_1, u64: RvbarEl3, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(s2pir_el2: s3_4_c10_c2_5, u64: S2pirEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(s2por_el1: s3_0_c10_c2_5, u64: S2porEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(scr, u32: Scr, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(scr2_el3: s3_6_c1_c2_2, u64: Scr2El3, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(scr_el3, u64: ScrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(sctlr, u32: Sctlr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(sctlr2mask_el1: s3_0_c1_c4_3, u64: Sctlr2maskEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(sctlr2mask_el2: s3_4_c1_c4_3, u64: Sctlr2maskEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(sctlr2_el1: s3_0_c1_c0_3, u64: Sctlr2El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(sctlr2_el2: s3_4_c1_c0_3, u64: Sctlr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(sctlr2_el3: s3_6_c1_c0_3, u64: Sctlr2El3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(sctlrmask_el1: s3_0_c1_c4_0, u64: SctlrmaskEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(sctlrmask_el2: s3_4_c1_c4_0, u64: SctlrmaskEl2, safe_read, fake::SYSREGS);
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
read_write_sysreg!(scxtnum_el0: s3_3_c13_c0_7, u64: ScxtnumEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(scxtnum_el1: s3_0_c13_c0_7, u64: ScxtnumEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(scxtnum_el2: s3_4_c13_c0_7, u64: ScxtnumEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(scxtnum_el3: s3_6_c13_c0_7, u64: ScxtnumEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(sdcr, u32: Sdcr, safe_read, fake::SYSREGS);
read_write_sysreg!(sder, u32: Sder, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(sder32_el2: s3_4_c1_c3_1, u64: Sder32El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(sder32_el3: s3_6_c1_c1_1, u64: Sder32El3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(smcr_el1: s3_0_c1_c2_6, u64: SmcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(smcr_el2: s3_4_c1_c2_6, u64: SmcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(smcr_el3: s3_6_c1_c2_6, u64: SmcrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(smidr_el1: s3_1_c0_c0_6, u64: SmidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(smprimap_el2: s3_4_c1_c2_5, u64: SmprimapEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(smpri_el1: s3_0_c1_c2_4, u64: SmpriEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(spmaccessr_el1: s2_0_c9_c13_3, u64: SpmaccessrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(spmaccessr_el2: s2_4_c9_c13_3, u64: SpmaccessrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(spmaccessr_el3: s2_6_c9_c13_3, u64: SpmaccessrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(spmcfgr_el1: s2_0_c9_c13_7, u64: SpmcfgrEl1, safe, fake::SYSREGS);
read_write_sysreg!(spmcntenclr_el0: s2_3_c9_c12_2, u64: SpmcntenclrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(spmcntenset_el0: s2_3_c9_c12_1, u64: SpmcntensetEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(spmcr_el0: s2_3_c9_c12_0, u64: SpmcrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(spmdevaff_el1: s2_0_c9_c13_6, u64: SpmdevaffEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(spmdevarch_el1: s2_0_c9_c13_5, u64: SpmdevarchEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(spmiidr_el1: s2_0_c9_c13_4, u64: SpmiidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(spmintenclr_el1: s2_0_c9_c14_2, u64: SpmintenclrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(spmintenset_el1: s2_0_c9_c14_1, u64: SpmintensetEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(spmovsclr_el0: s2_3_c9_c12_3, u64: SpmovsclrEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(spmovsset_el0: s2_3_c9_c14_3, u64: SpmovssetEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(spmrootcr_el3: s2_6_c9_c14_7, u64: SpmrootcrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(spmscr_el1: s2_7_c9_c14_7, u64: SpmscrEl1, safe_read, fake::SYSREGS);
read_write_sysreg!(spmselr_el0: s2_3_c9_c12_5, u64: SpmselrEl0, safe_read, fake::SYSREGS);
write_sysreg!(spmzr_el0: s2_3_c9_c12_4, u64: SpmzrEl0, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(spsr_el1, u64: SpsrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(spsr_el2, u64: SpsrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(spsr_el3, u64: SpsrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(spsr_abt: s3_4_c4_c3_1, u64: SpsrAbt, safe_read, fake::SYSREGS);
read_write_sysreg!(spsr_fiq: s3_4_c4_c3_3, u64: SpsrFiq, safe_read, fake::SYSREGS);
read_write_sysreg!(spsr_irq: s3_4_c4_c3_0, u64: SpsrIrq, safe_read, fake::SYSREGS);
read_write_sysreg!(spsr_und: s3_4_c4_c3_2, u64: SpsrUnd, safe_read, fake::SYSREGS);
read_write_sysreg!(spsel: s3_0_c4_c2_0, u64: Spsel, safe_read, fake::SYSREGS);
read_write_sysreg!(sp_el0: s3_0_c4_c1_0, u64: SpEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(sp_el1, u64: SpEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(sp_el2, u64: SpEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(ssbs: s3_3_c4_c2_6, u64: Ssbs, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(stindex_el1: s3_0_c4_c0_2, u64: StindexEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(stindex_el2: s3_4_c4_c0_2, u64: StindexEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(stindex_el3: s3_6_c4_c0_2, u64: StindexEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(svcr: s3_3_c4_c2_2, u64: Svcr, safe_read, fake::SYSREGS);
read_sysreg!(tcmtr, u32, safe, fake::SYSREGS);
read_write_sysreg!(tco: s3_3_c4_c2_7, u64: Tco, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tcr2mask_el1: s3_0_c2_c7_3, u64: Tcr2maskEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tcr2mask_el2: s3_4_c2_c7_3, u64: Tcr2maskEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tcr2_el1, u64: Tcr2El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tcr2_el2, u64: Tcr2El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tcrmask_el1: s3_0_c2_c7_2, u64: TcrmaskEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tcrmask_el2: s3_4_c2_c7_2, u64: TcrmaskEl2, safe_read, fake::SYSREGS);
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
read_write_sysreg!(tfsre0_el1, u64: Tfsre0El1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tfsr_el1, u64: TfsrEl1, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tfsr_el2, u64: TfsrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(tfsr_el3: s3_6_c5_c6_0, u64: TfsrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(tindex_el0: s3_3_c4_c0_3, u64: TindexEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tindex_el1: s3_0_c4_c0_3, u64: TindexEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tindex_el2: s3_4_c4_c0_3, u64: TindexEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_sysreg!(tindex_el3: s3_6_c4_c0_3, u64: TindexEl3, safe, fake::SYSREGS);
write_sysreg!(tlbiall, u32, fake::SYSREGS);
write_sysreg!(tlbiallh, u32, fake::SYSREGS);
write_sysreg!(tlbiallhis, u32, fake::SYSREGS);
write_sysreg!(tlbiallis, u32, fake::SYSREGS);
write_sysreg!(tlbiallnsnh, u32, fake::SYSREGS);
write_sysreg!(tlbiallnsnhis, u32, fake::SYSREGS);
write_sysreg!(tlbiasid, u32: Tlbiasid, fake::SYSREGS);
write_sysreg!(tlbiasidis, u32: Tlbiasidis, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(tlbididr_el1: s3_0_c10_c4_6, u64: TlbididrEl1, safe, fake::SYSREGS);
write_sysreg!(tlbiipas2, u32: Tlbiipas2, fake::SYSREGS);
write_sysreg!(tlbiipas2is, u32: Tlbiipas2is, fake::SYSREGS);
write_sysreg!(tlbiipas2l, u32: Tlbiipas2l, fake::SYSREGS);
write_sysreg!(tlbiipas2lis, u32: Tlbiipas2lis, fake::SYSREGS);
write_sysreg!(tlbimva, u32: Tlbimva, fake::SYSREGS);
write_sysreg!(tlbimvaa, u32: Tlbimvaa, fake::SYSREGS);
write_sysreg!(tlbimvaais, u32: Tlbimvaais, fake::SYSREGS);
write_sysreg!(tlbimvaal, u32: Tlbimvaal, fake::SYSREGS);
write_sysreg!(tlbimvaalis, u32: Tlbimvaalis, fake::SYSREGS);
write_sysreg!(tlbimvah, u32: Tlbimvah, fake::SYSREGS);
write_sysreg!(tlbimvahis, u32: Tlbimvahis, fake::SYSREGS);
write_sysreg!(tlbimvais, u32: Tlbimvais, fake::SYSREGS);
write_sysreg!(tlbimval, u32: Tlbimval, fake::SYSREGS);
write_sysreg!(tlbimvalh, u32: Tlbimvalh, fake::SYSREGS);
write_sysreg!(tlbimvalhis, u32: Tlbimvalhis, fake::SYSREGS);
write_sysreg!(tlbimvalis, u32: Tlbimvalis, fake::SYSREGS);
read_sysreg!(tlbtr, u32: Tlbtr, safe, fake::SYSREGS);
read_write_sysreg!(tpidr2_el0: s3_3_c13_c0_5, u64: Tpidr2El0, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidr3_el0: s3_3_c13_c0_0, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tpidr3_el1: s3_0_c13_c0_0, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tpidr3_el2: s3_4_c13_c0_0, u64, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(tpidr3_el3: s3_6_c13_c0_0, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidrprw, u32: Tpidrprw, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidrro_el0, u64: TpidrroEl0, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidruro, u32: Tpidruro, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidrurw, u32: Tpidrurw, safe_read, fake::SYSREGS);
read_write_sysreg!(tpidr_el0, u64: TpidrEl0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tpidr_el1, u64: TpidrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tpidr_el2, u64: TpidrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(tpidr_el3: s3_6_c13_c0_2, u64: TpidrEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(tpmax0_el0: s3_3_c2_c2_5, u64: Tpmax0El0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tpmax0_el1: s3_0_c2_c2_5, u64: Tpmax0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tpmax0_el2: s3_4_c2_c2_5, u64: Tpmax0El2, safe_read, fake::SYSREGS);
read_write_sysreg!(tpmax1_el0: s3_3_c2_c2_7, u64: Tpmax1El0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tpmax1_el1: s3_0_c2_c2_7, u64: Tpmax1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tpmax1_el2: s3_4_c2_c2_7, u64: Tpmax1El2, safe_read, fake::SYSREGS);
read_write_sysreg!(tpmin0_el0: s3_3_c2_c2_4, u64: Tpmin0El0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tpmin0_el1: s3_0_c2_c2_4, u64: Tpmin0El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tpmin0_el2: s3_4_c2_c2_4, u64: Tpmin0El2, safe_read, fake::SYSREGS);
read_write_sysreg!(tpmin1_el0: s3_3_c2_c2_6, u64: Tpmin1El0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tpmin1_el1: s3_0_c2_c2_6, u64: Tpmin1El1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tpmin1_el2: s3_4_c2_c2_6, u64: Tpmin1El2, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trbbaser_el1: s3_0_c9_c11_2, u64: TrbbaserEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_sysreg!(trbidr_el1: s3_0_c9_c11_7, u64: TrbidrEl1, safe, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trblimitr_el1: s3_0_c9_c11_0, u64: TrblimitrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trbmar_el1: s3_0_c9_c11_4, u64: TrbmarEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trbmpam_el1: s3_0_c9_c11_5, u64: TrbmpamEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trbptr_el1: s3_0_c9_c11_1, u64: TrbptrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trbsr_el1: s3_0_c9_c11_3, u64: TrbsrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(trbsr_el2: s3_4_c9_c11_3, u64: TrbsrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(trbsr_el3: s3_6_c9_c11_3, u64: TrbsrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trbtrg_el1: s3_0_c9_c11_6, u64: TrbtrgEl1, safe_read, fake::SYSREGS);
read_sysreg!(trcauthstatus: s2_1_c7_c14_6, u64: Trcauthstatus, safe, fake::SYSREGS);
read_write_sysreg!(trcauxctlr: s2_1_c0_c6_0, u64, safe_read, fake::SYSREGS);
read_write_sysreg!(trcbbctlr: s2_1_c0_c15_0, u64: Trcbbctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcccctlr: s2_1_c0_c14_0, u64: Trcccctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trccidcctlr0: s2_1_c3_c0_2, u64: Trccidcctlr0, safe_read, fake::SYSREGS);
read_write_sysreg!(trccidcctlr1: s2_1_c3_c1_2, u64: Trccidcctlr1, safe_read, fake::SYSREGS);
read_write_sysreg!(trcclaimclr: s2_1_c7_c9_6, u64: Trcclaimclr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcclaimset: s2_1_c7_c8_6, u64: Trcclaimset, safe_read, fake::SYSREGS);
read_write_sysreg!(trcconfigr: s2_1_c0_c4_0, u64: Trcconfigr, safe_read, fake::SYSREGS);
read_sysreg!(trcdevarch: s2_1_c7_c15_6, u64: Trcdevarch, safe, fake::SYSREGS);
read_sysreg!(trcdevid: s2_1_c7_c2_7, u64, safe, fake::SYSREGS);
read_write_sysreg!(trceventctl0r: s2_1_c0_c8_0, u64: Trceventctl0r, safe_read, fake::SYSREGS);
read_write_sysreg!(trceventctl1r: s2_1_c0_c9_0, u64: Trceventctl1r, safe_read, fake::SYSREGS);
read_sysreg!(trcidr0: s2_1_c0_c8_7, u64: Trcidr0, safe, fake::SYSREGS);
read_sysreg!(trcidr1: s2_1_c0_c9_7, u64: Trcidr1, safe, fake::SYSREGS);
read_sysreg!(trcidr10: s2_1_c0_c2_6, u64: Trcidr10, safe, fake::SYSREGS);
read_sysreg!(trcidr11: s2_1_c0_c3_6, u64: Trcidr11, safe, fake::SYSREGS);
read_sysreg!(trcidr12: s2_1_c0_c4_6, u64: Trcidr12, safe, fake::SYSREGS);
read_sysreg!(trcidr13: s2_1_c0_c5_6, u64: Trcidr13, safe, fake::SYSREGS);
read_sysreg!(trcidr2: s2_1_c0_c10_7, u64: Trcidr2, safe, fake::SYSREGS);
read_sysreg!(trcidr3: s2_1_c0_c11_7, u64: Trcidr3, safe, fake::SYSREGS);
read_sysreg!(trcidr4: s2_1_c0_c12_7, u64: Trcidr4, safe, fake::SYSREGS);
read_sysreg!(trcidr5: s2_1_c0_c13_7, u64: Trcidr5, safe, fake::SYSREGS);
read_sysreg!(trcidr6: s2_1_c0_c14_7, u64: Trcidr6, safe, fake::SYSREGS);
read_sysreg!(trcidr7: s2_1_c0_c15_7, u64, safe, fake::SYSREGS);
read_sysreg!(trcidr8: s2_1_c0_c0_6, u64: Trcidr8, safe, fake::SYSREGS);
read_sysreg!(trcidr9: s2_1_c0_c1_6, u64: Trcidr9, safe, fake::SYSREGS);
read_write_sysreg!(trcimspec0: s2_1_c0_c0_7, u64: Trcimspec0, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trcitecr_el1: s3_0_c1_c2_3, u64: TrcitecrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(trcitecr_el2: s3_4_c1_c2_3, u64: TrcitecrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(trciteedcr: s2_1_c0_c2_1, u64: Trciteedcr, safe_read, fake::SYSREGS);
read_sysreg!(trcoslsr: s2_1_c1_c1_4, u64: Trcoslsr, safe, fake::SYSREGS);
read_write_sysreg!(trcprgctlr: s2_1_c0_c1_0, u64: Trcprgctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcqctlr: s2_1_c0_c1_1, u64: Trcqctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcrsr: s2_1_c0_c10_0, u64: Trcrsr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcseqrstevr: s2_1_c0_c6_4, u64: Trcseqrstevr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcseqstr: s2_1_c0_c7_4, u64: Trcseqstr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcstallctlr: s2_1_c0_c11_0, u64: Trcstallctlr, safe_read, fake::SYSREGS);
read_sysreg!(trcstatr: s2_1_c0_c3_0, u64: Trcstatr, safe, fake::SYSREGS);
read_write_sysreg!(trcsyncpr: s2_1_c0_c13_0, u64: Trcsyncpr, safe_read, fake::SYSREGS);
read_write_sysreg!(trctraceidr: s2_1_c0_c0_1, u64: Trctraceidr, safe_read, fake::SYSREGS);
read_write_sysreg!(trctsctlr: s2_1_c0_c12_0, u64: Trctsctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcvictlr: s2_1_c0_c0_2, u64: Trcvictlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcviiectlr: s2_1_c0_c1_2, u64: Trcviiectlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcvipcssctlr: s2_1_c0_c3_2, u64: Trcvipcssctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcvissctlr: s2_1_c0_c2_2, u64: Trcvissctlr, safe_read, fake::SYSREGS);
read_write_sysreg!(trcvmidcctlr0: s2_1_c3_c2_2, u64: Trcvmidcctlr0, safe_read, fake::SYSREGS);
read_write_sysreg!(trcvmidcctlr1: s2_1_c3_c3_2, u64: Trcvmidcctlr1, safe_read, fake::SYSREGS);
read_write_sysreg!(trfcr, u32: Trfcr, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(trfcr_el1: s3_0_c1_c2_1, u64: TrfcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(trfcr_el2: s3_4_c1_c2_1, u64: TrfcrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(ttbcr, u32: Ttbcr, safe_read, fake::SYSREGS);
read_write_sysreg!(ttbcr2, u32: Ttbcr2, safe_read, fake::SYSREGS);
read_write_sysreg!(ttbr0, u32: Ttbr0, safe_read, fake::SYSREGS);
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
read_write_sysreg!(ttbr1, u32: Ttbr1, safe_read, fake::SYSREGS);
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
read_write_sysreg!(tttbrp_el1: s3_0_c10_c2_7, u64: TttbrpEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tttbrp_el2: s3_4_c10_c2_7, u64: TttbrpEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(tttbrp_el3: s3_6_c10_c2_7, u64: TttbrpEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el1")]
read_write_sysreg!(tttbru_el1: s3_0_c10_c2_6, u64: TttbruEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(tttbru_el2: s3_4_c10_c2_6, u64: TttbruEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(uao: s3_0_c4_c2_4, u64: Uao, safe_read, fake::SYSREGS);
read_write_sysreg!(vbar, u32: Vbar, safe_read, fake::SYSREGS);
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
#[cfg(feature = "el3")]
read_write_sysreg!(vbar_el3: s3_6_c12_c0_0, u64: VbarEl3, safe_read, fake::SYSREGS);
read_write_sysreg!(vdfsr, u32: Vdfsr, safe_read, fake::SYSREGS);
read_write_sysreg!(vdisr, u32: Vdisr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vdisr_el2, u64: VdisrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(vdisr_el3: s3_6_c12_c1_1, u64: VdisrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vmecid_a_el2: s3_4_c10_c9_1, u64: VmecidAEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vmecid_p_el2: s3_4_c10_c9_0, u64: VmecidPEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(vmpidr, u32: Vmpidr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vmpidr_el2, u64: VmpidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vnccr_el2: s3_4_c2_c2_1, u64: VnccrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vncr_el2: s3_4_c2_c2_0, u64: VncrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(vpidr, u32: Vpidr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vpidr_el2, u64: VpidrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vsesr_el2, u64: VsesrEl2, safe_read, safe_write, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(vsesr_el3: s3_6_c5_c2_3, u64: VsesrEl3, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vstcr_el2: s3_4_c2_c6_2, u64: VstcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vsttbr_el2: s3_4_c2_c6_0, u64: VsttbrEl2, safe_read, fake::SYSREGS);
read_write_sysreg!(vtcr, u32: Vtcr, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(vtcr_el2, u64: VtcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned stage 2 translation table.
    vttbr_el2, u64: VttbrEl2, safe_read, fake::SYSREGS
}
#[cfg(feature = "el1")]
read_write_sysreg!(zcr_el1: s3_0_c1_c2_0, u64: ZcrEl1, safe_read, fake::SYSREGS);
#[cfg(feature = "el2")]
read_write_sysreg!(zcr_el2: s3_4_c1_c2_0, u64: ZcrEl2, safe_read, fake::SYSREGS);
#[cfg(feature = "el3")]
read_write_sysreg!(zcr_el3, u64: ZcrEl3, safe_read, fake::SYSREGS);
