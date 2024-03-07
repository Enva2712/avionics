mod esc;
mod sim7000g;
use esc::ESC;
use esp_idf_svc::hal::{
    delay::Ets,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use mpu9250::Mpu9250;
use std::{thread::sleep, time::Duration};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut peripherals = Peripherals::take()?;
    /*
    let d = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio21, // data
        peripherals.pins.gpio22, // clock
        &I2cConfig::new()
            .baudrate(40.kHz().into())
            .sda_enable_pullup(false)
            .scl_enable_pullup(false),
    )?;

    let mut ac = Mpu9250::marg_default(d, &mut Ets).expect("Failed to initialize IMU");

    for _ in 1..50 {
        let a = ac.all::<[f32; 3]>().unwrap();
        println!("{:?}", a);
    }
    */

    sleep(Duration::from_secs(2)); // gives escs time to prepare

    /*
    let mut left_motor = ESC::setup(
        &mut peripherals.pins.gpio32,
        &mut peripherals.ledc.timer0,
        &mut peripherals.ledc.channel0,
    )?;
    */
    let mut right_motor = ESC::setup(
        &mut peripherals.pins.gpio33,
        &mut peripherals.ledc.timer1,
        &mut peripherals.ledc.channel1,
    )?;

    // left_motor.set_speed(10)?;
    for i in (5..=40).step_by(5) {
        right_motor.tween_to(i, Duration::from_secs(2))?;
        right_motor.set_speed(0)?;
        sleep(Duration::from_secs(2));
    }

    Ok(())
}
