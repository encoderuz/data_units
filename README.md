# Data Units Converter
![Test](https://github.com/encoderuz/data_units/actions/workflows/data_units.yml/badge.svg)
[![Current Crates.io Version](https://img.shields.io/crates/v/data_units.svg)](https://crates.io/crates/data_units)
[![Downloads](https://img.shields.io/crates/d/data_units.svg)](https://crates.io/crates/data_units)
[![Docs](https://docs.rs/data_units/badge.svg)](https://docs.rs/data_units/latest/data_units/)
[![MSRV](https://img.shields.io/badge/rustc-1.64.0+-ab6000.svg)](https://blog.rust-lang.org/2022/09/22/Rust-1.64.0.html)

This library allows you to convert data units. 
### Convert units
```rust
use data_units::convert_units;
let result = convert_units(8, "bit", "byte"); // 1
```

Allowed units
```bit,byte,kb,mb,gb,tb,pb,eb```
