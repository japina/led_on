#![no_std] // using embedded so no std libraries
#![no_main]

use cortex_m_rt::entry; // The runtime
use stm32l1::stm32l151;
#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {
    let dp = stm32l151::Peripherals::take().unwrap();
    dp.RCC
        .ahbenr
        .write(|w| w.gpiopben().set_bit());
    dp.GPIOB.moder.write(|w| w.moder7().bits(0b01));
    dp.GPIOB.odr.write(|w| w.odr7().set_bit());
    loop {
    }
}