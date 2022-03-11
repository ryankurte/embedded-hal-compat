// Embedded HAL Compat
// Copyright 2021 Ryan Kurte
//!
//! A compatibility layer to alleviate (some) of the issues resolving from changes to embedded-hal.
//! This crate lets you _easily_ mix and match drivers and hal implementations using `v1.x.x` and `v0.2.x` versions of embedded-hal, just add a `.forward()` or a `.reverse()` wherever you see trait bounds errors.
//!
//! Note that no effort is made to support interoperability between alpha versions, we'll do our best to keep
//! up with the latest alpha and swap to `1.0.0` on release. In the future these traits may be renamed to support more hal versions.
//!
//! ## Forward compatibility:
//! Calling `ForwardCompat::forward()` (or `.forward()`) on `v0.2.x` types creates a wrapper for use with `v1.0.x` consumers, so you can drop these wrapped types into drivers expecting `v1.0.x` types.
//!```
//! # use embedded_hal_compat::mock::OutputPin0_2;
//! use embedded_hal_compat::ForwardCompat;
//!
//! // Create e-h v0.2.x based type (mock)
//! let mut old = OutputPin0_2;
//! // Access via e-h v0.2.x methods
//! let _ = eh0_2::digital::v2::OutputPin::set_high(&mut old);
//!
//! // Apply forward compatibility wrapper
//! let mut new = old.forward();
//! // Access via e-h v1.x.x methods
//! let _ = eh1_0::digital::blocking::OutputPin::set_high(&mut new);
//!```
//!
//! ## Backwards compatibility:
//! Calling `ReverseCompat::reverse()` (or `.reverse()`) on `v1.0.x` types creates a wrapper for use with `v0.2.x` consumers, so you can drop these wrapped types into drivers expecting `v0.2.x` types.
//!```
//! # use embedded_hal_compat::mock::OutputPin1_0;
//! use embedded_hal_compat::ReverseCompat;
//!
//! // Create e-h v1.x.x based type (mock)
//! let mut new = OutputPin1_0;
//! // Access via e-h v1.x.x methods
//! let _ = eh1_0::digital::blocking::OutputPin::set_high(&mut new);
//!
//! // Apply backwards compatibility wrapper
//! let mut old = new.reverse();
//! // Access via e-h v0.2.x methods
//! let _ = eh0_2::digital::v2::OutputPin::set_high(&mut old);
//!```

#![no_std]

/// Re-export of the linked embedded-hal `v0.2.x` version for convenience
pub use eh0_2;

/// Re-export of the linked embedded-hal `v1.0.x` version for convenience
pub use eh1_0;

mod forward;
mod reverse;

// Forward compatibility wrapper trait, access using `.forward()`
pub use forward::{Forward, ForwardCompat};

// Reverse compatibility wrapper trait, access using `.reverse()`
pub use reverse::{Reverse, ReverseCompat};

/// Mock types for documentation, please ignore
pub mod mock {
    use core::convert::Infallible;

    pub struct OutputPin0_2;

    impl eh0_2::digital::v2::OutputPin for OutputPin0_2 {
        type Error = Infallible;

        /// Set the output as high
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        /// Set the output as low
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    pub struct OutputPin1_0;

    impl eh1_0::digital::ErrorType for OutputPin1_0 {
        type Error = Infallible;
    }

    impl eh1_0::digital::blocking::OutputPin for OutputPin1_0 {
        /// Set the output as high
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        /// Set the output as low
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
}
