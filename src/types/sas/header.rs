use derive_builder::Builder;

// typedef struct sas_header_start_s {
// unsigned char magic[32];
// unsigned char a2;
// unsigned char mystery1[2];
// unsigned char a1;
// unsigned char mystery2[1];
// unsigned char endian;
// unsigned char mystery3[1];
// char          file_format;
// unsigned char mystery4[30];
// unsigned char encoding;
// unsigned char mystery5[13];
// char          file_type[8];
// char          table_name[32];
// unsigned char mystery6[32];
// char          file_info[8];
// } sas_header_start_t;
#[derive(Debug, Clone, PartialEq, Builder)]
pub struct SasHeaderStart {
    pub magic: [u8; 32],
    pub a2: u8,
    pub mystery1: [u8; 2],
    pub a1: u8,
    pub mystery2: [u8; 1],
    pub endian: u8,
    pub mystery3: [u8; 1],
    pub file_format: char,
    pub mystery4: [u8; 30],
    pub encoding: u8,
    pub mystery5: [u8; 13],
    pub file_type: [char; 8],
    pub table_name: [char; 32],
    pub mystery6: [u8; 32],
    pub file_info: [char; 8],
}

impl SasHeaderStart {
    pub fn new(
        magic: [u8; 32],
        a2: u8,
        mystery1: [u8; 2],
        a1: u8,
        mystery2: [u8; 1],
        endian: u8,
        mystery3: [u8; 1],
        file_format: char,
        mystery4: [u8; 30],
        encoding: u8,
        mystery5: [u8; 13],
        file_type: [char; 8],
        table_name: [char; 32],
        mystery6: [u8; 32],
        file_info: [char; 8],
    ) -> Self {
        Self {
            magic,
            a2,
            mystery1,
            a1,
            mystery2,
            endian,
            mystery3,
            file_format,
            mystery4,
            encoding,
            mystery5,
            file_type,
            table_name,
            mystery6,
            file_info,
        }
    }

    pub fn builder() -> SasHeaderStartBuilder {
        SasHeaderStartBuilder::default()
    }
}

impl SasHeaderStartBuilder {
    pub fn default() -> Self {
        Self {
            magic: Some([0; 32]),
            a2: Some(0),
            mystery1: Some([0; 2]),
            a1: Some(0),
            mystery2: Some([0; 1]),
            endian: Some(0),
            mystery3: Some([0; 1]),
            file_format: Some(' '),
            mystery4: Some([0; 30]),
            encoding: Some(0),
            mystery5: Some([0; 13]),
            file_type: Some([' '; 8]),
            table_name: Some([' '; 32]),
            mystery6: Some([0; 32]),
            file_info: Some([' '; 8]),
        }
    }
}

// typedef struct sas_header_end_s {
// char          release[8];
// char          host[16];
// char          version[16];
// char          os_vendor[16];
// char          os_name[16];
// char          extra[48];
// } sas_header_end_t;
#[derive(Debug, Clone, PartialEq, Builder)]
pub struct SasHeaderEnd {
    pub release: [char; 8],
    pub host: [char; 16],
    pub version: [char; 16],
    pub os_vendor: [char; 16],
    pub os_name: [char; 16],
    pub extra: [char; 48],
}

impl SasHeaderEnd {
    pub fn new(
        release: [char; 8],
        host: [char; 16],
        version: [char; 16],
        os_vendor: [char; 16],
        os_name: [char; 16],
        extra: [char; 48],
    ) -> Self {
        Self {
            release,
            host,
            version,
            os_vendor,
            os_name,
            extra,
        }
    }

    pub fn builder() -> SasHeaderEndBuilder {
        SasHeaderEndBuilder::default()
    }
}

impl SasHeaderEndBuilder {
    pub fn default() -> Self {
        Self {
            release: Some([' '; 8]),
            host: Some([' '; 16]),
            version: Some([' '; 16]),
            os_vendor: Some([' '; 16]),
            os_name: Some([' '; 16]),
            extra: Some([' '; 48]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct SasHeader {
    pub start: SasHeaderStart,
    pub end: SasHeaderEnd,
}

impl SasHeader {
    pub fn new(start: SasHeaderStart, end: SasHeaderEnd) -> Self {
        Self { start, end }
    }

    pub fn builder() -> SasHeaderBuilder {
        SasHeaderBuilder::default()
    }
}

impl SasHeaderBuilder {
    pub fn default() -> Self {
        Self {
            start: Some(SasHeaderStart::builder().build().unwrap()),
            end: Some(SasHeaderEnd::builder().build().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sas_header_start() {
        let header_start = SasHeaderStart::builder().build().unwrap();
        assert_eq!(header_start.magic, [0; 32]);
    }

    #[test]
    fn test_sas_header_start_new() {
        let header_start = SasHeaderStart::new(
            [0; 32], 0, [0; 2], 0, [0; 1], 0, [0; 1], ' ', [0; 30], 0, [0; 13], [' '; 8],
            [' '; 32], [0; 32], [' '; 8],
        );
        assert_eq!(header_start.magic, [0; 32]);
    }

    #[test]
    fn test_sas_header_end() {
        let header_end = SasHeaderEnd::builder().build().unwrap();
        assert_eq!(header_end.release, [' '; 8]);
    }

    #[test]
    fn test_sas_header_end_new() {
        let header_end = SasHeaderEnd::new(
            [' '; 8], [' '; 16], [' '; 16], [' '; 16], [' '; 16], [' '; 48],
        );
        assert_eq!(header_end.release, [' '; 8]);
    }

    #[test]
    fn test_sas_header() {
        let header = SasHeader::builder().build().unwrap();
        assert_eq!(header.start.magic, [0; 32]);
    }

    #[test]
    fn test_sas_header_new() {
        let header = SasHeader::new(
            SasHeaderStart::builder().build().unwrap(),
            SasHeaderEnd::builder().build().unwrap(),
        );
        assert_eq!(header.start.magic, [0; 32]);
    }
}
