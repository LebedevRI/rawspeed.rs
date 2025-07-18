use crate::coord_common::Coord2D;
use crate::coord_common::RowIndex;
use crate::coord_common::RowLength;
use crate::coord_common::RowPitch;

#[derive(Debug)]
struct Array2DRefMut<'a, T> {
    slice: &'a mut [T],
    pitch: RowPitch,
    row_length: RowLength,
}

#[cfg_attr(not(test), expect(dead_code))]
impl<'a, T> Array2DRefMut<'a, T> {
    #[inline]
    const fn new(
        slice: &'a mut [T],
        row_length: RowLength,
        pitch: RowPitch,
    ) -> Self {
        assert!(!slice.is_empty());
        assert!(row_length.val() > 0);
        assert!(pitch.val() > 0);
        assert!(pitch.val() >= row_length.val());
        assert!(slice.len().is_multiple_of(pitch.val()));
        Self {
            slice,
            pitch,
            row_length,
        }
    }

    const fn pitch(&self) -> usize {
        self.pitch.val()
    }

    #[inline]
    #[must_use]
    const fn row_length(&self) -> usize {
        self.row_length.val()
    }

    #[inline]
    #[must_use]
    const fn num_rows(&self) -> usize {
        self.slice.len().checked_div(self.pitch()).unwrap()
    }

    #[inline]
    #[must_use]
    #[expect(clippy::unwrap_in_result)]
    fn get_row(&self, row: RowIndex) -> Option<&[T]> {
        if *row >= self.num_rows() {
            return None;
        }
        Some(
            {
                let full_row =
                    self.slice.chunks_exact(self.pitch()).nth(*row)?;
                full_row.get(..self.row_length())
            }
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    #[expect(clippy::unwrap_in_result)]
    fn get_row_mut(&mut self, row: RowIndex) -> Option<&mut [T]> {
        if *row >= self.num_rows() {
            return None;
        }
        Some(
            {
                let row_len = self.row_length();
                let full_row =
                    self.slice.chunks_exact_mut(self.pitch()).nth(*row)?;
                full_row.get_mut(..row_len)
            }
            .unwrap(),
        )
    }

    #[inline]
    #[must_use]
    fn get_elt(&self, index: Coord2D) -> Option<&T> {
        let row = self.get_row(RowIndex::new(index.row()))?;
        row.get(index.col())
    }

    #[inline]
    #[must_use]
    fn get_elt_mut(&mut self, index: Coord2D) -> Option<&mut T> {
        let row = self.get_row_mut(RowIndex::new(index.row()))?;
        row.get_mut(index.col())
    }
}

impl<T> core::ops::Index<RowIndex> for Array2DRefMut<'_, T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RowIndex) -> &Self::Output {
        self.get_row(index).unwrap()
    }
}

impl<T> core::ops::IndexMut<RowIndex> for Array2DRefMut<'_, T> {
    #[inline]
    fn index_mut(&mut self, index: RowIndex) -> &mut Self::Output {
        self.get_row_mut(index).unwrap()
    }
}

impl<T> core::ops::Index<Coord2D> for Array2DRefMut<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord2D) -> &Self::Output {
        self.get_elt(index).unwrap()
    }
}

impl<T> core::ops::IndexMut<Coord2D> for Array2DRefMut<'_, T> {
    #[inline]
    fn index_mut(&mut self, index: Coord2D) -> &mut Self::Output {
        self.get_elt_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests;
