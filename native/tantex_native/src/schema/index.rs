use std::path::PathBuf;
use tantivy::schema::Schema;
use tantivy::{Index, TantivyError};

use super::super::tantex_error::TantexError;

use TantexError::{FailedToCreateIndex, PathDoesNotExist};

pub fn open_or_create_index(path: &str, schema: Schema) -> Result<Index, TantexError> {
    let path_buf = PathBuf::from(path);
    match Index::open_in_dir(&path_buf) {
        Ok(index) => return Ok(index),
        Err(TantivyError::PathDoesNotExist(_)) => create_index_at_path(path, schema),
        Err(reason) => Err(FailedToCreateIndex(path.to_string(), reason)),
    }
}

fn create_index_at_path(path: &str, schema: Schema) -> Result<Index, TantexError> {
    match Index::create_in_dir(path, schema) {
        Ok(index) => Ok(index),
        Err(TantivyError::PathDoesNotExist(_)) => Err(PathDoesNotExist(path.to_string())),
        Err(reason) => Err(FailedToCreateIndex(path.to_string(), reason)),
    }
}
