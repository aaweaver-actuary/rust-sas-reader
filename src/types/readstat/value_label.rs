use derive_builder::Builder;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct ReadStatValueLabel {
    pub label: String,
    pub label_length: u32,
    pub double_key: f64,
    pub int32_key: i32,
    pub string_key: String,
    pub string_key_length: u32,
    pub tags: Vec<String>,
}

impl ReadStatValueLabel {
    pub fn new(
        label: String,
        label_length: u32,
        double_key: f64,
        int32_key: i32,
        string_key: String,
        string_key_length: u32,
        tags: Vec<String>,
    ) -> Self {
        Self {
            label,
            label_length,
            double_key,
            int32_key,
            string_key,
            string_key_length,
            tags,
        }
    }

    pub fn builder() -> ReadStatValueLabelBuilder {
        ReadStatValueLabelBuilder::default()
    }
}

impl ReadStatValueLabelBuilder {
    pub fn default() -> Self {
        Self {
            label: Some(String::new()),
            label_length: Some(0),
            double_key: Some(0.0),
            int32_key: Some(0),
            string_key: Some(String::new()),
            string_key_length: Some(0),
            tags: Some(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stat_value_label() {
        let value_label = ReadStatValueLabel::builder().build().unwrap();
        assert_eq!(value_label.label, String::new());
        assert_eq!(value_label.label_length, 0);
        assert_eq!(value_label.double_key, 0.0);
        assert_eq!(value_label.int32_key, 0);
        assert_eq!(value_label.string_key, String::new());
        assert_eq!(value_label.string_key_length, 0);
        assert_eq!(value_label.tags, Vec::<String>::new());
    }
}
