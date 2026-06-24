// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

// This file is generated, do not edit manually.

use crate::registers::{
    Amevcntvoff00El2, Amevcntvoff01El2, Amevcntvoff02El2, Amevcntvoff03El2, Amevcntvoff04El2,
    Amevcntvoff05El2, Amevcntvoff06El2, Amevcntvoff07El2, Amevcntvoff08El2, Amevcntvoff09El2,
    Amevcntvoff010El2, Amevcntvoff10El2, Amevcntvoff011El2, Amevcntvoff11El2, Amevcntvoff012El2,
    Amevcntvoff12El2, Amevcntvoff013El2, Amevcntvoff13El2, Amevcntvoff014El2, Amevcntvoff14El2,
    Amevcntvoff015El2, Amevcntvoff15El2, Amevcntvoff16El2, Amevcntvoff17El2, Amevcntvoff18El2,
    Amevcntvoff19El2, Amevcntvoff110El2, Amevcntvoff111El2, Amevcntvoff112El2, Amevcntvoff113El2,
    Amevcntvoff114El2, Amevcntvoff115El2, BrbcrEl2, CnthctlEl2, CnthpCtlEl2, CnthpCvalEl2,
    CnthpTvalEl2, CnthpsCtlEl2, CnthpsCvalEl2, CnthpsTvalEl2, CnthvCtlEl2, CnthvCvalEl2,
    CnthvTvalEl2, CnthvsCtlEl2, CnthvsCvalEl2, CnthvsTvalEl2, CntpoffEl2, CntvoffEl2,
    ContextidrEl2, CptrEl2, ElrEl2, EsrEl2, FarEl2, GcscrEl2, GcsprEl2, HafgrtrEl2, HcrEl2,
    HcrxEl2, Hdfgrtr2El2, HdfgrtrEl2, Hdfgwtr2El2, HdfgwtrEl2, Hfgitr2El2, HfgitrEl2, Hfgrtr2El2,
    HfgrtrEl2, Hfgwtr2El2, HfgwtrEl2, HpfarEl2, IccSreEl2, IchHcrEl2, IchVmcrEl2, MairEl2, MdcrEl2,
    Mpam2El2, MpamhcrEl2, Mpamvpm0El2, Mpamvpm1El2, Mpamvpm2El2, Mpamvpm3El2, Mpamvpm4El2,
    Mpamvpm5El2, Mpamvpm6El2, Mpamvpm7El2, MpamvpmvEl2, PfarEl2, PirEl2, Pire0El2, PorEl2,
    S2pirEl2, Sctlr2El2, SctlrEl2, SpEl2, SpsrEl2, Tcr2El2, TcrEl2, TfsrEl2, TpidrEl2, Ttbr0El2,
    Ttbr1El2, VbarEl2, VdisrEl2, VmpidrEl2, VpidrEl2, VsesrEl2, VtcrEl2, VttbrEl2,
};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SystemRegisters {
    /// Fake value for the `ACTLR_EL2` system register.
    pub actlr_el2: u64,
    /// Fake value for the `AFSR0_EL2` system register.
    pub afsr0_el2: u64,
    /// Fake value for the `AFSR1_EL2` system register.
    pub afsr1_el2: u64,
    /// Fake value for the `AMAIR_EL2` system register.
    pub amair_el2: u64,
    /// Fake value for the `AMEVCNTVOFF00_EL2` system register.
    pub amevcntvoff00_el2: Amevcntvoff00El2,
    /// Fake value for the `AMEVCNTVOFF010_EL2` system register.
    pub amevcntvoff010_el2: Amevcntvoff010El2,
    /// Fake value for the `AMEVCNTVOFF011_EL2` system register.
    pub amevcntvoff011_el2: Amevcntvoff011El2,
    /// Fake value for the `AMEVCNTVOFF012_EL2` system register.
    pub amevcntvoff012_el2: Amevcntvoff012El2,
    /// Fake value for the `AMEVCNTVOFF013_EL2` system register.
    pub amevcntvoff013_el2: Amevcntvoff013El2,
    /// Fake value for the `AMEVCNTVOFF014_EL2` system register.
    pub amevcntvoff014_el2: Amevcntvoff014El2,
    /// Fake value for the `AMEVCNTVOFF015_EL2` system register.
    pub amevcntvoff015_el2: Amevcntvoff015El2,
    /// Fake value for the `AMEVCNTVOFF01_EL2` system register.
    pub amevcntvoff01_el2: Amevcntvoff01El2,
    /// Fake value for the `AMEVCNTVOFF02_EL2` system register.
    pub amevcntvoff02_el2: Amevcntvoff02El2,
    /// Fake value for the `AMEVCNTVOFF03_EL2` system register.
    pub amevcntvoff03_el2: Amevcntvoff03El2,
    /// Fake value for the `AMEVCNTVOFF04_EL2` system register.
    pub amevcntvoff04_el2: Amevcntvoff04El2,
    /// Fake value for the `AMEVCNTVOFF05_EL2` system register.
    pub amevcntvoff05_el2: Amevcntvoff05El2,
    /// Fake value for the `AMEVCNTVOFF06_EL2` system register.
    pub amevcntvoff06_el2: Amevcntvoff06El2,
    /// Fake value for the `AMEVCNTVOFF07_EL2` system register.
    pub amevcntvoff07_el2: Amevcntvoff07El2,
    /// Fake value for the `AMEVCNTVOFF08_EL2` system register.
    pub amevcntvoff08_el2: Amevcntvoff08El2,
    /// Fake value for the `AMEVCNTVOFF09_EL2` system register.
    pub amevcntvoff09_el2: Amevcntvoff09El2,
    /// Fake value for the `AMEVCNTVOFF10_EL2` system register.
    pub amevcntvoff10_el2: Amevcntvoff10El2,
    /// Fake value for the `AMEVCNTVOFF110_EL2` system register.
    pub amevcntvoff110_el2: Amevcntvoff110El2,
    /// Fake value for the `AMEVCNTVOFF111_EL2` system register.
    pub amevcntvoff111_el2: Amevcntvoff111El2,
    /// Fake value for the `AMEVCNTVOFF112_EL2` system register.
    pub amevcntvoff112_el2: Amevcntvoff112El2,
    /// Fake value for the `AMEVCNTVOFF113_EL2` system register.
    pub amevcntvoff113_el2: Amevcntvoff113El2,
    /// Fake value for the `AMEVCNTVOFF114_EL2` system register.
    pub amevcntvoff114_el2: Amevcntvoff114El2,
    /// Fake value for the `AMEVCNTVOFF115_EL2` system register.
    pub amevcntvoff115_el2: Amevcntvoff115El2,
    /// Fake value for the `AMEVCNTVOFF11_EL2` system register.
    pub amevcntvoff11_el2: Amevcntvoff11El2,
    /// Fake value for the `AMEVCNTVOFF12_EL2` system register.
    pub amevcntvoff12_el2: Amevcntvoff12El2,
    /// Fake value for the `AMEVCNTVOFF13_EL2` system register.
    pub amevcntvoff13_el2: Amevcntvoff13El2,
    /// Fake value for the `AMEVCNTVOFF14_EL2` system register.
    pub amevcntvoff14_el2: Amevcntvoff14El2,
    /// Fake value for the `AMEVCNTVOFF15_EL2` system register.
    pub amevcntvoff15_el2: Amevcntvoff15El2,
    /// Fake value for the `AMEVCNTVOFF16_EL2` system register.
    pub amevcntvoff16_el2: Amevcntvoff16El2,
    /// Fake value for the `AMEVCNTVOFF17_EL2` system register.
    pub amevcntvoff17_el2: Amevcntvoff17El2,
    /// Fake value for the `AMEVCNTVOFF18_EL2` system register.
    pub amevcntvoff18_el2: Amevcntvoff18El2,
    /// Fake value for the `AMEVCNTVOFF19_EL2` system register.
    pub amevcntvoff19_el2: Amevcntvoff19El2,
    /// Fake value for the `BRBCR_EL2` system register.
    pub brbcr_el2: BrbcrEl2,
    /// Fake value for the `CNTHCTL_EL2` system register.
    pub cnthctl_el2: CnthctlEl2,
    /// Fake value for the `CNTHPS_CTL_EL2` system register.
    pub cnthps_ctl_el2: CnthpsCtlEl2,
    /// Fake value for the `CNTHPS_CVAL_EL2` system register.
    pub cnthps_cval_el2: CnthpsCvalEl2,
    /// Fake value for the `CNTHPS_TVAL_EL2` system register.
    pub cnthps_tval_el2: CnthpsTvalEl2,
    /// Fake value for the `CNTHP_CTL_EL2` system register.
    pub cnthp_ctl_el2: CnthpCtlEl2,
    /// Fake value for the `CNTHP_CVAL_EL2` system register.
    pub cnthp_cval_el2: CnthpCvalEl2,
    /// Fake value for the `CNTHP_TVAL_EL2` system register.
    pub cnthp_tval_el2: CnthpTvalEl2,
    /// Fake value for the `CNTHVS_CTL_EL2` system register.
    pub cnthvs_ctl_el2: CnthvsCtlEl2,
    /// Fake value for the `CNTHVS_CVAL_EL2` system register.
    pub cnthvs_cval_el2: CnthvsCvalEl2,
    /// Fake value for the `CNTHVS_TVAL_EL2` system register.
    pub cnthvs_tval_el2: CnthvsTvalEl2,
    /// Fake value for the `CNTHV_CTL_EL2` system register.
    pub cnthv_ctl_el2: CnthvCtlEl2,
    /// Fake value for the `CNTHV_CVAL_EL2` system register.
    pub cnthv_cval_el2: CnthvCvalEl2,
    /// Fake value for the `CNTHV_TVAL_EL2` system register.
    pub cnthv_tval_el2: CnthvTvalEl2,
    /// Fake value for the `CNTPOFF_EL2` system register.
    pub cntpoff_el2: CntpoffEl2,
    /// Fake value for the `CNTVOFF_EL2` system register.
    pub cntvoff_el2: CntvoffEl2,
    /// Fake value for the `CONTEXTIDR_EL2` system register.
    pub contextidr_el2: ContextidrEl2,
    /// Fake value for the `CPTR_EL2` system register.
    pub cptr_el2: CptrEl2,
    /// Fake value for the `ELR_EL2` system register.
    pub elr_el2: ElrEl2,
    /// Fake value for the `ESR_EL2` system register.
    pub esr_el2: EsrEl2,
    /// Fake value for the `FAR_EL2` system register.
    pub far_el2: FarEl2,
    /// Fake value for the `GCSCR_EL2` system register.
    pub gcscr_el2: GcscrEl2,
    /// Fake value for the `GCSPR_EL2` system register.
    pub gcspr_el2: GcsprEl2,
    /// Fake value for the `HACR_EL2` system register.
    pub hacr_el2: u64,
    /// Fake value for the `HAFGRTR_EL2` system register.
    pub hafgrtr_el2: HafgrtrEl2,
    /// Fake value for the `HCRX_EL2` system register.
    pub hcrx_el2: HcrxEl2,
    /// Fake value for the `HCR_EL2` system register.
    pub hcr_el2: HcrEl2,
    /// Fake value for the `HDFGRTR2_EL2` system register.
    pub hdfgrtr2_el2: Hdfgrtr2El2,
    /// Fake value for the `HDFGRTR_EL2` system register.
    pub hdfgrtr_el2: HdfgrtrEl2,
    /// Fake value for the `HDFGWTR2_EL2` system register.
    pub hdfgwtr2_el2: Hdfgwtr2El2,
    /// Fake value for the `HDFGWTR_EL2` system register.
    pub hdfgwtr_el2: HdfgwtrEl2,
    /// Fake value for the `HFGITR2_EL2` system register.
    pub hfgitr2_el2: Hfgitr2El2,
    /// Fake value for the `HFGITR_EL2` system register.
    pub hfgitr_el2: HfgitrEl2,
    /// Fake value for the `HFGRTR2_EL2` system register.
    pub hfgrtr2_el2: Hfgrtr2El2,
    /// Fake value for the `HFGRTR_EL2` system register.
    pub hfgrtr_el2: HfgrtrEl2,
    /// Fake value for the `HFGWTR2_EL2` system register.
    pub hfgwtr2_el2: Hfgwtr2El2,
    /// Fake value for the `HFGWTR_EL2` system register.
    pub hfgwtr_el2: HfgwtrEl2,
    /// Fake value for the `HPFAR_EL2` system register.
    pub hpfar_el2: HpfarEl2,
    /// Fake value for the `HSTR_EL2` system register.
    pub hstr_el2: u64,
    /// Fake value for the `ICC_SRE_EL2` system register.
    pub icc_sre_el2: IccSreEl2,
    /// Fake value for the `ICH_HCR_EL2` system register.
    pub ich_hcr_el2: IchHcrEl2,
    /// Fake value for the `ICH_VMCR_EL2` system register.
    pub ich_vmcr_el2: IchVmcrEl2,
    /// Fake value for the `MAIR_EL2` system register.
    pub mair_el2: MairEl2,
    /// Fake value for the `MDCR_EL2` system register.
    pub mdcr_el2: MdcrEl2,
    /// Fake value for the `MPAM2_EL2` system register.
    pub mpam2_el2: Mpam2El2,
    /// Fake value for the `MPAMHCR_EL2` system register.
    pub mpamhcr_el2: MpamhcrEl2,
    /// Fake value for the `MPAMVPM0_EL2` system register.
    pub mpamvpm0_el2: Mpamvpm0El2,
    /// Fake value for the `MPAMVPM1_EL2` system register.
    pub mpamvpm1_el2: Mpamvpm1El2,
    /// Fake value for the `MPAMVPM2_EL2` system register.
    pub mpamvpm2_el2: Mpamvpm2El2,
    /// Fake value for the `MPAMVPM3_EL2` system register.
    pub mpamvpm3_el2: Mpamvpm3El2,
    /// Fake value for the `MPAMVPM4_EL2` system register.
    pub mpamvpm4_el2: Mpamvpm4El2,
    /// Fake value for the `MPAMVPM5_EL2` system register.
    pub mpamvpm5_el2: Mpamvpm5El2,
    /// Fake value for the `MPAMVPM6_EL2` system register.
    pub mpamvpm6_el2: Mpamvpm6El2,
    /// Fake value for the `MPAMVPM7_EL2` system register.
    pub mpamvpm7_el2: Mpamvpm7El2,
    /// Fake value for the `MPAMVPMV_EL2` system register.
    pub mpamvpmv_el2: MpamvpmvEl2,
    /// Fake value for the `PFAR_EL2` system register.
    pub pfar_el2: PfarEl2,
    /// Fake value for the `PIRE0_EL2` system register.
    pub pire0_el2: Pire0El2,
    /// Fake value for the `PIR_EL2` system register.
    pub pir_el2: PirEl2,
    /// Fake value for the `POR_EL2` system register.
    pub por_el2: PorEl2,
    /// Fake value for the `S2PIR_EL2` system register.
    pub s2pir_el2: S2pirEl2,
    /// Fake value for the `SCTLR2_EL2` system register.
    pub sctlr2_el2: Sctlr2El2,
    /// Fake value for the `SCTLR_EL2` system register.
    pub sctlr_el2: SctlrEl2,
    /// Fake value for the `SPSR_EL2` system register.
    pub spsr_el2: SpsrEl2,
    /// Fake value for the `SP_EL2` system register.
    pub sp_el2: SpEl2,
    /// Fake value for the `TCR2_EL2` system register.
    pub tcr2_el2: Tcr2El2,
    /// Fake value for the `TCR_EL2` system register.
    pub tcr_el2: TcrEl2,
    /// Fake value for the `TFSR_EL2` system register.
    pub tfsr_el2: TfsrEl2,
    /// Fake value for the `TPIDR_EL2` system register.
    pub tpidr_el2: TpidrEl2,
    /// Fake value for the `TTBR0_EL2` system register.
    pub ttbr0_el2: Ttbr0El2,
    /// Fake value for the `TTBR1_EL2` system register.
    pub ttbr1_el2: Ttbr1El2,
    /// Fake value for the `VBAR_EL2` system register.
    pub vbar_el2: VbarEl2,
    /// Fake value for the `VDISR_EL2` system register.
    pub vdisr_el2: VdisrEl2,
    /// Fake value for the `VMPIDR_EL2` system register.
    pub vmpidr_el2: VmpidrEl2,
    /// Fake value for the `VPIDR_EL2` system register.
    pub vpidr_el2: VpidrEl2,
    /// Fake value for the `VSESR_EL2` system register.
    pub vsesr_el2: VsesrEl2,
    /// Fake value for the `VTCR_EL2` system register.
    pub vtcr_el2: VtcrEl2,
    /// Fake value for the `VTTBR_EL2` system register.
    pub vttbr_el2: VttbrEl2,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            actlr_el2: 0,
            afsr0_el2: 0,
            afsr1_el2: 0,
            amair_el2: 0,
            amevcntvoff00_el2: Amevcntvoff00El2::empty(),
            amevcntvoff010_el2: Amevcntvoff010El2::empty(),
            amevcntvoff011_el2: Amevcntvoff011El2::empty(),
            amevcntvoff012_el2: Amevcntvoff012El2::empty(),
            amevcntvoff013_el2: Amevcntvoff013El2::empty(),
            amevcntvoff014_el2: Amevcntvoff014El2::empty(),
            amevcntvoff015_el2: Amevcntvoff015El2::empty(),
            amevcntvoff01_el2: Amevcntvoff01El2::empty(),
            amevcntvoff02_el2: Amevcntvoff02El2::empty(),
            amevcntvoff03_el2: Amevcntvoff03El2::empty(),
            amevcntvoff04_el2: Amevcntvoff04El2::empty(),
            amevcntvoff05_el2: Amevcntvoff05El2::empty(),
            amevcntvoff06_el2: Amevcntvoff06El2::empty(),
            amevcntvoff07_el2: Amevcntvoff07El2::empty(),
            amevcntvoff08_el2: Amevcntvoff08El2::empty(),
            amevcntvoff09_el2: Amevcntvoff09El2::empty(),
            amevcntvoff10_el2: Amevcntvoff10El2::empty(),
            amevcntvoff110_el2: Amevcntvoff110El2::empty(),
            amevcntvoff111_el2: Amevcntvoff111El2::empty(),
            amevcntvoff112_el2: Amevcntvoff112El2::empty(),
            amevcntvoff113_el2: Amevcntvoff113El2::empty(),
            amevcntvoff114_el2: Amevcntvoff114El2::empty(),
            amevcntvoff115_el2: Amevcntvoff115El2::empty(),
            amevcntvoff11_el2: Amevcntvoff11El2::empty(),
            amevcntvoff12_el2: Amevcntvoff12El2::empty(),
            amevcntvoff13_el2: Amevcntvoff13El2::empty(),
            amevcntvoff14_el2: Amevcntvoff14El2::empty(),
            amevcntvoff15_el2: Amevcntvoff15El2::empty(),
            amevcntvoff16_el2: Amevcntvoff16El2::empty(),
            amevcntvoff17_el2: Amevcntvoff17El2::empty(),
            amevcntvoff18_el2: Amevcntvoff18El2::empty(),
            amevcntvoff19_el2: Amevcntvoff19El2::empty(),
            brbcr_el2: BrbcrEl2::empty(),
            cnthctl_el2: CnthctlEl2::empty(),
            cnthps_ctl_el2: CnthpsCtlEl2::empty(),
            cnthps_cval_el2: CnthpsCvalEl2::empty(),
            cnthps_tval_el2: CnthpsTvalEl2::empty(),
            cnthp_ctl_el2: CnthpCtlEl2::empty(),
            cnthp_cval_el2: CnthpCvalEl2::empty(),
            cnthp_tval_el2: CnthpTvalEl2::empty(),
            cnthvs_ctl_el2: CnthvsCtlEl2::empty(),
            cnthvs_cval_el2: CnthvsCvalEl2::empty(),
            cnthvs_tval_el2: CnthvsTvalEl2::empty(),
            cnthv_ctl_el2: CnthvCtlEl2::empty(),
            cnthv_cval_el2: CnthvCvalEl2::empty(),
            cnthv_tval_el2: CnthvTvalEl2::empty(),
            cntpoff_el2: CntpoffEl2::empty(),
            cntvoff_el2: CntvoffEl2::empty(),
            contextidr_el2: ContextidrEl2::empty(),
            cptr_el2: CptrEl2::empty(),
            elr_el2: ElrEl2::empty(),
            esr_el2: EsrEl2::empty(),
            far_el2: FarEl2::empty(),
            gcscr_el2: GcscrEl2::empty(),
            gcspr_el2: GcsprEl2::empty(),
            hacr_el2: 0,
            hafgrtr_el2: HafgrtrEl2::empty(),
            hcrx_el2: HcrxEl2::empty(),
            hcr_el2: HcrEl2::empty(),
            hdfgrtr2_el2: Hdfgrtr2El2::empty(),
            hdfgrtr_el2: HdfgrtrEl2::empty(),
            hdfgwtr2_el2: Hdfgwtr2El2::empty(),
            hdfgwtr_el2: HdfgwtrEl2::empty(),
            hfgitr2_el2: Hfgitr2El2::empty(),
            hfgitr_el2: HfgitrEl2::empty(),
            hfgrtr2_el2: Hfgrtr2El2::empty(),
            hfgrtr_el2: HfgrtrEl2::empty(),
            hfgwtr2_el2: Hfgwtr2El2::empty(),
            hfgwtr_el2: HfgwtrEl2::empty(),
            hpfar_el2: HpfarEl2::empty(),
            hstr_el2: 0,
            icc_sre_el2: IccSreEl2::empty(),
            ich_hcr_el2: IchHcrEl2::empty(),
            ich_vmcr_el2: IchVmcrEl2::empty(),
            mair_el2: MairEl2::empty(),
            mdcr_el2: MdcrEl2::empty(),
            mpam2_el2: Mpam2El2::empty(),
            mpamhcr_el2: MpamhcrEl2::empty(),
            mpamvpm0_el2: Mpamvpm0El2::empty(),
            mpamvpm1_el2: Mpamvpm1El2::empty(),
            mpamvpm2_el2: Mpamvpm2El2::empty(),
            mpamvpm3_el2: Mpamvpm3El2::empty(),
            mpamvpm4_el2: Mpamvpm4El2::empty(),
            mpamvpm5_el2: Mpamvpm5El2::empty(),
            mpamvpm6_el2: Mpamvpm6El2::empty(),
            mpamvpm7_el2: Mpamvpm7El2::empty(),
            mpamvpmv_el2: MpamvpmvEl2::empty(),
            pfar_el2: PfarEl2::empty(),
            pire0_el2: Pire0El2::empty(),
            pir_el2: PirEl2::empty(),
            por_el2: PorEl2::empty(),
            s2pir_el2: S2pirEl2::empty(),
            sctlr2_el2: Sctlr2El2::empty(),
            sctlr_el2: SctlrEl2::empty(),
            spsr_el2: SpsrEl2::empty(),
            sp_el2: SpEl2::empty(),
            tcr2_el2: Tcr2El2::empty(),
            tcr_el2: TcrEl2::empty(),
            tfsr_el2: TfsrEl2::empty(),
            tpidr_el2: TpidrEl2::empty(),
            ttbr0_el2: Ttbr0El2::empty(),
            ttbr1_el2: Ttbr1El2::empty(),
            vbar_el2: VbarEl2::empty(),
            vdisr_el2: VdisrEl2::empty(),
            vmpidr_el2: VmpidrEl2::empty(),
            vpidr_el2: VpidrEl2::empty(),
            vsesr_el2: VsesrEl2::empty(),
            vtcr_el2: VtcrEl2::empty(),
            vttbr_el2: VttbrEl2::empty(),
        }
    }
}
