# Porting Guide

This guide explains how to add support for new chips using MUSB.

## Known chips using MUSB IP

| Manufacturer             | Chips (not fully listed)         | IP             |
| ------------------------ | -------------------------------- | -------------- |
| Texas Instruments        | am335x,am1802, F2837xD, LM3S6911 | musb*          |
| Allwinner(全志)          | F1C100S, F133                    | musb phy ip*   |
| SiFli (思澈)             | SF32LB52x                        | musb std + phy |
| beken (北科微 or 小博通) | beken725x                        | musb std       |
| essemi (东软载波微)      | ES32F027x                        | musb std       |
| jieli (杰理)             |                                  | musb*          |
| SPRD                     | sp7862                           | musb*          |
| MediaTek                 | MT6735                           | musb*          |
| Puya (普冉)              | py32f071, py32f403               | musb mini      |
| STC (姚永平)             | stc8, stc32, ai8051u             | musb mini      |

I'm not sure about the official sub-name/branch name of them.

*: Further identification is needed

## Overview

MUSB uses YAML files to describe register layouts and chip-specific configurations.

## Creating a New Profile

1. ##### **Identify Your Chip**

   - Registers start with POWER, FADDR (though not always at the beginning)
   - Most registers are 8-bit (some manufacturers group them into 32-bit, but it's usually clear they're composed of 8-bit registers)
   - An INDEX register exists, requiring setting before endpoint operations. EP0 registers are separate, other endpoints use TXCSRH, TXCSRL, RXCSRH, RXCSRL (or IN_CSR1, IN_CSR2, OUT_CSR1, OUT_CSR2)
   - Compare with [block-peri-std](../registers/blocks/peri_std.yaml) to confirm register similarity

2. ##### **Create Profile YAML**

   To write a profile, please refer to [existing profiles](../registers/profiles) and [serialization-related data types](../build_src/build_serde.rs)

   ```yaml
   # Example profile structure
   name: my_chip
   block: peri_std
   base_address: 0x40005c00
   base_address: 0x40005c00
   fifo:
     type: fixed
     shared: false
   reg_bit_size:
     fifo: 8
     intr: 8
   endpoints:
     - type: rxtx
       max_packet_size: 64
     - type: rxtx
       max_packet_size: 64
   
   ```

3. ##### **Register Definitions**

   Each manufacturer's SVD or manual exhibits significant register name variations, despite functional consistency. This crate uses standard MUSB register names.

   The crate describes registers using YAML, with these register description files manually maintained. It then leverages [chiptool](https://github.com/embassy-rs/chiptool) and [yaml2pac](https://github.com/embedded-drivers/yaml2pac) to generate register operation functions. These operations can be found in the `build.rs` file.

   These replacements are automatically generated from profile contents and can be used in register description YAML files.

   - **ENDPOINTS_NUM**

     `profile.endpoints.len()`

   - **FIFO_REG_BIT_SIZE**

     `profile.reg_bit_size.fifo`

     Note: This does not change the offset.

   - **INTR_REG_BIT_SIZE**

     `profile.reg_bit_size.intr`
     
     Note: This does not change the offset.

4. ##### **Register Patches**

   When your IP has non-official registers or its layout differs from the standard, you can use **patches** in your profile:

   ```yaml
   patches:
     - fieldset: USBCFG
       version: sf32
   ```

   Then, you can add your register fieldset file under `registers\fieldsets`, for example: `registers\fieldsets\vendor\USBCFG_sf32.yaml`.
   The version (e.g., `sf32`) is the last part of the filename separated by an underscore ( `_` ).It must be lowercase letters (digits are allowed).

## Add features & prebuilds

You need to add `builtin-your-chip-name` in `Cargo.toml`, and it must match the profile name.

Then, in a proper place within `src\generated.rs`, add the following lines by imitating the existing pattern:

```rust
#[cfg(feature = "builtin-your-chip-name")]
pub mod regs {
   include!("prebuilds/your-chip-name/regs.rs");
}
#[cfg(feature = "builtin-std-8bep-2048")]
include!("prebuilds/your-chip-name/_generated.rs");
```

Next, create an empty folder named `your-chip-name` under `src\prebuilds`.

After that, run:

```shell
python update_prebuild.py
```

If compilation succeeds, the script will automatically add or update the pre-generated code and files (such as PAC) under `src\prebuilds\your-chip-name`.

## Integrate this crate into a HAL crate

#### Examples

[py32-hal/src/usb.rs · py32-rs/py32-hal](https://github.com/py32-rs/py32-hal/blob/main/src/usb.rs)

[sifli-rs/sifli-hal/src/usb.rs · OpenSiFli/sifli-rs](https://github.com/OpenSiFli/sifli-rs/blob/main/sifli-hal/src/usb.rs)

#### Instance

Every struct in this crate has a generic parameter `T: MusbInstance`:

```rust
pub trait MusbInstance: 'static {
    fn regs() -> regs::Usb;
}
```

If the profile contains a base_address field, you can directly use:

```rust
let musb_driver = musb::MusbDriver::new::<musb::UsbInstance>();
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

#### Driver

This crate includes a `MusbDriver` that has methods from `embassy_usb_driver::Driver` but doesn't implement it. The intention is for HAL libraries to create a `Driver` that wraps `MusbDriver` to handle platform-specific peripheral initialization. Please refer to the [Examples](#examples) .

## Then

Contribution back is welcome!