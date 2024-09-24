use crate::types::{ReadStatType, ReadStatValue};
use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct ReadStatMissingValue {
    pub value: ReadStatValue,
    pub type_: ReadStatType,
}

impl ReadStatMissingValue {
    pub fn new(value: ReadStatValue, type_: ReadStatType) -> Self {
        Self { value, type_ }
    }

    pub fn builder() -> ReadStatMissingValueBuilder {
        ReadStatMissingValueBuilder::default()
    }
}
