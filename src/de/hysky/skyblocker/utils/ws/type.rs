//! Ported from de.hysky.skyblocker.utils.ws.Type by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

//   java: public static final Codec<Type> CODEC = StringRepresentable.fromValues(Type::values);
/// java: public static final Codec<Type> CODEC = StringRepresentable.fromValues(Type::values);
pub static CODEC: compat::MainThread<CodecRef<Type>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_values(Type::values(), |r#type: &Type| r#type.get_serialized_name()));

/// java: public enum Type implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Subscribe,
    InitialMessage,
    Publish,
    Response,
    Unsubscribe,
}

impl Type {
    pub const VALUES: [Type; 5] = [Type::Subscribe, Type::InitialMessage, Type::Publish, Type::Response, Type::Unsubscribe];

    /// java: values()
    pub fn values() -> Vec<Type> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Type::Subscribe => "SUBSCRIBE",
            Type::InitialMessage => "INITIAL_MESSAGE",
            Type::Publish => "PUBLISH",
            Type::Response => "RESPONSE",
            Type::Unsubscribe => "UNSUBSCRIBE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Type {
        match name {
            "SUBSCRIBE" => Type::Subscribe,
            "INITIAL_MESSAGE" => Type::InitialMessage,
            "PUBLISH" => Type::Publish,
            "RESPONSE" => Type::Response,
            "UNSUBSCRIBE" => Type::Unsubscribe,
            _ => panic!("IllegalArgumentException: No enum constant Type.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static TypeFields {
        static FIELDS: LazyLock<Vec<TypeFields>> = LazyLock::new(|| vec![
            TypeFields::new("subscribe".to_string()),
            TypeFields::new("initialMessage".to_string()),
            TypeFields::new("publish".to_string()),
            TypeFields::new("response".to_string()),
            TypeFields::new("unsubscribe".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().id.clone();
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Type constant
pub struct TypeFields {
    pub id: String,
}

impl TypeFields {
    fn new(mut id: String) -> Self {
        let mut this = Self {
            id: Default::default(),
        };
        this.id = id.clone();
        this
    }

}
