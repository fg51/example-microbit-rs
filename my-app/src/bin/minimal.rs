#![no_main]
#![no_std]
#![deny(unsafe_code)]

use my_app as _; // global logger + panicking-behavior + memory layout

// RTIC requires that unused interrupts are declared in an extern block when
// using software tasks; these free interrupts will be used to dispatch the
// software tasks.
// TODO: Replace `some_hal::pac` with the path to the PAC
//#[rtic::app(device = some_hal::pac)]
#[rtic::app(device = microbit::pac, peripherals = true, dispatchers = [UARTE1])]
mod app {
    use embedded_hal::serial::Write;

    use microbit::board::Board;
    use microbit::hal::uarte::{self, Baudrate, Parity};
    //    hal::uarte,
    //    hal::uarte::{Baudrate, Parity},
    use super::serial_setup::UartePort;

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
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");

        let board = Board::new(cx.device, cx.core);

        let mut serial = {
            UartePort::new(uarte::Uarte::new(
                board.UARTE0,
                board.uart.into(),
                Parity::EXCLUDED,
                Baudrate::BAUD115200,
            ))
        };
        nb::block!(serial.write(b'X')).unwrap();
        nb::block!(serial.flush()).unwrap();

        // debug::exit(debug::EXIT_SUCCESS);

        task1::spawn().ok();

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
    #[task]
    fn task1(_cx: task1::Context) {
        defmt::info!("Hello from task1!");
    }
}

mod serial_setup {
    use core::fmt;
    use embedded_hal::blocking::serial as bserial;
    use embedded_hal::serial;
    use microbit::hal::uarte::{Error, Instance, Uarte, UarteRx, UarteTx};

    static mut TX_BUF: [u8; 1] = [0; 1];
    static mut RX_BUF: [u8; 1] = [0; 1];

    pub struct UartePort<T: Instance>(UarteTx<T>, UarteRx<T>);

    impl<T: Instance> UartePort<T> {
#![allow(unsafe_code)]
        pub fn new(serial: Uarte<T>) -> UartePort<T> {
            let (tx, rx) = serial
                .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
                .unwrap();
            UartePort(tx, rx)
        }
    }

    impl<T: Instance> fmt::Write for UartePort<T> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.write_str(s)
        }
    }

    impl<T: Instance> serial::Write<u8> for UartePort<T> {
        type Error = Error;

        fn write(&mut self, b: u8) -> nb::Result<(), Self::Error> {
            self.0.write(b)
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            self.0.flush()
        }
    }

    impl<T: Instance> bserial::write::Default<u8> for UartePort<T> {}

    impl<T: Instance> serial::Read<u8> for UartePort<T> {
        type Error = Error;

        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            self.1.read()
        }
    }
}
