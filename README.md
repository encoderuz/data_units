# Data Units Converter
![Test](https://github.com/encoderuz/data_units/actions/workflows/data_units.yml/badge.svg)
[![Current Crates.io Version](https://img.shields.io/crates/v/data_units.svg)](https://crates.io/crates/data_units)
[![Downloads](https://img.shields.io/crates/data_units/bevy.svg)](https://crates.io/crates/data_units)

This library allows you to convert data units. 
### Convert units
```rust
use data_units::convert_units;
let result = convert_units(8, "bit", "byte"); // 1
```

Allowed units
```bit,byte,kb,mb,gb,tb,pb,eb```
