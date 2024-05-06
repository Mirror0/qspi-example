#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::time::Hertz;

use crate::memory::FlashMemory;

mod qspi;
mod memory;

#[macro_export]
macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        let (x,) = STATIC_CELL.init(($val,));
        x
    }};
}

const MEMORY_ADDR: u32 = 0x00000000u32;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL216,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 216 / 2 = 216Mhz
            divq: Some(PllQDiv::DIV9), // 8mhz / 4 * 216 / 9 = 48Mhz
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }
    let p = embassy_stm32::init(config);

    info!("Embassy initialized");
    let qspi = qspi::create_qspi(
        p.QUADSPI, p.PF8, p.PF9, p.PE2, p.PF6, p.PF10, p.PB10, p.DMA2_CH7,
    );
    let mut flash = FlashMemory::new(qspi);
    let flash_id = flash.read_id();
    info!("FLASH ID: {:?}", flash_id);
    let mut wr_buf = [0u8; 256];
    for i in 0..256 {
        wr_buf[i] = i as u8;
    }
    let mut rd_buf = [0u8; 256];
    flash.erase_sector(MEMORY_ADDR);
    flash.write_memory(MEMORY_ADDR, &wr_buf, true);
    flash.read_memory(MEMORY_ADDR, &mut rd_buf, true);
    info!("WRITE BUF: {:?}", wr_buf);
    info!("READ BUF: {:?}", rd_buf);
    info!("End of Program, proceed to empty endless loop");
    loop {}
}
