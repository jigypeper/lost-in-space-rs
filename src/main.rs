#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use panic_halt as _;

#[derive(PartialEq)]
enum SwitchState {
    GreenOnly,
    BlueOnly,
    RedOnly,
    GreenBlue,
    BlueRed,
    GreenRed,
    Off,
}

struct SwitchController {
    state: SwitchState,
}

impl SwitchController {
    fn new() -> Self {
        Self {
            state: SwitchState::Off,
        }
    }

    fn read_switches<G, B, R>(&mut self, green: &mut G, blue: &mut B, red: &mut R)
    where
        G: InputPin,
        B: InputPin,
        R: InputPin,
    {
        let green_high = green.is_high().unwrap();
        let blue_high = blue.is_high().unwrap();
        let red_high = red.is_high().unwrap();

        self.state = match (green_high, blue_high, red_high) {
            (true, false, false) => SwitchState::GreenOnly,
            (false, true, false) => SwitchState::BlueOnly,
            (false, false, true) => SwitchState::RedOnly,
            (true, true, false) => SwitchState::GreenBlue,
            (false, true, true) => SwitchState::BlueRed,
            (true, false, true) => SwitchState::GreenRed,
            _ => SwitchState::Off,
        };
    }

    fn control_leds<GL, BL, RL, GL2, YL, WL>(&self, 
        green_led: &mut GL,
        blue_led: &mut BL,
        red_led: &mut RL,
        green_led_2: &mut GL2,
        yellow_led: &mut YL,
        white_led: &mut WL,
    )
    where
        GL: OutputPin,
        BL: OutputPin,
        RL: OutputPin,
        GL2: OutputPin,
        YL: OutputPin,
        WL: OutputPin,
    {
        let _ = green_led.set_low();
        let _ = blue_led.set_low();
        let _ = red_led.set_low();
        let _ = green_led_2.set_low();
        let _ = yellow_led.set_low();
        let _ = white_led.set_low();

        match self.state {
            SwitchState::GreenOnly => { let _ = green_led.set_high(); },
            SwitchState::BlueOnly => { let _ = blue_led.set_high(); },
            SwitchState::RedOnly => { let _ = red_led.set_high(); },
            SwitchState::GreenBlue => { let _ = green_led_2.set_high(); },
            SwitchState::BlueRed => { let _ = yellow_led.set_high(); },
            SwitchState::GreenRed => { let _ = white_led.set_high(); },
            SwitchState::Off => {},
        }
    }
}


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
    let mut cabin_lights_green_switch = pins.d2.into_pull_up_input();
    let mut cabin_lights_blue_switch = pins.d3.into_pull_up_input();
    let mut cabin_lights_red_switch = pins.d4.into_pull_up_input();

    let mut switch_controller = SwitchController::new();

    loop {
        switch_controller.read_switches(
            &mut cabin_lights_green_switch,
            &mut cabin_lights_blue_switch,
            &mut cabin_lights_red_switch,
        );

        switch_controller.control_leds(
            &mut green_led,
            &mut blue_led,
            &mut red_led,
            &mut green_led_2,
            &mut yellow_led,
            &mut white_led,
        );
    }
}
