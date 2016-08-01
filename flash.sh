#!/bin/sh

MBED_CMSIS=lancaster-university/mbed-classic/targets/cmsis
MICROBIT_TARGET=lancaster-university/microbit-targets/bbc-microbit-classic-gcc

set -e
cd $(dirname $0)
cargo build --target=cortex-m0 --release
arm-none-eabi-g++ \
    -Wl,--gc-sections \
    -Wl,--sort-section=alignment \
    -mcpu=cortex-m0 \
    -mthumb \
    -T$MICROBIT_TARGET/ld/NRF51822.ld \
    $MBED_CMSIS/TARGET_NORDIC/TARGET_MCU_NRF51822/TOOLCHAIN_GCC_ARM/startup_NRF51822.S \
    target/cortex-m0/release/librust_on_bbc_microbit.a \
    -Wl,--start-group \
    -lnosys \
    -lc \
    -lgcc \
    -lc \
    -Wl,--end-group \
    -o target/bin
arm-none-eabi-objcopy -O ihex target/bin target/hex
srec_cat \
    $MICROBIT_TARGET/bootloader/BLE_BOOTLOADER_RESERVED.hex -intel \
    $MICROBIT_TARGET/softdevice/s110_nrf51822_8.0.0_softdevice.hex -intel \
    target/hex -intel \
    -o target/combined.hex -intel
echo Flashing…
cp target/combined.hex /run/media/$USER/MICROBIT/
