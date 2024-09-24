// #define SAS_SUBHEADER_SIGNATURE_ROW_SIZE       0xF7F7F7F7
// #define SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE    0xF6F6F6F6
// #define SAS_SUBHEADER_SIGNATURE_COUNTS         0xFFFFFC00
// #define SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT  0xFFFFFBFE

// #define SAS_SUBHEADER_SIGNATURE_COLUMN_MASK    0xFFFFFFF8
// /* Seen in the wild: FA (unknown), F8 (locale?) */
// #define SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS   0xFFFFFFFC
// #define SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT    0xFFFFFFFD
// #define SAS_SUBHEADER_SIGNATURE_COLUMN_LIST    0xFFFFFFFE
// #define SAS_SUBHEADER_SIGNATURE_COLUMN_NAME    0xFFFFFFFF

use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct SasSubheaderSignature {
    pub row_size: u32,
    pub column_size: u32,
    pub counts: u32,
    pub column_format: u32,
    pub column_mask: u32,
    pub column_attrs: u32,
    pub column_text: u32,
    pub column_list: u32,
    pub column_name: u32,
}

impl SasSubheaderSignature {
    pub fn new(
        row_size: u32,
        column_size: u32,
        counts: u32,
        column_format: u32,
        column_mask: u32,
        column_attrs: u32,
        column_text: u32,
        column_list: u32,
        column_name: u32,
    ) -> Self {
        Self {
            row_size,
            column_size,
            counts,
            column_format,
            column_mask,
            column_attrs,
            column_text,
            column_list,
            column_name,
        }
    }

    pub fn default() -> Self {
        Self {
            row_size: 0xF7F7F7F7,
            column_size: 0xF6F6F6F6,
            counts: 0xFFFFFC00,
            column_format: 0xFFFFFBFE,
            column_mask: 0xFFFFFFF8,
            column_attrs: 0xFFFFFFFC,
            column_text: 0xFFFFFFFD,
            column_list: 0xFFFFFFFE,
            column_name: 0xFFFFFFFF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stat_subheader_signature() {
        let subheader_signature = SasSubheaderSignature::default();
        assert_eq!(subheader_signature.row_size, 0xF7F7F7F7);
    }

    #[test]
    fn test_read_stat_subheader_signature_new() {
        let subheader_signature = SasSubheaderSignature::new(
            0xF7F7F7F7, 0xF6F6F6F6, 0xFFFFFC00, 0xFFFFFBFE, 0xFFFFFFF8, 0xFFFFFFFC, 0xFFFFFFFD,
            0xFFFFFFFE, 0xFFFFFFFF,
        );
        assert_eq!(subheader_signature.row_size, 0xF7F7F7F7);
    }
}
