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
            presets: (compat::shared(JsonDataBase::new(PRESETS_FILE.get_mut().clone(), MAP_CODEC.get_mut().clone(), SpeedPresets::create_default_map())) as compat::Shared<dyn JsonData<compat::Object>>),
        };
        this
    }

    /// java: public static SpeedPresets getInstance()
    pub fn get_instance() -> SpeedPresets {
        // TODO(fabric2seer): not translated (expected ':' at '=' (offset 1778))
        //   java: return instance == null ? instance = new SpeedPresets() : instance;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static CommandNode<FabricClientCommandSource> getCommandNode()
    pub fn get_command_node() -> CommandNode<crate::unresolved::FabricClientCommandSource> {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
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
        todo!("fabric2seer: the value of this function is in a TODO above")
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
