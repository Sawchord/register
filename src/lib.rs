#![no_std]

extern crate byteorder;

use core::mem;
use core::ops::Range;
use byteorder::{ByteOrder, BE, LE};


extern crate num;
use num::Integer;

pub trait OffsetSize {
    fn offset(self) -> u8;
    fn size(self) -> u8;
}

impl OffsetSize for u8 {
    fn offset(self) -> u8 {
        self
    }

    fn size(self) -> u8 {
        1
    }
}

impl OffsetSize for Range<u8> {
    fn offset(self) -> u8 {
        self.start
    }

    fn size(self) -> u8 {
        self.end - self.start
    }
}

pub trait RegisterGet<N: Integer> {
    fn get(&self) -> N;
}

// The access traits of the registers
#[derive(Clone, Copy)]
pub struct Mask;

#[derive(Clone, Copy)]
pub struct Read;

#[derive(Clone, Copy)]
pub struct Write;

#[macro_export]
macro_rules! register {

($REGISTER:ident, $reset_value:expr, $uux:ty, {
        $(#[$($attr:tt)*] $bitfield:ident @ $range:expr,)+
    }) => {

        // FIXME: does copying make sense?
        // FIXME: pub(crate) or just pub
        #[derive(Clone, Copy)]
        pub struct $REGISTER<MODE> {
            bits: $uux,
            _mode: PhantomData<MODE>
        }

        impl $REGISTER<Mask> {
            #[allow(dead_code)]
            pub fn mask() -> $REGISTER<Mask> {
                $REGISTER { bits: 0, _mode: PhantomData}
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub fn $bitfield(&self) -> $uux {
                    let size = $range.size();
                    let offset = $range.offset();

                    ((1 << size) - 1) << offset
                }
            )+
        }

        impl<MODE> Default for $REGISTER<MODE> {
            fn default() -> Self {
                $REGISTER { bits: $reset_value, _mode: PhantomData}
            }
        }

        impl<MODE> RegisterGet<$uux> for $REGISTER<MODE> {
            fn get(&self) -> $uux {
                self.bits
            }
        }

        #[allow(non_snake_case)]
        #[allow(dead_code)]
        // Constructor
        pub fn $REGISTER(bits: $uux) -> $REGISTER<Read> {
            $REGISTER { bits, _mode: PhantomData}
        }

        impl $REGISTER<Read> {
            #[allow(dead_code)]
            pub fn modify(self) -> $REGISTER<Write> {
                $REGISTER { bits: self.bits, _mode: PhantomData}
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub fn $bitfield(&self) -> $uux {
                    let size = $range.size();
                    let offset = $range.size();

                    let mask = (1 << size) - 1;

                    (self.bits >> offset) & mask
                }
            )+
        }

        impl $REGISTER<Write> {
            #[allow(dead_code)]
            pub fn bits(self) -> $uux {
                self.bits
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub fn $bitfield(&mut self, mut bits: $uux) -> &mut Self {

                    let offset = $range.offset();
                    let size = $range.size();
                    let mask = (1 << size) - 1;

                    debug_assert!(bits <= mask);
                    bits &= mask;

                    self.bits &= !(mask << offset);
                    self.bits |= bits << offset;

                    self
                }
            )+
        }
    }
}


#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    use {Mask, Read, Write};
    use OffsetSize;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sanity() {
        register!(EIE, 0, u8, {
        #[doc = "Receive Error Interrupt Enable bit"]
        rxerie @ 0,
        #[doc = "Transmit Error Interrupt Enable bit"]
        txerie @ 1,
        #[doc = "Transmit Enable bit"]
        txie @ 3,
        #[doc = "Link Status Change Interrupt Enable bit"]
        linkie @ 4,
        #[doc = "DMA Interrupt Enable bit"]
        dmaie @ 5,
        #[doc = "Receive Packet Pending Interrupt Enable bit"]
        pktie @ 6,
        #[doc = "Global INT Interrupt Enable bit"]
        intie @ 7,
        });
        // TODO: Add actual test case


    }
}

