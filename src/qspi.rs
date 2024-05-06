#![allow(unused_imports)]
use embassy_stm32::peripherals::{self, DMA2_CH7, PB10, PE2, PF10, PF6, PF8, PF9, QUADSPI};
use embassy_stm32::qspi::enums::{AddressSize, ChipSelectHighTime, FIFOThresholdLevel, MemorySize};
use embassy_stm32::qspi::{Config, Qspi};
use embassy_stm32::{bind_interrupts, qspi};
use static_cell::StaticCell;

use crate::singleton;

pub fn create_qspi(
    qspi: QUADSPI,
    pf8: PF8,
    pf9: PF9,
    pe2: PE2,
    pf6: PF6,
    pf10: PF10,
    pb10: PB10,
    dma: DMA2_CH7,
) -> &'static mut Qspi<'static, QUADSPI, DMA2_CH7> {
    let config: Config = Config {
        memory_size: MemorySize::_8MiB,
        address_size: AddressSize::_24bit,
        prescaler: 16,
        cs_high_time: ChipSelectHighTime::_1Cycle,
        fifo_threshold: FIFOThresholdLevel::_16Bytes,
    };
    singleton!(Qspi::new_bk1(qspi, pf8, pf9, pe2, pf6, pf10, pb10, dma, config))
}
