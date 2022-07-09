//! Embedded HAL Reverse compatibility shim
//! A compatibility layer to alleviate (some) of the issues resolving from changes to embedded-hal
// Copyright 2021 Ryan Kurte

use core::fmt::Debug;

/// Reverse compatibility container object.
/// This is generic over different E-H types and will provide adaption
/// depending on the bound type.
pub struct Reverse<T> {
    inner: T,
}

/// Convert a type into a forward compatibility wrapper object
/// call `.reverse()` on `e-h@1.0.x` types to create an `e-h@0.2.x` compatible wrapper object
pub trait ReverseCompat<T> {
    fn reverse(self) -> Reverse<T>;
}

impl<T> ReverseCompat<T> for T {
    /// Create an e-h-c wrapper around and e-h object
    /// Available methods depend on the wrapped type
    fn reverse(self) -> Reverse<T> {
        Reverse::new(self)
    }
}

impl<T> Reverse<T> {
    /// Create a new compatibility wrapper object
    pub fn new(inner: T) -> Reverse<T> {
        Reverse { inner }
    }

    /// Fetch a reference to the wrapped object
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Fetch a mutable reference to the wrapped object
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Destroy the compatibility wrapper, returning the wrapped object
    pub fn unwrap(self) -> T {
        self.inner
    }
}

// note that implementations over Reverse cannot be generic over word type
// etc. due to orphan rules (ie. what happens if someone else defines a word type?)

// Digital / GPIOs
mod digital {
    use super::{Debug, Reverse};

    impl<T, E> eh0_2::digital::v2::InputPin for Reverse<T>
    where
        T: eh1_0::digital::blocking::InputPin<Error = E>,
        E: Debug,
    {
        type Error = E;

        /// Is the input pin high?
        fn is_high(&self) -> Result<bool, Self::Error> {
            self.inner.is_high()
        }

        /// Is the input pin low?
        fn is_low(&self) -> Result<bool, Self::Error> {
            self.inner.is_low()
        }
    }

    impl<T, E> eh0_2::digital::v2::OutputPin for Reverse<T>
    where
        T: eh1_0::digital::blocking::OutputPin<Error = E>,
        E: Debug,
    {
        type Error = E;

        /// Set the output as high
        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.inner.set_high()
        }

        /// Set the output as low
        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.inner.set_low()
        }
    }
}

/// Delays (blocking)
mod delay {
    use super::{Debug, Reverse};

    impl<T, E> eh0_2::blocking::delay::DelayMs<u32> for Reverse<T>
    where
        T: eh1_0::delay::blocking::DelayUs<Error = E>,
        E: Debug,
    {
        fn delay_ms(&mut self, ms: u32) {
            self.inner.delay_us(ms * 1000).unwrap();
        }
    }

    impl<T, E> eh0_2::blocking::delay::DelayMs<u16> for Reverse<T>
    where
        T: eh1_0::delay::blocking::DelayUs<Error = E>,
        E: Debug,
    {
        fn delay_ms(&mut self, ms: u16) {
            self.inner.delay_us(ms as u32 * 1000).unwrap();
        }
    }

    impl<T, E> eh0_2::blocking::delay::DelayUs<u32> for Reverse<T>
    where
        T: eh1_0::delay::blocking::DelayUs<Error = E>,
        E: Debug,
    {
        fn delay_us(&mut self, us: u32) {
            self.inner.delay_us(us).unwrap();
        }
    }

    impl<T, E> eh0_2::blocking::delay::DelayUs<u16> for Reverse<T>
    where
        T: eh1_0::delay::blocking::DelayUs<Error = E>,
        E: Debug,
    {
        fn delay_us(&mut self, us: u16) {
            self.inner.delay_us(us as u32).unwrap();
        }
    }
}

/// SPI (blocking)
mod spi {
    use super::{Debug, Reverse};

    impl<T, E> eh0_2::blocking::spi::Write<u8> for Reverse<T>
    where
        T: eh1_0::spi::blocking::SpiBusWrite<u8, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(words)
        }
    }

    impl<T, E> eh0_2::blocking::spi::Transfer<u8> for Reverse<T>
    where
        T: eh1_0::spi::blocking::SpiBus<u8, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn transfer<'a>(&mut self, words: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
            self.inner.transfer_in_place(words)?;
            Ok(words)
        }
    }

    impl<T, E> eh0_2::blocking::spi::WriteIter<u8> for Reverse<T>
    where
        T: eh1_0::spi::blocking::SpiBusWrite<u8, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            for word in words.into_iter() {
                self.inner.write(&[word])?;
            }
            Ok(())
        }
    }
}

// I2C (blocking)
mod i2c {
    use super::{Debug, Reverse};
    use eh1_0::i2c::SevenBitAddress;

    impl<T, E> eh0_2::blocking::i2c::Read for Reverse<T>
    where
        T: eh1_0::i2c::blocking::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn read(&mut self, address: SevenBitAddress, words: &mut [u8]) -> Result<(), Self::Error> {
            self.inner.read(address, words)
        }
    }

    impl<T, E> eh0_2::blocking::i2c::Write for Reverse<T>
    where
        T: eh1_0::i2c::blocking::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write(&mut self, address: SevenBitAddress, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(address, words)
        }
    }

    impl<T, E> eh0_2::blocking::i2c::WriteIter for Reverse<T>
    where
        T: eh1_0::i2c::blocking::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write<B>(&mut self, address: SevenBitAddress, words: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.inner.write_iter(address, words)
        }
    }

    impl<T, E> eh0_2::blocking::i2c::WriteRead for Reverse<T>
    where
        T: eh1_0::i2c::blocking::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write_read(
            &mut self,
            address: SevenBitAddress,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.inner.write_read(address, bytes, buffer)
        }
    }

    impl<T, E> eh0_2::blocking::i2c::WriteIterRead for Reverse<T>
    where
        T: eh1_0::i2c::blocking::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write_iter_read<B>(
            &mut self,
            address: SevenBitAddress,
            bytes: B,
            buffer: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.inner.write_iter_read(address, bytes, buffer)
        }
    }
}

/// Serial (UART etc.)
mod serial {
    use super::{Debug, Reverse};

    impl<T, E> eh0_2::blocking::serial::Write<u8> for Reverse<T>
    where
        T: eh1_0::serial::blocking::Write<u8, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn bwrite_all(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(words)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.inner.flush()
        }
    }
}
