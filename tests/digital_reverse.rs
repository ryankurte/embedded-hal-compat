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

struct Pin0;

impl eh1_0::digital::ErrorType for Pin0 {
    type Error = PinError;
}

impl eh1_0::digital::OutputPin for Pin0 {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh1_0::digital::InputPin for Pin0 {
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn can_reverse() {
    let p_1_0 = Pin0;
    let mut p_0_2 = p_1_0.reverse();
    assert!(eh0_2::digital::v2::OutputPin::set_high(&mut p_0_2).is_ok());
    assert!(eh0_2::digital::v2::InputPin::is_high(&p_0_2).unwrap());
}
