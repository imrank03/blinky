# **Blinky**
[Rust](https://www.rust-lang.org) is showing great promise in the embedded world, and offers many benefits (like memory safety!). In this project, we will walk through developing a basic application for an STM32 microcontroller. I developed this using a [NUCLEO-F091RC](https://www.st.com/en/evaluation-tools/nucleo-f091rc.html) development board, but it should be easily adaptable to most STM32 devices.

`Blinky - Just a LED blink and nothing more!`

## Toolchin-Setup 
We need to make sure that we have an up-to-date version of Rust installed. If you do not have Rust installed, please refer to the [Rust documentation](https://www.rust-lang.org/tools/install) for installation instructions. To avoid problems it's recommended to use the most recent version of Rust available to you.

    ~ rustup default nightly
    ~ rustup update
    ~ rustc --version
    rustc 1.60.0-nightly (777bb86bc 2022-01-20)

With Rust installed and up to date, we need to add support for our required compilation target, which differs depending on the type of microcontroller in use. Refer to the below table to determine the suitable target for your application :

|Target|Series|FPU|
|----|-----|-------|
|thumbv6m-none-eabi|Cortex-M0, Cortex-M0+|No|
|thumbv7m-none-eabi|Cortex-M3|No|
|thumbv7em-none-eabi|Cortex-M4, Cortex-M7|No|
|thumbv7em-none-eabihf|Cortex-M4F, Cortex-M7F|Yes|

Since I am using a Cortex-M0 device in this case, I chose the thumbv6m-none-eabi instruction set for the compilation target:

    ~ rustup target add thumbv6m-none-eabi

## Project-Setup
Generating the project using Cargo :

    ~ cargo new blinky

We will next add a configuration file to the project to instruct Cargo to compile for the appropriate target by default. In the root of the project, create the directory .cargo Create and open config.toml file and add the following :

    [target.'cfg(all(target_arch = "arm", target_os = "none"))']
    runner = "probe-run --chip stm32f091rctx"

    rustflags = [
    "-C", "linker=flip-link",
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=-Tdefmt.x",
    # This is needed if your flash or ram addresses are not aligned to 0x10000 in memory.x
    "-C", "link-arg=--nmagic",
    ]

    [build]
    # (`thumbv6m-*` is compatible with all ARM Cortex-M chips but using the right target improves performance)
    target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
    # target = "thumbv7m-none-eabi"    # Cortex-M3
    # target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
    # target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)

    [alias]
    rb = "run --bin"
    rrb = "run --release --bin"

Since I am using a Cortex-M0 device in this case, so I have enabled `target = "thumbv6m-none-eabi"`

Since different devices have varying amounts of Flash and RAM, we need to define a linker file, its values will need to be updated according to the values in the datasheet of your specific device (in the Memory Mapping section). `Note` that using this linker file on a device with more available Flash and/or RAM will render that memory unusable by the application. Create the file memory.x in the root of your project, and populate it with the following, updating the ORIGIN and LENGTH fields if required :

    MEMORY
    {
    /* NOTE 1 K = 1 KiBi = 1024 bytes */
    /* TODO Adjust these memory regions to match your device memory layout */
    /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
    FLASH    (rx)  : ORIGIN = 0x08000000, LENGTH = 64K
    RAM      (rwx) : ORIGIN = 0x20000000, LENGTH = 28K 
    }

    /* This is where the call stack will be allocated. */
    /* The stack is of the full descending type. */
    /* You may want to use this variable to locate the call stack and static
    variables in different memory regions. Below is shown the default value */
    /* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

We'll update Cargo.toml, open it in a text editor and add the prerequisite crates under `[dependencies]`. If you are not using an STM32F0 series device, change the stm32f0xx-hal crate to the appropriate alternative.

    [dependencies]
    cortex-m = "0.7.4"
    cortex-m-rt = "0.7.1"
    defmt = "0.3.0"
    defmt-rtt = {version = "0.3.1"}
    panic-probe = { version = "0.3.0", features = ["print-defmt"] }
    panic-halt = "0.2.0"
    stm32f0xx-hal =  { version = "0.18.0", default-features = false, features = ["rt", "stm32f091"]}

    [profile.release]
    debug = true
    lto = true
    opt-level = "s"

## Finally writing the main source code!
    Source code! for this project will be contained within src/main.rs.

## Compilation & Flashing

At this point, you should be able to build the project. To do so, we simply run `cargo build --release` in the root project directory. Once the build has completed, you should see the newly created target/ directory in the project root.

probe-run this tool will be used for flashing and debugging the embedded device.

To install probe-run, use this command :

    ~ cargo install probe-run

To flash the binary use this command :

    ~ probe-run blinky --chip stm32f091rctx

    Note - If you are not using this chip stm32f091rctx change the chip.

It is just a example, here's the command line output that should be produced.

    imrankhaleelsab@Imrans-MacBook-Pro blinky % cargo build --release                
    Compiling proc-macro2 v1.0.33
    Compiling unicode-xid v0.2.2
    Compiling syn v1.0.82
    Compiling version_check v0.9.4
    Compiling semver-parser v0.7.0
    Compiling cortex-m v0.7.4
    Compiling nb v1.0.0
    Compiling semver v1.0.4
    Compiling vcell v0.1.3
    Compiling void v1.0.2
    Compiling defmt v0.3.0
    Compiling bitfield v0.13.2
    Compiling defmt-macros v0.3.1
    Compiling bitflags v1.3.2
    Compiling cortex-m-rt v0.7.1
    Compiling defmt-parser v0.3.0
    Compiling stm32f0 v0.14.0
    Compiling bare-metal v1.0.0
    Compiling panic-probe v0.3.0
    Compiling defmt-rtt v0.3.1
    Compiling panic-halt v0.2.0
    Compiling volatile-register v0.2.1
    Compiling nb v0.1.3
    Compiling bxcan v0.6.2
    Compiling semver v0.9.0
    Compiling embedded-hal v0.2.6
    Compiling proc-macro-error-attr v1.0.4
    Compiling proc-macro-error v1.0.4
    Compiling rustc_version v0.2.3
    Compiling bare-metal v0.2.5
    Compiling rustc_version v0.4.0
    Compiling cast v0.2.7
    Compiling quote v1.0.10
    Compiling cortex-m-rt-macros v0.7.0
    Compiling stm32f0xx-hal v0.18.0
    Compiling blinky v0.1.0 (/Users/Imran/Boschspace/blinky)
        Finished release [optimized + debuginfo] target(s) in 26.19s
    imrankhaleelsab@Imrans-MacBook-Pro blinky % cd target/thumbv6m-none-eabi/release 
    imrankhaleelsab@Imrans-MacBook-Pro release % probe-run blinky --chip stm32f091rctx
    (HOST) INFO  flashing program (2 pages / 2.00 KiB)
    (HOST) INFO  success!
    ────────────────────────────────────────────────────────────────────────────────
    ^C────────────────────────────────────────────────────────────────────────────────
    stack backtrace:
    0: core::num::<impl u32>::wrapping_sub
            at /rustc/777bb86bcdbc568be7cff6eeeaaf81a89b4aa50b/library/core/src/num/uint_macros.rs:1225:13
    1: <stm32f0xx_hal::delay::Delay as embedded_hal::blocking::delay::DelayUs<u32>>::delay_us
            at /Users/imrankhaleelsab/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f0xx-hal-0.18.0/src/delay.rs:110:20
    2: <stm32f0xx_hal::delay::Delay as embedded_hal::blocking::delay::DelayMs<u16>>::delay_ms
            at /Users/imrankhaleelsab/.cargo/registry/src/github.com-1ecc6299db9ec823/stm32f0xx-hal-0.18.0/src/delay.rs:79:9
    3: blinky::__cortex_m_rt_main
            at /Users/Imran/Boschspace/blinky/src/main.rs:31:13
    4: main
            at /Users/Imran/Boschspace/blinky/src/main.rs:16:1
    5: Reset
    (HOST) INFO  device halted by user

## Blinky(s):

**blinks green LED for every one miliseconds:**

[![blinky-green](https://user-images.githubusercontent.com/92363511/151695520-aae3dcbd-cf8b-4f7f-81ac-b2a07f0ab9ba.png)](https://user-images.githubusercontent.com/92363511/151695606-9ecc9f5c-cf65-4502-9e7b-2b5cdb468cc8.mp4)

## Support:

For questions, issues, feature requests, and other changes, please file an issue in the github project.

## License:

blinky is licensed under 
 
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)



