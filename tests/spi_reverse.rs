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

impl eh1_0::spi::SpiBus for Peripheral {
    fn read(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn write(&mut self, _words: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn transfer(&mut self, _read: &mut [u8], _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn transfer_in_place(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
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

#[cfg(feature = "alloc")]
#[test]
fn can_perform_trasaction_reverse() {
    let mut read0 = [2; 3];
    let mut read1 = [4; 3];
    let periph_1_0 = Peripheral;
    let mut periph_0_2 = periph_1_0.reverse();

    let mut ops = [
        eh0_2::blocking::spi::Operation::Write(&[1; 5]),
        eh0_2::blocking::spi::Operation::Transfer(&mut read0),
        eh0_2::blocking::spi::Operation::Write(&[3; 7]),
        eh0_2::blocking::spi::Operation::Transfer(&mut read1),
    ];

    assert!(eh0_2::blocking::spi::Transactional::exec(&mut periph_0_2, &mut ops).is_ok());

    if let eh0_2::blocking::spi::Operation::Write(buf) = ops[0] {
        assert_eq!(buf, [1; 5]);
    } else {
        panic!();
    }
    if let eh0_2::blocking::spi::Operation::Transfer(ref buf) = ops[1] {
        assert_eq!(*buf, [2; 3]);
    } else {
        panic!();
    }
    if let eh0_2::blocking::spi::Operation::Write(buf) = ops[2] {
        assert_eq!(buf, [3; 7]);
    } else {
        panic!();
    }
    if let eh0_2::blocking::spi::Operation::Transfer(ref buf) = ops[3] {
        assert_eq!(*buf, [4; 3]);
    } else {
        panic!();
    }
}
