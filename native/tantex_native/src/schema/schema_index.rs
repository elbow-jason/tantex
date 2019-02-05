use tantivy::query::{QueryParser, TermQuery};
use tantivy::schema::{Field, IndexRecordOption, Schema, SchemaBuilder};
use tantivy::{Index, IndexWriter};

use super::super::tantex_error::TantexError;
use super::super::utils::{fetch_field, fetch_schema_fields, parse_query, search_with_limit};
use super::field_config::{
    build_int_options, build_string_options, build_text_options, build_trigram_options, FieldConfig,
};
use super::index::open_or_create_index;
use FieldConfig::{Bytes, Facet, Str, Text, Trigram, I64, U64};

pub struct SchemaIndex {
    builder: Option<SchemaBuilder>,
    index: Option<Index>,
    schema: Option<Schema>,
    index_path: Option<String>,
}

impl SchemaIndex {
    pub fn new() -> SchemaIndex {
        SchemaIndex {
            builder: Some(SchemaBuilder::new()),
            index: None,
            schema: None,
            index_path: None,
        }
    }

    pub fn add_field(&mut self, name: &str, field_config: FieldConfig) -> Result<(), TantexError> {
        if name == "" {
            return Err(TantexError::NameCannotBeBlank);
        }
        if let Some(ref mut schema_builder) = self.builder {
            add_field_config(schema_builder, name, field_config);
            Ok(())
        } else {
            Err(TantexError::SchemaBuilderNotFound)
        }
    }

    pub fn finalize_schema(&mut self) -> Result<(), TantexError> {
        let schema_builder = self.fetch_schema_builder()?;
        self.schema = Some(schema_builder.build());
        Ok(())
    }

    fn fetch_schema_builder(&mut self) -> Result<SchemaBuilder, TantexError> {
        match self.builder {
            Some(ref mut schema_builder_ref) => {
                let mut schema_builder = SchemaBuilder::new();
                std::mem::swap(schema_builder_ref, &mut schema_builder);
                self.builder = None;
                Ok(schema_builder)
            }
            None => Err(TantexError::SchemaBuilderNotFound),
        }
    }

    pub fn open_index(&mut self, index_path: &str) -> Result<(), TantexError> {
        let schema = self.fetch_schema()?;
        let index = open_or_create_index(index_path, schema.clone())?;
        self.index = Some(index);
        self.index_path = Some(index_path.to_string());
        Ok(())
    }

    pub fn limit_search(
        &self,
        field_strings: Vec<String>,
        pattern: &str,
        limit: usize,
    ) -> Result<Vec<String>, TantexError> {
        let schema = self.fetch_schema()?;
        let index = self.fetch_index()?;
        let fields: Vec<Field> = fetch_schema_fields(&schema, field_strings)?;
        let query_parser = QueryParser::for_index(&index, fields);
        let query = parse_query(&query_parser, &pattern)?;
        let docs = search_with_limit(index, &query, limit)?;
        let mut json_docs: Vec<String> = Vec::with_capacity(docs.len());
        let searcher = index.searcher();
        for (_score, doc_address) in docs.iter() {
            match searcher.doc(*doc_address) {
                Ok(retrieved_doc) => json_docs.push(schema.to_json(&retrieved_doc)),
                Err(e1) => {
                    let e2 = TantexError::DocumentRetrievalFailed(e1);
                    return Err(e2);
                }
            }
        }
        Ok(json_docs)
    }

    pub fn fetch_one_by_text(&self, field_name: &str, text: &str) -> Result<String, TantexError> {
        let schema = self.fetch_schema()?;
        let index = self.fetch_index()?;
        let field = fetch_field(&schema, &field_name)?;
        let term = tantivy::schema::Term::from_field_text(field, &text);
        let searcher = index.searcher();
        let term_query = TermQuery::new(term, IndexRecordOption::Basic);
        let found = search_with_limit(index, &term_query, 1)?;
        if let Some((_score, doc_address)) = found.first() {
            match searcher.doc(*doc_address) {
                Ok(doc) => Ok(schema.to_json(&doc)),
                Err(e) => Err(TantexError::DocumentRetrievalFailed(e)),
            }
        } else {
            Err(TantexError::DocumentNotFound)
        }
    }

    fn fetch_index_writer(&self, heap_size: usize) -> Result<IndexWriter, TantexError> {
        let index = self.fetch_index()?;
        new_index_writer(&index, heap_size)
    }

    fn fetch_index(&self) -> Result<&Index, TantexError> {
        match self.index {
            Some(ref index) => Ok(index),
            None => Err(TantexError::IndexNotFound),
        }
    }

    fn fetch_schema(&self) -> Result<&Schema, TantexError> {
        match self.schema {
            Some(ref schema) => Ok(schema),
            None => Err(TantexError::SchemaNotFound),
        }
    }

    pub fn write_documents(
        &self,
        json_docs: Vec<String>,
        heap_size: usize,
    ) -> Result<u64, TantexError> {
        let index = self.fetch_index()?;
        let mut index_writer = self.fetch_index_writer(heap_size)?;
        let schema = self.fetch_schema()?;
        for json_item in json_docs {
            match schema.parse_document(&json_item) {
                Ok(doc) => {
                    let _ = index_writer.add_document(doc);
                }
                Err(e1) => {
                    let json_str = json_item.to_string();
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
}

fn new_index_writer(index: &Index, heap_size: usize) -> Result<IndexWriter, TantexError> {
    match index.writer(heap_size) {
        Ok(writer) => Ok(writer),
        Err(reason) => {
            let e = TantexError::FailedToCreateIndexWriter(reason);
            Err(e)
        }
    }
}

fn add_field_config(schema_builder: &mut SchemaBuilder, name: &str, field_config: FieldConfig) {
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
    };
}
