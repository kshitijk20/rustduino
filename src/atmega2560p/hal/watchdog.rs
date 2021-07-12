//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Devansh Kumar Jha,Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>

//! Control on Watchdog timer in ATMEGA2560P.
//! Section 12.5 of manual.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

use crate::atmega2560p::hal::interrupt;
/// Crates required in the code for reading and writing to registers.
/// Interrupts would be used for disabling global interrupts which may create problem while execution.
use core;

/// Contains various registers to control the functioning of registers Watchdog.
/// MCUSR : Contains 5 writable bits which are used for various watchdog settings.
/// WDTCSR : Contains 8 writable bits which are used for various watchdog settings.
#[repr(C, packed)]
pub struct Watchdog {
    mcusr: u8,
    pad_1: [char; 11], // padding for empty memory space
    wdtcsr: u8,
}

impl Watchdog {
    /// Returns a static mutable reference to the structure Watchdog.
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x54 as *mut Watchdog) // memory address to check
    }

    /// If the WDIE bit is enabled it will be disabled otherwise enabled.
    pub fn interrupt_toggle(&mut self) {
        unsafe {
            let mut wdtcsr = core::ptr::read_volatile(&mut self.wdtcsr);
            if wdtcsr & 0xBF == wdtcsr {
                wdtcsr = wdtcsr | 0x40;
            } else {
                wdtcsr = wdtcsr & 0xBF;
            }
            core::ptr::write_volatile(&mut self.wdtcsr, wdtcsr);
        }
    }

    /// For disabling watchdog in ATMEGA2560P it is first essential to disable
    /// global standard interrupts inbuild in the chip. Then we need to write
    /// the WDE bit of wdtcsr as 0 but for that first WDRF bit of mcusr is to be changed to 0
    /// and WDCE bit of wdtcsr to 1.
    pub fn disable(&mut self) {
        unsafe {
            let itr = interrupt::Status::new(); // Object created for interrupt handling
            self.interrupt_toggle(); // Disable watchdog interrupts
            itr.disable(); // Disable global interrupts

            let mut wdtcsr = core::ptr::read_volatile(&mut self.wdtcsr);
            let mut mcusr = core::ptr::read_volatile(&mut self.mcusr);
            // First set WDCE bit of wdtcsr register as 1
            wdtcsr = wdtcsr | 0x10;
            core::ptr::write_volatile(&mut self.wdtcsr, wdtcsr);
            // Then change the WDRF bit of mcusr register as 0
            mcusr = mcusr & 0xF7;
            core::ptr::write_volatile(&mut self.mcusr, mcusr);
            // Then change the WDE bit of wdtcsr register to 0
            wdtcsr = wdtcsr & 0xF7;
            core::ptr::write_volatile(&mut self.wdtcsr, wdtcsr);

            self.interrupt_toggle(); // Enable watchdog interrupts
            itr.enable(); // Enable global interrupts
        }
    }
}
