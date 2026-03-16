# Changelog

## Unreleased

### New features

- Added more GIC registers.
- Made some GIC registers safe to write.
- Added `with_*` methods to set fields taking `self` by value and returning it. This allows a less
  verbose builder pattern for register values.

### Bugfixes

- Fixed compilation error for output of `read_write_sysregs!` on AArch32 in some cases.

## 0.2.5

### New features

- Added SVCR register.
- Added TPIDR_EL3 register.
- Added Generic Timer registers.
- Added GIC registers.
- Added more fields to `MpamhcrEl2`.

## 0.2.4

### Bugfixes

- Fixed accessors for `ID_AA64SMFR0_EL1` and `TTBR1_EL2` raw encoding name in assembly, to avoid
  compilation errors when the assembler doesn't recognise the system register names.

## 0.2.3

### New features

- Added AMU feature detection for the `ID_AA64PFR0_EL1` register.
- Added FGT feature registers.

## 0.2.2

### New features

- Register structs now implement `Default`.
- Register structs now have setters for multibit fields.

## 0.2.1

### Bugfixes

- Fixed `IdAa64mmfr0El1::FGT2_SUPPORTED` definition.

### New features

- Added AArch32 support to sysregs macros.
- Added many AArch32 system registers.

## 0.2.0

### Breaking changes

- Added `CurrentEL` register.
- Added `GPCCR_EL3` and `GPTBR_EL3` registers.
- Added `CNTPCT_EL0`, `ID_AA64ISAR1_EL1`, `ID_AA64ISAR2_EL1` and `SCTLR2_EL3` registers.
- Marked fake `SystemRegisters` struct as `#[non_exhaustive]`.

### Other changes

- Fixed warnings in Rustdoc.
- Included all fearures in docs.rs build.
- Documented feature flags in README.

## 0.1.0

Initial release.
