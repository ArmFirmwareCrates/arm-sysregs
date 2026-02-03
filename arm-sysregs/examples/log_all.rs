// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Example to log all readable system register values.

#![no_std]
#![cfg_attr(not(any(test, feature = "fakes")), no_main)]

#[cfg(target_arch = "aarch64")]
use aarch64_rt::entry;
#[cfg(not(any(test, feature = "fakes")))]
use core::panic::PanicInfo;
use log::info;

#[cfg(target_arch = "aarch64")]
entry!(entry);
#[cfg_attr(any(test, feature = "fakes"), allow(unused))]
fn entry(_: u64, _: u64, _: u64, _: u64) -> ! {
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("actlr = {:?}", arm_sysregs::read_actlr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("actlr2 = {:?}", arm_sysregs::read_actlr2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("actlr_el1 = {:?}", arm_sysregs::read_actlr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("actlr_el2 = {:?}", arm_sysregs::read_actlr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("adfsr = {:?}", arm_sysregs::read_adfsr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("afsr0_el1 = {:?}", arm_sysregs::read_afsr0_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("afsr0_el2 = {:?}", arm_sysregs::read_afsr0_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("afsr1_el1 = {:?}", arm_sysregs::read_afsr1_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("afsr1_el2 = {:?}", arm_sysregs::read_afsr1_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("aidr = {:?}", arm_sysregs::read_aidr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("aifsr = {:?}", arm_sysregs::read_aifsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amair0 = {:?}", arm_sysregs::read_amair0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amair1 = {:?}", arm_sysregs::read_amair1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("amair_el1 = {:?}", arm_sysregs::read_amair_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("amair_el2 = {:?}", arm_sysregs::read_amair_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcfgr = {:?}", arm_sysregs::read_amcfgr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcgcr = {:?}", arm_sysregs::read_amcgcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcntenclr0 = {:?}", arm_sysregs::read_amcntenclr0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcntenclr1 = {:?}", arm_sysregs::read_amcntenclr1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcntenset0 = {:?}", arm_sysregs::read_amcntenset0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcntenset1 = {:?}", arm_sysregs::read_amcntenset1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amcr = {:?}", arm_sysregs::read_amcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("amuserenr = {:?}", arm_sysregs::read_amuserenr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("apiakeyhi_el1 = {:?}", arm_sysregs::read_apiakeyhi_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("apiakeylo_el1 = {:?}", arm_sysregs::read_apiakeylo_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ccsidr = {:?}", arm_sysregs::read_ccsidr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ccsidr2 = {:?}", arm_sysregs::read_ccsidr2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("ccsidr_el1 = {:?}", arm_sysregs::read_ccsidr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("clidr = {:?}", arm_sysregs::read_clidr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("clidr_el1 = {:?}", arm_sysregs::read_clidr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntfrq = {:?}", arm_sysregs::read_cntfrq());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("cntfrq_el0 = {:?}", arm_sysregs::read_cntfrq_el0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthctl = {:?}", arm_sysregs::read_cnthctl());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("cnthctl_el2 = {:?}", arm_sysregs::read_cnthctl_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthps_ctl = {:?}", arm_sysregs::read_cnthps_ctl());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthps_cval = {:?}", arm_sysregs::read_cnthps_cval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthps_tval = {:?}", arm_sysregs::read_cnthps_tval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthp_ctl = {:?}", arm_sysregs::read_cnthp_ctl());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthp_cval = {:?}", arm_sysregs::read_cnthp_cval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthp_tval = {:?}", arm_sysregs::read_cnthp_tval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthvs_ctl = {:?}", arm_sysregs::read_cnthvs_ctl());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthvs_cval = {:?}", arm_sysregs::read_cnthvs_cval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthvs_tval = {:?}", arm_sysregs::read_cnthvs_tval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthv_ctl = {:?}", arm_sysregs::read_cnthv_ctl());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthv_cval = {:?}", arm_sysregs::read_cnthv_cval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cnthv_tval = {:?}", arm_sysregs::read_cnthv_tval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntkctl = {:?}", arm_sysregs::read_cntkctl());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntpct = {:?}", arm_sysregs::read_cntpct());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntpctss = {:?}", arm_sysregs::read_cntpctss());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("cntpct_el0 = {:?}", arm_sysregs::read_cntpct_el0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntp_ctl = {:?}", arm_sysregs::read_cntp_ctl());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntp_cval = {:?}", arm_sysregs::read_cntp_cval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntp_tval = {:?}", arm_sysregs::read_cntp_tval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntvct = {:?}", arm_sysregs::read_cntvct());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntvctss = {:?}", arm_sysregs::read_cntvctss());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntvoff = {:?}", arm_sysregs::read_cntvoff());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("cntvoff_el2 = {:?}", arm_sysregs::read_cntvoff_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntv_ctl = {:?}", arm_sysregs::read_cntv_ctl());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntv_cval = {:?}", arm_sysregs::read_cntv_cval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cntv_tval = {:?}", arm_sysregs::read_cntv_tval());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("contextidr = {:?}", arm_sysregs::read_contextidr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("contextidr_el1 = {:?}", arm_sysregs::read_contextidr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("contextidr_el2 = {:?}", arm_sysregs::read_contextidr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("cpacr = {:?}", arm_sysregs::read_cpacr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("cpacr_el1 = {:?}", arm_sysregs::read_cpacr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("cptr_el2 = {:?}", arm_sysregs::read_cptr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("cptr_el3 = {:?}", arm_sysregs::read_cptr_el3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("csselr = {:?}", arm_sysregs::read_csselr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("csselr_el1 = {:?}", arm_sysregs::read_csselr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ctr = {:?}", arm_sysregs::read_ctr());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("ctr_el0 = {:?}", arm_sysregs::read_ctr_el0());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("currentel = {:?}", arm_sysregs::read_currentel());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dacr = {:?}", arm_sysregs::read_dacr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgauthstatus = {:?}", arm_sysregs::read_dbgauthstatus());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgclaimclr = {:?}", arm_sysregs::read_dbgclaimclr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgclaimset = {:?}", arm_sysregs::read_dbgclaimset());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdccint = {:?}", arm_sysregs::read_dbgdccint());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdevid = {:?}", arm_sysregs::read_dbgdevid());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdevid1 = {:?}", arm_sysregs::read_dbgdevid1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdevid2 = {:?}", arm_sysregs::read_dbgdevid2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdidr = {:?}", arm_sysregs::read_dbgdidr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdrar = {:?}", arm_sysregs::read_dbgdrar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdsar = {:?}", arm_sysregs::read_dbgdsar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdscrext = {:?}", arm_sysregs::read_dbgdscrext());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdscrint = {:?}", arm_sysregs::read_dbgdscrint());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdtrrxext = {:?}", arm_sysregs::read_dbgdtrrxext());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdtrrxint = {:?}", arm_sysregs::read_dbgdtrrxint());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgdtrtxext = {:?}", arm_sysregs::read_dbgdtrtxext());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgosdlr = {:?}", arm_sysregs::read_dbgosdlr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgoseccr = {:?}", arm_sysregs::read_dbgoseccr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgoslsr = {:?}", arm_sysregs::read_dbgoslsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgprcr = {:?}", arm_sysregs::read_dbgprcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgvcr = {:?}", arm_sysregs::read_dbgvcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dbgwfar = {:?}", arm_sysregs::read_dbgwfar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dfar = {:?}", arm_sysregs::read_dfar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dfsr = {:?}", arm_sysregs::read_dfsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("disr = {:?}", arm_sysregs::read_disr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("disr_el1 = {:?}", arm_sysregs::read_disr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("dit = {:?}", arm_sysregs::read_dit());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dlr = {:?}", arm_sysregs::read_dlr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dspsr = {:?}", arm_sysregs::read_dspsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("dspsr2 = {:?}", arm_sysregs::read_dspsr2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("elr_el1 = {:?}", arm_sysregs::read_elr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("elr_el2 = {:?}", arm_sysregs::read_elr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "arm"), feature = "el2"))]
    info!("elr_hyp = {:?}", arm_sysregs::read_elr_hyp());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erridr = {:?}", arm_sysregs::read_erridr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("errselr = {:?}", arm_sysregs::read_errselr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxaddr = {:?}", arm_sysregs::read_erxaddr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxaddr2 = {:?}", arm_sysregs::read_erxaddr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxctlr = {:?}", arm_sysregs::read_erxctlr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxctlr2 = {:?}", arm_sysregs::read_erxctlr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxfr = {:?}", arm_sysregs::read_erxfr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxfr2 = {:?}", arm_sysregs::read_erxfr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc0 = {:?}", arm_sysregs::read_erxmisc0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc1 = {:?}", arm_sysregs::read_erxmisc1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc2 = {:?}", arm_sysregs::read_erxmisc2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc3 = {:?}", arm_sysregs::read_erxmisc3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc4 = {:?}", arm_sysregs::read_erxmisc4());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc5 = {:?}", arm_sysregs::read_erxmisc5());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc6 = {:?}", arm_sysregs::read_erxmisc6());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxmisc7 = {:?}", arm_sysregs::read_erxmisc7());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("erxstatus = {:?}", arm_sysregs::read_erxstatus());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("esr_el1 = {:?}", arm_sysregs::read_esr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("esr_el2 = {:?}", arm_sysregs::read_esr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("esr_el3 = {:?}", arm_sysregs::read_esr_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("far_el1 = {:?}", arm_sysregs::read_far_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("far_el2 = {:?}", arm_sysregs::read_far_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("fcseidr = {:?}", arm_sysregs::read_fcseidr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("gcr_el1 = {:?}", arm_sysregs::read_gcr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("gcscr_el1 = {:?}", arm_sysregs::read_gcscr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("gcscr_el2 = {:?}", arm_sysregs::read_gcscr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("gpccr_el3 = {:?}", arm_sysregs::read_gpccr_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("gptbr_el3 = {:?}", arm_sysregs::read_gptbr_el3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hacr = {:?}", arm_sysregs::read_hacr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hacr_el2 = {:?}", arm_sysregs::read_hacr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hactlr = {:?}", arm_sysregs::read_hactlr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hactlr2 = {:?}", arm_sysregs::read_hactlr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hadfsr = {:?}", arm_sysregs::read_hadfsr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hafgrtr_el2 = {:?}", arm_sysregs::read_hafgrtr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("haifsr = {:?}", arm_sysregs::read_haifsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hamair0 = {:?}", arm_sysregs::read_hamair0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hamair1 = {:?}", arm_sysregs::read_hamair1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hcptr = {:?}", arm_sysregs::read_hcptr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hcr = {:?}", arm_sysregs::read_hcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hcr2 = {:?}", arm_sysregs::read_hcr2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hcrx_el2 = {:?}", arm_sysregs::read_hcrx_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hcr_el2 = {:?}", arm_sysregs::read_hcr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hdcr = {:?}", arm_sysregs::read_hdcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hdfar = {:?}", arm_sysregs::read_hdfar());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hdfgrtr2_el2 = {:?}", arm_sysregs::read_hdfgrtr2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hdfgrtr_el2 = {:?}", arm_sysregs::read_hdfgrtr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hdfgwtr2_el2 = {:?}", arm_sysregs::read_hdfgwtr2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hdfgwtr_el2 = {:?}", arm_sysregs::read_hdfgwtr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hfgitr2_el2 = {:?}", arm_sysregs::read_hfgitr2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hfgitr_el2 = {:?}", arm_sysregs::read_hfgitr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hfgrtr2_el2 = {:?}", arm_sysregs::read_hfgrtr2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hfgrtr_el2 = {:?}", arm_sysregs::read_hfgrtr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hfgwtr2_el2 = {:?}", arm_sysregs::read_hfgwtr2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hfgwtr_el2 = {:?}", arm_sysregs::read_hfgwtr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hifar = {:?}", arm_sysregs::read_hifar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hmair0 = {:?}", arm_sysregs::read_hmair0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hmair1 = {:?}", arm_sysregs::read_hmair1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hpfar = {:?}", arm_sysregs::read_hpfar());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hpfar_el2 = {:?}", arm_sysregs::read_hpfar_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hrmr = {:?}", arm_sysregs::read_hrmr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hsctlr = {:?}", arm_sysregs::read_hsctlr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hsr = {:?}", arm_sysregs::read_hsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hstr = {:?}", arm_sysregs::read_hstr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("hstr_el2 = {:?}", arm_sysregs::read_hstr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("htcr = {:?}", arm_sysregs::read_htcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("htpidr = {:?}", arm_sysregs::read_htpidr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("htrfcr = {:?}", arm_sysregs::read_htrfcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("httbr = {:?}", arm_sysregs::read_httbr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("hvbar = {:?}", arm_sysregs::read_hvbar());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("icc_sre_el1 = {:?}", arm_sysregs::read_icc_sre_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("icc_sre_el2 = {:?}", arm_sysregs::read_icc_sre_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("icc_sre_el3 = {:?}", arm_sysregs::read_icc_sre_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("ich_hcr_el2 = {:?}", arm_sysregs::read_ich_hcr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("ich_vmcr_el2 = {:?}", arm_sysregs::read_ich_vmcr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64dfr0_el1 = {:?}",
        arm_sysregs::read_id_aa64dfr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64dfr1_el1 = {:?}",
        arm_sysregs::read_id_aa64dfr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64isar1_el1 = {:?}",
        arm_sysregs::read_id_aa64isar1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64isar2_el1 = {:?}",
        arm_sysregs::read_id_aa64isar2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr0_el1 = {:?}",
        arm_sysregs::read_id_aa64mmfr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr1_el1 = {:?}",
        arm_sysregs::read_id_aa64mmfr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr2_el1 = {:?}",
        arm_sysregs::read_id_aa64mmfr2_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64mmfr3_el1 = {:?}",
        arm_sysregs::read_id_aa64mmfr3_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64pfr0_el1 = {:?}",
        arm_sysregs::read_id_aa64pfr0_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64pfr1_el1 = {:?}",
        arm_sysregs::read_id_aa64pfr1_el1()
    );
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!(
        "id_aa64smfr0_el1 = {:?}",
        arm_sysregs::read_id_aa64smfr0_el1()
    );
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_afr0 = {:?}", arm_sysregs::read_id_afr0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_dfr0 = {:?}", arm_sysregs::read_id_dfr0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_dfr1 = {:?}", arm_sysregs::read_id_dfr1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_isar0 = {:?}", arm_sysregs::read_id_isar0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_isar1 = {:?}", arm_sysregs::read_id_isar1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_isar2 = {:?}", arm_sysregs::read_id_isar2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_isar3 = {:?}", arm_sysregs::read_id_isar3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_isar4 = {:?}", arm_sysregs::read_id_isar4());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_isar5 = {:?}", arm_sysregs::read_id_isar5());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_isar6 = {:?}", arm_sysregs::read_id_isar6());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_mmfr0 = {:?}", arm_sysregs::read_id_mmfr0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_mmfr1 = {:?}", arm_sysregs::read_id_mmfr1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_mmfr2 = {:?}", arm_sysregs::read_id_mmfr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_mmfr3 = {:?}", arm_sysregs::read_id_mmfr3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_mmfr4 = {:?}", arm_sysregs::read_id_mmfr4());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_mmfr5 = {:?}", arm_sysregs::read_id_mmfr5());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_pfr0 = {:?}", arm_sysregs::read_id_pfr0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_pfr1 = {:?}", arm_sysregs::read_id_pfr1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("id_pfr2 = {:?}", arm_sysregs::read_id_pfr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ifar = {:?}", arm_sysregs::read_ifar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ifsr = {:?}", arm_sysregs::read_ifsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("isr = {:?}", arm_sysregs::read_isr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("isr_el1 = {:?}", arm_sysregs::read_isr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("jidr = {:?}", arm_sysregs::read_jidr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("jmcr = {:?}", arm_sysregs::read_jmcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("joscr = {:?}", arm_sysregs::read_joscr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("mair0 = {:?}", arm_sysregs::read_mair0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("mair1 = {:?}", arm_sysregs::read_mair1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("mair_el1 = {:?}", arm_sysregs::read_mair_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mair_el2 = {:?}", arm_sysregs::read_mair_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("mair_el3 = {:?}", arm_sysregs::read_mair_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("mdccint_el1 = {:?}", arm_sysregs::read_mdccint_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mdcr_el2 = {:?}", arm_sysregs::read_mdcr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("mdcr_el3 = {:?}", arm_sysregs::read_mdcr_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("mdscr_el1 = {:?}", arm_sysregs::read_mdscr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("midr = {:?}", arm_sysregs::read_midr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("midr_el1 = {:?}", arm_sysregs::read_midr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpam2_el2 = {:?}", arm_sysregs::read_mpam2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("mpam3_el3 = {:?}", arm_sysregs::read_mpam3_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamhcr_el2 = {:?}", arm_sysregs::read_mpamhcr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("mpamidr_el1 = {:?}", arm_sysregs::read_mpamidr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm0_el2 = {:?}", arm_sysregs::read_mpamvpm0_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm1_el2 = {:?}", arm_sysregs::read_mpamvpm1_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm2_el2 = {:?}", arm_sysregs::read_mpamvpm2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm3_el2 = {:?}", arm_sysregs::read_mpamvpm3_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm4_el2 = {:?}", arm_sysregs::read_mpamvpm4_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm5_el2 = {:?}", arm_sysregs::read_mpamvpm5_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm6_el2 = {:?}", arm_sysregs::read_mpamvpm6_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpm7_el2 = {:?}", arm_sysregs::read_mpamvpm7_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("mpamvpmv_el2 = {:?}", arm_sysregs::read_mpamvpmv_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("mpidr = {:?}", arm_sysregs::read_mpidr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("mpidr_el1 = {:?}", arm_sysregs::read_mpidr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("mvbar = {:?}", arm_sysregs::read_mvbar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("nmrr = {:?}", arm_sysregs::read_nmrr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("nsacr = {:?}", arm_sysregs::read_nsacr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("par = {:?}", arm_sysregs::read_par());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("par_el1 = {:?}", arm_sysregs::read_par_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmccfiltr = {:?}", arm_sysregs::read_pmccfiltr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmccntr = {:?}", arm_sysregs::read_pmccntr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmceid0 = {:?}", arm_sysregs::read_pmceid0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmceid1 = {:?}", arm_sysregs::read_pmceid1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmceid2 = {:?}", arm_sysregs::read_pmceid2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmceid3 = {:?}", arm_sysregs::read_pmceid3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmcntenclr = {:?}", arm_sysregs::read_pmcntenclr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmcntenset = {:?}", arm_sysregs::read_pmcntenset());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmcr = {:?}", arm_sysregs::read_pmcr());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("pmcr_el0 = {:?}", arm_sysregs::read_pmcr_el0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmintenclr = {:?}", arm_sysregs::read_pmintenclr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmintenset = {:?}", arm_sysregs::read_pmintenset());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmmir = {:?}", arm_sysregs::read_pmmir());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmovsr = {:?}", arm_sysregs::read_pmovsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmovsset = {:?}", arm_sysregs::read_pmovsset());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmselr = {:?}", arm_sysregs::read_pmselr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmuserenr = {:?}", arm_sysregs::read_pmuserenr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("pmxevtyper = {:?}", arm_sysregs::read_pmxevtyper());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("prrr = {:?}", arm_sysregs::read_prrr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("revidr = {:?}", arm_sysregs::read_revidr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("rgsr_el1 = {:?}", arm_sysregs::read_rgsr_el1());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("rmr = {:?}", arm_sysregs::read_rmr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("rvbar = {:?}", arm_sysregs::read_rvbar());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("scr = {:?}", arm_sysregs::read_scr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("scr_el3 = {:?}", arm_sysregs::read_scr_el3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("sctlr = {:?}", arm_sysregs::read_sctlr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("sctlr2_el3 = {:?}", arm_sysregs::read_sctlr2_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("sctlr_el1 = {:?}", arm_sysregs::read_sctlr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("sctlr_el2 = {:?}", arm_sysregs::read_sctlr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("sctlr_el3 = {:?}", arm_sysregs::read_sctlr_el3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("sdcr = {:?}", arm_sysregs::read_sdcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("sder = {:?}", arm_sysregs::read_sder());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("smcr_el3 = {:?}", arm_sysregs::read_smcr_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("spsr_el1 = {:?}", arm_sysregs::read_spsr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("spsr_el2 = {:?}", arm_sysregs::read_spsr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("spsr_el3 = {:?}", arm_sysregs::read_spsr_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("sp_el1 = {:?}", arm_sysregs::read_sp_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("sp_el2 = {:?}", arm_sysregs::read_sp_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("tcmtr = {:?}", arm_sysregs::read_tcmtr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("tcr2_el1 = {:?}", arm_sysregs::read_tcr2_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("tcr2_el2 = {:?}", arm_sysregs::read_tcr2_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("tcr_el1 = {:?}", arm_sysregs::read_tcr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("tcr_el2 = {:?}", arm_sysregs::read_tcr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("tcr_el3 = {:?}", arm_sysregs::read_tcr_el3());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("tfsre0_el1 = {:?}", arm_sysregs::read_tfsre0_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("tfsr_el1 = {:?}", arm_sysregs::read_tfsr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("tfsr_el2 = {:?}", arm_sysregs::read_tfsr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("tlbtr = {:?}", arm_sysregs::read_tlbtr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("tpidrprw = {:?}", arm_sysregs::read_tpidrprw());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("tpidrro_el0 = {:?}", arm_sysregs::read_tpidrro_el0());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("tpidruro = {:?}", arm_sysregs::read_tpidruro());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("tpidrurw = {:?}", arm_sysregs::read_tpidrurw());
    #[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
    info!("tpidr_el0 = {:?}", arm_sysregs::read_tpidr_el0());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("tpidr_el1 = {:?}", arm_sysregs::read_tpidr_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("tpidr_el2 = {:?}", arm_sysregs::read_tpidr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("trfcr = {:?}", arm_sysregs::read_trfcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ttbcr = {:?}", arm_sysregs::read_ttbcr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ttbcr2 = {:?}", arm_sysregs::read_ttbcr2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ttbr0 = {:?}", arm_sysregs::read_ttbr0());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("ttbr0_el1 = {:?}", arm_sysregs::read_ttbr0_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("ttbr0_el2 = {:?}", arm_sysregs::read_ttbr0_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("ttbr0_el3 = {:?}", arm_sysregs::read_ttbr0_el3());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("ttbr1 = {:?}", arm_sysregs::read_ttbr1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("ttbr1_el1 = {:?}", arm_sysregs::read_ttbr1_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("ttbr1_el2 = {:?}", arm_sysregs::read_ttbr1_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vbar = {:?}", arm_sysregs::read_vbar());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
    info!("vbar_el1 = {:?}", arm_sysregs::read_vbar_el1());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("vbar_el2 = {:?}", arm_sysregs::read_vbar_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vdfsr = {:?}", arm_sysregs::read_vdfsr());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vdisr = {:?}", arm_sysregs::read_vdisr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("vdisr_el2 = {:?}", arm_sysregs::read_vdisr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vmpidr = {:?}", arm_sysregs::read_vmpidr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("vmpidr_el2 = {:?}", arm_sysregs::read_vmpidr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vpidr = {:?}", arm_sysregs::read_vpidr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("vpidr_el2 = {:?}", arm_sysregs::read_vpidr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("vsesr_el2 = {:?}", arm_sysregs::read_vsesr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vtcr = {:?}", arm_sysregs::read_vtcr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("vtcr_el2 = {:?}", arm_sysregs::read_vtcr_el2());
    #[cfg(any(test, feature = "fakes", target_arch = "arm"))]
    info!("vttbr = {:?}", arm_sysregs::read_vttbr());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
    info!("vttbr_el2 = {:?}", arm_sysregs::read_vttbr_el2());
    #[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
    info!("zcr_el3 = {:?}", arm_sysregs::read_zcr_el3());
    loop {}
}

#[cfg(any(test, feature = "fakes"))]
fn main() {}

#[cfg(not(any(test, feature = "fakes")))]
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
