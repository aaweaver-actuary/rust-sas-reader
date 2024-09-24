use crate::types::{ReadStatType, ReadStatValueLabel, SasLabelName};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct ReadStatLabelSet {
    pub type_: ReadStatType,
    pub name: SasLabelName,
    pub value_labels: ReadStatValueLabel,
    pub value_labels_count: u32,
    pub value_labels_capacity: u32,

    pub variables: Option<Vec<SasLabelName>>,
    pub variable_count: Option<u32>,
    pub variable_capacity: Option<u32>,
}

impl ReadStatLabelSet {
    pub fn new(
        type_: ReadStatType,
        name: SasLabelName,
        value_labels: ReadStatValueLabel,
        value_labels_count: u32,
        value_labels_capacity: u32,
    ) -> Self {
        Self {
            type_,
            name,
            value_labels,
            value_labels_count,
            value_labels_capacity,
            variables: None,
            variable_count: None,
            variable_capacity: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stat_label_set() {
        let label_set = ReadStatLabelSet::new(
            ReadStatType::Int64,
            SasLabelName::from_str("label"),
            ReadStatValueLabel::new(String::new(), 0, 0.0, 0, String::new(), 0, Vec::new()),
            0,
            0,
        );

        assert_eq!(label_set.type_, ReadStatType::Int64);
        assert_eq!(label_set.name, SasLabelName::from_str("label"));
        assert_eq!(label_set.value_labels_count, 0);
        assert_eq!(label_set.value_labels_capacity, 0);
    }
}
