use eh0_2;
use eh1_0;
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

struct I2c0;

impl eh0_2::blocking::i2c::Write for I2c0 {
    type Error = ImplError;
    fn write(&mut self, _address: u8, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::Read for I2c0 {
    type Error = ImplError;
    fn read(&mut self, _address: u8, _buffer: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::WriteRead for I2c0 {
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

impl eh0_2::blocking::i2c::WriteIter for I2c0 {
    type Error = ImplError;
    fn write<B>(&mut self, _address: u8, _bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::WriteIterRead for I2c0 {
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

impl eh0_2::blocking::i2c::Transactional for I2c0 {
    type Error = ImplError;
    fn exec<'a>(
        &mut self,
        _address: u8,
        _operations: &mut [eh0_2::blocking::i2c::Operation<'a>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::i2c::TransactionalIter for I2c0 {
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
    let i2c_0_2 = I2c0;
    let mut i2c_1_0 = i2c_0_2.forward();
    assert!(eh1_0::i2c::I2c::write(&mut i2c_1_0, 0, &[]).is_ok());
    assert!(eh1_0::i2c::I2c::write_read(&mut i2c_1_0, 0, &[], &mut data).is_ok());
    assert!(eh1_0::i2c::I2c::read(&mut i2c_1_0, 0, &mut data).is_ok());
    assert!(eh1_0::i2c::I2c::transaction(&mut i2c_1_0, 0, &mut []).is_ok());
}
