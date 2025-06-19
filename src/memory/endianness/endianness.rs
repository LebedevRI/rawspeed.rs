pub mod endianness {

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Endianness {
        Little,
        Big,
    }

    #[inline]
    #[cfg(target_endian = "little")]
    pub fn get_host_endianness() -> Endianness {
        Endianness::Little
    }

    #[inline]
    #[cfg(target_endian = "big")]
    pub fn get_host_endianness() -> Endianness {
        Endianness::Big
    }

    pub trait SwapBytes<T> {
        fn swap_bytes(self) -> Self;
    }

    macro_rules! impl_swap_bytes {
        ($($t:ty)+) => {
            $(
                impl<T> SwapBytes<T> for $t {
                    #[inline]
                    fn swap_bytes(self) -> Self {
                        self.swap_bytes()
                    }
                }
            )+
        };
    }

    impl_swap_bytes!(u16 u32 u64);

    #[cfg(test)]
    mod tests {

        use super::*;

        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum EndiannessDetectionError {
            Unknown,
        }

        pub fn get_host_endianness_runtime()
        -> Result<Endianness, EndiannessDetectionError> {
            match u16::from_ne_bytes([1, 0]).to_le_bytes() {
                [1, 0] => Ok(Endianness::Little),
                [0, 1] => Ok(Endianness::Big),
                _ => Err(EndiannessDetectionError::Unknown),
            }
        }

        #[test]
        fn compile_time_succeeds() {
            get_host_endianness();
        }

        #[test]
        fn run_time_succeeds() {
            get_host_endianness_runtime().unwrap();
        }

        #[test]
        fn basic() {
            assert_eq!(
                get_host_endianness(),
                get_host_endianness_runtime().expect("")
            );
        }

        #[test]
        fn swap_bytes_test() {
            macro_rules! test {
                ($($t:ty)+) => {
                    $(
                        {
                            const NUM_BYTES: usize = (<$t>::BITS / 8) as usize;
                            let mut bytes = [0; NUM_BYTES];
                            for i in 0..NUM_BYTES {
                                bytes[i] = 1 + i as u8;
                            }
                            let bits = <$t>::from_ne_bytes(bytes);
                            let swapped_bits = <$t as SwapBytes<$t>>::swap_bytes(bits);
                            let swapped_bytes = <$t>::to_ne_bytes(swapped_bits);
                            assert_eq!(swapped_bytes.to_vec(),
                                       (bytes.iter().copied().rev().collect::<Vec<u8>>()));
                        }
                    )+
                };
            }
            test!(u16 u32 u64);
        }
    }
}
