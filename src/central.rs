#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod keymap;
mod vial;

use defmt::info;
use dummy_pin::DummyPin;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::USB_OTG_FS;
use embassy_stm32::spi::{self, Spi};
use embassy_stm32::time::Hertz;
use embassy_stm32::usb::{Driver, InterruptHandler};
use embassy_stm32::{Config, bind_interrupts};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use keymap::{COL, ROW};
use rmk::channel::EVENT_CHANNEL;
use rmk::config::{BehaviorConfig, ControllerConfig, RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join3;
use rmk::input_device::Runnable;
use rmk::keyboard::Keyboard;
use rmk::light::LightController;
use rmk::matrix::Matrix;
use rmk::{initialize_keymap_and_storage, run_devices, run_rmk};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};
use w25::W25;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    OTG_FS => InterruptHandler<USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // info!("RMK start!");

    // loop {
    //     info!("YAY!");
    //     Timer::after_secs(1).await;
    // }

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
            // divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 168 / 4 = 84Mhz. (=MAX SYSCLK FREQENCY FOR f401)
            // divq: Some(PllQDiv::DIV2), // 8mhz / 4 * 168 / 7 = 48Mhz. (=Needed for clk48)
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
    //let (input_pins, output_pins) = config_matrix_pins_stm32!(peripherals: p, input: [PB9, PB15, PB14, PB13, PB6, PB7, PB8, PB12], output: [PB0, PA1, PB3, PB4, PB5]);
    //Change B3 and B4 to A2 and A3 because they seem to clash with jtag?
    let (input_pins, output_pins) = config_matrix_pins_stm32!(peripherals: p, input: [PB9, PB15, PB14, PB13, PB6, PB7, PB8, PB12], output: [PB0, PA1, PA2, PA3, PB5]);

    //A4: Select
    //A5: SCK
    //A6: MISO
    //A7: MOSI

    // Use internal flash to emulate eeprom
    // The SOIC8 port at the bottom of the blackpill does not provide pins for the hold/wp pins, and hardwires them to 3.3V
    // and the W25 driver doesn't do anything with those pins.
    //Capacity = 64MBit = 8MByte

    static SPI_BUS: StaticCell<Mutex<NoopRawMutex, Spi<'static, Async>>> = StaticCell::new();

    //let spi = Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, spi::Config::default());
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
    let mut default_keymap = keymap::get_default_keymap();
    let behavior_config = BehaviorConfig::default();
    let storage_config = StorageConfig::default();

    // loop {
    //     info!("YAY!");
    //     Timer::after_secs(1).await;
    // }

    info!("Initializing storage and keymap");
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut default_keymap,
        flash_chip,
        &storage_config,
        behavior_config,
    )
    .await;
    info!("Initialized storage and keymap");

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::<COL, ROW>::new();
    let mut matrix = Matrix::<_, _, _, COL, ROW>::new(input_pins, output_pins, debouncer);
    let mut keyboard = Keyboard::new(&keymap);

    info!("Created Keyboard");

    // Initialize the light controller
    let mut light_controller: LightController<Output> =
        LightController::new(ControllerConfig::default().light_config);

    info!("Starting!");
    // Start
    join3(
        run_devices! (
            (matrix) => EVENT_CHANNEL,
        ),
        keyboard.run(),
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
