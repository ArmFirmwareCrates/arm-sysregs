// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Manually implemented methods for EL0 Aarch64 system register types.

#[cfg(test)]
mod tests {
    use crate::registers::CntpCtlEl0;

    #[test]
    fn debug_cntp_ctl_el0() {
        assert_eq!(format!("{:?}", CntpCtlEl0::empty()), "CntpCtlEl0(0x0)");
        assert_eq!(
            format!("{:?}", CntpCtlEl0::ENABLE | CntpCtlEl0::ISTATUS),
            "CntpCtlEl0(ENABLE | ISTATUS)"
        );
    }
}
