use crate::types::ReadStatValue;
use derive_builder::Builder;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct ReadStatMissingRanges {
    pub missing_ranges: Vec<ReadStatValue>,
    pub missing_ranges_count: u32,
}

impl ReadStatMissingRanges {
    pub fn new(missing_ranges: Vec<ReadStatValue>, missing_ranges_count: u32) -> Self {
        Self {
            missing_ranges,
            missing_ranges_count,
        }
    }

    pub fn builder() -> ReadStatMissingRangesBuilder {
        ReadStatMissingRangesBuilder::default()
    }
}

impl ReadStatMissingRangesBuilder {
    pub fn default() -> Self {
        Self {
            missing_ranges: Some(Vec::with_capacity(32)),
            missing_ranges_count: Some(0),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_read_stat_missing_ranges() {
        let missing_ranges = ReadStatMissingRanges::builder().build().unwrap();
        assert_eq!(missing_ranges.missing_ranges_count, 0);
    }

    #[test]
    fn test_read_stat_missing_ranges_new() {
        let missing_ranges = ReadStatMissingRanges::new(Vec::new(), 0);
        assert_eq!(missing_ranges.missing_ranges_count, 0);
    }
}
