use rawspeed_memory_bitstream::bitstream::BitOrderTrait;

pub struct BitVacuumerTraits<T: BitOrderTrait>(T);

pub struct BitVacuumer<T: BitOrderTrait> {}
