#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;

// Note frequencies
const NOTE_B0: u16 = 31;
const NOTE_C1: u16 = 33;
const NOTE_CS1: u16 = 35;
const NOTE_D1: u16 = 37;
const NOTE_DS1: u16 = 39;
const NOTE_E1: u16 = 41;
const NOTE_F1: u16 = 44;
const NOTE_FS1: u16 = 46;
const NOTE_G1: u16 = 49;
const NOTE_GS1: u16 = 52;
const NOTE_A1: u16 = 55;
const NOTE_AS1: u16 = 58;
const NOTE_B1: u16 = 62;
const NOTE_C2: u16 = 65;
const NOTE_CS2: u16 = 69;
const NOTE_D2: u16 = 73;
const NOTE_DS2: u16 = 78;
const NOTE_E2: u16 = 82;
const NOTE_F2: u16 = 87;
const NOTE_FS2: u16 = 93;
const NOTE_G2: u16 = 98;
const NOTE_GS2: u16 = 104;
const NOTE_A2: u16 = 110;
const NOTE_AS2: u16 = 117;
const NOTE_B2: u16 = 123;
const NOTE_C3: u16 = 131;
const NOTE_CS3: u16 = 139;
const NOTE_D3: u16 = 147;
const NOTE_DS3: u16 = 156;
const NOTE_E3: u16 = 165;
const NOTE_F3: u16 = 175;
const NOTE_FS3: u16 = 185;
const NOTE_G3: u16 = 196;
const NOTE_GS3: u16 = 208;
const NOTE_A3: u16 = 220;
const NOTE_AS3: u16 = 233;
const NOTE_B3: u16 = 247;
const NOTE_C4: u16 = 262;
const NOTE_CS4: u16 = 277;
const NOTE_D4: u16 = 294;
const NOTE_DS4: u16 = 311;
const NOTE_E4: u16 = 330;
const NOTE_F4: u16 = 349;
const NOTE_FS4: u16 = 370;
const NOTE_G4: u16 = 392;
const NOTE_GS4: u16 = 415;
const NOTE_A4: u16 = 440;
const NOTE_AS4: u16 = 466;
const NOTE_B4: u16 = 494;
const NOTE_C5: u16 = 523;
const NOTE_CS5: u16 = 554;
const NOTE_D5: u16 = 587;
const NOTE_DS5: u16 = 622;
const NOTE_E5: u16 = 659;
const NOTE_F5: u16 = 698;
const NOTE_FS5: u16 = 740;
const NOTE_G5: u16 = 784;
const NOTE_GS5: u16 = 831;
const NOTE_A5: u16 = 880;
const NOTE_AS5: u16 = 932;
const NOTE_B5: u16 = 988;
const NOTE_C6: u16 = 1047;
const NOTE_CS6: u16 = 1109;
const NOTE_D6: u16 = 1175;
const NOTE_DS6: u16 = 1245;
const NOTE_E6: u16 = 1319;
const NOTE_F6: u16 = 1397;
const NOTE_FS6: u16 = 1480;
const NOTE_G6: u16 = 1568;
const NOTE_GS6: u16 = 1661;
const NOTE_A6: u16 = 1760;
const NOTE_AS6: u16 = 1865;
const NOTE_B6: u16 = 1976;
const NOTE_C7: u16 = 2093;
const NOTE_CS7: u16 = 2217;
const NOTE_D7: u16 = 2349;
const NOTE_DS7: u16 = 2489;
const NOTE_E7: u16 = 2637;
const NOTE_F7: u16 = 2794;
const NOTE_FS7: u16 = 2960;
const NOTE_G7: u16 = 3136;
const NOTE_GS7: u16 = 3322;
const NOTE_A7: u16 = 3520;
const NOTE_AS7: u16 = 3729;
const NOTE_B7: u16 = 3951;
const NOTE_C8: u16 = 4186;
const NOTE_CS8: u16 = 4435;
const NOTE_D8: u16 = 4699;
const NOTE_DS8: u16 = 4978;
const REST: u16 = 0;

// Never Gonna Give You Up melody - (note, duration)
const MELODY: [(u16, i8); 256] = [
    (NOTE_D5, -4), (NOTE_E5, -4), (NOTE_A4, 4),
    (NOTE_E5, -4), (NOTE_FS5, -4), (NOTE_A5, 16), (NOTE_G5, 16), (NOTE_FS5, 8),
    (NOTE_D5, -4), (NOTE_E5, -4), (NOTE_A4, 2),
    (NOTE_A4, 16), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 8), (NOTE_D5, 16),
    (NOTE_D5, -4), (NOTE_E5, -4), (NOTE_A4, 4),
    (NOTE_E5, -4), (NOTE_FS5, -4), (NOTE_A5, 16), (NOTE_G5, 16), (NOTE_FS5, 8),
    (NOTE_D5, -4), (NOTE_E5, -4), (NOTE_A4, 2),
    (NOTE_A4, 16), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 8), (NOTE_D5, 16),
    (REST, 4), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_D5, 8), (NOTE_E5, 8), (NOTE_CS5, -8),
    (NOTE_B4, 16), (NOTE_A4, 2), (REST, 4),
    
    (REST, 8), (NOTE_B4, 8), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 4), (NOTE_A4, 8),
    (NOTE_A5, 8), (REST, 8), (NOTE_A5, 8), (NOTE_E5, -4), (REST, 4),
    (NOTE_B4, 8), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 8), (NOTE_D5, 8), (NOTE_E5, 8), (REST, 8),
    (REST, 8), (NOTE_CS5, 8), (NOTE_B4, 8), (NOTE_A4, -4), (REST, 4),
    (REST, 8), (NOTE_B4, 8), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 8), (NOTE_A4, 4),
    (NOTE_E5, 8), (NOTE_E5, 8), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_E5, 4), (REST, 4),
    
    (NOTE_D5, 2), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_D5, 8),
    (NOTE_E5, 8), (NOTE_E5, 8), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_E5, 4), (NOTE_A4, 4),
    (REST, 2), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 8),
    (REST, 8), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_E5, -4), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    (NOTE_FS5, -8), (NOTE_FS5, -8), (NOTE_E5, -4), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    
    (NOTE_E5, -8), (NOTE_E5, -8), (NOTE_D5, -8), (NOTE_CS5, 16), (NOTE_B4, -8), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    (NOTE_D5, 4), (NOTE_E5, 8), (NOTE_CS5, -8), (NOTE_B4, 16), (NOTE_A4, 8), (NOTE_A4, 8), (NOTE_A4, 8),
    (NOTE_E5, 4), (NOTE_D5, 2), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    (NOTE_FS5, -8), (NOTE_FS5, -8), (NOTE_E5, -4), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    (NOTE_A5, 4), (NOTE_CS5, 8), (NOTE_D5, -8), (NOTE_CS5, 16), (NOTE_B4, 8), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    
    (NOTE_D5, 4), (NOTE_E5, 8), (NOTE_CS5, -8), (NOTE_B4, 16), (NOTE_A4, 4), (NOTE_A4, 8),
    (NOTE_E5, 4), (NOTE_D5, 2), (REST, 4),
    (REST, 8), (NOTE_B4, 8), (NOTE_D5, 8), (NOTE_B4, 8), (NOTE_D5, 8), (NOTE_E5, 4), (REST, 8),
    (REST, 8), (NOTE_CS5, 8), (NOTE_B4, 8), (NOTE_A4, -4), (REST, 4),
    (REST, 8), (NOTE_B4, 8), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 8), (NOTE_A4, 4),
    (REST, 8), (NOTE_A5, 8), (NOTE_A5, 8), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_E5, 8), (NOTE_D5, 8),
    
    (REST, 8), (NOTE_A4, 8), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 8),
    (REST, 8), (NOTE_CS5, 8), (NOTE_B4, 8), (NOTE_A4, -4), (REST, 4),
    (NOTE_B4, 8), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 8), (NOTE_A4, 4), (REST, 8),
    (REST, 8), (NOTE_E5, 8), (NOTE_E5, 8), (NOTE_FS5, 4), (NOTE_E5, -4),
    (NOTE_D5, 2), (NOTE_D5, 8), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_E5, 4),
    (NOTE_E5, 8), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_E5, 8), (NOTE_A4, 8), (NOTE_A4, 4),
    
    (REST, -4), (NOTE_A4, 8), (NOTE_B4, 8), (NOTE_CS5, 8), (NOTE_D5, 8), (NOTE_B4, 8),
    (REST, 8), (NOTE_E5, 8), (NOTE_FS5, 8), (NOTE_E5, -4), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    (NOTE_FS5, -8), (NOTE_FS5, -8), (NOTE_E5, -4), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    (NOTE_E5, -8), (NOTE_E5, -8), (NOTE_D5, -8), (NOTE_CS5, 16), (NOTE_B4, 8), (NOTE_A4, 16), (NOTE_B4, 16), (NOTE_D5, 16), (NOTE_B4, 16),
    (NOTE_D5, 4), (NOTE_E5, 8), (NOTE_CS5, -8), (NOTE_B4, 16), (NOTE_A4, 4), (NOTE_A4, 8),
    
    (NOTE_E5, 4), (NOTE_D5, 2), (REST, 4)
];

const TEMPO: u16 = 114;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut buzzer = pins.d10.into_output();
    let mut song_playing = false;
    let mut song_index = 0usize;
    let mut current_note_freq = 0u16;
    let mut note_duration_ms = 0u32;
    let mut note_elapsed_ms = 0u32;
    
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
                    
                    if sound == 0 {
                        // Stop song when key with frequency 0 is pressed
                        song_playing = false;
                        buzzer.set_low();
                    } else {
                        // Start song when any other key is pressed
                        song_playing = true;
                        song_index = 0;
                        note_elapsed_ms = 0;
                    }
                    arduino_hal::delay_ms(200);
                }
            }
        }
        
        // Play song if active
        if song_playing {
            if song_index < MELODY.len() {
                // Start new note if elapsed time is 0
                if note_elapsed_ms == 0 {
                    let (note_freq, duration) = MELODY[song_index];
                    current_note_freq = note_freq;
                    
                    // Calculate note duration in milliseconds
                    let wholenote = (60000 * 4) / TEMPO as u32;
                    note_duration_ms = if duration > 0 {
                        wholenote / duration as u32
                    } else {
                        let base_duration = wholenote / (duration.abs() as u32);
                        (base_duration * 3) / 2 // Dotted note
                    };
                }
                
                // Play current note for a short time (about 5ms worth)
                if current_note_freq > 0 {
                    let period_us = 1_000_000 / current_note_freq as u32;
                    let half_period_us = period_us / 2;
                    let cycles_to_play = (5000 / period_us).max(1); // Play for ~5ms
                    
                    for _ in 0..cycles_to_play {
                        buzzer.set_high();
                        arduino_hal::delay_us(half_period_us);
                        buzzer.set_low();
                        arduino_hal::delay_us(half_period_us);
                    }
                    note_elapsed_ms += 5;
                } else {
                    // Rest note
                    buzzer.set_low();
                    arduino_hal::delay_ms(5);
                    note_elapsed_ms += 5;
                }
                
                // Move to next note when current one is finished
                if note_elapsed_ms >= note_duration_ms {
                    song_index += 1;
                    note_elapsed_ms = 0;
                }
            } else {
                // Song finished, restart
                song_index = 0;
                note_elapsed_ms = 0;
            }
        } else {
            buzzer.set_low();
            arduino_hal::delay_us(10);
        }
    }
}
