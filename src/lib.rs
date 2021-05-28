//! Embedded HAL Compat
//! A compatibility layer to alleviate (some) of the issues resolving from changes to embedded-hal
// Copyright 2021 Ryan Kurte

#![no_std]

pub use eh0_2 as eh0_2;
pub use eh1_0 as eh1_0;

pub mod forward;
pub mod reverse;
