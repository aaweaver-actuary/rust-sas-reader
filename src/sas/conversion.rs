#[derive(Debug, Clone, PartialEq)]
pub enum ConversionType {
    Native,
    Xport,
    IeeeBigEndian,
    IeeeLittleEndian,
}

impl Default for ConversionType {
    fn default() -> Self {
        ConversionType::Native
    }
}

impl ConversionType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(ConversionType::Native),
            1 => Some(ConversionType::Xport),
            2 => Some(ConversionType::IeeeBigEndian),
            3 => Some(ConversionType::IeeeLittleEndian),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            ConversionType::Native => 0,
            ConversionType::Xport => 1,
            ConversionType::IeeeBigEndian => 2,
            ConversionType::IeeeLittleEndian => 3,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ConversionType::Native => "native".to_string(),
            ConversionType::Xport => "xport".to_string(),
            ConversionType::IeeeBigEndian => "ieee-big-endian".to_string(),
            ConversionType::IeeeLittleEndian => "ieee-little-endian".to_string(),
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "native" => Some(ConversionType::Native),
            "xport" => Some(ConversionType::Xport),
            "ieee-big-endian" => Some(ConversionType::IeeeBigEndian),
            "ieee-little-endian" => Some(ConversionType::IeeeLittleEndian),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_type() {
        let conversion_type = ConversionType::from_i32(0).unwrap();
        assert_eq!(conversion_type, ConversionType::Native);
        assert_eq!(conversion_type.to_i32(), 0);
        assert_eq!(conversion_type.to_string(), "native");
        assert_eq!(
            ConversionType::from_str("native").unwrap(),
            ConversionType::Native
        );
    }
}
