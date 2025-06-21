#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

#[inline]
#[cfg(target_endian = "little")]
#[must_use]
pub const fn get_host_endianness() -> Endianness {
    Endianness::Little
}

#[inline]
#[cfg(target_endian = "big")]
#[must_use]
pub const fn get_host_endianness() -> Endianness {
    Endianness::Big
}

pub trait SwapBytes {
    fn swap_bytes(self) -> Self;

    fn get_byte_swapped(self, cond: bool) -> Self;
}

macro_rules! impl_swap_bytes {
    ($($t:ty)+) => {
        $(
            impl SwapBytes for $t {
                #[inline]
                fn swap_bytes(self) -> Self {
                    self.swap_bytes()
                }

                #[inline]
                fn get_byte_swapped(self, cond: bool) -> Self {
                    if !cond {
                        self
                    } else {
                        <$t>::swap_bytes(self)
                    }
                }
            }
        )+
    };
}

impl_swap_bytes!(u16 u32 u64);

#[cfg(test)]
mod test;
