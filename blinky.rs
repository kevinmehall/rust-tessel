#![no_std]
#![no_main]
#![feature(globs)]
#![feature(lang_items)]
#![feature(asm)]

#![allow(uppercase_variables)]
#![allow(non_camel_case_types)]

extern crate core;

use core::prelude::*;

use core::mem;
use core::raw::Slice;
use core::intrinsics::{volatile_load, volatile_store};
use core::ty::Unsafe;

#[lang = "begin_unwind"]
extern fn begin_unwind(args: &core::fmt::Arguments,
                       file: &str,
                       line: uint) -> ! {
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle]
pub unsafe extern "C" fn _kill() { loop{} }

#[no_mangle]
pub unsafe extern "C" fn _getpid() { loop{} }

static SCS_BASE: uint = 0xE000E000; // System Control Space Base Address
static SysTick_BASE: uint = SCS_BASE +  0x0010; // SysTick Base Address
static NVIC_BASE: uint = SCS_BASE +  0x0100;    //NVIC Base Address
static SCB_BASE: uint = SCS_BASE +  0x0D00;

#[repr(C)]
struct SysTick {
  CTRL: Reg<u32>, // Offset: 0x000 (R/W)  SysTick Control and Status Register
  LOAD: Reg<u32>, // Offset: 0x004 (R/W)  SysTick Reload Value Register
  VAL: Reg<u32>,  // Offset: 0x008 (R/W)  SysTick Current Value Register
  CALIB: Reg<u32>,
}

static SysTick_CTRL_COUNTFLAG_Pos: uint = 16;                                    // SysTick CTRL: COUNTFLAG Position
static SysTick_CTRL_COUNTFLAG_Msk: u32 = (1 << SysTick_CTRL_COUNTFLAG_Pos);   // SysTick CTRL: COUNTFLAG Mask

static SysTick_CTRL_CLKSOURCE_Pos: uint =  2;                                    // SysTick CTRL: CLKSOURCE Position
static SysTick_CTRL_CLKSOURCE_Msk: u32 = (1 << SysTick_CTRL_CLKSOURCE_Pos);   // SysTick CTRL: CLKSOURCE Mask

static SysTick_CTRL_TICKINT_Pos: uint =    1;                                    // SysTick CTRL: TICKINT Position
static SysTick_CTRL_TICKINT_Msk: u32 =   (1 << SysTick_CTRL_TICKINT_Pos);    //  SysTick CTRL: TICKINT Mask

static SysTick_CTRL_ENABLE_Pos: uint =     0;                                  // SysTick CTRL: ENABLE Position
static SysTick_CTRL_ENABLE_Msk: u32 =    (1 << SysTick_CTRL_ENABLE_Pos);    // SysTick CTRL: ENABLE Mask

#[repr(C)]
struct Reg<T> {
  value: Unsafe<T>,
}

impl<T:Copy> Reg<T> {
  #[inline]
  fn get(&self) -> T {
    unsafe {
      volatile_load(self.value.get() as *T)
    }
  }

  #[inline]
  fn set(&self, value: T) {
    unsafe {
      volatile_store(self.value.get(), value)
    }
  }
}


impl Reg<u32> {
  fn write_bit(&self, b: u8) {
    self.set(1 << (b as uint));
  }

  fn set_bit(&self, b: u8) {
    let old = self.get();
    self.set(old | (1 << (b as uint)));
  }

  fn clear_bit(&self, b: u8) {
    let old = self.get();
    self.set(old & !(1 << (b as uint)));
  }
}


impl SysTick {
  fn get() -> &'static SysTick {
    unsafe { &*(SysTick_BASE as *SysTick) }
  }

  fn init(&self, ticks: u32) {
    self.LOAD.set(ticks - 1);
    self.VAL.set(0);
    self.CTRL.set(SysTick_CTRL_CLKSOURCE_Msk | SysTick_CTRL_TICKINT_Msk | SysTick_CTRL_ENABLE_Msk);
  }
}

static SCU_BASE: u32 = 0x40086000;

#[repr(C)]
struct SCU {
  SFSP: [[Reg<u32>, ..32], ..16],
  RSVD1: [u32, ..256],
  SFSCLK: [Reg<u32>, ..4],
  RSVD2: [u32, ..28],
  SFSUSB: Reg<u32>,
  SFSI2C0: Reg<u32>,
  SFSENAIO: [Reg<u32>, ..3],
  RSVD3: [u32, ..27],
  EMCDELAYCLK: Reg<u32>,
  RSVD4: [u32, ..63],
  PINTSEL: [Reg<u32>, ..2],
}

static PDN_ENABLE: u32 =        (1 << 3);	// Pull-down enable
static PDN_DISABLE: u32 =       (0 << 3);      // Pull-down disable
static PUP_ENABLE: u32 =        (0 << 4);      // Pull-up enable
static PUP_DISABLE: u32 =       (1 << 4);	// Pull-up disable
static SLEWRATE_SLOW: u32 =	  (0 << 5);	// Slew rate for low noise with medium speed
static SLEWRATE_FAST: u32 =	  (1 << 5);	// Slew rate for medium noise with fast speed
static INBUF_ENABLE: u32 =	  (1 << 6);	// Input buffer
static INBUF_DISABLE: u32 =	  (0 << 6);	// Input buffer
static FILTER_ENABLE: u32 =	  (0 << 7);	// Glitch filter (for signals below 30MHz)
static FILTER_DISABLE: u32 =	  (1 << 7);	// No glitch filter (for signals above 30MHz)
static DRIVE_8MA: u32 =         (1 << 8);	// Drive strength of 8mA
static DRIVE_14MA: u32 =        (1 << 9);	// Drive strength of 14mA
static DRIVE_20MA: u32 =        (3 << 8);	// Drive strength of 20mA

impl SCU {
  fn get() -> &'static SCU {
    unsafe { &*(SCU_BASE as *SCU) }
  }

  fn pinmux(&self, (port, pin): (u8, u8), mode: u32, func: u32) {
    self.SFSP[port as uint][pin as uint].set(mode | func);
  }
}

static LED1: (u8, u8) = (11, 2);
static LED2: (u8, u8) = (7, 3);

static GPIO_PORT_BASE: u32 = 0x400F4000;

#[repr(C)]
struct GPIO {
  B: [Reg<u8>, ..256],
  RESERVED0: [u32, ..960],
  W: [Reg<u32>, ..256],
  RESERVED1: [u32, ..768],
  DIR: [Reg<u32>, ..8],
  RESERVED2: [u32, ..24],
  MASK: [Reg<u32>, ..8],
  RESERVED3: [u32, ..24],
  PIN: [Reg<u32>, ..8],
  RESERVED4: [u32, ..24],
  MPIN: [Reg<u32>, ..8],
  RESERVED5: [u32, ..24],
  SET: [Reg<u32>, ..8],
  RESERVED6: [u32, ..24],
  CLR: [Reg<u32>, ..8],
  RESERVED7: [u32, ..24],
  NOT: [Reg<u32>, ..8],
  RESERVED8: [u32, ..24],
}

impl GPIO {
  fn get() -> &'static GPIO {
    unsafe { &*(GPIO_PORT_BASE as *GPIO) }
  }

  fn dir(&self, (port, pin): (u8, u8), output: bool) {
    if output {
      self.DIR[port as uint].set_bit(pin);
    } else {
      self.DIR[port as uint].clear_bit(pin);
    }
  }

  fn write(&self, (port, pin): (u8, u8), value: bool) {
    if value {
      self.SET[port as uint].write_bit(pin);
    } else {
      self.CLR[port as uint].write_bit(pin);
    }
  }

  fn toggle(&self, (port, pin): (u8, u8)) {
    self.NOT[port as uint].write_bit(pin);
  }
}

static LED1_GPIO: (u8, u8) = (5, 22);
static LED2_GPIO: (u8, u8) = (3, 11);

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
