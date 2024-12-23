# MUSB
`Musbmhdrc` IP Registers and `embassy-usb-driver` Implementation.



This crate contains register information because some manufacturers modify register offsets or mask certain registers, making it challenging for PACs to handle these uniformly.

## How to Identify a MUSB IP?
- Registers start with POWER, FADDR (though not always at the beginning)
- Most registers are 8-bit (some manufacturers group them into 32-bit, but it's usually clear they're composed of 8-bit registers)
- An INDEX register exists, requiring setting before endpoint operations. EP0 registers are separate, other endpoints use TXCSRH, TXCSRL, RXCSRH, RXCSRL (or IN_CSR1, IN_CSR2, OUT_CSR1, OUT_CSR2)
- Compare with [block-peri-std](registers/blocks/peri_std.yaml) to confirm register similarity

## Known chips using MUSB IP

| Manufacturer      |                  | Chips (not fully listed)         | IP          |
| ----------------- | ---------------- | -------------------------------- | ----------- |
| Texas Instruments |                  | am335x,am1802, F2837xD, LM3S6911 | musb**      |
| allwinner         | 全志             | F1C100S, F133                    | musb phy ip |
| SiFli             | 思澈             | SF32LB52x                        | musb std*   |
| beken             | 北科微 or 小博通 | beken725x                        | musb std*   |
| essemi            | 东软载波微       | ES32F027x                        | musb std*   |
| STC               | 姚永平           | stc8, stc32, ai8051u             | musb mini*  |
| Puya              | 普冉             | py32f071, py32f403               | musb mini*  |

*: Not sure about the IP name

**: Further identification is needed

# Usage

- If your chip uses standard MUSB IP (and hasn't disabled features like Dynamic FIFO size configuration):
  
  You can use the [std profile](registers/profiles/std.yaml) by enabling the `builtin-std` feature. This profile doesn't include a base_address, so it won't generate a UsbInstance (explained below).
  
  You can then set the number of endpoints using the `endpoints-num-x` feature (e.g., `endpoints-num-8`). The total FIFO size can be configured using the `total-fifo-size-dword-x` feature (e.g., `total-fifo-size-dword-256` where 256 double-words = 2048 bytes) *(TODO)*.
  Currently, `endpoints-num-x` and `total-fifo-size-dword-x` are **not effective** when the `prebuild` feature is enabled.
  
- If your chip's MUSB implementation differs significantly from the standard MUSB IP:
  You'll need to create a new profile. You can reference [the PY32F07x profile example](registers/profiles/py32f07x.yaml). Specific details will be covered below. If your register offsets or sizes differ from existing [blocks](registers/blocks) and [fieldsets](registers/fieldsets), you'll need to add your specific blocks or fieldsets.

## Available Built-in Profiles

These built-in profiles are used via Cargo features (see below), with only one selectable:
- `builtin-py32f07x`
- `builtin-py32f403`
- `builtin-std` (excludes base_address and endpoints_num)

These profiles include register descriptions, number of endpoints, etc.

## Prebuild

Pre-generated Rust register code for each builtin is available in `src/prebuild`, eliminating the need to rerun build scripts generating these contents.

Note: `endpoints-num-x` and `total-fifo-size-dword-x` are not effective when the `prebuild` feature is enabled.

When you don't use the `prebuild` feature, you need to install:
```shell
cargo install --git https://github.com/embedded-drivers/yaml2pac --rev 0b96c69a30557214ceb16bd7429ab9ca1c52fc7e --locked
# On the Stable toolchain
rustup component add rustfmt
# On the Nightly toolchain
rustup component add rustfmt --toolchain nightly
```

## Integrate this crate into a HAL crate

### Examples
Example: [py32-hal/src/usb.rs · py32-rs/py32-hal](https://github.com/py32-rs/py32-hal/blob/main/src/usb.rs)

### Instance

Every struct in this crate has a generic parameter `T: MusbInstance`.

```rust
pub trait MusbInstance: 'static {
    fn regs() -> regs::Usb;
}
```

If the profile contains a base_address field, you can directly use:
```rust
let musb_driver = musb::Driver::<musb::UsbInstance>(new);
```

`musb::UsbInstance` is a struct that has already implemented the `MusbInstance` trait based on base_address.

If not, you need to create this type:
```rust
pub struct UsbInstance;
impl musb::MusbInstance for UsbInstance {
    fn regs() -> musb::regs::Usb {
        unsafe { musb::regs::Usb::from_ptr((0x40005c00) as _ ) }
    }
}
```

### Driver

This crate includes a `MusbDriver` that has methods from `embassy_usb_driver::Driver` but doesn't implement it. The intention is for HAL libraries to create a `Driver` that wraps `MusbDriver` to handle platform-specific peripheral initialization. Please refer to the [Examples](#examples) section.

# Profiles & Registers

Each manufacturer's SVD or manual exhibits significant register name variations, despite functional consistency. This crate uses standard MUSB register names.

The crate describes registers using YAML, with these register description files manually maintained. It then leverages [chiptool](https://github.com/embassy-rs/chiptool) and [yaml2pac](https://github.com/embedded-drivers/yaml2pac) to generate register operation functions. These operations can be found in the `build.rs` file.

To write a profile, please refer to [existing profiles](registers/profiles) and [serialization-related data types](build_src/build_serde.rs)

## Available Replacements

These replacements are automatically generated from profile contents and can be used in register description YAML files.

- **ENDPOINTS_NUM**

  `profile.endpoints_num` OR `endpoints-num-x` feature (e.g. `endpoints-num-8`)。

- **FIFO_REG_BIT_SIZE** 

  `profile.reg_bit_size.fifo`

  Note: This does not change the offset.

- **INTR_REG_BIT_SIZE**

  `profile.reg_bit_size.intr`

  Note: This does not change the offset.

# Contribute

If you have any questions or uncertainties, feel free to create an Issue or start a Discussion.

## TODOs:

- **Support Dynamic FIFO Size**
- Support SiFli SF32BL52
- Other Chips
- better support for standard musb
- host mode 