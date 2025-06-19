#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut green_led = pins.d10.into_output();
    let mut blue_led = pins.d11.into_output();
    let mut red_led = pins.d12.into_output();
    let cabin_lights_green_switch = pins.d2.into_pull_up_input();
    let cabin_lights_blue_switch = pins.d3.into_pull_up_input();
    let cabin_lights_red_switch = pins.d4.into_pull_up_input();

    loop {
        
        if cabin_lights_green_switch.is_high() {
            green_led.set_high();
        } else {
            green_led.set_low();
        }

        if cabin_lights_blue_switch.is_high() {
            blue_led.set_high();
        } else {
            blue_led.set_low();
        }
        
        if cabin_lights_red_switch.is_high() {
            red_led.set_high();
        } else {
            red_led.set_low();
        }
    }
}
