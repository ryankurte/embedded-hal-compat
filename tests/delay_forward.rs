use embedded_hal_compat::ForwardCompat;

struct Peripheral;

impl eh0_2::blocking::delay::DelayMs<u32> for Peripheral {
    fn delay_ms(&mut self, _ms: u32) {}
}

impl eh0_2::blocking::delay::DelayUs<u32> for Peripheral {
    fn delay_us(&mut self, _us: u32) {}
}

#[test]
fn can_forward() {
    let periph_0_2 = Peripheral;
    let mut periph_1_0 = periph_0_2.forward();
    eh1_0::delay::DelayUs::delay_ms(&mut periph_1_0, 0);
    eh1_0::delay::DelayUs::delay_ms(&mut periph_1_0, 0);
}
