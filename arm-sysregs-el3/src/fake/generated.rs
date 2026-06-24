// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

// This file is generated, do not edit manually.

use crate::registers::{
    CptrEl3, EsrEl3, Fgwte3El3, GpccrEl3, GptbrEl3, IccCtlrEl3, IccIgrpen1El3, IccSreEl3, MairEl3,
    MdcrEl3, Mpam3El3, PirEl3, PorEl3, ScrEl3, Sctlr2El3, SctlrEl3, SmcrEl3, SpsrEl3, TcrEl3,
    TpidrEl3, Ttbr0El3, ZcrEl3,
};

/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct SystemRegisters {
    /// Fake value for the `CPTR_EL3` system register.
    pub cptr_el3: CptrEl3,
    /// Fake value for the `ESR_EL3` system register.
    pub esr_el3: EsrEl3,
    /// Fake value for the `FGWTE3_EL3` system register.
    pub fgwte3_el3: Fgwte3El3,
    /// Fake value for the `GPCCR_EL3` system register.
    pub gpccr_el3: GpccrEl3,
    /// Fake value for the `GPTBR_EL3` system register.
    pub gptbr_el3: GptbrEl3,
    /// Fake value for the `ICC_CTLR_EL3` system register.
    pub icc_ctlr_el3: IccCtlrEl3,
    /// Fake value for the `ICC_IGRPEN1_EL3` system register.
    pub icc_igrpen1_el3: IccIgrpen1El3,
    /// Fake value for the `ICC_SRE_EL3` system register.
    pub icc_sre_el3: IccSreEl3,
    /// Fake value for the `MAIR_EL3` system register.
    pub mair_el3: MairEl3,
    /// Fake value for the `MDCR_EL3` system register.
    pub mdcr_el3: MdcrEl3,
    /// Fake value for the `MPAM3_EL3` system register.
    pub mpam3_el3: Mpam3El3,
    /// Fake value for the `PIR_EL3` system register.
    pub pir_el3: PirEl3,
    /// Fake value for the `POR_EL3` system register.
    pub por_el3: PorEl3,
    /// Fake value for the `SCR_EL3` system register.
    pub scr_el3: ScrEl3,
    /// Fake value for the `SCTLR2_EL3` system register.
    pub sctlr2_el3: Sctlr2El3,
    /// Fake value for the `SCTLR_EL3` system register.
    pub sctlr_el3: SctlrEl3,
    /// Fake value for the `SMCR_EL3` system register.
    pub smcr_el3: SmcrEl3,
    /// Fake value for the `SPSR_EL3` system register.
    pub spsr_el3: SpsrEl3,
    /// Fake value for the `TCR_EL3` system register.
    pub tcr_el3: TcrEl3,
    /// Fake value for the `TPIDR_EL3` system register.
    pub tpidr_el3: TpidrEl3,
    /// Fake value for the `TTBR0_EL3` system register.
    pub ttbr0_el3: Ttbr0El3,
    /// Fake value for the `ZCR_EL3` system register.
    pub zcr_el3: ZcrEl3,
}

impl SystemRegisters {
    pub(crate) const fn new() -> Self {
        Self {
            cptr_el3: CptrEl3::empty(),
            esr_el3: EsrEl3::empty(),
            fgwte3_el3: Fgwte3El3::empty(),
            gpccr_el3: GpccrEl3::empty(),
            gptbr_el3: GptbrEl3::empty(),
            icc_ctlr_el3: IccCtlrEl3::empty(),
            icc_igrpen1_el3: IccIgrpen1El3::empty(),
            icc_sre_el3: IccSreEl3::empty(),
            mair_el3: MairEl3::empty(),
            mdcr_el3: MdcrEl3::empty(),
            mpam3_el3: Mpam3El3::empty(),
            pir_el3: PirEl3::empty(),
            por_el3: PorEl3::empty(),
            scr_el3: ScrEl3::empty(),
            sctlr2_el3: Sctlr2El3::empty(),
            sctlr_el3: SctlrEl3::empty(),
            smcr_el3: SmcrEl3::empty(),
            spsr_el3: SpsrEl3::empty(),
            tcr_el3: TcrEl3::empty(),
            tpidr_el3: TpidrEl3::empty(),
            ttbr0_el3: Ttbr0El3::empty(),
            zcr_el3: ZcrEl3::empty(),
        }
    }
}
