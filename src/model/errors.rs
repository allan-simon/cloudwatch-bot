use serde_json;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParseEnumError<V> {
    pub value: String,
    pub mapping: HashMap<&'static str, V>,
}

impl<V> Display for ParseEnumError<V> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let keys: Vec<&'static str> = self.mapping.iter().map(|(k, _)| (*k)).collect();
        write!(fmt, "'{}' does not match allowed enum values: {}", self.value, keys.join(" | "))
    }
}

pub(crate) type EnumResult<T> = Result<T, ParseEnumError<T>>;
pub(crate) type JsonResult<T> = Result<T, serde_json::Error>;
