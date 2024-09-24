// typedef struct readstat_charset_entry_s {
// int     code;
// char    name[32];
// } readstat_charset_entry_t;

use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct ReadStatCharsetEntry {
    pub code: i32,
    pub name: String,
}

impl ReadStatCharsetEntry {
    pub fn new(code: i32, name: String) -> Self {
        Self { code, name }
    }

    pub fn builder() -> ReadStatCharsetEntryBuilder {
        ReadStatCharsetEntryBuilder::default()
    }
}

impl ReadStatCharsetEntryBuilder {
    pub fn default() -> Self {
        Self {
            code: Some(0),
            name: Some(String::new()),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_read_stat_charset_entry() {
        let charset_entry = ReadStatCharsetEntry::builder().build().unwrap();
        assert_eq!(charset_entry.code, 0);
    }

    #[test]
    fn test_read_stat_charset_entry_new() {
        let charset_entry = ReadStatCharsetEntry::new(0, String::new());
        assert_eq!(charset_entry.code, 0);
    }
}
