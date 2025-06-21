#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use panic_halt as _;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let photo_resister = pins.a0.into_analog_input(&mut adc);

    loop {
        let light_value = photo_resister.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Light value: {}", light_value).unwrap();
        arduino_hal::delay_ms(500);
        
        if light_value < 7 {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}
