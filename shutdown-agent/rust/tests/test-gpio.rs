
extern crate sysfs_gpio;

#[cfg(test)]
mod tests {
    // https://github.com/rust-embedded/rust-sysfs-gpio
    use sysfs_gpio::{Direction, Pin};

    #[test]
    fn test_init_gpio() {
        let gpio = Pin::new(7);
        match gpio.export() {
            Ok(()) => assert_eq!(gpio.get_pin(), 7),
            Err(err) => println!("Gpio {} could not be exported: {}", gpio.get_pin(), err),
        }
    }
}
