// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Logic for writing out a Rust source file with system register types and accessors.

use crate::{ExceptionLevel, RegisterField, RegisterInfo, Safety, ones};
use std::io::{self, Write};

pub fn write_lib(mut writer: impl Write + Copy, registers: &[RegisterInfo]) -> io::Result<()> {
    writer.write_all(
        "\
// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Access to Arm CPU system registers.

#![cfg_attr(not(any(test, feature = \"fakes\")), no_std)]

#[cfg(not(any(test, feature = \"fakes\")))]
mod aarch64;
#[cfg(any(test, feature = \"fakes\"))]
pub mod fake;
mod macros;
mod manual;

use bitflags::bitflags;
pub use manual::*;
#[doc(hidden)]
pub use paste as _paste;
"
        .as_bytes(),
    )?;

    for register in registers {
        if register.use_struct() {
            writeln!(writer)?;
            register.write_bitflags(writer)?;
            register.write_impl(writer)?;
        }
    }
    writeln!(writer)?;
    for register in registers {
        register.write_accessor(writer)?;
    }

    Ok(())
}

pub fn write_fake(mut writer: impl Write + Copy, registers: &[RegisterInfo]) -> io::Result<()> {
    writer.write_all(
        "\
// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

"
        .as_bytes(),
    )?;

    for exception_level in [
        ExceptionLevel::El0,
        ExceptionLevel::El1,
        ExceptionLevel::El2,
        ExceptionLevel::El3,
    ] {
        let struct_names = registers
            .iter()
            .filter(|register| register.use_struct() && register.exception_level == exception_level)
            .map(RegisterInfo::struct_name)
            .collect::<Vec<_>>()
            .join(", ");
        if let Some(guard) = exception_level.cfg_guard() {
            writeln!(writer, "{guard}")?;
        }
        writeln!(writer, "use crate::{{{struct_names}}};")?;
    }

    writer.write_all(
        "
/// A set of fake system registers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemRegisters {
"
        .as_bytes(),
    )?;

    for register in registers {
        if let Some(guard) = register.exception_level.cfg_guard() {
            writeln!(writer, "    {guard}")?;
        }
        writeln!(
            writer,
            "    /// Fake value for the `{}` system register.",
            register.name
        )?;
        let register_type = if register.use_struct() {
            register.struct_name()
        } else {
            format!("u{}", register.width)
        };
        writeln!(
            writer,
            "    pub {}: {},",
            register.variable_name(),
            register_type
        )?;
    }
    writeln!(writer, "}}")?;
    writeln!(writer)?;
    writeln!(writer, "impl SystemRegisters {{")?;
    writeln!(writer, "    pub(crate) const fn new() -> Self {{")?;
    writeln!(writer, "        Self {{")?;
    for register in registers {
        if let Some(guard) = register.exception_level.cfg_guard() {
            writeln!(writer, "            {guard}")?;
        }
        if register.use_struct() {
            writeln!(
                writer,
                "            {}: {}::empty(),",
                register.variable_name(),
                register.struct_name(),
            )?;
        } else {
            writeln!(writer, "            {}: 0,", register.variable_name())?;
        }
    }
    writeln!(writer, "        }}")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}")?;

    Ok(())
}

impl RegisterInfo {
    /// Whether to use a wrapper bitflags struct type for the register, rather than just a raw
    /// primitive type.
    fn use_struct(&self) -> bool {
        !self.fields.is_empty()
    }

    /// The name to use for the struct type for the register.
    fn struct_name(&self) -> String {
        camel_case(&self.name)
    }

    fn write_bitflags(&self, mut writer: impl Write) -> io::Result<()> {
        if let Some(guard) = self.exception_level.cfg_guard() {
            writeln!(writer, "{guard}")?;
        }
        writeln!(writer, "bitflags! {{")?;
        writeln!(writer, "    /// `{}` system register value.", self.name)?;
        if let Some(description) = &self.description {
            writeln!(writer, "    ///")?;
            writeln!(writer, "    /// {description}")?;
        }
        if self.derive_debug {
            writeln!(writer, "    #[derive(Clone, Copy, Debug, Eq, PartialEq)]")?;
        } else {
            writeln!(writer, "    #[derive(Clone, Copy, Eq, PartialEq)]")?;
        }
        writeln!(writer, "    #[repr(transparent)]")?;
        writeln!(
            writer,
            "    pub struct {}: u{} {{",
            self.struct_name(),
            self.width
        )?;
        if self.res1 != 0 {
            writeln!(
                writer,
                "        /// RES1 bits in the `{}` register.",
                self.name
            )?;
            writeln!(writer, "        const RES1 = {:#b};", self.res1)?;
        }
        for field in &self.fields {
            if field.width == 1 {
                if let Some(array_info) = &field.array_info {
                    let placeholder = array_info.placeholder();
                    for i in array_info.indices.clone() {
                        writeln!(writer, "        /// `{}` bit {}.", field.name, i)?;
                        if let Some(description) = &field.description {
                            writeln!(writer, "        ///")?;
                            writeln!(writer, "        /// {description}")?;
                        }
                        writeln!(
                            writer,
                            "        const {} = 1 << {};",
                            uppercase_name(&field.name.replace(&placeholder, &format!("{}", i))),
                            field.index + i - array_info.indices.start,
                        )?;
                    }
                } else {
                    if let Some(description) = &field.description {
                        writeln!(writer, "        /// {description}")?;
                    } else {
                        writeln!(writer, "        /// `{}` bit.", field.name)?;
                    }
                    writeln!(
                        writer,
                        "        const {} = 1 << {};",
                        field.constant_name(),
                        field.index,
                    )?;
                }
            }
        }
        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn write_impl(&self, mut writer: impl Write) -> io::Result<()> {
        let mut first = true;
        for field in &self.fields {
            if field.width <= 1 {
                continue;
            }

            writeln!(writer)?;

            if first {
                if let Some(guard) = self.exception_level.cfg_guard() {
                    writeln!(writer, "{guard}")?;
                }
                writeln!(writer, "impl {} {{", self.struct_name())?;
                first = false;
            }

            let field_type = type_for_width(field.width);

            if let Some(array_info) = &field.array_info {
                writeln!(
                    writer,
                    "    /// Returns the value of the given `{}` field.",
                    field.name,
                )?;
                if let Some(description) = &field.description {
                    writeln!(writer, "    ///")?;
                    writeln!(writer, "    /// {description}")?;
                }
                writeln!(
                    writer,
                    "    pub const fn {}(self, {}: u32) -> {} {{",
                    field.function_name().replace(&array_info.placeholder(), ""),
                    array_info.index_variable,
                    field_type,
                )?;
                if array_info.indices.start > 0 {
                    writeln!(
                        writer,
                        "        assert!({} >= {} && {} < {});",
                        array_info.index_variable,
                        array_info.indices.start,
                        array_info.index_variable,
                        array_info.indices.end,
                    )?;
                } else {
                    writeln!(
                        writer,
                        "        assert!({} < {});",
                        array_info.index_variable, array_info.indices.end,
                    )?;
                }
                writeln!(
                    writer,
                    "        (self.bits() >> ({} + ({} - {}) * {})) as {} & {:#b}",
                    field.index,
                    array_info.index_variable,
                    array_info.indices.start,
                    field.width,
                    field_type,
                    ones(field.width),
                )?;
                writeln!(writer, "    }}")?;
            } else {
                writeln!(
                    writer,
                    "    /// Returns the value of the `{}` field.",
                    field.name
                )?;
                if let Some(description) = &field.description {
                    writeln!(writer, "    ///")?;
                    writeln!(writer, "    /// {description}")?;
                }
                writeln!(
                    writer,
                    "    pub const fn {}(self) -> {} {{",
                    field.function_name(),
                    field_type
                )?;
                writeln!(
                    writer,
                    "        (self.bits() >> {}) as {} & {:#b}",
                    field.index,
                    field_type,
                    ones(field.width),
                )?;
                writeln!(writer, "    }}")?;
            }
        }
        if !first {
            writeln!(writer, "}}")?;
        }
        Ok(())
    }

    fn write_accessor(&self, mut writer: impl Write) -> io::Result<()> {
        if let Some(guard) = self.exception_level.cfg_guard() {
            writeln!(writer, "{guard}")?;
        }
        let register_type = if self.use_struct() {
            format!("u{}: {}", self.width, self.struct_name())
        } else {
            format!("u{}", self.width)
        };
        let register_assembly_name = self
            .assembly_name
            .as_ref()
            .map(|register_name| format!(": {register_name}"))
            .unwrap_or_default();
        match (self.read, self.write) {
            (None, None) => {}
            (None, Some(write_safety)) => {
                let safe_write = match write_safety {
                    Safety::Safe => ", safe",
                    Safety::Unsafe => "",
                };
                if let Some(safety_doc) = &self.write_safety_doc {
                    writeln!(
                        writer,
                        "\
write_sysreg! {{
    /// # Safety
    ///
    /// {}
    {}{}, {}{}, fake::SYSREGS
}}",
                        safety_doc,
                        self.variable_name(),
                        register_assembly_name,
                        register_type,
                        safe_write,
                    )?;
                } else {
                    writeln!(
                        writer,
                        "write_sysreg!({}{}, {}{}, fake::SYSREGS);",
                        self.variable_name(),
                        register_assembly_name,
                        register_type,
                        safe_write,
                    )?;
                }
            }
            (Some(read_safety), None) => {
                let safe_read = match read_safety {
                    Safety::Safe => ", safe",
                    Safety::Unsafe => "",
                };
                writeln!(
                    writer,
                    "read_sysreg!({}{}, {}{}, fake::SYSREGS);",
                    self.variable_name(),
                    register_assembly_name,
                    register_type,
                    safe_read,
                )?;
            }
            (Some(read_safety), Some(write_safety)) => {
                let safe_read = match read_safety {
                    Safety::Safe => ", safe_read",
                    Safety::Unsafe => "",
                };
                let safe_write = match write_safety {
                    Safety::Safe => ", safe_write",
                    Safety::Unsafe => "",
                };
                if let Some(safety_doc) = &self.write_safety_doc {
                    writeln!(
                        writer,
                        "\
read_write_sysreg! {{
    /// # Safety
    ///
    /// {}
    {}{}, {}{}{}, fake::SYSREGS
}}",
                        safety_doc,
                        self.variable_name(),
                        register_assembly_name,
                        register_type,
                        safe_read,
                        safe_write,
                    )?;
                } else {
                    writeln!(
                        writer,
                        "read_write_sysreg!({}{}, {}{}{}, fake::SYSREGS);",
                        self.variable_name(),
                        register_assembly_name,
                        register_type,
                        safe_read,
                        safe_write,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Returns the name of the field formatted to be a valid Rust variable name.
    fn variable_name(&self) -> String {
        lowercase_name(&self.name)
    }
}

impl RegisterField {
    /// Returns the name of the field formatted to be a valid Rust constant name.
    fn constant_name(&self) -> String {
        uppercase_name(&self.name)
    }

    /// Returns the name of the field formatted to be a valid Rust function name.
    fn function_name(&self) -> String {
        lowercase_name(&self.name)
    }
}

impl ExceptionLevel {
    fn cfg_guard(self) -> Option<&'static str> {
        match self {
            ExceptionLevel::El0 => None,
            ExceptionLevel::El1 => Some("#[cfg(feature = \"el1\")]"),
            ExceptionLevel::El2 => Some("#[cfg(feature = \"el2\")]"),
            ExceptionLevel::El3 => Some("#[cfg(feature = \"el3\")]"),
        }
    }
}

fn camel_case(name: &str) -> String {
    name.split('_')
        .flat_map(|part| [part[0..1].to_uppercase(), part[1..].to_lowercase()])
        .collect()
}

fn lowercase_name(name: &str) -> String {
    name.replace([':', '['], "_")
        .replace(']', "")
        .to_lowercase()
}

fn uppercase_name(name: &str) -> String {
    name.replace([':', '['], "_")
        .replace(']', "")
        .to_uppercase()
}

/// Returns the smallest unsigned type that can hold at least the given number of bits
fn type_for_width(width: u32) -> &'static str {
    if width > 32 {
        "u64"
    } else if width > 16 {
        "u32"
    } else if width > 8 {
        "u16"
    } else {
        "u8"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case("SCR_EL3"), "ScrEl3");
        assert_eq!(camel_case("aBc_de_FGh_3a"), "AbcDeFgh3a");
    }
}
