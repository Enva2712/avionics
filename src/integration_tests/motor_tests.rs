use crate::esc::ESC;
use esp_idf_svc::hal::{peripherals::Peripherals, prelude::*};
use std::{thread::sleep, time::Duration};

pub fn test_motor_ramp_up_down() -> anyhow::Result<()> {
    //esp_idf_svc::sys::link_patches();
    //esp_idf_svc::log::EspLogger::initialize_default();
    let mut peripherals = Peripherals::take()?;

    println!("Initializing motors...");
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

    println!("Testing motor speed control...");
    for i in (5..=40).step_by(5) {
        println!("Setting motors to {}%", i);
        right_motor.set_speed(i)?;
        left_motor.set_speed(i)?;
        sleep(Duration::from_secs(2));
        println!("Stopping motors");
        right_motor.set_speed(0)?;
        left_motor.set_speed(0)?;
        sleep(Duration::from_secs(2));
    }

    Ok(())
}

pub fn test_motor_tween_patterns() -> anyhow::Result<()> {
    let mut peripherals = Peripherals::take()?;

    println!("Initializing motors...");
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

    // Test pattern for left motor
    println!("Testing left motor tween patterns...");
    println!("Slow ramp up to 5%...");
    left_motor.tween_to(5, Duration::from_secs(20))?;
    println!("Faster ramp down to 0%...");
    left_motor.tween_to(0, Duration::from_secs(10))?;
    println!("Medium ramp up to 10%...");
    left_motor.tween_to(10, Duration::from_secs(15))?;
    println!("Slow ramp down to 2.5%...");
    left_motor.tween_to(2, Duration::from_secs(20))?;
    println!("Quick stop...");
    left_motor.tween_to(0, Duration::from_secs(5))?;

    sleep(Duration::from_secs(5)); // Pause between motors

    // Test pattern for right motor
    println!("Testing right motor tween patterns...");
    println!("Slow ramp up to 5%...");
    right_motor.tween_to(5, Duration::from_secs(20))?;
    println!("Faster ramp down to 0%...");
    right_motor.tween_to(0, Duration::from_secs(10))?;
    println!("Medium ramp up to 10%...");
    right_motor.tween_to(10, Duration::from_secs(15))?;
    println!("Slow ramp down to 2.5%...");
    right_motor.tween_to(2, Duration::from_secs(20))?;
    println!("Quick stop...");
    right_motor.tween_to(0, Duration::from_secs(5))?;

    Ok(())
}


