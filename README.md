If you get the error

```
/avionics$ cargo build
error: Toolchain esp in ../avionics/rust-toolchain.toml is custom and not installed
```
what I did was get "out of the directory" and run

```
cargo install espup
```
then do
```
espup install
```
(this can be in this avionics repository "base" directory). Note that this "takes a while" (more than a minute, less than a hour).

Then do, in the avionics "base" repository directory, do cargo run. Notice in .cargo/config.toml, it has `linker = "ldproxy"`, `runner = "espflash flash --monitor --baud=921600"`.

If  you get

```
error: linker `ldproxy` not found
  |
  = note: No such file or directory (os error 2)

warning: `avionics` (bin "avionics") generated 2 warnings
```
consider running

```
cargo install ldproxy
```

Also consider installing espflash

```
cargo install espflash
```
If having trouble, make sure to have installed this:

```
sudo apt install libudev-dev pkg-config
cargo install espflash
```

If doesn't work, 
```
Error: espflash::serial_error

  × Failed to open serial port /dev/ttyACM0
  ├─▶ Error while connecting to device
  ├─▶ IO error while using serial port: Permission denied
  ╰─▶ Permission denied
```
then to determine the group members:

```
$ getent group dialout

# example output:
dialout:x:20:
```

```
# Groups that the user, e.g. ernest, belongs to:
groups ernest
```

```
sudo usermod -aG dialout $USER
newgrp dialout
```

Lilygo 

https://randomnerdtutorials.com/lilygo-t-sim7000g-esp32-lte-gprs-gps/

ESP32 SIM7000G Version 1.1

https://github.com/Xinyuan-LilyGO/LilyGO-T-SIM7000G/blob/master/schematic/SIM7000G_20200415.pdf

#### It's in a bootloop during development

What we can do to is "wrap" with #[cfg(feature = "imu")] code that only works if an actual IMU is attached. When ready, run with

```
cargo run --features imu
```

Pinouts

IMU 22 21 Wire SCL SDA

left motor 

black is gnd
white is GPIO33 Touch8 ADC05
orange GPIO12 12 
VBAT pull resistor

servos
VIN orange.
GPIO 34 35

RadioShack
Servo
Cat. No. 2730765 4.8V - 6.0V
Micro Servo