use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitStreamFlow {
    LowInHighOut,
}

pub trait BitStreamFlowTrait {}

pub struct BitStreamCacheBase<T: BitStreamFlowTrait> {
    // The actual bits stored in the cache
    cache: u64,

    // Bits left in cache
    fill_level: u32,

    _phantom_data: PhantomData<T>,
}

impl<T: BitStreamFlowTrait> BitStreamCacheBase<T> {
    // Width of cache, in bits
    const SIZE: u32 = u8::BITS;

    // How many bits could be requested to be filled
    const MAX_GET_BITS: u32 = u32::BITS;

    fn new() -> Self {
        Self {
            cache: 0,
            fill_level: 0,
            _phantom_data: PhantomData,
        }
    }
}

pub trait BitStreamCache {
    fn push(&mut self, bits: u64, count: u32);
    fn peek(&self, count: u32) -> u64;
    fn skip(&mut self, count: u32);
}

trait Integer {}
impl Integer for u64 {}

trait Bitwidth {
    fn bitwidth() -> u32;
}

impl<T: Integer> Bitwidth for T {
    fn bitwidth() -> u32 {
        const BITS_PER_BYTE: usize = 8;
        (BITS_PER_BYTE * std::mem::size_of::<T>()) as u32
    }
}

fn extract_high_bits<T: Integer + std::ops::Shr<u32, Output = T>>(
    value: T,
    num_bits: u32,
) -> T {
    let num_low_bits_to_skip = T::bitwidth() - num_bits;
    value >> num_low_bits_to_skip
}

pub struct BitStreamFlowLowInHighOut;

impl BitStreamFlowTrait for BitStreamFlowLowInHighOut {}

impl BitStreamCache for BitStreamCacheBase<BitStreamFlowLowInHighOut> {
    fn push(&mut self, bits: u64, count: u32) {
        let vacant_bits: u32 = Self::SIZE - self.fill_level;
        // If we just directly 'or' these low bits into the cache right now,
        // how many unfilled bits of a gap will there be
        // in the middle of a cache?
        let empty_bits_gap = vacant_bits - count;
        if count != 0 {
            // So just shift the new bits so that there is no such gap.
            self.cache |= bits << empty_bits_gap;
        }
        self.fill_level += count;
    }
    fn peek(&self, count: u32) -> u64 {
        extract_high_bits(self.cache, count)
    }
    fn skip(&mut self, count: u32) {
        self.cache <<= count;
        self.fill_level -= count;
    }
}
