# Rust on Tessel

[Rust](http://rust-lang.org) is a new systems programming language emphasizing safety, concurrency, and speed. It requires no runtime, so it's suitable to use as a better C for microcontroller development. This is a simple demo running Rust on bare metal on a [Tessel](https://tessel.io),
which uses a NXP LPC1800 microcontroller with an ARM Cortex M3 core.

The standard Rust compiler is capable of cross-compiling for ARM. See the [Rust Guide](http://doc.rust-lang.org/guide.html) for installation instructions, as well as learning the language.

You'll also need a copy of [gcc-arm-embedded](https://launchpad.net/gcc-arm-embedded). We won't use its C compiler, but it includes an assembler, linker, and binutils.

Clone this repository:
```.sh
git clone https://github.com/kevinmehall/rust-tessel
cd rust-tessel
```

You need a copy of Rust's `libcore` and `rlibc` compiled for ARM, so clone the `rust` repository and build the library:
```.sh
git clone https://github.com/rust-lang/rust
rustc -O --target=thumbv7m-linux-eabi rust/src/libcore/lib.rs
rustc -O --target=thumbv7m-linux-eabi rust/src/librlibc/lib.rs -L .
```

I've wrapped the commands to compile, link, and produce a binary image into `build.sh` script
```.sh
./build.sh blinky.rs
```

Use the Tessel bootloader to run the executable:
```.sh
tessel boot blinky.bin
```

It runs from RAM, so press the Reset button to return to normal Tessel firmware.
