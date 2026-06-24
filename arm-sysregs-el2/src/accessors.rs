// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

// This file is generated, do not edit manually.

// `unused_imports` is allowed because it is possible that not all of these macros are used in the
// generated output.
#[allow(unused_imports)]
use arm_sysregs_common::{read_sysreg, read_write_sysreg, write_sysreg};

read_write_sysreg!(actlr_el2, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(afsr0_el2, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(afsr1_el2, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(amair_el2, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff00_el2: s3_4_c13_c8_0, u64: crate::registers::Amevcntvoff00El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff010_el2: s3_4_c13_c9_2, u64: crate::registers::Amevcntvoff010El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff011_el2: s3_4_c13_c9_3, u64: crate::registers::Amevcntvoff011El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff012_el2: s3_4_c13_c9_4, u64: crate::registers::Amevcntvoff012El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff013_el2: s3_4_c13_c9_5, u64: crate::registers::Amevcntvoff013El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff014_el2: s3_4_c13_c9_6, u64: crate::registers::Amevcntvoff014El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff015_el2: s3_4_c13_c9_7, u64: crate::registers::Amevcntvoff015El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff01_el2: s3_4_c13_c8_1, u64: crate::registers::Amevcntvoff01El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff02_el2: s3_4_c13_c8_2, u64: crate::registers::Amevcntvoff02El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff03_el2: s3_4_c13_c8_3, u64: crate::registers::Amevcntvoff03El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff04_el2: s3_4_c13_c8_4, u64: crate::registers::Amevcntvoff04El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff05_el2: s3_4_c13_c8_5, u64: crate::registers::Amevcntvoff05El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff06_el2: s3_4_c13_c8_6, u64: crate::registers::Amevcntvoff06El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff07_el2: s3_4_c13_c8_7, u64: crate::registers::Amevcntvoff07El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff08_el2: s3_4_c13_c9_0, u64: crate::registers::Amevcntvoff08El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff09_el2: s3_4_c13_c9_1, u64: crate::registers::Amevcntvoff09El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff10_el2: s3_4_c13_c10_0, u64: crate::registers::Amevcntvoff10El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff110_el2: s3_4_c13_c11_2, u64: crate::registers::Amevcntvoff110El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff111_el2: s3_4_c13_c11_3, u64: crate::registers::Amevcntvoff111El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff112_el2: s3_4_c13_c11_4, u64: crate::registers::Amevcntvoff112El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff113_el2: s3_4_c13_c11_5, u64: crate::registers::Amevcntvoff113El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff114_el2: s3_4_c13_c11_6, u64: crate::registers::Amevcntvoff114El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff115_el2: s3_4_c13_c11_7, u64: crate::registers::Amevcntvoff115El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff11_el2: s3_4_c13_c10_1, u64: crate::registers::Amevcntvoff11El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff12_el2: s3_4_c13_c10_2, u64: crate::registers::Amevcntvoff12El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff13_el2: s3_4_c13_c10_3, u64: crate::registers::Amevcntvoff13El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff14_el2: s3_4_c13_c10_4, u64: crate::registers::Amevcntvoff14El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff15_el2: s3_4_c13_c10_5, u64: crate::registers::Amevcntvoff15El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff16_el2: s3_4_c13_c10_6, u64: crate::registers::Amevcntvoff16El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff17_el2: s3_4_c13_c10_7, u64: crate::registers::Amevcntvoff17El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff18_el2: s3_4_c13_c11_0, u64: crate::registers::Amevcntvoff18El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(amevcntvoff19_el2: s3_4_c13_c11_1, u64: crate::registers::Amevcntvoff19El2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(brbcr_el2: s2_1_c9_c0_0, u64: crate::registers::BrbcrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(cnthctl_el2, u64: crate::registers::CnthctlEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthps_ctl_el2: s3_4_c14_c5_1, u64: crate::registers::CnthpsCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthps_cval_el2: s3_4_c14_c5_2, u64: crate::registers::CnthpsCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthps_tval_el2: s3_4_c14_c5_0, u64: crate::registers::CnthpsTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthp_ctl_el2: s3_4_c14_c2_1, u64: crate::registers::CnthpCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthp_cval_el2: s3_4_c14_c2_2, u64: crate::registers::CnthpCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthp_tval_el2: s3_4_c14_c2_0, u64: crate::registers::CnthpTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthvs_ctl_el2: s3_4_c14_c4_1, u64: crate::registers::CnthvsCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthvs_cval_el2: s3_4_c14_c4_2, u64: crate::registers::CnthvsCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthvs_tval_el2: s3_4_c14_c4_0, u64: crate::registers::CnthvsTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthv_ctl_el2: s3_4_c14_c3_1, u64: crate::registers::CnthvCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthv_cval_el2: s3_4_c14_c3_2, u64: crate::registers::CnthvCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cnthv_tval_el2: s3_4_c14_c3_0, u64: crate::registers::CnthvTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntpoff_el2: s3_4_c14_c0_6, u64: crate::registers::CntpoffEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cntvoff_el2, u64: crate::registers::CntvoffEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(contextidr_el2: s3_4_c13_c0_1, u64: crate::registers::ContextidrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(cptr_el2, u64: crate::registers::CptrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(elr_el2, u64: crate::registers::ElrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(esr_el2, u64: crate::registers::EsrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(far_el2, u64: crate::registers::FarEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gcscr_el2: s3_4_c2_c5_0, u64: crate::registers::GcscrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(gcspr_el2: s3_4_c2_c5_1, u64: crate::registers::GcsprEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hacr_el2, u64, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hafgrtr_el2: s3_4_c3_c1_6, u64: crate::registers::HafgrtrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hcrx_el2: s3_4_c1_c2_2, u64: crate::registers::HcrxEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hcr_el2, u64: crate::registers::HcrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hdfgrtr2_el2: s3_4_c3_c1_0, u64: crate::registers::Hdfgrtr2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hdfgrtr_el2: s3_4_c3_c1_4, u64: crate::registers::HdfgrtrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hdfgwtr2_el2: s3_4_c3_c1_1, u64: crate::registers::Hdfgwtr2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hdfgwtr_el2: s3_4_c3_c1_5, u64: crate::registers::HdfgwtrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hfgitr2_el2: s3_4_c3_c1_7, u64: crate::registers::Hfgitr2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hfgitr_el2: s3_4_c1_c1_6, u64: crate::registers::HfgitrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hfgrtr2_el2: s3_4_c3_c1_2, u64: crate::registers::Hfgrtr2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hfgrtr_el2: s3_4_c1_c1_4, u64: crate::registers::HfgrtrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hfgwtr2_el2: s3_4_c3_c1_3, u64: crate::registers::Hfgwtr2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hfgwtr_el2: s3_4_c1_c1_5, u64: crate::registers::HfgwtrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hpfar_el2, u64: crate::registers::HpfarEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(hstr_el2, u64, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(icc_sre_el2: s3_4_c12_c9_5, u64: crate::registers::IccSreEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(ich_hcr_el2: s3_4_c12_c11_0, u64: crate::registers::IchHcrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(ich_vmcr_el2: s3_4_c12_c11_7, u64: crate::registers::IchVmcrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(mair_el2, u64: crate::registers::MairEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mdcr_el2, u64: crate::registers::MdcrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(mpam2_el2: s3_4_c10_c5_0, u64: crate::registers::Mpam2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamhcr_el2: s3_4_c10_c4_0, u64: crate::registers::MpamhcrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm0_el2: s3_4_c10_c6_0, u64: crate::registers::Mpamvpm0El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm1_el2: s3_4_c10_c6_1, u64: crate::registers::Mpamvpm1El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm2_el2: s3_4_c10_c6_2, u64: crate::registers::Mpamvpm2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm3_el2: s3_4_c10_c6_3, u64: crate::registers::Mpamvpm3El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm4_el2: s3_4_c10_c6_4, u64: crate::registers::Mpamvpm4El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm5_el2: s3_4_c10_c6_5, u64: crate::registers::Mpamvpm5El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm6_el2: s3_4_c10_c6_6, u64: crate::registers::Mpamvpm6El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpm7_el2: s3_4_c10_c6_7, u64: crate::registers::Mpamvpm7El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(mpamvpmv_el2: s3_4_c10_c4_1, u64: crate::registers::MpamvpmvEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pfar_el2: s3_4_c6_c0_5, u64: crate::registers::PfarEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pire0_el2: s3_4_c10_c2_2, u64: crate::registers::Pire0El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(pir_el2: s3_4_c10_c2_3, u64: crate::registers::PirEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(por_el2: s3_4_c10_c2_4, u64: crate::registers::PorEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(s2pir_el2: s3_4_c10_c2_5, u64: crate::registers::S2pirEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(sctlr2_el2: s3_4_c1_c0_3, u64: crate::registers::Sctlr2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(sctlr_el2, u64: crate::registers::SctlrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(spsr_el2, u64: crate::registers::SpsrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(sp_el2, u64: crate::registers::SpEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tcr2_el2: s3_4_c2_c0_3, u64: crate::registers::Tcr2El2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tcr_el2, u64: crate::registers::TcrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg!(tfsr_el2: s3_4_c5_c6_0, u64: crate::registers::TfsrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(tpidr_el2, u64: crate::registers::TpidrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el2, u64: crate::registers::Ttbr0El2, safe_read, crate::fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el2: s3_4_c2_c0_1, u64: crate::registers::Ttbr1El2, safe_read, crate::fake::SYSREGS
}
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el2, u64: crate::registers::VbarEl2, safe_read, crate::fake::SYSREGS
}
read_write_sysreg!(vdisr_el2: s3_4_c12_c1_1, u64: crate::registers::VdisrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(vmpidr_el2, u64: crate::registers::VmpidrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(vpidr_el2, u64: crate::registers::VpidrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(vsesr_el2: s3_4_c5_c2_3, u64: crate::registers::VsesrEl2, safe_read, safe_write, crate::fake::SYSREGS);
read_write_sysreg!(vtcr_el2, u64: crate::registers::VtcrEl2, safe_read, crate::fake::SYSREGS);
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned stage 2 translation table.
    vttbr_el2, u64: crate::registers::VttbrEl2, safe_read, crate::fake::SYSREGS
}
