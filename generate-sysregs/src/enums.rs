// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    collections::{HashMap, hash_map::Entry},
    hash::Hash,
};

use arm_sysregs_json::{ValueEntry, Values};
use log::warn;

use crate::{RegisterField, RegisterInfo};

/// Entry of an [`Enum`], collected from the specification.
#[derive(Debug, Hash, PartialEq, Eq)]
struct Variant {
    /// Description of the variant, if any.
    pub description: Option<String>,
    /// Name of the variant, if any.
    pub name: Option<String>,
    /// Value of the variant as a string, as given in the specification. Radix is 2.
    pub value: String,
    /// Value of the variant parsed as an integer.
    pub value_parsed: i64,
}

/// Candidate for generating a rust `enum`.
#[derive(Debug, Hash, PartialEq, Eq)]
struct Enum {
    /// The possible variants of the enum.
    pub variants: Vec<Variant>,
}

type State<'a> = HashMap<Enum, Vec<(&'a RegisterInfo, &'a RegisterField)>>;

fn register_enum<'a>(state: &mut State<'a>, reg: &'a RegisterInfo, field: &'a RegisterField) {
    let Some(Values { values }) = &field.values else {
        return;
    };
    if values.is_empty() {
        return;
    }

    let mut valid_values = values
        .iter()
        .filter_map(|v| {
            let (meaning, name, value) = match v {
                ValueEntry::NamedValue(v) => (v.meaning.clone(), Some(v.name.clone()), &v.value),
                ValueEntry::Value(v) => (v.meaning.clone(), None, &v.value),
                _ => return None,
            };

            let Some(value) = value.strip_prefix("'").and_then(|v| v.strip_suffix("'")) else {
                warn!("Invalid value literal: {value}");
                return None;
            };

            Some(Variant {
                description: meaning,
                name,
                value: value.to_owned(),
                value_parsed: i64::from_str_radix(value, 2)
                    .inspect_err(|e| warn!("Invalid value literal: {value} ({e:?}"))
                    .ok()?,
            })
        })
        .collect::<Vec<_>>();
    if valid_values.len() != values.len() {
        return;
    }

    valid_values.sort_by_key(|c| c.value_parsed);

    if field.width == 1
        && valid_values.len() == 2
        && valid_values[0].value_parsed == 0
        && valid_values[1].value_parsed == 1
    {
        return;
    }

    state
        .entry(Enum {
            variants: valid_values,
        })
        .or_default()
        .push((reg, field));
}

fn generate_enum_stub(enum_: &Enum, usage: &[(&RegisterInfo, &RegisterField)]) {
    println!("Stub implementation:");

    let (_, field) = usage[0];

    // Determine the smallest u-type that can hold all the variants for this enum.
    let repr = enum_
        .variants
        .iter()
        .map(|v| v.value_parsed)
        .max()
        .map(|max| {
            (max as usize)
                .next_power_of_two()
                .trailing_zeros()
                .next_power_of_two()
        })
        .unwrap_or(64)
        .max(8);
    if repr > 64 {
        println!("/!\\ This enum is too big to fit in a u64 /!\\");
    }

    println!("#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]");
    println!("#[repr(u{repr})]");
    println!("pub enum {} {{", field.name);

    for variant in &enum_.variants {
        if let Some(meaning) = &variant.description {
            println!("   /// {}", meaning);
        }

        print!("    ");
        if let Some(name) = &variant.name {
            print!("{name}")
        } else {
            print!("Value{}", variant.value);
        }
        println!(" = {:#b},", variant.value_parsed);
    }
    println!("}}")
}

fn generate_config_stub(usage: &[(&RegisterInfo, &RegisterField)]) {
    println!("Config entries:");

    let grouped = usage.iter().fold(HashMap::new(), |mut acc, (reg, field)| {
        match acc.entry(reg.name.clone()) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(vec![&field.name]);
            }
            Entry::Occupied(mut occupied_entry) => occupied_entry.get_mut().push(&field.name),
        }
        acc
    });

    for (reg_name, fields) in grouped {
        println!("[registers.{reg_name}.types]");
        for field_name in fields {
            // We could use the same enum_name as for the implementation stub, but as will likely be
            // changed, using `<enum_name>` makes it easy to search and replace without clashing
            // with a field name.
            println!("{field_name} = \"<enum_name>\"");
        }
        println!()
    }
}

pub fn identify_enums(registers: &[RegisterInfo], generate_stubs: bool, skip_existing: bool) {
    let mut state = Default::default();

    for r in registers {
        for f in &r.fields {
            register_enum(&mut state, r, f);
        }
    }

    for (enum_, usage) in state {
        print!(
            "Suitable candidate with {} variants found for {} fields: ",
            enum_.variants.len(),
            usage.len()
        );

        if skip_existing {
            if usage.iter().all(|(_, f)| f.type_name.is_some()) {
                println!("Skipped (already implemented)");
                println!();
                continue;
            } else {
                let mut impls = usage
                    .iter()
                    .filter_map(|(_, f)| f.type_name.clone())
                    .collect::<Vec<_>>();
                impls.sort();
                impls.dedup();

                if !impls.is_empty() {
                    println!("Partial implementations with [{}]", impls.join(", "))
                } else {
                    println!()
                }
            }
        }

        for (reg, field) in &usage {
            println!(" - {}.{}", reg.name, field.name);
        }

        println!();

        if generate_stubs {
            generate_enum_stub(&enum_, &usage);
            println!();
            generate_config_stub(&usage);

            println!("------------------------");
            println!();
        }
    }
}
