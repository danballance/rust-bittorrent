use crate::bencode::character_processor::CharacterProcessor;
use crate::bencode::types::Context;

pub struct Decoder;

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode(&self, encoded_value: &str) -> Result<serde_json::Value, String> {
        if encoded_value.is_empty() {
            return Err("Empty encoded value".to_string());
        }
        let mut context = Context::new();
        let processor = CharacterProcessor::new();

        for c in encoded_value.chars() {
            context = processor.next_character(c, context)?;
        }
        Ok(context.value)
    }
}
