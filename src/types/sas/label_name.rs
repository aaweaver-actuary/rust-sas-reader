use derive_builder::Builder;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct SasLabelName {
    pub name: String,
}

impl SasLabelName {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn from_str(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn builder() -> SasLabelNameBuilder {
        SasLabelNameBuilder::default()
    }
}
