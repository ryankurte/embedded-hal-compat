//! Embedded HAL Forward Compat Later
//! A compatibility layer to alleviate (some) of the issues resolving from changes to embedded-hal
// Copyright 2021 Ryan Kurte

/// Forward compatibility container object.
/// This is generic over different E-H types and will provide adaption
/// depending on the bound type.
pub struct Forward<T> {
    inner: T,
}

/// Helper trait to convert a type for forward compatibility
/// call `.forward()` on `e-h@0.2.x` types to create an `e-h@1.x.x` compatible wrapper object
pub trait ForwardCompat<T> {
    fn forward(self) -> Forward<T>;
}

impl<T> ForwardCompat<T> for T {
    /// Create an e-h-c forward compatibility wrapper around and e-h object
    /// Available methods depend on the wrapped type
    fn forward(self) -> Forward<T> {
        Forward::new(self)
    }
}

impl<T> Forward<T> {
    /// Create a new compatibility wrapper object
    pub fn new(inner: T) -> Forward<T> {
        Forward { inner }
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

// note that implementations over Forward cannot be generic over word type
// etc. due to orphan rules (ie. what happens if someone else defines a word type?)

// Digital / GPIOs
mod digital {
    use super::Forward;

    impl<T, E> eh1_0::digital::blocking::InputPin for Forward<T>
    where
        T: eh0_2::digital::v2::InputPin<Error = E>,
        E: core::fmt::Debug,
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

    impl<T, E> eh1_0::digital::blocking::OutputPin for Forward<T>
    where
        T: eh0_2::digital::v2::OutputPin<Error = E>,
        E: core::fmt::Debug,
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
    use super::Forward;
    use core::convert::Infallible;

    impl<T> eh1_0::delay::blocking::DelayMs<u32> for Forward<T>
    where
        T: eh0_2::blocking::delay::DelayMs<u32>,
    {
        type Error = Infallible;

        fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
            self.inner.delay_ms(ms);
            Ok(())
        }
    }

    impl<T> eh1_0::delay::blocking::DelayMs<u16> for Forward<T>
    where
        T: eh0_2::blocking::delay::DelayMs<u16>,
    {
        type Error = Infallible;

        fn delay_ms(&mut self, ms: u16) -> Result<(), Self::Error> {
            self.inner.delay_ms(ms);
            Ok(())
        }
    }

    impl<T> eh1_0::delay::blocking::DelayUs<u32> for Forward<T>
    where
        T: eh0_2::blocking::delay::DelayUs<u32>,
    {
        type Error = Infallible;

        fn delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
            self.inner.delay_us(us);
            Ok(())
        }
    }

    impl<T> eh1_0::delay::blocking::DelayUs<u16> for Forward<T>
    where
        T: eh0_2::blocking::delay::DelayUs<u16>,
    {
        type Error = Infallible;

        fn delay_us(&mut self, us: u16) -> Result<(), Self::Error> {
            self.inner.delay_us(us);
            Ok(())
        }
    }
}

/// SPI (blocking)
mod spi {
    use super::Forward;

    impl<T, E> eh1_0::spi::blocking::Write<u8> for Forward<T>
    where
        T: eh0_2::blocking::spi::Write<u8, Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(words)
        }
    }

    impl<T, E> eh1_0::spi::blocking::Transfer<u8> for Forward<T>
    where
        T: eh0_2::blocking::spi::Transfer<u8, Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn transfer(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
            self.inner.transfer(words)?;
            Ok(())
        }
    }

    impl<T, E> eh1_0::spi::blocking::WriteIter<u8> for Forward<T>
    where
        T: eh0_2::blocking::spi::WriteIter<u8, Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            self.inner.write_iter(words)
        }
    }

    impl<T, E> eh1_0::spi::blocking::Transactional<u8> for Forward<T>
    where
        T: eh0_2::blocking::spi::Write<u8, Error = E>
            + eh0_2::blocking::spi::Transfer<u8, Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn exec<'a>(
            &mut self,
            operations: &mut [eh1_0::spi::blocking::Operation<'a, u8>],
        ) -> Result<(), Self::Error> {
            use eh1_0::spi::blocking::Operation;

            for op in operations {
                match op {
                    Operation::Write(w) => self.inner.write(w)?,
                    Operation::Transfer(t) => self.inner.transfer(t).map(|_| ())?,
                }
            }

            Ok(())
        }
    }
}

// I2C (blocking)
mod i2c {
    use super::Forward;
    use eh1_0::i2c::SevenBitAddress;

    impl<T, E> eh1_0::i2c::blocking::Read<SevenBitAddress> for Forward<T>
    where
        T: eh0_2::blocking::i2c::Read<Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn read(&mut self, address: SevenBitAddress, words: &mut [u8]) -> Result<(), Self::Error> {
            self.inner.read(address, words)
        }
    }

    impl<T, E> eh1_0::i2c::blocking::Write<SevenBitAddress> for Forward<T>
    where
        T: eh0_2::blocking::i2c::Write<Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn write(&mut self, address: SevenBitAddress, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(address, words)
        }
    }

    impl<T, E> eh1_0::i2c::blocking::WriteIter<SevenBitAddress> for Forward<T>
    where
        T: eh0_2::blocking::i2c::WriteIter<Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn write_iter<B>(&mut self, address: SevenBitAddress, words: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.inner.write(address, words)
        }
    }

    impl<T, E> eh1_0::i2c::blocking::WriteRead<SevenBitAddress> for Forward<T>
    where
        T: eh0_2::blocking::i2c::WriteRead<Error = E>,
        E: core::fmt::Debug,
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

    impl<T, E> eh1_0::i2c::blocking::WriteIterRead<SevenBitAddress> for Forward<T>
    where
        T: eh0_2::blocking::i2c::WriteIterRead<Error = E>,
        E: core::fmt::Debug,
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
    use super::Forward;

    impl<T, E> eh1_0::serial::blocking::Write<u8> for Forward<T>
    where
        T: eh0_2::blocking::serial::Write<u8, Error = E>,
        E: core::fmt::Debug,
    {
        type Error = E;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.bwrite_all(words)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.inner.bflush()
        }
    }
}
