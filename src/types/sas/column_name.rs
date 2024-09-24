/// In SAS, column names are limited to 32 characters, and can only contain alphanumeric characters
const MAX_COLUMN_NAME_LENGTH: usize = 32;

use derive_builder::Builder;

/// Wrapper around a normal string type that represents a SAS column name, and validates that
/// it is a valid SAS column name
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SasColumnName {
    pub name: String,
}

impl SasColumnName {
    /// Create a new SAS column name
    pub fn from_str(name: &str) -> Result<Self, &'static str> {
        if name.len() > MAX_COLUMN_NAME_LENGTH {
            return Err("Column name is too long");
        }

        if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
            return Err("Column name contains illegal characters");
        }

        Ok(Self {
            name: name.to_string(),
        })
    }

    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn builder() -> SasColumnNameBuilder {
        SasColumnNameBuilder::default()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_column_name() {
        let column_name = SasColumnName::from_str("valid_column_name").unwrap();
        assert_eq!(column_name.name, "valid_column_name");
    }

    #[test]
    fn test_column_name_builder() {
        let column_name = SasColumnName::builder()
            .name("valid_column_name".to_string())
            .build()
            .unwrap();
        assert_eq!(column_name.name, "valid_column_name");
    }

    #[test]
    fn test_invalid_column_name_too_long() {
        let a_column_name = "this_column_name_is_too_long_really_tooooooo_long";

        // This column name is too long, so it should fail
        let column_name = SasColumnName::from_str(a_column_name);
        assert!(column_name.is_err());
    }

    #[test]
    fn test_invalid_column_name_illegal_characters() {
        let a_column_name = "this_column_name_has_illegal_characters!";

        // This column name has illegal characters, so it should fail
        let column_name = SasColumnName::from_str(a_column_name);
        assert!(column_name.is_err());
    }
}
