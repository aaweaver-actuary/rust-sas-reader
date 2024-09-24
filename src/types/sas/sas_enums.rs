// #define SAS_ENDIAN_BIG       0x00
// #define SAS_ENDIAN_LITTLE    0x01
#[derive(Debug, PartialEq, Clone)]
pub enum SasEndian {
    Big = 0x00,
    Little = 0x01,
}

impl Default for SasEndian {
    fn default() -> Self {
        SasEndian::Big
    }
}

impl SasEndian {
    pub fn to_hex(&self) -> u8 {
        match self {
            SasEndian::Big => 0x00,
            SasEndian::Little => 0x01,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SasFileFormat {
    Unix = 1,
    Windows = 2,
}

// #define SAS_ALIGNMENT_OFFSET_0  0x22
// #define SAS_ALIGNMENT_OFFSET_4  0x33
#[derive(Debug, PartialEq, Clone)]
pub enum SasAlignmentOffset {
    Offset0 = 0x22,
    Offset4 = 0x33,
}

// #define SAS_COLUMN_TYPE_NUM  0x01
// #define SAS_COLUMN_TYPE_CHR  0x02
#[derive(Debug, PartialEq, Clone)]
pub enum SasColumnType {
    Numeric = 0x01,
    Character = 0x02,
}

// #define SAS_PAGE_TYPE_META   0x0000
// #define SAS_PAGE_TYPE_DATA   0x0100
// #define SAS_PAGE_TYPE_MIX    0x0200
// #define SAS_PAGE_TYPE_AMD    0x0400
// #define SAS_PAGE_TYPE_MASK   0x0F00

// #define SAS_PAGE_TYPE_META2  0x4000
// #define SAS_PAGE_TYPE_COMP   0x9000
#[derive(Debug, PartialEq, Clone)]
pub enum SasPageType {
    Meta = 0x0000,
    Data = 0x0100,
    Mix = 0x0200,
    Amd = 0x0400,
    Meta2 = 0x4000,
    Comp = 0x9000,
}

// #define SAS_SUBHEADER_POINTER_SIZE_32BIT    12
// #define SAS_SUBHEADER_POINTER_SIZE_64BIT    24
#[derive(Debug, PartialEq, Clone)]
pub enum SasSubheaderPointerSize {
    Bit32 = 12,
    Bit64 = 24,
}

// #define SAS_PAGE_HEADER_SIZE_32BIT  24
// #define SAS_PAGE_HEADER_SIZE_64BIT  40
#[derive(Debug, PartialEq, Clone)]
pub enum SasPageHeaderSize {
    Bit32 = 24,
    Bit64 = 40,
}

// #define SAS_COMPRESSION_NONE   0x00
// #define SAS_COMPRESSION_TRUNC  0x01
// #define SAS_COMPRESSION_ROW    0x04

// #define SAS_COMPRESSION_SIGNATURE_RLE  "SASYZCRL"
// #define SAS_COMPRESSION_SIGNATURE_RDC  "SASYZCR2"
#[derive(Debug, PartialEq, Clone)]
pub enum SasCompression {
    None = 0x00,
    Trunc = 0x01,
    Row = 0x04,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SasCompressionSignature {
    Rle,
    Rdc,
}

impl SasCompressionSignature {
    pub fn to_string(&self) -> String {
        match self {
            SasCompressionSignature::Rle => "SASYZCRL".to_string(),
            SasCompressionSignature::Rdc => "SASYZCR2".to_string(),
        }
    }
}

// #define SAS_FILE_HEADER_SIZE_32BIT 1024
// #define SAS_FILE_HEADER_SIZE_64BIT 8192
// #define SAS_DEFAULT_PAGE_SIZE      4096
#[derive(Debug, PartialEq, Clone)]
pub enum SasFileHeaderSize {
    Bit32 = 1024,
    Bit64 = 8192,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SasPageSize {
    Default = 4096,
}

// #define SAS_DEFAULT_STRING_ENCODING "WINDOWS-1252"
#[derive(Debug, PartialEq, Clone)]
pub enum SasStringEncoding {
    Windows1252,
}

impl SasStringEncoding {
    pub fn to_string(&self) -> String {
        match self {
            SasStringEncoding::Windows1252 => "WINDOWS-1252".to_string(),
        }
    }

    pub fn default() -> Self {
        SasStringEncoding::Windows1252
    }
}

// unsigned char sas7bdat_magic_number[32] = {
// 0x00, 0x00, 0x00, 0x00,   0x00, 0x00, 0x00, 0x00,
// 0x00, 0x00, 0x00, 0x00,   0xc2, 0xea, 0x81, 0x60,
// 0xb3, 0x14, 0x11, 0xcf,   0xbd, 0x92, 0x08, 0x00,
// 0x09, 0xc7, 0x31, 0x8c,   0x18, 0x1f, 0x10, 0x11
// };

// unsigned char sas7bcat_magic_number[32] = {
// 0x00, 0x00, 0x00, 0x00,   0x00, 0x00, 0x00, 0x00,
// 0x00, 0x00, 0x00, 0x00,   0xc2, 0xea, 0x81, 0x63,
// 0xb3, 0x14, 0x11, 0xcf,   0xbd, 0x92, 0x08, 0x00,
// 0x09, 0xc7, 0x31, 0x8c,   0x18, 0x1f, 0x10, 0x11
// };
#[derive(Debug, PartialEq, Clone)]
pub enum SasFileMagicNumber {
    Sas7bdat,
    Sas7bcat,
}

impl SasFileMagicNumber {
    pub fn to_string(&self) -> String {
        match self {
            SasFileMagicNumber::Sas7bdat => "sas7bdat".to_string(),
            SasFileMagicNumber::Sas7bcat => "sas7bcat".to_string(),
        }
    }

    pub fn get_array(&self) -> [u8; 32] {
        match self {
            SasFileMagicNumber::Sas7bdat => [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea,
                0x81, 0x60, 0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c,
                0x18, 0x1f, 0x10, 0x11,
            ],
            SasFileMagicNumber::Sas7bcat => [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea,
                0x81, 0x63, 0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c,
                0x18, 0x1f, 0x10, 0x11,
            ],
        }
    }
}
