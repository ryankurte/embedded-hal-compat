use embedded_hal_compat::ForwardCompat;

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

impl eh0_2::blocking::i2c::Write for Peripheral {
    type Error = ImplError;
    fn write(&mut self, _address: u8, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::Read for Peripheral {
    type Error = ImplError;
    fn read(&mut self, _address: u8, _buffer: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::WriteRead for Peripheral {
    type Error = ImplError;
    fn write_read(
        &mut self,
        _address: u8,
        _bytes: &[u8],
        _buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::WriteIter for Peripheral {
    type Error = ImplError;
    fn write<B>(&mut self, _address: u8, _bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::WriteIterRead for Peripheral {
    type Error = ImplError;
    fn write_iter_read<B>(
        &mut self,
        _address: u8,
        _bytes: B,
        _buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::Transactional for Peripheral {
    type Error = ImplError;
    fn exec(
        &mut self,
        _address: u8,
        _operations: &mut [eh0_2::blocking::i2c::Operation],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::TransactionalIter for Peripheral {
    type Error = ImplError;
    fn exec_iter<'a, O>(&mut self, _address: u8, _operations: O) -> Result<(), Self::Error>
    where
        O: IntoIterator<Item = eh0_2::blocking::i2c::Operation<'a>>,
    {
        Ok(())
    }
}

#[test]
fn can_forward() {
    let mut data = [];
    let periph_0_2 = Peripheral;
    let mut periph_1_0 = periph_0_2.forward();
    assert!(eh1_0::i2c::I2c::write(&mut periph_1_0, 0, &[]).is_ok());
    assert!(eh1_0::i2c::I2c::write_read(&mut periph_1_0, 0, &[], &mut data).is_ok());
    assert!(eh1_0::i2c::I2c::read(&mut periph_1_0, 0, &mut data).is_ok());
    assert!(eh1_0::i2c::I2c::transaction(&mut periph_1_0, 0, &mut []).is_ok());
}
