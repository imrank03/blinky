#![no_main]
#![no_std]
#![allow(warnings)]

use panic_halt as _;
use defmt_rtt as _;

use stm32f0xx_hal as hal;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use crate::hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

        let gpioa = p.GPIOA.split(&mut rcc);

        // configure PA5 as output
        let mut led = cortex_m::interrupt::free(move |cs| gpioa.pa5.into_push_pull_output(cs));

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &rcc);

        loop {
            led.set_high().ok();
            delay.delay_ms(1_000_u16);
            led.set_low().ok();
            delay.delay_ms(1_000_u16);
        }
 
    }
    
    loop {
        
    }
}