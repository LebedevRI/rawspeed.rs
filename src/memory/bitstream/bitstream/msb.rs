use super::{BitOrder, BitOrderTrait, BitStreamTraits, Endianness};

use rawspeed_memory_bitstreamcache::bitstreamcache;

#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct BitOrderMSB;

impl BitOrderTrait for BitOrderMSB {}

impl BitStreamTraits for BitOrderMSB {
    const TAG: BitOrder = BitOrder::MSB;

    type StreamFlow = bitstreamcache::BitStreamCacheLowInHighOut;

    const FIXED_SIZE_CHUNKS: bool = true;

    type ChunkType = u32;

    const CHUNK_ENDIANNESS: Endianness = Endianness::Big;

    const MIN_LOAD_STEP_BYTE_MULTIPLE: u32 = 1;
}
