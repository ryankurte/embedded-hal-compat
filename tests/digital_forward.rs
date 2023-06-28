use embedded_hal_compat::ForwardCompat;

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

impl eh0_2::digital::v2::OutputPin for Peripheral {
    type Error = PinError;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::digital::v2::InputPin for Peripheral {
    type Error = PinError;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn can_forward() {
    let periph_0_2 = Peripheral;
    let mut periph_1_0 = periph_0_2.forward();
    assert!(eh1_0::digital::OutputPin::set_high(&mut periph_1_0).is_ok());
    assert!(eh1_0::digital::InputPin::is_high(&periph_1_0).unwrap());
}
