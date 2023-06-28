use embedded_hal_compat::ReverseCompat;

#[derive(Debug)]
enum ImplError {
    _Something,
}

impl eh1_0::i2c::Error for ImplError {
    fn kind(&self) -> eh1_0::i2c::ErrorKind {
        eh1_0::i2c::ErrorKind::Other
    }
}

struct I2c0;

impl eh1_0::i2c::ErrorType for I2c0 {
    type Error = ImplError;
}

impl eh1_0::i2c::I2c for I2c0 {
    fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [eh1_0::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn can_reverse() {
    let mut data = [];
    let i2c_1_0 = I2c0;
    let mut i2c_0_2 = i2c_1_0.reverse();
    assert!(eh0_2::blocking::i2c::Write::write(&mut i2c_0_2, 0, &[]).is_ok());
    assert!(eh0_2::blocking::i2c::WriteRead::write_read(&mut i2c_0_2, 0, &[], &mut data).is_ok());
    assert!(eh0_2::blocking::i2c::Read::read(&mut i2c_0_2, 0, &mut data).is_ok());
}
