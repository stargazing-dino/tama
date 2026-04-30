# tama

![XIAO nRF54L15 pinout](hardware/datasheets/xiao-nrf54l15-pinout.png)

## Build & flash

```sh
cargo run --release
```

If you hit `APPROTECT` errors:

```sh
probe-rs erase --chip nRF54L15 --allow-erase-all
```
