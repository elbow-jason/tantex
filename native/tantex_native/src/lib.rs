// rustler
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;

extern crate rustler_codegen;
extern crate tantivy;

use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, SchemaBuilder};
use tantivy::{Index, IndexWriter};

use std::sync::RwLock;

mod atoms;
mod query;
mod schema;
mod tantex_error;
mod utils;

use schema::field_config::FieldConfig;
use schema::index::{index_into_writer, open_or_create_index};

use tantex_error::TantexError;
use utils::{fetch_schema_fields, parse_query};

struct Wrapper<T> {
    lock: RwLock<T>,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Wrapper<T> {
        Wrapper {
            lock: RwLock::new(value),
        }
    }
}

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(Wrapper<Schema>, env);
    resource_struct_init!(Wrapper<Index>, env);
    resource_struct_init!(Wrapper<IndexWriter>, env);
    true
}

rustler_export_nifs! {
    "Elixir.Tantex.Native",
    [
        ("build_schema", 1, build_schema),
        ("schema_into_index", 2, schema_into_index),
        ("write_documents", 4, write_documents),
        ("limit_search", 5, limit_search),
    ],
    Some(on_load)
}

fn build_schema<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let fields: Vec<(&str, &str, bool, bool)> = args[0].decode()?;
    let mut schema_builder = SchemaBuilder::new();
    for (name, kind, stored, fast) in fields.iter() {
        match add_to_schema(&mut schema_builder, name, kind, *stored, *fast) {
            Err(e) => return Ok((atoms::error(), e.to_reason()).encode(env)),
            _ => (),
        }
    }
    let schema: Schema = schema_builder.build();
    let wrapper: Wrapper<Schema> = Wrapper::new(schema);
    let resource: ResourceArc<Wrapper<Schema>> = ResourceArc::new(wrapper);
    Ok((atoms::ok(), resource).encode(env))
}

fn limit_search<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_wrapper: ResourceArc<Wrapper<Schema>> = args[0].decode()?;
    let index_wrapper: ResourceArc<Wrapper<Index>> = args[1].decode()?;
    let field_strings: Vec<String> = args[2].decode()?;
    let pattern: String = args[3].decode()?;
    let limit: usize = args[4].decode()?;
    let schema = schema_wrapper.lock.read().unwrap();
    let index = index_wrapper.lock.read().unwrap();
    match execute_search(&schema, &index, field_strings, pattern, limit) {
        Ok(docs) => Ok((atoms::ok(), docs).encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}

fn execute_search(
    schema: &Schema,
    index: &Index,
    field_strings: Vec<String>,
    pattern: String,
    limit: usize,
) -> Result<Vec<String>, TantexError> {
    let fields: Vec<Field> = fetch_schema_fields(&schema, field_strings)?;

    let query_parser = QueryParser::for_index(&index, fields);
    let query = parse_query(&query_parser, &pattern)?;
    let collector = TopDocs::with_limit(limit);
    let searcher = index.searcher();
    let docs = match searcher.search(&query, &collector) {
        Ok(found) => found,
        Err(e1) => {
            let e2 = TantexError::SearchExecutionFailed(pattern.to_string(), e1);
            return Err(e2);
        }
    };
    let mut json_docs: Vec<String> = Vec::with_capacity(docs.len());
    for (_score, doc_address) in docs {
        match searcher.doc(doc_address) {
            Ok(retrieved_doc) => json_docs.push(schema.to_json(&retrieved_doc)),
            Err(e1) => {
                let e2 = TantexError::DocumentRetrievalFailed(e1);
                return Err(e2);
            }
        }
    }
    Ok(json_docs)
}

fn schema_into_index<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_wrapper: ResourceArc<Wrapper<Schema>> = args[0].decode()?;
    let index_path: &str = args[1].decode()?;
    let schema = schema_wrapper.lock.read().unwrap();
    let c_schema = schema.clone();
    let index = match open_or_create_index(index_path, c_schema) {
        Ok(index) => index,
        Err(tantex_err) => {
            let status = atoms::error();
            let reason = tantex_err.to_reason();
            return Ok((status, reason).encode(env));
        }
    };
    let wrapper: Wrapper<Index> = Wrapper::new(index);
    let resource: ResourceArc<Wrapper<Index>> = ResourceArc::new(wrapper);
    Ok((atoms::ok(), resource).encode(env))
}

fn write_documents<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_wrapper: ResourceArc<Wrapper<Schema>> = args[0].decode()?;
    let index_wrapper: ResourceArc<Wrapper<Index>> = args[1].decode()?;
    let json_docs: Vec<String> = args[2].decode()?;
    let heap_size_in_bytes: usize = args[3].decode()?;
    let schema = schema_wrapper.lock.read().unwrap();
    let index = index_wrapper.lock.read().unwrap();
    match write_docs_to_writer(&schema, &index, json_docs, heap_size_in_bytes) {
        Ok(last) => Ok((atoms::ok(), last).encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}

fn write_docs_to_writer(
    schema: &Schema,
    index: &Index,
    json_maps: Vec<String>,
    heap_size: usize,
) -> Result<u64, TantexError> {
    let mut index_writer = index_into_writer(index, heap_size)?;
    for doc_json in json_maps {
        match schema.parse_document(&doc_json) {
            Ok(doc) => {
                let _ = index_writer.add_document(doc);
            }
            Err(e1) => {
                let json_str = doc_json.to_string();
                let e2 = TantexError::InvalidDocumentJSON(json_str, e1);
                return Err(e2);
            }
        }
    }
    let last: u64 = match index_writer.commit() {
        Ok(last) => last,
        Err(e1) => {
            let message = format!("index: {:?} - reason: {:?}", index, e1);
            let e2 = TantexError::FailedToWriteToIndex(message);
            return Err(e2);
        }
    };
    if let Err(e1) = index.load_searchers() {
        let message = format!("index: {:?} - reason: {:?}", index, e1);
        let e2 = TantexError::FailedToLoadSearchers(message);
        return Err(e2);
    };
    Ok(last)
}

fn add_to_schema(
    builder: &mut SchemaBuilder,
    name: &str,
    kind: &str,
    stored: bool,
    fast: bool,
) -> Result<(), TantexError> {
    if name == "" {
        return Err(TantexError::NameCannotBeBlank);
    }
    let config_result = FieldConfig::build(kind, stored, fast);
    match config_result {
        Ok(config) => {
            add_field_config_to_schema_builder(builder, name, config);
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn add_field_config_to_schema_builder(
    schema_builder: &mut SchemaBuilder,
    name: &str,
    field_config: FieldConfig,
) {
    use schema::field_config::*;
    use FieldConfig::*;
    match field_config {
        U64 { stored, fast } => {
            let int_options = build_int_options(stored, fast);
            schema_builder.add_u64_field(name, int_options);
        }
        I64 { stored, fast } => {
            let int_options = build_int_options(stored, fast);
            schema_builder.add_i64_field(name, int_options);
        }
        Str { stored } => {
            let string_options = build_string_options(stored);
            schema_builder.add_text_field(name, string_options);
        }
        Text { stored } => {
            let text_options = build_text_options(stored);
            schema_builder.add_text_field(name, text_options);
        }
        Trigram { stored } => {
            let text_options = build_trigram_options(stored);
            schema_builder.add_text_field(name, text_options);
        }
        Facet => {
            schema_builder.add_facet_field(name);
        }
        Bytes => {
            schema_builder.add_bytes_field(name);
        }
    }
}
