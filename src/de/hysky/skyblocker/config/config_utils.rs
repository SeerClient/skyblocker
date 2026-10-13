//! Ported from de.hysky.skyblocker.config.ConfigUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component;
use crate::de::hysky::skyblocker::config::categories::ConfigEnum;
use dandelion::net::azureaaron::dandelion::api::controllers::boolean_controller::{self, BooleanController, BooleanStyle, Builder as _};
use dandelion::net::azureaaron::dandelion::api::controllers::colour_controller::{self, Builder as _, ColourController};
use dandelion::net::azureaaron::dandelion::api::controllers::controller::Builder as _;
use dandelion::net::azureaaron::dandelion::api::controllers::enum_controller::{self, Builder as _, EnumController};

/// java: public static final Function<ChatFormatting, Component> FORMATTING_FORMATTER = formatting -> Component.literal(StringUtils.capitalize(formatting.name().replaceAll("_", " ")));
pub static FORMATTING_FORMATTER: compat::MainThread<Arc<dyn Fn(ChatFormatting) -> MutableComponent>> = compat::MainThread::new(|| {
    Arc::new(|formatting: ChatFormatting| -> MutableComponent {
        // StringUtils.capitalize: the first character title-cased, the rest as it is
        let name: String = compat::chat_formatting_name(formatting).replace("_", " ");
        let mut chars = name.chars();
        let capitalized: String = match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => name,
        };
        mcrust::net::minecraft::network::chat::component::literal(&capitalized)
    })
});

/// java: public class ConfigUtils
#[derive(Clone)]
pub struct ConfigUtils {}

impl ConfigUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static BooleanController createBooleanController()
    pub fn create_boolean_controller() -> Arc<dyn BooleanController> {
        return boolean_controller::create_builder().coloured(true).boolean_style(BooleanStyle::YesNo).build();
    }

    /// java: public static ColourController createColourController(boolean hasAlpha)
    pub fn create_colour_controller(has_alpha: bool) -> Arc<dyn ColourController> {
        return colour_controller::create_builder().has_alpha(has_alpha).build();
    }

    /// java: @SuppressWarnings("unchecked") public static <T extends Enum<T>> EnumController<T> createEnumController()
    ///
    /// Dandelion's default formatter is `Component.nullToEmpty(v.toString())`; Skyblocker's enums override
    /// `toString()`, which the Rust `JavaEnum` cannot see, so the same formatter is handed in through
    /// `ConfigEnum::java_to_string`.
    pub fn create_enum_controller<T: ConfigEnum>() -> Arc<dyn EnumController<T>> {
        return enum_controller::create_builder::<T>().formatter(|v: T| component::null_to_empty(Some(v.java_to_string().as_str()))).build();
    }

    /// java: @SuppressWarnings("unchecked") public static <T extends Enum<T>> EnumController<T> createEnumController(Function<T, Component> formatter)
    pub fn create_enum_controller_1<T: ConfigEnum>(formatter: Arc<dyn Fn(T) -> MutableComponent>) -> Arc<dyn EnumController<T>> {
        return enum_controller::create_builder::<T>().formatter(move |v: T| formatter(v)).build();
    }

    /// java: @SuppressWarnings("unchecked") public static <T extends Enum<T>> EnumController<T> createEnumDropdownController(Function<T, Component> formatter)
    pub fn create_enum_dropdown_controller<T: ConfigEnum>(formatter: Arc<dyn Fn(T) -> MutableComponent>) -> Arc<dyn EnumController<T>> {
        return enum_controller::create_builder::<T>().dropdown(true).formatter(move |v: T| formatter(v)).build();
    }

}
