//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021 Aniket Sharma, Indian Institute of Technology Kanpur
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

//! This is the implementation for Analog Referencing in Integrated circuit of AVR Chips.  
//! This code is written taking into account the features available in ATMEGA2560P.
//! This code implements the Analog Read function to read from the buffer using analog signals.
//! This code implements the Analog Write function to write into the buffer using analog signals.
//! Refer to section 25 and 26 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

use crate::atmega2560p::hal::port::*;
use crate::atmega2560p::hal::power::Power;
/// Crates to be used for the implementation.
use bit_field::BitField;
use core::ptr::write_volatile;
use volatile::Volatile;

/// Selection of reference type for the implementation of Analog Pins.
#[derive(Clone, Copy)]
pub enum RefType {
    DEFAULT,
    INTERNAL1V1,
    INTERNAL2V56,
    EXTERNAL,
}

/// Selection of timer mode for Timer 8 type.
#[derive(Clone, Copy)]
pub enum TimerNo8 {
    Timer0,
    Timer2,
}

/// Selection of timer mode for Timer 16 type.
#[derive(Clone, Copy)]
pub enum TimerNo16 {
    Timer1,
    Timer3,
    Timer4,
    Timer5,
}

/// Structure to control the implementation of Integrated Analog Circuit.
#[repr(C, packed)]
pub struct AnalogComparator {
    acsr: Volatile<u8>,
}

/// Structure to control data transfer from Analog to Digital signal conversions.
#[repr(C, packed)]
pub struct Analog {
    adcl: Volatile<u8>,
    adch: Volatile<u8>,
    adcsra: Volatile<u8>,
    adcsrb: Volatile<u8>,
    admux: Volatile<u8>,
    didr2: Volatile<u8>,
    didr0: Volatile<u8>,
    didr1: Volatile<u8>,
}

/// Structure to control the timer of type 8 for Analog Write.
pub struct Timer8 {
    tccra: Volatile<u8>,
    tccrb: Volatile<u8>,
    tcnt: Volatile<u8>,
    ocra: Volatile<u8>,
    ocrb: Volatile<u8>,
}

/// Structure to control the timer of type 16 for Analog Write.
pub struct Timer16 {
    tccra: Volatile<u8>,
    tccrb: Volatile<u8>,
    tccrc: Volatile<u8>,
    pad0: Volatile<u8>,
    tcntl: Volatile<u8>,
    tcnth: Volatile<u8>,
    icrl: Volatile<u8>,
    icrh: Volatile<u8>,
    ocral: Volatile<u8>,
    ocrah: Volatile<u8>,
    ocrbl: Volatile<u8>,
    ocrbh: Volatile<u8>,
    ocrcl: Volatile<u8>,
    ocrch: Volatile<u8>,
}

impl Timer8 {
    ///
    pub fn new(timer: TimerNo8) -> &'static mut Timer8 {
        match timer {
            TimerNo8::Timer0 => unsafe { &mut *(0x44 as *mut Timer8) },
            TimerNo8::Timer2 => unsafe { &mut *(0xB0 as *mut Timer8) },
        }
    }
}

impl Timer16 {
    ///
    pub fn new(timer: TimerNo16) -> &'static mut Timer16 {
        match timer {
            TimerNo16::Timer1 => unsafe { &mut *(0x80 as *mut Timer16) },
            TimerNo16::Timer3 => unsafe { &mut *(0x90 as *mut Timer16) },
            TimerNo16::Timer4 => unsafe { &mut *(0xA0 as *mut Timer16) },
            TimerNo16::Timer5 => unsafe { &mut *(0x120 as *mut Timer16) },
        }
    }
}

impl AnalogComparator {
    /// New pointer object created for Analog Comparator Structure.
    pub unsafe fn new() -> &'static mut AnalogComparator {
        &mut *(0x50 as *mut AnalogComparator)
    }
}

impl Pin {
    
    pub fn analog_read(&mut self, pin: u32, reftype: RefType) -> u32 {
        unsafe {
            let analog = Analog::new();

            analog.power_adc_disable();//PRADC disable to enable ADC

            analog.adc_enable();

            analog.adc_auto_trig();

            analog.analog_reference(reftype);

            match pin {
                0 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b000);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(0, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                1 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b001);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(1, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                2 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b010);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(2, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                3 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b011);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(3, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                4 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b100);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(4, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                5 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b101);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(5, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                6 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b110);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(6, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                7 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b111);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(7, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                8 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b000);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(0, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                9 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b001);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(1, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                10 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b010);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(2, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                11 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b011);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(4, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                12 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b100);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(4, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                13 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b101);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(5, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                14 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b110);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(6, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                15 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b111);
                    });
                    analog.didr2.update(|didr2| {
                        didr2.set_bit(7, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, true);
                    });
                }
                _ => unreachable!(),
            }

            analog.adc_con_start();

            // wait 25 ADC cycles
            let mut a:u32 = 0;
            let adcl =analog.adcl.read() as u32;
            a.set_bits(0..8,adcl.get_bits(0..8) );

            let adch =analog.adch.read() as u32;
            a.set_bits(8..10,adch.get_bits(0..2) );

            analog.adc_disable();

            a
        }
    }

    pub fn analog_write(&mut self, _pi: u32, _val: u32) {
        self.output();
    }
}

impl Analog {
    /// New pointer object created for Analog Structure.
    pub unsafe fn new() -> &'static mut Analog {
        &mut *(0x78 as *mut Analog)
    }

    /// Function to create a reference for Analog signals.
    pub fn analog_reference(&mut self, reftype: RefType) {
        match reftype {
            RefType::DEFAULT => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b01);
                });
            }
            RefType::INTERNAL1V1 => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b10);
                });
            }
            RefType::INTERNAL2V56 => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b11);
                });
            }
            RefType::EXTERNAL => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b00);
                });
            }
        }
    }

    ///Function is Used to enable the ADC
    pub fn adc_enable(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(7, true);
        });
    }

    pub fn power_adc_disable(&mut self) {
        unsafe {
            let pow = Power::new();
            write_volatile(&mut pow.prr0, pow.prr0 & (254));
        }
    }

    ///Function is Used to start a conversion in the ADC
    pub fn adc_con_start(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(6, true);
        });
    }

    ///Function is Used to stop auto triggering in the ADC
    pub fn adc_auto_trig(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(5, false);
        });
    }

    ///Function is Used to disable the ADC
    pub fn adc_disable(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(7, false);
        });
    }

    pub fn power_adc_enable(&mut self) {
        unsafe {
            let pow = Power::new();
            write_volatile(&mut pow.prr0, pow.prr0 | (1));
        }
    }
}
