// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

// This file is generated, do not edit manually.

// Unused imports are allowed here because write_sysreg might be unused when targeting aarch64
// without fakes.
#[allow(unused_imports)]
use crate::{read_sysreg, read_write_sysreg, write_sysreg};

#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(actlr: (p15, 0, c0, c1, 1), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(actlr2: (p15, 0, c0, c1, 3), u32, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(actlr_el1, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(actlr_el2, u64, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(adfsr: (p15, 0, c1, c5, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(afsr0_el1, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(afsr0_el2, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(afsr1_el1, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(afsr1_el2, u64, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(aidr: (p15, 1, c0, c0, 7), u32, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(aifsr: (p15, 0, c1, c5, 1), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amair0: (p15, 0, c3, c10, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amair1: (p15, 0, c3, c10, 1), u32, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(amair_el1, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amair_el2, u64, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amcfgr: (p15, 0, c2, c13, 1), u32: crate::registers::Amcfgr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(amcfgr_el0: s3_3_c13_c2_1, u64: crate::registers::AmcfgrEl0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(amcg1idr_el0: s3_3_c13_c2_6, u64: crate::registers::Amcg1idrEl0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amcgcr: (p15, 0, c2, c13, 2), u32: crate::registers::Amcgcr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(amcgcr_el0: s3_3_c13_c2_2, u64: crate::registers::AmcgcrEl0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenclr0: (p15, 0, c2, c13, 4), u32: crate::registers::Amcntenclr0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amcntenclr0_el0: s3_3_c13_c2_4, u64: crate::registers::Amcntenclr0El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenclr1: (p15, 0, c3, c13, 0), u32: crate::registers::Amcntenclr1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amcntenclr1_el0: s3_3_c13_c3_0, u64: crate::registers::Amcntenclr1El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenset0: (p15, 0, c2, c13, 5), u32: crate::registers::Amcntenset0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amcntenset0_el0: s3_3_c13_c2_5, u64: crate::registers::Amcntenset0El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcntenset1: (p15, 0, c3, c13, 1), u32: crate::registers::Amcntenset1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amcntenset1_el0: s3_3_c13_c3_1, u64: crate::registers::Amcntenset1El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amcr: (p15, 0, c2, c13, 0), u32: crate::registers::Amcr, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amcr_el0: s3_3_c13_c2_0, u64: crate::registers::AmcrEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevcntr00: (p15, 0, c0), u64: crate::registers::Amevcntr00, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr00_el0: s3_3_c13_c4_0, u64: crate::registers::Amevcntr00El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevcntr01: (p15, 1, c0), u64: crate::registers::Amevcntr01, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr01_el0: s3_3_c13_c4_1, u64: crate::registers::Amevcntr01El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevcntr02: (p15, 2, c0), u64: crate::registers::Amevcntr02, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr02_el0: s3_3_c13_c4_2, u64: crate::registers::Amevcntr02El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevcntr03: (p15, 3, c0), u64: crate::registers::Amevcntr03, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr03_el0: s3_3_c13_c4_3, u64: crate::registers::Amevcntr03El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr10_el0: s3_3_c13_c12_0, u64: crate::registers::Amevcntr10El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr110_el0: s3_3_c13_c13_2, u64: crate::registers::Amevcntr110El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr111_el0: s3_3_c13_c13_3, u64: crate::registers::Amevcntr111El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr112_el0: s3_3_c13_c13_4, u64: crate::registers::Amevcntr112El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr113_el0: s3_3_c13_c13_5, u64: crate::registers::Amevcntr113El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr114_el0: s3_3_c13_c13_6, u64: crate::registers::Amevcntr114El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr115_el0: s3_3_c13_c13_7, u64: crate::registers::Amevcntr115El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr11_el0: s3_3_c13_c12_1, u64: crate::registers::Amevcntr11El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr12_el0: s3_3_c13_c12_2, u64: crate::registers::Amevcntr12El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr13_el0: s3_3_c13_c12_3, u64: crate::registers::Amevcntr13El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr14_el0: s3_3_c13_c12_4, u64: crate::registers::Amevcntr14El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr15_el0: s3_3_c13_c12_5, u64: crate::registers::Amevcntr15El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr16_el0: s3_3_c13_c12_6, u64: crate::registers::Amevcntr16El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr17_el0: s3_3_c13_c12_7, u64: crate::registers::Amevcntr17El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr18_el0: s3_3_c13_c13_0, u64: crate::registers::Amevcntr18El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amevcntr19_el0: s3_3_c13_c13_1, u64: crate::registers::Amevcntr19El0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff00_el2: s3_4_c13_c8_0, u64: crate::registers::Amevcntvoff00El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff010_el2: s3_4_c13_c9_2, u64: crate::registers::Amevcntvoff010El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff011_el2: s3_4_c13_c9_3, u64: crate::registers::Amevcntvoff011El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff012_el2: s3_4_c13_c9_4, u64: crate::registers::Amevcntvoff012El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff013_el2: s3_4_c13_c9_5, u64: crate::registers::Amevcntvoff013El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff014_el2: s3_4_c13_c9_6, u64: crate::registers::Amevcntvoff014El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff015_el2: s3_4_c13_c9_7, u64: crate::registers::Amevcntvoff015El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff01_el2: s3_4_c13_c8_1, u64: crate::registers::Amevcntvoff01El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff02_el2: s3_4_c13_c8_2, u64: crate::registers::Amevcntvoff02El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff03_el2: s3_4_c13_c8_3, u64: crate::registers::Amevcntvoff03El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff04_el2: s3_4_c13_c8_4, u64: crate::registers::Amevcntvoff04El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff05_el2: s3_4_c13_c8_5, u64: crate::registers::Amevcntvoff05El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff06_el2: s3_4_c13_c8_6, u64: crate::registers::Amevcntvoff06El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff07_el2: s3_4_c13_c8_7, u64: crate::registers::Amevcntvoff07El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff08_el2: s3_4_c13_c9_0, u64: crate::registers::Amevcntvoff08El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff09_el2: s3_4_c13_c9_1, u64: crate::registers::Amevcntvoff09El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff10_el2: s3_4_c13_c10_0, u64: crate::registers::Amevcntvoff10El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff110_el2: s3_4_c13_c11_2, u64: crate::registers::Amevcntvoff110El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff111_el2: s3_4_c13_c11_3, u64: crate::registers::Amevcntvoff111El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff112_el2: s3_4_c13_c11_4, u64: crate::registers::Amevcntvoff112El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff113_el2: s3_4_c13_c11_5, u64: crate::registers::Amevcntvoff113El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff114_el2: s3_4_c13_c11_6, u64: crate::registers::Amevcntvoff114El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff115_el2: s3_4_c13_c11_7, u64: crate::registers::Amevcntvoff115El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff11_el2: s3_4_c13_c10_1, u64: crate::registers::Amevcntvoff11El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff12_el2: s3_4_c13_c10_2, u64: crate::registers::Amevcntvoff12El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff13_el2: s3_4_c13_c10_3, u64: crate::registers::Amevcntvoff13El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff14_el2: s3_4_c13_c10_4, u64: crate::registers::Amevcntvoff14El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff15_el2: s3_4_c13_c10_5, u64: crate::registers::Amevcntvoff15El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff16_el2: s3_4_c13_c10_6, u64: crate::registers::Amevcntvoff16El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff17_el2: s3_4_c13_c10_7, u64: crate::registers::Amevcntvoff17El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff18_el2: s3_4_c13_c11_0, u64: crate::registers::Amevcntvoff18El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(amevcntvoff19_el2: s3_4_c13_c11_1, u64: crate::registers::Amevcntvoff19El2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amevtyper00: (p15, 0, c6, c13, 0), u32: crate::registers::Amevtyper00, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(amevtyper00_el0: s3_3_c13_c6_0, u64: crate::registers::Amevtyper00El0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amevtyper01: (p15, 0, c6, c13, 1), u32: crate::registers::Amevtyper01, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(amevtyper01_el0: s3_3_c13_c6_1, u64: crate::registers::Amevtyper01El0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amevtyper02: (p15, 0, c6, c13, 2), u32: crate::registers::Amevtyper02, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(amevtyper02_el0: s3_3_c13_c6_2, u64: crate::registers::Amevtyper02El0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(amevtyper03: (p15, 0, c6, c13, 3), u32: crate::registers::Amevtyper03, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(amevtyper03_el0: s3_3_c13_c6_3, u64: crate::registers::Amevtyper03El0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper10: (p15, 0, c14, c13, 0), u32: crate::registers::Amevtyper10, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper11: (p15, 0, c14, c13, 1), u32: crate::registers::Amevtyper11, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper110: (p15, 0, c15, c13, 2), u32: crate::registers::Amevtyper110, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper111: (p15, 0, c15, c13, 3), u32: crate::registers::Amevtyper111, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper112: (p15, 0, c15, c13, 4), u32: crate::registers::Amevtyper112, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper113: (p15, 0, c15, c13, 5), u32: crate::registers::Amevtyper113, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper114: (p15, 0, c15, c13, 6), u32: crate::registers::Amevtyper114, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper115: (p15, 0, c15, c13, 7), u32: crate::registers::Amevtyper115, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper12: (p15, 0, c14, c13, 2), u32: crate::registers::Amevtyper12, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper13: (p15, 0, c14, c13, 3), u32: crate::registers::Amevtyper13, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper14: (p15, 0, c14, c13, 4), u32: crate::registers::Amevtyper14, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper15: (p15, 0, c14, c13, 5), u32: crate::registers::Amevtyper15, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper16: (p15, 0, c14, c13, 6), u32: crate::registers::Amevtyper16, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper17: (p15, 0, c14, c13, 7), u32: crate::registers::Amevtyper17, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper18: (p15, 0, c15, c13, 0), u32: crate::registers::Amevtyper18, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amevtyper19: (p15, 0, c15, c13, 1), u32: crate::registers::Amevtyper19, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(amuserenr: (p15, 0, c2, c13, 3), u32: crate::registers::Amuserenr, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(amuserenr_el0: s3_3_c13_c2_3, u64: crate::registers::AmuserenrEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(apiakeyhi_el1: s3_0_c2_c1_1, u64: crate::registers::ApiakeyhiEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(apiakeylo_el1: s3_0_c2_c1_0, u64: crate::registers::ApiakeyloEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(brbcr_el2: s2_1_c9_c0_0, u64: crate::registers::BrbcrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(ccsidr: (p15, 1, c0, c0, 0), u32: crate::registers::Ccsidr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(ccsidr2: (p15, 1, c0, c0, 2), u32: crate::registers::Ccsidr2, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(ccsidr_el1, u64: crate::registers::CcsidrEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(clidr: (p15, 1, c0, c0, 1), u32: crate::registers::Clidr, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(clidr_el1, u64: crate::registers::ClidrEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntfrq: (p15, 0, c0, c14, 0), u32: crate::registers::Cntfrq, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntfrq_el0, u64: crate::registers::CntfrqEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthctl: (p15, 4, c1, c14, 0), u32: crate::registers::Cnthctl, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthctl_el2, u64: crate::registers::CnthctlEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthps_ctl: (p15, 0, c2, c14, 1), u32: crate::registers::CnthpsCtl, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthps_ctl_el2: s3_4_c14_c5_1, u64: crate::registers::CnthpsCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthps_cval: (p15, 2, c14), u64: crate::registers::CnthpsCval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthps_cval_el2: s3_4_c14_c5_2, u64: crate::registers::CnthpsCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthps_tval: (p15, 0, c2, c14, 0), u32: crate::registers::CnthpsTval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthps_tval_el2: s3_4_c14_c5_0, u64: crate::registers::CnthpsTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthp_ctl: (p15, 0, c2, c14, 1), u32: crate::registers::CnthpCtl, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthp_ctl_el2: s3_4_c14_c2_1, u64: crate::registers::CnthpCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthp_cval: (p15, 2, c14), u64: crate::registers::CnthpCval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthp_cval_el2: s3_4_c14_c2_2, u64: crate::registers::CnthpCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthp_tval: (p15, 0, c2, c14, 0), u32: crate::registers::CnthpTval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthp_tval_el2: s3_4_c14_c2_0, u64: crate::registers::CnthpTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthvs_ctl: (p15, 0, c3, c14, 1), u32: crate::registers::CnthvsCtl, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthvs_ctl_el2: s3_4_c14_c4_1, u64: crate::registers::CnthvsCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthvs_cval: (p15, 3, c14), u64: crate::registers::CnthvsCval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthvs_cval_el2: s3_4_c14_c4_2, u64: crate::registers::CnthvsCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthvs_tval: (p15, 0, c3, c14, 0), u32: crate::registers::CnthvsTval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthvs_tval_el2: s3_4_c14_c4_0, u64: crate::registers::CnthvsTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthv_ctl: (p15, 0, c3, c14, 1), u32: crate::registers::CnthvCtl, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthv_ctl_el2: s3_4_c14_c3_1, u64: crate::registers::CnthvCtlEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthv_cval: (p15, 3, c14), u64: crate::registers::CnthvCval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthv_cval_el2: s3_4_c14_c3_2, u64: crate::registers::CnthvCvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cnthv_tval: (p15, 0, c3, c14, 0), u32: crate::registers::CnthvTval, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cnthv_tval_el2: s3_4_c14_c3_0, u64: crate::registers::CnthvTvalEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntkctl: (p15, 0, c1, c14, 0), u32: crate::registers::Cntkctl, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(cntkctl_el1, u64: crate::registers::CntkctlEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntpct: (p15, 0, c14), u64: crate::registers::Cntpct, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntpctss: (p15, 8, c14), u64: crate::registers::Cntpctss, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(cntpctss_el0: s3_3_c14_c0_5, u64: crate::registers::CntpctssEl0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(cntpct_el0, u64: crate::registers::CntpctEl0, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cntpoff_el2: s3_4_c14_c0_6, u64: crate::registers::CntpoffEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(cntps_ctl_el1, u64: crate::registers::CntpsCtlEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(cntps_cval_el1, u64: crate::registers::CntpsCvalEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(cntps_tval_el1, u64: crate::registers::CntpsTvalEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntp_ctl: (p15, 0, c2, c14, 1), u32: crate::registers::CntpCtl, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntp_ctl_el0, u64: crate::registers::CntpCtlEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntp_cval: (p15, 2, c14), u64: crate::registers::CntpCval, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntp_cval_el0, u64: crate::registers::CntpCvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntp_tval: (p15, 0, c2, c14, 0), u32: crate::registers::CntpTval, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntp_tval_el0, u64: crate::registers::CntpTvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntvct: (p15, 1, c14), u64: crate::registers::Cntvct, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(cntvctss: (p15, 9, c14), u64: crate::registers::Cntvctss, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(cntvctss_el0: s3_3_c14_c0_6, u64: crate::registers::CntvctssEl0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(cntvct_el0, u64: crate::registers::CntvctEl0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntvoff: (p15, 4, c14), u64: crate::registers::Cntvoff, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cntvoff_el2, u64: crate::registers::CntvoffEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntv_ctl: (p15, 0, c3, c14, 1), u32: crate::registers::CntvCtl, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntv_ctl_el0, u64: crate::registers::CntvCtlEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntv_cval: (p15, 3, c14), u64: crate::registers::CntvCval, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntv_cval_el0, u64: crate::registers::CntvCvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cntv_tval: (p15, 0, c3, c14, 0), u32: crate::registers::CntvTval, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(cntv_tval_el0, u64: crate::registers::CntvTvalEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(contextidr: (p15, 0, c0, c13, 1), u32: crate::registers::Contextidr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(contextidr_el1, u64: crate::registers::ContextidrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(contextidr_el2: s3_4_c13_c0_1, u64: crate::registers::ContextidrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(cpacr: (p15, 0, c0, c1, 2), u32: crate::registers::Cpacr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(cpacr_el1, u64: crate::registers::CpacrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(cptr_el2, u64: crate::registers::CptrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(cptr_el3, u64: crate::registers::CptrEl3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(csselr: (p15, 2, c0, c0, 0), u32: crate::registers::Csselr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(csselr_el1, u64: crate::registers::CsselrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(ctr: (p15, 0, c0, c0, 1), u32: crate::registers::Ctr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(ctr_el0, u64: crate::registers::CtrEl0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_sysreg!(currentel, u64: crate::registers::Currentel, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dacr: (p15, 0, c0, c3, 0), u32: crate::registers::Dacr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(daif, u64: crate::registers::Daif, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgauthstatus: (p14, 0, c14, c7, 6), u32: crate::registers::Dbgauthstatus, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgclaimclr: (p14, 0, c9, c7, 6), u32: crate::registers::Dbgclaimclr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgclaimset: (p14, 0, c8, c7, 6), u32: crate::registers::Dbgclaimset, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdccint: (p14, 0, c2, c0, 0), u32: crate::registers::Dbgdccint, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdevid: (p14, 0, c2, c7, 7), u32: crate::registers::Dbgdevid, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdevid1: (p14, 0, c1, c7, 7), u32: crate::registers::Dbgdevid1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdevid2: (p14, 0, c0, c7, 7), u32, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdidr: (p14, 0, c0, c0, 0), u32: crate::registers::Dbgdidr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdrar: (p14, 0, c1), u64: crate::registers::Dbgdrar, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdsar: (p14, 0, c2), u64, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdscrext: (p14, 0, c2, c0, 2), u32: crate::registers::Dbgdscrext, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdscrint: (p14, 0, c1, c0, 0), u32: crate::registers::Dbgdscrint, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdtrrxext: (p14, 0, c0, c0, 2), u32: crate::registers::Dbgdtrrxext, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgdtrrxint: (p14, 0, c5, c0, 0), u32: crate::registers::Dbgdtrrxint, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgdtrtxext: (p14, 0, c3, c0, 2), u32: crate::registers::Dbgdtrtxext, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(dbgdtrtxint: (p14, 0, c5, c0, 0), u32: crate::registers::Dbgdtrtxint, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgosdlr: (p14, 0, c3, c1, 4), u32: crate::registers::Dbgosdlr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgoseccr: (p14, 0, c6, c0, 2), u32: crate::registers::Dbgoseccr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(dbgoslar: (p14, 0, c0, c1, 4), u32: crate::registers::Dbgoslar, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(dbgoslsr: (p14, 0, c1, c1, 4), u32: crate::registers::Dbgoslsr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgprcr: (p14, 0, c4, c1, 4), u32: crate::registers::Dbgprcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgvcr: (p14, 0, c7, c0, 0), u32: crate::registers::Dbgvcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dbgwfar: (p14, 0, c6, c0, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dfar: (p15, 0, c0, c6, 0), u32: crate::registers::Dfar, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dfsr: (p15, 0, c0, c5, 0), u32: crate::registers::Dfsr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(disr: (p15, 0, c1, c12, 1), u32: crate::registers::Disr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(disr_el1: s3_0_c12_c1_1, u64: crate::registers::DisrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(dit: s3_3_c4_c2_5, u64: crate::registers::Dit, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dlr: (p15, 3, c5, c4, 1), u32: crate::registers::Dlr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dspsr: (p15, 3, c5, c4, 0), u32: crate::registers::Dspsr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(dspsr2: (p15, 3, c5, c4, 2), u32: crate::registers::Dspsr2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(elr_el1, u64: crate::registers::ElrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(elr_el2, u64: crate::registers::ElrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "arm"), feature = "el2"))]
read_write_sysreg!(elr_hyp, u32: crate::registers::ElrHyp, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(erridr: (p15, 0, c3, c5, 0), u32: crate::registers::Erridr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(errselr: (p15, 0, c3, c5, 1), u32: crate::registers::Errselr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxaddr: (p15, 0, c4, c5, 3), u32: crate::registers::Erxaddr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxaddr2: (p15, 0, c4, c5, 7), u32: crate::registers::Erxaddr2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxctlr: (p15, 0, c4, c5, 1), u32: crate::registers::Erxctlr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxctlr2: (p15, 0, c4, c5, 5), u32: crate::registers::Erxctlr2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(erxfr: (p15, 0, c4, c5, 0), u32: crate::registers::Erxfr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(erxfr2: (p15, 0, c4, c5, 4), u32: crate::registers::Erxfr2, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc0: (p15, 0, c5, c5, 0), u32: crate::registers::Erxmisc0, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc1: (p15, 0, c5, c5, 1), u32: crate::registers::Erxmisc1, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc2: (p15, 0, c5, c5, 4), u32: crate::registers::Erxmisc2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc3: (p15, 0, c5, c5, 5), u32: crate::registers::Erxmisc3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc4: (p15, 0, c5, c5, 2), u32: crate::registers::Erxmisc4, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc5: (p15, 0, c5, c5, 3), u32: crate::registers::Erxmisc5, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc6: (p15, 0, c5, c5, 6), u32: crate::registers::Erxmisc6, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxmisc7: (p15, 0, c5, c5, 7), u32: crate::registers::Erxmisc7, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(erxstatus: (p15, 0, c4, c5, 2), u32: crate::registers::Erxstatus, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(esr_el1, u64: crate::registers::EsrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(esr_el2, u64: crate::registers::EsrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(esr_el3, u64: crate::registers::EsrEl3, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(far_el1, u64: crate::registers::FarEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(far_el2, u64: crate::registers::FarEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(fcseidr: (p15, 0, c0, c13, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(fgwte3_el3: s3_6_c1_c1_5, u64: crate::registers::Fgwte3El3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(fpcr, u64: crate::registers::Fpcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(fpmr: s3_3_c4_c4_2, u64: crate::registers::Fpmr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(fpsr, u64: crate::registers::Fpsr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(gcr_el1: s3_0_c1_c0_6, u64: crate::registers::GcrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(gcscr_el1: s3_0_c2_c5_0, u64: crate::registers::GcscrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(gcscr_el2: s3_4_c2_c5_0, u64: crate::registers::GcscrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(gcspr_el1: s3_0_c2_c5_1, u64: crate::registers::GcsprEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(gcspr_el2: s3_4_c2_c5_1, u64: crate::registers::GcsprEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(gpccr_el3: s3_6_c2_c1_6, u64: crate::registers::GpccrEl3, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(gptbr_el3: s3_6_c2_c1_4, u64: crate::registers::GptbrEl3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hacr: (p15, 4, c1, c1, 7), u32, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hacr_el2, u64, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hactlr: (p15, 4, c0, c1, 1), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hactlr2: (p15, 4, c0, c1, 3), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hadfsr: (p15, 4, c1, c5, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hafgrtr_el2: s3_4_c3_c1_6, u64: crate::registers::HafgrtrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(haifsr: (p15, 4, c1, c5, 1), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hamair0: (p15, 4, c3, c10, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hamair1: (p15, 4, c3, c10, 1), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hcptr: (p15, 4, c1, c1, 2), u32: crate::registers::Hcptr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hcr: (p15, 4, c1, c1, 0), u32: crate::registers::Hcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hcr2: (p15, 4, c1, c1, 4), u32: crate::registers::Hcr2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hcrx_el2: s3_4_c1_c2_2, u64: crate::registers::HcrxEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hcr_el2, u64: crate::registers::HcrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hdcr: (p15, 4, c1, c1, 1), u32: crate::registers::Hdcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hdfar: (p15, 4, c0, c6, 0), u32: crate::registers::Hdfar, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgrtr2_el2: s3_4_c3_c1_0, u64: crate::registers::Hdfgrtr2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgrtr_el2: s3_4_c3_c1_4, u64: crate::registers::HdfgrtrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgwtr2_el2: s3_4_c3_c1_1, u64: crate::registers::Hdfgwtr2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hdfgwtr_el2: s3_4_c3_c1_5, u64: crate::registers::HdfgwtrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgitr2_el2: s3_4_c3_c1_7, u64: crate::registers::Hfgitr2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgitr_el2: s3_4_c1_c1_6, u64: crate::registers::HfgitrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgrtr2_el2: s3_4_c3_c1_2, u64: crate::registers::Hfgrtr2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgrtr_el2: s3_4_c1_c1_4, u64: crate::registers::HfgrtrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgwtr2_el2: s3_4_c3_c1_3, u64: crate::registers::Hfgwtr2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hfgwtr_el2: s3_4_c1_c1_5, u64: crate::registers::HfgwtrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hifar: (p15, 4, c0, c6, 2), u32: crate::registers::Hifar, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hmair0: (p15, 4, c2, c10, 0), u32: crate::registers::Hmair0, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hmair1: (p15, 4, c2, c10, 1), u32: crate::registers::Hmair1, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hpfar: (p15, 4, c0, c6, 4), u32: crate::registers::Hpfar, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hpfar_el2, u64: crate::registers::HpfarEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hrmr: (p15, 4, c0, c12, 2), u32: crate::registers::Hrmr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hsctlr: (p15, 4, c0, c1, 0), u32: crate::registers::Hsctlr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hsr: (p15, 4, c2, c5, 0), u32: crate::registers::Hsr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hstr: (p15, 4, c1, c1, 3), u32, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(hstr_el2, u64, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(htcr: (p15, 4, c0, c2, 2), u32: crate::registers::Htcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(htpidr: (p15, 4, c0, c13, 2), u32: crate::registers::Htpidr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(htrfcr: (p15, 4, c2, c1, 1), u32: crate::registers::Htrfcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(httbr: (p15, 4, c2), u64: crate::registers::Httbr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(hvbar: (p15, 4, c0, c12, 0), u32: crate::registers::Hvbar, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap0r0_el1: s3_0_c12_c8_4, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap0r1_el1: s3_0_c12_c8_5, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap0r2_el1: s3_0_c12_c8_6, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap0r3_el1: s3_0_c12_c8_7, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap1r0_el1: s3_0_c12_c9_0, u64: crate::registers::IccAp1r0El1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap1r1_el1: s3_0_c12_c9_1, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap1r2_el1: s3_0_c12_c9_2, u64, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ap1r3_el1: s3_0_c12_c9_3, u64, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(icc_asgi1r: (p15, 1, c12), u64: crate::registers::IccAsgi1r, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
write_sysreg!(icc_asgi1r_el1: s3_0_c12_c11_6, u64: crate::registers::IccAsgi1rEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_bpr0: (p15, 0, c8, c12, 3), u32: crate::registers::IccBpr0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_bpr0_el1: s3_0_c12_c8_3, u64: crate::registers::IccBpr0El1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_bpr1: (p15, 0, c12, c12, 3), u32: crate::registers::IccBpr1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_bpr1_el1: s3_0_c12_c12_3, u64: crate::registers::IccBpr1El1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_ctlr: (p15, 0, c12, c12, 4), u32: crate::registers::IccCtlr, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_ctlr_el1: s3_0_c12_c12_4, u64: crate::registers::IccCtlrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(icc_ctlr_el3: s3_6_c12_c12_4, u64: crate::registers::IccCtlrEl3, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(icc_dir: (p15, 0, c11, c12, 1), u32: crate::registers::IccDir, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
write_sysreg!(icc_dir_el1: s3_0_c12_c11_1, u64: crate::registers::IccDirEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(icc_eoir0: (p15, 0, c8, c12, 1), u32: crate::registers::IccEoir0, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
write_sysreg!(icc_eoir0_el1: s3_0_c12_c8_1, u64: crate::registers::IccEoir0El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(icc_eoir1: (p15, 0, c12, c12, 1), u32: crate::registers::IccEoir1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
write_sysreg!(icc_eoir1_el1: s3_0_c12_c12_1, u64: crate::registers::IccEoir1El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(icc_hppir0: (p15, 0, c8, c12, 2), u32: crate::registers::IccHppir0, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(icc_hppir0_el1: s3_0_c12_c8_2, u64: crate::registers::IccHppir0El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(icc_hppir1: (p15, 0, c12, c12, 2), u32: crate::registers::IccHppir1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(icc_hppir1_el1: s3_0_c12_c12_2, u64: crate::registers::IccHppir1El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_hsre: (p15, 4, c9, c12, 5), u32: crate::registers::IccHsre, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(icc_iar0: (p15, 0, c8, c12, 0), u32: crate::registers::IccIar0, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(icc_iar0_el1: s3_0_c12_c8_0, u64: crate::registers::IccIar0El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(icc_iar1: (p15, 0, c12, c12, 0), u32: crate::registers::IccIar1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(icc_iar1_el1: s3_0_c12_c12_0, u64: crate::registers::IccIar1El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_igrpen0: (p15, 0, c12, c12, 6), u32: crate::registers::IccIgrpen0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_igrpen0_el1: s3_0_c12_c12_6, u64: crate::registers::IccIgrpen0El1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_igrpen1: (p15, 0, c12, c12, 7), u32: crate::registers::IccIgrpen1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_igrpen1_el1: s3_0_c12_c12_7, u64: crate::registers::IccIgrpen1El1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(icc_igrpen1_el3: s3_6_c12_c12_7, u64: crate::registers::IccIgrpen1El3, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_mctlr: (p15, 6, c12, c12, 4), u32: crate::registers::IccMctlr, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_mgrpen1: (p15, 6, c12, c12, 7), u32: crate::registers::IccMgrpen1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_msre: (p15, 6, c12, c12, 5), u32: crate::registers::IccMsre, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(icc_nmiar1_el1: s3_0_c12_c9_5, u64: crate::registers::IccNmiar1El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_pmr: (p15, 0, c6, c4, 0), u32: crate::registers::IccPmr, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_pmr_el1: s3_0_c4_c6_0, u64: crate::registers::IccPmrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(icc_rpr: (p15, 0, c11, c12, 3), u32: crate::registers::IccRpr, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(icc_rpr_el1: s3_0_c12_c11_3, u64: crate::registers::IccRprEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(icc_sgi0r: (p15, 2, c12), u64: crate::registers::IccSgi0r, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
write_sysreg!(icc_sgi0r_el1: s3_0_c12_c11_7, u64: crate::registers::IccSgi0rEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(icc_sgi1r: (p15, 0, c12), u64: crate::registers::IccSgi1r, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
write_sysreg!(icc_sgi1r_el1: s3_0_c12_c11_5, u64: crate::registers::IccSgi1rEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(icc_sre: (p15, 0, c12, c12, 5), u32: crate::registers::IccSre, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(icc_sre_el1: s3_0_c12_c12_5, u64: crate::registers::IccSreEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(icc_sre_el2: s3_4_c12_c9_5, u64: crate::registers::IccSreEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The SRE bit of `icc_sre_el3` must not be changed from 1 to 0, as this can result in unpredictable behaviour.
    icc_sre_el3: s3_6_c12_c12_5, u64: crate::registers::IccSreEl3, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(ich_hcr_el2: s3_4_c12_c11_0, u64: crate::registers::IchHcrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(ich_vmcr_el2: s3_4_c12_c11_7, u64: crate::registers::IchVmcrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64dfr0_el1, u64: crate::registers::IdAa64dfr0El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64dfr1_el1, u64: crate::registers::IdAa64dfr1El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64isar1_el1, u64: crate::registers::IdAa64isar1El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64isar2_el1, u64: crate::registers::IdAa64isar2El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64isar3_el1, u64: crate::registers::IdAa64isar3El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr0_el1, u64: crate::registers::IdAa64mmfr0El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr1_el1, u64: crate::registers::IdAa64mmfr1El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr2_el1, u64: crate::registers::IdAa64mmfr2El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr3_el1, u64: crate::registers::IdAa64mmfr3El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64mmfr4_el1, u64: crate::registers::IdAa64mmfr4El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64pfr0_el1, u64: crate::registers::IdAa64pfr0El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64pfr1_el1, u64: crate::registers::IdAa64pfr1El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64pfr2_el1, u64: crate::registers::IdAa64pfr2El1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(id_aa64smfr0_el1: s3_0_c0_c4_5, u64: crate::registers::IdAa64smfr0El1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_afr0: (p15, 0, c1, c0, 3), u32, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_dfr0: (p15, 0, c1, c0, 2), u32: crate::registers::IdDfr0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_dfr1: (p15, 0, c3, c0, 5), u32: crate::registers::IdDfr1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar0: (p15, 0, c2, c0, 0), u32: crate::registers::IdIsar0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar1: (p15, 0, c2, c0, 1), u32: crate::registers::IdIsar1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar2: (p15, 0, c2, c0, 2), u32: crate::registers::IdIsar2, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar3: (p15, 0, c2, c0, 3), u32: crate::registers::IdIsar3, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar4: (p15, 0, c2, c0, 4), u32: crate::registers::IdIsar4, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar5: (p15, 0, c2, c0, 5), u32: crate::registers::IdIsar5, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_isar6: (p15, 0, c2, c0, 7), u32: crate::registers::IdIsar6, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr0: (p15, 0, c1, c0, 4), u32: crate::registers::IdMmfr0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr1: (p15, 0, c1, c0, 5), u32: crate::registers::IdMmfr1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr2: (p15, 0, c1, c0, 6), u32: crate::registers::IdMmfr2, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr3: (p15, 0, c1, c0, 7), u32: crate::registers::IdMmfr3, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr4: (p15, 0, c2, c0, 6), u32: crate::registers::IdMmfr4, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_mmfr5: (p15, 0, c3, c0, 6), u32: crate::registers::IdMmfr5, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_pfr0: (p15, 0, c1, c0, 0), u32: crate::registers::IdPfr0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_pfr1: (p15, 0, c1, c0, 1), u32: crate::registers::IdPfr1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(id_pfr2: (p15, 0, c3, c0, 4), u32: crate::registers::IdPfr2, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ifar: (p15, 0, c0, c6, 2), u32: crate::registers::Ifar, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ifsr: (p15, 0, c0, c5, 1), u32: crate::registers::Ifsr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(isr: (p15, 0, c1, c12, 0), u32: crate::registers::Isr, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(isr_el1, u64: crate::registers::IsrEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(jidr: (p14, 7, c0, c0, 0), u32, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(jmcr: (p14, 7, c0, c2, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(joscr: (p14, 7, c0, c1, 0), u32, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(mair0: (p15, 0, c2, c10, 0), u32: crate::registers::Mair0, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(mair1: (p15, 0, c2, c10, 1), u32: crate::registers::Mair1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(mair_el1, u64: crate::registers::MairEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mair_el2, u64: crate::registers::MairEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 memory attribute indirection register.
    mair_el3, u64: crate::registers::MairEl3, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(mdccint_el1, u64: crate::registers::MdccintEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mdcr_el2, u64: crate::registers::MdcrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(mdcr_el3, u64: crate::registers::MdcrEl3, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(mdscr_el1, u64: crate::registers::MdscrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(midr: (p15, 0, c0, c0, 0), u32: crate::registers::Midr, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(midr_el1, u64: crate::registers::MidrEl1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpam2_el2: s3_4_c10_c5_0, u64: crate::registers::Mpam2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(mpam3_el3: s3_6_c10_c5_0, u64: crate::registers::Mpam3El3, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamhcr_el2: s3_4_c10_c4_0, u64: crate::registers::MpamhcrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(mpamidr_el1: s3_0_c10_c4_4, u64: crate::registers::MpamidrEl1, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm0_el2: s3_4_c10_c6_0, u64: crate::registers::Mpamvpm0El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm1_el2: s3_4_c10_c6_1, u64: crate::registers::Mpamvpm1El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm2_el2: s3_4_c10_c6_2, u64: crate::registers::Mpamvpm2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm3_el2: s3_4_c10_c6_3, u64: crate::registers::Mpamvpm3El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm4_el2: s3_4_c10_c6_4, u64: crate::registers::Mpamvpm4El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm5_el2: s3_4_c10_c6_5, u64: crate::registers::Mpamvpm5El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm6_el2: s3_4_c10_c6_6, u64: crate::registers::Mpamvpm6El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpm7_el2: s3_4_c10_c6_7, u64: crate::registers::Mpamvpm7El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(mpamvpmv_el2: s3_4_c10_c4_1, u64: crate::registers::MpamvpmvEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(mpidr: (p15, 0, c0, c0, 5), u32: crate::registers::Mpidr, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_sysreg!(mpidr_el1, u64: crate::registers::MpidrEl1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(mvbar: (p15, 0, c0, c12, 1), u32: crate::registers::Mvbar, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(nmrr: (p15, 0, c2, c10, 1), u32: crate::registers::Nmrr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(nsacr: (p15, 0, c1, c1, 2), u32: crate::registers::Nsacr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(par: (p15, 0, c7), u64: crate::registers::Par, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(par_el1, u64: crate::registers::ParEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(pfar_el1: s3_0_c6_c0_5, u64: crate::registers::PfarEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(pfar_el2: s3_4_c6_c0_5, u64: crate::registers::PfarEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(pire0_el1: s3_0_c10_c2_2, u64: crate::registers::Pire0El1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(pire0_el2: s3_4_c10_c2_2, u64: crate::registers::Pire0El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(pir_el1: s3_0_c10_c2_3, u64: crate::registers::PirEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(pir_el2: s3_4_c10_c2_3, u64: crate::registers::PirEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(pir_el3: s3_6_c10_c2_3, u64: crate::registers::PirEl3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmccfiltr: (p15, 0, c15, c14, 7), u32: crate::registers::Pmccfiltr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmccntr: (p15, 0, c9), u64: crate::registers::Pmccntr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid0: (p15, 0, c12, c9, 6), u32: crate::registers::Pmceid0, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid1: (p15, 0, c12, c9, 7), u32: crate::registers::Pmceid1, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid2: (p15, 0, c14, c9, 4), u32: crate::registers::Pmceid2, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmceid3: (p15, 0, c14, c9, 5), u32: crate::registers::Pmceid3, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmcntenclr: (p15, 0, c12, c9, 2), u32: crate::registers::Pmcntenclr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmcntenset: (p15, 0, c12, c9, 1), u32: crate::registers::Pmcntenset, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmcr: (p15, 0, c12, c9, 0), u32: crate::registers::Pmcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(pmcr_el0: s3_3_c9_c12_0, u64: crate::registers::PmcrEl0, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmintenclr: (p15, 0, c14, c9, 2), u32: crate::registers::Pmintenclr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmintenset: (p15, 0, c14, c9, 1), u32: crate::registers::Pmintenset, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(pmmir: (p15, 0, c14, c9, 6), u32: crate::registers::Pmmir, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmovsr: (p15, 0, c12, c9, 3), u32: crate::registers::Pmovsr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmovsset: (p15, 0, c14, c9, 3), u32: crate::registers::Pmovsset, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmselr: (p15, 0, c12, c9, 5), u32: crate::registers::Pmselr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
write_sysreg!(pmswinc: (p15, 0, c12, c9, 4), u32: crate::registers::Pmswinc, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmuserenr: (p15, 0, c14, c9, 0), u32: crate::registers::Pmuserenr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(pmxevtyper: (p15, 0, c13, c9, 1), u32: crate::registers::Pmxevtyper, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(por_el0: s3_3_c10_c2_4, u64: crate::registers::PorEl0, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(por_el1: s3_0_c10_c2_4, u64: crate::registers::PorEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(por_el2: s3_4_c10_c2_4, u64: crate::registers::PorEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(por_el3: s3_6_c10_c2_4, u64: crate::registers::PorEl3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(prrr: (p15, 0, c2, c10, 0), u32: crate::registers::Prrr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(revidr: (p15, 0, c0, c0, 6), u32, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(rgsr_el1: s3_0_c1_c0_5, u64: crate::registers::RgsrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(rmr: (p15, 0, c0, c12, 2), u32: crate::registers::Rmr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(rvbar: (p15, 0, c0, c12, 1), u32: crate::registers::Rvbar, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(s2pir_el2: s3_4_c10_c2_5, u64: crate::registers::S2pirEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(s2por_el1: s3_0_c10_c2_5, u64: crate::registers::S2porEl1, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(scr: (p15, 0, c1, c1, 0), u32: crate::registers::Scr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(scr_el3, u64: crate::registers::ScrEl3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(sctlr: (p15, 0, c0, c1, 0), u32: crate::registers::Sctlr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(sctlr2_el1: s3_0_c1_c0_3, u64: crate::registers::Sctlr2El1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(sctlr2_el2: s3_4_c1_c0_3, u64: crate::registers::Sctlr2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(sctlr2_el3: s3_6_c1_c0_3, u64: crate::registers::Sctlr2El3, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(sctlr_el1, u64: crate::registers::SctlrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(sctlr_el2, u64: crate::registers::SctlrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 system control register.
    sctlr_el3, u64: crate::registers::SctlrEl3, safe_read, crate::fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(sdcr: (p15, 0, c3, c1, 1), u32: crate::registers::Sdcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(sder: (p15, 0, c1, c1, 1), u32: crate::registers::Sder, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(smcr_el3: s3_6_c1_c2_6, u64: crate::registers::SmcrEl3, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(spsr_el1, u64: crate::registers::SpsrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(spsr_el2, u64: crate::registers::SpsrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(spsr_el3, u64: crate::registers::SpsrEl3, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(sp_el1, u64: crate::registers::SpEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(sp_el2, u64: crate::registers::SpEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(svcr: s3_3_c4_c2_2, u64: crate::registers::Svcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(tcmtr: (p15, 0, c0, c0, 2), u32, safe, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tcr2_el1: s3_0_c2_c0_3, u64: crate::registers::Tcr2El1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tcr2_el2: s3_4_c2_c0_3, u64: crate::registers::Tcr2El2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tcr_el1, u64: crate::registers::TcrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tcr_el2, u64: crate::registers::TcrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The caller must ensure that `value` is a correct and safe configuration value for the EL3 translation control register.
    tcr_el3, u64: crate::registers::TcrEl3, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tfsre0_el1: s3_0_c5_c6_1, u64: crate::registers::Tfsre0El1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tfsr_el1: s3_0_c5_c6_0, u64: crate::registers::TfsrEl1, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tfsr_el2: s3_4_c5_c6_0, u64: crate::registers::TfsrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_sysreg!(tlbtr: (p15, 0, c0, c0, 3), u32: crate::registers::Tlbtr, safe, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(tpidrprw: (p15, 0, c0, c13, 4), u32: crate::registers::Tpidrprw, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(tpidrro_el0, u64: crate::registers::TpidrroEl0, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(tpidruro: (p15, 0, c0, c13, 3), u32: crate::registers::Tpidruro, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(tpidrurw: (p15, 0, c0, c13, 2), u32: crate::registers::Tpidrurw, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
read_write_sysreg!(tpidr_el0, u64: crate::registers::TpidrEl0, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg!(tpidr_el1, u64: crate::registers::TpidrEl1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(tpidr_el2, u64: crate::registers::TpidrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(tpidr_el3, u64: crate::registers::TpidrEl3, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(trfcr: (p15, 0, c2, c1, 1), u32: crate::registers::Trfcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbcr: (p15, 0, c0, c2, 2), u32: crate::registers::Ttbcr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbcr2: (p15, 0, c0, c2, 3), u32: crate::registers::Ttbcr2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbr0: (p15, 0, c2), u64: crate::registers::Ttbr0, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el1, u64: crate::registers::Ttbr0El1, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el2, u64: crate::registers::Ttbr0El2, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr0_el3, u64: crate::registers::Ttbr0El3, safe_read, crate::fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(ttbr1: (p15, 1, c2), u64: crate::registers::Ttbr1, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el1, u64: crate::registers::Ttbr1El1, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned translation table.
    ttbr1_el2: s3_4_c2_c0_1, u64: crate::registers::Ttbr1El2, safe_read, crate::fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vbar: (p15, 0, c0, c12, 0), u32: crate::registers::Vbar, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el1"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el1, u64: crate::registers::VbarEl1, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid exception vector.
    vbar_el2, u64: crate::registers::VbarEl2, safe_read, crate::fake::SYSREGS
}
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vdfsr: (p15, 4, c2, c5, 3), u32: crate::registers::Vdfsr, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vdisr: (p15, 0, c1, c12, 1), u32: crate::registers::Vdisr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vdisr_el2: s3_4_c12_c1_1, u64: crate::registers::VdisrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vmpidr: (p15, 0, c0, c0, 5), u32: crate::registers::Vmpidr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vmpidr_el2, u64: crate::registers::VmpidrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vpidr: (p15, 0, c0, c0, 0), u32: crate::registers::Vpidr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vpidr_el2, u64: crate::registers::VpidrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vsesr_el2: s3_4_c5_c2_3, u64: crate::registers::VsesrEl2, safe_read, safe_write, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vtcr: (p15, 4, c1, c2, 2), u32: crate::registers::Vtcr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg!(vtcr_el2, u64: crate::registers::VtcrEl2, safe_read, crate::fake::SYSREGS);
#[cfg(any(test, feature = "fakes", target_arch = "arm"))]
read_write_sysreg!(vttbr: (p15, 6, c2), u64: crate::registers::Vttbr, safe_read, crate::fake::SYSREGS);
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el2"))]
read_write_sysreg! {
    /// # Safety
    ///
    /// The base address must point to a valid and properly aligned stage 2 translation table.
    vttbr_el2, u64: crate::registers::VttbrEl2, safe_read, crate::fake::SYSREGS
}
#[cfg(all(any(test, feature = "fakes", target_arch = "aarch64"), feature = "el3"))]
read_write_sysreg!(zcr_el3: s3_6_c1_c2_0, u64: crate::registers::ZcrEl3, safe_read, crate::fake::SYSREGS);
