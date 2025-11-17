// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Logic for converting from a parsed JSON system register `RegisterEntry` to the `RegisterInfo`
//! intermediate representation.

use crate::{ArrayInfo, RegisterField, RegisterInfo, Safety, ones};
use arm_sysregs_json::{
    ArrayField, ConditionalField, ConstantField, DynamicField, Field, FieldEntry, Register,
    RegisterEntry,
};
use log::{info, trace};

/// Converts all the `registers` with names contained in `filter` to `RegisterInfo`s.
pub fn register_entries_to_register_infos(
    registers: &[RegisterEntry],
    filter: &[&String],
) -> Vec<RegisterInfo> {
    registers
        .into_iter()
        .filter_map(|register| match register {
            RegisterEntry::Register(register) => {
                if filter
                    .iter()
                    .any(|filter_entry| register.name == **filter_entry)
                {
                    Some(RegisterInfo::from_json_register(register))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

impl RegisterInfo {
    fn from_json_register(register: &Register) -> RegisterInfo {
        trace!("{:#?}", register);
        let mut fields = Vec::new();
        let mut res1 = 0;
        for fieldset in &register.fieldsets {
            for field_entry in &fieldset.values {
                fields.extend(RegisterField::from_field_entry(field_entry, 0));
                if let FieldEntry::Reserved(field) = field_entry
                    && field.value == "RES1"
                {
                    for range in &field.rangeset {
                        res1 |= ones(range.width) << range.start
                    }
                }
            }
        }
        fields.sort_by_key(|field| field.index);
        fields.dedup();
        // TODO: Find a better way to detect register width, including 32-bit registers.
        let width = if fields.iter().any(|field| field.index + field.width > 64) {
            128
        } else {
            64
        };
        let writable = fields.iter().any(|field| field.writable);
        RegisterInfo {
            name: register.name.clone(),
            width,
            fields,
            res1,
            read: Some(Safety::Safe),
            write: if writable { Some(Safety::Unsafe) } else { None },
            write_safety_doc: None,
            derive_debug: true,
        }
    }
}

impl RegisterField {
    fn from_field_entry(field_entry: &FieldEntry, offset: u32) -> Option<Self> {
        match field_entry {
            FieldEntry::Field(field) => {
                trace!("  Field: {:?} {:?}", field.name, field.rangeset);
                Self::from_field(field, offset)
            }
            FieldEntry::Reserved(field) => {
                trace!("  Reserved field: {:?}", field.rangeset);
                None
            }
            FieldEntry::ImplementationDefined(implementation_defined_field) => {
                info!("Skipping implementation defined field {implementation_defined_field:?}");
                None
            }
            FieldEntry::ConditionalField(field) => {
                trace!("  Conditional field: {:?} {:?}", field.name, field.rangeset);
                Self::from_conditional_field(field, offset)
            }
            FieldEntry::Array(field) => {
                info!(
                    "  Array field: {:?} {:?}, {}, {:?}",
                    field.name, field.rangeset, field.index_variable, field.indexes
                );
                Self::from_array_field(field, offset)
            }
            FieldEntry::ConstantField(constant_field) => {
                trace!(
                    "  Constant field: {:?} {:?}",
                    constant_field.name, constant_field.rangeset
                );
                Self::from_constant_field(constant_field, offset)
            }
            FieldEntry::Dynamic(field) => {
                trace!("  Dynamic field {:?} {:?}", field.name, field.rangeset);
                Self::from_dynamic_field(field, offset)
            }
            FieldEntry::Vector(_vector_field) => todo!(),
        }
    }

    fn from_conditional_field(field: &ConditionalField, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let mut bit = None;
            for field in &field.fields {
                if bit.is_none() {
                    bit = Self::from_field_entry(&field.field, offset + range.start);
                } else if Self::from_field_entry(&field.field, offset + range.start) != bit {
                    // If different options give a different RegisterField, ignore them all to be
                    // safe.
                    return None;
                }
            }
            bit
        } else {
            info!(
                "Skipping conditional field with multiple ranges {:?}",
                field.rangeset
            );
            None
        }
    }

    fn from_field(field: &Field, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let name = field.name.clone().unwrap();
            Some(RegisterField {
                name,
                description: None,
                index: offset + range.start,
                width: range.width,
                writable: true,
                array_info: None,
            })
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }

    fn from_array_field(field: &ArrayField, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            if let [array_range] = field.indexes.as_slice() {
                let name = field.name.clone().unwrap();
                Some(RegisterField {
                    name,
                    description: None,
                    index: offset + range.start,
                    width: range.width / array_range.width,
                    writable: true,
                    array_info: Some(ArrayInfo {
                        indices: array_range.start..array_range.start + array_range.width,
                        index_variable: field.index_variable.clone(),
                    }),
                })
            } else {
                info!(
                    "Skipping field with multiple array indices {:?}",
                    field.rangeset
                );
                None
            }
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }

    fn from_constant_field(field: &ConstantField, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let name = field.name.clone().unwrap();
            Some(RegisterField {
                name,
                description: None,
                index: offset + range.start,
                width: range.width,
                writable: false,
                array_info: None,
            })
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }

    fn from_dynamic_field(field: &DynamicField, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let name = field.name.clone().unwrap();
            Some(RegisterField {
                name,
                description: None,
                index: offset + range.start,
                width: range.width,
                writable: true,
                array_info: None,
            })
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }
}
