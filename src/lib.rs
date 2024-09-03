use std::collections::HashMap;
/// Converts a value from one unit to another.
///
/// # Arguments
///
/// * `value` - The numerical value to be converted.
/// * `from` - The unit of the input value (e.g., "bit", "byte", "kb").
/// * `to` - The unit to convert the value to (e.g., "bit", "byte", "kb").
/// # Units
/// ```bit,byte,kb,mb,gb,tb,pb,eb```
///```
/// use data_units::convert_units;
/// let result = convert_units(8, "bit", "byte"); // Result = 1
/// ```
/// # Returns
///
/// The converted value in the target unit.
///
/// # Example
///
/// ```
/// use data_units::convert_units;
/// let result = convert_units(8, "bit", "byte");
/// assert_eq!(result, 1);
/// ```
pub fn convert_units(value: u64, from: &str, to: &str) -> u64 {
    let mut units = HashMap::new();
    units.insert("bit", 1);
    units.insert("byte", 8);
    units.insert("kb", 8 * 1024);
    units.insert("mb", 8 * 1024 * 1024);
    units.insert("gb", 8 * 1024 * 1024 * 1024);
    units.insert("tb", 8 * 1024 * 1024 * 1024 * 1024);
    units.insert("pb", 8 * 1024 * 1024 * 1024 * 1024 * 1024);
    units.insert("eb", 8 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024);

    // Convert the given value to bits
    let value_in_bits = value * units[from];

    // Convert the bit value to the target unit
    let result = value_in_bits / units[to];

    result
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_should_convert_units(){
        assert_eq!(convert_units(8, "bit", "byte"), 1);
        assert_eq!(convert_units(1, "byte", "bit"), 8);
        assert_eq!(convert_units(1, "kb", "byte"), 1024);
        assert_eq!(convert_units(1, "mb", "kb"), 1024);
        assert_eq!(convert_units(1, "gb", "mb"), 1024);
        assert_eq!(convert_units(1, "tb", "gb"), 1024);
        assert_eq!(convert_units(1, "pb", "tb"), 1024);
        assert_eq!(convert_units(1, "eb", "pb"), 1024);
    }
}
