// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

mod config;
mod enums;
mod json_input;
mod output;

use crate::{
    config::Config,
    enums::identify_enums,
    json_input::register_entries_to_register_infos,
    output::{write_fake, write_lib},
};
use arm_sysregs_json::{RegisterEntry, Values};
use clap::{Parser, Subcommand};
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
    let register_infos = parse_registers(&config, args.registers_json, args.all)?;

    match args.command {
        Command::Generate { output_directory } => {
            let output_lib = File::create(output_directory.join("lib.rs"))?;
            let output_fake = File::create(output_directory.join("fake").join("generated.rs"))?;

            warn_missing(&register_infos, &config);
            write_lib(&output_lib, &register_infos)?;
            write_fake(&output_fake, &register_infos)?;
        }
        Command::Enums {
            generate_stubs,
            skip_existing,
        } => {
            identify_enums(&register_infos, generate_stubs, skip_existing);
        }
    }

    Ok(())
}

fn parse_registers(
    config: &Config,
    registers_json: PathBuf,
    use_all_registers: bool,
) -> Result<Vec<RegisterInfo>, Report> {
    let registers = serde_json::from_str::<Vec<RegisterEntry>>(&read_to_string(&registers_json)?)?;
    println!(
        "Read {} system registers from {}",
        registers.len(),
        registers_json.display()
    );

    let registers_filter = if use_all_registers {
        None
    } else {
        Some(config.registers.keys().collect::<Vec<_>>())
    };
    let mut register_infos =
        register_entries_to_register_infos(&registers, registers_filter.as_deref());

    for register in &mut register_infos {
        remove_clashes(register);
        add_details(register, config);
        remove_over_64bit(register);
    }

    register_infos.sort_by_cached_key(|register| register.name.clone());
    register_infos.retain(|register| register.width > 0);

    Ok(register_infos)
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
/// have the same main attributes (see [`RegisterField::is_incompatible_with`]).
fn remove_clashes(register: &mut RegisterInfo) {
    register.fields.sort_by_cached_key(|r| r.key());
    register.fields.dedup_by_key(|r| r.key());

    let fields_copy = register.fields.clone();
    register.fields.retain(|field| {
        let clash = fields_copy
            .iter()
            .any(|other_field| field.is_incompatible_with(other_field));
        if clash {
            if field.width == 1 {
                info!(
                    "Removing clashing field {} ({}) from register {}",
                    field.name, field.index, register.name,
                );
            } else {
                info!(
                    "Removing clashing field {} ({}..{}) from register {}",
                    field.name,
                    field.index,
                    field.index + field.width,
                    register.name,
                );
            }
        }
        !clash
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
            if let Some(ty) = register_config.types.get(&field.name) {
                field.type_name = Some(ty.clone());
            }
        }
        if !register.has_special_conditions && !register_config.use_raw_name {
            register.assembly_name = None;
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterField {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub type_name: Option<String>,
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
    /// Possible values that this field can hold, if specified.
    pub values: Option<Values>,
}

impl RegisterField {
    fn key(&self) -> (u32, String, u32, bool) {
        (self.index, self.name.clone(), self.width, self.writable)
    }

    /// Whether `self` and `other` are incompatible and should be ignored by generation.
    ///
    /// Two registers are considered to incompatible if they have the same name but their main
    /// attributes (position, width or writability) differ.
    fn is_incompatible_with(&self, other: &Self) -> bool {
        self.name == other.name && self.key() != other.key()
    }
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
    /// The register is available from AArch32 state.
    pub aarch32: bool,
    /// The register is available from AArch64 state.
    pub aarch64: bool,
    pub fields: Vec<RegisterField>,
    /// All the bits which are RES1.
    pub res1: u64,
    pub read: Option<Safety>,
    pub write: Option<Safety>,
    pub write_safety_doc: Option<String>,
    pub derive_debug: bool,
    pub assembly_name: Option<String>,
    pub aarch32_encoding: Option<AArch32Encoding>,
    /// The register has conditions beyond just AArch64 and having certain exception levels.
    ///
    /// For example, it might require certain CPU features.
    pub has_special_conditions: bool,
    /// The lowest exception level at which this system register is accessible.
    pub exception_level: ExceptionLevel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum AArch32Encoding {
    Single {
        crm: u8,
        crn: u8,
        coproc: u8,
        opc1: u8,
        opc2: u8,
    },
    Double {
        crm: u8,
        coproc: u8,
        opc1: u8,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ExceptionLevel {
    #[default]
    El0,
    El1,
    El2,
    El3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    /// Generate all system registers.
    Generate {
        /// Path to output directory.
        output_directory: PathBuf,
    },
    /// Scans the register values to identify fields that could be represented as Rust enums.
    Enums {
        /// Generate a stub implementation for the encountered enums, along with the corresponding
        /// configuration.
        #[arg(long)]
        generate_stubs: bool,
        /// Skip all fields which have a type assigned in the configuration file.
        #[arg(long)]
        skip_existing: bool,
    },
}

#[derive(Clone, Debug, Parser)]
struct Args {
    /// Path to config TOML file.
    config_toml: PathBuf,
    /// Path to JSON system registers file.
    registers_json: PathBuf,
    /// Include all registers from the JSON file, not just those in the config file.
    #[arg(long)]
    all: bool,
    /// Operation to execute.
    #[command(subcommand)]
    command: Command,
}

/// Returns a value with the given number of 1 bits, starting at the least significant bit.
const fn ones(n: u32) -> u64 {
    u64::MAX >> (64 - n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_args() {
        Args::command().debug_assert();
    }

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
                    type_name: None,
                    values: None,
                },
                RegisterField {
                    name: "FOO".to_string(),
                    description: None,
                    index: 1,
                    width: 1,
                    writable: false,
                    array_info: None,
                    type_name: None,
                    values: None,
                },
                RegisterField {
                    name: "BAR".to_string(),
                    description: None,
                    index: 0,
                    width: 1,
                    writable: false,
                    array_info: None,
                    type_name: None,
                    values: None,
                },
                RegisterField {
                    name: "BAZ".to_string(),
                    description: None,
                    index: 0,
                    width: 1,
                    writable: false,
                    array_info: None,
                    type_name: None,
                    values: None,
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
                        type_name: None,
                        values: None,
                    },
                    RegisterField {
                        name: "BAZ".to_string(),
                        description: None,
                        index: 0,
                        width: 1,
                        writable: false,
                        array_info: None,
                        type_name: None,
                        values: None,
                    },
                ],
                ..Default::default()
            },
        );
    }
}
