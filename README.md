`defmt::Format` proc macro for bilge bitsize structs.

## Usage

To use this crate, add the dependency to your `Cargo.toml`:

```toml
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