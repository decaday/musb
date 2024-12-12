# MUSB
`Musbmhdrc` IP Registers and `embassy-usb-driver` Implementation.



This crate contains register information because some manufacturers modify register offsets or mask certain registers, making it challenging for peripheral access crates (PACs) to handle these uniformly.

Therefore, this crate includes built-in profiles. While standard, complete MUSB chip profiles exist, other chips are recommended to be supported by adding custom profiles.

## How to Identify a MUSB IP?
- Registers start with POWER, FADDR (though not always at the beginning)
- Most registers are 8-bit (some manufacturers group them into 32-bit, but it's usually clear they're composed of 8-bit registers)
- Interrupt control and status registers are INTRRX(INTR_OUT), INTRTX(INTR_IN), INTRUSB
- An INDEX register exists, requiring setting before endpoint operations. EP0 registers are separate, other endpoints use TXCSRH, TXCSRL, RXCSRH, RXCSRL (or IN_CSR1, IN_CSR2, OUT_CSR1, OUT_CSR2)
- Compare with [block-peri-std](registers/blocks/peri_std.yaml) to confirm register similarity

# Chips with musb IP:

| Vendor            |                  | Chips (not fully listed)         | IP          |
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

## dependencies

When you don't use `prebuild` feature, you need to install:

``` shell
cargo install --git https://github.com/embedded-drivers/yaml2pac --rev 0b96c69a30557214ceb16bd7429ab9ca1c52fc7e --locked

# On the Stable toolchain
rustup component add rustfmt
# On the Nightly toolchain
rustup component add rustfmt --toolchain nightly
```

## Available Built-in Profiles

These built-in profiles are used via Cargo features (see below), with only one selectable:

`builtin-py32f07x`    `builtin-py32f403`    `builtin-std-full`

These profiles include register descriptions, number of endpoints, etc.
Base address is optional. If left blank, it reads from the `MUSB_BASE_ADDRESS` environment variable.

## Prebuild

Pre-generated Rust register code is available in `src/prebuild`, eliminating the need to rerun build scripts generating these contents.

## Embed this repository into a HAL crate:

Example: [py32-hal/src/usb.rs · py32-rs/py32-hal](https://github.com/py32-rs/py32-hal/blob/main/src/usb.rs)

# Profiles & Registers

Each manufacturer's SVD or manual exhibits significant register name variations, despite functional consistency. This crate uses standard MUSB register names.

The crate describes registers using YAML, with these register description files manually maintained. It then leverages [chiptool](https://github.com/embassy-rs/chiptool) and [yaml2pac](https://github.com/embedded-drivers/yaml2pac) to generate register operation functions. These operations can be found in the `build.rs` file.

## Available Replacements

These replacements are automatically generated from profile contents and can be used in register description YAML files.

### ENDPOINT_COUNT

`profile.endpoint_count`

### FIFO_REG_BIT_SIZE

`profile.reg_bit_size.fifo`

Note: This does not change the offset.

### INTR_REG_BIT_SIZE

`profile.reg_bit_size.intr`

Note: This does not change the offset.

# Contribute

If you have any questions or uncertainties, feel free to create an Issue or start a Discussion.

## TODOs:

- auto-generate interal features according to profile

- Support SiFli SF32BL52
- Other Chips
- better support for standard musb
- host mode 