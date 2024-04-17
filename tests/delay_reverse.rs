use embedded_hal_compat::ReverseCompat;

struct Peripheral;

impl eh1_0::delay::DelayNs for Peripheral {
    fn delay_ns(&mut self, _ns: u32) {}
}

#[test]
fn can_reverse() {
    let periph_1_0 = Peripheral;
    let mut periph_0_2 = periph_1_0.reverse();
    eh0_2::blocking::delay::DelayMs::delay_ms(&mut periph_0_2, 0_u32);
    eh0_2::blocking::delay::DelayUs::delay_us(&mut periph_0_2, 0_u32);
}
