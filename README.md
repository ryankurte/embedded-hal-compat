# Embedded HAL Compatibility Layer

A _forward_ compatibility layer to smooth the transition between different versions of [embedded-hal](https://github.com/rust-embedded/embedded-hal) (specifically `0.2.x` and `1.0.0-alpha.X` series).

This resolves the problem where a HAL implementation (ie. the implementation for your processor) is still published at `0.2.x`, and a driver expects `1.0.0-alpha.x`. In the opposite situation, please fork and update the driver.

## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/embedded-hal-compat.svg)](https://github.com/ryankurte/embedded-hal-compat)
![Build Status](https://github.com/ryankurte/embedded-hal-compat/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/embedded-hal-compat.svg)](https://crates.io/crates/embedded-hal-compat)
[![Docs.rs](https://docs.rs/embedded-hal-compat/badge.svg)](https://docs.rs/embedded-hal-compat)

Work in progress, `blocking` objects implemented, missing `nb` things as well as any reasonable documentation.

#### How to use this:

- Add `embedded-hal-compat` to your dependencies
- Import with `use embedded_hal_compat::IntoCompat;`
- Vicariously add `.compat()` on any `embedded-hal` outstanding type errors

#### How do I know that I need this?

Type errors. Lots of type errors.

(and `cargo tree -i embedded-hal` returns two versions)

#### Is this cursed?!

Yes. You're gonna end up with lots of cursed errors but at least you can compile things?


#### What kind of errors?

Because you'll have symbols with the same name you'll end up with errors like:

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

