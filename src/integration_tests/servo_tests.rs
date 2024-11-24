use crate::servo::{setup_left_servo, setup_right_servo};
use esp_idf_svc::hal::{peripherals::Peripherals, prelude::*};
use std::{thread::sleep, time::Duration};

pub fn test_servo_movement() -> anyhow::Result<()> {
    let mut peripherals = Peripherals::take()?;
    
    println!("Initializing servos...");
    let mut left_servo = setup_left_servo(
        &mut peripherals.pins.gpio12,
        &mut peripherals.ledc.timer2,
        &mut peripherals.ledc.channel2,
    )?;
    let mut right_servo = setup_right_servo(
        &mut peripherals.pins.gpio13,
        &mut peripherals.ledc.timer3,
        &mut peripherals.ledc.channel3,
    )?;

    println!("Beginning servo movement test sequence...");
    
    // Initial gentle movement
    println!("Testing initial movement (10% over 5 seconds)...");
    left_servo.tween_to(10, Duration::from_secs(5))?;
    right_servo.tween_to(10, Duration::from_secs(5))?;
    println!("Holding position...");
    sleep(Duration::from_secs(2));

    // Medium movement
    println!("Moving to 25% over 10 seconds...");
    left_servo.tween_to(25, Duration::from_secs(10))?;
    right_servo.tween_to(25, Duration::from_secs(10))?;
    println!("Holding position...");
    sleep(Duration::from_secs(3));

    // Testing independent movement
    println!("Testing independent servo movement...");
    println!("Left servo to 40%, right servo to 15%...");
    left_servo.tween_to(40, Duration::from_secs(8))?;
    right_servo.tween_to(15, Duration::from_secs(4))?;
    println!("Holding asymmetric position...");
    sleep(Duration::from_secs(4));

    // Synchronize back
    println!("Synchronizing both servos to 30%...");
    left_servo.tween_to(30, Duration::from_secs(15))?;
    right_servo.tween_to(30, Duration::from_secs(15))?;
    println!("Holding synchronized position...");
    sleep(Duration::from_secs(5));

    // Return to zero
    println!("Returning to 0% over 20 seconds...");
    left_servo.tween_to(0, Duration::from_secs(20))?;
    right_servo.tween_to(0, Duration::from_secs(20))?;
    println!("Test sequence completed.");

    Ok(())
}
