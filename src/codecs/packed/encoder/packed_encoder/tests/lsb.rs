use crate::packed_encoder::ExtraPadding;
use crate::packed_encoder::NumBytes;
use crate::packed_encoder::Packer;
use rawspeed_memory_bitstream::bitstream::BitOrder;
use rawspeed_std::array2dref::Array2DRef;
use rawspeed_std::coord_common::RowLength;
use rawspeed_std::coord_common::RowPitch;
use std::io::Write as _;

#[test]
fn vec_ctor_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::LSB;
    let img_storage: Vec<u8> = vec![1];
    let img =
        Array2DRef::new(&img_storage, RowLength::new(1), RowPitch::new(1));
    let mut buf = Cursor::new(vec![]);
    let _packer = Packer::new(&mut buf, bit_order, 8, img, |_| {
        ExtraPadding::new(NumBytes::new(0))
    });
}

#[test]
fn arr_ctor_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::LSB;
    let img_storage: Vec<u8> = vec![1];
    let img =
        Array2DRef::new(&img_storage, RowLength::new(1), RowPitch::new(1));
    let mut buf = [0_u8; 1024];
    let mut buf = Cursor::new(buf.as_mut());
    let _packer = Packer::new(&mut buf, bit_order, 8, img, |_| {
        ExtraPadding::new(NumBytes::new(0))
    });
}

#[test]
fn flush_arr_overflow_test() {
    use std::io::Cursor;
    let bit_order = BitOrder::LSB;
    let img_storage: Vec<u8> = vec![1];
    let img =
        Array2DRef::new(&img_storage, RowLength::new(1), RowPitch::new(1));
    let mut buf = [0_u8; 0];
    let mut buf = Cursor::new(buf.as_mut());
    let packer = Packer::new(&mut buf, bit_order, 8, img, |_| {
        ExtraPadding::new(NumBytes::new(0))
    });
    packer.pack().unwrap_err();
}

#[test]
fn u8_enumeration_test() -> std::io::Result<()> {
    type T = u8;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::LSB;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(
                    &mut buf,
                    bit_order,
                    T::BITS.try_into().unwrap(),
                    img,
                    |_| ExtraPadding::new(NumBytes::new(num_padding_bytes)),
                );
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![11, 0, 0, 0],
        vec![11, 0, 0, 0, 0],
        vec![11, 12, 0, 0],
        vec![11, 12, 0, 0, 0],
        vec![11, 12, 13, 0],
        vec![11, 12, 13, 0, 0],
        vec![11, 12, 13, 14],
        vec![11, 12, 13, 14, 0],
        vec![11, 12, 13, 14, 15, 0, 0, 0],
        vec![11, 12, 13, 14, 15, 0, 0, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 0, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 17, 0],
        vec![11, 12, 13, 14, 15, 16, 17, 0, 0],
        vec![11, 12, 13, 14, 15, 16, 17, 18],
        vec![11, 12, 13, 14, 15, 16, 17, 18, 0],
        vec![11, 0, 0, 0, 21, 0, 0, 0],
        vec![11, 0, 0, 0, 0, 21, 0, 0, 0, 0],
        vec![11, 12, 0, 0, 21, 22, 0, 0],
        vec![11, 12, 0, 0, 0, 21, 22, 0, 0, 0],
        vec![11, 12, 13, 0, 21, 22, 23, 0],
        vec![11, 12, 13, 0, 0, 21, 22, 23, 0, 0],
        vec![11, 12, 13, 14, 21, 22, 23, 24],
        vec![11, 12, 13, 14, 0, 21, 22, 23, 24, 0],
        vec![11, 12, 13, 14, 15, 0, 0, 0, 21, 22, 23, 24, 25, 0, 0, 0],
        vec![
            11, 12, 13, 14, 15, 0, 0, 0, 0, 21, 22, 23, 24, 25, 0, 0, 0, 0,
        ],
        vec![11, 12, 13, 14, 15, 16, 0, 0, 21, 22, 23, 24, 25, 26, 0, 0],
        vec![
            11, 12, 13, 14, 15, 16, 0, 0, 0, 21, 22, 23, 24, 25, 26, 0, 0, 0,
        ],
        vec![11, 12, 13, 14, 15, 16, 17, 0, 21, 22, 23, 24, 25, 26, 27, 0],
        vec![
            11, 12, 13, 14, 15, 16, 17, 0, 0, 21, 22, 23, 24, 25, 26, 27, 0, 0,
        ],
        vec![
            11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28,
        ],
        vec![
            11, 12, 13, 14, 15, 16, 17, 18, 0, 21, 22, 23, 24, 25, 26, 27, 28,
            0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u4_enumeration_test() -> std::io::Result<()> {
    type T = u8;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=7 {
            let mut storage: Vec<T> = vec![];
            for e in 1..=(num_rows * num_cols) {
                assert!(e <= 0xF);
                storage.push(e.try_into().unwrap());
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::LSB;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(&mut buf, bit_order, 4, img, |_| {
                    ExtraPadding::new(NumBytes::new(num_padding_bytes))
                });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![33, 0, 0, 0],
        vec![33, 0, 0, 0, 0],
        vec![33, 3, 0, 0],
        vec![33, 3, 0, 0, 0],
        vec![33, 67, 0, 0],
        vec![33, 67, 0, 0, 0],
        vec![33, 67, 5, 0],
        vec![33, 67, 5, 0, 0],
        vec![33, 67, 101, 0],
        vec![33, 67, 101, 0, 0],
        vec![33, 67, 101, 7],
        vec![33, 67, 101, 7, 0],
        vec![1, 0, 0, 0, 2, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 2, 0, 0, 0, 0],
        vec![33, 0, 0, 0, 67, 0, 0, 0],
        vec![33, 0, 0, 0, 0, 67, 0, 0, 0, 0],
        vec![33, 3, 0, 0, 84, 6, 0, 0],
        vec![33, 3, 0, 0, 0, 84, 6, 0, 0, 0],
        vec![33, 67, 0, 0, 101, 135, 0, 0],
        vec![33, 67, 0, 0, 0, 101, 135, 0, 0, 0],
        vec![33, 67, 5, 0, 118, 152, 10, 0],
        vec![33, 67, 5, 0, 0, 118, 152, 10, 0, 0],
        vec![33, 67, 101, 0, 135, 169, 203, 0],
        vec![33, 67, 101, 0, 0, 135, 169, 203, 0, 0],
        vec![33, 67, 101, 7, 152, 186, 220, 14],
        vec![33, 67, 101, 7, 0, 152, 186, 220, 14, 0],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u16_enumeration_test() -> std::io::Result<()> {
    type T = u16;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::LSB;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(
                    &mut buf,
                    bit_order,
                    T::BITS.try_into().unwrap(),
                    img,
                    |_| ExtraPadding::new(NumBytes::new(num_padding_bytes)),
                );
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![11, 0, 0, 0],
        vec![11, 0, 0, 0, 0],
        vec![11, 0, 12, 0],
        vec![11, 0, 12, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 0],
        vec![11, 0, 0, 0, 21, 0, 0, 0],
        vec![11, 0, 0, 0, 0, 21, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 21, 0, 22, 0],
        vec![11, 0, 12, 0, 0, 21, 0, 22, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0, 21, 0, 22, 0, 23, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 0, 0, 0, 21, 0, 22, 0, 23, 0, 0, 0, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 21, 0, 22, 0, 23, 0, 24, 0],
        vec![11, 0, 12, 0, 13, 0, 14, 0, 0, 21, 0, 22, 0, 23, 0, 24, 0, 0],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 21, 0, 22, 0, 23, 0, 24,
            0, 25, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0, 0, 21, 0, 22, 0, 23, 0,
            24, 0, 25, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 21, 0, 22, 0, 23, 0, 24,
            0, 25, 0, 26, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 0, 21, 0, 22, 0, 23, 0,
            24, 0, 25, 0, 26, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 21, 0, 22,
            0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0, 0, 21, 0,
            22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 21, 0, 22,
            0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 28, 0,
        ],
        vec![
            11, 0, 12, 0, 13, 0, 14, 0, 15, 0, 16, 0, 17, 0, 18, 0, 0, 21, 0,
            22, 0, 23, 0, 24, 0, 25, 0, 26, 0, 27, 0, 28, 0, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
fn u12_enumeration_test() -> std::io::Result<()> {
    type T = u16;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::LSB;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(&mut buf, bit_order, 12, img, |_| {
                    ExtraPadding::new(NumBytes::new(num_padding_bytes))
                });
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![11, 0, 0, 0],
        vec![11, 0, 0, 0, 0],
        vec![11, 192, 0, 0],
        vec![11, 192, 0, 0, 0],
        vec![11, 192, 0, 13, 0, 0, 0, 0],
        vec![11, 192, 0, 13, 0, 0, 0, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 0, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 15, 0],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 1, 0, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 1, 0, 0, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 0, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 32, 1],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 32, 1, 0],
        vec![11, 0, 0, 0, 21, 0, 0, 0],
        vec![11, 0, 0, 0, 0, 21, 0, 0, 0, 0],
        vec![11, 192, 0, 0, 21, 96, 1, 0],
        vec![11, 192, 0, 0, 0, 21, 96, 1, 0, 0],
        vec![11, 192, 0, 13, 0, 0, 0, 0, 21, 96, 1, 23, 0, 0, 0, 0],
        vec![11, 192, 0, 13, 0, 0, 0, 0, 0, 21, 96, 1, 23, 0, 0, 0, 0, 0],
        vec![11, 192, 0, 13, 224, 0, 0, 0, 21, 96, 1, 23, 128, 1, 0, 0],
        vec![
            11, 192, 0, 13, 224, 0, 0, 0, 0, 21, 96, 1, 23, 128, 1, 0, 0, 0,
        ],
        vec![11, 192, 0, 13, 224, 0, 15, 0, 21, 96, 1, 23, 128, 1, 25, 0],
        vec![
            11, 192, 0, 13, 224, 0, 15, 0, 0, 21, 96, 1, 23, 128, 1, 25, 0, 0,
        ],
        vec![
            11, 192, 0, 13, 224, 0, 15, 0, 1, 0, 0, 0, 21, 96, 1, 23, 128, 1,
            25, 160, 1, 0, 0, 0,
        ],
        vec![
            11, 192, 0, 13, 224, 0, 15, 0, 1, 0, 0, 0, 0, 21, 96, 1, 23, 128,
            1, 25, 160, 1, 0, 0, 0, 0,
        ],
        vec![
            11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 0, 0, 21, 96, 1, 23, 128, 1,
            25, 160, 1, 27, 0, 0,
        ],
        vec![
            11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 0, 0, 0, 21, 96, 1, 23, 128,
            1, 25, 160, 1, 27, 0, 0, 0,
        ],
        vec![
            11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 32, 1, 21, 96, 1, 23, 128, 1,
            25, 160, 1, 27, 192, 1,
        ],
        vec![
            11, 192, 0, 13, 224, 0, 15, 0, 1, 17, 32, 1, 0, 21, 96, 1, 23, 128,
            1, 25, 160, 1, 27, 192, 1, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}

#[test]
#[expect(clippy::too_many_lines)]
fn u32_enumeration_test() -> std::io::Result<()> {
    type T = u32;
    let mut res: Vec<Vec<u8>> = vec![];
    for num_rows in 1..=2 {
        for num_cols in 1..=8 {
            let mut storage: Vec<T> = vec![];
            for row in 1..=num_rows {
                for col in 1..=num_cols {
                    storage.push((10 * row + col).try_into().unwrap());
                }
            }
            let img = Array2DRef::new(
                &storage,
                RowLength::new(num_cols),
                RowPitch::new(num_cols),
            );
            for num_padding_bytes in 0..=1 {
                use std::io::Cursor;
                let bit_order = BitOrder::LSB;
                let mut buf = Cursor::new(vec![]);
                let packer = Packer::new(
                    &mut buf,
                    bit_order,
                    T::BITS.try_into().unwrap(),
                    img,
                    |_| ExtraPadding::new(NumBytes::new(num_padding_bytes)),
                );
                packer.pack()?;
                buf.flush()?;
                res.push(buf.get_ref().clone());
            }
        }
    }
    let expected: Vec<Vec<u8>> = vec![
        vec![11, 0, 0, 0],
        vec![11, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 0,
        ],
        vec![11, 0, 0, 0, 21, 0, 0, 0],
        vec![11, 0, 0, 0, 0, 21, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0],
        vec![11, 0, 0, 0, 12, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 0],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 21, 0, 0, 0,
            22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 0, 21, 0, 0, 0,
            22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 0,
            21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0,
            25, 0, 0, 0, 26, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0,
            25, 0, 0, 0, 26, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0,
            24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0, 23, 0, 0, 0,
            24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
            28, 0, 0, 0,
        ],
        vec![
            11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0,
            16, 0, 0, 0, 17, 0, 0, 0, 18, 0, 0, 0, 0, 21, 0, 0, 0, 22, 0, 0, 0,
            23, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 26, 0, 0, 0, 27, 0, 0, 0,
            28, 0, 0, 0, 0,
        ],
    ];
    assert_eq!(res, expected);
    Ok(())
}
