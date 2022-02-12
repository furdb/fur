use super::Converter;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FurDataType {
    name: String,
    converter: Converter,
}

impl FurDataType {
    pub fn new(name: &str, converter: Converter) -> FurDataType {
        let name = String::from(name);
        FurDataType { name, converter }
    }

    pub fn get_converter(&self) -> Converter {
        self.converter.clone()
    }
}
