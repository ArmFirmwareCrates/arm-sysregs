// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

// This file is generated, do not edit manually.

// `unused_imports` is allowed because it is possible that not all of these macros are used in the
// generated output.
#[allow(unused_imports)]
use arm_sysregs_common::{read_sysreg, read_write_sysreg, write_sysreg};

read_write_sysreg!(cptr_el3, u64: crate::registers::CptrEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(esr_el3, u64: crate::registers::EsrEl3, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(fgwte3_el3: s3_6_c1_c1_5, u64: crate::registers::Fgwte3El3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gpccr_el3: s3_6_c2_c1_6, u64: crate::registers::GpccrEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gptbr_el3: s3_6_c2_c1_4, u64: crate::registers::GptbrEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(icc_ctlr_el3: s3_6_c12_c12_4, u64: crate::registers::IccCtlrEl3, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(icc_igrpen1_el3: s3_6_c12_c12_7, u64: crate::registers::IccIgrpen1El3, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The SRE bit of `icc_sre_el3` must not be changed from 1 to 0, as this can result in unpredictable behaviour.
    icc_sre_el3: s3_6_c12_c12_5, u64: crate::registers::IccSreEl3, safe_read, crate::fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 memory attribute indirection register.
    mair_el3, u64: crate::registers::MairEl3, safe_read, crate::fake::SYSREGS
}
read_write_sysreg!(mdcr_el3, u64: crate::registers::MdcrEl3, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(mpam3_el3: s3_6_c10_c5_0, u64: crate::registers::Mpam3El3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pir_el3: s3_6_c10_c2_3, u64: crate::registers::PirEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(por_el3: s3_6_c10_c2_4, u64: crate::registers::PorEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(scr_el3, u64: crate::registers::ScrEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(sctlr2_el3: s3_6_c1_c0_3, u64: crate::registers::Sctlr2El3, safe_read, crate::fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 system control register.
    sctlr_el3, u64: crate::registers::SctlrEl3, safe_read, crate::fake::SYSREGS
}
read_write_sysreg!(smcr_el3: s3_6_c1_c2_6, u64: crate::registers::SmcrEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(spsr_el3, u64: crate::registers::SpsrEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 translation control register.
    tcr_el3, u64: crate::registers::TcrEl3, safe_read, crate::fake::SYSREGS
}
read_write_sysreg!(tpidr_el3, u64: crate::registers::TpidrEl3, safe_read, crate::fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el3, u64: crate::registers::Ttbr0El3, safe_read, crate::fake::SYSREGS
}
read_write_sysreg!(zcr_el3: s3_6_c1_c2_0, u64: crate::registers::ZcrEl3, safe_read, crate::fake::SYSREGS);
