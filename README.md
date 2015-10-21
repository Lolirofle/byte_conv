# byte_conv
Conversions between types and byte slices.

### Example:
```rust
use byte_conv::{As as AsBytes,From as FromBytes};

// Convert to bytes
println!("{:?}",258i32.as_bytes());
//Example output: "[2, 1, 0, 0]" or "[0, 0, 1, 2]"

//Convert from bytes
println!("{:?}",unsafe{i32::from_bytes(&[0,0,0,1u8] as &[u8])});
//Example output: "16777216" or "1"
```
