//#[macro_use]

extern crate sysfs_gpio;
use std::process::exit;
// https://github.com/rust-embedded/rust-sysfs-gpio
use sysfs_gpio::{Direction, Pin};
//use std::time::Duration;
//use std::thread::sleep;
//use std::env;


// Raspberry Pi Zero Active Buzzer Test : https://gist.github.com/actuino/d88061db0ec118d8de2b6f9d0d84d1d6
// https://github.com/rust-embedded/rust-sysfs-gpio/blob/master/examples/blinky.rs
// https://github.com/rust-embedded/gpio-utils/blob/master/src/commands/gpio_write.rs


fn main() {
    let buzzer_pin = Pin::new(40);
    buzzer_pin.with_exported(|| {
        try!(buzzer_pin.set_direction(Direction::Out));        
        Ok(())
    });

    /*buzzer_pin.set_direction(Direction::Out).unwrap_or_else(|e| {
        println!("Error setting GPIO direction: {:?}", e);
        exit(1)
    });*/

   /* pin.set_value(opts.value).unwrap_or_else(|e| {
        println!("There was an error writing to the gpio: {:?}", e);
        exit(1);
});*/


    println!("Hello, world!");
}

// https://github.com/inre/cupi/blob/master/examples/button.rs

// https://github.com/rust-embedded/gpio-utils/blob/master/src/main.rs