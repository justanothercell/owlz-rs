# Owlz >(^v^)<

[![Crate](https://img.shields.io/crates/v/owlz.svg)](https://crates.io/crates/owlz)

A rust library for creating cute ascii emoji owl faces, featuring:
- 7560 unique owls
- Random generation

## Why do I need this?
Do you want to sign your emails, bot messages, pull requests or memos with
lil owlz that improve the day of the people around you? 
Then this is exactly what you need!

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
owlz = "0.1.0"
rand = "0.8.5"
rand_derive2 = "0.1.17"
enum-assoc = "0.3.4"
```

Example:

```rs
use owlz::Owl;

fn main() {
    println!("{}", Owl::default());
    println!("{}", Owl::random());
    println!("{}", 
        Owl {
            beak: Beak::Happy,
            eyes: Eyes::Happy,
            head: Head::Curly,
            wing_shape: WingShape::None,
            wings: Wings::Outward
        }
    );
}
```

## Todo (Optional)
- [ ] string to owl conversion
- [ ] weighted randomness
- [ ] tooling for creation
- [ ] owl descriptors
- [ ] see if any libs can be removed to reduce dependencies