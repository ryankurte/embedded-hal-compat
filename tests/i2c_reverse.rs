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

struct Peripheral;

impl eh1_0::i2c::ErrorType for Peripheral {
    type Error = ImplError;
}

impl eh1_0::i2c::I2c for Peripheral {
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
    let periph_1_0 = Peripheral;
    let mut periph_0_2 = periph_1_0.reverse();
    assert!(eh0_2::blocking::i2c::Write::write(&mut periph_0_2, 0, &[]).is_ok());
    assert!(
        eh0_2::blocking::i2c::WriteRead::write_read(&mut periph_0_2, 0, &[], &mut data).is_ok()
    );
    assert!(eh0_2::blocking::i2c::Read::read(&mut periph_0_2, 0, &mut data).is_ok());
    #[cfg(feature = "alloc")]
    assert!(eh0_2::blocking::i2c::Transactional::exec(&mut periph_0_2, 0, &mut []).is_ok());
}
