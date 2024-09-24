use crate::types::{
    ReadStatAlignment, ReadStatLabelSet, ReadStatMeasure, ReadStatMissingRanges, ReadStatType,
    ReadStatValueLabel, SasLabelName,
};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct ReadStatVariable {
    pub type_: ReadStatType,
    pub index: u32,
    pub name: String,
    pub format: String,
    pub label: String,
    pub label_set: ReadStatLabelSet,
    pub offset: u64,
    pub storage_width: u64,
    pub user_width: u64,
    pub missing_ranges: ReadStatMissingRanges,
    pub measure: Option<ReadStatMeasure>,
    pub alignment: Option<ReadStatAlignment>,
    pub display_width: u64,
    pub display_decimals: u64,
    pub skip: bool,
    pub index_after_skipping: u32,
}

impl ReadStatVariable {
    pub fn new(
        type_: ReadStatType,
        index: u32,
        name: String,
        format: String,
        label: String,
        label_set: ReadStatLabelSet,
        offset: u64,
        storage_width: u64,
        user_width: u64,
        missing_ranges: ReadStatMissingRanges,
        measure: Option<ReadStatMeasure>,
        alignment: Option<ReadStatAlignment>,
        display_width: u64,
        display_decimals: u64,
        skip: bool,
        index_after_skipping: u32,
    ) -> Self {
        Self {
            type_,
            index,
            name,
            format,
            label,
            label_set,
            offset,
            storage_width,
            user_width,
            missing_ranges,
            measure,
            alignment,
            display_width,
            display_decimals,
            skip,
            index_after_skipping,
        }
    }

    pub fn builder() -> ReadStatVariableBuilder {
        ReadStatVariableBuilder::default()
    }
}

impl ReadStatVariableBuilder {
    pub fn default() -> Self {
        Self {
            type_: Some(ReadStatType::Double),
            index: Some(0),
            name: Some(String::new()),
            format: Some(String::new()),
            label: Some(String::new()),
            label_set: Some(ReadStatLabelSet::new(
                ReadStatType::Double,
                SasLabelName::from_str("label"),
                ReadStatValueLabel::new(String::new(), 0, 0.0, 0, String::new(), 0, Vec::new()),
                0,
                0,
            )),
            offset: Some(0),
            storage_width: Some(0),
            user_width: Some(0),
            missing_ranges: Some(ReadStatMissingRanges::new(Vec::new(), 0)),
            measure: Some(Some(ReadStatMeasure::Nominal)),
            alignment: Some(Some(ReadStatAlignment::Left)),
            display_width: Some(0),
            display_decimals: Some(0),
            skip: Some(false),
            index_after_skipping: Some(0),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::types::{ReadStatType, ReadStatValueLabel, SasLabelName};

    #[test]
    fn test_read_stat_variable_type() {
        let variable = ReadStatVariable::builder()
            .type_(ReadStatType::Int64)
            .build()
            .unwrap();
        assert_eq!(variable.type_, ReadStatType::Int64);
    }

    #[test]
    fn test_read_stat_variable_index() {
        let variable = ReadStatVariable::builder().index(1).build().unwrap();
        assert_eq!(variable.index, 1);
    }

    #[test]
    fn test_read_stat_variable_name() {
        let variable = ReadStatVariable::builder()
            .name(String::from("name"))
            .build()
            .unwrap();
        assert_eq!(variable.name, String::from("name"));
    }

    #[test]
    fn test_read_stat_variable_format() {
        let variable = ReadStatVariable::builder()
            .format(String::from("format"))
            .build()
            .unwrap();
        assert_eq!(variable.format, String::from("format"));
    }

    #[test]
    fn test_read_stat_variable_label() {
        let variable = ReadStatVariable::builder()
            .label(String::from("label"))
            .build()
            .unwrap();
        assert_eq!(variable.label, String::from("label"));
    }

    #[test]
    fn test_read_stat_variable_label_set() {
        let variable = ReadStatVariable::builder().build().unwrap();
        assert_eq!(
            variable.label_set,
            ReadStatLabelSet::new(
                ReadStatType::Double,
                SasLabelName::from_str("label"),
                ReadStatValueLabel::new(String::new(), 0, 0.0, 0, String::new(), 0, Vec::new()),
                0,
                0,
            )
        );
    }

    #[test]
    fn test_read_stat_variable_offset() {
        let variable = ReadStatVariable::builder().offset(1).build().unwrap();
        assert_eq!(variable.offset, 1);
    }

    #[test]
    fn test_read_stat_variable() {
        let variable = ReadStatVariable::builder().build().unwrap();
        assert_eq!(variable.storage_width, 0);
        assert_eq!(variable.user_width, 0);
        assert_eq!(
            variable.missing_ranges,
            ReadStatMissingRanges::new(Vec::new(), 0)
        );
        assert_eq!(variable.display_width, 0);
        assert_eq!(variable.display_decimals, 0);
        assert_eq!(variable.skip, false);
        assert_eq!(variable.index_after_skipping, 0);
    }
}
