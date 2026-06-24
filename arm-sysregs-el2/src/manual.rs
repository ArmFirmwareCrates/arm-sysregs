// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Manually implemented methods for EL2 system register types.

use crate::registers::{EsrEl2, SpsrEl2};
use core::fmt::{self, Debug, Formatter};

impl EsrEl2 {
    /// Mask for the parts of an ESR value containing the opcode.
    pub const ISS_SYSREG_OPCODE_MASK: Self = Self::from_bits_retain(0x003f_fc1e);
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
