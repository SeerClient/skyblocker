//! Ported from de.hysky.skyblocker.skyblock.slayers.SlayerTier by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<SlayerTier> CODEC = StringRepresentable.fromEnum(SlayerTier::values);
/// java: public static final Codec<SlayerTier> CODEC = StringRepresentable.fromEnum(SlayerTier::values);
pub static CODEC: compat::MainThread<CodecRef<SlayerTier>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public enum SlayerTier implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SlayerTier {
    I,
    Ii,
    Iii,
    Iv,
    V,
}

impl SlayerTier {
    pub const VALUES: [SlayerTier; 5] = [SlayerTier::I, SlayerTier::Ii, SlayerTier::Iii, SlayerTier::Iv, SlayerTier::V];

    /// java: values()
    pub fn values() -> Vec<SlayerTier> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SlayerTier::I => "I",
            SlayerTier::Ii => "II",
            SlayerTier::Iii => "III",
            SlayerTier::Iv => "IV",
            SlayerTier::V => "V",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SlayerTier {
        match name {
            "I" => SlayerTier::I,
            "II" => SlayerTier::Ii,
            "III" => SlayerTier::Iii,
            "IV" => SlayerTier::Iv,
            "V" => SlayerTier::V,
            _ => panic!("IllegalArgumentException: No enum constant SlayerTier.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SlayerTierFields {
        static FIELDS: LazyLock<Vec<SlayerTierFields>> = LazyLock::new(|| vec![
            SlayerTierFields::new("I".to_string(), ChatFormatting::Green),
            SlayerTierFields::new("II".to_string(), ChatFormatting::Yellow),
            SlayerTierFields::new("III".to_string(), ChatFormatting::Red),
            SlayerTierFields::new("IV".to_string(), ChatFormatting::DarkRed),
            SlayerTierFields::new("V".to_string(), ChatFormatting::DarkPurple),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static SlayerTier valueOf(@Nullable String name, String slayerName)
    pub fn value_of(mut name: Option<String>, slayer_name: &str) -> SlayerTier {
        if slayer_name.contains("Conjoined Brood") {
            return SlayerTier::V;
        }
        if slayer_name.contains("Atoned Horror") {
            return SlayerTier::V;
        }
        if slayer_name.contains("Bloodfiend") && name.is_none() {
            return SlayerTier::V;
        }
        let Some(mut name) = name.clone() else {
            return SlayerTier::I;
        };
        // TODO(fabric2seer): no overload of `valueOf` takes 1 argument(s)
        //   java: return SlayerTier.valueOf(name);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

}

impl std::fmt::Display for SlayerTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each SlayerTier constant
pub struct SlayerTierFields {
    pub name: Option<String>,
    pub color: ChatFormatting,
}

impl SlayerTierFields {
    fn new(mut name: String, mut color: ChatFormatting) -> Self {
        let mut this = Self {
            name: None,
            color: Default::default(),
        };
        this.name = name.clone();
        this.color = color;
        this
    }

}
