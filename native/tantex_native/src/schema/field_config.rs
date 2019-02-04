use super::super::tantex_error::TantexError;
use TantexError::{InvalidType, TypeCannotBeFast, TypeIsNotStored};

pub enum FieldConfig {
    I64 { stored: bool, fast: bool },
    U64 { stored: bool, fast: bool },
    Text { stored: bool },
    Str { stored: bool },
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
            ("string", _, _) => Ok(Str { stored: stored }),
            ("text", _, _) => Ok(Text { stored: stored }),
            ("bytes", true, _) => Err(TypeIsNotStored(kind_string)),
            ("bytes", false, false) => Ok(Bytes),
            ("facet", _, _) => Ok(Facet),
            _ => Err(InvalidType(kind_string)),
        }
    }
}
