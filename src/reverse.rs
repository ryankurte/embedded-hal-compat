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
        T: eh1_0::digital::InputPin<Error = E>,
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
        T: eh1_0::digital::OutputPin<Error = E>,
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
    use super::Reverse;

    impl<T> eh0_2::blocking::delay::DelayMs<u32> for Reverse<T>
    where
        T: eh1_0::delay::DelayUs,
    {
        fn delay_ms(&mut self, ms: u32) {
            self.inner.delay_us(ms * 1000)
        }
    }

    impl<T> eh0_2::blocking::delay::DelayMs<u16> for Reverse<T>
    where
        T: eh1_0::delay::DelayUs,
    {
        fn delay_ms(&mut self, ms: u16) {
            self.inner.delay_us(ms as u32 * 1000)
        }
    }

    impl<T> eh0_2::blocking::delay::DelayUs<u32> for Reverse<T>
    where
        T: eh1_0::delay::DelayUs,
    {
        fn delay_us(&mut self, us: u32) {
            self.inner.delay_us(us)
        }
    }

    impl<T> eh0_2::blocking::delay::DelayUs<u16> for Reverse<T>
    where
        T: eh1_0::delay::DelayUs,
    {
        fn delay_us(&mut self, us: u16) {
            self.inner.delay_us(us as u32)
        }
    }
}

/// SPI (blocking)
mod spi {
    use super::{Debug, Reverse};

    impl<T, E> eh0_2::blocking::spi::Write<u8> for Reverse<T>
    where
        T: eh1_0::spi::SpiBus<u8, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(words)
        }
    }

    impl<T, E> eh0_2::blocking::spi::Transfer<u8> for Reverse<T>
    where
        T: eh1_0::spi::SpiBus<u8, Error = E>,
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
        T: eh1_0::spi::SpiBus<u8, Error = E>,
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

    impl<T, E> eh0_2::spi::FullDuplex<u8> for Reverse<T>
    where
        T: eh1_0::spi::SpiBus<u8, Error = E>,
        E: Debug,
    {
        type Error = E;
        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            let mut data = [0];
            match self.inner.read(&mut data) {
                Ok(_) => Ok(data[0]),
                Err(e) => Err(nb::Error::Other(e)),
            }
        }
        fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
            self.inner.write(&[word]).map_err(nb::Error::Other)
        }
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    extern crate alloc;
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<T, E> eh0_2::blocking::spi::Transactional<u8> for Reverse<T>
    where
        T: eh1_0::spi::SpiBus<Error = E>,
        E: Debug,
    {
        type Error = E;

        fn exec(
            &mut self,
            operations: &mut [eh0_2::blocking::spi::Operation<u8>],
        ) -> Result<(), Self::Error> {
            let mut data = alloc::vec::Vec::<u8>::new();
            for op in operations.iter() {
                match op {
                    eh0_2::blocking::spi::Operation::Transfer(buf) => data.extend_from_slice(buf),
                    eh0_2::blocking::spi::Operation::Write(buf) => data.extend_from_slice(buf),
                }
            }
            let result = self.inner.transfer_in_place(&mut data);
            let mut data_iter = data.iter();
            for op in operations.iter_mut() {
                match op {
                    eh0_2::blocking::spi::Operation::Transfer(buf) => {
                        buf.copy_from_slice(&data_iter.as_slice()[0..buf.len()]);
                        if !buf.is_empty() {
                            data_iter.nth(buf.len() - 1);
                        }
                    }
                    eh0_2::blocking::spi::Operation::Write(buf) => {
                        if !buf.is_empty() {
                            data_iter.nth(buf.len() - 1);
                        }
                    }
                }
            }
            result
        }
    }
}

// I2C (blocking)
mod i2c {
    use super::{Debug, Reverse};
    use eh1_0::i2c::SevenBitAddress;

    impl<T, E> eh0_2::blocking::i2c::Read for Reverse<T>
    where
        T: eh1_0::i2c::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn read(&mut self, address: SevenBitAddress, words: &mut [u8]) -> Result<(), Self::Error> {
            self.inner.read(address, words)
        }
    }

    impl<T, E> eh0_2::blocking::i2c::Write for Reverse<T>
    where
        T: eh1_0::i2c::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write(&mut self, address: SevenBitAddress, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(address, words)
        }
    }

    impl<T, E> eh0_2::blocking::i2c::WriteRead for Reverse<T>
    where
        T: eh1_0::i2c::I2c<SevenBitAddress, Error = E>,
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

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    extern crate alloc;
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<T, E> eh0_2::blocking::i2c::Transactional for Reverse<T>
    where
        T: eh1_0::i2c::I2c<SevenBitAddress, Error = E>,
        E: Debug,
    {
        type Error = E;

        fn exec(
            &mut self,
            address: u8,
            operations: &mut [eh0_2::blocking::i2c::Operation],
        ) -> Result<(), Self::Error> {
            let mut ops: alloc::vec::Vec<eh1_0::i2c::Operation> = operations
                .iter_mut()
                .map(|op| match op {
                    eh0_2::blocking::i2c::Operation::Read(ref mut buff) => {
                        eh1_0::i2c::Operation::Read(buff)
                    }
                    eh0_2::blocking::i2c::Operation::Write(buff) => {
                        eh1_0::i2c::Operation::Write(buff)
                    }
                })
                .collect();
            self.inner.transaction(address, &mut ops)
        }
    }
}

/// Serial (UART etc.)
#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
mod serial {
    use super::{Debug, Reverse};

    impl<T, E> eh0_2::blocking::serial::Write<u8> for Reverse<T>
    where
        T: embedded_io::Write<Error = E>,
        E: Debug,
    {
        type Error = E;

        fn bwrite_all(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(words).map(drop)
        }

        fn bflush(&mut self) -> Result<(), Self::Error> {
            self.inner.flush()
        }
    }

    impl<T, E> eh0_2::serial::Write<u8> for Reverse<T>
    where
        T: embedded_io::Write<Error = E>,
        E: Debug,
    {
        type Error = E;

        fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
            self.inner
                .write(&[word])
                .map_err(nb::Error::Other)
                .map(drop)
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.inner.flush().map_err(nb::Error::Other)
        }
    }
}
