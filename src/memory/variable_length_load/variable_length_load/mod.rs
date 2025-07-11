pub trait VariableLengthLoad {
    fn variable_length_load(&mut self, src: &[u8], src_pos: usize);
}

impl VariableLengthLoad for [u8] {
    #[inline]
    fn variable_length_load(&mut self, src: &[u8], src_pos: usize) {
        self.fill(0);

        let src_pos = core::cmp::min(src_pos, src.len());
        let src = src.get(src_pos..).unwrap();
        let copy_size = core::cmp::min(self.len(), src.len());
        let dest = self.get_mut(..copy_size).unwrap();
        let src = src.get(..copy_size).unwrap();
        dest.copy_from_slice(src);
    }
}

#[cfg(test)]
mod tests;
