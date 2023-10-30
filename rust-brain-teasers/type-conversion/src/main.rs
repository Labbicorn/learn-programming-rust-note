#[warn(clippy::pedantic)]

fn main() {
    let x: u64 = 4_294_967_296;
    let y = x as u32;
    if x == y as u64 {
        println!("x equals y!");
    } else {
        println!("x does not equals y!");
    }
}

// in this example, y is assigned the value 4_294_967_296,
// but the result is truncated because the number is
// greater than the maximum value of a 32-bit unsigned integer,.
// The surprise is that neither the Rust compiler, Clippy,
// nor the runtime generates any kind of warning or error that
// data loss has occurred.
//

// Converting a smaller type into a larger type(for example, u32 to u64) cannot lose precision,
// so you're safe.
//

// Be carful with floating-point to integer conversions because Rust always rounds down.
// With that in mind, It's better to indicate the desired behavior with my_float.floor to round down,
// my_float.ceil to round up, or my_float.round to perform normal numerical rounding. If you want rounding,
// perform the rouding before you use as.
//
//

// Rust provides a trait named Into to provide compile-time safe type conversions. For example, you can
// convert from a u32 to a u64 with the following code:
//
// ```rust
// let y = u32::max_value();
// let z : u64 = y.into();
// ```
//
// Rust's Into trait resolves the problem of potentially impossible conversions by not implmenting them.
//
// But, The inverse of the example converting a u64 to a u32 is impossible with Into. If you try
//
// let z: u32 = (12_u64).into(). the Into function call will fail to compile.
//
// For conversions that may be possible. Rust provides  another trait: TryInto. The following code uses try_into
// to attempt to convert between a u64 and a u32.
//
//```rust
// use std::convert::TryInto;
// let z : u32 = (5000_u64).try_into().expect("Conversion error");
// ```
//
