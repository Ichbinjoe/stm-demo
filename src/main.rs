#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

use cortex_m_semihosting::*;

use stm32l4::stm32l4x5::*;

#[entry]
unsafe fn main() -> ! {
    hprintln!("Hello, world2!").unwrap();

    let mut peripherals = Peripherals::steal();

    peripherals.RCC.ahb2enr.write(|mut w| {
        w.gpioaen().set_bit();
        w.gpioben().set_bit();
        w.gpiocen().set_bit()
    });

    peripherals.GPIOA.moder.write(|w| w.moder5().output());
    peripherals.GPIOA.ospeedr.write(|w| w.ospeedr5().medium_speed());
    peripherals.GPIOA.otyper.write(|w| w.ot5().push_pull());
    peripherals.GPIOB.moder.write(|w| w.moder14().output());
    peripherals.GPIOB.ospeedr.write(|w| w.ospeedr14().medium_speed());
    peripherals.GPIOB.otyper.write(|w| w.ot14().push_pull());
    peripherals.GPIOC.moder.write(|w| w.moder13().input());

    loop {
        let set = peripherals.GPIOC.idr.read().idr13().bit();
/*
        if set {
            hprintln!("TRUE").unwrap();
        } else {
            hprintln!("FALSE").unwrap();
        }*/

        peripherals.GPIOB.odr.write(|w| w.odr14().bit(!set));
    }
}
