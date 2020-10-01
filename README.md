<img src="https://forthebadge.com/images/badges/works-on-my-machine.svg" height=23px> <a href="https://www.apache.org/licenses/LICENSE-2.0"><img src="https://img.shields.io/crates/l/saturating_arithmetic?style=for-the-badge" height=23px></a>

## Description
This crate provides a procedural macro that rewrites arithmetic operators `+,-,*` as well as their assigning versions `+=,-=,*=` into their saturating equivalents `saturating_add, saturating_sub, saturating_mul`.
This is, for example, useful for quickly *safening* older code, if you can live with the performance penalty caused by the checks that is.

### Links
[GitHub](https://github.com/ogierm/saturating-arithmetic)  
[Crates.io](https://crates.io/crates/saturating_arithmetic)

## Contents
- [Description](#description)
  - [Links](#links)
- [Contents](#contents)
- [Example](#example)
- [Installation](#installation)
- [Usage](#usage)
  - [Warnings](#warnings)
- [Disclaimer](#disclaimer)
  - [Todo](#todo)

## Example
The following function
```Rust
#[saturateit]
fn mix(a: u32, b: u32, c: &[u32]) -> u32 {
    let mut r = a + b;
    for u in c {
        r *= u;
    }
    r
}
```
is rewritten into
```Rust
fn mix(a: u32, b: u32, c: &[u32]) -> u32 {
    let mut r = a.saturating_add(b);
    for u in c {
        r = r.saturating_mul(u);
    }
    r
}
```

## Installation
To your `Cargo.toml` under `[dependencies]` add:
```toml
[dependencies]
saturating_arithmetic = "0.1"

# If you want this to work on your own types,
# you'll need this crate too:
num-traits = "0.2"
```

Then in your entry point (`main.rs` or `lib.rs`) add
```rust
extern crate saturating_arithmetic;
extern crate num_traits;
```

and then `use` them in your code.
```rust
use saturating_arithmetic::saturateit;
use num_traits::{SaturatingAdd,
  SaturatingMul, SaturatingSub};
```

## Usage
<img src="https://forthebadge.com/images/badges/60-percent-of-the-time-works-every-time.svg" height=23px>    

Add `#[saturateit]` above your function body.
```rust
#[saturateit]
fn lmao_jeff() {
    4 + 4;
}
```
You can use something like [cargo expand](https://crates.io/crates/cargo-expand) to check if the macro actually worked.
```rust
fn lmao_jeff() {
    4.saturating_add(4);
}
```

### Warnings
If you are using the traits too, you may see a warning about having to borrow the right hand side of the operator, *because the function signature according to the trait is `saturating_add(&self, rhs: &Self) -> Self`*.  
In my experience you can ignore these.

<br>

----
## Disclaimer
<img src="https://forthebadge.com/images/badges/ctrl-c-ctrl-v.svg" height=23px> <a href="https://unmaintained.tech"><img src="https://unmaintained.tech/badge.svg" height=23px></a>    

I forked this from [wrapping_arithmetic](https://crates.io/crates/wrapping_arithmetic), because I needed it (kind of). I have not clue how this actually works besides *procedural macro magic* and *those fat dependencies sure do something*. Feel free to create an Issue or a PR, but I can't promise that I'll be able to help you.

### Todo
In general the whole thing needs a redo to be more like [overflower](https://crates.io/crates/overflower), but actively maintained and without any nightly features. I'm also unhappy with the whole trait thing, there has to be a way to make this all work reliably. *For sure.*
