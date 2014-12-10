use reg::Reg;

const SCS_BASE: uint = 0xE000E000; // System Control Space Base Address
const SYSTICK_BASE: uint = SCS_BASE +  0x0010; // SysTick Base Address
const NVIC_BASE: uint = SCS_BASE +  0x0100;    //NVIC Base Address
const SCB_BASE: uint = SCS_BASE +  0x0D00;

#[repr(C)]
pub struct SysTick {
  pub CTRL: Reg<u32>, // Offset: 0x000 (R/W)  SysTick Control and Status Register
  pub LOAD: Reg<u32>, // Offset: 0x004 (R/W)  SysTick Reload Value Register
  pub VAL: Reg<u32>,  // Offset: 0x008 (R/W)  SysTick Current Value Register
  pub CALIB: Reg<u32>,
}

const SYSTICK_CTRL_COUNTFLAG_POS: uint = 16;                                    // SysTick CTRL: COUNTFLAG Position
const SYSTICK_CTRL_COUNTFLAG_MSK: u32 = (1 << SYSTICK_CTRL_COUNTFLAG_POS);   // SysTick CTRL: COUNTFLAG Mask

const SYSTICK_CTRL_CLKSOURCE_POS: uint =  2;                                    // SysTick CTRL: CLKSOURCE Position
const SYSTICK_CTRL_CLKSOURCE_MSK: u32 = (1 << SYSTICK_CTRL_CLKSOURCE_POS);   // SysTick CTRL: CLKSOURCE Mask

const SYSTICK_CTRL_TICKINT_POS: uint =    1;                                    // SysTick CTRL: TICKINT Position
const SYSTICK_CTRL_TICKINT_MSK: u32 =   (1 << SYSTICK_CTRL_TICKINT_POS);    //  SysTick CTRL: TICKINT Mask

const SYSTICK_CTRL_ENABLE_POS: uint =     0;                                  // SysTick CTRL: ENABLE Position
const SYSTICK_CTRL_ENABLE_MSK: u32 =    (1 << SYSTICK_CTRL_ENABLE_POS);    // SysTick CTRL: ENABLE Mask


impl SysTick {
  pub fn get() -> &'static SysTick {
    unsafe { &*(SYSTICK_BASE as *const SysTick) }
  }

  pub fn init(&self, ticks: u32) {
    self.LOAD.set(ticks - 1);
    self.VAL.set(0);
    self.CTRL.set(SYSTICK_CTRL_CLKSOURCE_MSK | SYSTICK_CTRL_TICKINT_MSK | SYSTICK_CTRL_ENABLE_MSK);
  }
}
