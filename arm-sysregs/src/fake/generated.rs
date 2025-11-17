// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{ApiakeyhiEl1, ApiakeyloEl1, CcsidrEl1, ClidrEl1, CntfrqEl0, CnthctlEl2, CntvoffEl2, ContextidrEl1, ContextidrEl2, CpacrEl1, CptrEl2, CptrEl3, CsselrEl1, CtrEl0, DisrEl1, Dit, ElrEl1, ElrEl2, EsrEl1, EsrEl2, EsrEl3, FarEl1, FarEl2, GcrEl1, GcscrEl1, GcscrEl2, HcrxEl2, HcrEl2, Hdfgrtr2El2, Hdfgwtr2El2, Hfgitr2El2, Hfgrtr2El2, Hfgwtr2El2, HfgwtrEl2, HpfarEl2, IccSreEl1, IccSreEl2, IccSreEl3, IchHcrEl2, IchVmcrEl2, IdAa64dfr0El1, IdAa64dfr1El1, IdAa64mmfr0El1, IdAa64mmfr1El1, IdAa64mmfr2El1, IdAa64mmfr3El1, IdAa64pfr0El1, IdAa64pfr1El1, IdAa64smfr0El1, IsrEl1, MairEl1, MairEl2, MairEl3, MdccintEl1, MdcrEl2, MdcrEl3, MdscrEl1, MidrEl1, MpamhcrEl2, MpamidrEl1, Mpamvpm0El2, Mpamvpm1El2, Mpamvpm2El2, Mpamvpm3El2, Mpamvpm4El2, Mpamvpm5El2, Mpamvpm6El2, Mpamvpm7El2, MpamvpmvEl2, MpidrEl1, ParEl1, PmcrEl0, RgsrEl1, ScrEl3, SctlrEl1, SctlrEl2, SctlrEl3, SmcrEl3, SpsrEl1, SpsrEl2, SpsrEl3, SpEl1, SpEl2, Tcr2El1, Tcr2El2, TcrEl1, TcrEl2, TcrEl3, Tfsre0El1, TfsrEl1, TfsrEl2, TpidrroEl0, TpidrEl0, TpidrEl1, TpidrEl2, Ttbr0El1, Ttbr0El2, Ttbr0El3, Ttbr1El1, Ttbr1El2, VbarEl1, VbarEl2, VdisrEl2, VmpidrEl2, VpidrEl2, VsesrEl2, VtcrEl2, VttbrEl2, ZcrEl3};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemRegisters {
    /// Fake value for the `ACTLR_EL1` system register.
    pub actlr_el1: u64,
    /// Fake value for the `ACTLR_EL2` system register.
    pub actlr_el2: u64,
    /// Fake value for the `AFSR0_EL1` system register.
    pub afsr0_el1: u64,
    /// Fake value for the `AFSR0_EL2` system register.
    pub afsr0_el2: u64,
    /// Fake value for the `AFSR1_EL1` system register.
    pub afsr1_el1: u64,
    /// Fake value for the `AFSR1_EL2` system register.
    pub afsr1_el2: u64,
    /// Fake value for the `AMAIR_EL1` system register.
    pub amair_el1: u64,
    /// Fake value for the `AMAIR_EL2` system register.
    pub amair_el2: u64,
    /// Fake value for the `APIAKeyHi_EL1` system register.
    pub apiakeyhi_el1: ApiakeyhiEl1,
    /// Fake value for the `APIAKeyLo_EL1` system register.
    pub apiakeylo_el1: ApiakeyloEl1,
    /// Fake value for the `CCSIDR_EL1` system register.
    pub ccsidr_el1: CcsidrEl1,
    /// Fake value for the `CLIDR_EL1` system register.
    pub clidr_el1: ClidrEl1,
    /// Fake value for the `CNTFRQ_EL0` system register.
    pub cntfrq_el0: CntfrqEl0,
    /// Fake value for the `CNTHCTL_EL2` system register.
    pub cnthctl_el2: CnthctlEl2,
    /// Fake value for the `CNTVOFF_EL2` system register.
    pub cntvoff_el2: CntvoffEl2,
    /// Fake value for the `CONTEXTIDR_EL1` system register.
    pub contextidr_el1: ContextidrEl1,
    /// Fake value for the `CONTEXTIDR_EL2` system register.
    pub contextidr_el2: ContextidrEl2,
    /// Fake value for the `CPACR_EL1` system register.
    pub cpacr_el1: CpacrEl1,
    /// Fake value for the `CPTR_EL2` system register.
    pub cptr_el2: CptrEl2,
    /// Fake value for the `CPTR_EL3` system register.
    pub cptr_el3: CptrEl3,
    /// Fake value for the `CSSELR_EL1` system register.
    pub csselr_el1: CsselrEl1,
    /// Fake value for the `CTR_EL0` system register.
    pub ctr_el0: CtrEl0,
    /// Fake value for the `DISR_EL1` system register.
    pub disr_el1: DisrEl1,
    /// Fake value for the `DIT` system register.
    pub dit: Dit,
    /// Fake value for the `ELR_EL1` system register.
    pub elr_el1: ElrEl1,
    /// Fake value for the `ELR_EL2` system register.
    pub elr_el2: ElrEl2,
    /// Fake value for the `ESR_EL1` system register.
    pub esr_el1: EsrEl1,
    /// Fake value for the `ESR_EL2` system register.
    pub esr_el2: EsrEl2,
    /// Fake value for the `ESR_EL3` system register.
    pub esr_el3: EsrEl3,
    /// Fake value for the `FAR_EL1` system register.
    pub far_el1: FarEl1,
    /// Fake value for the `FAR_EL2` system register.
    pub far_el2: FarEl2,
    /// Fake value for the `GCR_EL1` system register.
    pub gcr_el1: GcrEl1,
    /// Fake value for the `GCSCR_EL1` system register.
    pub gcscr_el1: GcscrEl1,
    /// Fake value for the `GCSCR_EL2` system register.
    pub gcscr_el2: GcscrEl2,
    /// Fake value for the `HACR_EL2` system register.
    pub hacr_el2: u64,
    /// Fake value for the `HCRX_EL2` system register.
    pub hcrx_el2: HcrxEl2,
    /// Fake value for the `HCR_EL2` system register.
    pub hcr_el2: HcrEl2,
    /// Fake value for the `HDFGRTR2_EL2` system register.
    pub hdfgrtr2_el2: Hdfgrtr2El2,
    /// Fake value for the `HDFGWTR2_EL2` system register.
    pub hdfgwtr2_el2: Hdfgwtr2El2,
    /// Fake value for the `HFGITR2_EL2` system register.
    pub hfgitr2_el2: Hfgitr2El2,
    /// Fake value for the `HFGRTR2_EL2` system register.
    pub hfgrtr2_el2: Hfgrtr2El2,
    /// Fake value for the `HFGWTR2_EL2` system register.
    pub hfgwtr2_el2: Hfgwtr2El2,
    /// Fake value for the `HFGWTR_EL2` system register.
    pub hfgwtr_el2: HfgwtrEl2,
    /// Fake value for the `HPFAR_EL2` system register.
    pub hpfar_el2: HpfarEl2,
    /// Fake value for the `HSTR_EL2` system register.
    pub hstr_el2: u64,
    /// Fake value for the `ICC_SRE_EL1` system register.
    pub icc_sre_el1: IccSreEl1,
    /// Fake value for the `ICC_SRE_EL2` system register.
    pub icc_sre_el2: IccSreEl2,
    /// Fake value for the `ICC_SRE_EL3` system register.
    pub icc_sre_el3: IccSreEl3,
    /// Fake value for the `ICH_HCR_EL2` system register.
    pub ich_hcr_el2: IchHcrEl2,
    /// Fake value for the `ICH_VMCR_EL2` system register.
    pub ich_vmcr_el2: IchVmcrEl2,
    /// Fake value for the `ID_AA64DFR0_EL1` system register.
    pub id_aa64dfr0_el1: IdAa64dfr0El1,
    /// Fake value for the `ID_AA64DFR1_EL1` system register.
    pub id_aa64dfr1_el1: IdAa64dfr1El1,
    /// Fake value for the `ID_AA64MMFR0_EL1` system register.
    pub id_aa64mmfr0_el1: IdAa64mmfr0El1,
    /// Fake value for the `ID_AA64MMFR1_EL1` system register.
    pub id_aa64mmfr1_el1: IdAa64mmfr1El1,
    /// Fake value for the `ID_AA64MMFR2_EL1` system register.
    pub id_aa64mmfr2_el1: IdAa64mmfr2El1,
    /// Fake value for the `ID_AA64MMFR3_EL1` system register.
    pub id_aa64mmfr3_el1: IdAa64mmfr3El1,
    /// Fake value for the `ID_AA64PFR0_EL1` system register.
    pub id_aa64pfr0_el1: IdAa64pfr0El1,
    /// Fake value for the `ID_AA64PFR1_EL1` system register.
    pub id_aa64pfr1_el1: IdAa64pfr1El1,
    /// Fake value for the `ID_AA64SMFR0_EL1` system register.
    pub id_aa64smfr0_el1: IdAa64smfr0El1,
    /// Fake value for the `ISR_EL1` system register.
    pub isr_el1: IsrEl1,
    /// Fake value for the `MAIR_EL1` system register.
    pub mair_el1: MairEl1,
    /// Fake value for the `MAIR_EL2` system register.
    pub mair_el2: MairEl2,
    /// Fake value for the `MAIR_EL3` system register.
    pub mair_el3: MairEl3,
    /// Fake value for the `MDCCINT_EL1` system register.
    pub mdccint_el1: MdccintEl1,
    /// Fake value for the `MDCR_EL2` system register.
    pub mdcr_el2: MdcrEl2,
    /// Fake value for the `MDCR_EL3` system register.
    pub mdcr_el3: MdcrEl3,
    /// Fake value for the `MDSCR_EL1` system register.
    pub mdscr_el1: MdscrEl1,
    /// Fake value for the `MIDR_EL1` system register.
    pub midr_el1: MidrEl1,
    /// Fake value for the `MPAM2_EL2` system register.
    pub mpam2_el2: u64,
    /// Fake value for the `MPAM3_EL3` system register.
    pub mpam3_el3: u64,
    /// Fake value for the `MPAMHCR_EL2` system register.
    pub mpamhcr_el2: MpamhcrEl2,
    /// Fake value for the `MPAMIDR_EL1` system register.
    pub mpamidr_el1: MpamidrEl1,
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
    /// Fake value for the `MPIDR_EL1` system register.
    pub mpidr_el1: MpidrEl1,
    /// Fake value for the `PAR_EL1` system register.
    pub par_el1: ParEl1,
    /// Fake value for the `PMCR_EL0` system register.
    pub pmcr_el0: PmcrEl0,
    /// Fake value for the `RGSR_EL1` system register.
    pub rgsr_el1: RgsrEl1,
    /// Fake value for the `SCR_EL3` system register.
    pub scr_el3: ScrEl3,
    /// Fake value for the `SCTLR_EL1` system register.
    pub sctlr_el1: SctlrEl1,
    /// Fake value for the `SCTLR_EL2` system register.
    pub sctlr_el2: SctlrEl2,
    /// Fake value for the `SCTLR_EL3` system register.
    pub sctlr_el3: SctlrEl3,
    /// Fake value for the `SMCR_EL3` system register.
    pub smcr_el3: SmcrEl3,
    /// Fake value for the `SPSR_EL1` system register.
    pub spsr_el1: SpsrEl1,
    /// Fake value for the `SPSR_EL2` system register.
    pub spsr_el2: SpsrEl2,
    /// Fake value for the `SPSR_EL3` system register.
    pub spsr_el3: SpsrEl3,
    /// Fake value for the `SP_EL1` system register.
    pub sp_el1: SpEl1,
    /// Fake value for the `SP_EL2` system register.
    pub sp_el2: SpEl2,
    /// Fake value for the `TCR2_EL1` system register.
    pub tcr2_el1: Tcr2El1,
    /// Fake value for the `TCR2_EL2` system register.
    pub tcr2_el2: Tcr2El2,
    /// Fake value for the `TCR_EL1` system register.
    pub tcr_el1: TcrEl1,
    /// Fake value for the `TCR_EL2` system register.
    pub tcr_el2: TcrEl2,
    /// Fake value for the `TCR_EL3` system register.
    pub tcr_el3: TcrEl3,
    /// Fake value for the `TFSRE0_EL1` system register.
    pub tfsre0_el1: Tfsre0El1,
    /// Fake value for the `TFSR_EL1` system register.
    pub tfsr_el1: TfsrEl1,
    /// Fake value for the `TFSR_EL2` system register.
    pub tfsr_el2: TfsrEl2,
    /// Fake value for the `TPIDRRO_EL0` system register.
    pub tpidrro_el0: TpidrroEl0,
    /// Fake value for the `TPIDR_EL0` system register.
    pub tpidr_el0: TpidrEl0,
    /// Fake value for the `TPIDR_EL1` system register.
    pub tpidr_el1: TpidrEl1,
    /// Fake value for the `TPIDR_EL2` system register.
    pub tpidr_el2: TpidrEl2,
    /// Fake value for the `TTBR0_EL1` system register.
    pub ttbr0_el1: Ttbr0El1,
    /// Fake value for the `TTBR0_EL2` system register.
    pub ttbr0_el2: Ttbr0El2,
    /// Fake value for the `TTBR0_EL3` system register.
    pub ttbr0_el3: Ttbr0El3,
    /// Fake value for the `TTBR1_EL1` system register.
    pub ttbr1_el1: Ttbr1El1,
    /// Fake value for the `TTBR1_EL2` system register.
    pub ttbr1_el2: Ttbr1El2,
    /// Fake value for the `VBAR_EL1` system register.
    pub vbar_el1: VbarEl1,
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
    /// Fake value for the `ZCR_EL3` system register.
    pub zcr_el3: ZcrEl3,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            actlr_el1: 0,
            actlr_el2: 0,
            afsr0_el1: 0,
            afsr0_el2: 0,
            afsr1_el1: 0,
            afsr1_el2: 0,
            amair_el1: 0,
            amair_el2: 0,
            apiakeyhi_el1: ApiakeyhiEl1::empty(),
            apiakeylo_el1: ApiakeyloEl1::empty(),
            ccsidr_el1: CcsidrEl1::empty(),
            clidr_el1: ClidrEl1::empty(),
            cntfrq_el0: CntfrqEl0::empty(),
            cnthctl_el2: CnthctlEl2::empty(),
            cntvoff_el2: CntvoffEl2::empty(),
            contextidr_el1: ContextidrEl1::empty(),
            contextidr_el2: ContextidrEl2::empty(),
            cpacr_el1: CpacrEl1::empty(),
            cptr_el2: CptrEl2::empty(),
            cptr_el3: CptrEl3::empty(),
            csselr_el1: CsselrEl1::empty(),
            ctr_el0: CtrEl0::empty(),
            disr_el1: DisrEl1::empty(),
            dit: Dit::empty(),
            elr_el1: ElrEl1::empty(),
            elr_el2: ElrEl2::empty(),
            esr_el1: EsrEl1::empty(),
            esr_el2: EsrEl2::empty(),
            esr_el3: EsrEl3::empty(),
            far_el1: FarEl1::empty(),
            far_el2: FarEl2::empty(),
            gcr_el1: GcrEl1::empty(),
            gcscr_el1: GcscrEl1::empty(),
            gcscr_el2: GcscrEl2::empty(),
            hacr_el2: 0,
            hcrx_el2: HcrxEl2::empty(),
            hcr_el2: HcrEl2::empty(),
            hdfgrtr2_el2: Hdfgrtr2El2::empty(),
            hdfgwtr2_el2: Hdfgwtr2El2::empty(),
            hfgitr2_el2: Hfgitr2El2::empty(),
            hfgrtr2_el2: Hfgrtr2El2::empty(),
            hfgwtr2_el2: Hfgwtr2El2::empty(),
            hfgwtr_el2: HfgwtrEl2::empty(),
            hpfar_el2: HpfarEl2::empty(),
            hstr_el2: 0,
            icc_sre_el1: IccSreEl1::empty(),
            icc_sre_el2: IccSreEl2::empty(),
            icc_sre_el3: IccSreEl3::empty(),
            ich_hcr_el2: IchHcrEl2::empty(),
            ich_vmcr_el2: IchVmcrEl2::empty(),
            id_aa64dfr0_el1: IdAa64dfr0El1::empty(),
            id_aa64dfr1_el1: IdAa64dfr1El1::empty(),
            id_aa64mmfr0_el1: IdAa64mmfr0El1::empty(),
            id_aa64mmfr1_el1: IdAa64mmfr1El1::empty(),
            id_aa64mmfr2_el1: IdAa64mmfr2El1::empty(),
            id_aa64mmfr3_el1: IdAa64mmfr3El1::empty(),
            id_aa64pfr0_el1: IdAa64pfr0El1::empty(),
            id_aa64pfr1_el1: IdAa64pfr1El1::empty(),
            id_aa64smfr0_el1: IdAa64smfr0El1::empty(),
            isr_el1: IsrEl1::empty(),
            mair_el1: MairEl1::empty(),
            mair_el2: MairEl2::empty(),
            mair_el3: MairEl3::empty(),
            mdccint_el1: MdccintEl1::empty(),
            mdcr_el2: MdcrEl2::empty(),
            mdcr_el3: MdcrEl3::empty(),
            mdscr_el1: MdscrEl1::empty(),
            midr_el1: MidrEl1::empty(),
            mpam2_el2: 0,
            mpam3_el3: 0,
            mpamhcr_el2: MpamhcrEl2::empty(),
            mpamidr_el1: MpamidrEl1::empty(),
            mpamvpm0_el2: Mpamvpm0El2::empty(),
            mpamvpm1_el2: Mpamvpm1El2::empty(),
            mpamvpm2_el2: Mpamvpm2El2::empty(),
            mpamvpm3_el2: Mpamvpm3El2::empty(),
            mpamvpm4_el2: Mpamvpm4El2::empty(),
            mpamvpm5_el2: Mpamvpm5El2::empty(),
            mpamvpm6_el2: Mpamvpm6El2::empty(),
            mpamvpm7_el2: Mpamvpm7El2::empty(),
            mpamvpmv_el2: MpamvpmvEl2::empty(),
            mpidr_el1: MpidrEl1::empty(),
            par_el1: ParEl1::empty(),
            pmcr_el0: PmcrEl0::empty(),
            rgsr_el1: RgsrEl1::empty(),
            scr_el3: ScrEl3::empty(),
            sctlr_el1: SctlrEl1::empty(),
            sctlr_el2: SctlrEl2::empty(),
            sctlr_el3: SctlrEl3::empty(),
            smcr_el3: SmcrEl3::empty(),
            spsr_el1: SpsrEl1::empty(),
            spsr_el2: SpsrEl2::empty(),
            spsr_el3: SpsrEl3::empty(),
            sp_el1: SpEl1::empty(),
            sp_el2: SpEl2::empty(),
            tcr2_el1: Tcr2El1::empty(),
            tcr2_el2: Tcr2El2::empty(),
            tcr_el1: TcrEl1::empty(),
            tcr_el2: TcrEl2::empty(),
            tcr_el3: TcrEl3::empty(),
            tfsre0_el1: Tfsre0El1::empty(),
            tfsr_el1: TfsrEl1::empty(),
            tfsr_el2: TfsrEl2::empty(),
            tpidrro_el0: TpidrroEl0::empty(),
            tpidr_el0: TpidrEl0::empty(),
            tpidr_el1: TpidrEl1::empty(),
            tpidr_el2: TpidrEl2::empty(),
            ttbr0_el1: Ttbr0El1::empty(),
            ttbr0_el2: Ttbr0El2::empty(),
            ttbr0_el3: Ttbr0El3::empty(),
            ttbr1_el1: Ttbr1El1::empty(),
            ttbr1_el2: Ttbr1El2::empty(),
            vbar_el1: VbarEl1::empty(),
            vbar_el2: VbarEl2::empty(),
            vdisr_el2: VdisrEl2::empty(),
            vmpidr_el2: VmpidrEl2::empty(),
            vpidr_el2: VpidrEl2::empty(),
            vsesr_el2: VsesrEl2::empty(),
            vtcr_el2: VtcrEl2::empty(),
            vttbr_el2: VttbrEl2::empty(),
            zcr_el3: ZcrEl3::empty(),
        }
    }
}
