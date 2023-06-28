use embedded_hal_compat::ReverseCompat;

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

impl eh1_0::serial::ErrorType for Peripheral {
    type Error = ImplError;
}

impl eh1_0::serial::Write<u8> for Peripheral {
    fn write(&mut self, _buffer: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn can_reverse() {
    let p_1_0 = Peripheral;
    let mut p_0_2 = p_1_0.reverse();
    assert!(eh0_2::blocking::serial::Write::bflush(&mut p_0_2).is_ok());
    assert!(eh0_2::blocking::serial::Write::bwrite_all(&mut p_0_2, &[]).is_ok());
    assert!(eh0_2::serial::Write::write(&mut p_0_2, 0).is_ok());
    assert!(eh0_2::serial::Write::flush(&mut p_0_2).is_ok());
}
