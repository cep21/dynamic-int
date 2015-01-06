
# dynamic-int

For tight loops that are usually normal integers but still need
to work when the integers overflow

## Example

```rust
extern crate dynamic_int;
use dynamic_int::DynamicChangingU64;
let a = DynamicChangingU64::new(123);
let b = a * 99999;
let c = b * 999999999999;
// ...
```
