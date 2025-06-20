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
    let mut green_led_2 = pins.d9.into_output();
    let mut yellow_led = pins.d8.into_output();
    let mut white_led = pins.d7.into_output();
    let cabin_lights_green_switch = pins.d2.into_pull_up_input();
    let cabin_lights_blue_switch = pins.d3.into_pull_up_input();
    let cabin_lights_red_switch = pins.d4.into_pull_up_input();

    loop {
        
        if cabin_lights_green_switch.is_high() && cabin_lights_red_switch.is_low() && cabin_lights_blue_switch.is_low() {
            green_led.set_high();
        } else {
            green_led.set_low();
        }

        if cabin_lights_blue_switch.is_high() && cabin_lights_red_switch.is_low() && cabin_lights_green_switch.is_low() {
            blue_led.set_high();
        } else {
            blue_led.set_low();
        }
        
        if cabin_lights_red_switch.is_high() && cabin_lights_blue_switch.is_low() && cabin_lights_green_switch.is_low()  {
            red_led.set_high();
        } else {
            red_led.set_low();
        }

        if cabin_lights_green_switch.is_high() && cabin_lights_blue_switch.is_high() && cabin_lights_red_switch.is_low() {
            green_led_2.set_high();
        } else {
            green_led_2.set_low();
        }
        
        if cabin_lights_green_switch.is_low() && cabin_lights_blue_switch.is_high() && cabin_lights_red_switch.is_high() {
            yellow_led.set_high();
        } else {
            yellow_led.set_low();
        }

        if cabin_lights_green_switch.is_high() && cabin_lights_blue_switch.is_low() && cabin_lights_red_switch.is_high() {
            white_led.set_high();
        } else {
            white_led.set_low();
        }
    }
}
