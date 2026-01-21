// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    Amcfgr, Amcgcr, Amcntenclr0, Amcntenclr1, Amcntenset0, Amcntenset1, Amcr, Amuserenr, Ccsidr,
    Ccsidr2, Clidr, Cntfrq, CntfrqEl0, Cnthctl, CnthpCtl, CnthpCval, CnthpTval, CnthpsCtl,
    CnthpsCval, CnthpsTval, CnthvCtl, CnthvCval, CnthvTval, CnthvsCtl, CnthvsCval, CnthvsTval,
    Cntkctl, CntpCtl, CntpCval, CntpTval, Cntpct, CntpctEl0, Cntpctss, CntvCtl, CntvCval, CntvTval,
    Cntvct, Cntvctss, Cntvoff, Contextidr, Cpacr, Csselr, Ctr, CtrEl0, Currentel, Dacr,
    Dbgauthstatus, Dbgclaimclr, Dbgclaimset, Dbgdccint, Dbgdevid, Dbgdevid1, Dbgdidr, Dbgdrar,
    Dbgdscrext, Dbgdscrint, Dbgdtrrxext, Dbgdtrrxint, Dbgdtrtxext, Dbgdtrtxint, Dbgosdlr,
    Dbgoseccr, Dbgoslar, Dbgoslsr, Dbgprcr, Dbgvcr, Dfar, Dfsr, Disr, Dit, Dlr, Dspsr, Dspsr2,
    Erridr, Errselr, Erxaddr, Erxaddr2, Erxctlr, Erxctlr2, Erxfr, Erxfr2, Erxmisc0, Erxmisc1,
    Erxmisc2, Erxmisc3, Erxmisc4, Erxmisc5, Erxmisc6, Erxmisc7, Erxstatus, Hcptr, Hcr, Hcr2, Hdcr,
    Hdfar, Hifar, Hmair0, Hmair1, Hpfar, Hrmr, Hsctlr, Hsr, Htcr, Htpidr, Htrfcr, Httbr, Hvbar,
    IdDfr0, IdDfr1, IdIsar0, IdIsar1, IdIsar2, IdIsar3, IdIsar4, IdIsar5, IdIsar6, IdMmfr0,
    IdMmfr1, IdMmfr2, IdMmfr3, IdMmfr4, IdMmfr5, IdPfr0, IdPfr1, IdPfr2, Ifar, Ifsr, Isr, Mair0,
    Mair1, Midr, Mpidr, Mvbar, Nmrr, Nsacr, Par, Pmccfiltr, Pmccntr, Pmceid0, Pmceid1, Pmceid2,
    Pmceid3, Pmcntenclr, Pmcntenset, Pmcr, PmcrEl0, Pmintenclr, Pmintenset, Pmmir, Pmovsr,
    Pmovsset, Pmselr, Pmswinc, Pmuserenr, Pmxevtyper, Prrr, Rmr, Rvbar, Scr, Sctlr, Sdcr, Sder,
    Tlbtr, TpidrEl0, Tpidrprw, TpidrroEl0, Tpidruro, Tpidrurw, Trfcr, Ttbcr, Ttbcr2, Ttbr0, Ttbr1,
    Vbar, Vdfsr, Vdisr, Vmpidr, Vpidr, Vtcr, Vttbr,
};
#[cfg(feature = "el1")]
use crate::{
    ApiakeyhiEl1, ApiakeyloEl1, CcsidrEl1, ClidrEl1, ContextidrEl1, CpacrEl1, CsselrEl1, DisrEl1,
    ElrEl1, EsrEl1, FarEl1, GcrEl1, GcscrEl1, IccSreEl1, IdAa64dfr0El1, IdAa64dfr1El1,
    IdAa64isar1El1, IdAa64isar2El1, IdAa64mmfr0El1, IdAa64mmfr1El1, IdAa64mmfr2El1, IdAa64mmfr3El1,
    IdAa64pfr0El1, IdAa64pfr1El1, IdAa64smfr0El1, IsrEl1, MairEl1, MdccintEl1, MdscrEl1, MidrEl1,
    MpamidrEl1, MpidrEl1, ParEl1, RgsrEl1, SctlrEl1, SpEl1, SpsrEl1, Tcr2El1, TcrEl1, TfsrEl1,
    Tfsre0El1, TpidrEl1, Ttbr0El1, Ttbr1El1, VbarEl1,
};
#[cfg(feature = "el2")]
use crate::{
    CnthctlEl2, CntvoffEl2, ContextidrEl2, CptrEl2, ElrEl2, ElrHyp, EsrEl2, FarEl2, GcscrEl2,
    HcrEl2, HcrxEl2, Hdfgrtr2El2, Hdfgwtr2El2, Hfgitr2El2, Hfgrtr2El2, Hfgwtr2El2, HfgwtrEl2,
    HpfarEl2, IccSreEl2, IchHcrEl2, IchVmcrEl2, MairEl2, MdcrEl2, Mpam2El2, MpamhcrEl2,
    Mpamvpm0El2, Mpamvpm1El2, Mpamvpm2El2, Mpamvpm3El2, Mpamvpm4El2, Mpamvpm5El2, Mpamvpm6El2,
    Mpamvpm7El2, MpamvpmvEl2, SctlrEl2, SpEl2, SpsrEl2, Tcr2El2, TcrEl2, TfsrEl2, TpidrEl2,
    Ttbr0El2, Ttbr1El2, VbarEl2, VdisrEl2, VmpidrEl2, VpidrEl2, VsesrEl2, VtcrEl2, VttbrEl2,
};
#[cfg(feature = "el3")]
use crate::{
    CptrEl3, EsrEl3, GpccrEl3, GptbrEl3, IccSreEl3, MairEl3, MdcrEl3, Mpam3El3, ScrEl3, Sctlr2El3,
    SctlrEl3, SmcrEl3, SpsrEl3, TcrEl3, Ttbr0El3, ZcrEl3,
};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SystemRegisters {
    /// Fake value for the `ACTLR` system register.
    pub actlr: u32,
    /// Fake value for the `ACTLR2` system register.
    pub actlr2: u32,
    #[cfg(feature = "el1")]
    /// Fake value for the `ACTLR_EL1` system register.
    pub actlr_el1: u64,
    #[cfg(feature = "el2")]
    /// Fake value for the `ACTLR_EL2` system register.
    pub actlr_el2: u64,
    /// Fake value for the `ADFSR` system register.
    pub adfsr: u32,
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
    /// Fake value for the `AIDR` system register.
    pub aidr: u32,
    /// Fake value for the `AIFSR` system register.
    pub aifsr: u32,
    /// Fake value for the `AMAIR0` system register.
    pub amair0: u32,
    /// Fake value for the `AMAIR1` system register.
    pub amair1: u32,
    #[cfg(feature = "el1")]
    /// Fake value for the `AMAIR_EL1` system register.
    pub amair_el1: u64,
    #[cfg(feature = "el2")]
    /// Fake value for the `AMAIR_EL2` system register.
    pub amair_el2: u64,
    /// Fake value for the `AMCFGR` system register.
    pub amcfgr: Amcfgr,
    /// Fake value for the `AMCGCR` system register.
    pub amcgcr: Amcgcr,
    /// Fake value for the `AMCNTENCLR0` system register.
    pub amcntenclr0: Amcntenclr0,
    /// Fake value for the `AMCNTENCLR1` system register.
    pub amcntenclr1: Amcntenclr1,
    /// Fake value for the `AMCNTENSET0` system register.
    pub amcntenset0: Amcntenset0,
    /// Fake value for the `AMCNTENSET1` system register.
    pub amcntenset1: Amcntenset1,
    /// Fake value for the `AMCR` system register.
    pub amcr: Amcr,
    /// Fake value for the `AMUSERENR` system register.
    pub amuserenr: Amuserenr,
    #[cfg(feature = "el1")]
    /// Fake value for the `APIAKeyHi_EL1` system register.
    pub apiakeyhi_el1: ApiakeyhiEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `APIAKeyLo_EL1` system register.
    pub apiakeylo_el1: ApiakeyloEl1,
    /// Fake value for the `CCSIDR` system register.
    pub ccsidr: Ccsidr,
    /// Fake value for the `CCSIDR2` system register.
    pub ccsidr2: Ccsidr2,
    #[cfg(feature = "el1")]
    /// Fake value for the `CCSIDR_EL1` system register.
    pub ccsidr_el1: CcsidrEl1,
    /// Fake value for the `CLIDR` system register.
    pub clidr: Clidr,
    #[cfg(feature = "el1")]
    /// Fake value for the `CLIDR_EL1` system register.
    pub clidr_el1: ClidrEl1,
    /// Fake value for the `CNTFRQ` system register.
    pub cntfrq: Cntfrq,
    /// Fake value for the `CNTFRQ_EL0` system register.
    pub cntfrq_el0: CntfrqEl0,
    /// Fake value for the `CNTHCTL` system register.
    pub cnthctl: Cnthctl,
    #[cfg(feature = "el2")]
    /// Fake value for the `CNTHCTL_EL2` system register.
    pub cnthctl_el2: CnthctlEl2,
    /// Fake value for the `CNTHPS_CTL` system register.
    pub cnthps_ctl: CnthpsCtl,
    /// Fake value for the `CNTHPS_CVAL` system register.
    pub cnthps_cval: CnthpsCval,
    /// Fake value for the `CNTHPS_TVAL` system register.
    pub cnthps_tval: CnthpsTval,
    /// Fake value for the `CNTHP_CTL` system register.
    pub cnthp_ctl: CnthpCtl,
    /// Fake value for the `CNTHP_CVAL` system register.
    pub cnthp_cval: CnthpCval,
    /// Fake value for the `CNTHP_TVAL` system register.
    pub cnthp_tval: CnthpTval,
    /// Fake value for the `CNTHVS_CTL` system register.
    pub cnthvs_ctl: CnthvsCtl,
    /// Fake value for the `CNTHVS_CVAL` system register.
    pub cnthvs_cval: CnthvsCval,
    /// Fake value for the `CNTHVS_TVAL` system register.
    pub cnthvs_tval: CnthvsTval,
    /// Fake value for the `CNTHV_CTL` system register.
    pub cnthv_ctl: CnthvCtl,
    /// Fake value for the `CNTHV_CVAL` system register.
    pub cnthv_cval: CnthvCval,
    /// Fake value for the `CNTHV_TVAL` system register.
    pub cnthv_tval: CnthvTval,
    /// Fake value for the `CNTKCTL` system register.
    pub cntkctl: Cntkctl,
    /// Fake value for the `CNTPCT` system register.
    pub cntpct: Cntpct,
    /// Fake value for the `CNTPCTSS` system register.
    pub cntpctss: Cntpctss,
    /// Fake value for the `CNTPCT_EL0` system register.
    pub cntpct_el0: CntpctEl0,
    /// Fake value for the `CNTP_CTL` system register.
    pub cntp_ctl: CntpCtl,
    /// Fake value for the `CNTP_CVAL` system register.
    pub cntp_cval: CntpCval,
    /// Fake value for the `CNTP_TVAL` system register.
    pub cntp_tval: CntpTval,
    /// Fake value for the `CNTVCT` system register.
    pub cntvct: Cntvct,
    /// Fake value for the `CNTVCTSS` system register.
    pub cntvctss: Cntvctss,
    /// Fake value for the `CNTVOFF` system register.
    pub cntvoff: Cntvoff,
    #[cfg(feature = "el2")]
    /// Fake value for the `CNTVOFF_EL2` system register.
    pub cntvoff_el2: CntvoffEl2,
    /// Fake value for the `CNTV_CTL` system register.
    pub cntv_ctl: CntvCtl,
    /// Fake value for the `CNTV_CVAL` system register.
    pub cntv_cval: CntvCval,
    /// Fake value for the `CNTV_TVAL` system register.
    pub cntv_tval: CntvTval,
    /// Fake value for the `CONTEXTIDR` system register.
    pub contextidr: Contextidr,
    #[cfg(feature = "el1")]
    /// Fake value for the `CONTEXTIDR_EL1` system register.
    pub contextidr_el1: ContextidrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `CONTEXTIDR_EL2` system register.
    pub contextidr_el2: ContextidrEl2,
    /// Fake value for the `CPACR` system register.
    pub cpacr: Cpacr,
    #[cfg(feature = "el1")]
    /// Fake value for the `CPACR_EL1` system register.
    pub cpacr_el1: CpacrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `CPTR_EL2` system register.
    pub cptr_el2: CptrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `CPTR_EL3` system register.
    pub cptr_el3: CptrEl3,
    /// Fake value for the `CSSELR` system register.
    pub csselr: Csselr,
    #[cfg(feature = "el1")]
    /// Fake value for the `CSSELR_EL1` system register.
    pub csselr_el1: CsselrEl1,
    /// Fake value for the `CTR` system register.
    pub ctr: Ctr,
    /// Fake value for the `CTR_EL0` system register.
    pub ctr_el0: CtrEl0,
    /// Fake value for the `CurrentEL` system register.
    pub currentel: Currentel,
    /// Fake value for the `DACR` system register.
    pub dacr: Dacr,
    /// Fake value for the `DBGAUTHSTATUS` system register.
    pub dbgauthstatus: Dbgauthstatus,
    /// Fake value for the `DBGCLAIMCLR` system register.
    pub dbgclaimclr: Dbgclaimclr,
    /// Fake value for the `DBGCLAIMSET` system register.
    pub dbgclaimset: Dbgclaimset,
    /// Fake value for the `DBGDCCINT` system register.
    pub dbgdccint: Dbgdccint,
    /// Fake value for the `DBGDEVID` system register.
    pub dbgdevid: Dbgdevid,
    /// Fake value for the `DBGDEVID1` system register.
    pub dbgdevid1: Dbgdevid1,
    /// Fake value for the `DBGDEVID2` system register.
    pub dbgdevid2: u32,
    /// Fake value for the `DBGDIDR` system register.
    pub dbgdidr: Dbgdidr,
    /// Fake value for the `DBGDRAR` system register.
    pub dbgdrar: Dbgdrar,
    /// Fake value for the `DBGDSAR` system register.
    pub dbgdsar: u64,
    /// Fake value for the `DBGDSCRext` system register.
    pub dbgdscrext: Dbgdscrext,
    /// Fake value for the `DBGDSCRint` system register.
    pub dbgdscrint: Dbgdscrint,
    /// Fake value for the `DBGDTRRXext` system register.
    pub dbgdtrrxext: Dbgdtrrxext,
    /// Fake value for the `DBGDTRRXint` system register.
    pub dbgdtrrxint: Dbgdtrrxint,
    /// Fake value for the `DBGDTRTXext` system register.
    pub dbgdtrtxext: Dbgdtrtxext,
    /// Fake value for the `DBGDTRTXint` system register.
    pub dbgdtrtxint: Dbgdtrtxint,
    /// Fake value for the `DBGOSDLR` system register.
    pub dbgosdlr: Dbgosdlr,
    /// Fake value for the `DBGOSECCR` system register.
    pub dbgoseccr: Dbgoseccr,
    /// Fake value for the `DBGOSLAR` system register.
    pub dbgoslar: Dbgoslar,
    /// Fake value for the `DBGOSLSR` system register.
    pub dbgoslsr: Dbgoslsr,
    /// Fake value for the `DBGPRCR` system register.
    pub dbgprcr: Dbgprcr,
    /// Fake value for the `DBGVCR` system register.
    pub dbgvcr: Dbgvcr,
    /// Fake value for the `DBGWFAR` system register.
    pub dbgwfar: u32,
    /// Fake value for the `DFAR` system register.
    pub dfar: Dfar,
    /// Fake value for the `DFSR` system register.
    pub dfsr: Dfsr,
    /// Fake value for the `DISR` system register.
    pub disr: Disr,
    #[cfg(feature = "el1")]
    /// Fake value for the `DISR_EL1` system register.
    pub disr_el1: DisrEl1,
    /// Fake value for the `DIT` system register.
    pub dit: Dit,
    /// Fake value for the `DLR` system register.
    pub dlr: Dlr,
    /// Fake value for the `DSPSR` system register.
    pub dspsr: Dspsr,
    /// Fake value for the `DSPSR2` system register.
    pub dspsr2: Dspsr2,
    #[cfg(feature = "el1")]
    /// Fake value for the `ELR_EL1` system register.
    pub elr_el1: ElrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `ELR_EL2` system register.
    pub elr_el2: ElrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `ELR_hyp` system register.
    pub elr_hyp: ElrHyp,
    /// Fake value for the `ERRIDR` system register.
    pub erridr: Erridr,
    /// Fake value for the `ERRSELR` system register.
    pub errselr: Errselr,
    /// Fake value for the `ERXADDR` system register.
    pub erxaddr: Erxaddr,
    /// Fake value for the `ERXADDR2` system register.
    pub erxaddr2: Erxaddr2,
    /// Fake value for the `ERXCTLR` system register.
    pub erxctlr: Erxctlr,
    /// Fake value for the `ERXCTLR2` system register.
    pub erxctlr2: Erxctlr2,
    /// Fake value for the `ERXFR` system register.
    pub erxfr: Erxfr,
    /// Fake value for the `ERXFR2` system register.
    pub erxfr2: Erxfr2,
    /// Fake value for the `ERXMISC0` system register.
    pub erxmisc0: Erxmisc0,
    /// Fake value for the `ERXMISC1` system register.
    pub erxmisc1: Erxmisc1,
    /// Fake value for the `ERXMISC2` system register.
    pub erxmisc2: Erxmisc2,
    /// Fake value for the `ERXMISC3` system register.
    pub erxmisc3: Erxmisc3,
    /// Fake value for the `ERXMISC4` system register.
    pub erxmisc4: Erxmisc4,
    /// Fake value for the `ERXMISC5` system register.
    pub erxmisc5: Erxmisc5,
    /// Fake value for the `ERXMISC6` system register.
    pub erxmisc6: Erxmisc6,
    /// Fake value for the `ERXMISC7` system register.
    pub erxmisc7: Erxmisc7,
    /// Fake value for the `ERXSTATUS` system register.
    pub erxstatus: Erxstatus,
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
    /// Fake value for the `FCSEIDR` system register.
    pub fcseidr: u32,
    #[cfg(feature = "el1")]
    /// Fake value for the `GCR_EL1` system register.
    pub gcr_el1: GcrEl1,
    #[cfg(feature = "el1")]
    /// Fake value for the `GCSCR_EL1` system register.
    pub gcscr_el1: GcscrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `GCSCR_EL2` system register.
    pub gcscr_el2: GcscrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `GPCCR_EL3` system register.
    pub gpccr_el3: GpccrEl3,
    #[cfg(feature = "el3")]
    /// Fake value for the `GPTBR_EL3` system register.
    pub gptbr_el3: GptbrEl3,
    /// Fake value for the `HACR` system register.
    pub hacr: u32,
    #[cfg(feature = "el2")]
    /// Fake value for the `HACR_EL2` system register.
    pub hacr_el2: u64,
    /// Fake value for the `HACTLR` system register.
    pub hactlr: u32,
    /// Fake value for the `HACTLR2` system register.
    pub hactlr2: u32,
    /// Fake value for the `HADFSR` system register.
    pub hadfsr: u32,
    /// Fake value for the `HAIFSR` system register.
    pub haifsr: u32,
    /// Fake value for the `HAMAIR0` system register.
    pub hamair0: u32,
    /// Fake value for the `HAMAIR1` system register.
    pub hamair1: u32,
    /// Fake value for the `HCPTR` system register.
    pub hcptr: Hcptr,
    /// Fake value for the `HCR` system register.
    pub hcr: Hcr,
    /// Fake value for the `HCR2` system register.
    pub hcr2: Hcr2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HCRX_EL2` system register.
    pub hcrx_el2: HcrxEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HCR_EL2` system register.
    pub hcr_el2: HcrEl2,
    /// Fake value for the `HDCR` system register.
    pub hdcr: Hdcr,
    /// Fake value for the `HDFAR` system register.
    pub hdfar: Hdfar,
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
    /// Fake value for the `HFGWTR2_EL2` system register.
    pub hfgwtr2_el2: Hfgwtr2El2,
    #[cfg(feature = "el2")]
    /// Fake value for the `HFGWTR_EL2` system register.
    pub hfgwtr_el2: HfgwtrEl2,
    /// Fake value for the `HIFAR` system register.
    pub hifar: Hifar,
    /// Fake value for the `HMAIR0` system register.
    pub hmair0: Hmair0,
    /// Fake value for the `HMAIR1` system register.
    pub hmair1: Hmair1,
    /// Fake value for the `HPFAR` system register.
    pub hpfar: Hpfar,
    #[cfg(feature = "el2")]
    /// Fake value for the `HPFAR_EL2` system register.
    pub hpfar_el2: HpfarEl2,
    /// Fake value for the `HRMR` system register.
    pub hrmr: Hrmr,
    /// Fake value for the `HSCTLR` system register.
    pub hsctlr: Hsctlr,
    /// Fake value for the `HSR` system register.
    pub hsr: Hsr,
    /// Fake value for the `HSTR` system register.
    pub hstr: u32,
    #[cfg(feature = "el2")]
    /// Fake value for the `HSTR_EL2` system register.
    pub hstr_el2: u64,
    /// Fake value for the `HTCR` system register.
    pub htcr: Htcr,
    /// Fake value for the `HTPIDR` system register.
    pub htpidr: Htpidr,
    /// Fake value for the `HTRFCR` system register.
    pub htrfcr: Htrfcr,
    /// Fake value for the `HTTBR` system register.
    pub httbr: Httbr,
    /// Fake value for the `HVBAR` system register.
    pub hvbar: Hvbar,
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
    /// Fake value for the `ID_AA64ISAR1_EL1` system register.
    pub id_aa64isar1_el1: IdAa64isar1El1,
    #[cfg(feature = "el1")]
    /// Fake value for the `ID_AA64ISAR2_EL1` system register.
    pub id_aa64isar2_el1: IdAa64isar2El1,
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
    /// Fake value for the `ID_AA64SMFR0_EL1` system register.
    pub id_aa64smfr0_el1: IdAa64smfr0El1,
    /// Fake value for the `ID_AFR0` system register.
    pub id_afr0: u32,
    /// Fake value for the `ID_DFR0` system register.
    pub id_dfr0: IdDfr0,
    /// Fake value for the `ID_DFR1` system register.
    pub id_dfr1: IdDfr1,
    /// Fake value for the `ID_ISAR0` system register.
    pub id_isar0: IdIsar0,
    /// Fake value for the `ID_ISAR1` system register.
    pub id_isar1: IdIsar1,
    /// Fake value for the `ID_ISAR2` system register.
    pub id_isar2: IdIsar2,
    /// Fake value for the `ID_ISAR3` system register.
    pub id_isar3: IdIsar3,
    /// Fake value for the `ID_ISAR4` system register.
    pub id_isar4: IdIsar4,
    /// Fake value for the `ID_ISAR5` system register.
    pub id_isar5: IdIsar5,
    /// Fake value for the `ID_ISAR6` system register.
    pub id_isar6: IdIsar6,
    /// Fake value for the `ID_MMFR0` system register.
    pub id_mmfr0: IdMmfr0,
    /// Fake value for the `ID_MMFR1` system register.
    pub id_mmfr1: IdMmfr1,
    /// Fake value for the `ID_MMFR2` system register.
    pub id_mmfr2: IdMmfr2,
    /// Fake value for the `ID_MMFR3` system register.
    pub id_mmfr3: IdMmfr3,
    /// Fake value for the `ID_MMFR4` system register.
    pub id_mmfr4: IdMmfr4,
    /// Fake value for the `ID_MMFR5` system register.
    pub id_mmfr5: IdMmfr5,
    /// Fake value for the `ID_PFR0` system register.
    pub id_pfr0: IdPfr0,
    /// Fake value for the `ID_PFR1` system register.
    pub id_pfr1: IdPfr1,
    /// Fake value for the `ID_PFR2` system register.
    pub id_pfr2: IdPfr2,
    /// Fake value for the `IFAR` system register.
    pub ifar: Ifar,
    /// Fake value for the `IFSR` system register.
    pub ifsr: Ifsr,
    /// Fake value for the `ISR` system register.
    pub isr: Isr,
    #[cfg(feature = "el1")]
    /// Fake value for the `ISR_EL1` system register.
    pub isr_el1: IsrEl1,
    /// Fake value for the `JIDR` system register.
    pub jidr: u32,
    /// Fake value for the `JMCR` system register.
    pub jmcr: u32,
    /// Fake value for the `JOSCR` system register.
    pub joscr: u32,
    /// Fake value for the `MAIR0` system register.
    pub mair0: Mair0,
    /// Fake value for the `MAIR1` system register.
    pub mair1: Mair1,
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
    /// Fake value for the `MIDR` system register.
    pub midr: Midr,
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
    /// Fake value for the `MPIDR` system register.
    pub mpidr: Mpidr,
    #[cfg(feature = "el1")]
    /// Fake value for the `MPIDR_EL1` system register.
    pub mpidr_el1: MpidrEl1,
    /// Fake value for the `MVBAR` system register.
    pub mvbar: Mvbar,
    /// Fake value for the `NMRR` system register.
    pub nmrr: Nmrr,
    /// Fake value for the `NSACR` system register.
    pub nsacr: Nsacr,
    /// Fake value for the `PAR` system register.
    pub par: Par,
    #[cfg(feature = "el1")]
    /// Fake value for the `PAR_EL1` system register.
    pub par_el1: ParEl1,
    /// Fake value for the `PMCCFILTR` system register.
    pub pmccfiltr: Pmccfiltr,
    /// Fake value for the `PMCCNTR` system register.
    pub pmccntr: Pmccntr,
    /// Fake value for the `PMCEID0` system register.
    pub pmceid0: Pmceid0,
    /// Fake value for the `PMCEID1` system register.
    pub pmceid1: Pmceid1,
    /// Fake value for the `PMCEID2` system register.
    pub pmceid2: Pmceid2,
    /// Fake value for the `PMCEID3` system register.
    pub pmceid3: Pmceid3,
    /// Fake value for the `PMCNTENCLR` system register.
    pub pmcntenclr: Pmcntenclr,
    /// Fake value for the `PMCNTENSET` system register.
    pub pmcntenset: Pmcntenset,
    /// Fake value for the `PMCR` system register.
    pub pmcr: Pmcr,
    /// Fake value for the `PMCR_EL0` system register.
    pub pmcr_el0: PmcrEl0,
    /// Fake value for the `PMINTENCLR` system register.
    pub pmintenclr: Pmintenclr,
    /// Fake value for the `PMINTENSET` system register.
    pub pmintenset: Pmintenset,
    /// Fake value for the `PMMIR` system register.
    pub pmmir: Pmmir,
    /// Fake value for the `PMOVSR` system register.
    pub pmovsr: Pmovsr,
    /// Fake value for the `PMOVSSET` system register.
    pub pmovsset: Pmovsset,
    /// Fake value for the `PMSELR` system register.
    pub pmselr: Pmselr,
    /// Fake value for the `PMSWINC` system register.
    pub pmswinc: Pmswinc,
    /// Fake value for the `PMUSERENR` system register.
    pub pmuserenr: Pmuserenr,
    /// Fake value for the `PMXEVTYPER` system register.
    pub pmxevtyper: Pmxevtyper,
    /// Fake value for the `PRRR` system register.
    pub prrr: Prrr,
    /// Fake value for the `REVIDR` system register.
    pub revidr: u32,
    #[cfg(feature = "el1")]
    /// Fake value for the `RGSR_EL1` system register.
    pub rgsr_el1: RgsrEl1,
    /// Fake value for the `RMR` system register.
    pub rmr: Rmr,
    /// Fake value for the `RVBAR` system register.
    pub rvbar: Rvbar,
    /// Fake value for the `SCR` system register.
    pub scr: Scr,
    #[cfg(feature = "el3")]
    /// Fake value for the `SCR_EL3` system register.
    pub scr_el3: ScrEl3,
    /// Fake value for the `SCTLR` system register.
    pub sctlr: Sctlr,
    #[cfg(feature = "el3")]
    /// Fake value for the `SCTLR2_EL3` system register.
    pub sctlr2_el3: Sctlr2El3,
    #[cfg(feature = "el1")]
    /// Fake value for the `SCTLR_EL1` system register.
    pub sctlr_el1: SctlrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `SCTLR_EL2` system register.
    pub sctlr_el2: SctlrEl2,
    #[cfg(feature = "el3")]
    /// Fake value for the `SCTLR_EL3` system register.
    pub sctlr_el3: SctlrEl3,
    /// Fake value for the `SDCR` system register.
    pub sdcr: Sdcr,
    /// Fake value for the `SDER` system register.
    pub sder: Sder,
    #[cfg(feature = "el3")]
    /// Fake value for the `SMCR_EL3` system register.
    pub smcr_el3: SmcrEl3,
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
    /// Fake value for the `TCMTR` system register.
    pub tcmtr: u32,
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
    /// Fake value for the `TLBTR` system register.
    pub tlbtr: Tlbtr,
    /// Fake value for the `TPIDRPRW` system register.
    pub tpidrprw: Tpidrprw,
    /// Fake value for the `TPIDRRO_EL0` system register.
    pub tpidrro_el0: TpidrroEl0,
    /// Fake value for the `TPIDRURO` system register.
    pub tpidruro: Tpidruro,
    /// Fake value for the `TPIDRURW` system register.
    pub tpidrurw: Tpidrurw,
    /// Fake value for the `TPIDR_EL0` system register.
    pub tpidr_el0: TpidrEl0,
    #[cfg(feature = "el1")]
    /// Fake value for the `TPIDR_EL1` system register.
    pub tpidr_el1: TpidrEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TPIDR_EL2` system register.
    pub tpidr_el2: TpidrEl2,
    /// Fake value for the `TRFCR` system register.
    pub trfcr: Trfcr,
    /// Fake value for the `TTBCR` system register.
    pub ttbcr: Ttbcr,
    /// Fake value for the `TTBCR2` system register.
    pub ttbcr2: Ttbcr2,
    /// Fake value for the `TTBR0` system register.
    pub ttbr0: Ttbr0,
    #[cfg(feature = "el1")]
    /// Fake value for the `TTBR0_EL1` system register.
    pub ttbr0_el1: Ttbr0El1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TTBR0_EL2` system register.
    pub ttbr0_el2: Ttbr0El2,
    #[cfg(feature = "el3")]
    /// Fake value for the `TTBR0_EL3` system register.
    pub ttbr0_el3: Ttbr0El3,
    /// Fake value for the `TTBR1` system register.
    pub ttbr1: Ttbr1,
    #[cfg(feature = "el1")]
    /// Fake value for the `TTBR1_EL1` system register.
    pub ttbr1_el1: Ttbr1El1,
    #[cfg(feature = "el2")]
    /// Fake value for the `TTBR1_EL2` system register.
    pub ttbr1_el2: Ttbr1El2,
    /// Fake value for the `VBAR` system register.
    pub vbar: Vbar,
    #[cfg(feature = "el1")]
    /// Fake value for the `VBAR_EL1` system register.
    pub vbar_el1: VbarEl1,
    #[cfg(feature = "el2")]
    /// Fake value for the `VBAR_EL2` system register.
    pub vbar_el2: VbarEl2,
    /// Fake value for the `VDFSR` system register.
    pub vdfsr: Vdfsr,
    /// Fake value for the `VDISR` system register.
    pub vdisr: Vdisr,
    #[cfg(feature = "el2")]
    /// Fake value for the `VDISR_EL2` system register.
    pub vdisr_el2: VdisrEl2,
    /// Fake value for the `VMPIDR` system register.
    pub vmpidr: Vmpidr,
    #[cfg(feature = "el2")]
    /// Fake value for the `VMPIDR_EL2` system register.
    pub vmpidr_el2: VmpidrEl2,
    /// Fake value for the `VPIDR` system register.
    pub vpidr: Vpidr,
    #[cfg(feature = "el2")]
    /// Fake value for the `VPIDR_EL2` system register.
    pub vpidr_el2: VpidrEl2,
    #[cfg(feature = "el2")]
    /// Fake value for the `VSESR_EL2` system register.
    pub vsesr_el2: VsesrEl2,
    /// Fake value for the `VTCR` system register.
    pub vtcr: Vtcr,
    #[cfg(feature = "el2")]
    /// Fake value for the `VTCR_EL2` system register.
    pub vtcr_el2: VtcrEl2,
    /// Fake value for the `VTTBR` system register.
    pub vttbr: Vttbr,
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
            actlr: 0,
            actlr2: 0,
            #[cfg(feature = "el1")]
            actlr_el1: 0,
            #[cfg(feature = "el2")]
            actlr_el2: 0,
            adfsr: 0,
            #[cfg(feature = "el1")]
            afsr0_el1: 0,
            #[cfg(feature = "el2")]
            afsr0_el2: 0,
            #[cfg(feature = "el1")]
            afsr1_el1: 0,
            #[cfg(feature = "el2")]
            afsr1_el2: 0,
            aidr: 0,
            aifsr: 0,
            amair0: 0,
            amair1: 0,
            #[cfg(feature = "el1")]
            amair_el1: 0,
            #[cfg(feature = "el2")]
            amair_el2: 0,
            amcfgr: Amcfgr::empty(),
            amcgcr: Amcgcr::empty(),
            amcntenclr0: Amcntenclr0::empty(),
            amcntenclr1: Amcntenclr1::empty(),
            amcntenset0: Amcntenset0::empty(),
            amcntenset1: Amcntenset1::empty(),
            amcr: Amcr::empty(),
            amuserenr: Amuserenr::empty(),
            #[cfg(feature = "el1")]
            apiakeyhi_el1: ApiakeyhiEl1::empty(),
            #[cfg(feature = "el1")]
            apiakeylo_el1: ApiakeyloEl1::empty(),
            ccsidr: Ccsidr::empty(),
            ccsidr2: Ccsidr2::empty(),
            #[cfg(feature = "el1")]
            ccsidr_el1: CcsidrEl1::empty(),
            clidr: Clidr::empty(),
            #[cfg(feature = "el1")]
            clidr_el1: ClidrEl1::empty(),
            cntfrq: Cntfrq::empty(),
            cntfrq_el0: CntfrqEl0::empty(),
            cnthctl: Cnthctl::empty(),
            #[cfg(feature = "el2")]
            cnthctl_el2: CnthctlEl2::empty(),
            cnthps_ctl: CnthpsCtl::empty(),
            cnthps_cval: CnthpsCval::empty(),
            cnthps_tval: CnthpsTval::empty(),
            cnthp_ctl: CnthpCtl::empty(),
            cnthp_cval: CnthpCval::empty(),
            cnthp_tval: CnthpTval::empty(),
            cnthvs_ctl: CnthvsCtl::empty(),
            cnthvs_cval: CnthvsCval::empty(),
            cnthvs_tval: CnthvsTval::empty(),
            cnthv_ctl: CnthvCtl::empty(),
            cnthv_cval: CnthvCval::empty(),
            cnthv_tval: CnthvTval::empty(),
            cntkctl: Cntkctl::empty(),
            cntpct: Cntpct::empty(),
            cntpctss: Cntpctss::empty(),
            cntpct_el0: CntpctEl0::empty(),
            cntp_ctl: CntpCtl::empty(),
            cntp_cval: CntpCval::empty(),
            cntp_tval: CntpTval::empty(),
            cntvct: Cntvct::empty(),
            cntvctss: Cntvctss::empty(),
            cntvoff: Cntvoff::empty(),
            #[cfg(feature = "el2")]
            cntvoff_el2: CntvoffEl2::empty(),
            cntv_ctl: CntvCtl::empty(),
            cntv_cval: CntvCval::empty(),
            cntv_tval: CntvTval::empty(),
            contextidr: Contextidr::empty(),
            #[cfg(feature = "el1")]
            contextidr_el1: ContextidrEl1::empty(),
            #[cfg(feature = "el2")]
            contextidr_el2: ContextidrEl2::empty(),
            cpacr: Cpacr::empty(),
            #[cfg(feature = "el1")]
            cpacr_el1: CpacrEl1::empty(),
            #[cfg(feature = "el2")]
            cptr_el2: CptrEl2::empty(),
            #[cfg(feature = "el3")]
            cptr_el3: CptrEl3::empty(),
            csselr: Csselr::empty(),
            #[cfg(feature = "el1")]
            csselr_el1: CsselrEl1::empty(),
            ctr: Ctr::empty(),
            ctr_el0: CtrEl0::empty(),
            currentel: Currentel::empty(),
            dacr: Dacr::empty(),
            dbgauthstatus: Dbgauthstatus::empty(),
            dbgclaimclr: Dbgclaimclr::empty(),
            dbgclaimset: Dbgclaimset::empty(),
            dbgdccint: Dbgdccint::empty(),
            dbgdevid: Dbgdevid::empty(),
            dbgdevid1: Dbgdevid1::empty(),
            dbgdevid2: 0,
            dbgdidr: Dbgdidr::empty(),
            dbgdrar: Dbgdrar::empty(),
            dbgdsar: 0,
            dbgdscrext: Dbgdscrext::empty(),
            dbgdscrint: Dbgdscrint::empty(),
            dbgdtrrxext: Dbgdtrrxext::empty(),
            dbgdtrrxint: Dbgdtrrxint::empty(),
            dbgdtrtxext: Dbgdtrtxext::empty(),
            dbgdtrtxint: Dbgdtrtxint::empty(),
            dbgosdlr: Dbgosdlr::empty(),
            dbgoseccr: Dbgoseccr::empty(),
            dbgoslar: Dbgoslar::empty(),
            dbgoslsr: Dbgoslsr::empty(),
            dbgprcr: Dbgprcr::empty(),
            dbgvcr: Dbgvcr::empty(),
            dbgwfar: 0,
            dfar: Dfar::empty(),
            dfsr: Dfsr::empty(),
            disr: Disr::empty(),
            #[cfg(feature = "el1")]
            disr_el1: DisrEl1::empty(),
            dit: Dit::empty(),
            dlr: Dlr::empty(),
            dspsr: Dspsr::empty(),
            dspsr2: Dspsr2::empty(),
            #[cfg(feature = "el1")]
            elr_el1: ElrEl1::empty(),
            #[cfg(feature = "el2")]
            elr_el2: ElrEl2::empty(),
            #[cfg(feature = "el2")]
            elr_hyp: ElrHyp::empty(),
            erridr: Erridr::empty(),
            errselr: Errselr::empty(),
            erxaddr: Erxaddr::empty(),
            erxaddr2: Erxaddr2::empty(),
            erxctlr: Erxctlr::empty(),
            erxctlr2: Erxctlr2::empty(),
            erxfr: Erxfr::empty(),
            erxfr2: Erxfr2::empty(),
            erxmisc0: Erxmisc0::empty(),
            erxmisc1: Erxmisc1::empty(),
            erxmisc2: Erxmisc2::empty(),
            erxmisc3: Erxmisc3::empty(),
            erxmisc4: Erxmisc4::empty(),
            erxmisc5: Erxmisc5::empty(),
            erxmisc6: Erxmisc6::empty(),
            erxmisc7: Erxmisc7::empty(),
            erxstatus: Erxstatus::empty(),
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
            fcseidr: 0,
            #[cfg(feature = "el1")]
            gcr_el1: GcrEl1::empty(),
            #[cfg(feature = "el1")]
            gcscr_el1: GcscrEl1::empty(),
            #[cfg(feature = "el2")]
            gcscr_el2: GcscrEl2::empty(),
            #[cfg(feature = "el3")]
            gpccr_el3: GpccrEl3::empty(),
            #[cfg(feature = "el3")]
            gptbr_el3: GptbrEl3::empty(),
            hacr: 0,
            #[cfg(feature = "el2")]
            hacr_el2: 0,
            hactlr: 0,
            hactlr2: 0,
            hadfsr: 0,
            haifsr: 0,
            hamair0: 0,
            hamair1: 0,
            hcptr: Hcptr::empty(),
            hcr: Hcr::empty(),
            hcr2: Hcr2::empty(),
            #[cfg(feature = "el2")]
            hcrx_el2: HcrxEl2::empty(),
            #[cfg(feature = "el2")]
            hcr_el2: HcrEl2::empty(),
            hdcr: Hdcr::empty(),
            hdfar: Hdfar::empty(),
            #[cfg(feature = "el2")]
            hdfgrtr2_el2: Hdfgrtr2El2::empty(),
            #[cfg(feature = "el2")]
            hdfgwtr2_el2: Hdfgwtr2El2::empty(),
            #[cfg(feature = "el2")]
            hfgitr2_el2: Hfgitr2El2::empty(),
            #[cfg(feature = "el2")]
            hfgrtr2_el2: Hfgrtr2El2::empty(),
            #[cfg(feature = "el2")]
            hfgwtr2_el2: Hfgwtr2El2::empty(),
            #[cfg(feature = "el2")]
            hfgwtr_el2: HfgwtrEl2::empty(),
            hifar: Hifar::empty(),
            hmair0: Hmair0::empty(),
            hmair1: Hmair1::empty(),
            hpfar: Hpfar::empty(),
            #[cfg(feature = "el2")]
            hpfar_el2: HpfarEl2::empty(),
            hrmr: Hrmr::empty(),
            hsctlr: Hsctlr::empty(),
            hsr: Hsr::empty(),
            hstr: 0,
            #[cfg(feature = "el2")]
            hstr_el2: 0,
            htcr: Htcr::empty(),
            htpidr: Htpidr::empty(),
            htrfcr: Htrfcr::empty(),
            httbr: Httbr::empty(),
            hvbar: Hvbar::empty(),
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
            id_aa64isar1_el1: IdAa64isar1El1::empty(),
            #[cfg(feature = "el1")]
            id_aa64isar2_el1: IdAa64isar2El1::empty(),
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
            id_aa64smfr0_el1: IdAa64smfr0El1::empty(),
            id_afr0: 0,
            id_dfr0: IdDfr0::empty(),
            id_dfr1: IdDfr1::empty(),
            id_isar0: IdIsar0::empty(),
            id_isar1: IdIsar1::empty(),
            id_isar2: IdIsar2::empty(),
            id_isar3: IdIsar3::empty(),
            id_isar4: IdIsar4::empty(),
            id_isar5: IdIsar5::empty(),
            id_isar6: IdIsar6::empty(),
            id_mmfr0: IdMmfr0::empty(),
            id_mmfr1: IdMmfr1::empty(),
            id_mmfr2: IdMmfr2::empty(),
            id_mmfr3: IdMmfr3::empty(),
            id_mmfr4: IdMmfr4::empty(),
            id_mmfr5: IdMmfr5::empty(),
            id_pfr0: IdPfr0::empty(),
            id_pfr1: IdPfr1::empty(),
            id_pfr2: IdPfr2::empty(),
            ifar: Ifar::empty(),
            ifsr: Ifsr::empty(),
            isr: Isr::empty(),
            #[cfg(feature = "el1")]
            isr_el1: IsrEl1::empty(),
            jidr: 0,
            jmcr: 0,
            joscr: 0,
            mair0: Mair0::empty(),
            mair1: Mair1::empty(),
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
            midr: Midr::empty(),
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
            mpidr: Mpidr::empty(),
            #[cfg(feature = "el1")]
            mpidr_el1: MpidrEl1::empty(),
            mvbar: Mvbar::empty(),
            nmrr: Nmrr::empty(),
            nsacr: Nsacr::empty(),
            par: Par::empty(),
            #[cfg(feature = "el1")]
            par_el1: ParEl1::empty(),
            pmccfiltr: Pmccfiltr::empty(),
            pmccntr: Pmccntr::empty(),
            pmceid0: Pmceid0::empty(),
            pmceid1: Pmceid1::empty(),
            pmceid2: Pmceid2::empty(),
            pmceid3: Pmceid3::empty(),
            pmcntenclr: Pmcntenclr::empty(),
            pmcntenset: Pmcntenset::empty(),
            pmcr: Pmcr::empty(),
            pmcr_el0: PmcrEl0::empty(),
            pmintenclr: Pmintenclr::empty(),
            pmintenset: Pmintenset::empty(),
            pmmir: Pmmir::empty(),
            pmovsr: Pmovsr::empty(),
            pmovsset: Pmovsset::empty(),
            pmselr: Pmselr::empty(),
            pmswinc: Pmswinc::empty(),
            pmuserenr: Pmuserenr::empty(),
            pmxevtyper: Pmxevtyper::empty(),
            prrr: Prrr::empty(),
            revidr: 0,
            #[cfg(feature = "el1")]
            rgsr_el1: RgsrEl1::empty(),
            rmr: Rmr::empty(),
            rvbar: Rvbar::empty(),
            scr: Scr::empty(),
            #[cfg(feature = "el3")]
            scr_el3: ScrEl3::empty(),
            sctlr: Sctlr::empty(),
            #[cfg(feature = "el3")]
            sctlr2_el3: Sctlr2El3::empty(),
            #[cfg(feature = "el1")]
            sctlr_el1: SctlrEl1::empty(),
            #[cfg(feature = "el2")]
            sctlr_el2: SctlrEl2::empty(),
            #[cfg(feature = "el3")]
            sctlr_el3: SctlrEl3::empty(),
            sdcr: Sdcr::empty(),
            sder: Sder::empty(),
            #[cfg(feature = "el3")]
            smcr_el3: SmcrEl3::empty(),
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
            tcmtr: 0,
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
            tlbtr: Tlbtr::empty(),
            tpidrprw: Tpidrprw::empty(),
            tpidrro_el0: TpidrroEl0::empty(),
            tpidruro: Tpidruro::empty(),
            tpidrurw: Tpidrurw::empty(),
            tpidr_el0: TpidrEl0::empty(),
            #[cfg(feature = "el1")]
            tpidr_el1: TpidrEl1::empty(),
            #[cfg(feature = "el2")]
            tpidr_el2: TpidrEl2::empty(),
            trfcr: Trfcr::empty(),
            ttbcr: Ttbcr::empty(),
            ttbcr2: Ttbcr2::empty(),
            ttbr0: Ttbr0::empty(),
            #[cfg(feature = "el1")]
            ttbr0_el1: Ttbr0El1::empty(),
            #[cfg(feature = "el2")]
            ttbr0_el2: Ttbr0El2::empty(),
            #[cfg(feature = "el3")]
            ttbr0_el3: Ttbr0El3::empty(),
            ttbr1: Ttbr1::empty(),
            #[cfg(feature = "el1")]
            ttbr1_el1: Ttbr1El1::empty(),
            #[cfg(feature = "el2")]
            ttbr1_el2: Ttbr1El2::empty(),
            vbar: Vbar::empty(),
            #[cfg(feature = "el1")]
            vbar_el1: VbarEl1::empty(),
            #[cfg(feature = "el2")]
            vbar_el2: VbarEl2::empty(),
            vdfsr: Vdfsr::empty(),
            vdisr: Vdisr::empty(),
            #[cfg(feature = "el2")]
            vdisr_el2: VdisrEl2::empty(),
            vmpidr: Vmpidr::empty(),
            #[cfg(feature = "el2")]
            vmpidr_el2: VmpidrEl2::empty(),
            vpidr: Vpidr::empty(),
            #[cfg(feature = "el2")]
            vpidr_el2: VpidrEl2::empty(),
            #[cfg(feature = "el2")]
            vsesr_el2: VsesrEl2::empty(),
            vtcr: Vtcr::empty(),
            #[cfg(feature = "el2")]
            vtcr_el2: VtcrEl2::empty(),
            vttbr: Vttbr::empty(),
            #[cfg(feature = "el2")]
            vttbr_el2: VttbrEl2::empty(),
            #[cfg(feature = "el3")]
            zcr_el3: ZcrEl3::empty(),
        }
    }
}
