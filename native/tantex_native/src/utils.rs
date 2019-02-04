use erl_term::{Sign, Term};
use tantivy::query::{Query, QueryParser};
use tantivy::schema::{Field, FieldType, Schema};

use super::tantex_error::TantexError;
use TantexError::{
    DocumentMustBeMap, Exceeded8Bytes, FieldNotFound, InvalidQuery, InvalidTermFormat, InvalidUTF8,
    TermCannotBeI64, TermCannotBeString, TermCannotBeU64,
};

pub fn term_to_string(term: Term) -> Result<String, TantexError> {
    let bytes: Vec<u8> = match term {
        Term::Binary(k) => k,
        Term::Atom(k) => k,
        _ => return Err(TermCannotBeString(term)),
    };
    bytes_to_utf8(&bytes)
}

pub fn bytes_to_utf8(bytes: &[u8]) -> Result<String, TantexError> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(key) => Ok(key),
        Err(_) => Err(InvalidUTF8(bytes.to_vec())),
    }
}

pub fn term_to_i64(term: Term) -> Result<i64, TantexError> {
    match term {
        Term::Integer(int) => Ok(int as i64),
        Term::SmallInt(int) => Ok(int as i64),
        Term::SmallBigInt(sign, bytes) => {
            let mut array = vec_to_64bit_array(&bytes)?;
            match sign {
                Sign::Pos => (),
                Sign::Neg => array[0] += 128_u8,
            };
            let int: i64 = i64::from_be_bytes(array);
            Ok(int)
        }
        t => Err(TermCannotBeI64(t)),
    }
}

pub fn term_to_u64(term: Term) -> Result<u64, TantexError> {
    match term {
        Term::Integer(int) => Ok(int as u64),
        Term::SmallInt(int) => Ok(int as u64),
        Term::SmallBigInt(sign, bytes) => {
            match sign {
                Sign::Pos => (),
                Sign::Neg => {
                    let term_copy = Term::SmallBigInt(sign, bytes);
                    let e = TermCannotBeU64(term_copy);
                    return Err(e);
                }
            };
            let array = vec_to_64bit_array(&bytes)?;
            let int: u64 = u64::from_be_bytes(array);
            Ok(int)
        }
        t => Err(TermCannotBeI64(t)),
    }
}

fn vec_to_64bit_array(bytes: &[u8]) -> Result<[u8; 8], TantexError> {
    let len = bytes.len();
    if len > 8 {
        return Err(Exceeded8Bytes(bytes.to_vec()));
    }
    let mut array: [u8; 8] = [0; 8];
    for (index, b) in bytes.iter().enumerate() {
        array[8 - len + index] = *b;
    }

    Ok(array)
}

pub fn bytes_to_term(bytes: &[u8]) -> Result<Term, TantexError> {
    match Term::from_bytes(bytes) {
        Ok(term) => Ok(term),
        Err(_) => {
            let message = format!("invalid term format {:?}", bytes).to_string();
            Err(InvalidTermFormat(message))
        }
    }
}

pub fn ensure_document_is_map(term: Term) -> Result<Term, TantexError> {
    match term {
        Term::Map(_) => Ok(term),
        _ => Err(DocumentMustBeMap(term)),
    }
}

pub fn bytes_to_document_map(bytes: &[u8]) -> Result<Term, TantexError> {
    let term = bytes_to_term(bytes)?;
    ensure_document_is_map(term)
}

pub fn fetch_schema_fields(
    schema: &Schema,
    field_strings: Vec<String>,
) -> Result<Vec<Field>, TantexError> {
    let mut fields: Vec<Field> = Vec::with_capacity(field_strings.len());
    for field_name in field_strings.iter() {
        let field = get_field(schema, field_name)?;
        fields.push(field);
    }
    Ok(fields)
}

pub fn get_type_and_field<'a>(
    schema: &'a Schema,
    field_name: &str,
) -> Result<(&'a FieldType, Field), TantexError> {
    let field: Field = get_field(schema, field_name)?;
    let entry = schema.get_field_entry(field);
    Ok((entry.field_type(), field))
}

pub fn get_field(schema: &Schema, field_name: &str) -> Result<Field, TantexError> {
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
