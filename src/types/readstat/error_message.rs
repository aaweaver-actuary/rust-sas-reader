use crate::types::readstat::ReadStatError;

pub fn readstat_error_message(err: ReadStatError, error_code: i32) -> String {
    match err {
        ReadStatError::Open => format!("Error opening file: {}", error_code),
        ReadStatError::Read => format!("Error reading file: {}", error_code),
        ReadStatError::Malloc => format!("Error allocating memory: {}", error_code),
        ReadStatError::UserAbort => format!("User aborted: {}", error_code),
        ReadStatError::Parse => format!("Error parsing file: {}", error_code),
        ReadStatError::UnsupportedCompression => {
            format!("Unsupported compression: {}", error_code)
        }
        ReadStatError::UnsupportedCharset => format!("Unsupported charset: {}", error_code),
        ReadStatError::ColumnCountMismatch => format!("Column count mismatch: {}", error_code),
        ReadStatError::RowCountMismatch => format!("Row count mismatch: {}", error_code),
        ReadStatError::RowWidthMismatch => format!("Row width mismatch: {}", error_code),
        ReadStatError::BadFormatString => format!("Bad format string: {}", error_code),
        ReadStatError::ValueTypeMismatch => format!("Value type mismatch: {}", error_code),
        ReadStatError::Write => format!("Error writing file: {}", error_code),
        ReadStatError::WriterNotInitialized => {
            format!("Writer not initialized: {}", error_code)
        }
        ReadStatError::Seek => format!("Error seeking file: {}", error_code),
        ReadStatError::Convert => format!("Error converting value: {}", error_code),
        ReadStatError::ConvertBadString => {
            format!("Error converting bad string: {}", error_code)
        }
        ReadStatError::ConvertShortString => {
            format!("Error converting short string: {}", error_code)
        }
        ReadStatError::ConvertLongString => {
            format!("Error converting long string: {}", error_code)
        }
        ReadStatError::NumericValueOutOfRange => {
            format!("Numeric value out of range: {}", error_code)
        }
        ReadStatError::TaggedValueOutOfRange => {
            format!("Tagged value out of range: {}", error_code)
        }
        ReadStatError::StringValueTooLong => format!("String value too long: {}", error_code),
        ReadStatError::TaggedValuesNotSupported => {
            format!("Tagged values not supported: {}", error_code)
        }
        ReadStatError::UnsupportedFileFormatVersion => {
            format!("Unsupported file format version: {}", error_code)
        }
        ReadStatError::NameBeginsWithIllegalCharacter => {
            format!("Name begins with illegal character: {}", error_code)
        }
        ReadStatError::NameContainsIllegalCharacter => {
            format!("Name contains illegal character: {}", error_code)
        }
        ReadStatError::NameIsReservedKeyword => {
            format!("Name is reserved keyword: {}", error_code)
        }
        ReadStatError::NameIsTooLong => format!("Name is too long: {}", error_code),
        ReadStatError::BadTimestampString => format!("Bad timestamp string: {}", error_code),
        ReadStatError::BadFrequencyWeight => format!("Bad frequency weight: {}", error_code),
        ReadStatError::TooManyMissingValueDefinitions => {
            format!("Too many missing value definitions: {}", error_code)
        }
        ReadStatError::NoteIsTooLong => format!("Note is too long: {}", error_code),
        ReadStatError::StringRefsNotSupported => {
            format!("String refs not supported: {}", error_code)
        }
        ReadStatError::StringRefIsRequired => format!("String ref is required: {}", error_code),
        ReadStatError::RowIsTooWideForPage => {
            format!("Row is too wide for page: {}", error_code)
        }
        ReadStatError::TooFewColumns => format!("Too few columns: {}", error_code),
        ReadStatError::TooManyColumns => format!("Too many columns: {}", error_code),
        ReadStatError::NameIsZeroLength => format!("Name is zero length: {}", error_code),
        ReadStatError::BadTimestampValue => format!("Bad timestamp value: {}", error_code),
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_readstat_error_message() {
        assert_eq!(
            readstat_error_message(ReadStatError::Open, 1),
            "Error opening file: 1"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::Read, 2),
            "Error reading file: 2"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::Malloc, 3),
            "Error allocating memory: 3"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::UserAbort, 4),
            "User aborted: 4"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::Parse, 5),
            "Error parsing file: 5"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::UnsupportedCompression, 6),
            "Unsupported compression: 6"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::UnsupportedCharset, 7),
            "Unsupported charset: 7"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::ColumnCountMismatch, 8),
            "Column count mismatch: 8"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::RowCountMismatch, 9),
            "Row count mismatch: 9"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::RowWidthMismatch, 10),
            "Row width mismatch: 10"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::BadFormatString, 11),
            "Bad format string: 11"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::ValueTypeMismatch, 12),
            "Value type mismatch: 12"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::Write, 13),
            "Error writing file: 13"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::WriterNotInitialized, 14),
            "Writer not initialized: 14"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::Seek, 15),
            "Error seeking file: 15"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::Convert, 16),
            "Error converting value: 16"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::ConvertBadString, 17),
            "Error converting bad string: 17"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::ConvertShortString, 18),
            "Error converting short string: 18"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::ConvertLongString, 19),
            "Error converting long string: 19"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::NumericValueOutOfRange, 20),
            "Numeric value out of range: 20"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::TaggedValueOutOfRange, 21),
            "Tagged value out of range: 21"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::StringValueTooLong, 22),
            "String value too long: 22"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::TaggedValuesNotSupported, 23),
            "Tagged values not supported: 23"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::UnsupportedFileFormatVersion, 24),
            "Unsupported file format version: 24"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::NameBeginsWithIllegalCharacter, 25),
            "Name begins with illegal character: 25"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::NameContainsIllegalCharacter, 26),
            "Name contains illegal character: 26"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::NameIsReservedKeyword, 27),
            "Name is reserved keyword: 27"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::NameIsTooLong, 28),
            "Name is too long: 28"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::BadTimestampString, 29),
            "Bad timestamp string: 29"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::BadFrequencyWeight, 30),
            "Bad frequency weight: 30"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::TooManyMissingValueDefinitions, 31),
            "Too many missing value definitions: 31"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::NoteIsTooLong, 32),
            "Note is too long: 32"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::StringRefsNotSupported, 33),
            "String refs not supported: 33"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::StringRefIsRequired, 34),
            "String ref is required: 34"
        );

        assert_eq!(
            readstat_error_message(ReadStatError::RowIsTooWideForPage, 35),
            "Row is too wide for page: 35"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::TooFewColumns, 36),
            "Too few columns: 36"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::TooManyColumns, 37),
            "Too many columns: 37"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::NameIsZeroLength, 38),
            "Name is zero length: 38"
        );
        assert_eq!(
            readstat_error_message(ReadStatError::BadTimestampValue, 39),
            "Bad timestamp value: 39"
        );
    }
}
