# biquad-micromath

Implementation of biquad filters using the [micromath crate](https://crates.io/crates/micromath) for calculation of coefficients.

**Note:** Using *micromath* optimizes performance and code size for embedded systems, but is less precise than using regular standard library functions or [libm](https://crates.io/crates/libm).

Available Filter Types:

- Low-pass
- High-pass
- Band-pass
- Notch
- Peaking EQ
- Low-shelf
- High-shelf
- All-pass
- First order low-pass
- First order high-pass
- First order low-shelf
- First order high-shelf
- First order all-pass
- One-pole low-pass

Notes:

- The *first order* and *one-pole* types are included primarily for convenience. They perform worse than dedicated implementations because of unnecessary calculations caused by some coefficients being 0.
- The *one-pole high-pass* is ommitted because it doesn't perform very well.

## Serialization Support

Use the optional `serde` feature to enable serialization support for the filter type and coefficients.

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.
