//! Embedded HAL Forward Compat Later
//! A compatibility layer to alleviate (some) of the issues resolving from changes to embedded-hal
// Copyright 2021 Ryan Kurte

use core::marker::PhantomData;

/// Forward compatibility container object.
/// This is generic over different E-H types and will provide adaption
/// depending on the bound type.
pub struct Forward<T, M = ()> {
    inner: T,
    _marker: PhantomData<M>,
}

/// Helper trait to convert a type for forward compatibility
/// call `.forward()` on `e-h@0.2.x` types to create an `e-h@1.x.x` compatible wrapper object
pub trait ForwardCompat<T, M = ()> {
    /// Create an e-h-c forward compatibility wrapper around and e-h object
    /// Available methods depend on the wrapped type
    fn forward(self) -> Forward<T, M>;
}

/// Blanket [ForwardCompat] implementation
/// (note input/output/io pins may require type annotations)
impl<T, M> ForwardCompat<T, M> for T {
    fn forward(self) -> Forward<T, M> {
        Forward::new(self)
    }
}

impl<T, M> Forward<T, M> {
    /// Create a new compatibility wrapper object
    pub fn new(inner: T) -> Forward<T, M> {
        Forward {
            inner,
            _marker: PhantomData,
        }
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

/// Fake SPI error type for forward compatibility.
///
/// This fulfils error trait bounds but `.kind()` always returns `Other`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ForwardError<E>(pub E);

// note that implementations over Forward cannot be generic over word type
// etc. due to orphan rules (ie. what happens if someone else defines a word type?)

// Digital / GPIOs
mod digital {
    use super::{Forward, ForwardError};
    use crate::markers::{ForwardInputPin, ForwardIoPin, ForwardOutputPin};

    impl<E: core::fmt::Debug> eh1_0::digital::Error for ForwardError<E> {
        fn kind(&self) -> eh1_0::digital::ErrorKind {
            eh1_0::digital::ErrorKind::Other
        }
    }

    impl<T, E> eh1_0::digital::ErrorType for Forward<T, ForwardInputPin>
    where
        T: eh0_2::digital::v2::InputPin<Error = E>,
        E: core::fmt::Debug,
    {
        type Error = super::ForwardError<E>;
    }

    impl<T, E> eh1_0::digital::InputPin for Forward<T, ForwardInputPin>
    where
        T: eh0_2::digital::v2::InputPin<Error = E>,
        E: core::fmt::Debug,
    {
        /// Is the input pin high?
        fn is_high(&self) -> Result<bool, Self::Error> {
            self.inner.is_high().map_err(ForwardError)
        }

        /// Is the input pin low?
        fn is_low(&self) -> Result<bool, Self::Error> {
            self.inner.is_low().map_err(ForwardError)
        }
    }

    impl<T, E> eh1_0::digital::ErrorType for Forward<T, ForwardOutputPin>
    where
        T: eh0_2::digital::v2::OutputPin<Error = E>,
        E: core::fmt::Debug,
    {
        type Error = super::ForwardError<E>;
    }

    impl<T, E> eh1_0::digital::OutputPin for Forward<T, ForwardOutputPin>
    where
        T: eh0_2::digital::v2::OutputPin<Error = E>,
        E: core::fmt::Debug,
    {
        /// Set the output as high
        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.inner.set_high().map_err(ForwardError)
        }

        /// Set the output as low
        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.inner.set_low().map_err(ForwardError)
        }
    }

    impl<T, E> eh1_0::digital::ErrorType for Forward<T, ForwardIoPin>
    where
        T: eh0_2::digital::v2::OutputPin<Error = E> + eh0_2::digital::v2::InputPin<Error = E>,
        E: core::fmt::Debug,
    {
        type Error = super::ForwardError<E>;
    }

    impl<T, E> eh1_0::digital::InputPin for Forward<T, ForwardIoPin>
    where
        T: eh0_2::digital::v2::InputPin<Error = E> + eh0_2::digital::v2::OutputPin<Error = E>,
        E: core::fmt::Debug,
    {
        /// Is the input pin high?
        fn is_high(&self) -> Result<bool, Self::Error> {
            self.inner.is_high().map_err(ForwardError)
        }

        /// Is the input pin low?
        fn is_low(&self) -> Result<bool, Self::Error> {
            self.inner.is_low().map_err(ForwardError)
        }
    }

    impl<T, E> eh1_0::digital::OutputPin for Forward<T, ForwardIoPin>
    where
        T: eh0_2::digital::v2::InputPin<Error = E> + eh0_2::digital::v2::OutputPin<Error = E>,
        E: core::fmt::Debug,
    {
        /// Set the output as high
        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.inner.set_high().map_err(ForwardError)
        }

        /// Set the output as low
        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.inner.set_low().map_err(ForwardError)
        }
    }
}

/// Delays (blocking)
mod delay {
    use super::Forward;

    impl<T> eh1_0::delay::DelayUs for Forward<T>
    where
        T: eh0_2::blocking::delay::DelayUs<u32>,
    {
        fn delay_us(&mut self, us: u32) {
            self.inner.delay_us(us)
        }
    }
}

/// SPI (blocking)
mod spi {
    use super::{Forward, ForwardError};

    impl<E: core::fmt::Debug> eh1_0::spi::Error for ForwardError<E> {
        fn kind(&self) -> eh1_0::spi::ErrorKind {
            eh1_0::spi::ErrorKind::Other
        }
    }

    impl<T, E> eh1_0::spi::ErrorType for Forward<T>
    where
        T: eh0_2::blocking::spi::Transfer<u8, Error = E>,
        E: core::fmt::Debug,
    {
        type Error = ForwardError<E>;
    }

    impl<T, E> eh1_0::spi::SpiBus<u8> for Forward<T>
    where
        T: eh0_2::blocking::spi::Transfer<u8, Error = E>
            + eh0_2::blocking::spi::Write<u8, Error = E>,
        E: core::fmt::Debug,
    {
        fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
            // Clear buffer to send empty bytes
            for word in words.iter_mut() {
                *word = 0x00;
            }
            self.inner.transfer(words).map_err(ForwardError)?;
            Ok(())
        }

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(words).map_err(ForwardError)
        }

        fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
            read.copy_from_slice(&write[..read.len()]);
            self.inner.transfer(read).map_err(ForwardError)?;
            Ok(())
        }

        fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
            self.inner.transfer(words).map_err(ForwardError)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            // TODO: This API doesn't exist in 0.2.7
            Ok(())
        }
    }
}

// I2C (blocking)
mod i2c {
    use super::{Forward, ForwardError};
    use eh1_0::i2c::SevenBitAddress;

    use eh0_2::blocking::i2c::{self as eh0_2_i2c};

    impl<E: core::fmt::Debug> eh1_0::i2c::Error for ForwardError<E> {
        fn kind(&self) -> eh1_0::i2c::ErrorKind {
            eh1_0::i2c::ErrorKind::Other
        }
    }

    impl<T, E> eh1_0::i2c::ErrorType for Forward<T>
    where
        T: eh0_2::blocking::i2c::Read<Error = E>,
        E: core::fmt::Debug,
    {
        type Error = ForwardError<E>;
    }

    impl<T, E> eh1_0::i2c::I2c<SevenBitAddress> for Forward<T>
    where
        T: eh0_2_i2c::Write<Error = E>
            + eh0_2_i2c::WriteIter<Error = E>
            + eh0_2_i2c::Read<Error = E>
            + eh0_2_i2c::WriteRead<Error = E>
            + eh0_2_i2c::WriteIterRead<Error = E>
            + eh0_2_i2c::Transactional<Error = E>
            + eh0_2_i2c::TransactionalIter<Error = E>,
        E: core::fmt::Debug,
    {
        fn read(&mut self, address: SevenBitAddress, words: &mut [u8]) -> Result<(), Self::Error> {
            self.inner.read(address, words).map_err(ForwardError)
        }

        fn write(&mut self, address: SevenBitAddress, words: &[u8]) -> Result<(), Self::Error> {
            eh0_2_i2c::Write::write(&mut self.inner, address, words).map_err(ForwardError)
        }

        fn write_read(
            &mut self,
            address: SevenBitAddress,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.inner
                .write_read(address, bytes, buffer)
                .map_err(ForwardError)
        }

        fn transaction(
            &mut self,
            address: SevenBitAddress,
            operations: &mut [eh1_0::i2c::Operation],
        ) -> Result<(), Self::Error> {
            let ops = operations.iter_mut().map(|op| match op {
                eh1_0::i2c::Operation::Read(ref mut buff) => {
                    eh0_2::blocking::i2c::Operation::Read(buff)
                }
                eh1_0::i2c::Operation::Write(buff) => eh0_2::blocking::i2c::Operation::Write(buff),
            });

            self.inner.exec_iter(address, ops).map_err(ForwardError)
        }
    }
}

/// Serial (UART etc.)
#[cfg(feature = "embedded-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "embedded-io")))]
mod serial {
    use super::{Forward, ForwardError};

    impl<E: core::fmt::Debug> embedded_io::Error for ForwardError<E> {
        fn kind(&self) -> embedded_io::ErrorKind {
            embedded_io::ErrorKind::Other
        }
    }

    impl<T, E> embedded_io::ErrorType for Forward<T>
    where
        T: eh0_2::blocking::serial::Write<u8, Error = E>,
        E: core::fmt::Debug,
    {
        type Error = ForwardError<E>;
    }

    impl<T, E> embedded_io::Write for Forward<T>
    where
        T: eh0_2::blocking::serial::Write<u8, Error = E>,
        E: core::fmt::Debug,
    {
        fn write(&mut self, words: &[u8]) -> Result<usize, Self::Error> {
            self.inner
                .bwrite_all(words)
                .map_err(ForwardError)
                .and(Ok(words.len()))
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.inner.bflush().map_err(ForwardError)
        }
    }
}
