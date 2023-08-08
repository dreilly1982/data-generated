include!("../metadata_0229.rs");
pub const METADATA: Metadata = Metadata {
    name: "STM32F730I8",
    family: "STM32F7",
    line: "STM32F7x0 Value line",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 16384,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 535853056,
            size: 512,
            settings: Some(FlashSettings {
                erase_size: 512,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 536936448,
            size: 196608,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 537116672,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
