// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

#![cfg_attr(not(any(test, feature = "fakes")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(not(any(test, feature = "fakes")), target_arch = "arm"))]
mod aarch32;
#[cfg(all(not(any(test, feature = "fakes")), target_arch = "aarch64"))]
mod aarch64;
#[cfg(any(test, feature = "fakes", target_arch = "arm", target_arch = "aarch64"))]
pub mod accessors;
#[cfg(any(test, feature = "fakes"))]
pub mod fake;
pub mod helpers;
mod macros;
pub mod manual;
pub mod registers;

#[doc(hidden)]
pub use paste as _paste;
