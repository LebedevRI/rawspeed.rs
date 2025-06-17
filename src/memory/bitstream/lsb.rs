use crate::bitstream::BitOrder;
use crate::bitstream::BitOrderTrait;
use crate::bitstream::BitStreamTraits;
use crate::bitstreamflow::BitStreamFlow;
use rawspeed_memory_endianness::endianness::Endianness;

pub struct BitOrderLSB;

impl BitOrderTrait for BitOrderLSB {}

impl BitStreamTraits<BitOrderLSB> {
    pub const TAG: BitOrder = BitOrder::LSB;
    pub const STREAM_FLOW: BitStreamFlow = BitStreamFlow::LowInHighOut;
    pub const FIXED_SIZE_CHUNKS: bool = true;

    // type ChunkType = u32;
    //
    // Needs "inherent associated types" unstable feature,
    // See https://github.com/rust-lang/rust/issues/8995

    pub const CHUNK_ENDIANNESS: Endianness = Endianness::Little;

    pub const MIN_LOAD_STEP_BYTE_MULTIPLE: i32 = 1;
}
