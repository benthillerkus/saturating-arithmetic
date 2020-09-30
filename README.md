This crate provides a proc macro that rewrites arithemtic operators `+,-,*` into their saturating equivalents `saturating_add, saturating_sub, saturating_mul` as well as their assigning versions `+=,-=,*=`.

The following function for example
````Rust
#[saturateit]
fn mix(a: u32, b: u32, c: &[u32]) -> u32 {
    let mut r = a + b;
    for u in c {
        r *= u;
    }
    r
}
````
is rewritten to
````Rust
fn mix(a: u32, b: u32, c: &[u32]) -> u32 {
    let mut r = a.saturating_add(b);
    for u in c {
        r = r.saturating_mul(u);
    }
    r
}
