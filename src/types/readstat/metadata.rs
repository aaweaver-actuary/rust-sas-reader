use crate::types::{
    ReadStatCompression, ReadStatEncoding, ReadStatEndianness, ReadStatFileLabel, SasTableName,
};
use datetime::Instant;
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct ReadStatMetadata {
    pub row_count: u128,
    pub var_count: u128,
    pub created_at: Instant,
    pub modified_at: Instant,
    pub file_format_version: u16,
    pub compression: ReadStatCompression,
    pub endianness: ReadStatEndianness,
    pub table_name: SasTableName,
    pub file_label: ReadStatFileLabel,
    pub file_encoding: ReadStatEncoding,
    pub is_64bit: bool,
}

impl ReadStatMetadata {
    pub fn new(
        row_count: u128,
        var_count: u128,
        created_at: Instant,
        modified_at: Instant,
        file_format_version: u16,
        compression: ReadStatCompression,
        endianness: ReadStatEndianness,
        table_name: SasTableName,
        file_label: ReadStatFileLabel,
        file_encoding: ReadStatEncoding,
        is_64bit: bool,
    ) -> Self {
        Self {
            row_count,
            var_count,
            created_at,
            modified_at,
            file_format_version,
            compression,
            endianness,
            table_name,
            file_label,
            file_encoding,
            is_64bit,
        }
    }

    pub fn builder() -> ReadStatMetadataBuilder {
        ReadStatMetadataBuilder::default()
    }
}

impl ReadStatMetadataBuilder {
    pub fn default() -> Self {
        Self {
            row_count: Some(0),
            var_count: Some(0),
            created_at: Some(Instant::now()),
            modified_at: Some(Instant::now()),
            file_format_version: Some(0),
            compression: Some(ReadStatCompression::None),
            endianness: Some(ReadStatEndianness::Little),
            table_name: Some(SasTableName::from_str("table").expect("Need a table name")),
            file_label: Some(ReadStatFileLabel::Sas),
            file_encoding: Some(ReadStatEncoding::Latin1),
            is_64bit: Some(false),
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_new_readstat_metadata() {
        let metadata = ReadStatMetadata::new(
            100,
            10,
            Instant::now(),
            Instant::now(),
            0,
            ReadStatCompression::None,
            ReadStatEndianness::Little,
            SasTableName::from_str("table").expect("Need a table name"),
            ReadStatFileLabel::Sas,
            ReadStatEncoding::Latin1,
            false,
        );

        assert_eq!(metadata.row_count, 100);
        assert_eq!(metadata.var_count, 10);
        assert_eq!(metadata.file_format_version, 0);
        assert_eq!(metadata.compression, ReadStatCompression::None);
        assert_eq!(metadata.endianness, ReadStatEndianness::Little);
        assert_eq!(
            metadata.table_name,
            SasTableName::from_str("table").expect("Need a table name")
        );
        assert_eq!(metadata.file_label, ReadStatFileLabel::Sas);
        assert_eq!(metadata.file_encoding, ReadStatEncoding::Latin1);
        assert_eq!(metadata.is_64bit, false);
    }

    #[test]
    fn test_readstat_metadata_builder() {
        let metadata = ReadStatMetadata::builder()
            .row_count(100)
            .var_count(10)
            .created_at(Instant::now())
            .modified_at(Instant::now())
            .file_format_version(0)
            .compression(ReadStatCompression::None)
            .endianness(ReadStatEndianness::Little)
            .table_name(SasTableName::from_str("table").expect("Need a table name"))
            .file_label(ReadStatFileLabel::Sas)
            .file_encoding(ReadStatEncoding::Latin1)
            .is_64bit(false)
            .build()
            .expect("Failed to build metadata");

        assert_eq!(metadata.row_count, 100);
        assert_eq!(metadata.var_count, 10);
        assert_eq!(metadata.file_format_version, 0);
        assert_eq!(metadata.compression, ReadStatCompression::None);
        assert_eq!(metadata.endianness, ReadStatEndianness::Little);
        assert_eq!(
            metadata.table_name,
            SasTableName::from_str("table").expect("Need a table name")
        );
        assert_eq!(metadata.file_label, ReadStatFileLabel::Sas);
        assert_eq!(metadata.file_encoding, ReadStatEncoding::Latin1);
        assert_eq!(metadata.is_64bit, false);
    }

    #[test]
    fn test_readstat_metadata_builder_with_defaults() {
        let metadata = ReadStatMetadata::builder()
            .build()
            .expect("Failed to build metadata");

        assert_eq!(metadata.row_count, 0);
        assert_eq!(metadata.var_count, 0);
        assert_eq!(metadata.file_format_version, 0);
        assert_eq!(metadata.compression, ReadStatCompression::None);
        assert_eq!(metadata.endianness, ReadStatEndianness::Little);
        assert_eq!(
            metadata.table_name,
            SasTableName::from_str("table").expect("Need a table name")
        );
        assert_eq!(metadata.file_label, ReadStatFileLabel::Sas);
        assert_eq!(metadata.file_encoding, ReadStatEncoding::Latin1);
        assert_eq!(metadata.is_64bit, false);
    }
}
