//! Ported from de.hysky.skyblocker.skyblock.speedpreset.SpeedPresets by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonDataBase;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::tree::command_node::CommandNode;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: private static final Pattern COMMAND_PATTERN = Pattern.compile("^setmaxspeed\\s([a-zA-Z]\\w*)$");
pub static COMMAND_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^setmaxspeed\\s([a-zA-Z]\\w*)$"));
/// java: private static final Codec<Object2IntMap<String>> MAP_CODEC = CodecUtils.object2IntMapCodec(Codec.STRING);
pub static MAP_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, i32>>> = compat::MainThread::new(|| CodecUtils::object2_int_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()));
/// java: private static final Path PRESETS_FILE = SkyblockerMod.CONFIG_DIR.resolve("speed_presets.json");
pub static PRESETS_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("speed_presets.json"));
/// java: private static SpeedPresets instance;
pub static INSTANCE: compat::MainThread<Option<SpeedPresets>> = compat::MainThread::new(|| None);

/// java: public class SpeedPresets
#[derive(Clone)]
pub struct SpeedPresets {
    /// java: private final JsonData<Object2IntMap<String>> presets = new JsonData<>(PRESETS_FILE, MAP_CODEC, createDefaultMap());
    pub presets: compat::Shared<dyn JsonData<std::collections::HashMap<String, i32>>>,
}

impl SpeedPresets {
    /// java: private SpeedPresets()
    pub fn new() -> Self {
        let mut this = Self {
            presets: (compat::shared(JsonDataBase::new(PRESETS_FILE.get_mut().clone(), MAP_CODEC.get_mut().clone(), SpeedPresets::create_default_map())) as compat::Shared<dyn JsonData<std::collections::HashMap<String, i32>>>),
        };
        this
    }

    /// java: public static SpeedPresets getInstance()
    pub fn get_instance() -> SpeedPresets {
        // java: return instance == null ? instance = new SpeedPresets() : instance;
        // (the object's one field is a shared handle, so the copy handed out is the same instance's data)
        return INSTANCE.get_mut().get_or_insert_with(SpeedPresets::new).clone();
    }

    /// java: public static CommandNode<FabricClientCommandSource> getCommandNode()
    pub fn get_command_node() -> crate::de::hysky::skyblocker::skyblock::viewstash_autocomplete::ClientCommandNode {
        use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
        use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
        use mcrust::mcrust::modding::events::client_command_registration_callback::{self, FabricClientCommandSource};
        //   java: return ClientCommands.literal("setmaxspeed")
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(ClientCommands.argument("preset", StringArgumentType.string())
        //   java: 						.suggests((_, builder) -> {
        //   java: 							if (SkyblockerConfigManager.get().general.speedPresets.enableSpeedPresets && getInstance().presets.isLoaded()) {
        //   java: 								return SharedSuggestionProvider.suggest(instance.getPresets().keySet(), builder);
        //   java: 							}
        //   java: 							return builder.buildFuture();
        //   java: 						}).executes(CommandUtils.noOp)).build();
        let mut preset = client_command_registration_callback::argument("preset", Arc::new(StringArgumentType::string()));
        preset.suggests(Some(Arc::new(|_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
            let instance = SpeedPresets::get_instance();
            if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.speed_presets.enable_speed_presets && instance.presets.lock().borrow().is_loaded() {
                let keys: Vec<String> = instance.get_presets().keys().cloned().collect();
                return Ok(mcrust::net::minecraft::commands::shared_suggestion_provider::suggest(&keys, builder));
            }
            Ok(builder.build_future())
        })));
        preset.executes(crate::de::hysky::skyblocker::utils::command::command_utils::no_op());
        let mut node = client_command_registration_callback::literal("setmaxspeed");
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(crate::de::hysky::skyblocker::utils::command::command_utils::no_op());
        node.then(&preset);
        return node.build();
    }

    /// java: @Init public static void init()
    pub fn init() {
        let mut instance: SpeedPresets = SpeedPresets::get_instance();
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> instance.presets.init());
        // TODO(fabric2seer): ClientSendMessageEvents.MODIFY_COMMAND.register: no SeerClient equivalent yet
        //   java: ClientSendMessageEvents.MODIFY_COMMAND.register(command -> {
        //   java: 			Matcher matcher = COMMAND_PATTERN.matcher(command);
        //   java: 			if (matcher.matches() && SkyblockerConfigManager.get().general.speedPresets.enableSpeedPresets) {
        //   java: 				String preset = matcher.group(1);
        //   java: 				if (instance.hasPreset(preset)) {
        //   java: 					return String.format("setmaxspeed %d", instance.getPreset(preset));
        //   java: 				}
        //   java: 			}
        //   java: 			return command;
        //   java: 		});
    }

    /// java: public static Object2IntOpenHashMap<String> createDefaultMap()
    pub fn create_default_map() -> std::collections::HashMap<String, i32> {
        let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        map.insert("default".to_string(), 100);
        map.insert("crops".to_string(), 93);
        map.insert("cocoa".to_string(), 155);
        map.insert("mushroom".to_string(), 233);
        map.insert("cane".to_string(), 327);
        map.insert("squash".to_string(), 327);
        map.insert("cactus".to_string(), 464);
        return map;
    }

    /// java: public boolean hasPreset(String name)
    pub fn has_preset(&self, name: &str) -> bool {
        return self.get_presets().contains_key(name);
    }

    /// java: public int getPreset(String name)
    pub fn get_preset(&self, name: &str) -> i32 {
        return self.get_presets().get(name).cloned().unwrap_or_else(|| 0);
    }

    /// java: public void setPreset(String name, int value)
    pub fn set_preset(&self, mut name: String, mut value: i32) {
        self.get_presets().insert(name.clone(), value);
        self.presets.lock().borrow().save();
    }

    /// java: public boolean arePresetsEqual(Map<String, Integer> presets)
    pub fn are_presets_equal(&self, presets: &std::collections::HashMap<String, i32>) -> bool {
        if (presets.len() as i32) != (self.get_presets().len() as i32) {
            return false;
        }
        for entry in presets.iter().map(|(k, v)| (k.clone(), v.clone())) {
            if !(self.get_presets().contains_key(&entry.0)) || self.get_presets().get(&entry.0).copied().unwrap_or_default() != entry.1 {
                return false;
            }
        }
        return true;
    }

    /// java: public Object2IntMap<String> getPresets()
    pub fn get_presets(&self) -> std::collections::HashMap<String, i32> {
        return self.presets.lock().borrow().get_data();
    }

    /// java: public CompletableFuture<Void> savePresets()
    pub fn save_presets(&self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        return self.presets.lock().borrow().save();
    }

}
