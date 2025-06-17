#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitOrder {
    LSB,
}

pub trait BitOrderTrait {}

pub struct BitStreamTraits<T: BitOrderTrait>(T);
