# Embedded HAL Compatibility Layer

A compatibility layer to smooth the transition between different versions of [embedded-hal](https://github.com/rust-embedded/embedded-hal) (specifically `0.2.x` and `1.0.0-alpha.X` series).

This resolves the problem where a HAL implementation (ie. the implementation for your processor) and driver you intend to use are mismatched.

This crate is intended to track `1.0.0-alpha` versions, and update to `1.0.0` on release, adaptation is not provided between `1.0.0-alpha.x` releases (though we _could_ do this if it was deemed worthwhile)

### Supported Versions

Each release of `embedded-hal-compat` supports one(ish) pair of e-h releases, because of changes to the `1.0.0-alpha`, use:

- `embedded-hal-compat = "0.7.0"` for `=1.0.0-alpha.8` and `^0.2.4`
- `embedded-hal-compat = "0.6.0"` for `=1.0.0-alpha.7` and `^0.2.4`
- `embedded-hal-compat = "0.5.0"` for `=1.0.0-alpha.6` and `^0.2.4`
- `embedded-hal-compat = "0.4.0"` for `=1.0.0-alpha.5` and `^0.2.4`
- `embedded-hal-compat = "0.3.0"` for `=1.0.0-alpha.4` and `^0.2.4`


## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/embedded-hal-compat.svg)](https://github.com/ryankurte/embedded-hal-compat)
[![Build Status](https://github.com/ryankurte/embedded-hal-compat/actions/workflows/rust.yml/badge.svg)](https://github.com/ryankurte/embedded-hal-compat/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/embedded-hal-compat.svg)](https://crates.io/crates/embedded-hal-compat)
[![Docs.rs](https://docs.rs/embedded-hal-compat/badge.svg)](https://docs.rs/embedded-hal-compat)

Not all wrappers are fully implemented, feel free to open a PR if you come across something missing!


#### How to use this:

- Add `embedded-hal-compat` to your dependencies
- Import with `use embedded_hal_compat::{ForwardCompat, ReverseCompat};`
- Vicariously add `.forward()` or `.reverse()` on any `embedded-hal` type mismatches
- See [docs.rs](https://docs.rs/embedded-hal-compat/latest/embedded_hal_compat/) for further documentation

#### How do I know that I need this?

Type errors. Lots of type errors.

(and `cargo tree -i embedded-hal` returns two versions)


#### Is this cursed?!

At least a little bit, because traits have the same method names you might end up with some cursed errors, and we have to pave over some differences around errors and error kinds...


#### What kind of errors?

Because you're likely to have symbols with the same name you'll end up with errors like:

```
error[E0599]: no method named `delay_ms` found for mutable reference `&mut Compat<atsamd_hal::delay::Delay>` in the current scope
   --> src/main.rs:351:27
    |
351 |         tlv.inner_delay().delay_ms(10u32);
    |                           ^^^^^^^^ method not found in `&mut Compat<atsamd_hal::delay::Delay>`

warning: unused import: `embedded_hal::blocking::delay::DelayMs`
  --> src/main.rs:22:5
   |
22 | use embedded_hal::blocking::delay::DelayMs;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Suggesting you're missing an import that's obviously there, because you have the `0.2.x` imports not the `1.0.0-alpha.x` imports, and the method is called `try_delay_ms` in the updated HAL (a fairly common renaming).

You can fix this by importing the correct trait with something like: `use embedded_hal_compat::eh1_0::blocking::delay::{DelayMs as _};` (this must be renamed with `as` / you can't use the prelude because the names overlap), and by swapping the method to use the correct `try_` name.

