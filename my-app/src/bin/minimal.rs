#![no_main]
#![no_std]

use my_app as _; // global logger + panicking-behavior + memory layout

// TODO: Replace `some_hal::pac` with the path to the PAC
//#[rtic::app(device = some_hal::pac)]
#[rtic::app(device = microbit::pac, peripherals = true)]
mod app {
    //use cortex_m_semihosting::debug;

    // TODO: Add a monotonic if scheduling will be used
    // #[monotonic(binds = SysTick, default = true)]
    // type DwtMono = DwtSystick<80_000_000>;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");

        // debug::exit(debug::EXIT_SUCCESS);

        // task1::spawn().ok();

        // Setup the monotonic timer
        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
            init::Monotonics(
                // Initialization of optional monotonic timers go here
            ),
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // TODO: Add tasks
    //#[task]
    //fn task1(_cx: task1::Context) {
    //    defmt::info!("Hello from task1!");
    //}
}
