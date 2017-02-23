// Automatically generated by tools/nRF51_codegen.py

use kernel::common::VolatileCell;

pub const RTC1_BASE: usize = 0x40011000;
pub struct RTC1 {
    pub tasks_start: VolatileCell<u32>,
    pub tasks_stop: VolatileCell<u32>,
    pub tasks_clear: VolatileCell<u32>,
    pub tasks_trigovrflw: VolatileCell<u32>,
    _reserved1: [u32; 60],
    pub events_tick: VolatileCell<u32>,
    pub events_ovrflw: VolatileCell<u32>,
    _reserved2: [u32; 14],
    pub events_compare: [VolatileCell<u32>; 4],
    _reserved3: [u32; 109],
    pub intenset: VolatileCell<u32>,
    pub intenclr: VolatileCell<u32>,
    _reserved4: [u32; 13],
    pub evten: VolatileCell<u32>,
    pub evtenset: VolatileCell<u32>,
    pub evtenclr: VolatileCell<u32>,
    _reserved5: [u32; 110],
    pub counter: VolatileCell<u32>,
    pub prescaler: VolatileCell<u32>,
    _reserved6: [u32; 13],
    pub cc: [VolatileCell<u32>; 4],
    _reserved7: [u32; 683],
    pub power: VolatileCell<u32>,
}

pub const GPIO_BASE: usize = 0x50000000;
pub struct GPIO {
    _reserved1: [u32; 321],
    pub out: VolatileCell<u32>,
    pub outset: VolatileCell<u32>,
    pub outclr: VolatileCell<u32>,
    pub in_: VolatileCell<u32>,
    pub dir: VolatileCell<u32>,
    pub dirset: VolatileCell<u32>,
    pub dirclr: VolatileCell<u32>,
    _reserved2: [u32; 120],
    pub pin_cnf: [VolatileCell<u32>; 32],
}

pub const TEMP_BASE: usize = 0x4000C000;
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct TEMP_REGS {
    pub START: VolatileCell<u32>, // 0x000 - 0x004
    pub STOP: VolatileCell<u32>, // 0x004 - 0x008
    pub _reserved1: [u32; 62], // 0x008 - 0x100
    pub DATARDY: VolatileCell<u32>, // 0x100 - 0x104
    pub _reserved2: [u32; 127], // 0x104 - 0x300
    pub INTEN: VolatileCell<u32>, // 0x300 - 0x304
    pub INTENSET: VolatileCell<u32>, // 0x304 - 0x308
    pub INTENCLR: VolatileCell<u32>, // 0x308 - 0x30c
    pub _reserved3: [u32; 127], // 0x30c - 0x508
    pub TEMP: VolatileCell<u32>, // 0x508 - 0x50c

pub const RNG_BASE: usize = 0x4000D000;
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct RNG_REGS {
    pub START: VolatileCell<u32>,                       // 0x000 - 0x004
    pub STOP: VolatileCell<u32>,                        // 0x004 - 0x008
    pub _reserved1: [u32; 62],                          // 0x008 - 0x100
    pub VALRDY: VolatileCell<u32>,                      // 0x100 - 0x104
    pub _reserved2: [u32; 63],                          // 0x104 - 0x200
    pub SHORTS: VolatileCell<u32>,                      // 0x200 - 0x204
    pub _reserved3: [u32; 63],                          // 0x204 - 0x300
    pub INTEN: VolatileCell<u32>,                       // 0x300 - 0x304
    pub INTENSET: VolatileCell<u32>,                    // 0x304 - 0x308
    pub INTENCLR: VolatileCell<u32>,                    // 0x308 - 0x30c
    pub _reserved4: [u32; 126],                         // 0x30c - 0x504
    pub CONFIG: VolatileCell<u32>,                      // 0x504 - 0x508
    pub VALUE: VolatileCell<u32>,                       // 0x508 - 0x50c
}
