#!/bin/bash
set -e

NAME=${1%.rs}
echo "Building $NAME"

CROSS=arm-none-eabi
LIBGCC=$($CROSS-gcc -march=armv7-m -print-libgcc-file-name)

rustc --target thumbv7m-none-eabi  -C target-cpu=cortex-m3 \
      -C relocation_model=static -Z no-landing-pads \
      -L . "$NAME.rs" --emit=obj -g  -o "$NAME.o"  --opt-level 2
$CROSS-gcc -c lpc18xx-startup.s
$CROSS-ld -T tessel-ram.ld --gc-sections "$NAME.o" lpc18xx-startup.o $LIBGCC -o "$NAME.elf"
arm-none-eabi-objcopy -O binary "$NAME.elf" "$NAME.bin"
