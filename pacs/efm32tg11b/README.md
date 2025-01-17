# EFM32TG11B
    
[![crates.io](https://img.shields.io/crates/v/efm32tg11b-pac?label=efm32tg11b)](https://crates.io/crates/efm32tg11b-pac)

This crate provides an autogenerated API for access to EFM32TG11B peripherals.

## Usage

Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.efm32tg11b-pac]
version = "0.1.3"
features = ["efm32tg11b120"]
```

The `rt` feature is enabled by default and brings in support for `cortex-m-rt`.
To disable, specify `default-features = false` in `Cargo.toml`.

For full details on the autogenerated API, please see `svd2rust` Peripheral API [here].

[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api

## Supported Devices
| Feature | Devices |
|:-----:|:-------:|
|`efm32tg11b120`|EFM32TG11B120F128GM32, EFM32TG11B120F128GM64, EFM32TG11B120F128GQ48, EFM32TG11B120F128GQ64, EFM32TG11B120F128IM32, EFM32TG11B120F128IM64, EFM32TG11B120F128IQ48, EFM32TG11B120F128IQ64|
|`efm32tg11b140`|EFM32TG11B140F64GM32, EFM32TG11B140F64GM64, EFM32TG11B140F64GQ48, EFM32TG11B140F64GQ64, EFM32TG11B140F64IM32, EFM32TG11B140F64IM64, EFM32TG11B140F64IQ48, EFM32TG11B140F64IQ64|
|`efm32tg11b320`|EFM32TG11B320F128GM64, EFM32TG11B320F128GQ48, EFM32TG11B320F128GQ64, EFM32TG11B320F128IM64, EFM32TG11B320F128IQ48, EFM32TG11B320F128IQ64|
|`efm32tg11b340`|EFM32TG11B340F64GM64, EFM32TG11B340F64GQ48, EFM32TG11B340F64GQ64, EFM32TG11B340F64IM64, EFM32TG11B340F64IQ48, EFM32TG11B340F64IQ64|
|`efm32tg11b520`|EFM32TG11B520F128GM32, EFM32TG11B520F128GM64, EFM32TG11B520F128GM80, EFM32TG11B520F128GQ48, EFM32TG11B520F128GQ64, EFM32TG11B520F128GQ80, EFM32TG11B520F128IM32, EFM32TG11B520F128IM64, EFM32TG11B520F128IM80, EFM32TG11B520F128IQ48, EFM32TG11B520F128IQ64, EFM32TG11B520F128IQ80|
|`efm32tg11b540`|EFM32TG11B540F64GM32, EFM32TG11B540F64GM64, EFM32TG11B540F64GM80, EFM32TG11B540F64GQ48, EFM32TG11B540F64GQ64, EFM32TG11B540F64GQ80, EFM32TG11B540F64IM32, EFM32TG11B540F64IM64, EFM32TG11B540F64IM80, EFM32TG11B540F64IQ48, EFM32TG11B540F64IQ64, EFM32TG11B540F64IQ80|
