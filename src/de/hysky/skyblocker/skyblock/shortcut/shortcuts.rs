//! Ported from de.hysky.skyblocker.skyblock.shortcut.Shortcuts by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonDataBase;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::blaze3d::platform::input_constants::Key;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final Path SHORTCUTS_FILE = SkyblockerMod.CONFIG_DIR.resolve("shortcuts.json");
pub static SHORTCUTS_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("shortcuts.json"));
/// java: public static final JsonData<ShortcutsRecord> shortcuts = new JsonData<>(SHORTCUTS_FILE, ShortcutsRecord.CODEC, getDefaultShortcuts());
pub static SHORTCUTS: compat::MainThread<compat::Shared<dyn JsonData<ShortcutsRecord>>> = compat::MainThread::new(|| (compat::shared(JsonDataBase::new(SHORTCUTS_FILE.get_mut().clone(), SHORTCUTS_RECORD_CODEC.clone(), Shortcuts::get_default_shortcuts())) as compat::Shared<dyn JsonData<ShortcutsRecord>>));
/// java: private static final List<InputConstants.Key> pressedKeys = new ArrayList<>();
pub static PRESSED_KEYS: compat::MainThread<Vec<Key>> = compat::MainThread::new(|| Vec::new());
/// java: private static final long KEY_BINDING_COOLDOWN = 200;
pub const KEY_BINDING_COOLDOWN: i64 = ((200) as i64) as i64;
/// java: private static final Component NOT_LOADED_TEXT = Component.translatable("skyblocker.shortcuts.notLoaded").withColor(TextColor.RED).withStyle(ChatFormatting.BOLD);
pub static NOT_LOADED_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.shortcuts.notLoaded").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED).with_style_format(ChatFormatting::Bold));
/// java: private static long lastKeyBindingCommandTime;
pub static LAST_KEY_BINDING_COMMAND_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: @VisibleForTesting static final Codec<ShortcutsRecord> CODEC = RecordCodecBuilder.create(instance -> instance.group(
///   CodecUtils.object2ObjectMapCodec(Codec.STRING, Codec.STRING).fieldOf("commands").forGetter(ShortcutsRecord::commands),
///   CodecUtils.object2ObjectMapCodec(Codec.STRING, Codec.STRING).fieldOf("commandArgs").forGetter(ShortcutsRecord::commandArgs),
///   CodecUtils.mutableOptional(CodecUtils.object2ObjectMapCodec(ShortcutKeyBinding.CODEC, Codec.STRING).optionalFieldOf("keyBindings", Object2ObjectMaps.emptyMap()), Object2ObjectOpenHashMap::new).forGetter(ShortcutsRecord::keyBindings)
/// ).apply(instance, ShortcutsRecord::new));
/// (`LazyLock`: `JsonData` loads and saves on a worker thread)
pub static SHORTCUTS_RECORD_CODEC: LazyLock<CodecRef<ShortcutsRecord>> = LazyLock::new(|| {
    let string_map = || CodecUtils::object2_object_map_codec(&codec::string_codec(), &codec::string_codec());
    record_codec_builder::create(record_codec_builder::apply3(
        record_codec_builder::of(|o: &ShortcutsRecord| o.commands(), codec::field_of(string_map(), "commands")),
        record_codec_builder::of(|o: &ShortcutsRecord| o.command_args(), codec::field_of(string_map(), "commandArgs")),
        record_codec_builder::of(
            |o: &ShortcutsRecord| o.key_bindings(),
            CodecUtils::mutable_optional(
                &codec::optional_field_of(CodecUtils::object2_object_map_codec(&shortcut_key_binding::CODEC.clone(), &codec::string_codec()), "keyBindings", std::collections::HashMap::new()),
                // java: Object2ObjectOpenHashMap::new - a mutable copy of the (possibly immutable) empty default
                Arc::new(|map: std::collections::HashMap<ShortcutKeyBinding, String>| map),
            ),
        ),
        move |__a0, __a1, __a2| ShortcutsRecord::new(__a0.clone(), __a1.clone(), __a2.clone()),
    ))
});

/// java: public class Shortcuts
#[derive(Clone)]
pub struct Shortcuts {}

impl Shortcuts {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isShortcutsLoaded()
    pub fn is_shortcuts_loaded() -> bool {
        return SHORTCUTS.get_mut().lock().borrow().is_loaded();
    }

    /// java: @Init public static void init()
    pub fn init() {
        SHORTCUTS.get_mut().lock().borrow_mut().init();
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("help").executes(context -> {
                //   java: 			FabricClientCommandSource source = context.getSource();
                //   java: 			GeneralConfig.Shortcuts shortcutsConf = SkyblockerConfigManager.get().general.shortcuts;
                //   java: 			boolean shortcutsEnabled = shortcutsConf.enableShortcuts;
                //   java: 			source.sendFeedback(statusMessage("Command Shortcuts", shortcutsEnabled && shortcutsConf.enableCommandShortcuts));
                //   java: 			if (!isShortcutsLoaded()) {
                //   java: 				source.sendFeedback(NOT_LOADED_TEXT);
                //   java: 			} else for (Map.Entry<String, String> command : shortcuts.getData().commands.entrySet()) {
                //   java: 				source.sendFeedback(shortcutEntry(command.getKey(), command.getValue()));
                //   java: 			}
                //   java:
                //   java: 			source.sendFeedback(statusMessage("Command Argument Shortcuts", shortcutsEnabled && shortcutsConf.enableCommandArgShortcuts));
                //   java: 			if (!isShortcutsLoaded()) {
                //   java: 				source.sendFeedback(NOT_LOADED_TEXT);
                //   java: 			} else for (Map.Entry<String, String> commandArg : shortcuts.getData().commandArgs.entrySet()) {
                //   java: 				source.sendFeedback(shortcutEntry(commandArg.getKey(), commandArg.getValue()));
                //   java: 			}
                //   java:
                //   java: 			source.sendFeedback(statusMessage("Key Binding Shortcuts", shortcutsEnabled && shortcutsConf.enableKeyBindingShortcuts));
                //   java: 			if (!isShortcutsLoaded()) {
                //   java: 				source.sendFeedback(NOT_LOADED_TEXT);
                //   java: 			} else for (Map.Entry<ShortcutKeyBinding, String> keyBinding : shortcuts.getData().keyBindings.entrySet()) {
                //   java: 				source.sendFeedback(shortcutEntry(keyBinding.getKey().getBoundKeysText().getString(), keyBinding.getValue()));
                //   java: 			}
                //   java:
                //   java: 			source.sendFeedback(Component.empty().withStyle(ChatFormatting.BOLD)
                //   java: 					.append(Component.literal("Skyblocker").withColor(TextColor.YELLOW))
                //   java: 					.append(" ")
                //   java: 					.append("Commands")
                //   java: 			);
                //   java: 			for (String command : dispatcher.getSmartUsage(dispatcher.getRoot().getChild(SkyblockerMod.NAMESPACE), source).values()) {
                //   java: 				source.sendFeedback(Component.literal("/" + SkyblockerMod.NAMESPACE + " " + command).withColor(TextColor.GRAY));
                //   java: 			}
                //   java: 			return Command.SINGLE_SUCCESS;
                //   java: 			// Queue the screen or else the screen will be immediately closed after executing this command
                //   java: 		})).then(literal("shortcuts").executes(Scheduler.queueOpenScreenCommand(ShortcutsConfigScreen::new))));
                if !(SkyblockerConfigManager::get().general.shortcuts.enable_shortcuts) {
                    return;
                }
                if !(Shortcuts::is_shortcuts_loaded()) {
                    logger().warn("[Skyblocker Shortcuts] Shortcuts not loaded yet, skipping command registration");
                    return;
                }
                Shortcuts::register_shortcut_commands(dispatcher.dispatcher());
            },
        ));
        // TODO(fabric2seer): ClientSendMessageEvents.MODIFY_COMMAND.register: no SeerClient equivalent yet
        //   java: ClientSendMessageEvents.MODIFY_COMMAND.register(Shortcuts::modifyCommand);
    }

    /// java: private static ShortcutsRecord getDefaultShortcuts()
    pub fn get_default_shortcuts() -> ShortcutsRecord {
        let mut commands: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut command_args: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        commands.insert("/s".to_string(), "/skyblock".to_string());
        commands.insert("/i".to_string(), "/is".to_string());
        commands.insert("/h".to_string(), "/hub".to_string());
        commands.insert("/ga".to_string(), "/warp garden".to_string());
        commands.insert("/d".to_string(), "/warp dungeon_hub".to_string());
        commands.insert("/dn".to_string(), "/warp dungeon_hub".to_string());
        commands.insert("/ca".to_string(), "/chat all".to_string());
        commands.insert("/cp".to_string(), "/chat party".to_string());
        commands.insert("/cg".to_string(), "/chat guild".to_string());
        commands.insert("/co".to_string(), "/chat officer".to_string());
        command_args.insert("/m".to_string(), "/msg".to_string());
        command_args.insert("/pa".to_string(), "/p accept".to_string());
        commands.insert("/pd".to_string(), "/p disband".to_string());
        command_args.insert("/v".to_string(), "/visit".to_string());
        commands.insert("/vp".to_string(), "/visit portalhub".to_string());
        return ShortcutsRecord::new(commands.clone(), command_args.clone(), std::collections::HashMap::new());
    }

    /// java: @SuppressWarnings("unused") private static ShortcutsRecord getMoreDefaultShortcuts()
    pub fn get_more_default_shortcuts() -> ShortcutsRecord {
        let mut commands: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut command_args: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        commands.insert("/spider".to_string(), "/warp spider".to_string());
        commands.insert("/crimson".to_string(), "/warp nether".to_string());
        commands.insert("/end".to_string(), "/warp end".to_string());
        commands.insert("/gold".to_string(), "/warp gold".to_string());
        commands.insert("/cavern".to_string(), "/warp deep".to_string());
        commands.insert("/dwarven".to_string(), "/warp mines".to_string());
        commands.insert("/fo".to_string(), "/warp forge".to_string());
        commands.insert("/ch".to_string(), "/warp crystals".to_string());
        commands.insert("/park".to_string(), "/warp park".to_string());
        commands.insert("/barn".to_string(), "/warp barn".to_string());
        commands.insert("/desert".to_string(), "/warp desert".to_string());
        commands.insert("/ga".to_string(), "/warp garden".to_string());
        commands.insert("/castle".to_string(), "/warp castle".to_string());
        commands.insert("/museum".to_string(), "/warp museum".to_string());
        commands.insert("/da".to_string(), "/warp da".to_string());
        commands.insert("/crypt".to_string(), "/warp crypt".to_string());
        commands.insert("/nest".to_string(), "/warp nest".to_string());
        commands.insert("/magma".to_string(), "/warp magma".to_string());
        commands.insert("/void".to_string(), "/warp void".to_string());
        commands.insert("/drag".to_string(), "/warp drag".to_string());
        commands.insert("/jungle".to_string(), "/warp jungle".to_string());
        commands.insert("/howl".to_string(), "/warp howl".to_string());
        return ShortcutsRecord::new(commands.clone(), command_args.clone(), std::collections::HashMap::new());
    }

    /// java: private static Component statusMessage(String name, boolean enabled)
    pub fn status_message(name: &str, mut enabled: bool) -> MutableComponent {
        return component::empty().with_style_format(ChatFormatting::Bold).append(component::literal("Skyblocker").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW)).append_str(" ").append_str(name).append_str(" ").append((if enabled { component::literal("(Enabled)").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN) } else { component::literal("(Disabled)").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED) }));
    }

    /// java: private static Component shortcutEntry(String key, String value)
    pub fn shortcut_entry(key: &str, value: &str) -> MutableComponent {
        return (component::empty().append(component::literal(key).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GRAY)).append_str(" → ")).append(component::literal(value).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GRAY));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("help").executes(context -> {
        //   java: 			FabricClientCommandSource source = context.getSource();
        //   java: 			GeneralConfig.Shortcuts shortcutsConf = SkyblockerConfigManager.get().general.shortcuts;
        //   java: 			boolean shortcutsEnabled = shortcutsConf.enableShortcuts;
        //   java: 			source.sendFeedback(statusMessage("Command Shortcuts", shortcutsEnabled && shortcutsConf.enableCommandShortcuts));
        //   java: 			if (!isShortcutsLoaded()) {
        //   java: 				source.sendFeedback(NOT_LOADED_TEXT);
        //   java: 			} else for (Map.Entry<String, String> command : shortcuts.getData().commands.entrySet()) {
        //   java: 				source.sendFeedback(shortcutEntry(command.getKey(), command.getValue()));
        //   java: 			}
        //   java:
        //   java: 			source.sendFeedback(statusMessage("Command Argument Shortcuts", shortcutsEnabled && shortcutsConf.enableCommandArgShortcuts));
        //   java: 			if (!isShortcutsLoaded()) {
        //   java: 				source.sendFeedback(NOT_LOADED_TEXT);
        //   java: 			} else for (Map.Entry<String, String> commandArg : shortcuts.getData().commandArgs.entrySet()) {
        //   java: 				source.sendFeedback(shortcutEntry(commandArg.getKey(), commandArg.getValue()));
        //   java: 			}
        //   java:
        //   java: 			source.sendFeedback(statusMessage("Key Binding Shortcuts", shortcutsEnabled && shortcutsConf.enableKeyBindingShortcuts));
        //   java: 			if (!isShortcutsLoaded()) {
        //   java: 				source.sendFeedback(NOT_LOADED_TEXT);
        //   java: 			} else for (Map.Entry<ShortcutKeyBinding, String> keyBinding : shortcuts.getData().keyBindings.entrySet()) {
        //   java: 				source.sendFeedback(shortcutEntry(keyBinding.getKey().getBoundKeysText().getString(), keyBinding.getValue()));
        //   java: 			}
        //   java:
        //   java: 			source.sendFeedback(Component.empty().withStyle(ChatFormatting.BOLD)
        //   java: 					.append(Component.literal("Skyblocker").withColor(TextColor.YELLOW))
        //   java: 					.append(" ")
        //   java: 					.append("Commands")
        //   java: 			);
        //   java: 			for (String command : dispatcher.getSmartUsage(dispatcher.getRoot().getChild(SkyblockerMod.NAMESPACE), source).values()) {
        //   java: 				source.sendFeedback(Component.literal("/" + SkyblockerMod.NAMESPACE + " " + command).withColor(TextColor.GRAY));
        //   java: 			}
        //   java: 			return Command.SINGLE_SUCCESS;
        //   java: 			// Queue the screen or else the screen will be immediately closed after executing this command
        //   java: 		})).then(literal("shortcuts").executes(Scheduler.queueOpenScreenCommand(ShortcutsConfigScreen::new))));
        if !(SkyblockerConfigManager::get().general.shortcuts.enable_shortcuts) {
            return;
        }
        if !(Shortcuts::is_shortcuts_loaded()) {
            logger().warn("[Skyblocker Shortcuts] Shortcuts not loaded yet, skipping command registration");
            return;
        }
        // (the callback above registers these: the port hands its listener the Fabric-side dispatcher)
    }

    /// java: the shortcut half of `registerCommands`: each command shortcut as a literal, each command argument
    /// shortcut as a redirect to the command it stands for (or a literal taking the rest as `args`)
    pub fn register_shortcut_commands(dispatcher: &CommandDispatcher<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>) {
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::mcrust::modding::events::client_command_registration_callback::{argument, literal};
        let data = SHORTCUTS.get().lock().borrow().get_data();
        for key in data.commands.keys() {
            if key.starts_with("/") {
                dispatcher.register(&literal(&compat::substring(key, 1)));
            }
        }
        for (set_key, set_value) in data.command_args.iter() {
            if set_key.starts_with("/") {
                let mut redirect_location = Some(dispatcher.get_root().clone());
                for word in compat::regex::split(&compat::substring(set_value, 1), " ") {
                    redirect_location = redirect_location.and_then(|node| node.get_child(&word));
                    if redirect_location.is_none() {
                        break;
                    }
                }
                let mut node = literal(&compat::substring(set_key, 1));
                match redirect_location {
                    None => node.then(&argument("args", Arc::new(mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType::greedy_string()))),
                    Some(redirect_location) => node.redirect(redirect_location),
                }
                dispatcher.register(&node);
            }
        }
    }

    /// java: private static String modifyCommand(String command)
    pub fn modify_command(mut command: String) -> String {
        if !(SkyblockerConfigManager::get().general.shortcuts.enable_shortcuts) {
            return command;
        }
        if !(Shortcuts::is_shortcuts_loaded()) {
            logger().warn(&format!("[Skyblocker Shortcuts] Shortcuts not loaded yet, skipping shortcut for command: {}", command));
            return command;
        }
        command = format!("/{}", command);
        if SkyblockerConfigManager::get().general.shortcuts.enable_command_shortcuts {
            command = SHORTCUTS.get().lock().borrow().get_data().commands.get(&command).cloned().unwrap_or(command);
        }
        if SkyblockerConfigManager::get().general.shortcuts.enable_command_arg_shortcuts {
            let mut message_args: Vec<String> = compat::regex::split(&command, " ");
            for i in 0..(message_args.len() as i32) {
                let __arg = message_args[(i) as usize].clone();
                message_args[(i) as usize] = SHORTCUTS.get().lock().borrow().get_data().command_args.get(&__arg).cloned().unwrap_or(__arg);
            }
            command = message_args.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");
        }
        return compat::substring(&command, 1);
    }

    /// java: public static void onKeyPressed(InputConstants.Key key)
    pub fn on_key_pressed(key: &Key) {
        if !(SkyblockerConfigManager::get().general.shortcuts.enable_shortcuts) || !(SkyblockerConfigManager::get().general.shortcuts.enable_key_binding_shortcuts) {
            return;
        }
        if !(Shortcuts::is_shortcuts_loaded()) {
            logger().warn(&format!("[Skyblocker Shortcuts] Shortcuts not loaded yet, skipping key binding check for key: {}", compat::jstring(&key)));
            return;
        }
        let __key_bindings = SHORTCUTS.get().lock().borrow().get_data().key_bindings;
        let mut command: Option<String> = __key_bindings.get(&ShortcutKeyBinding::new(vec![key.clone()])).cloned();
        // Check for combinations
        if command.is_none() {
            // This should never happen, but the last pressed key was not added to the list!
            if PRESSED_KEYS.get().is_empty() || !(PRESSED_KEYS.get()[PRESSED_KEYS.get().len() - 1] == *key) {
                Shortcuts::set_key_pressed(key.clone(), true);
                logger().warn(&format!("[Skyblocker Shortcuts] Key {} was not in the pressed keys list when it should be. Check if `setKeyPressed` is always called before `onKeyPressed`.", compat::jstring(&key)));
            }
            command = __key_bindings.get(&ShortcutKeyBinding::new(PRESSED_KEYS.get().clone())).cloned();
        }
        let Some(command) = command else {
            return;
        };
        if (compat::get_state(&LAST_KEY_BINDING_COMMAND_TIME)).wrapping_add(KEY_BINDING_COOLDOWN) > compat::millis() {
            return;
        }
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &command, true);
        compat::set_state(&LAST_KEY_BINDING_COMMAND_TIME, compat::millis());
    }

    /// java: public static void setKeyPressed(InputConstants.Key key, boolean pressed)
    pub fn set_key_pressed(mut key: Key, mut pressed: bool) {
        if !(SkyblockerConfigManager::get().general.shortcuts.enable_shortcuts) || !(SkyblockerConfigManager::get().general.shortcuts.enable_key_binding_shortcuts) {
            return;
        }
        if pressed {
            if PRESSED_KEYS.get_mut().is_empty() || !(PRESSED_KEYS.get_mut()[PRESSED_KEYS.get_mut().len() - 1] == key) {
                PRESSED_KEYS.get_mut().push(key.clone());
            }
        } else {
            compat::remove_item(&mut *PRESSED_KEYS.get_mut(), &key.clone());
        }
    }

}

/// java: public record ShortcutsRecord(Object2ObjectMap<String, String> commands, Object2ObjectMap<String, String> commandArgs, Object2ObjectMap<ShortcutKeyBinding, String> keyBindings)
#[derive(Clone, Debug)]
pub struct ShortcutsRecord {
    /// java: Object2ObjectMap<String, String> commands
    pub commands: std::collections::HashMap<String, String>,
    /// java: Object2ObjectMap<String, String> commandArgs
    pub command_args: std::collections::HashMap<String, String>,
    /// java: Object2ObjectMap<ShortcutKeyBinding, String> keyBindings
    pub key_bindings: std::collections::HashMap<ShortcutKeyBinding, String>,
}

impl ShortcutsRecord {
    /// java: the canonical record constructor
    pub fn new(mut commands: std::collections::HashMap<String, String>, mut command_args: std::collections::HashMap<String, String>, mut key_bindings: std::collections::HashMap<ShortcutKeyBinding, String>) -> Self {
        Self { commands, command_args, key_bindings }
    }

    /// java: the record accessor `commands()`
    pub fn commands(&self) -> std::collections::HashMap<String, String> {
        self.commands.clone()
    }

    /// java: the record accessor `commandArgs()`
    pub fn command_args(&self) -> std::collections::HashMap<String, String> {
        self.command_args.clone()
    }

    /// java: the record accessor `keyBindings()`
    pub fn key_bindings(&self) -> std::collections::HashMap<ShortcutKeyBinding, String> {
        self.key_bindings.clone()
    }

    /// java: public int size()
    pub fn size(&self) -> i32 {
        return ((self.commands.len() as i32).wrapping_add(self.command_args.len() as i32)).wrapping_add(self.key_bindings.len() as i32);
    }

    /// java: public void clear()
    pub fn clear(&mut self) {
        self.commands.clear();
        self.command_args.clear();
        self.key_bindings.clear();
    }

}
