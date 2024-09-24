use crate::sas::is_missing;
use crate::types::{ReadStatType, ReadStatValueType};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct ReadStatValue {
    pub type_: ReadStatType,
    pub tags: Vec<String>,
    pub value: ReadStatValueType,
}

impl ReadStatValue {
    pub fn new(type_: ReadStatType, tags: Vec<String>, value: ReadStatValueType) -> Self {
        Self { type_, tags, value }
    }

    pub fn builder() -> ReadStatValueBuilder {
        ReadStatValueBuilder::default()
    }

    /// Return true if the value is "system missing", eg if it was delivered to value
    /// handlers as NaN.
    pub fn is_system_missing(&self) -> bool {
        match self.value {
            ReadStatValueType::Double(v) => v.is_nan(),
            ReadStatValueType::String(ref v) => v.is_empty(),
            _ => false,
        }
    }

    /// Return true if the value is "tagged missing", eg if it was delivered to value
    pub fn is_tagged_missing_by_sas(&self) -> bool {
        is_missing::is_tagged_missing(&self)
    }

    /// Return true if the value is missing, either system missing or tagged missing.
    pub fn is_missing(&self) -> bool {
        self.is_system_missing() || self.is_tagged_missing_by_sas()
    }
}

impl ReadStatValueBuilder {
    pub fn default() -> Self {
        Self {
            type_: Some(ReadStatType::Double),
            tags: Some(Vec::new()),
            value: Some(ReadStatValueType::Double(0.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stat_value() {
        let value = ReadStatValue::builder().build().unwrap();
        assert_eq!(value.type_, ReadStatType::Double);
        assert_eq!(value.tags, Vec::<String>::new());
        assert_eq!(value.value, ReadStatValueType::Double(0.0));
    }

    #[test]
    fn test_read_stat_value_new() {
        let value = ReadStatValue::new(
            ReadStatType::Double,
            Vec::new(),
            ReadStatValueType::Double(0.0),
        );
        assert_eq!(value.type_, ReadStatType::Double);
        assert_eq!(value.tags, Vec::<String>::new());
        assert_eq!(value.value, ReadStatValueType::Double(0.0));
    }

    #[test]
    fn test_read_stat_value_is_system_missing() {
        let value = ReadStatValue::builder()
            .value(ReadStatValueType::Double(f64::NAN))
            .build()
            .unwrap();
        assert_eq!(value.is_system_missing(), true);
    }

    #[test]
    fn test_read_stat_value_is_tagged_missing() {
        let value = ReadStatValue::builder().build().unwrap();
        assert_eq!(value.is_tagged_missing_by_sas(), false);
    }

    #[test]
    fn test_sas_missing_char() {
        let value = ReadStatValue::builder()
            .tags(vec![String::from(".")])
            .build()
            .unwrap();
        assert_eq!(value.is_tagged_missing_by_sas(), true);
    }

    #[test]
    fn test_read_stat_value_is_missing() {
        let value = ReadStatValue::builder().build().unwrap();
        assert_eq!(value.is_missing(), true);
    }

    #[test]
    fn test_read_stat_value_is_not_missing() {
        let value = ReadStatValue::builder()
            .value(ReadStatValueType::Double(1.0))
            .build()
            .unwrap();
        assert_eq!(value.is_missing(), false);
    }
}
