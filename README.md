`defmt::Format` proc macro for bilge bitsize structs.

## Usage

To use this crate, update your `Cargo.toml`:

```toml
# Enable `defmt` formatting for `arbitrary-int`
[dependencies.arbitrary-int]
version = "2.0.0" # This version should match what `bilge` is using
features = [ "defmt" ]

# Add defmt dependency
[dependencies.defmt]
version = "1"

[dependencies.bilge-defmt]
version = "0.1.0"
```

and add the derives:

```rust
#[bitsize(5)]
#[derive(bilge_defmt::FormatBits)]
pub struct MyTest {
    pub a_field: u3,
    pub another_field: u2,
}
```