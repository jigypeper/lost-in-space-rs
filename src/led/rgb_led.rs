#[allow(dead_code)]

use embedded_hal::digital::OutputPin;

type Intensity = u8;

struct RgbLed<R: OutputPin, G: OutputPin, B: OutputPin> {
    red: R,
    green: G,
    blue: B,
}

impl<R: OutputPin, G: OutputPin, B: OutputPin> RgbLed<R, G, B> {
    fn new(red_pin: R, green_pin: G, blue_pin: B) -> Self {
        Self {
            red: red_pin,
            green: green_pin,
            blue: blue_pin,
        }
    }

    fn display_colour(&mut self, red: Intensity, green: Intensity, blue: Intensity) {
        const PWM_STEPS: u8 = 255;
        
        for step in 0..PWM_STEPS {
            // Set pins high if intensity is greater than current step
            if red > step {
                let _ = self.red.set_high();
            } else {
                let _ = self.red.set_low();
            }
            
            if green > step {
                let _ = self.green.set_high();
            } else {
                let _ = self.green.set_low();
            }
            
            if blue > step {
                let _ = self.blue.set_high();
            } else {
                let _ = self.blue.set_low();
            }
            
            arduino_hal::delay_us(50);
        }
    }

    fn fade_to(&mut self, from: (Intensity, Intensity, Intensity), to: (Intensity, Intensity, Intensity), steps: u16) {
        for step in 0..=steps {
            let progress = step as f32 / steps as f32;
            
            let red = (from.0 as f32 + (to.0 as f32 - from.0 as f32) * progress) as Intensity;
            let green = (from.1 as f32 + (to.1 as f32 - from.1 as f32) * progress) as Intensity;
            let blue = (from.2 as f32 + (to.2 as f32 - from.2 as f32) * progress) as Intensity;
            
            self.display_colour(red, green, blue);
            arduino_hal::delay_ms(10);
        }
    } 
}
