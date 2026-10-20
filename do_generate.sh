#!/bin/sh
# SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
# SPDX-License-Identifier: MIT OR Apache-2.0

VER=2026-03
BASE=AARCHMRS_OPENSOURCE_A_profile_FAT
URL=https://developer.arm.com/-/cdn-downloads/permalink/Exploration-Tools-OS-Machine-Readable-Data/AARCHMRS_BSD/$BASE-$VER.tar.gz
JSON=$BASE-$VER-Registers.json

set -ex

if [ ! -f "$JSON" ]; then
	curl -f "$URL" | tar -xzO Registers.json >"$JSON" || { rm -f "$JSON"; exit 1; }
fi

cargo run --package generate-sysregs -- registers.toml "$JSON" generate-crates .
cargo fmt
