use reg::Reg;

static SCS_BASE: uint = 0xE000E000; // System Control Space Base Address
static SysTick_BASE: uint = SCS_BASE +  0x0010; // SysTick Base Address
static NVIC_BASE: uint = SCS_BASE +  0x0100;    //NVIC Base Address
static SCB_BASE: uint = SCS_BASE +  0x0D00;

#[repr(C)]
pub struct SysTick {
  pub CTRL: Reg<u32>, // Offset: 0x000 (R/W)  SysTick Control and Status Register
  pub LOAD: Reg<u32>, // Offset: 0x004 (R/W)  SysTick Reload Value Register
  pub VAL: Reg<u32>,  // Offset: 0x008 (R/W)  SysTick Current Value Register
  pub CALIB: Reg<u32>,
}

static SysTick_CTRL_COUNTFLAG_Pos: uint = 16;                                    // SysTick CTRL: COUNTFLAG Position
static SysTick_CTRL_COUNTFLAG_Msk: u32 = (1 << SysTick_CTRL_COUNTFLAG_Pos);   // SysTick CTRL: COUNTFLAG Mask

static SysTick_CTRL_CLKSOURCE_Pos: uint =  2;                                    // SysTick CTRL: CLKSOURCE Position
static SysTick_CTRL_CLKSOURCE_Msk: u32 = (1 << SysTick_CTRL_CLKSOURCE_Pos);   // SysTick CTRL: CLKSOURCE Mask

static SysTick_CTRL_TICKINT_Pos: uint =    1;                                    // SysTick CTRL: TICKINT Position
static SysTick_CTRL_TICKINT_Msk: u32 =   (1 << SysTick_CTRL_TICKINT_Pos);    //  SysTick CTRL: TICKINT Mask

static SysTick_CTRL_ENABLE_Pos: uint =     0;                                  // SysTick CTRL: ENABLE Position
static SysTick_CTRL_ENABLE_Msk: u32 =    (1 << SysTick_CTRL_ENABLE_Pos);    // SysTick CTRL: ENABLE Mask


impl SysTick {
  pub fn get() -> &'static SysTick {
    unsafe { &*(SysTick_BASE as *const SysTick) }
  }

  pub fn init(&self, ticks: u32) {
    self.LOAD.set(ticks - 1);
    self.VAL.set(0);
    self.CTRL.set(SysTick_CTRL_CLKSOURCE_Msk | SysTick_CTRL_TICKINT_Msk | SysTick_CTRL_ENABLE_Msk);
  }
}
