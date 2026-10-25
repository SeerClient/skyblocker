//! Ported from de.hysky.skyblocker.config.ConfigCommands by fabric2seer.

#[allow(unused_imports)]
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::com::mojang::brigadier::arguments::bool_argument_type::BoolArgumentType;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

type Source = client_command_registration_callback::FabricClientCommandSource;

/// java: `Field field, Object object` - a field of a config object, as reflection hands it over. The generated
/// config classes answer reflection through their `ClassOf` (`SkyblockerConfig::__class_of()`): `to_gson` lists
/// the declared fields in declaration order with their values, and `patch` is `Field.setBoolean` on a dotted path
/// from the root config object. So a field is its path from the root, and the object is the live unpatched config.
#[derive(Clone, Debug)]
pub struct ConfigField {
    /// the field names from the root `SkyblockerConfig` down to this field
    pub path: Vec<String>,
}

impl ConfigField {
    /// `field.getBoolean(object)`: the field's value in the live unpatched config
    pub fn get_boolean(&self) -> bool {
        let value = SkyblockerConfigManager::get_unpatched(|config| (SkyblockerConfig::__class_of().to_gson)(config));
        let mut current = &value;
        for name in self.path.iter() {
            let dandelion::compat::GsonValue::Obj(fields) = current else {
                panic!("IllegalAccessException: {}", self.path.join("."));
            };
            let Some((_, next)) = fields.iter().find(|(key, _)| key == name) else {
                panic!("IllegalAccessException: {}", self.path.join("."));
            };
            current = next;
        }
        match current {
            dandelion::compat::GsonValue::Bool(b) => *b,
            _ => panic!("IllegalArgumentException: {} is not a boolean", self.path.join(".")),
        }
    }

    /// `field.setBoolean(object, value)` on the config `SkyblockerConfigManager.update` lends
    pub fn set_boolean(&self, config: &mut SkyblockerConfig, value: bool) {
        let path: Vec<&str> = self.path.iter().map(|s| s.as_str()).collect();
        if !(SkyblockerConfig::__class_of().patch)(config, &path, &dandelion::compat::PatchValue::Bool(value)) {
            panic!("IllegalAccessException: {}", self.path.join("."));
        }
    }
}

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
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 		.then(ConfigCommands.registerConfigEntries(literal("configExecute"))))
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&ConfigCommands::register_config_entries(client_command_registration_callback::literal("configExecute")));
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> registerConfigEntries(LiteralArgumentBuilder<FabricClientCommandSource> builder)
    pub fn register_config_entries(builder: LiteralArgumentBuilder<Source>) -> LiteralArgumentBuilder<Source> {
        // java: try { return registerConfigEntries(builder, SkyblockerConfigManager.getUnpatched()); } catch (Exception e) {..} return builder;
        // (the builder is shared with the try body: a failure part way keeps the entries already added, as in Java)
        let builder = std::cell::RefCell::new(builder);
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let value = SkyblockerConfigManager::get_unpatched(|config| (SkyblockerConfig::__class_of().to_gson)(config));
            ConfigCommands::register_config_entries_2(&mut builder.borrow_mut(), &value, &Vec::new());
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Config Manager] Failed to register config entries command! {}", compat::panic_message(&e)));
        }
        return builder.into_inner();
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> registerConfigEntries(LiteralArgumentBuilder<FabricClientCommandSource> builder, Object object) throws IllegalAccessException
    ///
    /// `object` is the config object's reflected form (`ClassOf.to_gson`: its declared fields in declaration order,
    /// with their values) and `path` the field path that reaches it from the root config.
    pub fn register_config_entries_2(builder: &mut LiteralArgumentBuilder<Source>, object: &dandelion::compat::GsonValue, path: &Vec<String>) {
        let dandelion::compat::GsonValue::Obj(fields) = object else {
            return;
        };
        //   java: for (Field field : object.getClass().getDeclaredFields()) {
        //   java: 	if (Modifier.isStatic(field.getModifiers())) continue;   (static fields are not serialized)
        //   java: 	field.setAccessible(true); Class<?> type = field.getType(); String name = field.getName(); Object value = field.get(object);
        for (name, value) in fields.iter() {
            let mut field_path: Vec<String> = path.clone();
            field_path.push(name.clone());
            match value {
                //   java: if (type == boolean.class) builder.then(registerBooleanConfigEntry(field, object, name));
                // (a boolean of a config class is exactly what `patch` can set: a `Map`'s boolean value is not)
                dandelion::compat::GsonValue::Bool(current) if ConfigCommands::is_boolean_field(&field_path, *current) => {
                    builder.then(&ConfigCommands::register_boolean_config_entry(&ConfigField { path: field_path }, name));
                }
                //   java: else if (value != null && SkyblockerConfigManager.isConfigClass(type)) builder.then(registerConfigEntries(literal(name), value));
                // (a config class is a nested object with at least one patchable field; a `Map` / record has none)
                dandelion::compat::GsonValue::Obj(_) => {
                    let mut nested = client_command_registration_callback::literal(name);
                    if ConfigCommands::is_config_object(value, &field_path) {
                        ConfigCommands::register_config_entries_2(&mut nested, value, &field_path);
                        builder.then(&nested);
                    }
                }
                _ => {}
            }
        }
    }

    /// `type == boolean.class`: the field at `path` is a boolean field of a config class (Dandelion's `patch` sets it on
    /// a scratch instance - `getDeclaredConstructor().newInstance()` - so the live config is untouched).
    fn is_boolean_field(path: &Vec<String>, current: bool) -> bool {
        thread_local! {
            /// the scratch instance (built once: the walk asks for every field of the config)
            static SCRATCH: std::cell::RefCell<Option<SkyblockerConfig>> = const { std::cell::RefCell::new(None) };
        }
        let class = SkyblockerConfig::__class_of();
        let path: Vec<&str> = path.iter().map(|s| s.as_str()).collect();
        SCRATCH.with(|scratch| {
            let mut scratch = scratch.borrow_mut();
            let scratch: &mut SkyblockerConfig = scratch.get_or_insert_with(class.new);
            (class.patch)(scratch, &path, &dandelion::compat::PatchValue::Bool(current))
        })
    }

    /// `SkyblockerConfigManager.isConfigClass(type)` for the nested object at `path`: a config class declares at least
    /// one field reflection can reach (a boolean, or a nested config class).
    fn is_config_object(object: &dandelion::compat::GsonValue, path: &Vec<String>) -> bool {
        let dandelion::compat::GsonValue::Obj(fields) = object else {
            return false;
        };
        fields.iter().any(|(name, value)| {
            let mut field_path: Vec<String> = path.clone();
            field_path.push(name.clone());
            match value {
                dandelion::compat::GsonValue::Bool(current) => ConfigCommands::is_boolean_field(&field_path, *current),
                dandelion::compat::GsonValue::Obj(_) => ConfigCommands::is_config_object(value, &field_path),
                _ => false,
            }
        })
    }

    /// java: private static Component formatValue(boolean bl)
    pub fn format_value(mut bl: bool) -> MutableComponent {
        return component::literal(&bl.to_string()).with_style_format((if bl { ChatFormatting::Green } else { ChatFormatting::Red }));
    }

    /// `Constants.PREFIX.get().append(Component.translatable(key, name, formatValue(value)))` as feedback
    fn send_value_feedback(key: &str, name: &str, value: bool) {
        let text = (constants::PREFIX.get())().append(component::translatable_args(key, vec![FormattedText::of(name), FormattedText::from(ConfigCommands::format_value(value))]));
        client_command_registration_callback::send_feedback(&text.get_string());
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> registerBooleanConfigEntry(Field field, Object object, String name)
    pub fn register_boolean_config_entry(field: &ConfigField, name: &str) -> LiteralArgumentBuilder<Source> {
        //   java: return literal(name).then(argument("value", BoolArgumentType.bool()).executes(context -> {
        //   java: 	SkyblockerConfigManager.update(_ -> { boolean value = BoolArgumentType.getBool(context, "value"); field.setBoolean(object, value);
        //   java: 		context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.config.commands.set", name, formatValue(value)))); });
        //   java: 	return Command.SINGLE_SUCCESS; }))
        let mut value_argument = client_command_registration_callback::argument("value", Arc::new(BoolArgumentType::bool()));
        let (set_field, set_name) = (field.clone(), name.to_string());
        value_argument.executes(Arc::new(move |context: &CommandContext<Source>| {
            let value: bool = BoolArgumentType::get_bool(context, "value");
            let (field, name) = (set_field.clone(), set_name.clone());
            SkyblockerConfigManager::update(Arc::new(move |config: &mut SkyblockerConfig| {
                field.set_boolean(config, value);
                ConfigCommands::send_value_feedback("skyblocker.config.commands.set", &name, value);
            }));
            Ok(1)
        }));
        //   java: .then(literal("toggle").executes(context -> {
        //   java: 	SkyblockerConfigManager.update(_ -> { boolean toggled = !field.getBoolean(object); field.setBoolean(object, toggled);
        //   java: 		context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.config.commands.set", name, formatValue(toggled)))); });
        //   java: 	return Command.SINGLE_SUCCESS; }))
        let mut toggle = client_command_registration_callback::literal("toggle");
        let (toggle_field, toggle_name) = (field.clone(), name.to_string());
        toggle.executes(Arc::new(move |_: &CommandContext<Source>| {
            // `field.getBoolean(object)` reads the unpatched config, which `update` then lends: read before it is lent
            let toggled: bool = !toggle_field.get_boolean();
            let (field, name) = (toggle_field.clone(), toggle_name.clone());
            SkyblockerConfigManager::update(Arc::new(move |config: &mut SkyblockerConfig| {
                field.set_boolean(config, toggled);
                ConfigCommands::send_value_feedback("skyblocker.config.commands.set", &name, toggled);
            }));
            Ok(1)
        }));
        //   java: .executes(context -> { context.getSource().sendFeedback(Constants.PREFIX.get().append(
        //   java: 	Component.translatable("skyblocker.config.commands.query", name, formatValue(field.getBoolean(object))))); return Command.SINGLE_SUCCESS; });
        let mut literal = client_command_registration_callback::literal(name);
        literal.then(&value_argument);
        literal.then(&toggle);
        let (query_field, query_name) = (field.clone(), name.to_string());
        literal.executes(Arc::new(move |_: &CommandContext<Source>| {
            ConfigCommands::send_value_feedback("skyblocker.config.commands.query", &query_name, query_field.get_boolean());
            Ok(1)
        }));
        return literal;
    }

}
