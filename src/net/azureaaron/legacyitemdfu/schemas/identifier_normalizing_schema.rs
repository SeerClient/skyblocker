//! Ported from net.azureaaron.legacyitemdfu.schemas.IdentifierNormalizingSchema (legacy-item-dfu 1.0.4+26.2).

use crate::net::azureaaron::legacyitemdfu::utils::identifier_utils::IdentifierUtils;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::serialization::Obj;
use mcrust::com::mojang::serialization::codecs::primitive_codec::PrimitiveCodec;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use std::sync::{Arc, LazyLock};

/// java: public static final PrimitiveCodec<String> CODEC = new PrimitiveCodec<String>() { .. } - its `toString()` is
/// "NamespacedString".
pub static CODEC: LazyLock<PrimitiveCodec<String>> =
    LazyLock::new(|| PrimitiveCodec::new("NamespacedString", IdentifierNormalizingSchema::read, IdentifierNormalizingSchema::write));

/// java: public final class IdentifierNormalizingSchema extends Schema
///
/// java: `private static final Type<String> IDENTIFIER_TYPE = new PrimitiveType<>(CODEC)` and its getter
/// `getIdentifierType()` have no Rust form: `Const.PrimitiveType` is one of the type kinds the game's DFU does not
/// build (`mcrust::com::mojang::datafixers::types::templates::const`'s header), and the game crate is read-only from
/// the mod. Their readers - the schemas' `ITEM_NAME` and the fixes' id finders - read the id through `CODEC` instead
/// (`schemas/item_stack_only_schema.rs`).
pub struct IdentifierNormalizingSchema;

impl IdentifierNormalizingSchema {
    /// java: public IdentifierNormalizingSchema(int versionKey, Schema parent) - `super(versionKey, parent)`.
    ///
    /// java: its one override, `getChoiceType(type, choiceName)` = `super.getChoiceType(type, normalize(choiceName))`,
    /// has no slot to fill: the game's `Schema` has no `getChoiceType` (schema.rs: the choice types are not ported).
    /// Nothing in the library constructs this schema; it is used for its statics.
    pub fn new(version_key: i32, parent: Option<Arc<Schema>>) -> Schema {
        Schema::new(version_key, parent)
    }

    /// java: CODEC's `public <T> DataResult<String> read(DynamicOps<T> ops, T input)` -
    /// `ops.getStringValue(input).map(IdentifierNormalizingSchema::normalize)`.
    fn read(ops: &Ops, input: &Obj) -> DataResult<String> {
        ops.get_string_value(input).map(|id| IdentifierNormalizingSchema::normalize(&id))
    }

    /// java: CODEC's `public <T> T write(DynamicOps<T> dynamicOps, String string)` - `dynamicOps.createString(string)`.
    fn write(ops: &Ops, string: &String) -> Obj {
        ops.create_string(string)
    }

    /// java: public static String normalize(String id)
    pub fn normalize(id: &str) -> String {
        let identifier = IdentifierUtils::try_parse(id);
        match identifier {
            Some(identifier) => identifier,
            None => id.to_string(),
        }
    }
}
