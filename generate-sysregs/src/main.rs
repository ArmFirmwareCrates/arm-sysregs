// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

mod config;
mod json_input;
mod output;

use crate::{
    config::Config,
    json_input::register_entries_to_register_infos,
    output::{write_fake, write_lib},
};
use arm_sysregs_json::RegisterEntry;
use clap::Parser;
use eyre::Report;
use log::{info, warn};
use std::{
    fs::{File, read_to_string},
    ops::Range,
    path::PathBuf,
};

fn main() -> Result<(), Report> {
    pretty_env_logger::init();
    let args = Args::parse();
    let config: Config = toml::from_str(&read_to_string(&args.config_toml)?)?;
    let registers =
        serde_json::from_str::<Vec<RegisterEntry>>(&read_to_string(&args.registers_json)?)?;
    println!(
        "Read {} system registers from {}",
        registers.len(),
        args.registers_json.display()
    );
    let output_lib = File::create(args.output_directory.join("lib.rs"))?;
    let output_fake = File::create(args.output_directory.join("fake").join("generated.rs"))?;
    let registers_filter = config.registers.keys().collect::<Vec<_>>();
    let mut register_infos = register_entries_to_register_infos(&registers, &registers_filter);
    for register in &mut register_infos {
        remove_clashes(register);
        add_details(register, &config);
        remove_over_64bit(register);
    }
    register_infos.sort_by_cached_key(|register| register.name.clone());
    register_infos.retain(|register| register.width > 0);
    warn_missing(&register_infos, &config);
    write_lib(&output_lib, &register_infos)?;
    write_fake(&output_fake, &register_infos)?;

    Ok(())
}

/// Logs warnings for any registers which are present in the config file but not the JSON file.
fn warn_missing(register_infos: &[RegisterInfo], config: &Config) {
    for register_name in config.registers.keys() {
        if !register_infos
            .iter()
            .any(|register_info| &register_info.name == register_name)
        {
            warn!(
                "Register {} in config file but missing from JSON input.",
                register_name
            );
        }
    }
}

/// Removes any fields which have the same name as each other, and only keep one copy of any that
/// are exact duplicates.
fn remove_clashes(register: &mut RegisterInfo) {
    register
        .fields
        .sort_by_cached_key(|field| (field.index, field.name.clone()));
    register.fields.dedup();

    let fields_copy = register.fields.clone();
    register.fields.retain(|field| {
        !fields_copy
            .iter()
            .any(|other_field| field != other_field && field.name == other_field.name)
    });
}

/// Remove any fields outside of the lower 64 bits, as we don't yet support 128-bit registers.
fn remove_over_64bit(register: &mut RegisterInfo) {
    if register.width > 64 {
        info!(
            "Trimming {}-bit register {} to 64 bit.",
            register.width, register.name
        );
        register.width = 64;
    }
    register
        .fields
        .retain(|field| field.index + field.width <= 64)
}

fn add_details(register: &mut RegisterInfo, config: &Config) {
    if let Some(register_config) = config.registers.get(&register.name) {
        if let Some(description) = &register_config.description {
            register.description = Some(description.clone());
        }
        if let Some(read) = register_config.read {
            register.read = read.into();
        }
        if let Some(write) = register_config.write {
            register.write = write.into();
        }
        register.write_safety_doc = register_config.write_safety_doc.clone();
        register.derive_debug = !register_config.manual_debug;
        for field in &mut register.fields {
            if let Some(description) = register_config.field_descriptions.get(&field.name) {
                field.description = Some(description.clone());
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterField {
    /// The name of the field.
    pub name: String,
    /// The description of the field, if available.
    pub description: Option<String>,
    /// The index of the least significant bit of the field.
    pub index: u32,
    /// The width of the field in bits.
    pub width: u32,
    /// Whether the field is writable.
    pub writable: bool,
    /// Information about the array, if it is an array field.
    pub array_info: Option<ArrayInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayInfo {
    /// The range of entries in the array.
    pub indices: Range<u32>,
    /// The placeholder variable name.
    pub index_variable: String,
}

impl ArrayInfo {
    fn placeholder(&self) -> String {
        format!("<{}>", self.index_variable)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RegisterInfo {
    pub name: String,
    /// The description of the register, if available.
    pub description: Option<String>,
    pub width: u32,
    pub fields: Vec<RegisterField>,
    /// All the bits which are RES1.
    pub res1: u64,
    pub read: Option<Safety>,
    pub write: Option<Safety>,
    pub write_safety_doc: Option<String>,
    pub derive_debug: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(Clone, Debug, Parser)]
struct Args {
    /// Path to config toml file.
    config_toml: PathBuf,
    /// Path to JSON system registers file.
    registers_json: PathBuf,
    /// Path to output directory.
    output_directory: PathBuf,
}

/// Returns a value with the given number of 1 bits, starting at the least significant bit.
const fn ones(n: u32) -> u64 {
    u64::MAX >> (64 - n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_clashing_names() {
        let mut register = RegisterInfo {
            fields: vec![
                RegisterField {
                    name: "FOO".to_string(),
                    description: None,
                    index: 0,
                    width: 1,
                    writable: false,
                    array_info: None,
                },
                RegisterField {
                    name: "FOO".to_string(),
                    description: None,
                    index: 1,
                    width: 1,
                    writable: false,
                    array_info: None,
                },
                RegisterField {
                    name: "BAR".to_string(),
                    description: None,
                    index: 0,
                    width: 1,
                    writable: false,
                    array_info: None,
                },
                RegisterField {
                    name: "BAZ".to_string(),
                    description: None,
                    index: 0,
                    width: 1,
                    writable: false,
                    array_info: None,
                },
            ],
            ..Default::default()
        };
        remove_clashes(&mut register);
        assert_eq!(
            register,
            RegisterInfo {
                fields: vec![
                    RegisterField {
                        name: "BAR".to_string(),
                        description: None,
                        index: 0,
                        width: 1,
                        writable: false,
                        array_info: None,
                    },
                    RegisterField {
                        name: "BAZ".to_string(),
                        description: None,
                        index: 0,
                        width: 1,
                        writable: false,
                        array_info: None,
                    },
                ],
                ..Default::default()
            },
        );
    }
}
