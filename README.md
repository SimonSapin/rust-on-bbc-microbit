# Rust on BBC micro:bit

This is where I play with BBC’s [micro:bit](https://www.microbit.co.uk/about)
and put some [Rust](http://rust-lang.org/) code on it.

It’s a tiny board with an ARM micro-controller and a bunch of peripherals on it.

<img src=pictures/front.jpg height=400>
<img src=pictures/back.jpg height=400>

It will be / is being given away to every Year 7 (~11 years old) kid in UK schools,
and can now be [bought](http://uk.farnell.com/bbc-microbit-reseller) by anyone.


## MicroPython

The microbit has a dedicated [online environment](https://www.microbit.co.uk/app/)
where you can program it in a few different high-level languages, right from your browser.
There’s a button to compile a program and download it as a `.hex` file.

When connected to a computer, the microbit acts as a USB Mass Storage device
(like a "memory stick").
Copying a `.hex` file into it flashes the program onto the micro-controller.
This is much easier to set up than Arduino
where you typically need a dedicated tool on your computer.

Let’s try a [MicroPython](https://micropython.org/) program:

```python
from microbit import *

while True:
    display.scroll('Hello Python!')
    sleep(2000)
```

And it runs!

<img src=pictures/python.gif alt="
    The microbit 5x5 LED display shows “Hello Python!”,
    scrolling one character at a time.">


## C++ and Lancaster University

[mbed](https://www.mbed.com/) is a C/C++ ecosystem developped by ARM for embedded systems.
It includes among other things a hardware and devices abstraction layer,
and [yotta](http://yotta.mbed.com/), a package manager and build system.

Lancaster University developped a C/C++ [runtime environment](
https://lancaster-university.github.io/microbit-docs/)
for code running on the microbit, based on mbed and yotta.
This is what MicroPython and other high-level languages for the microbit are based on.

Here again there’s an online development environment, but I went with [the offline one](
https://lancaster-university.github.io/microbit-docs/offline-toolchains).
The tools needed are:

* [Yotta](http://docs.yottabuild.org/#installing)
* [srecord](http://srecord.sourceforge.net/)
* A C and C++ cross-compiler for ARM embedded platforms (without an operating system).

I chose to install yotta with [pip](https://pip.pypa.io/en/stable/)
in a Python [virtualenv](http://virtualenv.org/),
and the other two from system packages.
On Arch Linux, the package names are `srecord` and `arm-none-eabi-gcc`.

The next step is to clone the [`microbit-samples` repository](
https://github.com/lancaster-university/microbit-samples) and run `yotta build` from there.
Yotta will automatically download the dependencies and,
since `microbit-samples` is pre-configured to target the microbit hardware,
the target definitions.

At this point, yotta asked me to log in with an mbed account.
I find it very strange to require an account just to download open-source code
(as opposed to publishing anything, where you want to control who can (re-)publish)
and didn’t feel like providing identifying information to yet another commercial company,
so I looked for another solution.

It turns out that:

* yotta keeps target definitions in a `yotta_targets` directory
* It doesn’t try to download them if they’re already there
* At least for the microbit they’re also available on GitHub
* For some reason, only downloading target definitions requires an account,
  not *modules* (the yotta name for software packages / dependencies)

So, all together:

```sh
git clone https://github.com/lancaster-university/microbit-samples
git clone https://github.com/lancaster-university/
git clone https://github.com/ARMmbed/target-mbed-gcc -b v0.1.3

cd microbit-samples
mkdir yotta_targets
ln -s ../microbit-targets/bbc-microbit-classic-gcc yotta_targets/bbc-microbit-classic-gcc
ln -s ../target-mbed-gcc yotta_targets/mbed-gcc
```

Now `yotta build` should run without an mbed account.
Let’s tweak the code at `source/main.cpp` a bit:


```cpp
#include "MicroBit.h"

MicroBit uBit;

int main() {
    for (;;) {
        uBit.display.scroll("Hello C++!");
        uBit.sleep(2000);
    }
}
```

And build again.
Now `build/bbc-microbit-classic-gcc/source/microbit-samples-combined.hex`
is the file we can copy to the microbit USB device to flash the micro-controller.

**Note:** make sure to pick the file with `combined.hex` in the name.
There’s also a `microbit-samples.hex` file which does not contain the bootloader.
It will apparently flash correctly, but then the program won’t run.

And it runs!

<img src=pictures/cpp.gif alt="The microbit 5x5 LED display shows “Hello C++!”">

(Note: in some frames of this GIF it looks like too many "pixels" of the display are on.
 This is due to the conversion to a low frame rate to keep the GIF’s size reasonable,
 not an actual problem with the microbit hardware or the code running on it.)

From the outside this looks very similar to the previous “Hello Python” example,
what’s going here is very different.

With MicroPython, source code was compile into a MicroPython-specific bytecode
that is interpreted by virtual machine.
This VM, in turn, is what the micro-controller runs.
With C++, source code is compiled into ARM machine code that the micro-controller runs directly.
We went “down” one layer of abstraction.


## Cross-compiling Rust

**Note:** This was written in July 2016,
and the cross-compilation tooling in the Rust ecosystem is changing rapidly.
This is intended more of a record of what I did at one point in time
than a reference guide that will be maintained up-to-date.

[Cargo](http://doc.crates.io/) (Rust’s package manager and build system)
distinguishes between “binaries” (executable programs) and libraries.
Since the microbit has no operating system, it has no concept of separate programs either.
Let’s start with a Rust library:

```
cargo new rust-on-bbc-microbit
cd rust-on-bbc-microbit
cargo build
```

By default, compiling Rust code produces machine code for the same “target”
(CPU and operating system) as where the compiler is running.
For my laptop, that’s Intel x86_64 and Linux.

Rust and Cargo support [cross-compilation](https://blog.rust-lang.org/2016/05/13/rustup.html)
to create programs that run in an environment different from where they’re compiled.

On the microbit, we have an ARM Cortex-M0 CPU and no operating system at all.
Unfortunately, this particular configuration is not
([yet?](https://github.com/rust-lang/rfcs/pull/1645))
one of the targets that Rust supports by default.
So we’ll need to configure it ourselves with a `cortex-m0.json` target file:

```json
{
    "arch": "arm",
    "data-layout": "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64",
    "executables": true,
    "os": "none",
    "pre-link-args": ["-Wl,-("],
    "post-link-args": ["-Wl,-)"],
    "target-endian": "little",
    "target-pointer-width": "32",
    "cpu": "cortex-m0",
    "llvm-target": "thumbv6m-none-eabi",
    "features": "+strict-align",
    "relocation-model": "static",
    "no-compiler-rt": true
}
```

I don’t understand most of it,
but the details turn out to be important as I found out later.
We can now try to build (cross-compile) for this target…

```sh
cargo build --target cortex-m0
```
```rust
   Compiling on-bbc-microbit v0.1.0 (file:///…/PATH/TO/rust-on-bbc-microbit)
error[E0463]: can't find crate for `std`
```

… which doesn’t work because we don’t have a standard library for it.
The full Rust standard library (the `std` crate) depends on a bunch of operating system things
that don’t exist on the tiny microbit, like threads and files.

By adding [`#![no_std]`](https://doc.rust-lang.org/nightly/book/no-stdlib.html)
at the top of `src/lib.rs`,
we declare that this library doesn’t use the `std` crate but only the `core` crate,
the subset of `std` that (almost) doesn’t have any external dependency.
(The details behind “almost” are not essential here.)
But we don’t have `core` for our custom target either:

```sh
cargo build --target cortex-m0
```
```rust
   Compiling on-bbc-microbit v0.1.0 (file:///…/PATH/TO/rust-on-bbc-microbit)
error[E0463]: can't find crate for `core`
```

Ideally [Cargo would be able to do this](https://github.com/rust-lang/rfcs/pull/1133),
but for now we’ll need to build `core` ourselves.

Although I don’t recommend building Rust from source entirely since it takes a lot of time,
we’ll need the source for this.
We’ll also need a nightly version of the compiler.

```sh
# Somewhere
git clone https://github.com/rust-lang/rust
cd rust
```

On principle, we need the source for the same version as the compiler we’re using.
In practice, the current master branch is usually still close enough to the current Nightly version.
So you may be able to get away with not doing this step:

```sh
HASH=$(rustc --version --verbose | grep commit-hash | cut -d' ' -f2)
DATE=$(rustc --version --verbose | grep commit-date | cut -d' ' -f2)
git checkout -b nightly-$DATE $HASH
```

Then, let’s build just `core` and copy it where our Rust install can find it:

```sh
cd src/libcore
cp /…/PATH/TO/rust-on-bbc-microbit/cortex-m0.json .
cargo build --release --target=cortex-m0
SYSROOT=$(rustc --print sysroot)
mkdir -p $SYSROOT/lib/rustlib/cortex-m0/lib
cp target/cortex-m0/release/libcore.rlib $SYSROOT/lib/rustlib/cortex-m0/lib
```

**Note:** you’ll need to do this again whenever your update your compiler,
or you’ll see an error message like this:

```rust
error[E0514]: found crate `core` compiled by an incompatible version of rustc
  |
  = help: please recompile that crate using this compiler (rustc 1.12.0-nightly (1225e122f 2016-07-30))
  = note: crate `core` path #1: /…/lib/rustlib/cortex-m0/lib/libcore.rlib compiled by "rustc 1.12.0-nightly (54c0dcfd6 2016-07-28)"
```

And now we can cross-compile!

```
# Back in rust-on-bbc-microbit
cargo build --target cortex-m0
```
```rust
   Compiling on-bbc-microbit v0.1.0 (file:///…/PATH/TO/rust-on-bbc-microbit)
    Finished debug [unoptimized + debuginfo] target(s) in 0.10 secs
```


## Rust and C talk to each other

Rust has a [FFI (Foreign Function Interface)](https://doc.rust-lang.org/book/ffi.html)
that can be use to:

* Call C functions (or functions with a C-compatible ABI)
* Expose a symbols for Rust functions that have a C-compatible ABI and so can be called from C.

That way, we can have Rust code that interfaces both ways with the Lancaster University
and mbed runtime environment.

So far, building our Rust library produces a `.rlib` file in a Rust-specific format
that contains a bunch of metadata in addition to the actual machine code.
Let’s add a new section to `Cargo.toml` file:

```toml
[lib]
crate-type = ["staticlib"]
```

With this, the library is built as a `.a` file instead, compatible with C / C++ compilers.
A “static” library is one whose content is copied in the end program
(which helps make it self-sufficient),
while a “dynamic” library (`.so` on Linux)
is looked up and loaded separately when the program starts.
Our tiny micro-controller does not support dynamic libraries.

Building with this now fails:

```sh
cargo build --target cortex-m0
```
```rust
   Compiling on-bbc-microbit v0.1.0 (file:///…/rust-on-bbc-microbit)
error: language item required, but not found: `panic_fmt`

error: language item required, but not found: `eh_personality`
```

Language items are normally defined by the standard library,
but with `#![no_std]` we have to define some of them ourselves.
Let’s copy into `src/lib.rs` some boilerplate
from [the `!#[no_std]` documentation](https://doc.rust-lang.org/nightly/book/no-stdlib.html).

```rust
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
```

I don’t know what `eh_personality` is.
`panic_fmt` is called when Rust code panics,
which is when something unexpected happens with no known way of recovering.
The `!` return type indicates that `panic_fmt` must never return.
Without an operating system we don’t have a thread or process to terminate,
so we’ll just loop in `panic_fmt` until the system is reset / rebooted.

Now let’s do some [FFI (Foreign Function Interface)](https://doc.rust-lang.org/book/ffi.html)
in `src/lib.rs`.

```rust
extern {
    fn microbit_display_scroll(s: *const u8);
    fn wait_ms(s: i32);
}

#[no_mangle]
pub extern fn rust_main() {
    loop {
        unsafe {
            microbit_display_scroll("Hello Rust!\0".as_ptr());
            wait_ms(2000);
        }
    }
}
```


## Using a Rust library in an mbed project

Yotta and Cargo both fill the dual role of package manager and build system
in their respective ecosystem: C/C++ and Rust.
It all works out nicely when you stay within one ecosystem,
but now let’s make them talk to each other.

Add a minimal `module.json` in `rust-on-bbc-microbit` next to `Cargo.toml`
to make it Yotta module as well as a Rust crate:

```json
{
  "name": "rust-on-bbc-microbit",
  "version": "0.0.0",
  "dependencies": {
    "microbit": "lancaster-university/microbit"
  }
}
```

If necessary, create `yotta_targets` like earlier in `microbit-samples`:

```sh
mkdir yotta_targets
ln -s ../microbit-targets/bbc-microbit-classic-gcc yotta_targets/bbc-microbit-classic-gcc
ln -s ../target-mbed-gcc yotta_targets/mbed-gcc
```

Now building should succeed:

```sh
yotta target bbc-microbit-classic-gcc
yotta build
```

But this module doesn’t have any code yet. Let’s create `source/main.cpp`:

```cpp
#include "MicroBit.h"

MicroBit uBit;

extern "C" {
    void rust_main();
}

int main() {
    uBit.init();
    rust_main();
}
```

Now building again should fail since yotta doesn’t know about our Rust code yet,
and so can’t find `rust_main`:

```sh
yotta build
```
```
source/CMakeFiles/rust-on-bbc-microbit.dir/…/rust-on-bbc-microbit/source/main.cpp.o: In function `main':
/…/rust-on-bbc-microbit/source/main.cpp:9: undefined reference to `rust_main()'
```

Now that we have Cargo produce a `.a` static library for our Rust code,
it should be a simple matter of instructing Yotta to link it into the program.
But that’s not really the typical scenario for Yotta which usually builds everything by itself.
I spent a while looking into how to do this,
and the answer to be buried in Yotta’s documentation: [Using Custom CMake to Control The Build](
http://yottadocs.mbed.com/reference/buildsystem.html#custom-cmake)

Yotta uses CMake, and we can write custom CMake files.
CMake also has an [`ExternalProject` module](
https://cmake.org/cmake/help/v3.6/module/ExternalProject.html) that seems relevant.
After a bunch of trial and error, I came up with this `source/rust.cmake` file:

```cmake
include(ExternalProject)

ExternalProject_Add(
    rust
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build --target=cortex-m0 --release
    BINARY_DIR "${CMAKE_CURRENT_LIST_DIR}/.."
    INSTALL_COMMAND ""
    USES_TERMINAL_BUILD 1
    BUILD_ALWAYS 1
    BUILD_BYPRODUCTS "${CMAKE_CURRENT_LIST_DIR}/../target/cortex-m0/release/libon_bbc_microbit.a")

add_dependencies(rust-on-bbc-microbit rust)

target_link_libraries(
    rust-on-bbc-microbit
    "${CMAKE_CURRENT_LIST_DIR}/../target/cortex-m0/release/libon_bbc_microbit.a")
```

Now, building gives a different error:

```
/…/target/cortex-m0/release/libon_bbc_microbit.a(on_bbc_microbit.0.o): In function `rust_main':
on_bbc_microbit.cgu-0.rs:(.text.rust_main+0x6): undefined reference to `microbit_display_scroll'
```

Note that `wait_ms` is *not* an undefined reference.
This is because it is already exported by mbed as a C function (not C++)
and so Rust’s FFI can call it directly.

`uBit.display.scroll()` however is a C++ method, which Rust’s FFI does not support easily.
So we’ll have to write a wrapper C function for it in `source/main.cpp`:

```cpp
extern "C" {
    void microbit_display_scroll(char *s) {
        uBit.display.scroll(s);
    }
}
```

This time we have everything!
Building and flashing should succeed:

```sh
yotta build
cp build/bbc-microbit-classic-gcc/source/rust-on-bbc-microbit-combined.hex /run/media/$USER/MICROBIT
```


(No GIF video this time, it still looks the same from the outside.)


## Ditching the device/hardware abstraction layer

Now that we have actual Rust code running on the micro-controller hardware,
I’d like to use less and less of the mbed ecosystem.
Let’s start by doing something simpler than scrolling text on a multiplexed display:
blinking a single LED, the “Hello world” of micro-controllers.

Based on reading both [the datasheet for the Nordic nRF51822 micro-controller](
https://lancaster-university.github.io/microbit-docs/resources/datasheets/nRF51822.pdf)
and [source code for the mbed runtime](https://github.com/lancaster-university/mbed-classic),
I found out that access the hardware is memory-mapped:
access is done by reading and writing at well-known memory addresses.
Rather than do pointer arithmetic everywhere,
we can define (like the C++ mbed runtime does) a `struct` with the appropriate memory layout,
and define a single pointer to that `struct`.

```rust
const GPIO_BASE: *mut NRF_GPIO_Type = 0x50000000 as *mut _;

#[repr(C)]
struct NRF_GPIO_Type {
    RESERVED_0: [u32; 321],
    OUT: u32,                               /* Write GPIO port. */
    OUTSET: u32,                            /* Set individual bits in GPIO port. */
    OUTCLR: u32,                            /* Clear individual bits in GPIO port. */
    // ...
}
```

See the full code in [an early commit of `src/lib.rs`](
https://github.com/SimonSapin/rust-on-bbc-microbit/blob/92bc9d62bae/src/lib.rs)
in this repository.

`wait_ms` is implemented with a hardware timer which is slightly more complicated to set up,
and I didn’t want to bother with that just yet.
Instead, I wrote a very rough approximation with a loop keeping the CPU busy for many cycles.
My first attempt didn’t work: the function returned immediately
regardless of the requested number of iterations.
This was because LLVM’s dead code optimization was seeing a loop that did nothing,
and removed it entirely.

Too fool the optimizer, I copied over the `black_box` function
from Rust’s built-in benchmark harness.
It uses some inline assembly that does nothing
but force the optimizer to assume that a value is “used”,
so that the code computing that value is not optimized away.

See [`busy_loop.rs`](
https://github.com/SimonSapin/rust-on-bbc-microbit/blob/68209f9660/src/busy_loop.rs).


## Getting rid of Yotta and CMake

Yotta is open source, so I could read its code to understand what it does.
But that sounds rather tedious.
Instead, we can [spy on it with `strace`](
http://jvns.ca/blog/2014/02/26/using-strace-to-avoid-reading-ruby-code/)
to see what it’s doing in this particular configuration.

```sh
touch src/lib.rs
strace -f -qq -e signal=\!all -e execve -s9999 yotta build
```

* `-f` says to also trace sub-processes
* `-qq` and `-e signal=\!all` remove from the output some information I don’t care about
* `-e execve` says to trace `execve` system calls,
  which on Linux is how programs ask the operating system to start a new program
  (typically after `fork`ing a new process).
* `-s9999` raise the length threshold for truncating long strings in syscall parameters.

Based on this (and again some trial and error) I wrote [a `flash.sh` script](
https://github.com/SimonSapin/rust-on-bbc-microbit/blob/95879c69d7/flash.sh) to replace Yotta


## `println` debugging with the serial port

The microbit has a UART a.k.a. “serial port” available over USB.
This allows sending data both ways between the microbit and a computer.
This can be particularly useful for debugging, since we don’t have a `gdb`-style debugger.

Similar to GPIO, the UART is memory-mapped: it is configured and used by writing and reading
various registers at given memory addresses.
For example, transmitting a byte is done by simply writing it to the `TXD` register
when the hardware is ready.
To write the next byte, we loop until the reading the `EVENTS_TXDRDY` register gives 1,
which indicates that the hardware is ready.

On the computer side, the USB device for the serial port shows up on my laptop
as a device `/dev/ttyACM0` in the filesystem.
`stty` is the tool to show or change its configuration.

```sh
stty -F /dev/ttyACM0 -a
```
```
speed 9600 baud; rows 0; columns 0; line = 0;
intr = ^C; quit = ^\; erase = ^?; kill = ^U; eof = ^D; eol = <undef>; eol2 = <undef>; swtch = <undef>; start = ^Q; stop = ^S;
susp = ^Z; rprnt = ^R; werase = ^W; lnext = ^V; discard = ^O; min = 1; time = 0;
-parenb -parodd -cmspar cs8 hupcl -cstopb cread clocal -crtscts
-ignbrk -brkint -ignpar -parmrk -inpck -istrip -inlcr -igncr icrnl ixon -ixoff -iuclc -ixany -imaxbel -iutf8
opost -olcuc -ocrnl onlcr -onocr -onlret -ofill -ofdel nl0 cr0 tab0 bs0 vt0 ff0
isig icanon iexten echo echoe echok -echonl -noflsh -xcase -tostop -echoprt echoctl echoke -flusho -extproc
```

See [`man stty`](http://linux.die.net/man/1/stty) for the meaning of all these obscure keywords.
What’s important is that the default on my laptop is a speed of 9600 baud and no parity bit,
so I configured the microbit’s UART the same way.
Both sides can probably handle faster speeds, but I don’t expect to transmit a lot of data
and prefer the convenience of not having to configure `stty`.

GNU screen can be used as `screen /dev/ttyACM0` to access the serial port as a terminal.
But for simple one-way-at-a-time communication, the device can also be used like a file:

```sh
echo "Something something" > /dev/ttyACM0
cat /dev/ttyACM0
```

At first, my code would block after writing one byte.
It turned out to be the optimizer again,
moving the `EVENTS_TXDRDY` read out of the loop since it had no apparent reason to change.
The fix is to use the [`core::ptr::read_volatile`](
https://doc.rust-lang.org/core/ptr/fn.read_volatile.html) function,
which is designed for pretty much exactly this case.

See [`serial.rs`](https://github.com/SimonSapin/rust-on-bbc-microbit/blob/2452b5e94a/src/serial.rs)


## String formatting and mysterious freezes

Now that we can write bytes one at a time to the serial port,
let’s hook it into Rust’s string formatting system:

```rust
use core::fmt::Write;

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            Serial::write_byte(b)
        }
        Ok(())
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        writeln!(Serial, $($arg)*).unwrap()
    }
}
```

Seems straightforward enough.
Yet, the first time I tried to use it my program froze (the LED stopped blinking).
I had a hard time debugging this, and got a lot of help on the `#rust-internals` IRC channel
from eddyb and Amanieu.
(Many thanks to them both!)

I first modified my copy of the `core` crate
to add calls to `Serial::write_str` at various points and see what was executed.
The relevant line turned out to be [this one](
https://github.com/rust-lang/rust/blob/28ce3e8a55/src/libcore/fmt/mod.rs#L829):


```rust
    (arg.formatter)(arg.value, &mut formatter)?;
```

`arg.formatter` is a function pointer, its type is `fn(&Void, &mut Formatter) -> Result`.
If I compared the numeric address of this pointer with that of the function I was expecting,
I got the same values.
The issue did not occur if I replaced this indirect call
with an explicit call to the expected function (actually a trait method applied for a given type).

After much discussion, @eddyb got the idea of adding `"relocation-model": "static"`
to the `cortex-m0.json` target definition file.
I don’t understand what a relocation model is or what exactly was happening before,
but this fixed the issue.

… Only to uncover another, more “interesting” one.
At that point I could format integers with a single decimal digit, but not with two digits!
(Two digits caused another freeze.)
Adding more calls to `Serial::write_str` lead to [the offending line](
https://github.com/rust-lang/rust/blob/28ce3e8a55/src/libcore/fmt/num.rs#L246):

```rust
    ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
```

@Amanieu asked me to send a compiled binary, disassembled it,
and found out that LLVM “optimized” this two-bytes copy
into a pair of `load-halfword` and `store-halfword` instructions.
As it [turns out](
http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0497a/BABFAIGG.html):

> There is no support for unaligned accesses on the Cortex-M0 processor.
> Any attempt to perform an unaligned memory access operation results in a HardFault exception.

And these pointers are not guaranteed to be aligned.
(Which, for 2-bytes “half words”, means that their address is a multiple of two.)
The fix here was to add `"features": "+strict-align"` to `cortex-m0.json`.


## Panic messages over serial

Now that string formatting and writing to the serial port both seem to work well,
we can use them in our panic handler:

```rust
#[lang = "panic_fmt"]
extern fn panic_fmt(message: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("Panic at {}:{}, {}", file, line, message);
    loop {}
}
```

Now if we cause a panic in `main()` with something like `assert_eq!(3, 42)`,
we get a nice error message on the computed through the serial port and USB cable.
Keep a terminal open on the computer with the `cat /dev/ttyACM0` command running to see it.


## Coming up

Next, I’ll look into

* Driving [a multiplexed 4-digit 7-segment LED display](
  https://shop.pimoroni.com/products/4-digit-7-segment-display)
* Accessing [a RTC (Real-Time Clock)](
  https://shop.pimoroni.com/products/sparkfun-deadon-rtc-ds3234-breakout)
  over SPI (Serial Peripheral Interface).
* [Calendaring while aware of summer time (daylight-saving time)](
  https://github.com/SimonSapin/gregor)
* Figuring out the wiring and soldering to put it together on something more permanent
  and more compact than a prototyping breadboard.

<img src=pictures/prototype.jpg height=600>
