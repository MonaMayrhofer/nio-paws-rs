#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod keymap;
mod vial;

use crate::keymap::{RIGHT_COL, RIGHT_ROW};
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::peripherals::{self};
use embassy_stm32::time::Hertz;
use embassy_stm32::usart::{self, BufferedInterruptHandler, BufferedUart};
use rmk::channel::EVENT_CHANNEL;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join;
use rmk::matrix::Matrix;
use rmk::run_devices;
use rmk::split::peripheral::run_rmk_split_peripheral;
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // RCC config
    let config = {
        use embassy_stm32::rcc::*;
        let mut config = embassy_stm32::Config::default();
        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25,
            mul: PllMul::MUL336,
            divp: Some(PllPDiv::DIV4), // 8mhz / 4 * 168 / 4 = 84Mhz. (=MAX SYSCLK FREQENCY FOR f401)
            divq: Some(PllQDiv::DIV7), // 8mhz / 4 * 168 / 7 = 48Mhz. (=Needed for clk48)
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
        config
    };

    // Initialize peripherals
    info!("Embassy Init Pre");
    let p = embassy_stm32::init(config);
    info!("Embassy Init");

    // Pin config
    // COL 2 ROW
    let (input_pins, output_pins) = config_matrix_pins_stm32!(peripherals: p,
        // input: [PB0, PA1, PB3, PB4, PB5],
        // output: [PB9, PB15, PB14, PB13, PB6, PB7, PB8, PB12]

        input: [PB0, PA1, PB3, PB4, PB5],
        output: [PB9, PB15, PB14, PB13, PB6, PB7, PB8, PB12]
    );

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::<RIGHT_ROW, RIGHT_COL>::new();
    let mut matrix =
        Matrix::<_, _, _, RIGHT_ROW, RIGHT_COL>::new(input_pins, output_pins, debouncer);

    static UART_OUT_BUFFER: StaticCell<[u8; 64]> = StaticCell::new();
    static UART_IN_BUFFER: StaticCell<[u8; 64]> = StaticCell::new();
    let uart = BufferedUart::new(
        p.USART2,
        Irqs,
        p.PA3,
        p.PA2,
        &mut UART_OUT_BUFFER.init([0; 64])[..],
        &mut UART_IN_BUFFER.init([0; 64])[..],
        usart::Config::default(),
    )
    .unwrap();

    info!("Starting!");
    // Start
    join(
        run_devices! (
            (matrix) => EVENT_CHANNEL,
        ),
        run_rmk_split_peripheral(uart),
    )
    .await;
}
