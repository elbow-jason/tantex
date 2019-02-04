use tantivy::schema::{Cardinality, INT_INDEXED, STRING, TEXT};
use tantivy::schema::{IndexRecordOption, IntOptions, TextFieldIndexing, TextOptions};

use super::super::tantex_error::TantexError;
use TantexError::{InvalidType, TypeCannotBeFast, TypeIsNotStored};

pub enum FieldConfig {
    I64 { stored: bool, fast: bool },
    U64 { stored: bool, fast: bool },
    Text { stored: bool },
    Str { stored: bool },
    Trigram { stored: bool },
    Facet,
    Bytes,
}

use FieldConfig::*;

impl FieldConfig {
    pub fn build(kind: &str, stored: bool, fast: bool) -> Result<FieldConfig, TantexError> {
        let kind_string: String = kind.to_string();
        match (kind, stored, fast) {
            ("i64", _, _) => Ok(I64 {
                stored: stored,
                fast: fast,
            }),
            ("u64", _, _) => Ok(U64 {
                stored: stored,
                fast: fast,
            }),
            (_, _, true) => Err(TypeCannotBeFast(kind_string)),
            ("string", _, _) => Ok(Str { stored }),
            ("text", _, _) => Ok(Text { stored }),
            ("bytes", true, _) => Err(TypeIsNotStored(kind_string)),
            ("bytes", false, false) => Ok(Bytes),
            ("facet", _, _) => Ok(Facet),
            ("trigram", _, _) => Ok(Trigram { stored }),
            _ => Err(InvalidType(kind_string)),
        }
    }
}

fn trigram_indexing() -> TextFieldIndexing {
    TextFieldIndexing::default()
        .set_tokenizer("trigram")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions)
}
fn trigram() -> TextOptions {
    TextOptions::default().set_indexing_options(trigram_indexing())
}

pub fn build_trigram_options(stored: bool) -> TextOptions {
    set_text_stored(trigram(), stored)
}

fn set_text_stored(options: TextOptions, stored: bool) -> TextOptions {
    if stored {
        options.set_stored()
    } else {
        options
    }
}

fn set_int_fast(options: IntOptions, fast: bool) -> IntOptions {
    if fast {
        options.set_fast(Cardinality::SingleValue)
    } else {
        options
    }
}

fn set_int_stored(options: IntOptions, stored: bool) -> IntOptions {
    if stored {
        options.set_stored()
    } else {
        options
    }
}

pub fn build_int_options(stored: bool, fast: bool) -> IntOptions {
    set_int_fast(set_int_stored(INT_INDEXED, stored), fast)
}

pub fn build_text_options(stored: bool) -> TextOptions {
    set_text_stored(TEXT, stored)
}

pub fn build_string_options(stored: bool) -> TextOptions {
    set_text_stored(STRING, stored)
}
