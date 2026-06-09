# generate-sysregs

`generate-sysregs` generates the contents of the `arm-sysregs` crate from Arm's
machine-readable system register descriptions.

The generator reads:

- Arm's "Features, Registers, A64, A32" JSON data.
- A local `registers.toml` configuration file that selects registers and adds
  crate-specific metadata.

It writes the generated register accessors, fake backend support, and examples
into the `arm-sysregs` crate.

## Generate `arm-sysregs`

1. Download and extract the **Features, Registers, A64, A32** machine-readable JSON specification
  from the Arm A-profile architecture [downloads page][1].
2. Run the following command from the repository root:

   ```sh
   cargo run --package generate-sysregs -- \
       registers.toml \
       /path/to/Registers.json \
       generate \
       arm-sysregs
   ```

   The arguments are:

   - `registers.toml`: the generator configuration file.
   - `/path/to/Registers.json`: the extracted Arm register description file.
   - `generate`: the subcommand that writes generated Rust files.
   - `arm-sysregs`: the output crate directory.

3. Format the generated files:

   ```sh
   cargo fmt --package arm-sysregs
   ```

4. Update `arm-sysregs/CHANGELOG.md`.

## Add a Register

1. Add a new `[registers.REGISTER_NAME]` entry to `registers.toml`. Please keep the register
   definitions in alphabetical order.
2. Add any descriptions, custom field types, or safety overrides needed for the
   generated API.
3. Regenerate and format the crate as described above.

## `registers.toml` Format

Each register is configured in its own TOML section. Register names must match the names from the
Arm JSON input. Use quoted TOML keys for register or field names that contain special characters,
such as array placeholders:

```toml
[registers.SCTLR_EL3]

[registers."AMEVCNTR0<n>"]
```

### Register Options

| Option | Description |
| --- | --- |
| `description = "..."` | Adds documentation to the generated register accessors. |
| `read = "safe"` | Generates a safe read accessor. This is the default for registers marked readable in the JSON input, so it only changes behavior when overriding a register that is not marked readable. |
| `read = "unsafe"` | Overrides the JSON input and marks reads as unsafe. Use this when reading can have side effects. |
| `read = "never"` | Disables read accessor generation. |
| `write = "safe"` | Overrides the default unsafe write accessor. Use this only when writing cannot violate Rust safety guarantees. |
| `write = "unsafe"` | Generates an unsafe write accessor. This is the default for registers marked writable in the JSON input, so it only changes behavior when overriding a register that is not marked writable. |
| `write = "never"` | Disables write accessor generation. |
| `write_safety_doc = "..."` | Adds the `# Safety` documentation for an unsafe write accessor. |
| `manual_debug = true` | Prevents `#[derive(Debug)]` when the register type has a manual `Debug` implementation. |
| `use_raw_name = true` | Keeps the raw assembly name from the JSON input instead of deriving one from the register name. |
| `disable_alias = true` | Forces an unique `bitflags!` and `impl` block to be generated for the register's type, instead of using type aliases when the register is identical to another. |

### Field Descriptions

Use a `field_descriptions` section to add documentation for generated field accessors:

```toml
[registers.CLIDR_EL1.field_descriptions]
Ctype = "Cache type."
```

### Custom Field Types

Use a `types` section to map generated field accessors to custom types:

```toml
[registers.CLIDR_EL1.types]
"Ctype<n>" = "crate::manual::CacheType"
```

Custom types must be absolute paths. They must implement `TryFrom<raw type>`, where `raw type` is
the smallest unsigned integer type that can hold the field value.

## Type aliasing
In the case of array registers (e.g. `AMEVCNTR1<n>_EL0`), and specific other registers (e.g. `PIRE0_EL1` and `POR_EL1`), the generated types might be identical.
Due to the current implementation of the rust compiler, these duplicate types contribute immensely to the size of the dependency graph at compile-time; this feature aims to reduce memory usage during compilation via generating type aliases where possible.

The generated types of two registers are considered identical if the generated Rust type would have the same public API and implementation-relevant metadata. This includes:
 - register width,
 - AArch32 and AArch64 availability,
 - RES1 bits,
 - read/write safety settings,
 - generated `Debug` behavior,
 - AArch32 encoding,
 - special condition and exception-level gating,
 - and fields in the same order - with the same names, descriptions, positions, widths, writability, custom types, and array metadata.

Notably, this does not include the name and description of the register.

The default behavior is to globally allow type aliasing.
It may be turned off globally with the `--disable-alias` flag, or per-register via setting `disable_alias=true` in the register configuration.

## Example Configuration

```toml
[registers.SCTLR_EL3]
write_safety_doc = "The caller must ensure that `value` is a correct and safe configuration value for the EL3 system control register."

[registers.SCTLR_EL3.field_descriptions]
M = "MMU enable for EL3 stage 1 address translation."
A = "Alignment check enable."
C = "Cacheability control, for data accesses at EL3."
SA = "SP alignment check enable."
I = "Cacheability control, for instruction accesses at EL3."
WXN = "Write permission implies XN (Execute-never). For the EL3 translation regime, this bit can force all memory regions that are writable to be treated as XN."
IESB = "Enable Implicit Error Synchronization events."
EnIB = "Enable pointer authentication using APIBKey_EL1."
EnIA = "Enable pointer authentication using APIAKey_EL1."

[registers.GPCCR_EL3]

[registers.GPCCR_EL3.types]
SH = "crate::manual::Shareability"
IRGN = "crate::manual::Cacheability"
ORGN = "crate::manual::Cacheability"

[registers.CONTEXTIDR_EL1]
write = "safe"
```

[1]: https://developer.arm.com/Architectures/A-Profile%20Architecture#Downloads

--------------

*Copyright The arm-sysregs Contributors.*
