# biquad-micromath

Implementation of biquad filters using the [micromath crate](https://crates.io/crates/micromath) for calculation of coefficients.

**Note:** Using *micromath* optimizes performance and code size for embedded systems, but is less precise than using regular standard library functions or [libm](https://crates.io/crates/libm).

## Serialization Support

Use the optional `serde` feature to enable serialization support for the filter type and coefficients.

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.
