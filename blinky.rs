#![no_std]
#![feature(globs, lang_items, asm)]
#![crate_type="rlib"]

#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate core;
extern crate rlibc;

use core::prelude::*;
use arm::SysTick;
use lpc1800::{GPIO, SCU, LED1, LED1_GPIO, LED2, LED2_GPIO, PUP_DISABLE};

pub mod support;
pub mod reg;
pub mod arm;
pub mod lpc1800;

#[no_mangle]
#[no_split_stack]
pub unsafe extern "C" fn _reset_handler() {
  let systick = SysTick::get();
  let scu = SCU::get();
  systick.init(180_000_000 / 1000);

  scu.pinmux(LED1, PUP_DISABLE, 4);
  scu.pinmux(LED2, PUP_DISABLE, 0);

  let gpio = GPIO::get();
  gpio.write(LED1_GPIO, true);
  gpio.dir(LED1_GPIO, true);
  gpio.write(LED2_GPIO, false);
  gpio.dir(LED2_GPIO, true);

  asm!("CPSIE i");
  loop{}
}

static mut count: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn SysTick_Handler() {
  let gpio = GPIO::get();
  count += 1;
  if count % 100 == 0 {
    gpio.toggle(LED1_GPIO);
    gpio.toggle(LED2_GPIO);
  }
}
