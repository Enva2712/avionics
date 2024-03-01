use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{AnyIOPin, OutputPin},
    i2c::{I2cConfig, I2cDriver},
    ledc::{config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver, Resolution},
    peripheral::Peripheral,
    peripherals::Peripherals,
    prelude::*,
    sys::EspError,
};
use mpu9250::Mpu9250;
use std::{thread::sleep, time::Duration};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut peripherals = Peripherals::take()?;
    let d = I2cDriver::new(
        peripherals.i2c0,
        Into::<AnyIOPin>::into(peripherals.pins.gpio21),
        Into::<AnyIOPin>::into(peripherals.pins.gpio22),
        &I2cConfig::new().baudrate(100.kHz().into()),
    )?;

    let mut _ac = Mpu9250::marg_default(d, &mut Ets); // .expect("Failed to initialize IMU");

    /*

    loop {
        let a = ac.all::<[f32; 3]>().unwrap();
        println!("{:?}", a);
    }
    */

    let mut left_motor = ESC::new(
        &mut peripherals.pins.gpio32,
        &mut peripherals.ledc.timer0,
        &mut peripherals.ledc.channel0,
    )?;
    let mut _right_motor = ESC::new(
        &mut peripherals.pins.gpio33,
        &mut peripherals.ledc.timer1,
        &mut peripherals.ledc.channel1,
    )?;

    println!("start");
    left_motor.set_speed(0)?;
    sleep(Duration::from_secs(1));
    println!("raising");
    left_motor.tween_to(10000, Duration::from_millis(2000))?;
    println!("high");
    sleep(Duration::from_millis(4000));
    println!("lowering");
    left_motor.tween_to(0, Duration::from_millis(3000))?;
    println!("low");

    // right_motor.tween_to(0, Duration::from_millis(2000))?;
    // right_motor.tween_to(10000, Duration::from_millis(2000))?;
    // right_motor.tween_to(0, Duration::from_millis(2000))?;

    Ok(())
}

struct ESC<'a> {
    pwm_driver: LedcDriver<'a>,
    timer_conf: TimerConfig,
}

impl<'a> ESC<'a> {
    fn new<P: OutputPin, T: LedcTimer, C: LedcChannel>(
        pin: &'a mut impl Peripheral<P = P>,
        timer: &'a mut impl Peripheral<P = T>,
        chan: &'a mut impl Peripheral<P = C>,
    ) -> Result<Self, EspError> {
        let timer_conf = TimerConfig::default()
            .frequency(50.Hz().into())
            .resolution(Resolution::Bits14);
        let td = LedcTimerDriver::new(timer, &timer_conf)?;
        let pwm_driver = LedcDriver::new(chan, td, pin)?;
        Ok(Self {
            timer_conf,
            pwm_driver,
        })
    }

    fn set_speed(&mut self, basis_points: u32) -> Result<(), EspError> {
        let d = basis_points * self.timer_conf.resolution.max_duty() / 10000;
        self.pwm_driver.set_duty(d)?;
        Ok(())
    }

    fn tween_to(&mut self, basis_points: u32, d: Duration) -> Result<(), EspError> {
        let cur = self.pwm_driver.get_duty() * 10000 / self.timer_conf.resolution.max_duty();
        let step_count = basis_points.abs_diff(cur);
        let step_duration = if step_count == 0 {
            Duration::from_millis(0)
        } else {
            d / step_count
        };
        // TODO: remove box dyn indirection
        let rng: Box<dyn Iterator<Item = u32>> = if basis_points > cur {
            Box::new(cur..=basis_points)
        } else {
            Box::new((basis_points..=cur).rev())
        };
        for setpoint in rng {
            sleep(step_duration);
            self.set_speed(setpoint)?;
            if setpoint % 100 == 0 {
                println!("{}", setpoint);
            }
        }
        Ok(())
    }
}
