use embedded_hal_compat::ReverseCompat;

#[derive(Debug)]
enum ImplError {
    _Something,
}

impl eh1_0::spi::Error for ImplError {
    fn kind(&self) -> eh1_0::spi::ErrorKind {
        eh1_0::spi::ErrorKind::Other
    }
}

struct Peripheral;

impl eh1_0::spi::ErrorType for Peripheral {
    type Error = ImplError;
}

impl eh1_0::spi::SpiBusRead for Peripheral {
    fn read(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh1_0::spi::SpiBusWrite for Peripheral {
    fn write(&mut self, _words: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh1_0::spi::SpiBusFlush for Peripheral {
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh1_0::spi::SpiBus for Peripheral {
    fn transfer(&mut self, _read: &mut [u8], _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn transfer_in_place(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn can_reverse() {
    let data = [];
    let periph_1_0 = Peripheral;
    let mut periph_0_2 = periph_1_0.reverse();
    assert!(eh0_2::blocking::spi::Write::write(&mut periph_0_2, &[]).is_ok());
    assert!(eh0_2::blocking::spi::Transfer::transfer(&mut periph_0_2, &mut []).is_ok());
    assert!(eh0_2::blocking::spi::WriteIter::write_iter(&mut periph_0_2, data).is_ok());
    assert!(eh0_2::spi::FullDuplex::send(&mut periph_0_2, 0).is_ok());
    assert_eq!(eh0_2::spi::FullDuplex::read(&mut periph_0_2).unwrap(), 0);
}
