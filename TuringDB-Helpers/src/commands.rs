use serde::{Deserialize, Serialize};
use tai64::TAI64N;
/// Commands to perform on the repo and its contents by the repo owner known as `SuperUser`
#[derive(Debug, Eq, PartialEq)]
pub enum TuringOp {
    /// Initialize the Repository
    RepoCreate,
    /// Delete the Repository
    RepoDrop,
    /// Create a database
    DbCreate,
    /// List all databases in a repo
    DbList,
    /// Delete a database
    DbDrop,
    /// Create a document
    DocumentCreate,
    /// List All Documents
    DocumentList,
    /// Delete a document and all its contents
    DocumentDrop,
    ///Insert a field into a document
    FieldInsert,
    /// Read contents particular document
    FieldGet,
    /// Remove a particular document
    FieldRemove,
    /// Updata a document
    FieldModify,
    /// List all fields in a document
    FieldList,
    /// The command is not supported
    NotSupported,
}
/// Converts a database operation to a header using the `TuringOp` enum
pub fn from_op<'op>(value: &TuringOp) -> &'op [u8] {
    match *value {
        TuringOp::RepoCreate => &[0x00],
        TuringOp::RepoDrop => &[0x01],
        TuringOp::DbCreate => &[0x02],
        TuringOp::DbList => &[0x03],
        TuringOp::DbDrop => &[0x04],
        TuringOp::DocumentCreate => &[0x05],
        TuringOp::DocumentList => &[0x06],
        TuringOp::DocumentDrop => &[0x07],
        TuringOp::FieldInsert => &[0x08],
        TuringOp::FieldGet => &[0x09],
        TuringOp::FieldRemove => &[0x0a],
        TuringOp::FieldModify => &[0x0b],
        TuringOp::FieldList => &[0x0c],
        TuringOp::NotSupported => &[0xf1],
    }
}
/// Converts a database operation from a header to `TuringOp` enum variant
pub fn to_op(value: &[u8]) -> TuringOp {
    match *value {
        [0x00] => TuringOp::RepoCreate,
        [0x01] => TuringOp::RepoDrop,
        [0x02] => TuringOp::DbCreate,
        [0x03] => TuringOp::DbList,
        [0x04] => TuringOp::DbDrop,
        [0x05] => TuringOp::DocumentCreate,
        [0x06] => TuringOp::DocumentList,
        [0x07] => TuringOp::DocumentDrop,
        [0x08] => TuringOp::FieldInsert,
        [0x09] => TuringOp::FieldGet,
        [0x0a] => TuringOp::FieldRemove,
        [0x0b] => TuringOp::FieldModify,
        [0x0c] => TuringOp::FieldList,
        [0xf1] => TuringOp::NotSupported,
        _ => TuringOp::NotSupported,
    }
}
/// Contains the structure of a value represented by a key
///
/// `Warning:` This is serialized using bincode so deserialization should be done using same version of bincode
/// ```
/// #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// pub struct FieldData {
///     data: Vec<u8>,
///     created: TAI64N,
///     modified: TAI64N,
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FieldData {
    data: Vec<u8>,
    created: TAI64N,
    modified: TAI64N,
}

impl FieldData {
    /// Initializes a new `FieldData` struct
    pub fn new(value: &[u8]) -> FieldData {
        let current_time = TAI64N::now();

        Self {
            data: value.into(),
            created: current_time,
            modified: current_time,
        }
    }
    /// Updates a `FieldData` by modifying its time with a new `TAI64N` timestamp
    pub fn update(&mut self, value: &[u8]) -> &FieldData {
        self.data = value.into();
        self.modified = TAI64N::now();

        self
    }
}
