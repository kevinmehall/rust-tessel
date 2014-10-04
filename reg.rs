use core::prelude::*;
use core::intrinsics::{volatile_load, volatile_store};
use core::ty::Unsafe;

#[repr(C)]
pub struct Reg<T> {
  value: Unsafe<T>,
}

impl<T:Copy> Reg<T> {
  #[inline]
  pub fn get(&self) -> T {
    unsafe {
      volatile_load(self.value.get() as *const T)
    }
  }

  #[inline]
  pub fn set(&self, value: T) {
    unsafe {
      volatile_store(self.value.get(), value)
    }
  }
}


impl Reg<u32> {
  pub fn write_bit(&self, b: u8) {
    self.set(1 << (b as uint));
  }

  pub fn set_bit(&self, b: u8) {
    let old = self.get();
    self.set(old | (1 << (b as uint)));
  }

  pub fn clear_bit(&self, b: u8) {
    let old = self.get();
    self.set(old & !(1 << (b as uint)));
  }
}
