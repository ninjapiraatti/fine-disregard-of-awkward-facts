use rust_gpiozero::*;

fn main() {
    // Create a new LED attached to Pin 18
    let mut led = LED::new(18);

    // on_time = 2 secs, off_time=3 secs
    led.blink(2.0,3.0);

    // prevent program from exiting immediately
    led.wait();
}