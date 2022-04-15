use sysfs_gpio;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let my_led = Pin::new(127); // number depends on chip, etc.
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
        loop {
            my_led.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
        }
    }).unwrap();
}