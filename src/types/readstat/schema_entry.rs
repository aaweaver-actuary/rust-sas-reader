// typedef struct readstat_schema_entry_s {
// uint32_t            row;
// uint32_t            col;
// uint32_t            len;
// int                 skip;
// readstat_variable_t variable;
// char                labelset[32];
// char                decimal_separator;
// } readstat_schema_entry_t;

use crate::types::ReadStatVariable;
use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct ReadStatSchemaEntry {
    pub row: u32,
    pub col: u32,
    pub len: u32,
    pub skip: bool,
    pub variable: ReadStatVariable,
    pub labelset: String,
    pub decimal_separator: char,
}

impl ReadStatSchemaEntry {
    pub fn new(
        row: u32,
        col: u32,
        len: u32,
        skip: bool,
        variable: ReadStatVariable,
        labelset: String,
        decimal_separator: char,
    ) -> Self {
        Self {
            row,
            col,
            len,
            skip,
            variable,
            labelset,
            decimal_separator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stat_schema_entry() {
        let schema_entry = ReadStatSchemaEntry::new(
            0,
            0,
            0,
            false,
            ReadStatVariable::builder().build().unwrap(),
            String::new(),
            ' ',
        );

        assert_eq!(schema_entry.row, 0);
        assert_eq!(schema_entry.col, 0);
        assert_eq!(schema_entry.len, 0);
        assert_eq!(schema_entry.skip, false);
        assert_eq!(schema_entry.labelset, String::new());
        assert_eq!(schema_entry.decimal_separator, ' ');
    }
}
