//! Ported from de.hysky.skyblocker.skyblock.dwarven.CorpseType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::commands::arguments::string_representable_argument::StringRepresentableArgument;

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<CorpseType> CODEC = StringRepresentable.fromEnum(CorpseType::values);
/// java: public static final Codec<CorpseType> CODEC = StringRepresentable.fromEnum(CorpseType::values);
pub static CODEC: compat::MainThread<CodecRef<CorpseType>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public enum CorpseType implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CorpseType {
    Lapis,
    Umber,
    Tungsten,
    Vanguard,
    Unknown,
}

impl CorpseType {
    pub const VALUES: [CorpseType; 5] = [CorpseType::Lapis, CorpseType::Umber, CorpseType::Tungsten, CorpseType::Vanguard, CorpseType::Unknown];

    /// java: values()
    pub fn values() -> Vec<CorpseType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CorpseType::Lapis => "LAPIS",
            CorpseType::Umber => "UMBER",
            CorpseType::Tungsten => "TUNGSTEN",
            CorpseType::Vanguard => "VANGUARD",
            CorpseType::Unknown => "UNKNOWN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CorpseType {
        match name {
            "LAPIS" => CorpseType::Lapis,
            "UMBER" => CorpseType::Umber,
            "TUNGSTEN" => CorpseType::Tungsten,
            "VANGUARD" => CorpseType::Vanguard,
            "UNKNOWN" => CorpseType::Unknown,
            _ => panic!("IllegalArgumentException: No enum constant CorpseType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static CorpseTypeFields {
        static FIELDS: LazyLock<Vec<CorpseTypeFields>> = LazyLock::new(|| vec![
            CorpseTypeFields::new("LAPIS_ARMOR_HELMET".to_string(), None, ChatFormatting::Blue),
            CorpseTypeFields::new("ARMOR_OF_YOG_HELMET".to_string(), Some("UMBER_KEY".to_string()), ChatFormatting::Gold),
            CorpseTypeFields::new("MINERAL_HELMET".to_string(), Some("TUNGSTEN_KEY".to_string()), ChatFormatting::Gray),
            CorpseTypeFields::new("VANGUARD_HELMET".to_string(), Some("SKELETON_KEY".to_string()), ChatFormatting::Aqua),
            CorpseTypeFields::new("UNKNOWN".to_string(), None, ChatFormatting::Red),
        ]);
        &FIELDS[self as usize]
    }

    /// java: static CorpseType fromHelmetItemId(String helmetItemId)
    pub fn from_helmet_item_id(helmet_item_id: &str) -> CorpseType {
        // TODO(fabric2seer): call to `values(..)` (not a method of this class)
        //   java: for (CorpseType value : values()) {
        //   java: 			if (value.helmetItemId.equals(helmetItemId)) {
        //   java: 				return value;
        //   java: 			}
        //   java: 		}
        return UNKNOWN;
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public double getKeyPrice() throws IllegalStateException
    pub fn get_key_price(&self) -> f64 {
        return match *self {
            CorpseType::Unknown => {
                panic!("IllegalStateException: {}", "There's no key or key price for the UNKNOWN corpse type!");
            }
            CorpseType::Lapis => ((0) as f64) as f64,
            _ => ItemUtils::get_item_price_string(Some(self.fields().key_item_id.clone())).expect("Optional.orElseThrow"),
        };
    }

}

impl std::fmt::Display for CorpseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each CorpseType constant
pub struct CorpseTypeFields {
    pub helmet_item_id: String,
    pub key_item_id: Option<String>,
    pub color: ChatFormatting,
}

impl CorpseTypeFields {
    fn new(mut helmet_item_id: String, mut key_item_id: Option<String>, mut color: ChatFormatting) -> Self {
        let mut this = Self {
            helmet_item_id: Default::default(),
            key_item_id: None,
            color: Default::default(),
        };
        this.helmet_item_id = helmet_item_id.clone();
        this.key_item_id = key_item_id.clone().unwrap();
        this.color = color;
        this
    }

}

/// java: public static class CorpseTypeArgumentType extends StringRepresentableArgument<CorpseType>
pub struct CorpseTypeArgumentType {
    /// java: extends net.minecraft.commands.arguments.StringRepresentableArgument
    pub base: StringRepresentableArgument<compat::Object>,
}

impl std::ops::Deref for CorpseTypeArgumentType {
    type Target = StringRepresentableArgument<compat::Object>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CorpseTypeArgumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CorpseTypeArgumentType {
    /// java: protected CorpseTypeArgumentType()
    pub fn new() -> Self {
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`values: Vec<T>, codec_name: fn(&T) -> String, convert_id: fn(&str) -> String`), Java passes 2
        //   java: super(CODEC, CorpseType::values);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: static CorpseTypeArgumentType corpseType()
    pub fn corpse_type() -> CorpseTypeArgumentType {
        return CorpseTypeArgumentType::new();
    }

    /// java: static <S> CorpseType getCorpseType(CommandContext<S> context, String name)
    pub fn get_corpse_type<S: 'static>(context: &CommandContext<S>, name: &str) -> CorpseType {
        return context.get_argument::<CorpseType>(name).unwrap();
    }

}
