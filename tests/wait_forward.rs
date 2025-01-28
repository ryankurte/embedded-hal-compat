use embedded_hal_compat::ForwardCompat;
use futures::executor::block_on;

#[derive(Debug)]
enum InputPinError {
    _Something,
}

impl eh1_0::digital::Error for InputPinError {
    fn kind(&self) -> eh1_0::digital::ErrorKind {
        eh1_0::digital::ErrorKind::Other
    }
}

// An InputPin from embedded-hal 0.2, with a Wait implementation from embedded-hal 1.0 on it.
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

impl eh1_0::digital::ErrorType for InputPin {
    type Error = InputPinError;
}

impl eh1_0_async::digital::Wait for InputPin {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn input_pin_forward() {
    let periph_0_2 = InputPin;
    let mut periph_1_0 = periph_0_2.forward();
    assert!(eh1_0::digital::InputPin::is_high(&mut periph_1_0).unwrap());
    assert!(block_on(async {
        eh1_0_async::digital::Wait::wait_for_any_edge(&mut periph_1_0).await
    })
    .is_ok());
}
