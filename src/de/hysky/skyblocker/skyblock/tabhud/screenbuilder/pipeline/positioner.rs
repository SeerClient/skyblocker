//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.pipeline.Positioner by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::centered_widget_positioner::CenteredWidgetPositioner;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::top_aligned_widget_positioner::TopAlignedWidgetPositioner;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_positioner::WidgetPositioner;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<Positioner> CODEC = StringRepresentable.fromEnum(Positioner::values);
/// java: public static final Codec<Positioner> CODEC = StringRepresentable.fromEnum(Positioner::values);
pub static CODEC: compat::MainThread<CodecRef<Positioner>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(Positioner::values(), |e: &Positioner| e.get_serialized_name()));

/// java: public enum Positioner implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Positioner {
    Top,
    Centered,
}

impl Positioner {
    pub const VALUES: [Positioner; 2] = [Positioner::Top, Positioner::Centered];

    /// java: values()
    pub fn values() -> Vec<Positioner> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Positioner::Top => "TOP",
            Positioner::Centered => "CENTERED",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Positioner {
        match name {
            "TOP" => Positioner::Top,
            "CENTERED" => Positioner::Centered,
            _ => panic!("IllegalArgumentException: No enum constant Positioner.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static PositionerFields {
        static FIELDS: LazyLock<Vec<PositionerFields>> = LazyLock::new(|| vec![
            PositionerFields::new(|__a0, __a1| TopAlignedWidgetPositioner::new(__a0, __a1) as compat::Shared<dyn WidgetPositioner>),
            PositionerFields::new(|__a0, __a1| CenteredWidgetPositioner::new(__a0, __a1) as compat::Shared<dyn WidgetPositioner>),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public WidgetPositioner getNewPositioner(float maxHeight, int screenHeight)
    pub fn get_new_positioner(&self, mut max_height: f32, mut screen_height: i32) -> compat::Shared<dyn WidgetPositioner> {
        return (self.fields().function)(max_height, screen_height);
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // java: return name().toLowerCase(Locale.ENGLISH);
        return self.name().to_lowercase();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // java: return I18n.get("skyblocker.config.uiAndVisuals.tabHud.defaultPosition." + name());
        return mcrust::net::minecraft::locale::language::get_or_default(&format!("skyblocker.config.uiAndVisuals.tabHud.defaultPosition.{}", self.name()));
    }

}

impl std::fmt::Display for Positioner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Positioner::to_string(self))
    }
}

/// the constructor data of each Positioner constant (`TopAlignedWidgetPositioner::new` / `CenteredWidgetPositioner::new`,
/// constructor references with no captured state: function pointers, which the static table may hold)
pub struct PositionerFields {
    /// java: private final BiFunction<Float, Integer, WidgetPositioner> function;
    pub function: fn(f32, i32) -> compat::Shared<dyn WidgetPositioner>,
}

impl PositionerFields {
    /// java: Positioner(BiFunction<Float, Integer, WidgetPositioner> widgetPositionerSupplier)
    fn new(widget_positioner_supplier: fn(f32, i32) -> compat::Shared<dyn WidgetPositioner>) -> Self {
        Self { function: widget_positioner_supplier }
    }

}
