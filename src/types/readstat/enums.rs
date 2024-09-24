#[derive(Debug, PartialEq)]
pub enum ReadStatHandler {
    Ok = 0,
    Abort = -1,
    SkipVariable = 1,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatType {
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Double,
    StringRef,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatEncoding {
    Utf8,
    Latin1,
    Windows1252,
    Ascii,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatFileLabel {
    Sas,
    Xport,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatTypeClass {
    String,
    Numeric,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatMeasure {
    Nominal,
    Ordinal,
    Scale,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatCompression {
    None,
    Rows,
    Binary,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatEndianness {
    Little,
    Big,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatError {
    Open,
    Read,
    Malloc,
    UserAbort,
    Parse,
    UnsupportedCompression,
    UnsupportedCharset,
    ColumnCountMismatch,
    RowCountMismatch,
    RowWidthMismatch,
    BadFormatString,
    ValueTypeMismatch,
    Write,
    WriterNotInitialized,
    Seek,
    Convert,
    ConvertBadString,
    ConvertShortString,
    ConvertLongString,
    NumericValueOutOfRange,
    TaggedValueOutOfRange,
    StringValueTooLong,
    TaggedValuesNotSupported,
    UnsupportedFileFormatVersion,
    NameBeginsWithIllegalCharacter,
    NameContainsIllegalCharacter,
    NameIsReservedKeyword,
    NameIsTooLong,
    BadTimestampString,
    BadFrequencyWeight,
    TooManyMissingValueDefinitions,
    NoteIsTooLong,
    StringRefsNotSupported,
    StringRefIsRequired,
    RowIsTooWideForPage,
    TooFewColumns,
    TooManyColumns,
    NameIsZeroLength,
    BadTimestampValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReadStatValueType {
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float(f32),
    Double(f64),
}

// typedef enum readstat_io_flags_e {
// READSTAT_SEEK_SET,
// READSTAT_SEEK_CUR,
// READSTAT_SEEK_END
// } readstat_io_flags_t;
#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatIoFlags {
    SeekSet,
    SeekCurrent,
    SeekEnd,
}

// enum {
// READSTAT_VENDOR_STAT_TRANSFER,
// READSTAT_VENDOR_SAS
// };
#[derive(Debug, PartialEq, Clone)]
pub enum ReadStatVendor {
    StatTransfer,
    Sas,
}
