#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f4xx_hal as mcu;

#[cfg(feature = "defmt")]
use defmt_rtt as _; // global logger

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use crate::mcu::{
    pac,
    prelude::*,
}
use panic_probe as _;

use rustBoot_hal::stm::stm32f411::FlashWriterEraser;
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};

// struct Leds {
//     green: PD12<gpio::Output<gpio::PushPull>>,
// }

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpio = dp.GPIOA.split();
    let mut led = gpio.pa5.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();    
    let mut count = 0;
    
    let mut delay = dp.TIM1.delay_ms(&clocks);

    while count < 6 {
        led.toggle();
        delay.delay(500.millis());
        count = count + 1;
    }
    
    let flash1 = dp.FLASH;
    
       
    let flash_writer = FlashWriterEraser { nvm: flash1 };
    let updater = FlashUpdater::new(flash_writer);
    
    match updater.update_trigger() {
        Ok(_v) => {}
        Err(e) => panic!("couldnt trigger update: {}", e),
    }
    //nvic_systemreset();
    mcu::pac::SCB::sys_reset()
}
