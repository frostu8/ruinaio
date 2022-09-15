use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

/// A special type for patching values.
#[derive(Debug, Clone, PartialEq)]
pub enum Patch<T> {
    /// No data.
    None,
    /// The data should be unset.
    Null,
    /// The data should be changed to this.
    Some(T),
}

impl<T> Patch<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, Patch::None)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Patch::Null)
    }

    pub fn is_some(&self) -> bool {
        matches!(self, Patch::Some(_))
    }
}

impl<T> Default for Patch<T> {
    fn default() -> Self {
        Patch::None
    }
}

impl<T> From<Option<T>> for Patch<T> {
    fn from(opt: Option<T>) -> Patch<T> {
        match opt {
            Some(v) => Patch::Some(v),
            None => Patch::Null,
        }
    }
}

impl<T> Serialize for Patch<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Patch::None | Patch::Null => serializer.serialize_none(),
            Patch::Some(t) => serializer.serialize_some(t),
        }
    }
}

impl<'de, T> Deserialize<'de> for Patch<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

