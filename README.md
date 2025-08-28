# MUSB

[![Crates.io][badge-license]][crates]
[![Crates.io][badge-version]][crates]

[badge-license]: https://img.shields.io/crates/l/musb?style=for-the-badge
[badge-version]: https://img.shields.io/crates/v/musb?style=for-the-badge
[crates]: https://crates.io/crates/musb

musb(Mentor USB) Registers and `embassy-usb-driver` , `usb-device` Implementation.

The MUSBMHDRC (musb) is a USB 2.0 Multi-Point, Dual-Role Controller designed by Mentor Graphics Corp. It is widely used by various manufacturers in microcontrollers and SoCs, including companies like TI, MediaTek, Puya, Allwinner, and others.

## Quick Start

These built-in profiles are used via Cargo features (see below), with only one selectable:

- `builtin-py32f07x` (py32m070, py32f071, py32f072)

- `builtin-py32f403`

- `builtin-sf32lb52x`

- `builtin-std-8bep-2048` (8 bidirectional endpoints, 2048K FIFO size in total, without instance)

Add musb to your `Cargo.toml`:

```toml
[dependencies]
musb = { version = "0.3.0", features = ["builtin-std-8bep-2048"] }
```

You can use the [std profile](registers/profiles/) by enabling the `builtin-std-xxx` feature. This profile doesn't include a base_address, so it won't generate a `UsbInstance` (explained in [Porting Guide](docs/porting_guide.md)).

If your chip is not included, you'll need to create a new profile. Refer to the [Porting Guide](docs/porting_guide.md) for more details.

## Features

`embassy-usb-driver-impl`: Enables [embassy-usb-driver](https://crates.io/crates/embassy-usb-driver) implementation.

`usb-device-impl`: Enables [usb-device](https://crates.io/crates/usb-device) implementation.

Note: Only one of these two implementations can be enabled at a time.

`prebuild`(on by default): Uses pre-generated PAC (Peripheral Access Crate).

`builtin-xxxx` : Uses builtin profile.


`defmt`, `log`: Enables debug logging.

## Examples

hal example: [py32-hal/src/usb.rs · py32-rs/py32-hal](https://github.com/py32-rs/py32-hal/blob/main/src/usb.rs)

embassy-usb: [py32-hal/examples/py32f072](https://github.com/py32-rs/py32-hal/tree/main/examples/py32f072)

usb-device: [py32-hal/examples/usbd-f072](https://github.com/py32-rs/py32-hal/tree/main/examples/usbd-f072)

## Porting

Refer to the [Porting Guide](docs/porting_guide.md) for more details.

You can also refer to this blog (in Chinese): [PY32的musb(Mentor USB)的Rust支持 - Decaday](https://decaday.github.io/blog/py32-musb/)

## Contribution

If you have any questions or uncertainties, feel free to create an Issue or start a Discussion.

## TODOs

- **Support Dynamic FIFO Size**
- Support dual packet buffer
- Support SiFli SF32LB52
- Other Chips
- Host mode 

## License

This project is under Apache License, Version 2.0 ([LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>).