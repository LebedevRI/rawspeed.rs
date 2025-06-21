use super::*;

use rawspeed_memory_bitstream::bitstream::BitOrderMSB32;

#[allow(dead_code)]
pub type BitVacuumerMSB32<'a, W> = BitVacuumerBase<'a, BitOrderMSB32, W>;

impl BitVacuumerUseDefaultDrainImpl for BitOrderMSB32 {}

#[cfg(test)]
mod test;
