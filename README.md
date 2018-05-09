# `max31865`

A generic driver for the MAX31865 RTD to Digital converter

## [Documentation](https://rudihorn.github.io/max31865/max31865/index.html)

## [Examples](https://github.com/rudihorn/max31865/tree/extra_examples/examples)

## What works

- reading the raw value and the converted temperature value
- setting the ohmic calibration value
- configuring V_BIAS, one shot, filter frequency

## TODO

- [ ] Fault tolerance / detection / status
- [ ] Ensure temperature conversion table handles out of bounds values using interpolation (less than 0°C or more than 130°C)
- [ ] Non Raspberry Pi example (as input pins aren't handled correctly)
- [ ] Conversion to non Celsius units, e.g. Kelvin

## Examples

There is an example for the Raspberry pi in the examples directory.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

