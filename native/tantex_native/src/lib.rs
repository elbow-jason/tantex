// rustler
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;

extern crate rustler_codegen;
extern crate tantivy;

use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};

mod atoms;
mod query;
mod schema;
mod tantex_error;
mod utils;
mod wrapper;

use schema::field_config::FieldConfig;
use schema::schema_index::SchemaIndex;
use wrapper::Wrapper;

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(Wrapper<SchemaIndex>, env);
    true
}

rustler_export_nifs! {
    "Elixir.Tantex.Native",
    [
        ("new_schema_index", 0, new_schema_index),
        ("add_field", 5, add_field),
        ("finalize_schema", 1, finalize_schema),
        ("open_index", 2, open_index),
        ("write_documents", 4, write_documents),
        ("limit_search", 4, limit_search),
        ("find_one_by_text", 3, find_one_by_text),
    ],
    Some(on_load)
}

fn new_schema_index<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index = SchemaIndex::new();
    let wrapper: Wrapper<SchemaIndex> = Wrapper::new(schema_index);
    let resource: ResourceArc<Wrapper<SchemaIndex>> = ResourceArc::new(wrapper);
    Ok((atoms::ok(), resource).encode(env))
}

fn add_field<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index_wrapper: ResourceArc<Wrapper<SchemaIndex>> = args[0].decode()?;
    let field_name: String = args[1].decode()?;
    let kind: String = args[2].decode()?;
    let stored: bool = args[3].decode()?;
    let fast: bool = args[4].decode()?;
    let mut schema_index = schema_index_wrapper.lock.write().unwrap();
    let field_config = match FieldConfig::build(&kind, stored, fast) {
        Ok(f) => f,
        Err(e) => return Ok((atoms::error(), e.to_reason()).encode(env)),
    };
    match schema_index.add_field(&field_name, field_config) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}

fn finalize_schema<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index_wrapper: ResourceArc<Wrapper<SchemaIndex>> = args[0].decode()?;
    let mut schema_index = schema_index_wrapper.lock.write().unwrap();
    match schema_index.finalize_schema() {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}

fn open_index<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index_wrapper: ResourceArc<Wrapper<SchemaIndex>> = args[0].decode()?;
    let index_path: String = args[1].decode()?;
    let mut schema_index = schema_index_wrapper.lock.write().unwrap();

    match schema_index.open_index(&index_path) {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}

fn limit_search<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index_wrapper: ResourceArc<Wrapper<SchemaIndex>> = args[0].decode()?;
    let field_strings: Vec<String> = args[1].decode()?;
    let pattern: String = args[2].decode()?;
    let limit: usize = args[3].decode()?;
    let schema_index = schema_index_wrapper.lock.read().unwrap();
    // let index = index_wrapper.lock.read().unwrap();
    match schema_index.limit_search(field_strings, &pattern, limit) {
        Ok(docs) => Ok((atoms::ok(), docs).encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}

fn write_documents<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index_wrapper: ResourceArc<Wrapper<SchemaIndex>> = args[0].decode()?;
    let schema_index = schema_index_wrapper.lock.read().unwrap();
    let json_docs: Vec<String> = args[1].decode()?;
    let heap_size: usize = args[2].decode()?;

    match schema_index.write_documents(json_docs, heap_size) {
        Ok(last) => Ok((atoms::ok(), last).encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}

fn find_one_by_text<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index_wrapper: ResourceArc<Wrapper<SchemaIndex>> = args[0].decode()?;

    let field_name: String = args[1].decode()?;
    let text: String = args[2].decode()?;
    let schema_index = schema_index_wrapper.lock.read().unwrap();
    match schema_index.fetch_one_by_text(&field_name, &text) {
        Ok(json_doc) => Ok((atoms::ok(), json_doc).encode(env)),
        Err(e) => Ok((atoms::error(), e.to_reason()).encode(env)),
    }
}
