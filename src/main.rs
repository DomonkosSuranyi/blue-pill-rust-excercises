#![no_std]
#![no_main]

// Panic handler
// semihosting for debugability
extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    prelude::*,
    timer::Timer,
};
use embedded_hal::digital::v2::OutputPin;
use nb::block;


#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32f1xx_hal::stm32::Peripherals::take().unwrap();

    let mut rcc = device.RCC.constrain();
    let mut flash = device.FLASH.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer = Timer::syst(core.SYST, &clocks).start_count_down(1.hz());
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}
