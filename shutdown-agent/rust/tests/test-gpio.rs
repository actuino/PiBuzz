
extern crate sysfs_gpio;

// https://github.com/rust-embedded/rust-sysfs-gpio
use sysfs_gpio::{Direction, Pin};
use sysfs_gpio::Error;

pub fn setup(numgpio: u64) -> Pin {
    return Pin::new(numgpio);
}

pub fn exit(gpio: Pin) -> Result<(), Error> {
    return gpio.unexport();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio() {
        let gpio = setup(7);
        match gpio.export() {
            Ok(()) => assert_eq!(gpio.get_pin(), 7),
            Err(err) => println!("Gpio {} could not be exported: {}", gpio.get_pin(), err),
        }

        let result = exit(gpio);
        assert!(result.is_ok());
    }
}
