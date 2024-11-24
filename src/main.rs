mod esc;
mod sim7000g;
use esc::ESC;
use esp_idf_svc::hal::{
    peripherals::Peripherals,
    prelude::*,
};
use std::{thread::sleep, time::Duration};

#[cfg(feature = "imu")]
use esp_idf_svc::hal::{
    delay::Ets,
    i2c::{I2cConfig, I2cDriver},
};

#[cfg(feature = "imu")]
use mpu9250::Mpu9250;

//use avionics::integration_tests::motor_tests;
use avionics::integration_tests::servo_tests;

fn main() -> anyhow::Result<()> {
    /*
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut peripherals = Peripherals::take()?;
    */

    #[cfg(feature = "imu")]
    {
        let d = I2cDriver::new(
            peripherals.i2c0,
            peripherals.pins.gpio21,
            peripherals.pins.gpio22,
            &I2cConfig::new()
                .baudrate(40.kHz().into())
                .sda_enable_pullup(false)
                .scl_enable_pullup(false),
        )?;

        let mut ac = Mpu9250::marg_default(d, &mut Ets).expect(
            "Failed to initialize IMU");
        for _ in 1..5 {
            let a = ac.all::<[f32; 3]>().unwrap();
            println!("{:?}", a);
        }
    }

    sleep(Duration::from_secs(2)); // gives escs time to prepare

    /*
    let mut left_motor = ESC::setup(
        &mut peripherals.pins.gpio32,
        &mut peripherals.ledc.timer0,
        &mut peripherals.ledc.channel0,
    )?;
    let mut right_motor = ESC::setup(
        &mut peripherals.pins.gpio33,
        &mut peripherals.ledc.timer1,
        &mut peripherals.ledc.channel1,
    )?;
    */

    println!("Right before running servo tests...");
    servo_tests::test_servo_movement()

    //println!("Right before running test_tween_ramp_up_down...");
    //motor_tests::test_motor_ramp_up_down()
    //motor_tests::test_motor_tween_patterns()
}
