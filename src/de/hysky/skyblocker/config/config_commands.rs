//! Ported from de.hysky.skyblocker.config.ConfigCommands by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class ConfigCommands
#[derive(Clone)]
pub struct ConfigCommands {}

impl ConfigCommands {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> registerConfigEntries(LiteralArgumentBuilder<FabricClientCommandSource> builder)
    pub fn register_config_entries(builder: &LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource>) -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return SkyblockerConfigManager::get_unpatched(|config| ConfigCommands::register_config_entries_2(builder, config));
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Config Manager] Failed to register config entries command!"));
            }
        }
        // TODO(fabric2seer): the mod keeps the game's `LiteralArgumentBuilder` (Java holds a reference to the live object); the port's `LiteralArgumentBuilder` is not a value to copy and the port lends the mod no handle to it
        // rust: return builder.clone();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> registerConfigEntries(LiteralArgumentBuilder<FabricClientCommandSource> builder, Object object) throws IllegalAccessException
    ///
    /// `Object object`: the object itself (`&dyn Any`).
    /// rust: port gap - the body walks `getDeclaredFields()` (Java reflection), which Rust has no counterpart for (GAPS_batchG).
    pub fn register_config_entries_2(builder: &LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource>, object: &dyn std::any::Any) -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): `.getDeclaredFields(..)` on a Class
        //   java: for (Field field : object.getClass().getDeclaredFields()) {
        //   java: 			if (Modifier.isStatic(field.getModifiers())) continue;
        //   java: 			field.setAccessible(true);
        //   java:
        //   java: 			Class<?> type = field.getType();
        //   java: 			String name = field.getName();
        //   java: 			Object value = field.get(object);
        //   java:
        //   java: 			if (type == boolean.class) {
        //   java: 				builder.then(registerBooleanConfigEntry(field, object, name));
        //   java: 			} else if (value != null && SkyblockerConfigManager.isConfigClass(type)) {
        //   java: 				builder.then(registerConfigEntries(literal(name), value));
        //   java: 			}
        //   java: 		}
        // TODO(fabric2seer): the mod keeps the game's `LiteralArgumentBuilder` (Java holds a reference to the live object); the port's `LiteralArgumentBuilder` is not a value to copy and the port lends the mod no handle to it
        // rust: return builder.clone();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private static Component formatValue(boolean bl)
    pub fn format_value(mut bl: bool) -> MutableComponent {
        return component::literal(&bl.to_string()).with_style_format((if bl { ChatFormatting::Green } else { ChatFormatting::Red }));
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> registerBooleanConfigEntry(Field field, Object object, String name)
    pub fn register_boolean_config_entry(field: &mut crate::unresolved::Field, object: &compat::Object, name: &str) -> LiteralArgumentBuilder<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal(name).then(argument("value", BoolArgumentType.bool()).executes(context -> {
        //   java: 			SkyblockerConfigManager.update(_ -> {
        //   java: 				try {
        //   java: 					boolean value = BoolArgumentType.getBool(context, "value");
        //   java: 					field.setBoolean(object, value);
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.config.commands.set", name, formatValue(value))));
        //   java: 				} catch (IllegalAccessException e) {
        //   java: 					throw new RuntimeException(e);
        //   java: 				}
        //   java: 			});
        //   java: 			return Command.SINGLE_SUCCESS;
        //   java: 		})).then(literal("toggle").executes(context -> {
        //   java: 			SkyblockerConfigManager.update(_ -> {
        //   java: 				try {
        //   java: 					boolean toggled = !field.getBoolean(object);
        //   java: 					field.setBoolean(object, toggled);
        //   java: 					context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.config.commands.set", name, formatValue(toggled))));
        //   java: 				} catch (IllegalAccessException e) {
        //   java: 					throw new RuntimeException(e);
        //   java: 				}
        //   java: 			});
        //   java: 			return Command.SINGLE_SUCCESS;
        //   java: 		})).executes(context -> {
        //   java: 			try {
        //   java: 				context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.config.commands.query", name, formatValue(field.getBoolean(object)))));
        //   java: 			} catch (IllegalAccessException e) {
        //   java: 				throw new RuntimeException(e);
        //   java: 			}
        //   java: 			return Command.SINGLE_SUCCESS;
        //   java: 		});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
