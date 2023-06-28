use embedded_hal_compat::ForwardCompat;

#[derive(Debug)]
enum ImplError {
    _Something,
}

impl eh1_0::serial::Error for ImplError {
    fn kind(&self) -> eh1_0::serial::ErrorKind {
        eh1_0::serial::ErrorKind::Other
    }
}

struct Peripheral;

impl eh0_2::blocking::serial::Write<u8> for Peripheral {
    type Error = ImplError;

    fn bwrite_all(&mut self, _buffer: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn bflush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::serial::Write<u8> for Peripheral {
    type Error = ImplError;

    fn write(&mut self, _word: u8) -> nb::Result<(), Self::Error> {
        Ok(())
    }
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn can_forward() {
    let periph_0_2 = Peripheral;
    let mut periph_1_0 = periph_0_2.forward();
    assert!(eh1_0::serial::Write::write(&mut periph_1_0, &[0]).is_ok());
    assert!(eh1_0::serial::Write::flush(&mut periph_1_0).is_ok());
}
