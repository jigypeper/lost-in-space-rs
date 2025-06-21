#![no_std]

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use core::prelude::v1::*;

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
