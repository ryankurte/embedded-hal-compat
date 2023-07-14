use embedded_hal_compat::{markers::*, Forward, ForwardCompat};

#[derive(Debug)]
enum InputPinError {
    _Something,
}

impl eh1_0::digital::Error for InputPinError {
    fn kind(&self) -> eh1_0::digital::ErrorKind {
        eh1_0::digital::ErrorKind::Other
    }
}

#[derive(Debug)]
enum OutputPinError {
    _Something,
}

impl eh1_0::digital::Error for OutputPinError {
    fn kind(&self) -> eh1_0::digital::ErrorKind {
        eh1_0::digital::ErrorKind::Other
    }
}

struct IoPin;

impl eh0_2::digital::v2::OutputPin for IoPin {
    type Error = InputPinError;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl eh0_2::digital::v2::InputPin for IoPin {
    type Error = InputPinError;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

struct OutputPin;

impl eh0_2::digital::v2::OutputPin for OutputPin {
    type Error = OutputPinError;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct InputPin;

impl eh0_2::digital::v2::InputPin for InputPin {
    type Error = InputPinError;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn io_pin_forward() {
    let periph_0_2 = IoPin;
    let mut periph_1_0: Forward<_, ForwardIoPin> = periph_0_2.forward();
    assert!(eh1_0::digital::OutputPin::set_high(&mut periph_1_0).is_ok());
    assert!(eh1_0::digital::InputPin::is_high(&periph_1_0).unwrap());
}

#[test]
fn input_pin_forward() {
    let periph_0_2 = InputPin;
    let periph_1_0 = periph_0_2.forward();
    assert!(eh1_0::digital::InputPin::is_high(&periph_1_0).unwrap());
}

#[test]
fn output_pin_forward() {
    let periph_0_2 = OutputPin;
    let mut periph_1_0 = periph_0_2.forward();
    assert!(eh1_0::digital::OutputPin::set_high(&mut periph_1_0).is_ok());
}
