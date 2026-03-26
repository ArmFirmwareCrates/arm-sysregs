// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Logic for converting from a parsed JSON system register `RegisterEntry` to the `RegisterInfo`
//! intermediate representation.

mod conditions;

use crate::{
    AArch32Encoding, ArrayInfo, ExceptionLevel, RegisterField, RegisterInfo, Safety,
    json_input::conditions::{Environment, EvalValue, Evaluable},
    ones,
};
use arm_sysregs_json::{
    Accessor, ArrayField, AstBinaryOp, AstBool, AstFunction, AstIdentifier, ConditionalField,
    ConstantField, DynamicField, Encoding, Expression, Field, FieldEntry, Fieldset, Register,
    RegisterArray, RegisterEntry, ValueEntry, Values, VectorField,
};
use eyre::{Report, bail, eyre};
use log::{info, trace};
use regex::Regex;
use std::{collections::BTreeMap, fmt::Write, sync::LazyLock};

static STANDARD_CONDITIONS: LazyLock<Vec<Expression>> = LazyLock::new(|| {
    vec![
        Expression::Bool(AstBool { value: true }),
        Expression::Function(AstFunction {
            arguments: vec![Expression::Identifier(AstIdentifier {
                value: "FEAT_AA64".to_string(),
            })],
            name: "IsFeatureImplemented".into(),
            parameters: vec![],
        }),
        Expression::BinaryOp(AstBinaryOp {
            op: "&&".to_string(),
            left: Box::new(Expression::Function(AstFunction {
                arguments: vec![Expression::Identifier(AstIdentifier {
                    value: "FEAT_VHE".to_string(),
                })],
                name: "IsFeatureImplemented".into(),
                parameters: vec![],
            })),
            right: Box::new(Expression::Function(AstFunction {
                arguments: vec![Expression::Identifier(AstIdentifier {
                    value: "FEAT_AA64".to_string(),
                })],
                name: "IsFeatureImplemented".into(),
                parameters: vec![],
            })),
        }),
        Expression::BinaryOp(AstBinaryOp {
            op: "&&".to_string(),
            left: Box::new(Expression::Function(AstFunction {
                arguments: vec![Expression::Identifier(AstIdentifier {
                    value: "EL3".to_string(),
                })],
                name: "HaveEL".into(),
                parameters: vec![],
            })),
            right: Box::new(Expression::Function(AstFunction {
                arguments: vec![Expression::Identifier(AstIdentifier {
                    value: "FEAT_AA64".to_string(),
                })],
                name: "IsFeatureImplemented".into(),
                parameters: vec![],
            })),
        }),
    ]
});

/// Converts all the `registers` with names contained in `filter` to `RegisterInfo`s.
///
/// If `filter` is `None` then all registers are included.
pub fn register_entries_to_register_infos(
    registers: &[RegisterEntry],
    filter: Option<&[&String]>,
) -> Vec<RegisterInfo> {
    registers
        .iter()
        .flat_map(|register| match register {
            RegisterEntry::RegisterArray(register_array) => {
                if filter
                    .map(|filter| {
                        filter
                            .iter()
                            .any(|filter_entry| register_array.name == **filter_entry)
                    })
                    .unwrap_or(true)
                {
                    RegisterInfo::from_json_register_array(register_array)
                } else {
                    Vec::new()
                }
            }
            RegisterEntry::Register(register) => {
                if filter
                    .map(|filter| {
                        filter
                            .iter()
                            .any(|filter_entry| register.name == **filter_entry)
                    })
                    .unwrap_or(true)
                {
                    vec![RegisterInfo::from_json_register(register)]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        })
        .collect()
}

impl RegisterInfo {
    fn from_json_register_array(register: &RegisterArray) -> Vec<RegisterInfo> {
        if !STANDARD_CONDITIONS.contains(&register.condition) {
            trace!("condition for {}: {:#?}", register.name, register.condition);
        }
        let exception_level = get_exception_level(&register.name);

        register
            .indexes
            .iter()
            .flat_map(|range| {
                (range.start..range.start + range.width).map(|i| {
                    let AccessorDetails {
                        readable,
                        writable,
                        aarch32,
                        aarch64,
                        width,
                        assembly_name,
                        aarch32_encoding,
                    } = AccessorDetails::from_json_accessors(&register.accessors, Some(i));

                    let (fields, res1) =
                        convert_fields(&register.fieldsets, Some((&register.index_variable, i)));
                    let name = register
                        .name
                        .replace(&format!("<{}>", register.index_variable), &format!("{i}"));
                    RegisterInfo {
                        name,
                        original_name: register.name.clone(),
                        description: None,
                        width,
                        aarch32,
                        aarch64,
                        fields,
                        res1,
                        read: readable.then_some(Safety::Safe),
                        write: writable.then_some(Safety::Unsafe),
                        write_safety_doc: None,
                        derive_debug: true,
                        assembly_name,
                        aarch32_encoding,
                        has_special_conditions: !STANDARD_CONDITIONS.contains(&register.condition),
                        exception_level,
                    }
                })
            })
            .collect()
    }

    fn from_json_register(register: &Register) -> RegisterInfo {
        if !STANDARD_CONDITIONS.contains(&register.condition) {
            trace!("condition for {}: {:#?}", register.name, register.condition);
        }
        let (fields, res1) = convert_fields(&register.fieldsets, None);
        let exception_level = get_exception_level(&register.name);
        let AccessorDetails {
            readable,
            writable,
            aarch32,
            aarch64,
            width,
            assembly_name,
            aarch32_encoding,
        } = AccessorDetails::from_json_accessors(&register.accessors, None);

        RegisterInfo {
            name: register.name.clone(),
            original_name: register.name.clone(),
            description: None,
            width,
            aarch32,
            aarch64,
            fields,
            res1,
            read: readable.then_some(Safety::Safe),
            write: writable.then_some(Safety::Unsafe),
            write_safety_doc: None,
            derive_debug: true,
            assembly_name,
            aarch32_encoding,
            has_special_conditions: !STANDARD_CONDITIONS.contains(&register.condition),
            exception_level,
        }
    }
}

fn convert_fields(
    fieldsets: &[Fieldset],
    index_value: Option<(&str, u32)>,
) -> (Vec<RegisterField>, u64) {
    let mut fields = Vec::new();
    let mut res1 = 0;
    for fieldset in fieldsets {
        for field_entry in &fieldset.values {
            fields.extend(RegisterField::from_field_entry(field_entry, 0, index_value));
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

    (fields, res1)
}

fn get_exception_level(name: &str) -> ExceptionLevel {
    if name.ends_with("_EL3") || name.ends_with("_mon") {
        ExceptionLevel::El3
    } else if name.ends_with("_EL2") || name.ends_with("_hyp") {
        ExceptionLevel::El2
    } else if name.ends_with("_EL1") || name.ends_with("_svc") {
        ExceptionLevel::El1
    } else if name.ends_with("_EL0") {
        ExceptionLevel::El0
    } else {
        info!("Assuming {} is available to EL0.", name);
        ExceptionLevel::El0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct AccessorDetails {
    readable: bool,
    writable: bool,
    aarch32: bool,
    aarch64: bool,
    width: u32,
    assembly_name: Option<String>,
    aarch32_encoding: Option<AArch32Encoding>,
}

impl AccessorDetails {
    fn from_json_accessors(accessors: &[Accessor], index: Option<u32>) -> Self {
        let mut details = Self::default();
        for accessor in accessors {
            match accessor {
                Accessor::SystemAccessorArray(system_accessor_array) => {
                    let mut values = BTreeMap::new();
                    if let Some(index) = index {
                        values.insert(system_accessor_array.index_variable.clone(), index);
                    }
                    details.add_from_name(
                        &system_accessor_array.name,
                        &system_accessor_array.encoding[0],
                        &values,
                    );
                }
                Accessor::SystemAccessor(system_accessor) => {
                    details.add_from_name(
                        &system_accessor.name,
                        &system_accessor.encoding[0],
                        &BTreeMap::new(),
                    );
                }
                _ => {}
            }
        }

        details
    }

    fn add_from_name(&mut self, name: &str, encoding: &Encoding, values: &BTreeMap<String, u32>) {
        match name {
            "A32.MRC" => {
                self.aarch32 = true;
                self.aarch32_encoding = AArch32Encoding::single_from_encoding(&encoding, values);
                self.readable = true;
                self.width = 32;
            }
            "A32.MCR" => {
                self.aarch32 = true;
                self.aarch32_encoding = AArch32Encoding::single_from_encoding(&encoding, values);
                self.writable = true;
                self.width = 32;
            }
            "A32.MRRC" => {
                self.aarch32 = true;
                self.aarch32_encoding = AArch32Encoding::double_from_encoding(&encoding, values);
                self.readable = true;
                self.width = 64;
            }
            "A32.MCRR" => {
                self.aarch32 = true;
                self.aarch32_encoding = AArch32Encoding::double_from_encoding(&encoding, values);
                self.writable = true;
                self.width = 64;
            }
            "A32.MRSbanked" => {
                self.aarch32 = true;
                self.readable = true;
                self.width = 32;
            }
            "A32.MSRbanked" => {
                self.aarch32 = true;
                self.writable = true;
                self.width = 32;
            }
            "A64.MRS" => {
                self.aarch64 = true;
                self.readable = true;
                self.width = 64;
            }
            "A64.MSRregister" => {
                self.aarch64 = true;
                self.writable = true;
                self.width = 64;
            }
            "A64.MRRS" => {
                self.aarch64 = true;
                self.readable = true;
                self.width = 128;
            }
            "A64.MSRRregister" => {
                self.aarch64 = true;
                self.writable = true;
                self.width = 128;
            }
            other_name => {
                log::info!("Unexpected system accessor name {other_name}.");
            }
        }

        if self.assembly_name.is_none() {
            self.assembly_name = encoding_to_assembly_name(&encoding, values);
        }
    }
}

fn parse_binary_value(
    value_entry: &ValueEntry,
    values: &BTreeMap<String, u32>,
) -> Result<u8, Report> {
    let value_pattern = Regex::new("^'([01]+)'(?::|$)").unwrap();
    let index_pattern = Regex::new("^([a-z])\\[([0-9]+)\\](?::|$)").unwrap();
    let slice_pattern = Regex::new("^([a-z])\\[([0-9]+):([0-9]+)\\](?::|$)").unwrap();

    match value_entry {
        ValueEntry::Value(value) => Ok(u8::from_str_radix(value.value.trim_matches('\''), 2)?),
        ValueEntry::Group(group) => {
            let mut concatenated_value = String::new();
            let mut group_value: &str = &group.value;
            while !group_value.is_empty() {
                if let Some(captures) = value_pattern.captures(group_value) {
                    group_value = &group_value[captures.get_match().end()..];
                    concatenated_value += &captures[1];
                } else if let Some(captures) = index_pattern.captures(group_value) {
                    group_value = &group_value[captures.get_match().end()..];
                    let variable = &captures[1];
                    let bit_index = captures[2].parse::<u8>()?;
                    let value = values
                        .get(variable)
                        .ok_or_else(|| eyre!("Value {} not found", variable))?;
                    write!(concatenated_value, "{:01b}", (value >> bit_index) & 1).unwrap();
                } else if let Some(captures) = slice_pattern.captures(group_value) {
                    group_value = &group_value[captures.get_match().end()..];
                    let variable = &captures[1];
                    let high_index = captures[2].parse::<u8>()?;
                    let low_index = captures[3].parse::<u8>()?;
                    let value = values
                        .get(variable)
                        .ok_or_else(|| eyre!("Value {} not found", variable))?;
                    let width = high_index - low_index + 1;
                    let mask = (1 << width) - 1;
                    write!(
                        concatenated_value,
                        "{:0width$b}",
                        (value >> low_index) & mask,
                        width = width.into(),
                    )
                    .unwrap();
                } else {
                    bail!("Unsupported group value part {group_value:?}");
                }
            }
            Ok(u8::from_str_radix(&concatenated_value, 2)?)
        }
        ValueEntry::EquationValue(equation) => {
            let value = values
                .get(&equation.value)
                .ok_or_else(|| eyre!("Value {} not found", equation.value))?;
            let mut sliced_value = 0;
            for range in &equation.slice {
                sliced_value <<= range.width;
                let mask = (1 << range.width) - 1;
                sliced_value |= (value >> range.start) & mask;
            }
            Ok(sliced_value.try_into()?)
        }
        _ => bail!("Unsupported value entry {value_entry:?}"),
    }
}

fn encoding_to_assembly_name(
    encoding: &Encoding,
    values: &BTreeMap<String, u32>,
) -> Option<String> {
    let op0 = parse_binary_value(encoding.encodings.get("op0")?, values).expect("op0");
    let op1 = parse_binary_value(encoding.encodings.get("op1")?, values).expect("op1");
    let op2 = parse_binary_value(encoding.encodings.get("op2")?, values).expect("op2");
    let crn = parse_binary_value(encoding.encodings.get("CRn")?, values).expect("CRn");
    let crm = parse_binary_value(encoding.encodings.get("CRm")?, values).expect("CRm");
    Some(format!("s{op0}_{op1}_c{crn}_c{crm}_{op2}"))
}

impl AArch32Encoding {
    fn single_from_encoding(encoding: &Encoding, values: &BTreeMap<String, u32>) -> Option<Self> {
        let crm = parse_binary_value(encoding.encodings.get("CRm")?, values).expect("CRm");
        let crn = parse_binary_value(encoding.encodings.get("CRn")?, values).expect("CRn");
        let coproc = parse_binary_value(encoding.encodings.get("coproc")?, values).expect("coproc");
        let opc1 = parse_binary_value(encoding.encodings.get("opc1")?, values).expect("opc1");
        let opc2 = parse_binary_value(encoding.encodings.get("opc2")?, values).expect("opc2");
        Some(Self::Single {
            crm,
            crn,
            coproc,
            opc1,
            opc2,
        })
    }

    fn double_from_encoding(encoding: &Encoding, values: &BTreeMap<String, u32>) -> Option<Self> {
        let crm = parse_binary_value(encoding.encodings.get("CRm")?, values).expect("CRm");
        let coproc = parse_binary_value(encoding.encodings.get("coproc")?, values).expect("coproc");
        let opc1 = parse_binary_value(encoding.encodings.get("opc1")?, values).expect("opc1");
        Some(Self::Double { crm, coproc, opc1 })
    }
}

impl RegisterField {
    fn from_field_entry(
        field_entry: &FieldEntry,
        offset: u32,
        index_value: Option<(&str, u32)>,
    ) -> Option<Self> {
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
                Self::from_conditional_field(field, offset, index_value)
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

    fn from_conditional_field(
        field: &ConditionalField,
        offset: u32,
        index_value: Option<(&str, u32)>,
    ) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let environment = Environment {
                variables: index_value
                    .into_iter()
                    .map(|(name, value)| (name.to_owned(), EvalValue::Integer(value.into())))
                    .collect(),
            };
            let mut bit = None;
            for field in &field.fields {
                if !field.condition.eval(&environment).unwrap().could_be_true() {
                    // Skip the field.
                    continue;
                }
                if bit.is_none() {
                    bit = Self::from_field_entry(&field.field, offset + range.start, index_value);
                } else if Self::from_field_entry(&field.field, offset + range.start, index_value)
                    .map(|r| r.key())
                    != bit.as_ref().map(|r| r.key())
                {
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
                type_name: None,
                values: field.values.clone(),
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
                    type_name: None,
                    values: field.values.clone(),
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
                type_name: None,
                values: Some(Values {
                    values: vec![field.value.clone()],
                }),
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
                type_name: None,
                values: None,
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
                    type_name: None,
                    values: field.values.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use arm_sysregs_json::{EquationValue, Group, Range, Value};

    #[test]
    fn parse_value() {
        assert_eq!(
            parse_binary_value(
                &ValueEntry::Value(Value {
                    meaning: None,
                    value: "'0101'".to_string(),
                }),
                &BTreeMap::new()
            )
            .unwrap(),
            0b101
        );
    }

    #[test]
    fn parse_value_group() {
        let values = [("m".to_string(), 0b1100)].into_iter().collect();
        assert_eq!(
            parse_binary_value(
                &ValueEntry::Group(Group {
                    meaning: None,
                    value: "'10':m[2]".to_string(),
                    values: Values::default(),
                }),
                &values
            )
            .unwrap(),
            0b101
        );
        assert_eq!(
            parse_binary_value(
                &ValueEntry::Group(Group {
                    meaning: None,
                    value: "'10':m[2:1]".to_string(),
                    values: Values::default(),
                }),
                &values
            )
            .unwrap(),
            0b1010
        );
    }

    #[test]
    fn parse_value_equation() {
        let values = [("m".to_string(), 0b1100)].into_iter().collect();
        assert_eq!(
            parse_binary_value(
                &ValueEntry::EquationValue(EquationValue {
                    meaning: None,
                    value: "m".to_string(),
                    slice: vec![Range { start: 1, width: 2 }],
                }),
                &values
            )
            .unwrap(),
            0b10
        );
    }
}
