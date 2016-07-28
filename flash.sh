#!/bin/sh

# https://github.com/lancaster-university/mbed-classic
MBED_CLASSIC=../lancaster-university/mbed-classic

# https://github.com/lancaster-university/microbit-targets
MICROBIT_TARGETS=../lancaster-university/microbit-targets

MICROBIT_TARGET=$MICROBIT_TARGETS/bbc-microbit-classic-gcc
BOOTLOADER=$MICROBIT_TARGET/bootloader/BLE_BOOTLOADER_RESERVED.hex
SOFTDEVICE=$MICROBIT_TARGET/softdevice/s110_nrf51822_8.0.0_softdevice.hex

set -e

cd $(dirname $0)

cargo build --target=cortex-m0 --release

arm-none-eabi-g++ \
    -fno-exceptions \
    -fno-unwind-tables \
    -Wl,--gc-sections \
    -Wl,--sort-common \
    -Wl,--sort-section=alignment \
    -Wl,-wrap,main \
    -mcpu=cortex-m0 \
    -mthumb \
    -T$MICROBIT_TARGET/ld/NRF51822.ld \
    -Wl,--start-group \
    $MBED_CLASSIC/targets/cmsis/TARGET_NORDIC/TARGET_MCU_NRF51822/TOOLCHAIN_GCC_ARM/startup_NRF51822.S \
    target/cortex-m0/release/librust.a \
    -lnosys \
    -lstdc++ \
    -lsupc++ \
    -lm \
    -lc \
    -lgcc \
    -lstdc++ \
    -lsupc++ \
    -lm \
    -lc \
    -lgcc \
    -Wl,--end-group \
    -o target/bin

arm-none-eabi-objcopy -O ihex target/bin target/hex

srec_cat $BOOTLOADER -intel $SOFTDEVICE -intel target/hex -intel \
    -o target/combined.hex -intel --line-length=44

cp target/combined.hex /run/media/simon/MICROBIT/
