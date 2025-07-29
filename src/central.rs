#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod keymap;
mod vial;

use crate::keymap::{
    LEFT_COL, LEFT_COL_OFFSET, LEFT_ROW, LEFT_ROW_OFFSET, RIGHT_COL_OFFSET, RIGHT_ROW_OFFSET,
    TOTAL_COL, TOTAL_ROW,
};
use defmt::info;
use dummy_pin::DummyPin;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::USB_OTG_FS;
use embassy_stm32::spi::{self, Spi};
use embassy_stm32::time::Hertz;
use embassy_stm32::usart::{BufferedInterruptHandler, BufferedUart};
use embassy_stm32::usb::{Driver, InterruptHandler};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use rmk::channel::EVENT_CHANNEL;
use rmk::config::{BehaviorConfig, ControllerConfig, RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join4;
use rmk::input_device::Runnable;
use rmk::keyboard::Keyboard;
use rmk::light::LightController;
use rmk::split::central::{CentralMatrix, run_peripheral_manager};
use rmk::{initialize_keymap_and_storage, run_devices, run_rmk};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};
use w25::W25;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    OTG_FS => InterruptHandler<USB_OTG_FS>;
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

    // Usb config
    static EP_OUT_BUFFER: StaticCell<[u8; 1024]> = StaticCell::new();
    let mut usb_config = embassy_stm32::usb::Config::default();

    // Do not enable vbus_detection. This is a safe default that works in all boards.
    // However, if your USB device is self-powered (can stay powered on if USB is unplugged), you need
    // to enable vbus_detection to comply with the USB spec. If you enable it, the board
    // has to support it or USB won't work at all. See docs on `vbus_detection` for details.
    usb_config.vbus_detection = false;
    let driver = Driver::new_fs(
        p.USB_OTG_FS,
        Irqs,
        p.PA12,
        p.PA11,
        &mut EP_OUT_BUFFER.init([0; 1024])[..],
        usb_config,
    );

    // Pin config
    // COL 2 ROW
    let (input_pins, output_pins) = config_matrix_pins_stm32!(peripherals: p,
        input: [PB0, PA1, PB3, PB4, PB5],
        output: [PB9, PB15, PB14, PB13, PB6, PB7, PB8, PB12]
    );

    //A4: Select
    //A5: SCK
    //A6: MISO
    //A7: MOSI
    static SPI_BUS: StaticCell<Mutex<NoopRawMutex, Spi<'static, Async>>> = StaticCell::new();
    let spi = Spi::new(
        p.SPI1,
        p.PA5,
        p.PA7,
        p.PA6,
        p.DMA2_CH3,
        p.DMA2_CH2,
        spi::Config::default(),
    );
    let spi_bus = Mutex::new(spi);
    let spi_bus = SPI_BUS.init(spi_bus);
    let cs_pin = Output::new(p.PA4, Level::Low, Speed::Medium);
    let flash_spi = SpiDevice::new(spi_bus, cs_pin);

    let hold = DummyPin::new_high();
    let wp = DummyPin::new_high();
    let flash_chip = W25::<w25::Q, _, _, _>::new(flash_spi, hold, wp, 8 * 1024 * 1024).unwrap();
    //let flash = async_flash_wrapper(flashChip);

    // Keyboard config
    let rmk_config = RmkConfig {
        vial_config: VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF),
        ..Default::default()
    };

    // Initialize the storage and keymap
    info!("Initializing storage and keymap");
    let mut default_keymap = keymap::get_default_keymap();
    let behavior_config = BehaviorConfig::default();
    let storage_config = StorageConfig::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut default_keymap,
        flash_chip,
        &storage_config,
        behavior_config,
    )
    .await;
    info!("Initialized storage and keymap");

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::<LEFT_ROW, LEFT_COL>::new();
    let mut matrix =
        CentralMatrix::<_, _, _, LEFT_ROW_OFFSET, LEFT_COL_OFFSET, LEFT_ROW, LEFT_COL>::new(
            input_pins,
            output_pins,
            debouncer,
        );
    let mut keyboard = Keyboard::<TOTAL_ROW, TOTAL_COL, _, _>::new(&keymap);

    info!("Created Keyboard");

    // Initialize the light controller
    let mut light_controller: LightController<Output> =
        LightController::new(ControllerConfig::default().light_config);

    // Initilize UART
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
    join4(
        run_devices! (
            (matrix) => EVENT_CHANNEL,
        ),
        keyboard.run(),
        run_peripheral_manager::<LEFT_ROW, LEFT_COL, RIGHT_ROW_OFFSET, RIGHT_COL_OFFSET, _>(
            0, uart,
        ),
        run_rmk(
            &keymap,
            driver,
            &mut storage,
            &mut light_controller,
            rmk_config,
        ),
    )
    .await;
}
