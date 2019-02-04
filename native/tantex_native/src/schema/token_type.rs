use tantivy::schema::{FieldType, TextOptions};

#[derive(Debug)]
pub enum TokenType {
    Text,
    Str,
    Bytes,
    U64,
    I64,
    Facet,
}

impl TokenType {
    pub fn from_text_options(opts: &TextOptions) -> TokenType {
        if let Some(indexing) = opts.get_indexing_options() {
            match indexing.tokenizer() {
                "default" => TokenType::Text,
                "raw" => TokenType::Str,
                got => panic!(
                    "FieldType::Str can only be \"default\" or \"raw\" - got: {:?}",
                    got
                ),
            }
        } else {
            TokenType::Text
        }
    }

    pub fn from_field_type(ft: &FieldType) -> TokenType {
        use FieldType::*;
        match ft {
            Bytes => TokenType::Bytes,
            HierarchicalFacet => TokenType::Facet,
            U64(_opts) => TokenType::U64,
            I64(_opts) => TokenType::I64,
            Str(opts) => TokenType::from_text_options(opts),
        }
    }
}
