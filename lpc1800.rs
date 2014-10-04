use reg::Reg;

static SCU_BASE: u32 = 0x40086000;

#[repr(C)]
pub struct SCU {
  pub SFSP: [[Reg<u32>, ..32], ..16],
  pub RSVD1: [u32, ..256],
  pub SFSCLK: [Reg<u32>, ..4],
  pub RSVD2: [u32, ..28],
  pub SFSUSB: Reg<u32>,
  pub SFSI2C0: Reg<u32>,
  pub SFSENAIO: [Reg<u32>, ..3],
  pub RSVD3: [u32, ..27],
  pub EMCDELAYCLK: Reg<u32>,
  pub RSVD4: [u32, ..63],
  pub PINTSEL: [Reg<u32>, ..2],
}

pub static PDN_ENABLE: u32 =        (1 << 3);	// Pull-down enable
pub static PDN_DISABLE: u32 =       (0 << 3);      // Pull-down disable
pub static PUP_ENABLE: u32 =        (0 << 4);      // Pull-up enable
pub static PUP_DISABLE: u32 =       (1 << 4);	// Pull-up disable
pub static SLEWRATE_SLOW: u32 =	  (0 << 5);	// Slew rate for low noise with medium speed
pub static INBUF_ENABLE: u32 =	  (1 << 6);	// Input buffer
pub static INBUF_DISABLE: u32 =	  (0 << 6);	// Input buffer
pub static FILTER_ENABLE: u32 =	  (0 << 7);	// Glitch filter (for signals below 30MHz)
pub static FILTER_DISABLE: u32 =	  (1 << 7);	// No glitch filter (for signals above 30MHz)
pub static DRIVE_8MA: u32 =         (1 << 8);	// Drive strength of 8mA
pub static DRIVE_14MA: u32 =        (1 << 9);	// Drive strength of 14mA
pub static DRIVE_20MA: u32 =        (3 << 8);	// Drive strength of 20mA

impl SCU {
  pub fn get() -> &'static SCU {
    unsafe { &*(SCU_BASE as *const SCU) }
  }

  pub fn pinmux(&self, (port, pin): (u8, u8), mode: u32, func: u32) {
    self.SFSP[port as uint][pin as uint].set(mode | func);
  }
}

pub static LED1: (u8, u8) = (11, 2);
pub static LED2: (u8, u8) = (7, 3);

static GPIO_PORT_BASE: u32 = 0x400F4000;

#[repr(C)]
pub struct GPIO {
  pub B: [Reg<u8>, ..256],
  pub RESERVED0: [u32, ..960],
  pub W: [Reg<u32>, ..256],
  pub RESERVED1: [u32, ..768],
  pub DIR: [Reg<u32>, ..8],
  pub RESERVED2: [u32, ..24],
  pub MASK: [Reg<u32>, ..8],
  pub RESERVED3: [u32, ..24],
  pub PIN: [Reg<u32>, ..8],
  pub RESERVED4: [u32, ..24],
  pub MPIN: [Reg<u32>, ..8],
  pub RESERVED5: [u32, ..24],
  pub SET: [Reg<u32>, ..8],
  pub RESERVED6: [u32, ..24],
  pub CLR: [Reg<u32>, ..8],
  pub RESERVED7: [u32, ..24],
  pub NOT: [Reg<u32>, ..8],
  pub RESERVED8: [u32, ..24],
}

impl GPIO {
  pub fn get() -> &'static GPIO {
    unsafe { &*(GPIO_PORT_BASE as *const GPIO) }
  }

  pub fn dir(&self, (port, pin): (u8, u8), output: bool) {
    if output {
      self.DIR[port as uint].set_bit(pin);
    } else {
      self.DIR[port as uint].clear_bit(pin);
    }
  }

  pub fn write(&self, (port, pin): (u8, u8), value: bool) {
    if value {
      self.SET[port as uint].write_bit(pin);
    } else {
      self.CLR[port as uint].write_bit(pin);
    }
  }

  pub fn toggle(&self, (port, pin): (u8, u8)) {
    self.NOT[port as uint].write_bit(pin);
  }
}

pub static LED1_GPIO: (u8, u8) = (5, 22);
pub static LED2_GPIO: (u8, u8) = (3, 11);
