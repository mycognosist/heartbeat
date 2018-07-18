// heart led
//
// uses linux sysfs-gpio and takes pin definition as arg

extern crate sysfs_gpio;
extern crate clap;

use clap::{Arg, App};
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    let matches = App::new("heartbeat")
        .version("0.1.0")
        .author("Andrew Reid <gnomad@cryptolab.net>")
        .arg(Arg::with_name("LED_PIN")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("gpio pin address for heartbeat led"))
        .get_matches();
    // assign variable based on input parameter
    let led_pin = matches.value_of("LED_PIN").unwrap();
    let pin: u8 = led_pin
        .trim()
        .parse()
        .expect("Failed to parse");

    let heartbeat_led = Pin::new(pin);
    heartbeat_led.with_exported(|| {
        loop {
            heartbeat_led.set_value(0).unwrap();
            sleep(Duration::from_millis(1000));
            heartbeat_led.set_value(1).unwrap();
            sleep(Duration::from_millis(1000));
        }
    }).unwrap();
}
