// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{AccdataEl1, Allint, Amcfgr, AmcfgrEl0, Amcg1idrEl0, Amcgcr, AmcgcrEl0, Amcntenclr0, Amcntenclr0El0, Amcntenclr1, Amcntenclr1El0, Amcntenset0, Amcntenset0El0, Amcntenset1, Amcntenset1El0, Amcr, AmcrEl0, Amuserenr, AmuserenrEl0, ApdakeyhiEl1, ApdakeyloEl1, ApdbkeyhiEl1, ApdbkeyloEl1, ApgakeyhiEl1, ApgakeyloEl1, ApiakeyhiEl1, ApiakeyloEl1, ApibkeyhiEl1, ApibkeyloEl1, Ats12nsopr, Ats12nsopw, Ats12nsour, Ats12nsouw, Ats1cpr, Ats1cprp, Ats1cpw, Ats1cpwp, Ats1cur, Ats1cuw, Ats1hr, Ats1hw, Bpimva, BrbcrEl1, BrbcrEl2, BrbfcrEl1, Brbidr0El1, BrbinfinjEl1, BrbsrcinjEl1, BrbtgtinjEl1, BrbtsEl1, Ccsidr, Ccsidr2, Ccsidr2El1, CcsidrEl1, Cfprctx, Clidr, ClidrEl1, Cntfrq, CntfrqEl0, Cnthctl, CnthctlEl2, CnthpsCtl, CnthpsCtlEl2, CnthpsCvalEl2, CnthpsTval, CnthpsTvalEl2, CnthpCtl, CnthpCtlEl2, CnthpCvalEl2, CnthpTval, CnthpTvalEl2, CnthvsCtl, CnthvsCtlEl2, CnthvsCvalEl2, CnthvsTval, CnthvsTvalEl2, CnthvCtl, CnthvCtlEl2, CnthvCvalEl2, CnthvTval, CnthvTvalEl2, Cntkctl, CntkctlEl1, CntpctssEl0, CntpctEl0, CntpoffEl2, CntpsCtlEl1, CntpsCvalEl1, CntpsTvalEl1, CntpCtl, CntpCtlEl0, CntpCvalEl0, CntpTval, CntpTvalEl0, CntvctssEl0, CntvctEl0, CntvoffEl2, CntvCtl, CntvCtlEl0, CntvCvalEl0, CntvTval, CntvTvalEl0, Contextidr, ContextidrEl1, ContextidrEl2, Cosprctx, Cpacr, CpacrmaskEl1, CpacrEl1, Cpprctx, CptrmaskEl2, CptrEl2, CptrEl3, Csselr, CsselrEl1, Ctr, CtrEl0, Currentel, Dacr, Dacr32El2, Daif, Dbgauthstatus, DbgauthstatusEl1, Dbgclaimclr, DbgclaimclrEl1, Dbgclaimset, DbgclaimsetEl1, Dbgdccint, Dbgdevid, Dbgdevid1, Dbgdidr, Dbgdrar, Dbgdscrext, Dbgdscrint, DbgdtrrxEl0, Dbgdtrrxext, Dbgdtrrxint, DbgdtrtxEl0, Dbgdtrtxext, Dbgdtrtxint, DbgdtrEl0, Dbgosdlr, Dbgoseccr, Dbgoslar, Dbgoslsr, Dbgprcr, DbgprcrEl1, Dbgvcr, Dbgvcr32El2, Dccimvac, Dccisw, Dccmvac, Dccmvau, Dccsw, Dcimvac, Dcisw, DczidEl0, Dfar, Dfsr, Disr, DisrEl1, Dit, Dlr, DlrEl0, DpocrEl0, Dpotbr0El1, Dpotbr0El2, Dpotbr0El3, Dpotbr1El1, Dpotbr1El2, Dspsr, Dspsr2, DspsrEl0, Dtlbiasid, Dtlbimva, Dvprctx, ElrEl1, ElrEl2, ElrEl3, Erridr, ErridrEl1, Errselr, ErrselrEl1, Erxaddr, Erxaddr2, ErxaddrEl1, Erxctlr, Erxctlr2, ErxctlrEl1, Erxfr, Erxfr2, ErxfrEl1, ErxgsrEl1, Erxmisc0, Erxmisc0El1, Erxmisc1, Erxmisc1El1, Erxmisc2, Erxmisc2El1, Erxmisc3, Erxmisc3El1, Erxmisc4, Erxmisc5, Erxmisc6, Erxmisc7, ErxpfgcdnEl1, ErxpfgctlEl1, ErxpfgfEl1, Erxstatus, ErxstatusEl1, EsrEl1, EsrEl2, EsrEl3, FarEl1, FarEl2, FarEl3, Fgwte3El3, Fpcr, Fpexc32El2, Fpmr, Fpsr, GcrEl1, Gcscre0El1, GcscrEl1, GcscrEl2, GcscrEl3, GcsprEl0, GcsprEl1, GcsprEl2, GcsprEl3, GmidEl1, GpcbwEl3, GpccrEl3, GptbrEl3, HacdbsbrEl2, HacdbsconsEl2, HafgrtrEl2, Hcptr, Hcr, Hcr2, HcrmaskEl2, HcrxmaskEl2, HcrxEl2, HcrEl2, HdbssbrEl2, HdbssprodEl2, Hdcr, Hdfar, Hdfgrtr2El2, HdfgrtrEl2, Hdfgwtr2El2, HdfgwtrEl2, Hfgitr2El2, HfgitrEl2, Hfgrtr2El2, HfgrtrEl2, Hfgwtr2El2, HfgwtrEl2, Hifar, Hmair0, Hmair1, Hpfar, HpfarEl2, Hrmr, Hsctlr, Hsr, Htcr, Htpidr, Htrfcr, Hvbar, IccAprEl1, IccAprEl3, IccAsgi1rEl1, IccBpr0, IccBpr0El1, IccBpr1, IccBpr1El1, IccCr0El1, IccCr0El3, IccCtlr, IccCtlrEl1, IccCtlrEl3, IccDir, IccDirEl1, IccDomhppirEl3, IccEoir0, IccEoir0El1, IccEoir1, IccEoir1El1, IccHaprEl1, IccHppir0, IccHppir0El1, IccHppir1, IccHppir1El1, IccHppirEl1, IccHppirEl3, IccHsre, IccIaffidrEl1, IccIar0, IccIar0El1, IccIar1, IccIar1El1, IccIcsrEl1, IccIdr0El1, IccIgrpen0, IccIgrpen0El1, IccIgrpen1, IccIgrpen1El1, IccIgrpen1El3, IccMctlr, IccMgrpen1, IccMsre, IccNmiar1El1, IccPcrEl1, IccPcrEl3, IccPmr, IccPmrEl1, IccRpr, IccRprEl1, IccSgi0rEl1, IccSgi1rEl1, IccSre, IccSreEl1, IccSreEl2, IccSreEl3, IchAprEl2, IchContextrEl2, IchEisr, IchEisrEl2, IchElrsr, IchElrsrEl2, IchHcr, IchHcrEl2, IchHfgitrEl2, IchHfgrtrEl2, IchHfgwtrEl2, IchHppirEl2, IchMisr, IchMisrEl2, IchVctlrEl2, IchVmcr, IchVmcrEl2, IchVtr, IchVtrEl2, Icimvau, IcvAprEl1, IcvBpr0, IcvBpr0El1, IcvBpr1, IcvBpr1El1, IcvCr0El1, IcvCtlr, IcvCtlrEl1, IcvDir, IcvDirEl1, IcvEoir0, IcvEoir0El1, IcvEoir1, IcvEoir1El1, IcvHaprEl1, IcvHppir0, IcvHppir0El1, IcvHppir1, IcvHppir1El1, IcvHppirEl1, IcvIar0, IcvIar0El1, IcvIar1, IcvIar1El1, IcvIgrpen0, IcvIgrpen0El1, IcvIgrpen1, IcvIgrpen1El1, IcvNmiar1El1, IcvPcrEl1, IcvPmr, IcvPmrEl1, IcvRpr, IcvRprEl1, IdAa64dfr0El1, IdAa64dfr1El1, IdAa64dfr2El1, IdAa64fpfr0El1, IdAa64isar0El1, IdAa64isar1El1, IdAa64isar2El1, IdAa64isar3El1, IdAa64mmfr0El1, IdAa64mmfr1El1, IdAa64mmfr2El1, IdAa64mmfr3El1, IdAa64mmfr4El1, IdAa64pfr0El1, IdAa64pfr1El1, IdAa64pfr2El1, IdAa64smfr0El1, IdAa64zfr0El1, IdDfr0, IdDfr0El1, IdDfr1, IdDfr1El1, IdIsar0, IdIsar0El1, IdIsar1, IdIsar1El1, IdIsar2, IdIsar2El1, IdIsar3, IdIsar3El1, IdIsar4, IdIsar4El1, IdIsar5, IdIsar5El1, IdIsar6, IdIsar6El1, IdMmfr0, IdMmfr0El1, IdMmfr1, IdMmfr1El1, IdMmfr2, IdMmfr2El1, IdMmfr3, IdMmfr3El1, IdMmfr4, IdMmfr4El1, IdMmfr5, IdMmfr5El1, IdPfr0, IdPfr0El1, IdPfr1, IdPfr1El1, IdPfr2, IdPfr2El1, Ifar, Ifsr, Ifsr32El2, IrtbrpEl1, IrtbrpEl2, IrtbrpEl3, IrtbruEl1, IrtbruEl2, Isr, IsrEl1, Itlbiasid, Itlbimva, LdsttEl1, LdsttEl2, LorcEl1, LoreaEl1, LoridEl1, LornEl1, LorsaEl1, Mair0, Mair1, Mair2El1, Mair2El2, Mair2El3, MairEl1, MairEl2, MairEl3, MdccintEl1, MdccsrEl0, MdcrEl2, MdcrEl3, MdrarEl1, MdscrEl1, MdselrEl1, MdstepopEl1, MecidrEl2, MecidA0El2, MecidA1El2, MecidP0El2, MecidP1El2, MecidRlAEl3, MfarEl3, Midr, MidrEl1, Mpam0El1, Mpam1El1, Mpam2El2, Mpam3El3, Mpambw0El1, Mpambw1El1, Mpambw2El2, Mpambw3El3, MpambwcapEl2, MpambwidrEl1, MpambwsmEl1, MpamctlEl1, MpamctlEl2, MpamctlEl3, MpamhcrEl2, MpamidrEl1, MpamsmEl1, MpamvidcrEl2, MpamvidsrEl2, Mpamvpm0El2, Mpamvpm1El2, Mpamvpm2El2, Mpamvpm3El2, Mpamvpm4El2, Mpamvpm5El2, Mpamvpm6El2, Mpamvpm7El2, MpamvpmvEl2, Mpidr, MpidrEl1, Mvbar, Mvfr0El1, Mvfr1El1, Mvfr2El1, Nmrr, Nsacr, NvhcrmaskEl2, NvhcrxmaskEl2, NvhcrxEl2, NvhcrEl2, Nzcv, OsdlrEl1, OsdtrrxEl1, OsdtrtxEl1, OseccrEl1, OslarEl1, OslsrEl1, Pan, Par, ParEl1, PfarEl1, PfarEl2, Pire0El1, Pire0El2, PirEl1, PirEl2, PirEl3, Pm, PmbidrEl1, PmblimitrEl1, PmbmarEl1, PmbptrEl1, PmbsrEl1, PmbsrEl2, PmbsrEl3, Pmccfiltr, PmccfiltrEl0, Pmccntr, PmccntrEl0, PmccntsvrEl1, Pmceid0, Pmceid0El0, Pmceid1, Pmceid1El0, Pmceid2, Pmceid3, Pmcntenclr, PmcntenclrEl0, Pmcntenset, PmcntensetEl0, Pmcr, PmcrEl0, PmecrEl1, PmiarEl1, PmicfiltrEl0, PmicntrEl0, PmicntsvrEl1, Pmintenclr, PmintenclrEl1, Pmintenset, PmintensetEl1, Pmmir, PmmirEl1, PmovsclrEl0, Pmovsr, Pmovsset, PmovssetEl0, PmscrEl1, PmscrEl2, PmsdsfrEl1, Pmselr, PmselrEl0, PmsevfrEl1, PmsfcrEl1, PmsicrEl1, PmsidrEl1, PmsirrEl1, PmslatfrEl1, PmsnevfrEl1, PmsscrEl1, Pmswinc, PmswincEl0, PmuacrEl1, Pmuserenr, PmuserenrEl0, Pmxevcntr, Pmxevtyper, PmxevtyperEl0, PmzrEl0, PorEl0, PorEl1, PorEl2, PorEl3, Prrr, RgsrEl1, Rmr, RmrEl1, RmrEl2, RmrEl3, Rndr, Rndrrs, Rvbar, RvbarEl1, RvbarEl2, RvbarEl3, S2pirEl2, S2porEl1, Scr, Scr2El3, ScrEl3, Sctlr, Sctlr2maskEl1, Sctlr2maskEl2, Sctlr2El1, Sctlr2El2, Sctlr2El3, SctlrmaskEl1, SctlrmaskEl2, SctlrEl1, SctlrEl2, SctlrEl3, ScxtnumEl0, ScxtnumEl1, ScxtnumEl2, ScxtnumEl3, Sdcr, Sder, Sder32El2, Sder32El3, SmcrEl1, SmcrEl2, SmcrEl3, SmidrEl1, SmprimapEl2, SmpriEl1, SpmaccessrEl1, SpmaccessrEl2, SpmaccessrEl3, SpmcfgrEl1, SpmcntenclrEl0, SpmcntensetEl0, SpmcrEl0, SpmdevaffEl1, SpmdevarchEl1, SpmiidrEl1, SpmintenclrEl1, SpmintensetEl1, SpmovsclrEl0, SpmovssetEl0, SpmrootcrEl3, SpmscrEl1, SpmselrEl0, SpmzrEl0, SpsrEl1, SpsrEl2, SpsrEl3, SpsrAbt, SpsrFiq, SpsrIrq, SpsrUnd, Spsel, SpEl0, SpEl1, SpEl2, Ssbs, StindexEl1, StindexEl2, StindexEl3, Svcr, Tco, Tcr2maskEl1, Tcr2maskEl2, Tcr2El1, Tcr2El2, TcrmaskEl1, TcrmaskEl2, TcrEl1, TcrEl2, TcrEl3, Tfsre0El1, TfsrEl1, TfsrEl2, TfsrEl3, TindexEl0, TindexEl1, TindexEl2, TindexEl3, Tlbiasid, Tlbiasidis, TlbididrEl1, Tlbiipas2, Tlbiipas2is, Tlbiipas2l, Tlbiipas2lis, Tlbimva, Tlbimvaa, Tlbimvaais, Tlbimvaal, Tlbimvaalis, Tlbimvah, Tlbimvahis, Tlbimvais, Tlbimval, Tlbimvalh, Tlbimvalhis, Tlbimvalis, Tlbtr, Tpidr2El0, Tpidrprw, TpidrroEl0, Tpidruro, Tpidrurw, TpidrEl0, TpidrEl1, TpidrEl2, TpidrEl3, Tpmax0El0, Tpmax0El1, Tpmax0El2, Tpmax1El0, Tpmax1El1, Tpmax1El2, Tpmin0El0, Tpmin0El1, Tpmin0El2, Tpmin1El0, Tpmin1El1, Tpmin1El2, TrbbaserEl1, TrbidrEl1, TrblimitrEl1, TrbmarEl1, TrbmpamEl1, TrbptrEl1, TrbsrEl1, TrbsrEl2, TrbsrEl3, TrbtrgEl1, Trcauthstatus, Trcbbctlr, Trcccctlr, Trccidcctlr0, Trccidcctlr1, Trcclaimclr, Trcclaimset, Trcconfigr, Trcdevarch, Trceventctl0r, Trceventctl1r, Trcidr0, Trcidr1, Trcidr10, Trcidr11, Trcidr12, Trcidr13, Trcidr2, Trcidr3, Trcidr4, Trcidr5, Trcidr6, Trcidr8, Trcidr9, Trcimspec0, TrcitecrEl1, TrcitecrEl2, Trciteedcr, Trcoslsr, Trcprgctlr, Trcqctlr, Trcrsr, Trcseqrstevr, Trcseqstr, Trcstallctlr, Trcstatr, Trcsyncpr, Trctraceidr, Trctsctlr, Trcvictlr, Trcviiectlr, Trcvipcssctlr, Trcvissctlr, Trcvmidcctlr0, Trcvmidcctlr1, Trfcr, TrfcrEl1, TrfcrEl2, Ttbcr, Ttbcr2, Ttbr0, Ttbr0El1, Ttbr0El2, Ttbr0El3, Ttbr1, Ttbr1El1, Ttbr1El2, TttbrpEl1, TttbrpEl2, TttbrpEl3, TttbruEl1, TttbruEl2, Uao, Vbar, VbarEl1, VbarEl2, VbarEl3, Vdfsr, Vdisr, VdisrEl2, VdisrEl3, VmecidAEl2, VmecidPEl2, Vmpidr, VmpidrEl2, VnccrEl2, VncrEl2, Vpidr, VpidrEl2, VsesrEl2, VsesrEl3, VstcrEl2, VsttbrEl2, Vtcr, VtcrEl2, VttbrEl2, ZcrEl1, ZcrEl2, ZcrEl3};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemRegisters {
    /// Fake value for the `ACCDATA_EL1` system register.
    pub accdata_el1: AccdataEl1,
    /// Fake value for the `ACTLR` system register.
    pub actlr: u32,
    /// Fake value for the `ACTLR2` system register.
    pub actlr2: u32,
    /// Fake value for the `ACTLRMASK_EL1` system register.
    pub actlrmask_el1: u64,
    /// Fake value for the `ACTLRMASK_EL2` system register.
    pub actlrmask_el2: u64,
    /// Fake value for the `ACTLR_EL1` system register.
    pub actlr_el1: u64,
    /// Fake value for the `ACTLR_EL2` system register.
    pub actlr_el2: u64,
    /// Fake value for the `ACTLR_EL3` system register.
    pub actlr_el3: u64,
    /// Fake value for the `ADFSR` system register.
    pub adfsr: u32,
    /// Fake value for the `AFSR0_EL1` system register.
    pub afsr0_el1: u64,
    /// Fake value for the `AFSR0_EL2` system register.
    pub afsr0_el2: u64,
    /// Fake value for the `AFSR0_EL3` system register.
    pub afsr0_el3: u64,
    /// Fake value for the `AFSR1_EL1` system register.
    pub afsr1_el1: u64,
    /// Fake value for the `AFSR1_EL2` system register.
    pub afsr1_el2: u64,
    /// Fake value for the `AFSR1_EL3` system register.
    pub afsr1_el3: u64,
    /// Fake value for the `AIDR` system register.
    pub aidr: u32,
    /// Fake value for the `AIDR_EL1` system register.
    pub aidr_el1: u64,
    /// Fake value for the `AIFSR` system register.
    pub aifsr: u32,
    /// Fake value for the `ALLINT` system register.
    pub allint: Allint,
    /// Fake value for the `AMAIR0` system register.
    pub amair0: u32,
    /// Fake value for the `AMAIR1` system register.
    pub amair1: u32,
    /// Fake value for the `AMAIR2_EL1` system register.
    pub amair2_el1: u64,
    /// Fake value for the `AMAIR2_EL2` system register.
    pub amair2_el2: u64,
    /// Fake value for the `AMAIR2_EL3` system register.
    pub amair2_el3: u64,
    /// Fake value for the `AMAIR_EL1` system register.
    pub amair_el1: u64,
    /// Fake value for the `AMAIR_EL2` system register.
    pub amair_el2: u64,
    /// Fake value for the `AMAIR_EL3` system register.
    pub amair_el3: u64,
    /// Fake value for the `AMCFGR` system register.
    pub amcfgr: Amcfgr,
    /// Fake value for the `AMCFGR_EL0` system register.
    pub amcfgr_el0: AmcfgrEl0,
    /// Fake value for the `AMCG1IDR_EL0` system register.
    pub amcg1idr_el0: Amcg1idrEl0,
    /// Fake value for the `AMCGCR` system register.
    pub amcgcr: Amcgcr,
    /// Fake value for the `AMCGCR_EL0` system register.
    pub amcgcr_el0: AmcgcrEl0,
    /// Fake value for the `AMCNTENCLR0` system register.
    pub amcntenclr0: Amcntenclr0,
    /// Fake value for the `AMCNTENCLR0_EL0` system register.
    pub amcntenclr0_el0: Amcntenclr0El0,
    /// Fake value for the `AMCNTENCLR1` system register.
    pub amcntenclr1: Amcntenclr1,
    /// Fake value for the `AMCNTENCLR1_EL0` system register.
    pub amcntenclr1_el0: Amcntenclr1El0,
    /// Fake value for the `AMCNTENSET0` system register.
    pub amcntenset0: Amcntenset0,
    /// Fake value for the `AMCNTENSET0_EL0` system register.
    pub amcntenset0_el0: Amcntenset0El0,
    /// Fake value for the `AMCNTENSET1` system register.
    pub amcntenset1: Amcntenset1,
    /// Fake value for the `AMCNTENSET1_EL0` system register.
    pub amcntenset1_el0: Amcntenset1El0,
    /// Fake value for the `AMCR` system register.
    pub amcr: Amcr,
    /// Fake value for the `AMCR_EL0` system register.
    pub amcr_el0: AmcrEl0,
    /// Fake value for the `AMUSERENR` system register.
    pub amuserenr: Amuserenr,
    /// Fake value for the `AMUSERENR_EL0` system register.
    pub amuserenr_el0: AmuserenrEl0,
    /// Fake value for the `APDAKeyHi_EL1` system register.
    pub apdakeyhi_el1: ApdakeyhiEl1,
    /// Fake value for the `APDAKeyLo_EL1` system register.
    pub apdakeylo_el1: ApdakeyloEl1,
    /// Fake value for the `APDBKeyHi_EL1` system register.
    pub apdbkeyhi_el1: ApdbkeyhiEl1,
    /// Fake value for the `APDBKeyLo_EL1` system register.
    pub apdbkeylo_el1: ApdbkeyloEl1,
    /// Fake value for the `APGAKeyHi_EL1` system register.
    pub apgakeyhi_el1: ApgakeyhiEl1,
    /// Fake value for the `APGAKeyLo_EL1` system register.
    pub apgakeylo_el1: ApgakeyloEl1,
    /// Fake value for the `APIAKeyHi_EL1` system register.
    pub apiakeyhi_el1: ApiakeyhiEl1,
    /// Fake value for the `APIAKeyLo_EL1` system register.
    pub apiakeylo_el1: ApiakeyloEl1,
    /// Fake value for the `APIBKeyHi_EL1` system register.
    pub apibkeyhi_el1: ApibkeyhiEl1,
    /// Fake value for the `APIBKeyLo_EL1` system register.
    pub apibkeylo_el1: ApibkeyloEl1,
    /// Fake value for the `ATS12NSOPR` system register.
    pub ats12nsopr: Ats12nsopr,
    /// Fake value for the `ATS12NSOPW` system register.
    pub ats12nsopw: Ats12nsopw,
    /// Fake value for the `ATS12NSOUR` system register.
    pub ats12nsour: Ats12nsour,
    /// Fake value for the `ATS12NSOUW` system register.
    pub ats12nsouw: Ats12nsouw,
    /// Fake value for the `ATS1CPR` system register.
    pub ats1cpr: Ats1cpr,
    /// Fake value for the `ATS1CPRP` system register.
    pub ats1cprp: Ats1cprp,
    /// Fake value for the `ATS1CPW` system register.
    pub ats1cpw: Ats1cpw,
    /// Fake value for the `ATS1CPWP` system register.
    pub ats1cpwp: Ats1cpwp,
    /// Fake value for the `ATS1CUR` system register.
    pub ats1cur: Ats1cur,
    /// Fake value for the `ATS1CUW` system register.
    pub ats1cuw: Ats1cuw,
    /// Fake value for the `ATS1HR` system register.
    pub ats1hr: Ats1hr,
    /// Fake value for the `ATS1HW` system register.
    pub ats1hw: Ats1hw,
    /// Fake value for the `BPIALL` system register.
    pub bpiall: u32,
    /// Fake value for the `BPIALLIS` system register.
    pub bpiallis: u32,
    /// Fake value for the `BPIMVA` system register.
    pub bpimva: Bpimva,
    /// Fake value for the `BRBCR_EL1` system register.
    pub brbcr_el1: BrbcrEl1,
    /// Fake value for the `BRBCR_EL2` system register.
    pub brbcr_el2: BrbcrEl2,
    /// Fake value for the `BRBFCR_EL1` system register.
    pub brbfcr_el1: BrbfcrEl1,
    /// Fake value for the `BRBIDR0_EL1` system register.
    pub brbidr0_el1: Brbidr0El1,
    /// Fake value for the `BRBINFINJ_EL1` system register.
    pub brbinfinj_el1: BrbinfinjEl1,
    /// Fake value for the `BRBSRCINJ_EL1` system register.
    pub brbsrcinj_el1: BrbsrcinjEl1,
    /// Fake value for the `BRBTGTINJ_EL1` system register.
    pub brbtgtinj_el1: BrbtgtinjEl1,
    /// Fake value for the `BRBTS_EL1` system register.
    pub brbts_el1: BrbtsEl1,
    /// Fake value for the `CCSIDR` system register.
    pub ccsidr: Ccsidr,
    /// Fake value for the `CCSIDR2` system register.
    pub ccsidr2: Ccsidr2,
    /// Fake value for the `CCSIDR2_EL1` system register.
    pub ccsidr2_el1: Ccsidr2El1,
    /// Fake value for the `CCSIDR_EL1` system register.
    pub ccsidr_el1: CcsidrEl1,
    /// Fake value for the `CFPRCTX` system register.
    pub cfprctx: Cfprctx,
    /// Fake value for the `CLIDR` system register.
    pub clidr: Clidr,
    /// Fake value for the `CLIDR_EL1` system register.
    pub clidr_el1: ClidrEl1,
    /// Fake value for the `CNTFRQ` system register.
    pub cntfrq: Cntfrq,
    /// Fake value for the `CNTFRQ_EL0` system register.
    pub cntfrq_el0: CntfrqEl0,
    /// Fake value for the `CNTHCTL` system register.
    pub cnthctl: Cnthctl,
    /// Fake value for the `CNTHCTL_EL2` system register.
    pub cnthctl_el2: CnthctlEl2,
    /// Fake value for the `CNTHPS_CTL` system register.
    pub cnthps_ctl: CnthpsCtl,
    /// Fake value for the `CNTHPS_CTL_EL2` system register.
    pub cnthps_ctl_el2: CnthpsCtlEl2,
    /// Fake value for the `CNTHPS_CVAL_EL2` system register.
    pub cnthps_cval_el2: CnthpsCvalEl2,
    /// Fake value for the `CNTHPS_TVAL` system register.
    pub cnthps_tval: CnthpsTval,
    /// Fake value for the `CNTHPS_TVAL_EL2` system register.
    pub cnthps_tval_el2: CnthpsTvalEl2,
    /// Fake value for the `CNTHP_CTL` system register.
    pub cnthp_ctl: CnthpCtl,
    /// Fake value for the `CNTHP_CTL_EL2` system register.
    pub cnthp_ctl_el2: CnthpCtlEl2,
    /// Fake value for the `CNTHP_CVAL_EL2` system register.
    pub cnthp_cval_el2: CnthpCvalEl2,
    /// Fake value for the `CNTHP_TVAL` system register.
    pub cnthp_tval: CnthpTval,
    /// Fake value for the `CNTHP_TVAL_EL2` system register.
    pub cnthp_tval_el2: CnthpTvalEl2,
    /// Fake value for the `CNTHVS_CTL` system register.
    pub cnthvs_ctl: CnthvsCtl,
    /// Fake value for the `CNTHVS_CTL_EL2` system register.
    pub cnthvs_ctl_el2: CnthvsCtlEl2,
    /// Fake value for the `CNTHVS_CVAL_EL2` system register.
    pub cnthvs_cval_el2: CnthvsCvalEl2,
    /// Fake value for the `CNTHVS_TVAL` system register.
    pub cnthvs_tval: CnthvsTval,
    /// Fake value for the `CNTHVS_TVAL_EL2` system register.
    pub cnthvs_tval_el2: CnthvsTvalEl2,
    /// Fake value for the `CNTHV_CTL` system register.
    pub cnthv_ctl: CnthvCtl,
    /// Fake value for the `CNTHV_CTL_EL2` system register.
    pub cnthv_ctl_el2: CnthvCtlEl2,
    /// Fake value for the `CNTHV_CVAL_EL2` system register.
    pub cnthv_cval_el2: CnthvCvalEl2,
    /// Fake value for the `CNTHV_TVAL` system register.
    pub cnthv_tval: CnthvTval,
    /// Fake value for the `CNTHV_TVAL_EL2` system register.
    pub cnthv_tval_el2: CnthvTvalEl2,
    /// Fake value for the `CNTKCTL` system register.
    pub cntkctl: Cntkctl,
    /// Fake value for the `CNTKCTL_EL1` system register.
    pub cntkctl_el1: CntkctlEl1,
    /// Fake value for the `CNTPCTSS_EL0` system register.
    pub cntpctss_el0: CntpctssEl0,
    /// Fake value for the `CNTPCT_EL0` system register.
    pub cntpct_el0: CntpctEl0,
    /// Fake value for the `CNTPOFF_EL2` system register.
    pub cntpoff_el2: CntpoffEl2,
    /// Fake value for the `CNTPS_CTL_EL1` system register.
    pub cntps_ctl_el1: CntpsCtlEl1,
    /// Fake value for the `CNTPS_CVAL_EL1` system register.
    pub cntps_cval_el1: CntpsCvalEl1,
    /// Fake value for the `CNTPS_TVAL_EL1` system register.
    pub cntps_tval_el1: CntpsTvalEl1,
    /// Fake value for the `CNTP_CTL` system register.
    pub cntp_ctl: CntpCtl,
    /// Fake value for the `CNTP_CTL_EL0` system register.
    pub cntp_ctl_el0: CntpCtlEl0,
    /// Fake value for the `CNTP_CVAL_EL0` system register.
    pub cntp_cval_el0: CntpCvalEl0,
    /// Fake value for the `CNTP_TVAL` system register.
    pub cntp_tval: CntpTval,
    /// Fake value for the `CNTP_TVAL_EL0` system register.
    pub cntp_tval_el0: CntpTvalEl0,
    /// Fake value for the `CNTVCTSS_EL0` system register.
    pub cntvctss_el0: CntvctssEl0,
    /// Fake value for the `CNTVCT_EL0` system register.
    pub cntvct_el0: CntvctEl0,
    /// Fake value for the `CNTVOFF_EL2` system register.
    pub cntvoff_el2: CntvoffEl2,
    /// Fake value for the `CNTV_CTL` system register.
    pub cntv_ctl: CntvCtl,
    /// Fake value for the `CNTV_CTL_EL0` system register.
    pub cntv_ctl_el0: CntvCtlEl0,
    /// Fake value for the `CNTV_CVAL_EL0` system register.
    pub cntv_cval_el0: CntvCvalEl0,
    /// Fake value for the `CNTV_TVAL` system register.
    pub cntv_tval: CntvTval,
    /// Fake value for the `CNTV_TVAL_EL0` system register.
    pub cntv_tval_el0: CntvTvalEl0,
    /// Fake value for the `CONTEXTIDR` system register.
    pub contextidr: Contextidr,
    /// Fake value for the `CONTEXTIDR_EL1` system register.
    pub contextidr_el1: ContextidrEl1,
    /// Fake value for the `CONTEXTIDR_EL2` system register.
    pub contextidr_el2: ContextidrEl2,
    /// Fake value for the `COSPRCTX` system register.
    pub cosprctx: Cosprctx,
    /// Fake value for the `CP15DMB` system register.
    pub cp15dmb: u32,
    /// Fake value for the `CP15DSB` system register.
    pub cp15dsb: u32,
    /// Fake value for the `CP15ISB` system register.
    pub cp15isb: u32,
    /// Fake value for the `CPACR` system register.
    pub cpacr: Cpacr,
    /// Fake value for the `CPACRMASK_EL1` system register.
    pub cpacrmask_el1: CpacrmaskEl1,
    /// Fake value for the `CPACR_EL1` system register.
    pub cpacr_el1: CpacrEl1,
    /// Fake value for the `CPPRCTX` system register.
    pub cpprctx: Cpprctx,
    /// Fake value for the `CPTRMASK_EL2` system register.
    pub cptrmask_el2: CptrmaskEl2,
    /// Fake value for the `CPTR_EL2` system register.
    pub cptr_el2: CptrEl2,
    /// Fake value for the `CPTR_EL3` system register.
    pub cptr_el3: CptrEl3,
    /// Fake value for the `CSSELR` system register.
    pub csselr: Csselr,
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
    /// Fake value for the `DACR32_EL2` system register.
    pub dacr32_el2: Dacr32El2,
    /// Fake value for the `DAIF` system register.
    pub daif: Daif,
    /// Fake value for the `DBGAUTHSTATUS` system register.
    pub dbgauthstatus: Dbgauthstatus,
    /// Fake value for the `DBGAUTHSTATUS_EL1` system register.
    pub dbgauthstatus_el1: DbgauthstatusEl1,
    /// Fake value for the `DBGCLAIMCLR` system register.
    pub dbgclaimclr: Dbgclaimclr,
    /// Fake value for the `DBGCLAIMCLR_EL1` system register.
    pub dbgclaimclr_el1: DbgclaimclrEl1,
    /// Fake value for the `DBGCLAIMSET` system register.
    pub dbgclaimset: Dbgclaimset,
    /// Fake value for the `DBGCLAIMSET_EL1` system register.
    pub dbgclaimset_el1: DbgclaimsetEl1,
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
    pub dbgdsar: u32,
    /// Fake value for the `DBGDSCRext` system register.
    pub dbgdscrext: Dbgdscrext,
    /// Fake value for the `DBGDSCRint` system register.
    pub dbgdscrint: Dbgdscrint,
    /// Fake value for the `DBGDTRRX_EL0` system register.
    pub dbgdtrrx_el0: DbgdtrrxEl0,
    /// Fake value for the `DBGDTRRXext` system register.
    pub dbgdtrrxext: Dbgdtrrxext,
    /// Fake value for the `DBGDTRRXint` system register.
    pub dbgdtrrxint: Dbgdtrrxint,
    /// Fake value for the `DBGDTRTX_EL0` system register.
    pub dbgdtrtx_el0: DbgdtrtxEl0,
    /// Fake value for the `DBGDTRTXext` system register.
    pub dbgdtrtxext: Dbgdtrtxext,
    /// Fake value for the `DBGDTRTXint` system register.
    pub dbgdtrtxint: Dbgdtrtxint,
    /// Fake value for the `DBGDTR_EL0` system register.
    pub dbgdtr_el0: DbgdtrEl0,
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
    /// Fake value for the `DBGPRCR_EL1` system register.
    pub dbgprcr_el1: DbgprcrEl1,
    /// Fake value for the `DBGVCR` system register.
    pub dbgvcr: Dbgvcr,
    /// Fake value for the `DBGVCR32_EL2` system register.
    pub dbgvcr32_el2: Dbgvcr32El2,
    /// Fake value for the `DBGWFAR` system register.
    pub dbgwfar: u32,
    /// Fake value for the `DCCIMVAC` system register.
    pub dccimvac: Dccimvac,
    /// Fake value for the `DCCISW` system register.
    pub dccisw: Dccisw,
    /// Fake value for the `DCCMVAC` system register.
    pub dccmvac: Dccmvac,
    /// Fake value for the `DCCMVAU` system register.
    pub dccmvau: Dccmvau,
    /// Fake value for the `DCCSW` system register.
    pub dccsw: Dccsw,
    /// Fake value for the `DCIMVAC` system register.
    pub dcimvac: Dcimvac,
    /// Fake value for the `DCISW` system register.
    pub dcisw: Dcisw,
    /// Fake value for the `DCZID_EL0` system register.
    pub dczid_el0: DczidEl0,
    /// Fake value for the `DFAR` system register.
    pub dfar: Dfar,
    /// Fake value for the `DFSR` system register.
    pub dfsr: Dfsr,
    /// Fake value for the `DISR` system register.
    pub disr: Disr,
    /// Fake value for the `DISR_EL1` system register.
    pub disr_el1: DisrEl1,
    /// Fake value for the `DIT` system register.
    pub dit: Dit,
    /// Fake value for the `DLR` system register.
    pub dlr: Dlr,
    /// Fake value for the `DLR_EL0` system register.
    pub dlr_el0: DlrEl0,
    /// Fake value for the `DPOCR_EL0` system register.
    pub dpocr_el0: DpocrEl0,
    /// Fake value for the `DPOTBR0_EL1` system register.
    pub dpotbr0_el1: Dpotbr0El1,
    /// Fake value for the `DPOTBR0_EL2` system register.
    pub dpotbr0_el2: Dpotbr0El2,
    /// Fake value for the `DPOTBR0_EL3` system register.
    pub dpotbr0_el3: Dpotbr0El3,
    /// Fake value for the `DPOTBR1_EL1` system register.
    pub dpotbr1_el1: Dpotbr1El1,
    /// Fake value for the `DPOTBR1_EL2` system register.
    pub dpotbr1_el2: Dpotbr1El2,
    /// Fake value for the `DSPSR` system register.
    pub dspsr: Dspsr,
    /// Fake value for the `DSPSR2` system register.
    pub dspsr2: Dspsr2,
    /// Fake value for the `DSPSR_EL0` system register.
    pub dspsr_el0: DspsrEl0,
    /// Fake value for the `DTLBIALL` system register.
    pub dtlbiall: u32,
    /// Fake value for the `DTLBIASID` system register.
    pub dtlbiasid: Dtlbiasid,
    /// Fake value for the `DTLBIMVA` system register.
    pub dtlbimva: Dtlbimva,
    /// Fake value for the `DVPRCTX` system register.
    pub dvprctx: Dvprctx,
    /// Fake value for the `ELR_EL1` system register.
    pub elr_el1: ElrEl1,
    /// Fake value for the `ELR_EL2` system register.
    pub elr_el2: ElrEl2,
    /// Fake value for the `ELR_EL3` system register.
    pub elr_el3: ElrEl3,
    /// Fake value for the `ERRIDR` system register.
    pub erridr: Erridr,
    /// Fake value for the `ERRIDR_EL1` system register.
    pub erridr_el1: ErridrEl1,
    /// Fake value for the `ERRSELR` system register.
    pub errselr: Errselr,
    /// Fake value for the `ERRSELR_EL1` system register.
    pub errselr_el1: ErrselrEl1,
    /// Fake value for the `ERXADDR` system register.
    pub erxaddr: Erxaddr,
    /// Fake value for the `ERXADDR2` system register.
    pub erxaddr2: Erxaddr2,
    /// Fake value for the `ERXADDR_EL1` system register.
    pub erxaddr_el1: ErxaddrEl1,
    /// Fake value for the `ERXCTLR` system register.
    pub erxctlr: Erxctlr,
    /// Fake value for the `ERXCTLR2` system register.
    pub erxctlr2: Erxctlr2,
    /// Fake value for the `ERXCTLR_EL1` system register.
    pub erxctlr_el1: ErxctlrEl1,
    /// Fake value for the `ERXFR` system register.
    pub erxfr: Erxfr,
    /// Fake value for the `ERXFR2` system register.
    pub erxfr2: Erxfr2,
    /// Fake value for the `ERXFR_EL1` system register.
    pub erxfr_el1: ErxfrEl1,
    /// Fake value for the `ERXGSR_EL1` system register.
    pub erxgsr_el1: ErxgsrEl1,
    /// Fake value for the `ERXMISC0` system register.
    pub erxmisc0: Erxmisc0,
    /// Fake value for the `ERXMISC0_EL1` system register.
    pub erxmisc0_el1: Erxmisc0El1,
    /// Fake value for the `ERXMISC1` system register.
    pub erxmisc1: Erxmisc1,
    /// Fake value for the `ERXMISC1_EL1` system register.
    pub erxmisc1_el1: Erxmisc1El1,
    /// Fake value for the `ERXMISC2` system register.
    pub erxmisc2: Erxmisc2,
    /// Fake value for the `ERXMISC2_EL1` system register.
    pub erxmisc2_el1: Erxmisc2El1,
    /// Fake value for the `ERXMISC3` system register.
    pub erxmisc3: Erxmisc3,
    /// Fake value for the `ERXMISC3_EL1` system register.
    pub erxmisc3_el1: Erxmisc3El1,
    /// Fake value for the `ERXMISC4` system register.
    pub erxmisc4: Erxmisc4,
    /// Fake value for the `ERXMISC5` system register.
    pub erxmisc5: Erxmisc5,
    /// Fake value for the `ERXMISC6` system register.
    pub erxmisc6: Erxmisc6,
    /// Fake value for the `ERXMISC7` system register.
    pub erxmisc7: Erxmisc7,
    /// Fake value for the `ERXPFGCDN_EL1` system register.
    pub erxpfgcdn_el1: ErxpfgcdnEl1,
    /// Fake value for the `ERXPFGCTL_EL1` system register.
    pub erxpfgctl_el1: ErxpfgctlEl1,
    /// Fake value for the `ERXPFGF_EL1` system register.
    pub erxpfgf_el1: ErxpfgfEl1,
    /// Fake value for the `ERXSTATUS` system register.
    pub erxstatus: Erxstatus,
    /// Fake value for the `ERXSTATUS_EL1` system register.
    pub erxstatus_el1: ErxstatusEl1,
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
    /// Fake value for the `FAR_EL3` system register.
    pub far_el3: FarEl3,
    /// Fake value for the `FCSEIDR` system register.
    pub fcseidr: u32,
    /// Fake value for the `FGWTE3_EL3` system register.
    pub fgwte3_el3: Fgwte3El3,
    /// Fake value for the `FPCR` system register.
    pub fpcr: Fpcr,
    /// Fake value for the `FPEXC32_EL2` system register.
    pub fpexc32_el2: Fpexc32El2,
    /// Fake value for the `FPMR` system register.
    pub fpmr: Fpmr,
    /// Fake value for the `FPSR` system register.
    pub fpsr: Fpsr,
    /// Fake value for the `GCR_EL1` system register.
    pub gcr_el1: GcrEl1,
    /// Fake value for the `GCSCRE0_EL1` system register.
    pub gcscre0_el1: Gcscre0El1,
    /// Fake value for the `GCSCR_EL1` system register.
    pub gcscr_el1: GcscrEl1,
    /// Fake value for the `GCSCR_EL2` system register.
    pub gcscr_el2: GcscrEl2,
    /// Fake value for the `GCSCR_EL3` system register.
    pub gcscr_el3: GcscrEl3,
    /// Fake value for the `GCSPR_EL0` system register.
    pub gcspr_el0: GcsprEl0,
    /// Fake value for the `GCSPR_EL1` system register.
    pub gcspr_el1: GcsprEl1,
    /// Fake value for the `GCSPR_EL2` system register.
    pub gcspr_el2: GcsprEl2,
    /// Fake value for the `GCSPR_EL3` system register.
    pub gcspr_el3: GcsprEl3,
    /// Fake value for the `GMID_EL1` system register.
    pub gmid_el1: GmidEl1,
    /// Fake value for the `GPCBW_EL3` system register.
    pub gpcbw_el3: GpcbwEl3,
    /// Fake value for the `GPCCR_EL3` system register.
    pub gpccr_el3: GpccrEl3,
    /// Fake value for the `GPTBR_EL3` system register.
    pub gptbr_el3: GptbrEl3,
    /// Fake value for the `HACDBSBR_EL2` system register.
    pub hacdbsbr_el2: HacdbsbrEl2,
    /// Fake value for the `HACDBSCONS_EL2` system register.
    pub hacdbscons_el2: HacdbsconsEl2,
    /// Fake value for the `HACR` system register.
    pub hacr: u32,
    /// Fake value for the `HACR_EL2` system register.
    pub hacr_el2: u64,
    /// Fake value for the `HACTLR` system register.
    pub hactlr: u32,
    /// Fake value for the `HACTLR2` system register.
    pub hactlr2: u32,
    /// Fake value for the `HADFSR` system register.
    pub hadfsr: u32,
    /// Fake value for the `HAFGRTR_EL2` system register.
    pub hafgrtr_el2: HafgrtrEl2,
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
    /// Fake value for the `HCRMASK_EL2` system register.
    pub hcrmask_el2: HcrmaskEl2,
    /// Fake value for the `HCRXMASK_EL2` system register.
    pub hcrxmask_el2: HcrxmaskEl2,
    /// Fake value for the `HCRX_EL2` system register.
    pub hcrx_el2: HcrxEl2,
    /// Fake value for the `HCR_EL2` system register.
    pub hcr_el2: HcrEl2,
    /// Fake value for the `HDBSSBR_EL2` system register.
    pub hdbssbr_el2: HdbssbrEl2,
    /// Fake value for the `HDBSSPROD_EL2` system register.
    pub hdbssprod_el2: HdbssprodEl2,
    /// Fake value for the `HDCR` system register.
    pub hdcr: Hdcr,
    /// Fake value for the `HDFAR` system register.
    pub hdfar: Hdfar,
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
    /// Fake value for the `HIFAR` system register.
    pub hifar: Hifar,
    /// Fake value for the `HMAIR0` system register.
    pub hmair0: Hmair0,
    /// Fake value for the `HMAIR1` system register.
    pub hmair1: Hmair1,
    /// Fake value for the `HPFAR` system register.
    pub hpfar: Hpfar,
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
    /// Fake value for the `HSTR_EL2` system register.
    pub hstr_el2: u64,
    /// Fake value for the `HTCR` system register.
    pub htcr: Htcr,
    /// Fake value for the `HTPIDR` system register.
    pub htpidr: Htpidr,
    /// Fake value for the `HTRFCR` system register.
    pub htrfcr: Htrfcr,
    /// Fake value for the `HVBAR` system register.
    pub hvbar: Hvbar,
    /// Fake value for the `ICC_APR_EL1` system register.
    pub icc_apr_el1: IccAprEl1,
    /// Fake value for the `ICC_APR_EL3` system register.
    pub icc_apr_el3: IccAprEl3,
    /// Fake value for the `ICC_ASGI1R_EL1` system register.
    pub icc_asgi1r_el1: IccAsgi1rEl1,
    /// Fake value for the `ICC_BPR0` system register.
    pub icc_bpr0: IccBpr0,
    /// Fake value for the `ICC_BPR0_EL1` system register.
    pub icc_bpr0_el1: IccBpr0El1,
    /// Fake value for the `ICC_BPR1` system register.
    pub icc_bpr1: IccBpr1,
    /// Fake value for the `ICC_BPR1_EL1` system register.
    pub icc_bpr1_el1: IccBpr1El1,
    /// Fake value for the `ICC_CR0_EL1` system register.
    pub icc_cr0_el1: IccCr0El1,
    /// Fake value for the `ICC_CR0_EL3` system register.
    pub icc_cr0_el3: IccCr0El3,
    /// Fake value for the `ICC_CTLR` system register.
    pub icc_ctlr: IccCtlr,
    /// Fake value for the `ICC_CTLR_EL1` system register.
    pub icc_ctlr_el1: IccCtlrEl1,
    /// Fake value for the `ICC_CTLR_EL3` system register.
    pub icc_ctlr_el3: IccCtlrEl3,
    /// Fake value for the `ICC_DIR` system register.
    pub icc_dir: IccDir,
    /// Fake value for the `ICC_DIR_EL1` system register.
    pub icc_dir_el1: IccDirEl1,
    /// Fake value for the `ICC_DOMHPPIR_EL3` system register.
    pub icc_domhppir_el3: IccDomhppirEl3,
    /// Fake value for the `ICC_EOIR0` system register.
    pub icc_eoir0: IccEoir0,
    /// Fake value for the `ICC_EOIR0_EL1` system register.
    pub icc_eoir0_el1: IccEoir0El1,
    /// Fake value for the `ICC_EOIR1` system register.
    pub icc_eoir1: IccEoir1,
    /// Fake value for the `ICC_EOIR1_EL1` system register.
    pub icc_eoir1_el1: IccEoir1El1,
    /// Fake value for the `ICC_HAPR_EL1` system register.
    pub icc_hapr_el1: IccHaprEl1,
    /// Fake value for the `ICC_HPPIR0` system register.
    pub icc_hppir0: IccHppir0,
    /// Fake value for the `ICC_HPPIR0_EL1` system register.
    pub icc_hppir0_el1: IccHppir0El1,
    /// Fake value for the `ICC_HPPIR1` system register.
    pub icc_hppir1: IccHppir1,
    /// Fake value for the `ICC_HPPIR1_EL1` system register.
    pub icc_hppir1_el1: IccHppir1El1,
    /// Fake value for the `ICC_HPPIR_EL1` system register.
    pub icc_hppir_el1: IccHppirEl1,
    /// Fake value for the `ICC_HPPIR_EL3` system register.
    pub icc_hppir_el3: IccHppirEl3,
    /// Fake value for the `ICC_HSRE` system register.
    pub icc_hsre: IccHsre,
    /// Fake value for the `ICC_IAFFIDR_EL1` system register.
    pub icc_iaffidr_el1: IccIaffidrEl1,
    /// Fake value for the `ICC_IAR0` system register.
    pub icc_iar0: IccIar0,
    /// Fake value for the `ICC_IAR0_EL1` system register.
    pub icc_iar0_el1: IccIar0El1,
    /// Fake value for the `ICC_IAR1` system register.
    pub icc_iar1: IccIar1,
    /// Fake value for the `ICC_IAR1_EL1` system register.
    pub icc_iar1_el1: IccIar1El1,
    /// Fake value for the `ICC_ICSR_EL1` system register.
    pub icc_icsr_el1: IccIcsrEl1,
    /// Fake value for the `ICC_IDR0_EL1` system register.
    pub icc_idr0_el1: IccIdr0El1,
    /// Fake value for the `ICC_IGRPEN0` system register.
    pub icc_igrpen0: IccIgrpen0,
    /// Fake value for the `ICC_IGRPEN0_EL1` system register.
    pub icc_igrpen0_el1: IccIgrpen0El1,
    /// Fake value for the `ICC_IGRPEN1` system register.
    pub icc_igrpen1: IccIgrpen1,
    /// Fake value for the `ICC_IGRPEN1_EL1` system register.
    pub icc_igrpen1_el1: IccIgrpen1El1,
    /// Fake value for the `ICC_IGRPEN1_EL3` system register.
    pub icc_igrpen1_el3: IccIgrpen1El3,
    /// Fake value for the `ICC_MCTLR` system register.
    pub icc_mctlr: IccMctlr,
    /// Fake value for the `ICC_MGRPEN1` system register.
    pub icc_mgrpen1: IccMgrpen1,
    /// Fake value for the `ICC_MSRE` system register.
    pub icc_msre: IccMsre,
    /// Fake value for the `ICC_NMIAR1_EL1` system register.
    pub icc_nmiar1_el1: IccNmiar1El1,
    /// Fake value for the `ICC_PCR_EL1` system register.
    pub icc_pcr_el1: IccPcrEl1,
    /// Fake value for the `ICC_PCR_EL3` system register.
    pub icc_pcr_el3: IccPcrEl3,
    /// Fake value for the `ICC_PMR` system register.
    pub icc_pmr: IccPmr,
    /// Fake value for the `ICC_PMR_EL1` system register.
    pub icc_pmr_el1: IccPmrEl1,
    /// Fake value for the `ICC_RPR` system register.
    pub icc_rpr: IccRpr,
    /// Fake value for the `ICC_RPR_EL1` system register.
    pub icc_rpr_el1: IccRprEl1,
    /// Fake value for the `ICC_SGI0R_EL1` system register.
    pub icc_sgi0r_el1: IccSgi0rEl1,
    /// Fake value for the `ICC_SGI1R_EL1` system register.
    pub icc_sgi1r_el1: IccSgi1rEl1,
    /// Fake value for the `ICC_SRE` system register.
    pub icc_sre: IccSre,
    /// Fake value for the `ICC_SRE_EL1` system register.
    pub icc_sre_el1: IccSreEl1,
    /// Fake value for the `ICC_SRE_EL2` system register.
    pub icc_sre_el2: IccSreEl2,
    /// Fake value for the `ICC_SRE_EL3` system register.
    pub icc_sre_el3: IccSreEl3,
    /// Fake value for the `ICH_APR_EL2` system register.
    pub ich_apr_el2: IchAprEl2,
    /// Fake value for the `ICH_CONTEXTR_EL2` system register.
    pub ich_contextr_el2: IchContextrEl2,
    /// Fake value for the `ICH_EISR` system register.
    pub ich_eisr: IchEisr,
    /// Fake value for the `ICH_EISR_EL2` system register.
    pub ich_eisr_el2: IchEisrEl2,
    /// Fake value for the `ICH_ELRSR` system register.
    pub ich_elrsr: IchElrsr,
    /// Fake value for the `ICH_ELRSR_EL2` system register.
    pub ich_elrsr_el2: IchElrsrEl2,
    /// Fake value for the `ICH_HCR` system register.
    pub ich_hcr: IchHcr,
    /// Fake value for the `ICH_HCR_EL2` system register.
    pub ich_hcr_el2: IchHcrEl2,
    /// Fake value for the `ICH_HFGITR_EL2` system register.
    pub ich_hfgitr_el2: IchHfgitrEl2,
    /// Fake value for the `ICH_HFGRTR_EL2` system register.
    pub ich_hfgrtr_el2: IchHfgrtrEl2,
    /// Fake value for the `ICH_HFGWTR_EL2` system register.
    pub ich_hfgwtr_el2: IchHfgwtrEl2,
    /// Fake value for the `ICH_HPPIR_EL2` system register.
    pub ich_hppir_el2: IchHppirEl2,
    /// Fake value for the `ICH_MISR` system register.
    pub ich_misr: IchMisr,
    /// Fake value for the `ICH_MISR_EL2` system register.
    pub ich_misr_el2: IchMisrEl2,
    /// Fake value for the `ICH_VCTLR_EL2` system register.
    pub ich_vctlr_el2: IchVctlrEl2,
    /// Fake value for the `ICH_VMCR` system register.
    pub ich_vmcr: IchVmcr,
    /// Fake value for the `ICH_VMCR_EL2` system register.
    pub ich_vmcr_el2: IchVmcrEl2,
    /// Fake value for the `ICH_VTR` system register.
    pub ich_vtr: IchVtr,
    /// Fake value for the `ICH_VTR_EL2` system register.
    pub ich_vtr_el2: IchVtrEl2,
    /// Fake value for the `ICIALLU` system register.
    pub iciallu: u32,
    /// Fake value for the `ICIALLUIS` system register.
    pub icialluis: u32,
    /// Fake value for the `ICIMVAU` system register.
    pub icimvau: Icimvau,
    /// Fake value for the `ICV_APR_EL1` system register.
    pub icv_apr_el1: IcvAprEl1,
    /// Fake value for the `ICV_BPR0` system register.
    pub icv_bpr0: IcvBpr0,
    /// Fake value for the `ICV_BPR0_EL1` system register.
    pub icv_bpr0_el1: IcvBpr0El1,
    /// Fake value for the `ICV_BPR1` system register.
    pub icv_bpr1: IcvBpr1,
    /// Fake value for the `ICV_BPR1_EL1` system register.
    pub icv_bpr1_el1: IcvBpr1El1,
    /// Fake value for the `ICV_CR0_EL1` system register.
    pub icv_cr0_el1: IcvCr0El1,
    /// Fake value for the `ICV_CTLR` system register.
    pub icv_ctlr: IcvCtlr,
    /// Fake value for the `ICV_CTLR_EL1` system register.
    pub icv_ctlr_el1: IcvCtlrEl1,
    /// Fake value for the `ICV_DIR` system register.
    pub icv_dir: IcvDir,
    /// Fake value for the `ICV_DIR_EL1` system register.
    pub icv_dir_el1: IcvDirEl1,
    /// Fake value for the `ICV_EOIR0` system register.
    pub icv_eoir0: IcvEoir0,
    /// Fake value for the `ICV_EOIR0_EL1` system register.
    pub icv_eoir0_el1: IcvEoir0El1,
    /// Fake value for the `ICV_EOIR1` system register.
    pub icv_eoir1: IcvEoir1,
    /// Fake value for the `ICV_EOIR1_EL1` system register.
    pub icv_eoir1_el1: IcvEoir1El1,
    /// Fake value for the `ICV_HAPR_EL1` system register.
    pub icv_hapr_el1: IcvHaprEl1,
    /// Fake value for the `ICV_HPPIR0` system register.
    pub icv_hppir0: IcvHppir0,
    /// Fake value for the `ICV_HPPIR0_EL1` system register.
    pub icv_hppir0_el1: IcvHppir0El1,
    /// Fake value for the `ICV_HPPIR1` system register.
    pub icv_hppir1: IcvHppir1,
    /// Fake value for the `ICV_HPPIR1_EL1` system register.
    pub icv_hppir1_el1: IcvHppir1El1,
    /// Fake value for the `ICV_HPPIR_EL1` system register.
    pub icv_hppir_el1: IcvHppirEl1,
    /// Fake value for the `ICV_IAR0` system register.
    pub icv_iar0: IcvIar0,
    /// Fake value for the `ICV_IAR0_EL1` system register.
    pub icv_iar0_el1: IcvIar0El1,
    /// Fake value for the `ICV_IAR1` system register.
    pub icv_iar1: IcvIar1,
    /// Fake value for the `ICV_IAR1_EL1` system register.
    pub icv_iar1_el1: IcvIar1El1,
    /// Fake value for the `ICV_IGRPEN0` system register.
    pub icv_igrpen0: IcvIgrpen0,
    /// Fake value for the `ICV_IGRPEN0_EL1` system register.
    pub icv_igrpen0_el1: IcvIgrpen0El1,
    /// Fake value for the `ICV_IGRPEN1` system register.
    pub icv_igrpen1: IcvIgrpen1,
    /// Fake value for the `ICV_IGRPEN1_EL1` system register.
    pub icv_igrpen1_el1: IcvIgrpen1El1,
    /// Fake value for the `ICV_NMIAR1_EL1` system register.
    pub icv_nmiar1_el1: IcvNmiar1El1,
    /// Fake value for the `ICV_PCR_EL1` system register.
    pub icv_pcr_el1: IcvPcrEl1,
    /// Fake value for the `ICV_PMR` system register.
    pub icv_pmr: IcvPmr,
    /// Fake value for the `ICV_PMR_EL1` system register.
    pub icv_pmr_el1: IcvPmrEl1,
    /// Fake value for the `ICV_RPR` system register.
    pub icv_rpr: IcvRpr,
    /// Fake value for the `ICV_RPR_EL1` system register.
    pub icv_rpr_el1: IcvRprEl1,
    /// Fake value for the `ID_AA64AFR0_EL1` system register.
    pub id_aa64afr0_el1: u64,
    /// Fake value for the `ID_AA64AFR1_EL1` system register.
    pub id_aa64afr1_el1: u64,
    /// Fake value for the `ID_AA64DFR0_EL1` system register.
    pub id_aa64dfr0_el1: IdAa64dfr0El1,
    /// Fake value for the `ID_AA64DFR1_EL1` system register.
    pub id_aa64dfr1_el1: IdAa64dfr1El1,
    /// Fake value for the `ID_AA64DFR2_EL1` system register.
    pub id_aa64dfr2_el1: IdAa64dfr2El1,
    /// Fake value for the `ID_AA64FPFR0_EL1` system register.
    pub id_aa64fpfr0_el1: IdAa64fpfr0El1,
    /// Fake value for the `ID_AA64ISAR0_EL1` system register.
    pub id_aa64isar0_el1: IdAa64isar0El1,
    /// Fake value for the `ID_AA64ISAR1_EL1` system register.
    pub id_aa64isar1_el1: IdAa64isar1El1,
    /// Fake value for the `ID_AA64ISAR2_EL1` system register.
    pub id_aa64isar2_el1: IdAa64isar2El1,
    /// Fake value for the `ID_AA64ISAR3_EL1` system register.
    pub id_aa64isar3_el1: IdAa64isar3El1,
    /// Fake value for the `ID_AA64MMFR0_EL1` system register.
    pub id_aa64mmfr0_el1: IdAa64mmfr0El1,
    /// Fake value for the `ID_AA64MMFR1_EL1` system register.
    pub id_aa64mmfr1_el1: IdAa64mmfr1El1,
    /// Fake value for the `ID_AA64MMFR2_EL1` system register.
    pub id_aa64mmfr2_el1: IdAa64mmfr2El1,
    /// Fake value for the `ID_AA64MMFR3_EL1` system register.
    pub id_aa64mmfr3_el1: IdAa64mmfr3El1,
    /// Fake value for the `ID_AA64MMFR4_EL1` system register.
    pub id_aa64mmfr4_el1: IdAa64mmfr4El1,
    /// Fake value for the `ID_AA64PFR0_EL1` system register.
    pub id_aa64pfr0_el1: IdAa64pfr0El1,
    /// Fake value for the `ID_AA64PFR1_EL1` system register.
    pub id_aa64pfr1_el1: IdAa64pfr1El1,
    /// Fake value for the `ID_AA64PFR2_EL1` system register.
    pub id_aa64pfr2_el1: IdAa64pfr2El1,
    /// Fake value for the `ID_AA64SMFR0_EL1` system register.
    pub id_aa64smfr0_el1: IdAa64smfr0El1,
    /// Fake value for the `ID_AA64ZFR0_EL1` system register.
    pub id_aa64zfr0_el1: IdAa64zfr0El1,
    /// Fake value for the `ID_AFR0` system register.
    pub id_afr0: u32,
    /// Fake value for the `ID_AFR0_EL1` system register.
    pub id_afr0_el1: u64,
    /// Fake value for the `ID_DFR0` system register.
    pub id_dfr0: IdDfr0,
    /// Fake value for the `ID_DFR0_EL1` system register.
    pub id_dfr0_el1: IdDfr0El1,
    /// Fake value for the `ID_DFR1` system register.
    pub id_dfr1: IdDfr1,
    /// Fake value for the `ID_DFR1_EL1` system register.
    pub id_dfr1_el1: IdDfr1El1,
    /// Fake value for the `ID_ISAR0` system register.
    pub id_isar0: IdIsar0,
    /// Fake value for the `ID_ISAR0_EL1` system register.
    pub id_isar0_el1: IdIsar0El1,
    /// Fake value for the `ID_ISAR1` system register.
    pub id_isar1: IdIsar1,
    /// Fake value for the `ID_ISAR1_EL1` system register.
    pub id_isar1_el1: IdIsar1El1,
    /// Fake value for the `ID_ISAR2` system register.
    pub id_isar2: IdIsar2,
    /// Fake value for the `ID_ISAR2_EL1` system register.
    pub id_isar2_el1: IdIsar2El1,
    /// Fake value for the `ID_ISAR3` system register.
    pub id_isar3: IdIsar3,
    /// Fake value for the `ID_ISAR3_EL1` system register.
    pub id_isar3_el1: IdIsar3El1,
    /// Fake value for the `ID_ISAR4` system register.
    pub id_isar4: IdIsar4,
    /// Fake value for the `ID_ISAR4_EL1` system register.
    pub id_isar4_el1: IdIsar4El1,
    /// Fake value for the `ID_ISAR5` system register.
    pub id_isar5: IdIsar5,
    /// Fake value for the `ID_ISAR5_EL1` system register.
    pub id_isar5_el1: IdIsar5El1,
    /// Fake value for the `ID_ISAR6` system register.
    pub id_isar6: IdIsar6,
    /// Fake value for the `ID_ISAR6_EL1` system register.
    pub id_isar6_el1: IdIsar6El1,
    /// Fake value for the `ID_MMFR0` system register.
    pub id_mmfr0: IdMmfr0,
    /// Fake value for the `ID_MMFR0_EL1` system register.
    pub id_mmfr0_el1: IdMmfr0El1,
    /// Fake value for the `ID_MMFR1` system register.
    pub id_mmfr1: IdMmfr1,
    /// Fake value for the `ID_MMFR1_EL1` system register.
    pub id_mmfr1_el1: IdMmfr1El1,
    /// Fake value for the `ID_MMFR2` system register.
    pub id_mmfr2: IdMmfr2,
    /// Fake value for the `ID_MMFR2_EL1` system register.
    pub id_mmfr2_el1: IdMmfr2El1,
    /// Fake value for the `ID_MMFR3` system register.
    pub id_mmfr3: IdMmfr3,
    /// Fake value for the `ID_MMFR3_EL1` system register.
    pub id_mmfr3_el1: IdMmfr3El1,
    /// Fake value for the `ID_MMFR4` system register.
    pub id_mmfr4: IdMmfr4,
    /// Fake value for the `ID_MMFR4_EL1` system register.
    pub id_mmfr4_el1: IdMmfr4El1,
    /// Fake value for the `ID_MMFR5` system register.
    pub id_mmfr5: IdMmfr5,
    /// Fake value for the `ID_MMFR5_EL1` system register.
    pub id_mmfr5_el1: IdMmfr5El1,
    /// Fake value for the `ID_PFR0` system register.
    pub id_pfr0: IdPfr0,
    /// Fake value for the `ID_PFR0_EL1` system register.
    pub id_pfr0_el1: IdPfr0El1,
    /// Fake value for the `ID_PFR1` system register.
    pub id_pfr1: IdPfr1,
    /// Fake value for the `ID_PFR1_EL1` system register.
    pub id_pfr1_el1: IdPfr1El1,
    /// Fake value for the `ID_PFR2` system register.
    pub id_pfr2: IdPfr2,
    /// Fake value for the `ID_PFR2_EL1` system register.
    pub id_pfr2_el1: IdPfr2El1,
    /// Fake value for the `IFAR` system register.
    pub ifar: Ifar,
    /// Fake value for the `IFSR` system register.
    pub ifsr: Ifsr,
    /// Fake value for the `IFSR32_EL2` system register.
    pub ifsr32_el2: Ifsr32El2,
    /// Fake value for the `IRTBRP_EL1` system register.
    pub irtbrp_el1: IrtbrpEl1,
    /// Fake value for the `IRTBRP_EL2` system register.
    pub irtbrp_el2: IrtbrpEl2,
    /// Fake value for the `IRTBRP_EL3` system register.
    pub irtbrp_el3: IrtbrpEl3,
    /// Fake value for the `IRTBRU_EL1` system register.
    pub irtbru_el1: IrtbruEl1,
    /// Fake value for the `IRTBRU_EL2` system register.
    pub irtbru_el2: IrtbruEl2,
    /// Fake value for the `ISR` system register.
    pub isr: Isr,
    /// Fake value for the `ISR_EL1` system register.
    pub isr_el1: IsrEl1,
    /// Fake value for the `ITLBIALL` system register.
    pub itlbiall: u32,
    /// Fake value for the `ITLBIASID` system register.
    pub itlbiasid: Itlbiasid,
    /// Fake value for the `ITLBIMVA` system register.
    pub itlbimva: Itlbimva,
    /// Fake value for the `JIDR` system register.
    pub jidr: u32,
    /// Fake value for the `JMCR` system register.
    pub jmcr: u32,
    /// Fake value for the `JOSCR` system register.
    pub joscr: u32,
    /// Fake value for the `LDSTT_EL1` system register.
    pub ldstt_el1: LdsttEl1,
    /// Fake value for the `LDSTT_EL2` system register.
    pub ldstt_el2: LdsttEl2,
    /// Fake value for the `LORC_EL1` system register.
    pub lorc_el1: LorcEl1,
    /// Fake value for the `LOREA_EL1` system register.
    pub lorea_el1: LoreaEl1,
    /// Fake value for the `LORID_EL1` system register.
    pub lorid_el1: LoridEl1,
    /// Fake value for the `LORN_EL1` system register.
    pub lorn_el1: LornEl1,
    /// Fake value for the `LORSA_EL1` system register.
    pub lorsa_el1: LorsaEl1,
    /// Fake value for the `MAIR0` system register.
    pub mair0: Mair0,
    /// Fake value for the `MAIR1` system register.
    pub mair1: Mair1,
    /// Fake value for the `MAIR2_EL1` system register.
    pub mair2_el1: Mair2El1,
    /// Fake value for the `MAIR2_EL2` system register.
    pub mair2_el2: Mair2El2,
    /// Fake value for the `MAIR2_EL3` system register.
    pub mair2_el3: Mair2El3,
    /// Fake value for the `MAIR_EL1` system register.
    pub mair_el1: MairEl1,
    /// Fake value for the `MAIR_EL2` system register.
    pub mair_el2: MairEl2,
    /// Fake value for the `MAIR_EL3` system register.
    pub mair_el3: MairEl3,
    /// Fake value for the `MDCCINT_EL1` system register.
    pub mdccint_el1: MdccintEl1,
    /// Fake value for the `MDCCSR_EL0` system register.
    pub mdccsr_el0: MdccsrEl0,
    /// Fake value for the `MDCR_EL2` system register.
    pub mdcr_el2: MdcrEl2,
    /// Fake value for the `MDCR_EL3` system register.
    pub mdcr_el3: MdcrEl3,
    /// Fake value for the `MDRAR_EL1` system register.
    pub mdrar_el1: MdrarEl1,
    /// Fake value for the `MDSCR_EL1` system register.
    pub mdscr_el1: MdscrEl1,
    /// Fake value for the `MDSELR_EL1` system register.
    pub mdselr_el1: MdselrEl1,
    /// Fake value for the `MDSTEPOP_EL1` system register.
    pub mdstepop_el1: MdstepopEl1,
    /// Fake value for the `MECIDR_EL2` system register.
    pub mecidr_el2: MecidrEl2,
    /// Fake value for the `MECID_A0_EL2` system register.
    pub mecid_a0_el2: MecidA0El2,
    /// Fake value for the `MECID_A1_EL2` system register.
    pub mecid_a1_el2: MecidA1El2,
    /// Fake value for the `MECID_P0_EL2` system register.
    pub mecid_p0_el2: MecidP0El2,
    /// Fake value for the `MECID_P1_EL2` system register.
    pub mecid_p1_el2: MecidP1El2,
    /// Fake value for the `MECID_RL_A_EL3` system register.
    pub mecid_rl_a_el3: MecidRlAEl3,
    /// Fake value for the `MFAR_EL3` system register.
    pub mfar_el3: MfarEl3,
    /// Fake value for the `MIDR` system register.
    pub midr: Midr,
    /// Fake value for the `MIDR_EL1` system register.
    pub midr_el1: MidrEl1,
    /// Fake value for the `MPAM0_EL1` system register.
    pub mpam0_el1: Mpam0El1,
    /// Fake value for the `MPAM1_EL1` system register.
    pub mpam1_el1: Mpam1El1,
    /// Fake value for the `MPAM2_EL2` system register.
    pub mpam2_el2: Mpam2El2,
    /// Fake value for the `MPAM3_EL3` system register.
    pub mpam3_el3: Mpam3El3,
    /// Fake value for the `MPAMBW0_EL1` system register.
    pub mpambw0_el1: Mpambw0El1,
    /// Fake value for the `MPAMBW1_EL1` system register.
    pub mpambw1_el1: Mpambw1El1,
    /// Fake value for the `MPAMBW2_EL2` system register.
    pub mpambw2_el2: Mpambw2El2,
    /// Fake value for the `MPAMBW3_EL3` system register.
    pub mpambw3_el3: Mpambw3El3,
    /// Fake value for the `MPAMBWCAP_EL2` system register.
    pub mpambwcap_el2: MpambwcapEl2,
    /// Fake value for the `MPAMBWIDR_EL1` system register.
    pub mpambwidr_el1: MpambwidrEl1,
    /// Fake value for the `MPAMBWSM_EL1` system register.
    pub mpambwsm_el1: MpambwsmEl1,
    /// Fake value for the `MPAMCTL_EL1` system register.
    pub mpamctl_el1: MpamctlEl1,
    /// Fake value for the `MPAMCTL_EL2` system register.
    pub mpamctl_el2: MpamctlEl2,
    /// Fake value for the `MPAMCTL_EL3` system register.
    pub mpamctl_el3: MpamctlEl3,
    /// Fake value for the `MPAMHCR_EL2` system register.
    pub mpamhcr_el2: MpamhcrEl2,
    /// Fake value for the `MPAMIDR_EL1` system register.
    pub mpamidr_el1: MpamidrEl1,
    /// Fake value for the `MPAMSM_EL1` system register.
    pub mpamsm_el1: MpamsmEl1,
    /// Fake value for the `MPAMVIDCR_EL2` system register.
    pub mpamvidcr_el2: MpamvidcrEl2,
    /// Fake value for the `MPAMVIDSR_EL2` system register.
    pub mpamvidsr_el2: MpamvidsrEl2,
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
    /// Fake value for the `MPIDR` system register.
    pub mpidr: Mpidr,
    /// Fake value for the `MPIDR_EL1` system register.
    pub mpidr_el1: MpidrEl1,
    /// Fake value for the `MVBAR` system register.
    pub mvbar: Mvbar,
    /// Fake value for the `MVFR0_EL1` system register.
    pub mvfr0_el1: Mvfr0El1,
    /// Fake value for the `MVFR1_EL1` system register.
    pub mvfr1_el1: Mvfr1El1,
    /// Fake value for the `MVFR2_EL1` system register.
    pub mvfr2_el1: Mvfr2El1,
    /// Fake value for the `NMRR` system register.
    pub nmrr: Nmrr,
    /// Fake value for the `NSACR` system register.
    pub nsacr: Nsacr,
    /// Fake value for the `NVHCRMASK_EL2` system register.
    pub nvhcrmask_el2: NvhcrmaskEl2,
    /// Fake value for the `NVHCRXMASK_EL2` system register.
    pub nvhcrxmask_el2: NvhcrxmaskEl2,
    /// Fake value for the `NVHCRX_EL2` system register.
    pub nvhcrx_el2: NvhcrxEl2,
    /// Fake value for the `NVHCR_EL2` system register.
    pub nvhcr_el2: NvhcrEl2,
    /// Fake value for the `NZCV` system register.
    pub nzcv: Nzcv,
    /// Fake value for the `OSDLR_EL1` system register.
    pub osdlr_el1: OsdlrEl1,
    /// Fake value for the `OSDTRRX_EL1` system register.
    pub osdtrrx_el1: OsdtrrxEl1,
    /// Fake value for the `OSDTRTX_EL1` system register.
    pub osdtrtx_el1: OsdtrtxEl1,
    /// Fake value for the `OSECCR_EL1` system register.
    pub oseccr_el1: OseccrEl1,
    /// Fake value for the `OSLAR_EL1` system register.
    pub oslar_el1: OslarEl1,
    /// Fake value for the `OSLSR_EL1` system register.
    pub oslsr_el1: OslsrEl1,
    /// Fake value for the `PAN` system register.
    pub pan: Pan,
    /// Fake value for the `PAR` system register.
    pub par: Par,
    /// Fake value for the `PAR_EL1` system register.
    pub par_el1: ParEl1,
    /// Fake value for the `PFAR_EL1` system register.
    pub pfar_el1: PfarEl1,
    /// Fake value for the `PFAR_EL2` system register.
    pub pfar_el2: PfarEl2,
    /// Fake value for the `PIRE0_EL1` system register.
    pub pire0_el1: Pire0El1,
    /// Fake value for the `PIRE0_EL2` system register.
    pub pire0_el2: Pire0El2,
    /// Fake value for the `PIR_EL1` system register.
    pub pir_el1: PirEl1,
    /// Fake value for the `PIR_EL2` system register.
    pub pir_el2: PirEl2,
    /// Fake value for the `PIR_EL3` system register.
    pub pir_el3: PirEl3,
    /// Fake value for the `PM` system register.
    pub pm: Pm,
    /// Fake value for the `PMBIDR_EL1` system register.
    pub pmbidr_el1: PmbidrEl1,
    /// Fake value for the `PMBLIMITR_EL1` system register.
    pub pmblimitr_el1: PmblimitrEl1,
    /// Fake value for the `PMBMAR_EL1` system register.
    pub pmbmar_el1: PmbmarEl1,
    /// Fake value for the `PMBPTR_EL1` system register.
    pub pmbptr_el1: PmbptrEl1,
    /// Fake value for the `PMBSR_EL1` system register.
    pub pmbsr_el1: PmbsrEl1,
    /// Fake value for the `PMBSR_EL2` system register.
    pub pmbsr_el2: PmbsrEl2,
    /// Fake value for the `PMBSR_EL3` system register.
    pub pmbsr_el3: PmbsrEl3,
    /// Fake value for the `PMCCFILTR` system register.
    pub pmccfiltr: Pmccfiltr,
    /// Fake value for the `PMCCFILTR_EL0` system register.
    pub pmccfiltr_el0: PmccfiltrEl0,
    /// Fake value for the `PMCCNTR` system register.
    pub pmccntr: Pmccntr,
    /// Fake value for the `PMCCNTR_EL0` system register.
    pub pmccntr_el0: PmccntrEl0,
    /// Fake value for the `PMCCNTSVR_EL1` system register.
    pub pmccntsvr_el1: PmccntsvrEl1,
    /// Fake value for the `PMCEID0` system register.
    pub pmceid0: Pmceid0,
    /// Fake value for the `PMCEID0_EL0` system register.
    pub pmceid0_el0: Pmceid0El0,
    /// Fake value for the `PMCEID1` system register.
    pub pmceid1: Pmceid1,
    /// Fake value for the `PMCEID1_EL0` system register.
    pub pmceid1_el0: Pmceid1El0,
    /// Fake value for the `PMCEID2` system register.
    pub pmceid2: Pmceid2,
    /// Fake value for the `PMCEID3` system register.
    pub pmceid3: Pmceid3,
    /// Fake value for the `PMCNTENCLR` system register.
    pub pmcntenclr: Pmcntenclr,
    /// Fake value for the `PMCNTENCLR_EL0` system register.
    pub pmcntenclr_el0: PmcntenclrEl0,
    /// Fake value for the `PMCNTENSET` system register.
    pub pmcntenset: Pmcntenset,
    /// Fake value for the `PMCNTENSET_EL0` system register.
    pub pmcntenset_el0: PmcntensetEl0,
    /// Fake value for the `PMCR` system register.
    pub pmcr: Pmcr,
    /// Fake value for the `PMCR_EL0` system register.
    pub pmcr_el0: PmcrEl0,
    /// Fake value for the `PMECR_EL1` system register.
    pub pmecr_el1: PmecrEl1,
    /// Fake value for the `PMIAR_EL1` system register.
    pub pmiar_el1: PmiarEl1,
    /// Fake value for the `PMICFILTR_EL0` system register.
    pub pmicfiltr_el0: PmicfiltrEl0,
    /// Fake value for the `PMICNTR_EL0` system register.
    pub pmicntr_el0: PmicntrEl0,
    /// Fake value for the `PMICNTSVR_EL1` system register.
    pub pmicntsvr_el1: PmicntsvrEl1,
    /// Fake value for the `PMINTENCLR` system register.
    pub pmintenclr: Pmintenclr,
    /// Fake value for the `PMINTENCLR_EL1` system register.
    pub pmintenclr_el1: PmintenclrEl1,
    /// Fake value for the `PMINTENSET` system register.
    pub pmintenset: Pmintenset,
    /// Fake value for the `PMINTENSET_EL1` system register.
    pub pmintenset_el1: PmintensetEl1,
    /// Fake value for the `PMMIR` system register.
    pub pmmir: Pmmir,
    /// Fake value for the `PMMIR_EL1` system register.
    pub pmmir_el1: PmmirEl1,
    /// Fake value for the `PMOVSCLR_EL0` system register.
    pub pmovsclr_el0: PmovsclrEl0,
    /// Fake value for the `PMOVSR` system register.
    pub pmovsr: Pmovsr,
    /// Fake value for the `PMOVSSET` system register.
    pub pmovsset: Pmovsset,
    /// Fake value for the `PMOVSSET_EL0` system register.
    pub pmovsset_el0: PmovssetEl0,
    /// Fake value for the `PMSCR_EL1` system register.
    pub pmscr_el1: PmscrEl1,
    /// Fake value for the `PMSCR_EL2` system register.
    pub pmscr_el2: PmscrEl2,
    /// Fake value for the `PMSDSFR_EL1` system register.
    pub pmsdsfr_el1: PmsdsfrEl1,
    /// Fake value for the `PMSELR` system register.
    pub pmselr: Pmselr,
    /// Fake value for the `PMSELR_EL0` system register.
    pub pmselr_el0: PmselrEl0,
    /// Fake value for the `PMSEVFR_EL1` system register.
    pub pmsevfr_el1: PmsevfrEl1,
    /// Fake value for the `PMSFCR_EL1` system register.
    pub pmsfcr_el1: PmsfcrEl1,
    /// Fake value for the `PMSICR_EL1` system register.
    pub pmsicr_el1: PmsicrEl1,
    /// Fake value for the `PMSIDR_EL1` system register.
    pub pmsidr_el1: PmsidrEl1,
    /// Fake value for the `PMSIRR_EL1` system register.
    pub pmsirr_el1: PmsirrEl1,
    /// Fake value for the `PMSLATFR_EL1` system register.
    pub pmslatfr_el1: PmslatfrEl1,
    /// Fake value for the `PMSNEVFR_EL1` system register.
    pub pmsnevfr_el1: PmsnevfrEl1,
    /// Fake value for the `PMSSCR_EL1` system register.
    pub pmsscr_el1: PmsscrEl1,
    /// Fake value for the `PMSWINC` system register.
    pub pmswinc: Pmswinc,
    /// Fake value for the `PMSWINC_EL0` system register.
    pub pmswinc_el0: PmswincEl0,
    /// Fake value for the `PMUACR_EL1` system register.
    pub pmuacr_el1: PmuacrEl1,
    /// Fake value for the `PMUSERENR` system register.
    pub pmuserenr: Pmuserenr,
    /// Fake value for the `PMUSERENR_EL0` system register.
    pub pmuserenr_el0: PmuserenrEl0,
    /// Fake value for the `PMXEVCNTR` system register.
    pub pmxevcntr: Pmxevcntr,
    /// Fake value for the `PMXEVCNTR_EL0` system register.
    pub pmxevcntr_el0: u64,
    /// Fake value for the `PMXEVTYPER` system register.
    pub pmxevtyper: Pmxevtyper,
    /// Fake value for the `PMXEVTYPER_EL0` system register.
    pub pmxevtyper_el0: PmxevtyperEl0,
    /// Fake value for the `PMZR_EL0` system register.
    pub pmzr_el0: PmzrEl0,
    /// Fake value for the `POR_EL0` system register.
    pub por_el0: PorEl0,
    /// Fake value for the `POR_EL1` system register.
    pub por_el1: PorEl1,
    /// Fake value for the `POR_EL2` system register.
    pub por_el2: PorEl2,
    /// Fake value for the `POR_EL3` system register.
    pub por_el3: PorEl3,
    /// Fake value for the `PRRR` system register.
    pub prrr: Prrr,
    /// Fake value for the `RCWMASK_EL1` system register.
    pub rcwmask_el1: u64,
    /// Fake value for the `RCWSMASK_EL1` system register.
    pub rcwsmask_el1: u64,
    /// Fake value for the `REVIDR` system register.
    pub revidr: u32,
    /// Fake value for the `REVIDR_EL1` system register.
    pub revidr_el1: u64,
    /// Fake value for the `RGSR_EL1` system register.
    pub rgsr_el1: RgsrEl1,
    /// Fake value for the `RMR` system register.
    pub rmr: Rmr,
    /// Fake value for the `RMR_EL1` system register.
    pub rmr_el1: RmrEl1,
    /// Fake value for the `RMR_EL2` system register.
    pub rmr_el2: RmrEl2,
    /// Fake value for the `RMR_EL3` system register.
    pub rmr_el3: RmrEl3,
    /// Fake value for the `RNDR` system register.
    pub rndr: Rndr,
    /// Fake value for the `RNDRRS` system register.
    pub rndrrs: Rndrrs,
    /// Fake value for the `RVBAR` system register.
    pub rvbar: Rvbar,
    /// Fake value for the `RVBAR_EL1` system register.
    pub rvbar_el1: RvbarEl1,
    /// Fake value for the `RVBAR_EL2` system register.
    pub rvbar_el2: RvbarEl2,
    /// Fake value for the `RVBAR_EL3` system register.
    pub rvbar_el3: RvbarEl3,
    /// Fake value for the `S2PIR_EL2` system register.
    pub s2pir_el2: S2pirEl2,
    /// Fake value for the `S2POR_EL1` system register.
    pub s2por_el1: S2porEl1,
    /// Fake value for the `SCR` system register.
    pub scr: Scr,
    /// Fake value for the `SCR2_EL3` system register.
    pub scr2_el3: Scr2El3,
    /// Fake value for the `SCR_EL3` system register.
    pub scr_el3: ScrEl3,
    /// Fake value for the `SCTLR` system register.
    pub sctlr: Sctlr,
    /// Fake value for the `SCTLR2MASK_EL1` system register.
    pub sctlr2mask_el1: Sctlr2maskEl1,
    /// Fake value for the `SCTLR2MASK_EL2` system register.
    pub sctlr2mask_el2: Sctlr2maskEl2,
    /// Fake value for the `SCTLR2_EL1` system register.
    pub sctlr2_el1: Sctlr2El1,
    /// Fake value for the `SCTLR2_EL2` system register.
    pub sctlr2_el2: Sctlr2El2,
    /// Fake value for the `SCTLR2_EL3` system register.
    pub sctlr2_el3: Sctlr2El3,
    /// Fake value for the `SCTLRMASK_EL1` system register.
    pub sctlrmask_el1: SctlrmaskEl1,
    /// Fake value for the `SCTLRMASK_EL2` system register.
    pub sctlrmask_el2: SctlrmaskEl2,
    /// Fake value for the `SCTLR_EL1` system register.
    pub sctlr_el1: SctlrEl1,
    /// Fake value for the `SCTLR_EL2` system register.
    pub sctlr_el2: SctlrEl2,
    /// Fake value for the `SCTLR_EL3` system register.
    pub sctlr_el3: SctlrEl3,
    /// Fake value for the `SCXTNUM_EL0` system register.
    pub scxtnum_el0: ScxtnumEl0,
    /// Fake value for the `SCXTNUM_EL1` system register.
    pub scxtnum_el1: ScxtnumEl1,
    /// Fake value for the `SCXTNUM_EL2` system register.
    pub scxtnum_el2: ScxtnumEl2,
    /// Fake value for the `SCXTNUM_EL3` system register.
    pub scxtnum_el3: ScxtnumEl3,
    /// Fake value for the `SDCR` system register.
    pub sdcr: Sdcr,
    /// Fake value for the `SDER` system register.
    pub sder: Sder,
    /// Fake value for the `SDER32_EL2` system register.
    pub sder32_el2: Sder32El2,
    /// Fake value for the `SDER32_EL3` system register.
    pub sder32_el3: Sder32El3,
    /// Fake value for the `SMCR_EL1` system register.
    pub smcr_el1: SmcrEl1,
    /// Fake value for the `SMCR_EL2` system register.
    pub smcr_el2: SmcrEl2,
    /// Fake value for the `SMCR_EL3` system register.
    pub smcr_el3: SmcrEl3,
    /// Fake value for the `SMIDR_EL1` system register.
    pub smidr_el1: SmidrEl1,
    /// Fake value for the `SMPRIMAP_EL2` system register.
    pub smprimap_el2: SmprimapEl2,
    /// Fake value for the `SMPRI_EL1` system register.
    pub smpri_el1: SmpriEl1,
    /// Fake value for the `SPMACCESSR_EL1` system register.
    pub spmaccessr_el1: SpmaccessrEl1,
    /// Fake value for the `SPMACCESSR_EL2` system register.
    pub spmaccessr_el2: SpmaccessrEl2,
    /// Fake value for the `SPMACCESSR_EL3` system register.
    pub spmaccessr_el3: SpmaccessrEl3,
    /// Fake value for the `SPMCFGR_EL1` system register.
    pub spmcfgr_el1: SpmcfgrEl1,
    /// Fake value for the `SPMCNTENCLR_EL0` system register.
    pub spmcntenclr_el0: SpmcntenclrEl0,
    /// Fake value for the `SPMCNTENSET_EL0` system register.
    pub spmcntenset_el0: SpmcntensetEl0,
    /// Fake value for the `SPMCR_EL0` system register.
    pub spmcr_el0: SpmcrEl0,
    /// Fake value for the `SPMDEVAFF_EL1` system register.
    pub spmdevaff_el1: SpmdevaffEl1,
    /// Fake value for the `SPMDEVARCH_EL1` system register.
    pub spmdevarch_el1: SpmdevarchEl1,
    /// Fake value for the `SPMIIDR_EL1` system register.
    pub spmiidr_el1: SpmiidrEl1,
    /// Fake value for the `SPMINTENCLR_EL1` system register.
    pub spmintenclr_el1: SpmintenclrEl1,
    /// Fake value for the `SPMINTENSET_EL1` system register.
    pub spmintenset_el1: SpmintensetEl1,
    /// Fake value for the `SPMOVSCLR_EL0` system register.
    pub spmovsclr_el0: SpmovsclrEl0,
    /// Fake value for the `SPMOVSSET_EL0` system register.
    pub spmovsset_el0: SpmovssetEl0,
    /// Fake value for the `SPMROOTCR_EL3` system register.
    pub spmrootcr_el3: SpmrootcrEl3,
    /// Fake value for the `SPMSCR_EL1` system register.
    pub spmscr_el1: SpmscrEl1,
    /// Fake value for the `SPMSELR_EL0` system register.
    pub spmselr_el0: SpmselrEl0,
    /// Fake value for the `SPMZR_EL0` system register.
    pub spmzr_el0: SpmzrEl0,
    /// Fake value for the `SPSR_EL1` system register.
    pub spsr_el1: SpsrEl1,
    /// Fake value for the `SPSR_EL2` system register.
    pub spsr_el2: SpsrEl2,
    /// Fake value for the `SPSR_EL3` system register.
    pub spsr_el3: SpsrEl3,
    /// Fake value for the `SPSR_abt` system register.
    pub spsr_abt: SpsrAbt,
    /// Fake value for the `SPSR_fiq` system register.
    pub spsr_fiq: SpsrFiq,
    /// Fake value for the `SPSR_irq` system register.
    pub spsr_irq: SpsrIrq,
    /// Fake value for the `SPSR_und` system register.
    pub spsr_und: SpsrUnd,
    /// Fake value for the `SPSel` system register.
    pub spsel: Spsel,
    /// Fake value for the `SP_EL0` system register.
    pub sp_el0: SpEl0,
    /// Fake value for the `SP_EL1` system register.
    pub sp_el1: SpEl1,
    /// Fake value for the `SP_EL2` system register.
    pub sp_el2: SpEl2,
    /// Fake value for the `SSBS` system register.
    pub ssbs: Ssbs,
    /// Fake value for the `STINDEX_EL1` system register.
    pub stindex_el1: StindexEl1,
    /// Fake value for the `STINDEX_EL2` system register.
    pub stindex_el2: StindexEl2,
    /// Fake value for the `STINDEX_EL3` system register.
    pub stindex_el3: StindexEl3,
    /// Fake value for the `SVCR` system register.
    pub svcr: Svcr,
    /// Fake value for the `TCMTR` system register.
    pub tcmtr: u32,
    /// Fake value for the `TCO` system register.
    pub tco: Tco,
    /// Fake value for the `TCR2MASK_EL1` system register.
    pub tcr2mask_el1: Tcr2maskEl1,
    /// Fake value for the `TCR2MASK_EL2` system register.
    pub tcr2mask_el2: Tcr2maskEl2,
    /// Fake value for the `TCR2_EL1` system register.
    pub tcr2_el1: Tcr2El1,
    /// Fake value for the `TCR2_EL2` system register.
    pub tcr2_el2: Tcr2El2,
    /// Fake value for the `TCRMASK_EL1` system register.
    pub tcrmask_el1: TcrmaskEl1,
    /// Fake value for the `TCRMASK_EL2` system register.
    pub tcrmask_el2: TcrmaskEl2,
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
    /// Fake value for the `TFSR_EL3` system register.
    pub tfsr_el3: TfsrEl3,
    /// Fake value for the `TINDEX_EL0` system register.
    pub tindex_el0: TindexEl0,
    /// Fake value for the `TINDEX_EL1` system register.
    pub tindex_el1: TindexEl1,
    /// Fake value for the `TINDEX_EL2` system register.
    pub tindex_el2: TindexEl2,
    /// Fake value for the `TINDEX_EL3` system register.
    pub tindex_el3: TindexEl3,
    /// Fake value for the `TLBIALL` system register.
    pub tlbiall: u32,
    /// Fake value for the `TLBIALLH` system register.
    pub tlbiallh: u32,
    /// Fake value for the `TLBIALLHIS` system register.
    pub tlbiallhis: u32,
    /// Fake value for the `TLBIALLIS` system register.
    pub tlbiallis: u32,
    /// Fake value for the `TLBIALLNSNH` system register.
    pub tlbiallnsnh: u32,
    /// Fake value for the `TLBIALLNSNHIS` system register.
    pub tlbiallnsnhis: u32,
    /// Fake value for the `TLBIASID` system register.
    pub tlbiasid: Tlbiasid,
    /// Fake value for the `TLBIASIDIS` system register.
    pub tlbiasidis: Tlbiasidis,
    /// Fake value for the `TLBIDIDR_EL1` system register.
    pub tlbididr_el1: TlbididrEl1,
    /// Fake value for the `TLBIIPAS2` system register.
    pub tlbiipas2: Tlbiipas2,
    /// Fake value for the `TLBIIPAS2IS` system register.
    pub tlbiipas2is: Tlbiipas2is,
    /// Fake value for the `TLBIIPAS2L` system register.
    pub tlbiipas2l: Tlbiipas2l,
    /// Fake value for the `TLBIIPAS2LIS` system register.
    pub tlbiipas2lis: Tlbiipas2lis,
    /// Fake value for the `TLBIMVA` system register.
    pub tlbimva: Tlbimva,
    /// Fake value for the `TLBIMVAA` system register.
    pub tlbimvaa: Tlbimvaa,
    /// Fake value for the `TLBIMVAAIS` system register.
    pub tlbimvaais: Tlbimvaais,
    /// Fake value for the `TLBIMVAAL` system register.
    pub tlbimvaal: Tlbimvaal,
    /// Fake value for the `TLBIMVAALIS` system register.
    pub tlbimvaalis: Tlbimvaalis,
    /// Fake value for the `TLBIMVAH` system register.
    pub tlbimvah: Tlbimvah,
    /// Fake value for the `TLBIMVAHIS` system register.
    pub tlbimvahis: Tlbimvahis,
    /// Fake value for the `TLBIMVAIS` system register.
    pub tlbimvais: Tlbimvais,
    /// Fake value for the `TLBIMVAL` system register.
    pub tlbimval: Tlbimval,
    /// Fake value for the `TLBIMVALH` system register.
    pub tlbimvalh: Tlbimvalh,
    /// Fake value for the `TLBIMVALHIS` system register.
    pub tlbimvalhis: Tlbimvalhis,
    /// Fake value for the `TLBIMVALIS` system register.
    pub tlbimvalis: Tlbimvalis,
    /// Fake value for the `TLBTR` system register.
    pub tlbtr: Tlbtr,
    /// Fake value for the `TPIDR2_EL0` system register.
    pub tpidr2_el0: Tpidr2El0,
    /// Fake value for the `TPIDR3_EL0` system register.
    pub tpidr3_el0: u64,
    /// Fake value for the `TPIDR3_EL1` system register.
    pub tpidr3_el1: u64,
    /// Fake value for the `TPIDR3_EL2` system register.
    pub tpidr3_el2: u64,
    /// Fake value for the `TPIDR3_EL3` system register.
    pub tpidr3_el3: u64,
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
    /// Fake value for the `TPIDR_EL1` system register.
    pub tpidr_el1: TpidrEl1,
    /// Fake value for the `TPIDR_EL2` system register.
    pub tpidr_el2: TpidrEl2,
    /// Fake value for the `TPIDR_EL3` system register.
    pub tpidr_el3: TpidrEl3,
    /// Fake value for the `TPMAX0_EL0` system register.
    pub tpmax0_el0: Tpmax0El0,
    /// Fake value for the `TPMAX0_EL1` system register.
    pub tpmax0_el1: Tpmax0El1,
    /// Fake value for the `TPMAX0_EL2` system register.
    pub tpmax0_el2: Tpmax0El2,
    /// Fake value for the `TPMAX1_EL0` system register.
    pub tpmax1_el0: Tpmax1El0,
    /// Fake value for the `TPMAX1_EL1` system register.
    pub tpmax1_el1: Tpmax1El1,
    /// Fake value for the `TPMAX1_EL2` system register.
    pub tpmax1_el2: Tpmax1El2,
    /// Fake value for the `TPMIN0_EL0` system register.
    pub tpmin0_el0: Tpmin0El0,
    /// Fake value for the `TPMIN0_EL1` system register.
    pub tpmin0_el1: Tpmin0El1,
    /// Fake value for the `TPMIN0_EL2` system register.
    pub tpmin0_el2: Tpmin0El2,
    /// Fake value for the `TPMIN1_EL0` system register.
    pub tpmin1_el0: Tpmin1El0,
    /// Fake value for the `TPMIN1_EL1` system register.
    pub tpmin1_el1: Tpmin1El1,
    /// Fake value for the `TPMIN1_EL2` system register.
    pub tpmin1_el2: Tpmin1El2,
    /// Fake value for the `TRBBASER_EL1` system register.
    pub trbbaser_el1: TrbbaserEl1,
    /// Fake value for the `TRBIDR_EL1` system register.
    pub trbidr_el1: TrbidrEl1,
    /// Fake value for the `TRBLIMITR_EL1` system register.
    pub trblimitr_el1: TrblimitrEl1,
    /// Fake value for the `TRBMAR_EL1` system register.
    pub trbmar_el1: TrbmarEl1,
    /// Fake value for the `TRBMPAM_EL1` system register.
    pub trbmpam_el1: TrbmpamEl1,
    /// Fake value for the `TRBPTR_EL1` system register.
    pub trbptr_el1: TrbptrEl1,
    /// Fake value for the `TRBSR_EL1` system register.
    pub trbsr_el1: TrbsrEl1,
    /// Fake value for the `TRBSR_EL2` system register.
    pub trbsr_el2: TrbsrEl2,
    /// Fake value for the `TRBSR_EL3` system register.
    pub trbsr_el3: TrbsrEl3,
    /// Fake value for the `TRBTRG_EL1` system register.
    pub trbtrg_el1: TrbtrgEl1,
    /// Fake value for the `TRCAUTHSTATUS` system register.
    pub trcauthstatus: Trcauthstatus,
    /// Fake value for the `TRCAUXCTLR` system register.
    pub trcauxctlr: u64,
    /// Fake value for the `TRCBBCTLR` system register.
    pub trcbbctlr: Trcbbctlr,
    /// Fake value for the `TRCCCCTLR` system register.
    pub trcccctlr: Trcccctlr,
    /// Fake value for the `TRCCIDCCTLR0` system register.
    pub trccidcctlr0: Trccidcctlr0,
    /// Fake value for the `TRCCIDCCTLR1` system register.
    pub trccidcctlr1: Trccidcctlr1,
    /// Fake value for the `TRCCLAIMCLR` system register.
    pub trcclaimclr: Trcclaimclr,
    /// Fake value for the `TRCCLAIMSET` system register.
    pub trcclaimset: Trcclaimset,
    /// Fake value for the `TRCCONFIGR` system register.
    pub trcconfigr: Trcconfigr,
    /// Fake value for the `TRCDEVARCH` system register.
    pub trcdevarch: Trcdevarch,
    /// Fake value for the `TRCDEVID` system register.
    pub trcdevid: u64,
    /// Fake value for the `TRCEVENTCTL0R` system register.
    pub trceventctl0r: Trceventctl0r,
    /// Fake value for the `TRCEVENTCTL1R` system register.
    pub trceventctl1r: Trceventctl1r,
    /// Fake value for the `TRCIDR0` system register.
    pub trcidr0: Trcidr0,
    /// Fake value for the `TRCIDR1` system register.
    pub trcidr1: Trcidr1,
    /// Fake value for the `TRCIDR10` system register.
    pub trcidr10: Trcidr10,
    /// Fake value for the `TRCIDR11` system register.
    pub trcidr11: Trcidr11,
    /// Fake value for the `TRCIDR12` system register.
    pub trcidr12: Trcidr12,
    /// Fake value for the `TRCIDR13` system register.
    pub trcidr13: Trcidr13,
    /// Fake value for the `TRCIDR2` system register.
    pub trcidr2: Trcidr2,
    /// Fake value for the `TRCIDR3` system register.
    pub trcidr3: Trcidr3,
    /// Fake value for the `TRCIDR4` system register.
    pub trcidr4: Trcidr4,
    /// Fake value for the `TRCIDR5` system register.
    pub trcidr5: Trcidr5,
    /// Fake value for the `TRCIDR6` system register.
    pub trcidr6: Trcidr6,
    /// Fake value for the `TRCIDR7` system register.
    pub trcidr7: u64,
    /// Fake value for the `TRCIDR8` system register.
    pub trcidr8: Trcidr8,
    /// Fake value for the `TRCIDR9` system register.
    pub trcidr9: Trcidr9,
    /// Fake value for the `TRCIMSPEC0` system register.
    pub trcimspec0: Trcimspec0,
    /// Fake value for the `TRCITECR_EL1` system register.
    pub trcitecr_el1: TrcitecrEl1,
    /// Fake value for the `TRCITECR_EL2` system register.
    pub trcitecr_el2: TrcitecrEl2,
    /// Fake value for the `TRCITEEDCR` system register.
    pub trciteedcr: Trciteedcr,
    /// Fake value for the `TRCOSLSR` system register.
    pub trcoslsr: Trcoslsr,
    /// Fake value for the `TRCPRGCTLR` system register.
    pub trcprgctlr: Trcprgctlr,
    /// Fake value for the `TRCQCTLR` system register.
    pub trcqctlr: Trcqctlr,
    /// Fake value for the `TRCRSR` system register.
    pub trcrsr: Trcrsr,
    /// Fake value for the `TRCSEQRSTEVR` system register.
    pub trcseqrstevr: Trcseqrstevr,
    /// Fake value for the `TRCSEQSTR` system register.
    pub trcseqstr: Trcseqstr,
    /// Fake value for the `TRCSTALLCTLR` system register.
    pub trcstallctlr: Trcstallctlr,
    /// Fake value for the `TRCSTATR` system register.
    pub trcstatr: Trcstatr,
    /// Fake value for the `TRCSYNCPR` system register.
    pub trcsyncpr: Trcsyncpr,
    /// Fake value for the `TRCTRACEIDR` system register.
    pub trctraceidr: Trctraceidr,
    /// Fake value for the `TRCTSCTLR` system register.
    pub trctsctlr: Trctsctlr,
    /// Fake value for the `TRCVICTLR` system register.
    pub trcvictlr: Trcvictlr,
    /// Fake value for the `TRCVIIECTLR` system register.
    pub trcviiectlr: Trcviiectlr,
    /// Fake value for the `TRCVIPCSSCTLR` system register.
    pub trcvipcssctlr: Trcvipcssctlr,
    /// Fake value for the `TRCVISSCTLR` system register.
    pub trcvissctlr: Trcvissctlr,
    /// Fake value for the `TRCVMIDCCTLR0` system register.
    pub trcvmidcctlr0: Trcvmidcctlr0,
    /// Fake value for the `TRCVMIDCCTLR1` system register.
    pub trcvmidcctlr1: Trcvmidcctlr1,
    /// Fake value for the `TRFCR` system register.
    pub trfcr: Trfcr,
    /// Fake value for the `TRFCR_EL1` system register.
    pub trfcr_el1: TrfcrEl1,
    /// Fake value for the `TRFCR_EL2` system register.
    pub trfcr_el2: TrfcrEl2,
    /// Fake value for the `TTBCR` system register.
    pub ttbcr: Ttbcr,
    /// Fake value for the `TTBCR2` system register.
    pub ttbcr2: Ttbcr2,
    /// Fake value for the `TTBR0` system register.
    pub ttbr0: Ttbr0,
    /// Fake value for the `TTBR0_EL1` system register.
    pub ttbr0_el1: Ttbr0El1,
    /// Fake value for the `TTBR0_EL2` system register.
    pub ttbr0_el2: Ttbr0El2,
    /// Fake value for the `TTBR0_EL3` system register.
    pub ttbr0_el3: Ttbr0El3,
    /// Fake value for the `TTBR1` system register.
    pub ttbr1: Ttbr1,
    /// Fake value for the `TTBR1_EL1` system register.
    pub ttbr1_el1: Ttbr1El1,
    /// Fake value for the `TTBR1_EL2` system register.
    pub ttbr1_el2: Ttbr1El2,
    /// Fake value for the `TTTBRP_EL1` system register.
    pub tttbrp_el1: TttbrpEl1,
    /// Fake value for the `TTTBRP_EL2` system register.
    pub tttbrp_el2: TttbrpEl2,
    /// Fake value for the `TTTBRP_EL3` system register.
    pub tttbrp_el3: TttbrpEl3,
    /// Fake value for the `TTTBRU_EL1` system register.
    pub tttbru_el1: TttbruEl1,
    /// Fake value for the `TTTBRU_EL2` system register.
    pub tttbru_el2: TttbruEl2,
    /// Fake value for the `UAO` system register.
    pub uao: Uao,
    /// Fake value for the `VBAR` system register.
    pub vbar: Vbar,
    /// Fake value for the `VBAR_EL1` system register.
    pub vbar_el1: VbarEl1,
    /// Fake value for the `VBAR_EL2` system register.
    pub vbar_el2: VbarEl2,
    /// Fake value for the `VBAR_EL3` system register.
    pub vbar_el3: VbarEl3,
    /// Fake value for the `VDFSR` system register.
    pub vdfsr: Vdfsr,
    /// Fake value for the `VDISR` system register.
    pub vdisr: Vdisr,
    /// Fake value for the `VDISR_EL2` system register.
    pub vdisr_el2: VdisrEl2,
    /// Fake value for the `VDISR_EL3` system register.
    pub vdisr_el3: VdisrEl3,
    /// Fake value for the `VMECID_A_EL2` system register.
    pub vmecid_a_el2: VmecidAEl2,
    /// Fake value for the `VMECID_P_EL2` system register.
    pub vmecid_p_el2: VmecidPEl2,
    /// Fake value for the `VMPIDR` system register.
    pub vmpidr: Vmpidr,
    /// Fake value for the `VMPIDR_EL2` system register.
    pub vmpidr_el2: VmpidrEl2,
    /// Fake value for the `VNCCR_EL2` system register.
    pub vnccr_el2: VnccrEl2,
    /// Fake value for the `VNCR_EL2` system register.
    pub vncr_el2: VncrEl2,
    /// Fake value for the `VPIDR` system register.
    pub vpidr: Vpidr,
    /// Fake value for the `VPIDR_EL2` system register.
    pub vpidr_el2: VpidrEl2,
    /// Fake value for the `VSESR_EL2` system register.
    pub vsesr_el2: VsesrEl2,
    /// Fake value for the `VSESR_EL3` system register.
    pub vsesr_el3: VsesrEl3,
    /// Fake value for the `VSTCR_EL2` system register.
    pub vstcr_el2: VstcrEl2,
    /// Fake value for the `VSTTBR_EL2` system register.
    pub vsttbr_el2: VsttbrEl2,
    /// Fake value for the `VTCR` system register.
    pub vtcr: Vtcr,
    /// Fake value for the `VTCR_EL2` system register.
    pub vtcr_el2: VtcrEl2,
    /// Fake value for the `VTTBR_EL2` system register.
    pub vttbr_el2: VttbrEl2,
    /// Fake value for the `ZCR_EL1` system register.
    pub zcr_el1: ZcrEl1,
    /// Fake value for the `ZCR_EL2` system register.
    pub zcr_el2: ZcrEl2,
    /// Fake value for the `ZCR_EL3` system register.
    pub zcr_el3: ZcrEl3,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            accdata_el1: AccdataEl1::empty(),
            actlr: 0,
            actlr2: 0,
            actlrmask_el1: 0,
            actlrmask_el2: 0,
            actlr_el1: 0,
            actlr_el2: 0,
            actlr_el3: 0,
            adfsr: 0,
            afsr0_el1: 0,
            afsr0_el2: 0,
            afsr0_el3: 0,
            afsr1_el1: 0,
            afsr1_el2: 0,
            afsr1_el3: 0,
            aidr: 0,
            aidr_el1: 0,
            aifsr: 0,
            allint: Allint::empty(),
            amair0: 0,
            amair1: 0,
            amair2_el1: 0,
            amair2_el2: 0,
            amair2_el3: 0,
            amair_el1: 0,
            amair_el2: 0,
            amair_el3: 0,
            amcfgr: Amcfgr::empty(),
            amcfgr_el0: AmcfgrEl0::empty(),
            amcg1idr_el0: Amcg1idrEl0::empty(),
            amcgcr: Amcgcr::empty(),
            amcgcr_el0: AmcgcrEl0::empty(),
            amcntenclr0: Amcntenclr0::empty(),
            amcntenclr0_el0: Amcntenclr0El0::empty(),
            amcntenclr1: Amcntenclr1::empty(),
            amcntenclr1_el0: Amcntenclr1El0::empty(),
            amcntenset0: Amcntenset0::empty(),
            amcntenset0_el0: Amcntenset0El0::empty(),
            amcntenset1: Amcntenset1::empty(),
            amcntenset1_el0: Amcntenset1El0::empty(),
            amcr: Amcr::empty(),
            amcr_el0: AmcrEl0::empty(),
            amuserenr: Amuserenr::empty(),
            amuserenr_el0: AmuserenrEl0::empty(),
            apdakeyhi_el1: ApdakeyhiEl1::empty(),
            apdakeylo_el1: ApdakeyloEl1::empty(),
            apdbkeyhi_el1: ApdbkeyhiEl1::empty(),
            apdbkeylo_el1: ApdbkeyloEl1::empty(),
            apgakeyhi_el1: ApgakeyhiEl1::empty(),
            apgakeylo_el1: ApgakeyloEl1::empty(),
            apiakeyhi_el1: ApiakeyhiEl1::empty(),
            apiakeylo_el1: ApiakeyloEl1::empty(),
            apibkeyhi_el1: ApibkeyhiEl1::empty(),
            apibkeylo_el1: ApibkeyloEl1::empty(),
            ats12nsopr: Ats12nsopr::empty(),
            ats12nsopw: Ats12nsopw::empty(),
            ats12nsour: Ats12nsour::empty(),
            ats12nsouw: Ats12nsouw::empty(),
            ats1cpr: Ats1cpr::empty(),
            ats1cprp: Ats1cprp::empty(),
            ats1cpw: Ats1cpw::empty(),
            ats1cpwp: Ats1cpwp::empty(),
            ats1cur: Ats1cur::empty(),
            ats1cuw: Ats1cuw::empty(),
            ats1hr: Ats1hr::empty(),
            ats1hw: Ats1hw::empty(),
            bpiall: 0,
            bpiallis: 0,
            bpimva: Bpimva::empty(),
            brbcr_el1: BrbcrEl1::empty(),
            brbcr_el2: BrbcrEl2::empty(),
            brbfcr_el1: BrbfcrEl1::empty(),
            brbidr0_el1: Brbidr0El1::empty(),
            brbinfinj_el1: BrbinfinjEl1::empty(),
            brbsrcinj_el1: BrbsrcinjEl1::empty(),
            brbtgtinj_el1: BrbtgtinjEl1::empty(),
            brbts_el1: BrbtsEl1::empty(),
            ccsidr: Ccsidr::empty(),
            ccsidr2: Ccsidr2::empty(),
            ccsidr2_el1: Ccsidr2El1::empty(),
            ccsidr_el1: CcsidrEl1::empty(),
            cfprctx: Cfprctx::empty(),
            clidr: Clidr::empty(),
            clidr_el1: ClidrEl1::empty(),
            cntfrq: Cntfrq::empty(),
            cntfrq_el0: CntfrqEl0::empty(),
            cnthctl: Cnthctl::empty(),
            cnthctl_el2: CnthctlEl2::empty(),
            cnthps_ctl: CnthpsCtl::empty(),
            cnthps_ctl_el2: CnthpsCtlEl2::empty(),
            cnthps_cval_el2: CnthpsCvalEl2::empty(),
            cnthps_tval: CnthpsTval::empty(),
            cnthps_tval_el2: CnthpsTvalEl2::empty(),
            cnthp_ctl: CnthpCtl::empty(),
            cnthp_ctl_el2: CnthpCtlEl2::empty(),
            cnthp_cval_el2: CnthpCvalEl2::empty(),
            cnthp_tval: CnthpTval::empty(),
            cnthp_tval_el2: CnthpTvalEl2::empty(),
            cnthvs_ctl: CnthvsCtl::empty(),
            cnthvs_ctl_el2: CnthvsCtlEl2::empty(),
            cnthvs_cval_el2: CnthvsCvalEl2::empty(),
            cnthvs_tval: CnthvsTval::empty(),
            cnthvs_tval_el2: CnthvsTvalEl2::empty(),
            cnthv_ctl: CnthvCtl::empty(),
            cnthv_ctl_el2: CnthvCtlEl2::empty(),
            cnthv_cval_el2: CnthvCvalEl2::empty(),
            cnthv_tval: CnthvTval::empty(),
            cnthv_tval_el2: CnthvTvalEl2::empty(),
            cntkctl: Cntkctl::empty(),
            cntkctl_el1: CntkctlEl1::empty(),
            cntpctss_el0: CntpctssEl0::empty(),
            cntpct_el0: CntpctEl0::empty(),
            cntpoff_el2: CntpoffEl2::empty(),
            cntps_ctl_el1: CntpsCtlEl1::empty(),
            cntps_cval_el1: CntpsCvalEl1::empty(),
            cntps_tval_el1: CntpsTvalEl1::empty(),
            cntp_ctl: CntpCtl::empty(),
            cntp_ctl_el0: CntpCtlEl0::empty(),
            cntp_cval_el0: CntpCvalEl0::empty(),
            cntp_tval: CntpTval::empty(),
            cntp_tval_el0: CntpTvalEl0::empty(),
            cntvctss_el0: CntvctssEl0::empty(),
            cntvct_el0: CntvctEl0::empty(),
            cntvoff_el2: CntvoffEl2::empty(),
            cntv_ctl: CntvCtl::empty(),
            cntv_ctl_el0: CntvCtlEl0::empty(),
            cntv_cval_el0: CntvCvalEl0::empty(),
            cntv_tval: CntvTval::empty(),
            cntv_tval_el0: CntvTvalEl0::empty(),
            contextidr: Contextidr::empty(),
            contextidr_el1: ContextidrEl1::empty(),
            contextidr_el2: ContextidrEl2::empty(),
            cosprctx: Cosprctx::empty(),
            cp15dmb: 0,
            cp15dsb: 0,
            cp15isb: 0,
            cpacr: Cpacr::empty(),
            cpacrmask_el1: CpacrmaskEl1::empty(),
            cpacr_el1: CpacrEl1::empty(),
            cpprctx: Cpprctx::empty(),
            cptrmask_el2: CptrmaskEl2::empty(),
            cptr_el2: CptrEl2::empty(),
            cptr_el3: CptrEl3::empty(),
            csselr: Csselr::empty(),
            csselr_el1: CsselrEl1::empty(),
            ctr: Ctr::empty(),
            ctr_el0: CtrEl0::empty(),
            currentel: Currentel::empty(),
            dacr: Dacr::empty(),
            dacr32_el2: Dacr32El2::empty(),
            daif: Daif::empty(),
            dbgauthstatus: Dbgauthstatus::empty(),
            dbgauthstatus_el1: DbgauthstatusEl1::empty(),
            dbgclaimclr: Dbgclaimclr::empty(),
            dbgclaimclr_el1: DbgclaimclrEl1::empty(),
            dbgclaimset: Dbgclaimset::empty(),
            dbgclaimset_el1: DbgclaimsetEl1::empty(),
            dbgdccint: Dbgdccint::empty(),
            dbgdevid: Dbgdevid::empty(),
            dbgdevid1: Dbgdevid1::empty(),
            dbgdevid2: 0,
            dbgdidr: Dbgdidr::empty(),
            dbgdrar: Dbgdrar::empty(),
            dbgdsar: 0,
            dbgdscrext: Dbgdscrext::empty(),
            dbgdscrint: Dbgdscrint::empty(),
            dbgdtrrx_el0: DbgdtrrxEl0::empty(),
            dbgdtrrxext: Dbgdtrrxext::empty(),
            dbgdtrrxint: Dbgdtrrxint::empty(),
            dbgdtrtx_el0: DbgdtrtxEl0::empty(),
            dbgdtrtxext: Dbgdtrtxext::empty(),
            dbgdtrtxint: Dbgdtrtxint::empty(),
            dbgdtr_el0: DbgdtrEl0::empty(),
            dbgosdlr: Dbgosdlr::empty(),
            dbgoseccr: Dbgoseccr::empty(),
            dbgoslar: Dbgoslar::empty(),
            dbgoslsr: Dbgoslsr::empty(),
            dbgprcr: Dbgprcr::empty(),
            dbgprcr_el1: DbgprcrEl1::empty(),
            dbgvcr: Dbgvcr::empty(),
            dbgvcr32_el2: Dbgvcr32El2::empty(),
            dbgwfar: 0,
            dccimvac: Dccimvac::empty(),
            dccisw: Dccisw::empty(),
            dccmvac: Dccmvac::empty(),
            dccmvau: Dccmvau::empty(),
            dccsw: Dccsw::empty(),
            dcimvac: Dcimvac::empty(),
            dcisw: Dcisw::empty(),
            dczid_el0: DczidEl0::empty(),
            dfar: Dfar::empty(),
            dfsr: Dfsr::empty(),
            disr: Disr::empty(),
            disr_el1: DisrEl1::empty(),
            dit: Dit::empty(),
            dlr: Dlr::empty(),
            dlr_el0: DlrEl0::empty(),
            dpocr_el0: DpocrEl0::empty(),
            dpotbr0_el1: Dpotbr0El1::empty(),
            dpotbr0_el2: Dpotbr0El2::empty(),
            dpotbr0_el3: Dpotbr0El3::empty(),
            dpotbr1_el1: Dpotbr1El1::empty(),
            dpotbr1_el2: Dpotbr1El2::empty(),
            dspsr: Dspsr::empty(),
            dspsr2: Dspsr2::empty(),
            dspsr_el0: DspsrEl0::empty(),
            dtlbiall: 0,
            dtlbiasid: Dtlbiasid::empty(),
            dtlbimva: Dtlbimva::empty(),
            dvprctx: Dvprctx::empty(),
            elr_el1: ElrEl1::empty(),
            elr_el2: ElrEl2::empty(),
            elr_el3: ElrEl3::empty(),
            erridr: Erridr::empty(),
            erridr_el1: ErridrEl1::empty(),
            errselr: Errselr::empty(),
            errselr_el1: ErrselrEl1::empty(),
            erxaddr: Erxaddr::empty(),
            erxaddr2: Erxaddr2::empty(),
            erxaddr_el1: ErxaddrEl1::empty(),
            erxctlr: Erxctlr::empty(),
            erxctlr2: Erxctlr2::empty(),
            erxctlr_el1: ErxctlrEl1::empty(),
            erxfr: Erxfr::empty(),
            erxfr2: Erxfr2::empty(),
            erxfr_el1: ErxfrEl1::empty(),
            erxgsr_el1: ErxgsrEl1::empty(),
            erxmisc0: Erxmisc0::empty(),
            erxmisc0_el1: Erxmisc0El1::empty(),
            erxmisc1: Erxmisc1::empty(),
            erxmisc1_el1: Erxmisc1El1::empty(),
            erxmisc2: Erxmisc2::empty(),
            erxmisc2_el1: Erxmisc2El1::empty(),
            erxmisc3: Erxmisc3::empty(),
            erxmisc3_el1: Erxmisc3El1::empty(),
            erxmisc4: Erxmisc4::empty(),
            erxmisc5: Erxmisc5::empty(),
            erxmisc6: Erxmisc6::empty(),
            erxmisc7: Erxmisc7::empty(),
            erxpfgcdn_el1: ErxpfgcdnEl1::empty(),
            erxpfgctl_el1: ErxpfgctlEl1::empty(),
            erxpfgf_el1: ErxpfgfEl1::empty(),
            erxstatus: Erxstatus::empty(),
            erxstatus_el1: ErxstatusEl1::empty(),
            esr_el1: EsrEl1::empty(),
            esr_el2: EsrEl2::empty(),
            esr_el3: EsrEl3::empty(),
            far_el1: FarEl1::empty(),
            far_el2: FarEl2::empty(),
            far_el3: FarEl3::empty(),
            fcseidr: 0,
            fgwte3_el3: Fgwte3El3::empty(),
            fpcr: Fpcr::empty(),
            fpexc32_el2: Fpexc32El2::empty(),
            fpmr: Fpmr::empty(),
            fpsr: Fpsr::empty(),
            gcr_el1: GcrEl1::empty(),
            gcscre0_el1: Gcscre0El1::empty(),
            gcscr_el1: GcscrEl1::empty(),
            gcscr_el2: GcscrEl2::empty(),
            gcscr_el3: GcscrEl3::empty(),
            gcspr_el0: GcsprEl0::empty(),
            gcspr_el1: GcsprEl1::empty(),
            gcspr_el2: GcsprEl2::empty(),
            gcspr_el3: GcsprEl3::empty(),
            gmid_el1: GmidEl1::empty(),
            gpcbw_el3: GpcbwEl3::empty(),
            gpccr_el3: GpccrEl3::empty(),
            gptbr_el3: GptbrEl3::empty(),
            hacdbsbr_el2: HacdbsbrEl2::empty(),
            hacdbscons_el2: HacdbsconsEl2::empty(),
            hacr: 0,
            hacr_el2: 0,
            hactlr: 0,
            hactlr2: 0,
            hadfsr: 0,
            hafgrtr_el2: HafgrtrEl2::empty(),
            haifsr: 0,
            hamair0: 0,
            hamair1: 0,
            hcptr: Hcptr::empty(),
            hcr: Hcr::empty(),
            hcr2: Hcr2::empty(),
            hcrmask_el2: HcrmaskEl2::empty(),
            hcrxmask_el2: HcrxmaskEl2::empty(),
            hcrx_el2: HcrxEl2::empty(),
            hcr_el2: HcrEl2::empty(),
            hdbssbr_el2: HdbssbrEl2::empty(),
            hdbssprod_el2: HdbssprodEl2::empty(),
            hdcr: Hdcr::empty(),
            hdfar: Hdfar::empty(),
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
            hifar: Hifar::empty(),
            hmair0: Hmair0::empty(),
            hmair1: Hmair1::empty(),
            hpfar: Hpfar::empty(),
            hpfar_el2: HpfarEl2::empty(),
            hrmr: Hrmr::empty(),
            hsctlr: Hsctlr::empty(),
            hsr: Hsr::empty(),
            hstr: 0,
            hstr_el2: 0,
            htcr: Htcr::empty(),
            htpidr: Htpidr::empty(),
            htrfcr: Htrfcr::empty(),
            hvbar: Hvbar::empty(),
            icc_apr_el1: IccAprEl1::empty(),
            icc_apr_el3: IccAprEl3::empty(),
            icc_asgi1r_el1: IccAsgi1rEl1::empty(),
            icc_bpr0: IccBpr0::empty(),
            icc_bpr0_el1: IccBpr0El1::empty(),
            icc_bpr1: IccBpr1::empty(),
            icc_bpr1_el1: IccBpr1El1::empty(),
            icc_cr0_el1: IccCr0El1::empty(),
            icc_cr0_el3: IccCr0El3::empty(),
            icc_ctlr: IccCtlr::empty(),
            icc_ctlr_el1: IccCtlrEl1::empty(),
            icc_ctlr_el3: IccCtlrEl3::empty(),
            icc_dir: IccDir::empty(),
            icc_dir_el1: IccDirEl1::empty(),
            icc_domhppir_el3: IccDomhppirEl3::empty(),
            icc_eoir0: IccEoir0::empty(),
            icc_eoir0_el1: IccEoir0El1::empty(),
            icc_eoir1: IccEoir1::empty(),
            icc_eoir1_el1: IccEoir1El1::empty(),
            icc_hapr_el1: IccHaprEl1::empty(),
            icc_hppir0: IccHppir0::empty(),
            icc_hppir0_el1: IccHppir0El1::empty(),
            icc_hppir1: IccHppir1::empty(),
            icc_hppir1_el1: IccHppir1El1::empty(),
            icc_hppir_el1: IccHppirEl1::empty(),
            icc_hppir_el3: IccHppirEl3::empty(),
            icc_hsre: IccHsre::empty(),
            icc_iaffidr_el1: IccIaffidrEl1::empty(),
            icc_iar0: IccIar0::empty(),
            icc_iar0_el1: IccIar0El1::empty(),
            icc_iar1: IccIar1::empty(),
            icc_iar1_el1: IccIar1El1::empty(),
            icc_icsr_el1: IccIcsrEl1::empty(),
            icc_idr0_el1: IccIdr0El1::empty(),
            icc_igrpen0: IccIgrpen0::empty(),
            icc_igrpen0_el1: IccIgrpen0El1::empty(),
            icc_igrpen1: IccIgrpen1::empty(),
            icc_igrpen1_el1: IccIgrpen1El1::empty(),
            icc_igrpen1_el3: IccIgrpen1El3::empty(),
            icc_mctlr: IccMctlr::empty(),
            icc_mgrpen1: IccMgrpen1::empty(),
            icc_msre: IccMsre::empty(),
            icc_nmiar1_el1: IccNmiar1El1::empty(),
            icc_pcr_el1: IccPcrEl1::empty(),
            icc_pcr_el3: IccPcrEl3::empty(),
            icc_pmr: IccPmr::empty(),
            icc_pmr_el1: IccPmrEl1::empty(),
            icc_rpr: IccRpr::empty(),
            icc_rpr_el1: IccRprEl1::empty(),
            icc_sgi0r_el1: IccSgi0rEl1::empty(),
            icc_sgi1r_el1: IccSgi1rEl1::empty(),
            icc_sre: IccSre::empty(),
            icc_sre_el1: IccSreEl1::empty(),
            icc_sre_el2: IccSreEl2::empty(),
            icc_sre_el3: IccSreEl3::empty(),
            ich_apr_el2: IchAprEl2::empty(),
            ich_contextr_el2: IchContextrEl2::empty(),
            ich_eisr: IchEisr::empty(),
            ich_eisr_el2: IchEisrEl2::empty(),
            ich_elrsr: IchElrsr::empty(),
            ich_elrsr_el2: IchElrsrEl2::empty(),
            ich_hcr: IchHcr::empty(),
            ich_hcr_el2: IchHcrEl2::empty(),
            ich_hfgitr_el2: IchHfgitrEl2::empty(),
            ich_hfgrtr_el2: IchHfgrtrEl2::empty(),
            ich_hfgwtr_el2: IchHfgwtrEl2::empty(),
            ich_hppir_el2: IchHppirEl2::empty(),
            ich_misr: IchMisr::empty(),
            ich_misr_el2: IchMisrEl2::empty(),
            ich_vctlr_el2: IchVctlrEl2::empty(),
            ich_vmcr: IchVmcr::empty(),
            ich_vmcr_el2: IchVmcrEl2::empty(),
            ich_vtr: IchVtr::empty(),
            ich_vtr_el2: IchVtrEl2::empty(),
            iciallu: 0,
            icialluis: 0,
            icimvau: Icimvau::empty(),
            icv_apr_el1: IcvAprEl1::empty(),
            icv_bpr0: IcvBpr0::empty(),
            icv_bpr0_el1: IcvBpr0El1::empty(),
            icv_bpr1: IcvBpr1::empty(),
            icv_bpr1_el1: IcvBpr1El1::empty(),
            icv_cr0_el1: IcvCr0El1::empty(),
            icv_ctlr: IcvCtlr::empty(),
            icv_ctlr_el1: IcvCtlrEl1::empty(),
            icv_dir: IcvDir::empty(),
            icv_dir_el1: IcvDirEl1::empty(),
            icv_eoir0: IcvEoir0::empty(),
            icv_eoir0_el1: IcvEoir0El1::empty(),
            icv_eoir1: IcvEoir1::empty(),
            icv_eoir1_el1: IcvEoir1El1::empty(),
            icv_hapr_el1: IcvHaprEl1::empty(),
            icv_hppir0: IcvHppir0::empty(),
            icv_hppir0_el1: IcvHppir0El1::empty(),
            icv_hppir1: IcvHppir1::empty(),
            icv_hppir1_el1: IcvHppir1El1::empty(),
            icv_hppir_el1: IcvHppirEl1::empty(),
            icv_iar0: IcvIar0::empty(),
            icv_iar0_el1: IcvIar0El1::empty(),
            icv_iar1: IcvIar1::empty(),
            icv_iar1_el1: IcvIar1El1::empty(),
            icv_igrpen0: IcvIgrpen0::empty(),
            icv_igrpen0_el1: IcvIgrpen0El1::empty(),
            icv_igrpen1: IcvIgrpen1::empty(),
            icv_igrpen1_el1: IcvIgrpen1El1::empty(),
            icv_nmiar1_el1: IcvNmiar1El1::empty(),
            icv_pcr_el1: IcvPcrEl1::empty(),
            icv_pmr: IcvPmr::empty(),
            icv_pmr_el1: IcvPmrEl1::empty(),
            icv_rpr: IcvRpr::empty(),
            icv_rpr_el1: IcvRprEl1::empty(),
            id_aa64afr0_el1: 0,
            id_aa64afr1_el1: 0,
            id_aa64dfr0_el1: IdAa64dfr0El1::empty(),
            id_aa64dfr1_el1: IdAa64dfr1El1::empty(),
            id_aa64dfr2_el1: IdAa64dfr2El1::empty(),
            id_aa64fpfr0_el1: IdAa64fpfr0El1::empty(),
            id_aa64isar0_el1: IdAa64isar0El1::empty(),
            id_aa64isar1_el1: IdAa64isar1El1::empty(),
            id_aa64isar2_el1: IdAa64isar2El1::empty(),
            id_aa64isar3_el1: IdAa64isar3El1::empty(),
            id_aa64mmfr0_el1: IdAa64mmfr0El1::empty(),
            id_aa64mmfr1_el1: IdAa64mmfr1El1::empty(),
            id_aa64mmfr2_el1: IdAa64mmfr2El1::empty(),
            id_aa64mmfr3_el1: IdAa64mmfr3El1::empty(),
            id_aa64mmfr4_el1: IdAa64mmfr4El1::empty(),
            id_aa64pfr0_el1: IdAa64pfr0El1::empty(),
            id_aa64pfr1_el1: IdAa64pfr1El1::empty(),
            id_aa64pfr2_el1: IdAa64pfr2El1::empty(),
            id_aa64smfr0_el1: IdAa64smfr0El1::empty(),
            id_aa64zfr0_el1: IdAa64zfr0El1::empty(),
            id_afr0: 0,
            id_afr0_el1: 0,
            id_dfr0: IdDfr0::empty(),
            id_dfr0_el1: IdDfr0El1::empty(),
            id_dfr1: IdDfr1::empty(),
            id_dfr1_el1: IdDfr1El1::empty(),
            id_isar0: IdIsar0::empty(),
            id_isar0_el1: IdIsar0El1::empty(),
            id_isar1: IdIsar1::empty(),
            id_isar1_el1: IdIsar1El1::empty(),
            id_isar2: IdIsar2::empty(),
            id_isar2_el1: IdIsar2El1::empty(),
            id_isar3: IdIsar3::empty(),
            id_isar3_el1: IdIsar3El1::empty(),
            id_isar4: IdIsar4::empty(),
            id_isar4_el1: IdIsar4El1::empty(),
            id_isar5: IdIsar5::empty(),
            id_isar5_el1: IdIsar5El1::empty(),
            id_isar6: IdIsar6::empty(),
            id_isar6_el1: IdIsar6El1::empty(),
            id_mmfr0: IdMmfr0::empty(),
            id_mmfr0_el1: IdMmfr0El1::empty(),
            id_mmfr1: IdMmfr1::empty(),
            id_mmfr1_el1: IdMmfr1El1::empty(),
            id_mmfr2: IdMmfr2::empty(),
            id_mmfr2_el1: IdMmfr2El1::empty(),
            id_mmfr3: IdMmfr3::empty(),
            id_mmfr3_el1: IdMmfr3El1::empty(),
            id_mmfr4: IdMmfr4::empty(),
            id_mmfr4_el1: IdMmfr4El1::empty(),
            id_mmfr5: IdMmfr5::empty(),
            id_mmfr5_el1: IdMmfr5El1::empty(),
            id_pfr0: IdPfr0::empty(),
            id_pfr0_el1: IdPfr0El1::empty(),
            id_pfr1: IdPfr1::empty(),
            id_pfr1_el1: IdPfr1El1::empty(),
            id_pfr2: IdPfr2::empty(),
            id_pfr2_el1: IdPfr2El1::empty(),
            ifar: Ifar::empty(),
            ifsr: Ifsr::empty(),
            ifsr32_el2: Ifsr32El2::empty(),
            irtbrp_el1: IrtbrpEl1::empty(),
            irtbrp_el2: IrtbrpEl2::empty(),
            irtbrp_el3: IrtbrpEl3::empty(),
            irtbru_el1: IrtbruEl1::empty(),
            irtbru_el2: IrtbruEl2::empty(),
            isr: Isr::empty(),
            isr_el1: IsrEl1::empty(),
            itlbiall: 0,
            itlbiasid: Itlbiasid::empty(),
            itlbimva: Itlbimva::empty(),
            jidr: 0,
            jmcr: 0,
            joscr: 0,
            ldstt_el1: LdsttEl1::empty(),
            ldstt_el2: LdsttEl2::empty(),
            lorc_el1: LorcEl1::empty(),
            lorea_el1: LoreaEl1::empty(),
            lorid_el1: LoridEl1::empty(),
            lorn_el1: LornEl1::empty(),
            lorsa_el1: LorsaEl1::empty(),
            mair0: Mair0::empty(),
            mair1: Mair1::empty(),
            mair2_el1: Mair2El1::empty(),
            mair2_el2: Mair2El2::empty(),
            mair2_el3: Mair2El3::empty(),
            mair_el1: MairEl1::empty(),
            mair_el2: MairEl2::empty(),
            mair_el3: MairEl3::empty(),
            mdccint_el1: MdccintEl1::empty(),
            mdccsr_el0: MdccsrEl0::empty(),
            mdcr_el2: MdcrEl2::empty(),
            mdcr_el3: MdcrEl3::empty(),
            mdrar_el1: MdrarEl1::empty(),
            mdscr_el1: MdscrEl1::empty(),
            mdselr_el1: MdselrEl1::empty(),
            mdstepop_el1: MdstepopEl1::empty(),
            mecidr_el2: MecidrEl2::empty(),
            mecid_a0_el2: MecidA0El2::empty(),
            mecid_a1_el2: MecidA1El2::empty(),
            mecid_p0_el2: MecidP0El2::empty(),
            mecid_p1_el2: MecidP1El2::empty(),
            mecid_rl_a_el3: MecidRlAEl3::empty(),
            mfar_el3: MfarEl3::empty(),
            midr: Midr::empty(),
            midr_el1: MidrEl1::empty(),
            mpam0_el1: Mpam0El1::empty(),
            mpam1_el1: Mpam1El1::empty(),
            mpam2_el2: Mpam2El2::empty(),
            mpam3_el3: Mpam3El3::empty(),
            mpambw0_el1: Mpambw0El1::empty(),
            mpambw1_el1: Mpambw1El1::empty(),
            mpambw2_el2: Mpambw2El2::empty(),
            mpambw3_el3: Mpambw3El3::empty(),
            mpambwcap_el2: MpambwcapEl2::empty(),
            mpambwidr_el1: MpambwidrEl1::empty(),
            mpambwsm_el1: MpambwsmEl1::empty(),
            mpamctl_el1: MpamctlEl1::empty(),
            mpamctl_el2: MpamctlEl2::empty(),
            mpamctl_el3: MpamctlEl3::empty(),
            mpamhcr_el2: MpamhcrEl2::empty(),
            mpamidr_el1: MpamidrEl1::empty(),
            mpamsm_el1: MpamsmEl1::empty(),
            mpamvidcr_el2: MpamvidcrEl2::empty(),
            mpamvidsr_el2: MpamvidsrEl2::empty(),
            mpamvpm0_el2: Mpamvpm0El2::empty(),
            mpamvpm1_el2: Mpamvpm1El2::empty(),
            mpamvpm2_el2: Mpamvpm2El2::empty(),
            mpamvpm3_el2: Mpamvpm3El2::empty(),
            mpamvpm4_el2: Mpamvpm4El2::empty(),
            mpamvpm5_el2: Mpamvpm5El2::empty(),
            mpamvpm6_el2: Mpamvpm6El2::empty(),
            mpamvpm7_el2: Mpamvpm7El2::empty(),
            mpamvpmv_el2: MpamvpmvEl2::empty(),
            mpidr: Mpidr::empty(),
            mpidr_el1: MpidrEl1::empty(),
            mvbar: Mvbar::empty(),
            mvfr0_el1: Mvfr0El1::empty(),
            mvfr1_el1: Mvfr1El1::empty(),
            mvfr2_el1: Mvfr2El1::empty(),
            nmrr: Nmrr::empty(),
            nsacr: Nsacr::empty(),
            nvhcrmask_el2: NvhcrmaskEl2::empty(),
            nvhcrxmask_el2: NvhcrxmaskEl2::empty(),
            nvhcrx_el2: NvhcrxEl2::empty(),
            nvhcr_el2: NvhcrEl2::empty(),
            nzcv: Nzcv::empty(),
            osdlr_el1: OsdlrEl1::empty(),
            osdtrrx_el1: OsdtrrxEl1::empty(),
            osdtrtx_el1: OsdtrtxEl1::empty(),
            oseccr_el1: OseccrEl1::empty(),
            oslar_el1: OslarEl1::empty(),
            oslsr_el1: OslsrEl1::empty(),
            pan: Pan::empty(),
            par: Par::empty(),
            par_el1: ParEl1::empty(),
            pfar_el1: PfarEl1::empty(),
            pfar_el2: PfarEl2::empty(),
            pire0_el1: Pire0El1::empty(),
            pire0_el2: Pire0El2::empty(),
            pir_el1: PirEl1::empty(),
            pir_el2: PirEl2::empty(),
            pir_el3: PirEl3::empty(),
            pm: Pm::empty(),
            pmbidr_el1: PmbidrEl1::empty(),
            pmblimitr_el1: PmblimitrEl1::empty(),
            pmbmar_el1: PmbmarEl1::empty(),
            pmbptr_el1: PmbptrEl1::empty(),
            pmbsr_el1: PmbsrEl1::empty(),
            pmbsr_el2: PmbsrEl2::empty(),
            pmbsr_el3: PmbsrEl3::empty(),
            pmccfiltr: Pmccfiltr::empty(),
            pmccfiltr_el0: PmccfiltrEl0::empty(),
            pmccntr: Pmccntr::empty(),
            pmccntr_el0: PmccntrEl0::empty(),
            pmccntsvr_el1: PmccntsvrEl1::empty(),
            pmceid0: Pmceid0::empty(),
            pmceid0_el0: Pmceid0El0::empty(),
            pmceid1: Pmceid1::empty(),
            pmceid1_el0: Pmceid1El0::empty(),
            pmceid2: Pmceid2::empty(),
            pmceid3: Pmceid3::empty(),
            pmcntenclr: Pmcntenclr::empty(),
            pmcntenclr_el0: PmcntenclrEl0::empty(),
            pmcntenset: Pmcntenset::empty(),
            pmcntenset_el0: PmcntensetEl0::empty(),
            pmcr: Pmcr::empty(),
            pmcr_el0: PmcrEl0::empty(),
            pmecr_el1: PmecrEl1::empty(),
            pmiar_el1: PmiarEl1::empty(),
            pmicfiltr_el0: PmicfiltrEl0::empty(),
            pmicntr_el0: PmicntrEl0::empty(),
            pmicntsvr_el1: PmicntsvrEl1::empty(),
            pmintenclr: Pmintenclr::empty(),
            pmintenclr_el1: PmintenclrEl1::empty(),
            pmintenset: Pmintenset::empty(),
            pmintenset_el1: PmintensetEl1::empty(),
            pmmir: Pmmir::empty(),
            pmmir_el1: PmmirEl1::empty(),
            pmovsclr_el0: PmovsclrEl0::empty(),
            pmovsr: Pmovsr::empty(),
            pmovsset: Pmovsset::empty(),
            pmovsset_el0: PmovssetEl0::empty(),
            pmscr_el1: PmscrEl1::empty(),
            pmscr_el2: PmscrEl2::empty(),
            pmsdsfr_el1: PmsdsfrEl1::empty(),
            pmselr: Pmselr::empty(),
            pmselr_el0: PmselrEl0::empty(),
            pmsevfr_el1: PmsevfrEl1::empty(),
            pmsfcr_el1: PmsfcrEl1::empty(),
            pmsicr_el1: PmsicrEl1::empty(),
            pmsidr_el1: PmsidrEl1::empty(),
            pmsirr_el1: PmsirrEl1::empty(),
            pmslatfr_el1: PmslatfrEl1::empty(),
            pmsnevfr_el1: PmsnevfrEl1::empty(),
            pmsscr_el1: PmsscrEl1::empty(),
            pmswinc: Pmswinc::empty(),
            pmswinc_el0: PmswincEl0::empty(),
            pmuacr_el1: PmuacrEl1::empty(),
            pmuserenr: Pmuserenr::empty(),
            pmuserenr_el0: PmuserenrEl0::empty(),
            pmxevcntr: Pmxevcntr::empty(),
            pmxevcntr_el0: 0,
            pmxevtyper: Pmxevtyper::empty(),
            pmxevtyper_el0: PmxevtyperEl0::empty(),
            pmzr_el0: PmzrEl0::empty(),
            por_el0: PorEl0::empty(),
            por_el1: PorEl1::empty(),
            por_el2: PorEl2::empty(),
            por_el3: PorEl3::empty(),
            prrr: Prrr::empty(),
            rcwmask_el1: 0,
            rcwsmask_el1: 0,
            revidr: 0,
            revidr_el1: 0,
            rgsr_el1: RgsrEl1::empty(),
            rmr: Rmr::empty(),
            rmr_el1: RmrEl1::empty(),
            rmr_el2: RmrEl2::empty(),
            rmr_el3: RmrEl3::empty(),
            rndr: Rndr::empty(),
            rndrrs: Rndrrs::empty(),
            rvbar: Rvbar::empty(),
            rvbar_el1: RvbarEl1::empty(),
            rvbar_el2: RvbarEl2::empty(),
            rvbar_el3: RvbarEl3::empty(),
            s2pir_el2: S2pirEl2::empty(),
            s2por_el1: S2porEl1::empty(),
            scr: Scr::empty(),
            scr2_el3: Scr2El3::empty(),
            scr_el3: ScrEl3::empty(),
            sctlr: Sctlr::empty(),
            sctlr2mask_el1: Sctlr2maskEl1::empty(),
            sctlr2mask_el2: Sctlr2maskEl2::empty(),
            sctlr2_el1: Sctlr2El1::empty(),
            sctlr2_el2: Sctlr2El2::empty(),
            sctlr2_el3: Sctlr2El3::empty(),
            sctlrmask_el1: SctlrmaskEl1::empty(),
            sctlrmask_el2: SctlrmaskEl2::empty(),
            sctlr_el1: SctlrEl1::empty(),
            sctlr_el2: SctlrEl2::empty(),
            sctlr_el3: SctlrEl3::empty(),
            scxtnum_el0: ScxtnumEl0::empty(),
            scxtnum_el1: ScxtnumEl1::empty(),
            scxtnum_el2: ScxtnumEl2::empty(),
            scxtnum_el3: ScxtnumEl3::empty(),
            sdcr: Sdcr::empty(),
            sder: Sder::empty(),
            sder32_el2: Sder32El2::empty(),
            sder32_el3: Sder32El3::empty(),
            smcr_el1: SmcrEl1::empty(),
            smcr_el2: SmcrEl2::empty(),
            smcr_el3: SmcrEl3::empty(),
            smidr_el1: SmidrEl1::empty(),
            smprimap_el2: SmprimapEl2::empty(),
            smpri_el1: SmpriEl1::empty(),
            spmaccessr_el1: SpmaccessrEl1::empty(),
            spmaccessr_el2: SpmaccessrEl2::empty(),
            spmaccessr_el3: SpmaccessrEl3::empty(),
            spmcfgr_el1: SpmcfgrEl1::empty(),
            spmcntenclr_el0: SpmcntenclrEl0::empty(),
            spmcntenset_el0: SpmcntensetEl0::empty(),
            spmcr_el0: SpmcrEl0::empty(),
            spmdevaff_el1: SpmdevaffEl1::empty(),
            spmdevarch_el1: SpmdevarchEl1::empty(),
            spmiidr_el1: SpmiidrEl1::empty(),
            spmintenclr_el1: SpmintenclrEl1::empty(),
            spmintenset_el1: SpmintensetEl1::empty(),
            spmovsclr_el0: SpmovsclrEl0::empty(),
            spmovsset_el0: SpmovssetEl0::empty(),
            spmrootcr_el3: SpmrootcrEl3::empty(),
            spmscr_el1: SpmscrEl1::empty(),
            spmselr_el0: SpmselrEl0::empty(),
            spmzr_el0: SpmzrEl0::empty(),
            spsr_el1: SpsrEl1::empty(),
            spsr_el2: SpsrEl2::empty(),
            spsr_el3: SpsrEl3::empty(),
            spsr_abt: SpsrAbt::empty(),
            spsr_fiq: SpsrFiq::empty(),
            spsr_irq: SpsrIrq::empty(),
            spsr_und: SpsrUnd::empty(),
            spsel: Spsel::empty(),
            sp_el0: SpEl0::empty(),
            sp_el1: SpEl1::empty(),
            sp_el2: SpEl2::empty(),
            ssbs: Ssbs::empty(),
            stindex_el1: StindexEl1::empty(),
            stindex_el2: StindexEl2::empty(),
            stindex_el3: StindexEl3::empty(),
            svcr: Svcr::empty(),
            tcmtr: 0,
            tco: Tco::empty(),
            tcr2mask_el1: Tcr2maskEl1::empty(),
            tcr2mask_el2: Tcr2maskEl2::empty(),
            tcr2_el1: Tcr2El1::empty(),
            tcr2_el2: Tcr2El2::empty(),
            tcrmask_el1: TcrmaskEl1::empty(),
            tcrmask_el2: TcrmaskEl2::empty(),
            tcr_el1: TcrEl1::empty(),
            tcr_el2: TcrEl2::empty(),
            tcr_el3: TcrEl3::empty(),
            tfsre0_el1: Tfsre0El1::empty(),
            tfsr_el1: TfsrEl1::empty(),
            tfsr_el2: TfsrEl2::empty(),
            tfsr_el3: TfsrEl3::empty(),
            tindex_el0: TindexEl0::empty(),
            tindex_el1: TindexEl1::empty(),
            tindex_el2: TindexEl2::empty(),
            tindex_el3: TindexEl3::empty(),
            tlbiall: 0,
            tlbiallh: 0,
            tlbiallhis: 0,
            tlbiallis: 0,
            tlbiallnsnh: 0,
            tlbiallnsnhis: 0,
            tlbiasid: Tlbiasid::empty(),
            tlbiasidis: Tlbiasidis::empty(),
            tlbididr_el1: TlbididrEl1::empty(),
            tlbiipas2: Tlbiipas2::empty(),
            tlbiipas2is: Tlbiipas2is::empty(),
            tlbiipas2l: Tlbiipas2l::empty(),
            tlbiipas2lis: Tlbiipas2lis::empty(),
            tlbimva: Tlbimva::empty(),
            tlbimvaa: Tlbimvaa::empty(),
            tlbimvaais: Tlbimvaais::empty(),
            tlbimvaal: Tlbimvaal::empty(),
            tlbimvaalis: Tlbimvaalis::empty(),
            tlbimvah: Tlbimvah::empty(),
            tlbimvahis: Tlbimvahis::empty(),
            tlbimvais: Tlbimvais::empty(),
            tlbimval: Tlbimval::empty(),
            tlbimvalh: Tlbimvalh::empty(),
            tlbimvalhis: Tlbimvalhis::empty(),
            tlbimvalis: Tlbimvalis::empty(),
            tlbtr: Tlbtr::empty(),
            tpidr2_el0: Tpidr2El0::empty(),
            tpidr3_el0: 0,
            tpidr3_el1: 0,
            tpidr3_el2: 0,
            tpidr3_el3: 0,
            tpidrprw: Tpidrprw::empty(),
            tpidrro_el0: TpidrroEl0::empty(),
            tpidruro: Tpidruro::empty(),
            tpidrurw: Tpidrurw::empty(),
            tpidr_el0: TpidrEl0::empty(),
            tpidr_el1: TpidrEl1::empty(),
            tpidr_el2: TpidrEl2::empty(),
            tpidr_el3: TpidrEl3::empty(),
            tpmax0_el0: Tpmax0El0::empty(),
            tpmax0_el1: Tpmax0El1::empty(),
            tpmax0_el2: Tpmax0El2::empty(),
            tpmax1_el0: Tpmax1El0::empty(),
            tpmax1_el1: Tpmax1El1::empty(),
            tpmax1_el2: Tpmax1El2::empty(),
            tpmin0_el0: Tpmin0El0::empty(),
            tpmin0_el1: Tpmin0El1::empty(),
            tpmin0_el2: Tpmin0El2::empty(),
            tpmin1_el0: Tpmin1El0::empty(),
            tpmin1_el1: Tpmin1El1::empty(),
            tpmin1_el2: Tpmin1El2::empty(),
            trbbaser_el1: TrbbaserEl1::empty(),
            trbidr_el1: TrbidrEl1::empty(),
            trblimitr_el1: TrblimitrEl1::empty(),
            trbmar_el1: TrbmarEl1::empty(),
            trbmpam_el1: TrbmpamEl1::empty(),
            trbptr_el1: TrbptrEl1::empty(),
            trbsr_el1: TrbsrEl1::empty(),
            trbsr_el2: TrbsrEl2::empty(),
            trbsr_el3: TrbsrEl3::empty(),
            trbtrg_el1: TrbtrgEl1::empty(),
            trcauthstatus: Trcauthstatus::empty(),
            trcauxctlr: 0,
            trcbbctlr: Trcbbctlr::empty(),
            trcccctlr: Trcccctlr::empty(),
            trccidcctlr0: Trccidcctlr0::empty(),
            trccidcctlr1: Trccidcctlr1::empty(),
            trcclaimclr: Trcclaimclr::empty(),
            trcclaimset: Trcclaimset::empty(),
            trcconfigr: Trcconfigr::empty(),
            trcdevarch: Trcdevarch::empty(),
            trcdevid: 0,
            trceventctl0r: Trceventctl0r::empty(),
            trceventctl1r: Trceventctl1r::empty(),
            trcidr0: Trcidr0::empty(),
            trcidr1: Trcidr1::empty(),
            trcidr10: Trcidr10::empty(),
            trcidr11: Trcidr11::empty(),
            trcidr12: Trcidr12::empty(),
            trcidr13: Trcidr13::empty(),
            trcidr2: Trcidr2::empty(),
            trcidr3: Trcidr3::empty(),
            trcidr4: Trcidr4::empty(),
            trcidr5: Trcidr5::empty(),
            trcidr6: Trcidr6::empty(),
            trcidr7: 0,
            trcidr8: Trcidr8::empty(),
            trcidr9: Trcidr9::empty(),
            trcimspec0: Trcimspec0::empty(),
            trcitecr_el1: TrcitecrEl1::empty(),
            trcitecr_el2: TrcitecrEl2::empty(),
            trciteedcr: Trciteedcr::empty(),
            trcoslsr: Trcoslsr::empty(),
            trcprgctlr: Trcprgctlr::empty(),
            trcqctlr: Trcqctlr::empty(),
            trcrsr: Trcrsr::empty(),
            trcseqrstevr: Trcseqrstevr::empty(),
            trcseqstr: Trcseqstr::empty(),
            trcstallctlr: Trcstallctlr::empty(),
            trcstatr: Trcstatr::empty(),
            trcsyncpr: Trcsyncpr::empty(),
            trctraceidr: Trctraceidr::empty(),
            trctsctlr: Trctsctlr::empty(),
            trcvictlr: Trcvictlr::empty(),
            trcviiectlr: Trcviiectlr::empty(),
            trcvipcssctlr: Trcvipcssctlr::empty(),
            trcvissctlr: Trcvissctlr::empty(),
            trcvmidcctlr0: Trcvmidcctlr0::empty(),
            trcvmidcctlr1: Trcvmidcctlr1::empty(),
            trfcr: Trfcr::empty(),
            trfcr_el1: TrfcrEl1::empty(),
            trfcr_el2: TrfcrEl2::empty(),
            ttbcr: Ttbcr::empty(),
            ttbcr2: Ttbcr2::empty(),
            ttbr0: Ttbr0::empty(),
            ttbr0_el1: Ttbr0El1::empty(),
            ttbr0_el2: Ttbr0El2::empty(),
            ttbr0_el3: Ttbr0El3::empty(),
            ttbr1: Ttbr1::empty(),
            ttbr1_el1: Ttbr1El1::empty(),
            ttbr1_el2: Ttbr1El2::empty(),
            tttbrp_el1: TttbrpEl1::empty(),
            tttbrp_el2: TttbrpEl2::empty(),
            tttbrp_el3: TttbrpEl3::empty(),
            tttbru_el1: TttbruEl1::empty(),
            tttbru_el2: TttbruEl2::empty(),
            uao: Uao::empty(),
            vbar: Vbar::empty(),
            vbar_el1: VbarEl1::empty(),
            vbar_el2: VbarEl2::empty(),
            vbar_el3: VbarEl3::empty(),
            vdfsr: Vdfsr::empty(),
            vdisr: Vdisr::empty(),
            vdisr_el2: VdisrEl2::empty(),
            vdisr_el3: VdisrEl3::empty(),
            vmecid_a_el2: VmecidAEl2::empty(),
            vmecid_p_el2: VmecidPEl2::empty(),
            vmpidr: Vmpidr::empty(),
            vmpidr_el2: VmpidrEl2::empty(),
            vnccr_el2: VnccrEl2::empty(),
            vncr_el2: VncrEl2::empty(),
            vpidr: Vpidr::empty(),
            vpidr_el2: VpidrEl2::empty(),
            vsesr_el2: VsesrEl2::empty(),
            vsesr_el3: VsesrEl3::empty(),
            vstcr_el2: VstcrEl2::empty(),
            vsttbr_el2: VsttbrEl2::empty(),
            vtcr: Vtcr::empty(),
            vtcr_el2: VtcrEl2::empty(),
            vttbr_el2: VttbrEl2::empty(),
            zcr_el1: ZcrEl1::empty(),
            zcr_el2: ZcrEl2::empty(),
            zcr_el3: ZcrEl3::empty(),
        }
    }
}
