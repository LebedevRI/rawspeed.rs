use crate::{LoopPeelParams, LoopUnrollParams, LoopXFormConf};

use super::UnrollMethod;

pub fn perform_loop_unroll(
    c: LoopXFormConf<LoopUnrollParams>,
) -> proc_macro2::TokenStream {
    match c.params.unroll_method {
        UnrollMethod::Runtime => unroll_runtime::transform(&c),
        UnrollMethod::WithRemainder => unroll_with_remainder::transform(c),
    }
}
pub fn perform_loop_peel(
    c: LoopXFormConf<LoopPeelParams>,
) -> proc_macro2::TokenStream {
    peel::transform(c)
}

mod utils {
    pub mod loop_break_labeller;
}

mod peel;
mod unroll_runtime;
mod unroll_with_remainder;
