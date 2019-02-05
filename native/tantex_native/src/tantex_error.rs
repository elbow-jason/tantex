use super::atoms;
use rustler::types::atom::Atom;
use tantivy::query::Query;
use tantivy::schema::DocParsingError;
use tantivy::schema::Type;
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
    FieldNotFound(String),
    DocumentNotFound,
    InvalidQuery(String),
    SearchExecutionFailed(Box<Query>, TantivyError),
    DocumentRetrievalFailed(TantivyError),
    InvalidDocumentJSON(String, DocParsingError),
    SchemaBuilderNotFound,
    SchemaNotFound,
    IndexNotFound,
    TypeCannotBeSearched(Type),
    InvalidFieldData(Type, String),
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
            FieldNotFound(field_name) => (atoms::field_not_found(), field_name.to_string()),
            FailedToCreateIndex(path, e) => {
                let message = format!("path: {:?} - reason: {:?}", path, e);
                (atoms::failed_to_create_index(), message)
            }
            FailedToCreateIndexWriter(tantivy_error) => {
                let message = format!("reason: {:?}", tantivy_error);
                (atoms::failed_to_create_index_writer(), message)
            }
            FailedToLoadSearchers(message) => {
                (atoms::failed_to_load_searchers(), message.to_string())
            }
            InvalidQuery(query) => (atoms::invalid_query_format(), query.to_string()),
            SearchExecutionFailed(query, tantivy_error) => {
                let message = format!("query: {:?} - error: {:?}", query, tantivy_error);
                (atoms::search_execution_failed(), message)
            }
            DocumentRetrievalFailed(tantivy_error) => {
                let message = format!("error: {:?}", tantivy_error);
                (atoms::document_retrieval_failed(), message)
            }
            InvalidDocumentJSON(json, tantivy_error) => {
                let message = format!("json: {:?} - error: {:?}", json, tantivy_error);
                (atoms::invalid_document_json(), message)
            }
            DocumentNotFound => (atoms::document_not_found(), "".to_string()),
            SchemaBuilderNotFound => (atoms::schema_builder_not_found(), "".to_string()),
            SchemaNotFound => (atoms::schema_not_found(), "".to_string()),
            IndexNotFound => (atoms::index_not_found(), "".to_string()),
            TypeCannotBeSearched(t) => {
                let message = format!("type: {:?}", t);
                (atoms::invalid_document_json(), message)
            }
            InvalidFieldData(tantivy_type, field_name) => {
                let message = format!("type: {:?} - field_name: {:?}", tantivy_type, field_name);
                (atoms::invalid_field_data(), message)
            }
        }
    }
}
