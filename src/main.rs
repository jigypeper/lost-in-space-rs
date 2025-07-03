#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut buzzer = pins.d10.into_output();
    let mut current_freq = 0u16;
    
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

    let tones = [
      // a frequency tone for each button
      [ 31, 93, 147, 208 ],
      [ 247, 311, 370, 440 ],
      [ 523, 587, 698, 880 ],
      [ 1397, 2637, 3729, 0 ],  // Use frequency of 0 for bottom right key to end tone.
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
                    let sound = tones[row_idx][col_idx];
                    ufmt::uwriteln!(&mut serial, "Key pressed: {}", pressed_char).unwrap();
                    current_freq = sound;
                    arduino_hal::delay_ms(200);
                }
            }
        }
        
        // Generate tone if frequency is set
        if current_freq > 0 {
            let period_us = 1_000_000 / current_freq as u32;
            let half_period_us = period_us / 2;
            
            buzzer.set_high();
            arduino_hal::delay_us(half_period_us);
            buzzer.set_low();
            arduino_hal::delay_us(half_period_us);
        } else {
            buzzer.set_low();
            arduino_hal::delay_us(10);
        }
    }
}
