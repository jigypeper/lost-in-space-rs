#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use panic_halt as _;

const BATTERY_CAPACITY: u16 = 50000;

fn charge_battery(charge_level: &mut u16, increment: u16) -> u16 {
    if *charge_level < BATTERY_CAPACITY {
        *charge_level += increment;
        ((*charge_level as u32 * 100) / BATTERY_CAPACITY as u32) as u16
    } else {
        100
    }
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let photo_resister = pins.a0.into_analog_input(&mut adc);
    let mut battery_charge_level: u16 = 0;

    loop {
        let light_value = photo_resister.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Light value: {}", light_value).unwrap();

        let (delay, increment): (u32, u16) = match light_value {
            0..=100 => (500, 10),    
            101..=300 => (300, 20),  
            301..=500 => (150, 30),  
            _ => (100, 500),          
        };

        ufmt::uwriteln!(&mut serial, "Charge level: {}", battery_charge_level).unwrap();
        let percentage = charge_battery(&mut battery_charge_level, increment);
        ufmt::uwriteln!(&mut serial, "Charge percentage: {}%", percentage).unwrap();

        led.set_high();
        arduino_hal::delay_ms(delay);
        led.set_low();
        arduino_hal::delay_ms(delay);
    }
}
