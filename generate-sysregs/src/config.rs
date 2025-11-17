// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Safety;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub registers: BTreeMap<String, RegisterConfig>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterConfig {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub field_descriptions: BTreeMap<String, String>,
    /// If this is set it overrides the read access from the JSON input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<AccessType>,
    /// If this is set it overrides the write access from the JSON input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<AccessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_safety_doc: Option<String>,
    #[serde(default)]
    pub manual_debug: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessType {
    Never,
    Unsafe,
    Safe,
}

impl From<AccessType> for Option<Safety> {
    fn from(value: AccessType) -> Self {
        match value {
            AccessType::Never => None,
            AccessType::Unsafe => Some(Safety::Unsafe),
            AccessType::Safe => Some(Safety::Safe),
        }
    }
}
