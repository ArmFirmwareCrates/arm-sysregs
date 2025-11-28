// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{CntfrqEl0, CtrEl0, Dit, PmcrEl0, TpidrroEl0, TpidrEl0};
#[cfg(feature = "el1")]
use crate::{ApiakeyhiEl1, ApiakeyloEl1, CcsidrEl1, ClidrEl1, ContextidrEl1, CpacrEl1, CsselrEl1, DisrEl1, ElrEl1, EsrEl1, FarEl1, GcrEl1, GcscrEl1, IccSreEl1, IdAa64dfr0El1, IdAa64dfr1El1, IdAa64mmfr0El1, IdAa64mmfr1El1, IdAa64mmfr2El1, IdAa64mmfr3El1, IdAa64pfr0El1, IdAa64pfr1El1, IsrEl1, MairEl1, MdccintEl1, MdscrEl1, MidrEl1, MpamidrEl1, MpidrEl1, ParEl1, RgsrEl1, SctlrEl1, SpsrEl1, SpEl1, Tcr2El1, TcrEl1, Tfsre0El1, TfsrEl1, TpidrEl1, Ttbr0El1, Ttbr1El1, VbarEl1};
#[cfg(feature = "el2")]
use crate::{CnthctlEl2, CntvoffEl2, ContextidrEl2, CptrEl2, ElrEl2, EsrEl2, FarEl2, GcscrEl2, HcrxEl2, HcrEl2, Hdfgrtr2El2, Hdfgwtr2El2, Hfgitr2El2, Hfgrtr2El2, HfgwtrEl2, HpfarEl2, IccSreEl2, IchHcrEl2, IchVmcrEl2, MairEl2, MdcrEl2, Mpam2El2, MpamhcrEl2, Mpamvpm0El2, Mpamvpm1El2, Mpamvpm2El2, Mpamvpm3El2, Mpamvpm4El2, Mpamvpm5El2, Mpamvpm6El2, Mpamvpm7El2, MpamvpmvEl2, SctlrEl2, SpsrEl2, SpEl2, Tcr2El2, TcrEl2, TfsrEl2, TpidrEl2, Ttbr0El2, Ttbr1El2, VbarEl2, VdisrEl2, VmpidrEl2, VpidrEl2, VsesrEl2, VtcrEl2, VttbrEl2};
#[cfg(feature = "el3")]
use crate::{CptrEl3, EsrEl3, IccSreEl3, MairEl3, MdcrEl3, Mpam3El3, ScrEl3, SctlrEl3, SpsrEl3, TcrEl3, Ttbr0El3, ZcrEl3};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemRegisters {
    #[cfg(feature = "el1")]
    /// Fake value for the `ACTLR_EL1` system register.
    pub actlr_el1: u64,
    #[cfg(feature = "el2")]
    /// Fake value for the `ACTLR_EL2` system register.
    pub actlr_el2: u64,
    #[cfg(feature = "el1")]
    /// Fake value for the `AFSR0_EL1` system register.
    pub afsr0_el1: u64,
    #[cfg(feature = "el2")]
    /// Fake value for the `AFSR0_EL2` system register.
    pub afsr0_el2: u64,
    #[cfg(feature = "el1")]
    /// Fake value for the `AFSR1_EL1` system register.
    pub afsr1_el1: u64,
    #[cfg(feature = "el2")]
    /// Fake value for the `AFSR1_EL2` system register.
    pub afsr1_el2: u64,
    #[cfg(feature = "el1")]
    /// Fake value for the `AMAIR_EL1` system register.
    pub amair_el1: u64,
    #[cfg(feature = "el2")]
    /// Fake value for the `AMAIR_EL2` system register.
    pub amair_el2: u64,
    #[cfg(feature = "el1")]
    /// Fake value for the `APIAKeyHi_EL1` system register.
    pub apiakeyhi_el1: ApiakeyhiEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `APIAKeyLo_EL1` system register.
    pub apiakeylo_el1: ApiakeyloEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `CCSIDR_EL1` system register.
    pub ccsidr_el1: CcsidrEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `CLIDR_EL1` system register.
    pub clidr_el1: ClidrEl1,
    /// Fake value for the `CNTFRQ_EL0` system register.
    pub cntfrq_el0: CntfrqEl0,
    #[cfg(feature = "el2")]
    /// Fake value for the `CNTHCTL_EL2` system register.
    pub cnthctl_el2: CnthctlEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `CNTVOFF_EL2` system register.
    pub cntvoff_el2: CntvoffEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `CONTEXTIDR_EL1` system register.
    pub contextidr_el1: ContextidrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `CONTEXTIDR_EL2` system register.
    pub contextidr_el2: ContextidrEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `CPACR_EL1` system register.
    pub cpacr_el1: CpacrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `CPTR_EL2` system register.
    pub cptr_el2: CptrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `CPTR_EL3` system register.
    pub cptr_el3: CptrEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `CSSELR_EL1` system register.
    pub csselr_el1: CsselrEl1,
    /// Fake value for the `CTR_EL0` system register.
    pub ctr_el0: CtrEl0,
    #[cfg(feature = "el1")]
    /// Fake value for the `DISR_EL1` system register.
    pub disr_el1: DisrEl1,
    /// Fake value for the `DIT` system register.
    pub dit: Dit,
    #[cfg(feature = "el1")]
    /// Fake value for the `ELR_EL1` system register.
    pub elr_el1: ElrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `ELR_EL2` system register.
    pub elr_el2: ElrEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `ESR_EL1` system register.
    pub esr_el1: EsrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `ESR_EL2` system register.
    pub esr_el2: EsrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `ESR_EL3` system register.
    pub esr_el3: EsrEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `FAR_EL1` system register.
    pub far_el1: FarEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `FAR_EL2` system register.
    pub far_el2: FarEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `GCR_EL1` system register.
    pub gcr_el1: GcrEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `GCSCR_EL1` system register.
    pub gcscr_el1: GcscrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `GCSCR_EL2` system register.
    pub gcscr_el2: GcscrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HACR_EL2` system register.
    pub hacr_el2: u64,
    #[cfg(feature = "el2")]
    /// Fake value for the `HCRX_EL2` system register.
    pub hcrx_el2: HcrxEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HCR_EL2` system register.
    pub hcr_el2: HcrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HDFGRTR2_EL2` system register.
    pub hdfgrtr2_el2: Hdfgrtr2El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HDFGWTR2_EL2` system register.
    pub hdfgwtr2_el2: Hdfgwtr2El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HFGITR2_EL2` system register.
    pub hfgitr2_el2: Hfgitr2El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HFGRTR2_EL2` system register.
    pub hfgrtr2_el2: Hfgrtr2El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HFGWTR_EL2` system register.
    pub hfgwtr_el2: HfgwtrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HPFAR_EL2` system register.
    pub hpfar_el2: HpfarEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HSTR_EL2` system register.
    pub hstr_el2: u64,
    #[cfg(feature = "el1")]
    /// Fake value for the `ICC_SRE_EL1` system register.
    pub icc_sre_el1: IccSreEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `ICC_SRE_EL2` system register.
    pub icc_sre_el2: IccSreEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `ICC_SRE_EL3` system register.
    pub icc_sre_el3: IccSreEl3,
    #[cfg(feature = "el2")]
    /// Fake value for the `ICH_HCR_EL2` system register.
    pub ich_hcr_el2: IchHcrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `ICH_VMCR_EL2` system register.
    pub ich_vmcr_el2: IchVmcrEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64DFR0_EL1` system register.
    pub id_aa64dfr0_el1: IdAa64dfr0El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64DFR1_EL1` system register.
    pub id_aa64dfr1_el1: IdAa64dfr1El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64MMFR0_EL1` system register.
    pub id_aa64mmfr0_el1: IdAa64mmfr0El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64MMFR1_EL1` system register.
    pub id_aa64mmfr1_el1: IdAa64mmfr1El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64MMFR2_EL1` system register.
    pub id_aa64mmfr2_el1: IdAa64mmfr2El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64MMFR3_EL1` system register.
    pub id_aa64mmfr3_el1: IdAa64mmfr3El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64PFR0_EL1` system register.
    pub id_aa64pfr0_el1: IdAa64pfr0El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64PFR1_EL1` system register.
    pub id_aa64pfr1_el1: IdAa64pfr1El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ISR_EL1` system register.
    pub isr_el1: IsrEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `MAIR_EL1` system register.
    pub mair_el1: MairEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `MAIR_EL2` system register.
    pub mair_el2: MairEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `MAIR_EL3` system register.
    pub mair_el3: MairEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `MDCCINT_EL1` system register.
    pub mdccint_el1: MdccintEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `MDCR_EL2` system register.
    pub mdcr_el2: MdcrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `MDCR_EL3` system register.
    pub mdcr_el3: MdcrEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `MDSCR_EL1` system register.
    pub mdscr_el1: MdscrEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `MIDR_EL1` system register.
    pub midr_el1: MidrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAM2_EL2` system register.
    pub mpam2_el2: Mpam2El2,
    #[cfg(feature = "el3")]
    /// Fake value for the `MPAM3_EL3` system register.
    pub mpam3_el3: Mpam3El3,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMHCR_EL2` system register.
    pub mpamhcr_el2: MpamhcrEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `MPAMIDR_EL1` system register.
    pub mpamidr_el1: MpamidrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM0_EL2` system register.
    pub mpamvpm0_el2: Mpamvpm0El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM1_EL2` system register.
    pub mpamvpm1_el2: Mpamvpm1El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM2_EL2` system register.
    pub mpamvpm2_el2: Mpamvpm2El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM3_EL2` system register.
    pub mpamvpm3_el2: Mpamvpm3El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM4_EL2` system register.
    pub mpamvpm4_el2: Mpamvpm4El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM5_EL2` system register.
    pub mpamvpm5_el2: Mpamvpm5El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM6_EL2` system register.
    pub mpamvpm6_el2: Mpamvpm6El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPM7_EL2` system register.
    pub mpamvpm7_el2: Mpamvpm7El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `MPAMVPMV_EL2` system register.
    pub mpamvpmv_el2: MpamvpmvEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `MPIDR_EL1` system register.
    pub mpidr_el1: MpidrEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `PAR_EL1` system register.
    pub par_el1: ParEl1,
    /// Fake value for the `PMCR_EL0` system register.
    pub pmcr_el0: PmcrEl0,
    #[cfg(feature = "el1")]
    /// Fake value for the `RGSR_EL1` system register.
    pub rgsr_el1: RgsrEl1,
    #[cfg(feature = "el3")]
    /// Fake value for the `SCR_EL3` system register.
    pub scr_el3: ScrEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `SCTLR_EL1` system register.
    pub sctlr_el1: SctlrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `SCTLR_EL2` system register.
    pub sctlr_el2: SctlrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `SCTLR_EL3` system register.
    pub sctlr_el3: SctlrEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `SPSR_EL1` system register.
    pub spsr_el1: SpsrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `SPSR_EL2` system register.
    pub spsr_el2: SpsrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `SPSR_EL3` system register.
    pub spsr_el3: SpsrEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `SP_EL1` system register.
    pub sp_el1: SpEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `SP_EL2` system register.
    pub sp_el2: SpEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `TCR2_EL1` system register.
    pub tcr2_el1: Tcr2El1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TCR2_EL2` system register.
    pub tcr2_el2: Tcr2El2,
    #[cfg(feature = "el1")]
    /// Fake value for the `TCR_EL1` system register.
    pub tcr_el1: TcrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TCR_EL2` system register.
    pub tcr_el2: TcrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `TCR_EL3` system register.
    pub tcr_el3: TcrEl3,
    #[cfg(feature = "el1")]
    /// Fake value for the `TFSRE0_EL1` system register.
    pub tfsre0_el1: Tfsre0El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `TFSR_EL1` system register.
    pub tfsr_el1: TfsrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TFSR_EL2` system register.
    pub tfsr_el2: TfsrEl2,
    /// Fake value for the `TPIDRRO_EL0` system register.
    pub tpidrro_el0: TpidrroEl0,
    /// Fake value for the `TPIDR_EL0` system register.
    pub tpidr_el0: TpidrEl0,
    #[cfg(feature = "el1")]
    /// Fake value for the `TPIDR_EL1` system register.
    pub tpidr_el1: TpidrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TPIDR_EL2` system register.
    pub tpidr_el2: TpidrEl2,
    #[cfg(feature = "el1")]
    /// Fake value for the `TTBR0_EL1` system register.
    pub ttbr0_el1: Ttbr0El1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TTBR0_EL2` system register.
    pub ttbr0_el2: Ttbr0El2,
    #[cfg(feature = "el3")]
    /// Fake value for the `TTBR0_EL3` system register.
    pub ttbr0_el3: Ttbr0El3,
    #[cfg(feature = "el1")]
    /// Fake value for the `TTBR1_EL1` system register.
    pub ttbr1_el1: Ttbr1El1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TTBR1_EL2` system register.
    pub ttbr1_el2: Ttbr1El2,
    #[cfg(feature = "el1")]
    /// Fake value for the `VBAR_EL1` system register.
    pub vbar_el1: VbarEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `VBAR_EL2` system register.
    pub vbar_el2: VbarEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `VDISR_EL2` system register.
    pub vdisr_el2: VdisrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `VMPIDR_EL2` system register.
    pub vmpidr_el2: VmpidrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `VPIDR_EL2` system register.
    pub vpidr_el2: VpidrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `VSESR_EL2` system register.
    pub vsesr_el2: VsesrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `VTCR_EL2` system register.
    pub vtcr_el2: VtcrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `VTTBR_EL2` system register.
    pub vttbr_el2: VttbrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `ZCR_EL3` system register.
    pub zcr_el3: ZcrEl3,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            #[cfg(feature = "el1")]
            actlr_el1: 0,
            #[cfg(feature = "el2")]
            actlr_el2: 0,
            #[cfg(feature = "el1")]
            afsr0_el1: 0,
            #[cfg(feature = "el2")]
            afsr0_el2: 0,
            #[cfg(feature = "el1")]
            afsr1_el1: 0,
            #[cfg(feature = "el2")]
            afsr1_el2: 0,
            #[cfg(feature = "el1")]
            amair_el1: 0,
            #[cfg(feature = "el2")]
            amair_el2: 0,
            #[cfg(feature = "el1")]
            apiakeyhi_el1: ApiakeyhiEl1::empty(),
            #[cfg(feature = "el1")]
            apiakeylo_el1: ApiakeyloEl1::empty(),
            #[cfg(feature = "el1")]
            ccsidr_el1: CcsidrEl1::empty(),
            #[cfg(feature = "el1")]
            clidr_el1: ClidrEl1::empty(),
            cntfrq_el0: CntfrqEl0::empty(),
            #[cfg(feature = "el2")]
            cnthctl_el2: CnthctlEl2::empty(),
            #[cfg(feature = "el2")]
            cntvoff_el2: CntvoffEl2::empty(),
            #[cfg(feature = "el1")]
            contextidr_el1: ContextidrEl1::empty(),
            #[cfg(feature = "el2")]
            contextidr_el2: ContextidrEl2::empty(),
            #[cfg(feature = "el1")]
            cpacr_el1: CpacrEl1::empty(),
            #[cfg(feature = "el2")]
            cptr_el2: CptrEl2::empty(),
            #[cfg(feature = "el3")]
            cptr_el3: CptrEl3::empty(),
            #[cfg(feature = "el1")]
            csselr_el1: CsselrEl1::empty(),
            ctr_el0: CtrEl0::empty(),
            #[cfg(feature = "el1")]
            disr_el1: DisrEl1::empty(),
            dit: Dit::empty(),
            #[cfg(feature = "el1")]
            elr_el1: ElrEl1::empty(),
            #[cfg(feature = "el2")]
            elr_el2: ElrEl2::empty(),
            #[cfg(feature = "el1")]
            esr_el1: EsrEl1::empty(),
            #[cfg(feature = "el2")]
            esr_el2: EsrEl2::empty(),
            #[cfg(feature = "el3")]
            esr_el3: EsrEl3::empty(),
            #[cfg(feature = "el1")]
            far_el1: FarEl1::empty(),
            #[cfg(feature = "el2")]
            far_el2: FarEl2::empty(),
            #[cfg(feature = "el1")]
            gcr_el1: GcrEl1::empty(),
            #[cfg(feature = "el1")]
            gcscr_el1: GcscrEl1::empty(),
            #[cfg(feature = "el2")]
            gcscr_el2: GcscrEl2::empty(),
            #[cfg(feature = "el2")]
            hacr_el2: 0,
            #[cfg(feature = "el2")]
            hcrx_el2: HcrxEl2::empty(),
            #[cfg(feature = "el2")]
            hcr_el2: HcrEl2::empty(),
            #[cfg(feature = "el2")]
            hdfgrtr2_el2: Hdfgrtr2El2::empty(),
            #[cfg(feature = "el2")]
            hdfgwtr2_el2: Hdfgwtr2El2::empty(),
            #[cfg(feature = "el2")]
            hfgitr2_el2: Hfgitr2El2::empty(),
            #[cfg(feature = "el2")]
            hfgrtr2_el2: Hfgrtr2El2::empty(),
            #[cfg(feature = "el2")]
            hfgwtr_el2: HfgwtrEl2::empty(),
            #[cfg(feature = "el2")]
            hpfar_el2: HpfarEl2::empty(),
            #[cfg(feature = "el2")]
            hstr_el2: 0,
            #[cfg(feature = "el1")]
            icc_sre_el1: IccSreEl1::empty(),
            #[cfg(feature = "el2")]
            icc_sre_el2: IccSreEl2::empty(),
            #[cfg(feature = "el3")]
            icc_sre_el3: IccSreEl3::empty(),
            #[cfg(feature = "el2")]
            ich_hcr_el2: IchHcrEl2::empty(),
            #[cfg(feature = "el2")]
            ich_vmcr_el2: IchVmcrEl2::empty(),
            #[cfg(feature = "el1")]
            id_aa64dfr0_el1: IdAa64dfr0El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64dfr1_el1: IdAa64dfr1El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64mmfr0_el1: IdAa64mmfr0El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64mmfr1_el1: IdAa64mmfr1El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64mmfr2_el1: IdAa64mmfr2El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64mmfr3_el1: IdAa64mmfr3El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64pfr0_el1: IdAa64pfr0El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64pfr1_el1: IdAa64pfr1El1::empty(),
            #[cfg(feature = "el1")]
            isr_el1: IsrEl1::empty(),
            #[cfg(feature = "el1")]
            mair_el1: MairEl1::empty(),
            #[cfg(feature = "el2")]
            mair_el2: MairEl2::empty(),
            #[cfg(feature = "el3")]
            mair_el3: MairEl3::empty(),
            #[cfg(feature = "el1")]
            mdccint_el1: MdccintEl1::empty(),
            #[cfg(feature = "el2")]
            mdcr_el2: MdcrEl2::empty(),
            #[cfg(feature = "el3")]
            mdcr_el3: MdcrEl3::empty(),
            #[cfg(feature = "el1")]
            mdscr_el1: MdscrEl1::empty(),
            #[cfg(feature = "el1")]
            midr_el1: MidrEl1::empty(),
            #[cfg(feature = "el2")]
            mpam2_el2: Mpam2El2::empty(),
            #[cfg(feature = "el3")]
            mpam3_el3: Mpam3El3::empty(),
            #[cfg(feature = "el2")]
            mpamhcr_el2: MpamhcrEl2::empty(),
            #[cfg(feature = "el1")]
            mpamidr_el1: MpamidrEl1::empty(),
            #[cfg(feature = "el2")]
            mpamvpm0_el2: Mpamvpm0El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpm1_el2: Mpamvpm1El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpm2_el2: Mpamvpm2El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpm3_el2: Mpamvpm3El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpm4_el2: Mpamvpm4El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpm5_el2: Mpamvpm5El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpm6_el2: Mpamvpm6El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpm7_el2: Mpamvpm7El2::empty(),
            #[cfg(feature = "el2")]
            mpamvpmv_el2: MpamvpmvEl2::empty(),
            #[cfg(feature = "el1")]
            mpidr_el1: MpidrEl1::empty(),
            #[cfg(feature = "el1")]
            par_el1: ParEl1::empty(),
            pmcr_el0: PmcrEl0::empty(),
            #[cfg(feature = "el1")]
            rgsr_el1: RgsrEl1::empty(),
            #[cfg(feature = "el3")]
            scr_el3: ScrEl3::empty(),
            #[cfg(feature = "el1")]
            sctlr_el1: SctlrEl1::empty(),
            #[cfg(feature = "el2")]
            sctlr_el2: SctlrEl2::empty(),
            #[cfg(feature = "el3")]
            sctlr_el3: SctlrEl3::empty(),
            #[cfg(feature = "el1")]
            spsr_el1: SpsrEl1::empty(),
            #[cfg(feature = "el2")]
            spsr_el2: SpsrEl2::empty(),
            #[cfg(feature = "el3")]
            spsr_el3: SpsrEl3::empty(),
            #[cfg(feature = "el1")]
            sp_el1: SpEl1::empty(),
            #[cfg(feature = "el2")]
            sp_el2: SpEl2::empty(),
            #[cfg(feature = "el1")]
            tcr2_el1: Tcr2El1::empty(),
            #[cfg(feature = "el2")]
            tcr2_el2: Tcr2El2::empty(),
            #[cfg(feature = "el1")]
            tcr_el1: TcrEl1::empty(),
            #[cfg(feature = "el2")]
            tcr_el2: TcrEl2::empty(),
            #[cfg(feature = "el3")]
            tcr_el3: TcrEl3::empty(),
            #[cfg(feature = "el1")]
            tfsre0_el1: Tfsre0El1::empty(),
            #[cfg(feature = "el1")]
            tfsr_el1: TfsrEl1::empty(),
            #[cfg(feature = "el2")]
            tfsr_el2: TfsrEl2::empty(),
            tpidrro_el0: TpidrroEl0::empty(),
            tpidr_el0: TpidrEl0::empty(),
            #[cfg(feature = "el1")]
            tpidr_el1: TpidrEl1::empty(),
            #[cfg(feature = "el2")]
            tpidr_el2: TpidrEl2::empty(),
            #[cfg(feature = "el1")]
            ttbr0_el1: Ttbr0El1::empty(),
            #[cfg(feature = "el2")]
            ttbr0_el2: Ttbr0El2::empty(),
            #[cfg(feature = "el3")]
            ttbr0_el3: Ttbr0El3::empty(),
            #[cfg(feature = "el1")]
            ttbr1_el1: Ttbr1El1::empty(),
            #[cfg(feature = "el2")]
            ttbr1_el2: Ttbr1El2::empty(),
            #[cfg(feature = "el1")]
            vbar_el1: VbarEl1::empty(),
            #[cfg(feature = "el2")]
            vbar_el2: VbarEl2::empty(),
            #[cfg(feature = "el2")]
            vdisr_el2: VdisrEl2::empty(),
            #[cfg(feature = "el2")]
            vmpidr_el2: VmpidrEl2::empty(),
            #[cfg(feature = "el2")]
            vpidr_el2: VpidrEl2::empty(),
            #[cfg(feature = "el2")]
            vsesr_el2: VsesrEl2::empty(),
            #[cfg(feature = "el2")]
            vtcr_el2: VtcrEl2::empty(),
            #[cfg(feature = "el2")]
            vttbr_el2: VttbrEl2::empty(),
            #[cfg(feature = "el3")]
            zcr_el3: ZcrEl3::empty(),
        }
    }
}
