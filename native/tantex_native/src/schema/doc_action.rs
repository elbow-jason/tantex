use super::super::tantex_error::TantexError;
use super::super::utils::{get_type_and_field, term_to_i64, term_to_string, term_to_u64};
use super::token_type::TokenType;
use erl_term::Term;
use tantivy::schema::{Field, Schema};

pub enum DocAction {
    AddText(Field, String),
    AddBytes(Field, Vec<u8>),
    AddI64(Field, i64),
    AddU64(Field, u64),
}

impl DocAction {
    pub fn build(
        schema: &Schema,
        erl_key: Term,
        erl_value: Term,
    ) -> Result<DocAction, TantexError> {
        let key: String = term_to_string(erl_key)?;
        let (field_type, field) = get_type_and_field(schema, &key)?;
        let token_typed = TokenType::from_field_type(&field_type);
        let action: DocAction = match (token_typed, erl_value) {
            (TokenType::Text, term) => {
                let text = term_to_string(term)?;
                DocAction::AddText(field, text)
            }
            (TokenType::Str, term) => {
                let text = term_to_string(term)?;
                DocAction::AddText(field, text)
            }
            (TokenType::Bytes, Term::CharList(bytes)) => DocAction::AddBytes(field, bytes),
            (TokenType::I64, term) => {
                let int = term_to_i64(term)?;
                DocAction::AddI64(field, int)
            }
            (TokenType::U64, term) => {
                let int: u64 = term_to_u64(term)?;
                DocAction::AddU64(field, int)
            }
            (token_type, term) => {
                let message = format!("token_type: {:?} - term: {:?}", token_type, term);
                let e = TantexError::UnhandledDocActionCombo(message);
                return Err(e);
            }
        };
        Ok(action)
    }
}
