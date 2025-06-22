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

        let delay = match light_value {
            0..=100 => 500,    
            101..=300 => 300,  
            301..=500 => 150,  
            _ => 100,          
        };
        ufmt::uwriteln!(&mut serial, "Delay: {}", delay).unwrap();
        
        led.set_high();
        arduino_hal::delay_ms(delay);
        led.set_low();
        arduino_hal::delay_ms(delay);
    }
}
