# Penis 2, An improved penis generator.

## Reasons to use:

- Clean, OO Api.

- Good documentation.

- Unit tested.

- Allows specifying full and partial len.

## Basic usage:

```rust
use penis2::PenisSpec;
fn main() {
    println!("default: {}", PenisSpec::new().generate());
    println!("lenght of 4: {}", PenisSpec::new_with_len(4).generate());
}
```

## How to usage

Add this to you ``Cargo.toml``.

```toml
penis2 = "0.1.0"
```

