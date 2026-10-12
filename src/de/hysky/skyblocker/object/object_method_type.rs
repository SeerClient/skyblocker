//! Ported from de.hysky.skyblocker.object.ObjectMethodType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum ObjectMethodType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectMethodType {
    Equals,
    HashCode,
    ToString,
}

impl ObjectMethodType {
    pub const VALUES: [ObjectMethodType; 3] = [ObjectMethodType::Equals, ObjectMethodType::HashCode, ObjectMethodType::ToString];

    /// java: values()
    pub fn values() -> Vec<ObjectMethodType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ObjectMethodType::Equals => "EQUALS",
            ObjectMethodType::HashCode => "HASH_CODE",
            ObjectMethodType::ToString => "TO_STRING",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ObjectMethodType {
        match name {
            "EQUALS" => ObjectMethodType::Equals,
            "HASH_CODE" => ObjectMethodType::HashCode,
            "TO_STRING" => ObjectMethodType::ToString,
            _ => panic!("IllegalArgumentException: No enum constant ObjectMethodType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static ObjectMethodTypeFields {
        static FIELDS: LazyLock<Vec<ObjectMethodTypeFields>> = LazyLock::new(|| vec![
            ObjectMethodTypeFields::new("equals".to_string()),
            ObjectMethodTypeFields::new("hashCode".to_string()),
            ObjectMethodTypeFields::new("toString".to_string()),
        ]);
        &FIELDS[self as usize]
    }

}

impl std::fmt::Display for ObjectMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each ObjectMethodType constant
pub struct ObjectMethodTypeFields {
    pub method_name: String,
}

impl ObjectMethodTypeFields {
    fn new(mut method_name: String) -> Self {
        let mut this = Self {
            method_name: Default::default(),
        };
        this.method_name = method_name.clone();
        this
    }

}
