use tantivy::collector::TopDocs;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::{Field, Schema};
use tantivy::{DocAddress, Index};

use super::tantex_error::TantexError;
use TantexError::{FieldNotFound, InvalidQuery};

pub fn fetch_schema_fields(
    schema: &Schema,
    field_strings: Vec<String>,
) -> Result<Vec<Field>, TantexError> {
    let mut fields: Vec<Field> = Vec::with_capacity(field_strings.len());
    for field_name in field_strings.iter() {
        let field = fetch_field(schema, field_name)?;
        fields.push(field);
    }
    Ok(fields)
}

pub fn fetch_field(schema: &Schema, field_name: &str) -> Result<Field, TantexError> {
    match schema.get_field(field_name) {
        Some(field) => Ok(field),
        None => {
            let e = FieldNotFound(field_name.to_string());
            Err(e)
        }
    }
}

pub fn parse_query(query_parser: &QueryParser, pattern: &str) -> Result<Box<Query>, TantexError> {
    match query_parser.parse_query(pattern) {
        Ok(q) => Ok(q),
        Err(_) => Err(InvalidQuery(pattern.to_string())),
    }
}

pub fn search_with_limit(
    index: &Index,
    query: &Query,
    limit: usize,
) -> Result<Vec<(f32, DocAddress)>, TantexError> {
    let collector = TopDocs::with_limit(limit);
    let searcher = index.searcher();
    match searcher.search(query, &collector) {
        Ok(found) => Ok(found),
        Err(e1) => {
            let e2 = TantexError::SearchExecutionFailed(query.box_clone(), e1);
            Err(e2)
        }
    }
}
