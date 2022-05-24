#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f4xx_hal as mcu;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use crate::mcu::{
    pac,
    prelude::*,
}
use panic_probe as _;

use rustBoot_hal::stm::stm32f411::FlashWriterEraser;
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpio = dp.GPIOA.split();
    let mut led = gpio.pa5.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();    

    let mut delay = dp.TIM1.delay_ms(&clocks);

    while count < 6 {
        led.toggle();
        delay.delay(1000.millis());
        count = count + 1;
    }
    
    let flash1 = dp.FLASH;

    let flash_writer = FlashWriterEraser { nvm: flash1 };
    let updater = FlashUpdater::new(flash_writer);
    match updater.update_success() {
        Ok(_v) => {}
        Err(e) => panic!("couldnt trigger update: {}", e),
    }

    loop {
        leds.red.toggle();
        delay.delay_ms(1000_u16);
    }
}
    loop {}
}
