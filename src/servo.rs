use crate::esc::ESC;

use esp_idf_svc::hal::{
    gpio::OutputPin,
    ledc::{config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver, Resolution},
    peripheral::Peripheral,
    prelude::*,
    sys::EspError,
};
use std::{thread::sleep, time::Duration};

pub enum ServoFrequency {
    Standard50Hz,    // Most common for RC servos
    Fast100Hz,       // Faster response
    Ultrafast333Hz,  // High-speed digital servos
}

impl ServoFrequency {
    fn to_hz(&self) -> u32 {
        match self {
            ServoFrequency::Standard50Hz => 50,
            ServoFrequency::Fast100Hz => 100,
            ServoFrequency::Ultrafast333Hz => 333,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ServoResolution {
    Low,      // Range 0-1023
    Medium,   // Range 0-4095
    High,     // Range 0-16383
}

impl ServoResolution {
    fn to_resolution(&self) -> Resolution {
        match self {
            ServoResolution::Low => Resolution::Bits10,
            ServoResolution::Medium => Resolution::Bits12,
            ServoResolution::High => Resolution::Bits14,
        }
    }
}

pub fn setup_servo<'a, P: OutputPin, T: LedcTimer, C: LedcChannel>(
    pin: &'a mut impl Peripheral<P = P>,
    timer: &'a mut impl Peripheral<P = T>,
    chan: &'a mut impl Peripheral<P = C>,
    freq: ServoFrequency,
    res: ServoResolution,
) -> Result<ESC<'a>, EspError> {
    let timer_conf = TimerConfig::default()
        .frequency(freq.to_hz().Hz().into())
        .resolution(res.to_resolution());
        
    ESC::setup(pin, timer, chan)
}

pub fn setup_left_servo<'a>(
    pin: &'a mut impl Peripheral<P = impl OutputPin>,
    timer: &'a mut impl Peripheral<P = impl LedcTimer>,
    channel: &'a mut impl Peripheral<P = impl LedcChannel>,
) -> Result<ESC<'a>, EspError> {
    setup_servo(
        pin,
        timer,
        channel,
        ServoFrequency::Standard50Hz,
        ServoResolution::High,
    )
}

pub fn setup_right_servo<'a>(
    pin: &'a mut impl Peripheral<P = impl OutputPin>,
    timer: &'a mut impl Peripheral<P = impl LedcTimer>,
    channel: &'a mut impl Peripheral<P = impl LedcChannel>,
) -> Result<ESC<'a>, EspError> {
    setup_servo(
        pin,
        timer,
        channel,
        ServoFrequency::Standard50Hz,
        ServoResolution::High,
    )
}
