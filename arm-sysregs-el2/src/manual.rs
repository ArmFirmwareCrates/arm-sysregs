// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Manually implemented methods for EL2 system register types.

use arm_sysregs_common::types::EsrIssSysreg;

use crate::registers::{EsrEl2, SpsrEl2};
use core::fmt::{self, Debug, Formatter};

impl EsrEl2 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);

    /// Returns the value of the `ISS` field, as interpreted in the case of an exception from MSR,
    /// MRS, or System instruction execution in AArch64 state.
    pub const fn iss_sysreg(self) -> Option<EsrIssSysreg> {
        if self.ec() == 0b01_1000 {
            Some(EsrIssSysreg::from_bits_retain(self.iss()))
        } else {
            None
        }
    }
}

impl Debug for EsrEl2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "EsrEl2({:#x})", self.bits())
    }
}

impl SpsrEl2 {
    /// All of the N, Z, C and V bits.
    pub const NZCV: Self = Self::V.union(Self::C).union(Self::Z).union(Self::N);
}
