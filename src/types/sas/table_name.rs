const MAX_TABLE_NAME_LENGTH: usize = 32;

use derive_builder::Builder;

/// Wrapper around a normal string type that represents a SAS table name, and validates that it
/// is a valid SAS table name
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SasTableName {
    pub name: String,
}

impl SasTableName {
    /// Create a new SAS table name
    pub fn from_str(name: &str) -> Result<Self, &'static str> {
        if name.len() > MAX_TABLE_NAME_LENGTH {
            return Err("Table name is too long");
        }

        if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
            return Err("Table name contains illegal characters");
        }

        Ok(Self {
            name: name.to_string(),
        })
    }

    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn builder() -> SasTableNameBuilder {
        SasTableNameBuilder::default()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_table_name() {
        let table_name = SasTableName::from_str("valid_table_name").unwrap();
        assert_eq!(table_name.name, "valid_table_name");
    }

    #[test]
    fn test_table_name_builder() {
        let table_name = SasTableName::builder()
            .name("valid_table_name".to_string())
            .build()
            .unwrap();
        assert_eq!(table_name.name, "valid_table_name");
    }

    #[test]
    fn test_invalid_table_name_too_long() {
        let a_table_name = "this_table_name_is_too_long_really_tooooooo_long";

        // This table name is too long, so it should fail
        let table_name = SasTableName::from_str(a_table_name);
        assert!(table_name.is_err());
    }

    #[test]
    fn test_invalid_table_name_illegal_characters() {
        let a_table_name = "this_table_name_has_illegal_characters!";

        // This table name has illegal characters, so it should fail
        let table_name = SasTableName::from_str(a_table_name);
        assert!(table_name.is_err());
    }
}
