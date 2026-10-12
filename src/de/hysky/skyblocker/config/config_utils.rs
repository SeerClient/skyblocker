//! Ported from de.hysky.skyblocker.config.ConfigUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

// TODO(fabric2seer): static call StringUtils.capitalize(..)
//   java: public static final Function<ChatFormatting, Component> FORMATTING_FORMATTER = formatting -> Component.literal(StringUtils.capitalize(formatting.name().replaceAll("_", " ")));
/// java: public static final Function<ChatFormatting, Component> FORMATTING_FORMATTER = formatting -> Component.literal(StringUtils.capitalize(formatting.name().replaceAll("_", " ")));
pub static FORMATTING_FORMATTER: compat::MainThread<Arc<dyn Fn(ChatFormatting) -> MutableComponent>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FORMATTING_FORMATTER` - see the TODO above"));

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
    pub fn create_boolean_controller() -> crate::unresolved::BooleanController {
        // TODO(fabric2seer): static call BooleanController.createBuilder(..)
        //   java: return BooleanController.createBuilder()
        //   java: 				.coloured(true)
        //   java: 				.booleanStyle(BooleanStyle.YES_NO)
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static ColourController createColourController(boolean hasAlpha)
    pub fn create_colour_controller(mut has_alpha: bool) -> crate::unresolved::ColourController {
        // TODO(fabric2seer): static call ColourController.createBuilder(..)
        //   java: return ColourController.createBuilder()
        //   java: 				.hasAlpha(hasAlpha)
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("unchecked") public static <T extends Enum<T>> EnumController<T> createEnumController()
    pub fn create_enum_controller<T: 'static>() -> crate::unresolved::EnumController {
        // TODO(fabric2seer): static call EnumController.createBuilder(..)
        //   java: return (EnumController<T>) EnumController.createBuilder().build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("unchecked") public static <T extends Enum<T>> EnumController<T> createEnumController(Function<T, Component> formatter)
    pub fn create_enum_controller_1<T: 'static>(formatter: Arc<dyn Fn(T) -> MutableComponent>) -> crate::unresolved::EnumController {
        // TODO(fabric2seer): static call EnumController.createBuilder(..)
        //   java: return (EnumController<T>) EnumController.createBuilder().formatter(Function.class.cast(formatter)).build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("unchecked") public static <T extends Enum<T>> EnumController<T> createEnumDropdownController(Function<T, Component> formatter)
    pub fn create_enum_dropdown_controller<T: 'static>(formatter: Arc<dyn Fn(T) -> MutableComponent>) -> crate::unresolved::EnumController {
        // TODO(fabric2seer): static call EnumController.createBuilder(..)
        //   java: return (EnumController<T>) EnumController.createBuilder().dropdown(true).formatter(Function.class.cast(formatter)).build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
