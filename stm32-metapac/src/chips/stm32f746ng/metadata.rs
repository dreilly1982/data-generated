include!("../metadata_0239.rs");
pub const METADATA: Metadata = Metadata {
    name: "STM32F746NG",
    family: "STM32F7",
    line: "STM32F7x6",
    memory: &[
        MemoryRegion {
            name: "BANK_1_REGION_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 32768,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_2",
            kind: MemoryRegionKind::Flash,
            address: 134348800,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_3",
            kind: MemoryRegionKind::Flash,
            address: 134479872,
            size: 786432,
            settings: Some(FlashSettings {
                erase_size: 262144,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 535883776,
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
            size: 327680,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 537182208,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
