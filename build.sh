#!/bin/bash
set -e

NAME=${1%.rs}
echo "Building $NAME"

rustc --target thumbv7m-linux-eabi -C target-cpu=cortex-m3 -L . "$NAME.rs" --emit=obj -g  -o "$NAME.o" -Z no-landing-pads -O
arm-none-eabi-gcc -g -T tessel-ram.ld -ffunction-sections -lm -lc -lnosys -Wl,--gc-sections lpc18xx-startup.s "$NAME.o" -o "$NAME.elf"
arm-none-eabi-objcopy -O binary "$NAME.elf" "$NAME.bin"
