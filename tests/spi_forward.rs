use embedded_hal_compat::ForwardCompat;

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

impl eh0_2::blocking::spi::Write<u8> for Peripheral {
    type Error = ImplError;
    fn write(&mut self, _words: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::blocking::spi::Transfer<u8> for Peripheral {
    type Error = ImplError;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        Ok(words)
    }
}

impl eh0_2::blocking::spi::WriteIter<u8> for Peripheral {
    type Error = ImplError;
    fn write_iter<WI>(&mut self, _words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        Ok(())
    }
}

impl eh0_2::spi::FullDuplex<u8> for Peripheral {
    type Error = ImplError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Ok(0)
    }
    fn send(&mut self, _word: u8) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn can_forward() {
    let periph_0_2 = Peripheral;
    let mut periph_1_0 = periph_0_2.forward();
    assert!(eh1_0::spi::SpiBusWrite::write(&mut periph_1_0, &[]).is_ok());
    assert!(eh1_0::spi::SpiBusRead::read(&mut periph_1_0, &mut []).is_ok());
    assert!(eh1_0::spi::SpiBusFlush::flush(&mut periph_1_0).is_ok());
    assert!(eh1_0::spi::SpiBus::transfer(&mut periph_1_0, &mut [], &[]).is_ok());
    assert!(eh1_0::spi::SpiBus::transfer_in_place(&mut periph_1_0, &mut []).is_ok());
}
