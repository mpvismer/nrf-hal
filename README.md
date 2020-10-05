# `nrf-hal`

> [HAL] for the nRF51, nRF52 and nRF91 families of microcontrollers

[HAL]: https://crates.io/crates/embedded-hal

![CI](https://github.com/nrf-rs/nrf-hal/workflows/CI/badge.svg)

Please refer to the [changelog] to see what changed in the last releases.

[changelog]: ./CHANGELOG.md
test2
## Crates

Every nRF chip has its own crate, listed below:

| Crate | Docs | crates.io |
|-------|------|-----------|
| [`nrf51-hal`](./nrf51-hal) | [![docs.rs](https://docs.rs/nrf51-hal/badge.svg)](https://docs.rs/nrf51-hal) | [![crates.io](https://img.shields.io/crates/d/nrf51-hal.svg)](https://crates.io/crates/nrf51-hal) |
| [`nrf52810-hal`](./nrf52810-hal) | [![docs.rs](https://docs.rs/nrf52810-hal/badge.svg)](https://docs.rs/nrf52810-hal) | [![crates.io](https://img.shields.io/crates/d/nrf52810-hal.svg)](https://crates.io/crates/nrf52810-hal) |
| [`nrf52811-hal`](./nrf52811-hal) | [![docs.rs](https://docs.rs/nrf52811-hal/badge.svg)](https://docs.rs/nrf52811-hal) | [![crates.io](https://img.shields.io/crates/d/nrf52811-hal.svg)](https://crates.io/crates/nrf52811-hal) |
| [`nrf52832-hal`](./nrf52832-hal) | [![docs.rs](https://docs.rs/nrf52832-hal/badge.svg)](https://docs.rs/nrf52832-hal) | [![crates.io](https://img.shields.io/crates/d/nrf52832-hal.svg)](https://crates.io/crates/nrf52832-hal) |
| [`nrf52833-hal`](./nrf52833-hal) | [![docs.rs](https://docs.rs/nrf52833-hal/badge.svg)](https://docs.rs/nrf52833-hal) | [![crates.io](https://img.shields.io/crates/d/nrf52833-hal.svg)](https://crates.io/crates/nrf52833-hal) |
| [`nrf52840-hal`](./nrf52840-hal) | [![docs.rs](https://docs.rs/nrf52840-hal/badge.svg)](https://docs.rs/nrf52840-hal) | [![crates.io](https://img.shields.io/crates/d/nrf52840-hal.svg)](https://crates.io/crates/nrf52840-hal) |
| [`nrf9160-hal`](./nrf9160-hal) | [![docs.rs](https://docs.rs/nrf9160-hal/badge.svg)](https://docs.rs/nrf9160-hal) | [![crates.io](https://img.shields.io/crates/d/nrf9160-hal.svg)](https://crates.io/crates/nrf9160-hal) |

## Device Reference Manuals from Nordic

| Device | Product Specification | DK Reference Guide |
|-------|------|-----------|
| [`nRF52810`](https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52810) | [`v1.3`](https://infocenter.nordicsemi.com/pdf/nRF52810_PS_v1.3.pdf) | [`v1.3.1*`](https://infocenter.nordicsemi.com/pdf/nRF52_DK_User_Guide_v1.3.1.pdf) |
| [`nRF52811`](https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52811) | [`v1.0`](https://infocenter.nordicsemi.com/pdf/nRF52811_PS_v1.0.pdf) | [`v1.3.1*`](https://infocenter.nordicsemi.com/pdf/nRF52_DK_User_Guide_v1.3.1.pdf) |
| [`nRF52832`](https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52832) | [`v1.4`](https://infocenter.nordicsemi.com/pdf/nRF52832_PS_v1.4.pdf) | [`v1.3.1*`](https://infocenter.nordicsemi.com/pdf/nRF52_DK_User_Guide_v1.3.1.pdf) |
| [`nRF52833`](https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52833) | [`v1.3`](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf) | [`v1.0.1`](http://infocenter.nordicsemi.com/pdf/nRF52833_DK_User_Guide_v1.0.1.pdf) |
| [`nRF52840`](https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52840) | [`v1.1`](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf) | [`v1.2`](https://infocenter.nordicsemi.com/pdf/nRF52840_DK_User_Guide_v1.2.pdf) |
| [`nRF9160`](https://www.nordicsemi.com/Products/Low-power-cellular-IoT/nRF9160) | [`v2.0`](https://infocenter.nordicsemi.com/pdf/nRF9160_PS_v2.0.pdf) | [`v0.9.3`](https://infocenter.nordicsemi.com/pdf/nRF9160_DK_HW_User_Guide_v0.9.3.pdf) |
\* These devices do not have a seperate developement kit and share the [NRF52 DK](https://www.nordicsemi.com/Software-and-tools/Development-Kits/nRF52-DK)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Running the Demos

All the demos can be found in the examples folder and are completely independent cargo projects. Therefore running something like `cargo run --bin ccm-demo` from the root of this repo (`./nrf-hal/`) *will not work*. You need to run your cargo commands from something like this `./nrf-hal/examples/ccm-demo/`. There are many ways to flash the demos to your device and debug them and this repository does not force you to make a choice. However, if you are struggling to get things working then hopefully this explanation of getting the demos to work on the nRF52840-DK using probe-rs will help. 

Once off system setup:
Install a target for your device for cross compilation as well as a mechanism to flash to your device.
```
rustup target add thumbv7em-none-eabihf
cargo install probe-run
```

For every project:
Setup your .cargo/config file (create one in your demo project root if it does not exist. e.g. `./nrf-hal/examples/ccm-demo/.cargo/config`). This will call the prope-run executable configured for the nrf52840 chip when you call `cargo run`. Here is an example `.cargo/config` file:
```
[target.thumbv7em-none-eabihf]
runner = "probe-run --chip nRF52840_xxAA"

[build]
target = "thumbv7em-none-eabihf"
```
Setup the `Cargo.toml` file to use the correct features. Features allow for conditional compilation which is essential for a library like this that supports multiple different devices. Under the `[features]` section add the following line `default = ["52840"]` for the nRF52840-DK device. This is optional but it will allow you to simply call `cargo run` and `cargo build` instead of `cargo run --features 52840` and `cargo build --features 52840` respectively. Note that some demo projects do not have features so this step may not be necessary. If you get a whole bunch of compilation errors then check that you have set the default features correctly.

To run:
Plug in your device (on the nRF52840-DK it is the J2 usb port)
`cargo run`
This will flash the device, reset it and send `rprintln!` debug messages from the device to the terminal automatically.

Here follows a brief description of each demo for quick reference. For a more in-depth explanation on how the peripherals work please refer to the device reference manuals above and the comments in the demo code itself.

### blinky-button-demo (Hello World)

The blinky button demo. This demonstrates a simple hello world blinky application targeted at the nrf52832 chip. Note that this has a different hardware abstraction layer to the nRD52840-DK board so you can expect to find different pins for the led and button. It is a useful exercise to experiment with what you need to change to get the same functionality working on the nrf52840 chip as this project does not use features to support multiple chips. This demo also introduces the cargo-embed tool which is an alternative to probe-run but part of the same family.

### ccm-demo (Encryption)

The cipher block chaining - message authentication code (CCM) mode demo. This demo initialises a text message of the maximum size of 251 bytes and encrypts and decrypts it, measuring the time it takes. It then repeats the process with smaller and smaller chunks of data to demonstrate how long smaller packets take to process.

### comp-demo (Analog Pins)

The comparator peripheral demo. This demo uses the comp peripheral to compare the differential voltages between two pins. If the voltage on Pin 30 is higher than Pin 31 (reference voltage) the built in LED will switch off otherwise it will switch on.

### ecb-demo (Encryption)

The AES electronic codebook mode encryption demo. Blocking 128-bit AES encryption of 16 bytes of data using a 16 byte key. Encryption only, no decryption.

### gpiote-demo (Digital Pins)

The General-Purpose Input Output Tasks and Events module demo. This demo targets the nRF52840-DK in particular because of the 4 available hardware buttons on the board itself. The demo shows how you can use the `cortex-m-rtic` crate to easily debounce some buttons without blocking the cpu. GPIO pin state changes fire events which can be used to carry out tasks. This showcases the PPI (programmable peripheral interconnect) system for which there is also a dedicated demo.

### i2s-controller-demo (Audio)

The Inter-IC Sound interface 'controller mode (aka master mode)' demo. This demo generates Morse code audio signals for text from UART and plays them back over I2S. Tested with nRF52840-DK and a UDA1334a DAC. 

### i2s-peripheral-demo (Audio)

The Inter-IC Sound interface 'peripheral mode (aka slave mode)' demo. This demonstrates full duplex communication between a controller and peripheral mode i2s peripheral using the EasyDMA capabilities of the chip. 

### lpcomp-demo (Analog Pins)

The low power comparator demo. This demo shows how you would keep the device in low power mode and power it up when an analog voltage on a pin changes with respect to a voltage on a reference pin.

### ppi-demo (Channels)

The programmable peripheral interconnect (PPI) demo. The PPI allows peripherals to interact with each other without having to go through the CPU. Note that you need to choose a default feature in order for this demo to build. See above. This demo uses the Bluetooth RADIO peripheral as an example but does nothing special with Bluetooth itself so this is not the demo to learn about that capability.

### pwm-demo (Digital Pins)

The pulse width modulation demo. This demonstrates various PWM use cases by allowing the user to press buttons to change demo modes. This outputs PWM signals to the built in LEDs on the nRF52840-DK.

### qdec-demo (Sensor Decoding)

The quadrature decoder (QDEC) demo. This peripheral supports buffered decoding of quadrature-encoded sensor signals (typically used for mechanical and optical sensors). The demo reads a byte value from two input pins expected to contain qdec encoded data at a given frequency.

### rtic-demo (Concurrency Framework)

The Real-Time Interrupt-driven Concurrency framework demo. Many of the demos in this project use RTIC and demonstrate its capabilities in more detail but this is a bare-bones default template for you to build off. RTIC is not unique to the nRF series but very useful for a chip that requires concurrency. Unfortunately, this demo does not appear to use rtt for logging so it crashes when used with probe-run. However, it will work with other debuggers. See other demos for rtt logging setup.

### spi-demo (Digital Pins)

The serial peripheral interface master (SPIM) with EasyDMA demo. Sends some text out on the spi peripheral and loops it back on itself to demonstrate full duplex direct-memory-access based SPI data transfer. You'll need a resistor to connect the output to the input. 

### twi-ssd1306 (Digital Pins)

I2C compatible Two-Wire Interface with the SSD1306 OLED Display demo. This demo uses the twim (Two-Wire Interface Master) peripheral along with the embedded_graphics library to draw "Hello Rust!" to an OLED screen. Note that you need to set a default feature to get this to compile (see "Running the demos" section).

### twim-demo (Digital Pins)

I2C compatible Two-Wire Interface Master mode demo. This demo uses the twim peripheral to read and write 8 bytes of data to arbitrary addresses on whatever device is connected to the I2C pins p0_30 and p0_31. It demonstrates error handling if the device does not respond properly (or it is not connected).

### twis-demo (Digital Pins)

I2C compatible Two-Wire Interface Slave mode demo. This demo uses the twis peripheral with rtic to listen for I2C signals which are exposed as events. The event handler reads data from the peripheral at the address specified.

### wdt-demo (Timer)

Watchdog timer demo. This demonstrates the how the entire device can be set to automatically reset if certain conditions are not met within a certain period of time. In this case you have to press all 4 buttons at least once within a 5 second period to prevent a reset. If you mash the buttons for a while you will encounter an 'attempt to subtract with overflow' panic at `main.rs:205` which is ultimately cleared by the watchdog timer. This is intended demo behaviour ;)
