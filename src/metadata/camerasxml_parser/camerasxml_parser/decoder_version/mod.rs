use super::Int;
use super::xmlparser;

impl_attr_matcher!(
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct DecoderVersion {
        decoder_version: Int,
    }
);
