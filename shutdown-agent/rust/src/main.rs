//#[macro_use]

extern crate sysfs_gpio;
use std::process::exit;
// https://github.com/rust-embedded/rust-sysfs-gpio
use sysfs_gpio::{Direction, Pin};
use std::time::Duration;
use std::thread::sleep;
//use std::env;


// Raspberry Pi Zero Active Buzzer Test : https://gist.github.com/actuino/d88061db0ec118d8de2b6f9d0d84d1d6
// https://github.com/rust-embedded/rust-sysfs-gpio/blob/master/examples/blinky.rs
// https://github.com/rust-embedded/gpio-utils/blob/master/src/commands/gpio_write.rs

// https://github.com/rust-embedded/gpio-utils/blob/master/src/main.rs
// https://github.com/inre/cupi/blob/master/examples/button.rs

// https://github.com/rust-embedded/rust-sysfs-gpio
// https://github.com/rust-embedded/gpio-utils

fn main() {
/*
    let my_led = Pin::new(40);
    my_led.with_exported(|| {
        loop {
            my_led.set_value(0).unwrap();		
			/*match my_led.get_value() {
						Ok(value) => println!("{}", value),
						Err(e) => println!("ERROR: {:?}", e),
			}*/			
            sleep(Duration::from_millis(200));
			
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
			match my_led.get_value() {
						Ok(value) => println!("{}", value),
						Err(e) => println!("ERROR: {:?}", e),
			}			
        }
    }).unwrap();
*/

	let buzzer_pin = Pin::new(7);
    buzzer_pin.with_exported(|| {
		sleep(Duration::from_millis(100));
		
		buzzer_pin.set_direction(Direction::Out).unwrap_or_else(|e| {
			println!("Error setting GPIO direction: {:?}", e);
			exit(1)
		});
		
		buzzer_pin.set_value(0).unwrap_or_else(|e| {
			println!("There was an error writing to the gpio: {:?}", e);
			exit(1);
		});
		
		match buzzer_pin.get_value() {
				Ok(value) => println!("{}", value),
				Err(e) => println!("ERROR: {:?}", e),
		};
		
	    Ok(())
    });
}


