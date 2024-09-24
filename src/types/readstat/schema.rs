// typedef struct readstat_schema_s {
// char                    filename[255];
// uint32_t                rows_per_observation;
// uint32_t                cols_per_observation;
// int                     first_line;
// int                     entry_count;
// char                    field_delimiter;
// readstat_schema_entry_t *entries;
// } readstat_schema_t;

use crate::types::ReadStatSchemaEntry;
use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct ReadStatSchema {
    pub filename: String,
    pub rows_per_observation: u32,
    pub cols_per_observation: u32,
    pub first_line: u32,
    pub entry_count: u32,
    pub field_delimiter: char,
    pub entries: Vec<ReadStatSchemaEntry>,
}

impl ReadStatSchema {
    pub fn new(
        filename: String,
        rows_per_observation: u32,
        cols_per_observation: u32,
        first_line: u32,
        entry_count: u32,
        field_delimiter: char,
        entries: Vec<ReadStatSchemaEntry>,
    ) -> Self {
        Self {
            filename,
            rows_per_observation,
            cols_per_observation,
            first_line,
            entry_count,
            field_delimiter,
            entries,
        }
    }

    pub fn builder() -> ReadStatSchemaBuilder {
        ReadStatSchemaBuilder::default()
    }
}

impl ReadStatSchemaBuilder {
    pub fn default() -> Self {
        Self {
            filename: Some(String::new()),
            rows_per_observation: Some(0),
            cols_per_observation: Some(0),
            first_line: Some(0),
            entry_count: Some(0),
            field_delimiter: Some(' '),
            entries: Some(Vec::with_capacity(32)),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    use crate::types::ReadStatVariable;

    #[test]
    fn test_read_stat_schema() {
        let schema = ReadStatSchema::builder().build().unwrap();
        assert_eq!(schema.rows_per_observation, 0);
        assert_eq!(schema.cols_per_observation, 0);
        assert_eq!(schema.first_line, 0);
        assert_eq!(schema.entry_count, 0);
        assert_eq!(schema.field_delimiter, ' ');
        assert_eq!(schema.entries, Vec::<ReadStatSchemaEntry>::new());
    }

    #[test]
    fn test_read_stat_schema_new() {
        let schema = ReadStatSchema::new(String::new(), 0, 0, 0, 0, ' ', Vec::new());

        assert_eq!(schema.rows_per_observation, 0);
        assert_eq!(schema.cols_per_observation, 0);
        assert_eq!(schema.first_line, 0);
        assert_eq!(schema.entry_count, 0);
        assert_eq!(schema.field_delimiter, ' ');
        assert_eq!(schema.entries, Vec::<ReadStatSchemaEntry>::new());
    }

    #[test]
    fn test_read_stat_schema_builder() {
        let schema = ReadStatSchema::builder()
            .filename("test".to_string())
            .rows_per_observation(1)
            .cols_per_observation(1)
            .first_line(1)
            .entry_count(1)
            .field_delimiter(',')
            .entries(vec![ReadStatSchemaEntry::new(
                0,
                0,
                0,
                false,
                ReadStatVariable::builder()
                    .name("test".to_string())
                    .build()
                    .unwrap(),
                String::new(),
                ',',
            )])
            .build()
            .unwrap();

        assert_eq!(schema.filename, "test");
        assert_eq!(schema.rows_per_observation, 1);
        assert_eq!(schema.cols_per_observation, 1);
        assert_eq!(schema.first_line, 1);
        assert_eq!(schema.entry_count, 1);
        assert_eq!(schema.field_delimiter, ',');
        assert_eq!(schema.entries.len(), 1);
    }

    #[test]
    fn test_read_stat_schema_builder_default() {
        let schema = ReadStatSchema::builder().build().unwrap();
        assert_eq!(schema.filename, String::new());
        assert_eq!(schema.rows_per_observation, 0);
        assert_eq!(schema.cols_per_observation, 0);
        assert_eq!(schema.first_line, 0);
        assert_eq!(schema.entry_count, 0);
        assert_eq!(schema.field_delimiter, ' ');
        assert_eq!(schema.entries, Vec::<ReadStatSchemaEntry>::new());
    }
}
