## heartbeat

Takes a GPIO pin as input and blinks an LED. Timing is currently hardcoded as being on for 1s and off for 1s. The next iteration of the utility may receive millisecond on and off values as arguments.

Based on [this example from rust-sysfs-gpio](https://github.com/rust-embedded/rust-sysfs-gpio#exampleapi). Arguments handled with [clap](https://crates.io/crates/clap).
