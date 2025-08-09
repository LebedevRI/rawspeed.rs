use rawspeed_demuxers_naked::naked_demuxer::NakedDemuxer;
use rawspeed_demuxers_rawdemuxer::rawdemuxer::RawDemuxer;
use rawspeed_memory_nd_slice_procurement::ndsliceprocurement::NDSliceProcurementRequest;
use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Cameras;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Supported;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum RawParserError {
    DecoderError(String),
}

#[derive(Debug)]
#[non_exhaustive]
pub struct RawParser;

impl<'a, 'b, 'c> RawParser {
    #[inline(never)]
    pub fn get_decoder<F>(
        input: &'c [u8],
        cameras: &'b Cameras<'a>,
        check_camera_support_fn: F,
    ) -> Result<
        (Box<dyn RawDemuxer + 'c>, NDSliceProcurementRequest<u16>),
        RawParserError,
    >
    where
        F: FnOnce(Supported) -> Result<DecodeableCamera, String>,
    {
        match NakedDemuxer::new(input, cameras, check_camera_support_fn) {
            Ok((d, r)) => {
                let d = Box::new(d);
                Ok((d, r))
            }
            Err(s) => Err(RawParserError::DecoderError(s)),
        }
    }
}

#[cfg(test)]
mod tests;
