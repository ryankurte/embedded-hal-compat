use embedded_hal_compat::ReverseCompat;

#[derive(Debug)]
enum PinError {
    _Something,
}

impl eh1_0::digital::Error for PinError {
    fn kind(&self) -> eh1_0::digital::ErrorKind {
        eh1_0::digital::ErrorKind::Other
    }
}

struct Peripheral;

impl eh1_0::digital::ErrorType for Peripheral {
    type Error = PinError;
}

impl eh1_0::digital::OutputPin for Peripheral {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh1_0::digital::InputPin for Peripheral {
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn can_reverse() {
    let periph_1_0 = Peripheral;
    let mut periph_0_2 = periph_1_0.reverse();
    assert!(eh0_2::digital::v2::OutputPin::set_high(&mut periph_0_2).is_ok());
    assert!(eh0_2::digital::v2::InputPin::is_high(&periph_0_2).unwrap());
}
