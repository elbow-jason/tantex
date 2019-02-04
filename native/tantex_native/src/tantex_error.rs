use super::atoms;
use erl_term::Term;
use rustler::types::atom::Atom;
use tantivy::TantivyError;

pub enum TantexError {
    TypeCannotBeFast(String),
    InvalidType(String),
    TypeIsNotStored(String),
    NameCannotBeBlank,
    PathDoesNotExist(String),
    FailedToCreateIndex(String, TantivyError),
    FailedToCreateIndexWriter(TantivyError),
    FailedToWriteToIndex(String),
    FailedToLoadSearchers(String),
    TermCannotBeString(Term),
    InvalidUTF8(Vec<u8>),
    Exceeded8Bytes(Vec<u8>),
    FieldNotFound(String),
    TermCannotBeI64(Term),
    TermCannotBeU64(Term),
    UnhandledDocActionCombo(String), // IndexInitFailed
    DocumentMustBeMap(Term),
    InvalidTermFormat(String),
    InvalidQuery(String),
    SearchExecutionFailed(String, TantivyError),
    DocumentRetrievalFailed(TantivyError),
}

use TantexError::*;

impl TantexError {
    pub fn to_reason(&self) -> (Atom, String) {
        match self {
            TypeCannotBeFast(t) => (atoms::cannot_be_fast(), t.to_string()),
            InvalidType(t) => (atoms::invalid_type(), t.to_string()),
            TypeIsNotStored(t) => (atoms::cannot_be_stored(), t.to_string()),
            NameCannotBeBlank => (atoms::name_cannot_be_blank(), "".to_string()),
            PathDoesNotExist(path) => (atoms::path_does_not_exist(), path.to_string()),
            FailedToWriteToIndex(path) => {
                let message = format!("path: {:?}", path);
                (atoms::failed_to_write_to_index(), message)
            }
            InvalidUTF8(bytes) => {
                let message = format!("invalid bytes: {:?}", bytes);
                (atoms::invalid_utf8(), message)
            }
            Exceeded8Bytes(bytes) => {
                let message = format!("u8 slice was more than 8 bytes long: {:?}", bytes);
                (atoms::exceeded_8_bytes(), message)
            }
            FieldNotFound(field_name) => (atoms::field_not_found(), field_name.to_string()),
            TermCannotBeI64(term) => {
                let message = format!("erl_term cannot be i64 - {:?}", term);
                (atoms::term_cannot_be_i64(), message)
            }
            TermCannotBeU64(term) => {
                let message = format!("erl_term cannot be u64 - {:?}", term);
                (atoms::term_cannot_be_u64(), message)
            }
            TermCannotBeString(term) => {
                let message = format!("erl_term cannot be string - {:?}", term);
                (atoms::term_cannot_be_string(), message)
            }
            UnhandledDocActionCombo(message) => (atoms::doc_action_error(), message.to_string()),
            FailedToCreateIndex(path, e) => {
                let message = format!("path: {:?} - reason: {:?}", path, e);
                (atoms::failed_to_create_index(), message)
            }
            FailedToCreateIndexWriter(tantivy_error) => {
                let message = format!("reason: {:?}", tantivy_error);
                (atoms::failed_to_create_index_writer(), message)
            }
            DocumentMustBeMap(term) => {
                let message = format!("wrong term: {:?}", term);
                (atoms::document_must_be_map(), message)
            }
            FailedToLoadSearchers(message) => {
                (atoms::failed_to_load_searchers(), message.to_string())
            }
            InvalidTermFormat(message) => {
                (atoms::invalid_term_format(), message.to_string())
            }
            InvalidQuery(query) => {
                (atoms::invalid_term_format(), query.to_string())
            }
            SearchExecutionFailed(pattern, tantivy_error) => {
                let message = format!("query: {:?} - error: {:?}", pattern, tantivy_error);
                (atoms::search_execution_failed(), message)
            }
            DocumentRetrievalFailed(tantivy_error) => {
                let message = format!("error: {:?}", tantivy_error);
                (atoms::document_retrieval_failed(), message)
            }
            // IndexInitFailed => (atoms::index_init_failed(), "".to_string()),
        }
    }
}
