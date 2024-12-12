# Now in:

https://github.com/py32-rs/py32-hal/pull/19

# musb IP:

| Vendor            |            | Chips (not fully listed)         | IP          |
| ----------------- | ---------- | -------------------------------- | ----------- |
| Texas Instruments | 德州仪器       | am335x,am1802, F2837xD, LM3S6911 | musb**      |
| allwinner         | 全志         | F1C100S, F133                    | musb phy ip |
| SiFli             | 思澈         | SF32LB52x                        | musb std*   |
| beken             | 北科微 or 小博通 | beken725x                        | musb std*   |
| essemi            | 东软载波微      | ES32F027x                        | musb std*   |
| STC               | 姚永平        | stc8, stc32, ai8051u             | musb mini*  |
| Puya              | 普冉         | py32f071, py32f403               | musb mini*  |

*: Not sure about the IP name

**: Further identification is needed

# Usage

## dependencies

``` shell
cargo install --git https://github.com/embedded-drivers/yaml2pac --rev 0b96c69a30557214ceb16bd7429ab9ca1c52fc7e --locked

# On the Stable toolchain
rustup component add rustfmt
# On the Nightly toolchain
rustup component add rustfmt --toolchain nightly
```

## Avaliable Replacements

### ENDPOINT_COUNT

`profile.endpoint_count`

### FIFO_REG_BIT_SIZE

`profile.reg_bit_size.fifo`

Note: This does not change the offset.

### INTR_REG_BIT_SIZE

`profile.reg_bit_size.intr`

Note: This does not change the offset.