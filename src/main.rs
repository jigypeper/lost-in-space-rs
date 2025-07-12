#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;

struct UserPin {
    pin: [char; 4],
    pin_set: bool,
}

impl UserPin {
    fn default() -> UserPin {
        UserPin { pin: ['0', '0', '0', '0'], pin_set: false }
    }

    fn set_pin(&mut self, pin: [char; 4]) {
        self.pin = pin;
        self.pin_set = true;
    }
    
    fn validate_pin(&self, pin: [char; 4]) -> bool {
        self.pin[0] == pin[0] && 
        self.pin[1] == pin[1] && 
        self.pin[2] == pin[2] && 
        self.pin[3] == pin[3]
    }
    
    fn get_pin(&self) -> [char; 4] {
        self.pin
    }
}

fn play_sound<T>(current_freq: u16, buzzer: &mut T)
where
T: OutputPin
{
    let period_us = 1_000_000 / current_freq as u32;
    let half_period_us = period_us / 2;

    buzzer.set_high().unwrap();
    arduino_hal::delay_us(half_period_us);
    buzzer.set_low().unwrap();
    arduino_hal::delay_us(half_period_us);
}

fn play_success_sound<T>(buzzer: &mut T)
where
T: OutputPin
{
    // Play ascending tones for success
    let success_tones = [523, 659, 784]; 
    for freq in success_tones.iter() {
        let period_us = 1_000_000 / freq;
        let half_period_us = period_us / 2;
        let cycles = (300 * 1000) / period_us; 
        
        for _ in 0..cycles {
            buzzer.set_high().unwrap();
            arduino_hal::delay_us(half_period_us);
            buzzer.set_low().unwrap();
            arduino_hal::delay_us(half_period_us);
        }
        arduino_hal::delay_ms(50); 
    }
}

fn play_error_sound<T>(buzzer: &mut T)
where
T: OutputPin
{
    // Play low frequency error beeps for 2 seconds
    let error_freq = 200; // Low frequency for error
    let period_us = 1_000_000 / error_freq;
    let half_period_us = period_us / 2;
    
    // Play for 2 seconds with beep pattern
    for _ in 0..4 {
        // 400ms beep
        let cycles = (400 * 1000) / period_us;
        for _ in 0..cycles {
            buzzer.set_high().unwrap();
            arduino_hal::delay_us(half_period_us);
            buzzer.set_low().unwrap();
            arduino_hal::delay_us(half_period_us);
        }
        // 100ms silence
        buzzer.set_low().unwrap();
        arduino_hal::delay_ms(100);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut buzzer = pins.d10.into_output();
    let mut user = UserPin::default();
    let mut current_pin: [char; 4] = ['*', '*', '*', '*'];
    let mut pin_position = 0usize;
    let mut setting_pin = true;
    
    ufmt::uwriteln!(&mut serial, "Pin Entry System Started").unwrap();
    ufmt::uwriteln!(&mut serial, "Please set your 4-digit pin using 0-9 keys").unwrap();
    
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
                    
                    // Handle digit input (0-9)
                    if pressed_char >= '0' && pressed_char <= '9' && pin_position < 4 {
                        current_pin[pin_position] = pressed_char;
                        pin_position += 1;
                        ufmt::uwriteln!(&mut serial, "Pin position {}: {}", pin_position, pressed_char).unwrap();
                        
                        if pin_position == 4 {
                            if setting_pin {
                                // Save the pin
                                user.set_pin(current_pin);
                                setting_pin = false;
                                ufmt::uwriteln!(&mut serial, "Pin set successfully! Press # to enter pin.").unwrap();
                                // Reset after setting pin
                                current_pin = ['*', '*', '*', '*'];
                                pin_position = 0;
                            } else {
                                // Validate the pin
                                let stored_pin = user.get_pin();
                                ufmt::uwriteln!(&mut serial, "Stored pin: {}{}{}{}", stored_pin[0], stored_pin[1], stored_pin[2], stored_pin[3]).unwrap();
                                ufmt::uwriteln!(&mut serial, "Entered pin: {}{}{}{}", current_pin[0], current_pin[1], current_pin[2], current_pin[3]).unwrap();
                                
                                if user.validate_pin(current_pin) {
                                    ufmt::uwriteln!(&mut serial, "Access granted!").unwrap();
                                    play_success_sound(&mut buzzer);
                                    // Reset after successful access
                                    current_pin = ['*', '*', '*', '*'];
                                    pin_position = 0;
                                } else {
                                    ufmt::uwriteln!(&mut serial, "Access denied! Press * to try again.").unwrap();
                                    play_error_sound(&mut buzzer);
                                    // Don't reset - let user press * to reset
                                }
                            }
                        }
                    }
                    // Handle # key - start pin entry mode
                    else if pressed_char == '#' && user.pin_set && !setting_pin {
                        ufmt::uwriteln!(&mut serial, "Enter pin code:").unwrap();
                        current_pin = ['*', '*', '*', '*'];
                        pin_position = 0;
                    }
                    // Handle * key - reset current entry
                    else if pressed_char == '*' {
                        ufmt::uwriteln!(&mut serial, "Entry cleared").unwrap();
                        current_pin = ['*', '*', '*', '*'];
                        pin_position = 0;
                        // Don't change setting_pin state - keep current mode
                    }
                    // Handle C key - change pin
                    else if pressed_char == 'C' {
                        ufmt::uwriteln!(&mut serial, "Entry cleared").unwrap();
                        current_pin = ['*', '*', '*', '*'];
                        pin_position = 0;
                        setting_pin = true;
                    }
                    
                    arduino_hal::delay_ms(200);
                }
            }
        }
        
    }
}
