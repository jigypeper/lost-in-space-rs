#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    
    // Configure keypad pins - rows as input with pullup, columns as output
    let row_pins = [
        pins.d2.into_pull_up_input().downgrade(),
        pins.d3.into_pull_up_input().downgrade(),
        pins.d4.into_pull_up_input().downgrade(),
        pins.d5.into_pull_up_input().downgrade(),
    ];
    
    let mut col_pins = [
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
        pins.d9.into_output().downgrade(),
    ];
    
    // Set all columns high initially
    for col in &mut col_pins {
        col.set_high();
    }
    
    let key_chars = [
        ['1', '2', '3', 'A'],
        ['4', '5', '6', 'B'],
        ['7', '8', '9', 'C'],
        ['*', '0', '#', 'D'],
    ];

    loop {
        for col_idx in 0..4 {
            // Set current column low, others high
            for (i, pin) in col_pins.iter_mut().enumerate() {
                if i == col_idx {
                    pin.set_low();
                } else {
                    pin.set_high();
                }
            }
            
            arduino_hal::delay_us(10);
            
            for (row_idx, row_pin) in row_pins.iter().enumerate() {
                if row_pin.is_low() {
                    let pressed_char = key_chars[row_idx][col_idx];
                    ufmt::uwriteln!(&mut serial, "Key pressed: {}", pressed_char).unwrap();
                    arduino_hal::delay_ms(200);
                }
            }
        }
        
        arduino_hal::delay_ms(10);
    }
}
