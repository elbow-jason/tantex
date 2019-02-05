// rustler
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;

extern crate rustler_codegen;
extern crate tantivy;

use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use tantivy::schema::{Field, Type};

mod atoms;
mod schema;
mod tantex_error;
mod utils;
mod wrapper;

use schema::field_config::FieldConfig;
use schema::schema_index::SchemaIndex;
use tantex_error::TantexError;
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
        ("write_documents", 3, write_documents),
        ("limit_search", 4, limit_search),
        ("find_one_by_term", 3, find_one_by_term),
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

fn find_one_by_term<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let schema_index_wrapper: ResourceArc<Wrapper<SchemaIndex>> = args[0].decode()?;
    let schema_index = schema_index_wrapper.lock.read().unwrap();
    let field_name: String = args[1].decode()?;
    let field: Field = match schema_index.fetch_field(&field_name) {
        Err(e) => return render_error(env, e),
        Ok(field) => field,
    };
    let term: tantivy::Term = match schema_index.fetch_field_type(&field_name) {
        Ok(Type::I64) => {
            let val: i64 = match args[2].decode() {
                Ok(v) => v,
                Err(_) => return bad_data(env, Type::I64, field_name),
            };
            tantivy::Term::from_field_i64(field, val)
        }
        Ok(Type::U64) => {
            let val: u64 = match args[2].decode() {
                Ok(v) => v,
                Err(_) => return bad_data(env, Type::U64, field_name),
            };
            tantivy::Term::from_field_u64(field, val)
        }
        Ok(Type::Str) => {
            let val: String = match args[2].decode() {
                Ok(v) => v,
                Err(_) => return bad_data(env, Type::Str, field_name),
            };
            tantivy::Term::from_field_text(field, &val)
        }
        Ok(t) => {
            let e = TantexError::TypeCannotBeSearched(t);
            return render_error(env, e);
        }
        Err(e) => {
            return render_error(env, e);
        }
    };
    match schema_index.fetch_one_by_term(term) {
        Ok(json_doc) => Ok((atoms::ok(), json_doc).encode(env)),
        Err(e) => return render_error(env, e),
    }
}

fn render_error<'a>(env: Env<'a>, e: TantexError) -> NifResult<Term<'a>> {
    Ok((atoms::error(), e.to_reason()).encode(env))
}

fn bad_data<'a>(env: Env<'a>, t: Type, field_name: String) -> NifResult<Term<'a>> {
    let e = TantexError::InvalidFieldData(t, field_name);
    render_error(env, e)
}
