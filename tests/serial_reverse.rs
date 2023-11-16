#[cfg(feature = "embedded-io")]
mod optional {
    use embedded_hal_compat::ReverseCompat;

    #[derive(Debug)]
    enum ImplError {
        _Something,
    }

    impl embedded_io::Error for ImplError {
        fn kind(&self) -> embedded_io::ErrorKind {
            embedded_io::ErrorKind::Other
        }
    }

    struct Peripheral;

    impl embedded_io::ErrorType for Peripheral {
        type Error = ImplError;
    }

    impl embedded_io::Write for Peripheral {
        fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
            Ok(buffer.len())
        }
        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn can_reverse() {
        let periph_1_0 = Peripheral;
        let mut periph_0_2 = periph_1_0.reverse();
        assert!(eh0_2::blocking::serial::Write::bflush(&mut periph_0_2).is_ok());
        assert!(eh0_2::blocking::serial::Write::bwrite_all(&mut periph_0_2, &[]).is_ok());
        assert!(eh0_2::serial::Write::write(&mut periph_0_2, 0).is_ok());
        assert!(eh0_2::serial::Write::flush(&mut periph_0_2).is_ok());
    }
}
