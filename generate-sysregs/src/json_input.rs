// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Logic for converting from a parsed JSON system register `RegisterEntry` to the `RegisterInfo`
//! intermediate representation.

use crate::{ArrayInfo, RegisterField, RegisterInfo, Safety, ones};
use arm_sysregs_json::{
    Accessor, ArrayField, ConditionalField, ConstantField, DynamicField, Field, FieldEntry,
    Register, RegisterEntry, VectorField,
};
use log::{info, trace};

/// Converts all the `registers` with names contained in `filter` to `RegisterInfo`s.
pub fn register_entries_to_register_infos(
    registers: &[RegisterEntry],
    filter: &[&String],
) -> Vec<RegisterInfo> {
    registers
        .iter()
        .filter_map(|register| match register {
            RegisterEntry::Register(register) => filter
                .iter()
                .any(|filter_entry| register.name == **filter_entry)
                .then(|| RegisterInfo::from_json_register(register)),
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
        let mut writable = false;
        let mut readable = false;
        let mut width = 0;
        for accessor in &register.accessors {
            match accessor {
                Accessor::SystemAccessor(system_accessor) => match system_accessor.name.as_str() {
                    "A32.MRC" => {
                        readable = true;
                        width = 32;
                    }
                    "A32.MCR" => {
                        writable = true;
                        width = 32;
                    }
                    "A64.MRS" => {
                        readable = true;
                        width = 64;
                    }
                    "A64.MSRregister" => {
                        writable = true;
                        width = 64;
                    }
                    "A64.MRRS" => {
                        readable = true;
                        width = 128;
                    }
                    "A64.MSRRregister" => {
                        writable = true;
                        width = 128;
                    }
                    other_name => {
                        log::info!("Unexpected system accessor name {other_name}.");
                    }
                },
                _ => {}
            }
        }
        RegisterInfo {
            name: register.name.clone(),
            description: None,
            width,
            fields,
            res1,
            read: readable.then_some(Safety::Safe),
            write: writable.then_some(Safety::Unsafe),
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
            FieldEntry::ConstantField(field) => {
                trace!("  Constant field: {:?} {:?}", field.name, field.rangeset);
                Self::from_constant_field(field, offset)
            }
            FieldEntry::Dynamic(field) => {
                trace!("  Dynamic field {:?} {:?}", field.name, field.rangeset);
                Self::from_dynamic_field(field, offset)
            }
            FieldEntry::Vector(field) => {
                trace!("  Vector field {:?} {:?}", field.name, field.rangeset);
                Self::from_vector_field(field, offset)
            }
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
            let name = field.name.clone()?;
            Some(RegisterField {
                name,
                description: None,
                index: offset + range.start,
                width: range.width,
                writable: true,
                array_info: None,
                r#type: None,
            })
        } else {
            info!(
                "Skipping field {:?} with multiple ranges {:?}",
                field.name, field.rangeset
            );
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
                    r#type: None,
                })
            } else {
                info!(
                    "Skipping field with multiple array indices {:?}",
                    field.rangeset
                );
                None
            }
        } else {
            info!(
                "Skipping field {:?} with multiple ranges {:?}",
                field.name, field.rangeset
            );
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
                r#type: None,
            })
        } else {
            info!(
                "Skipping field {:?} with multiple ranges {:?}",
                field.name, field.rangeset
            );
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
                r#type: None,
            })
        } else {
            info!(
                "Skipping field {:?} with multiple ranges {:?}",
                field.name, field.rangeset
            );
            None
        }
    }

    fn from_vector_field(field: &VectorField, offset: u32) -> Option<Self> {
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
                    r#type: None,
                })
            } else {
                info!(
                    "Skipping field {:?} with multiple array indices {:?}",
                    field.name, field.rangeset
                );
                None
            }
        } else {
            info!(
                "Skipping field {:?} with multiple ranges {:?}",
                field.name, field.rangeset
            );
            None
        }
    }
}
