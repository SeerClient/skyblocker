//! Ported from de.hysky.skyblocker.skyblock.WarpAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::command_utils::no_op;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::brigadier::tree::command_node::CommandNode;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::commands::shared_suggestion_provider;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("warp_autocomplete.json");
/// (read on the executor's thread: a `LazyLock`)
pub static FILE: LazyLock<std::path::PathBuf> = LazyLock::new(|| skyblocker_mod::CONFIG_DIR.get().join("warp_autocomplete.json"));
/// java: private static final Codec<Object2BooleanMap<String>> MAP_CODEC = CodecUtils.object2BooleanMapCodec(Codec.STRING);
/// (read on the executor's thread: a codec is `Send + Sync`, so a `LazyLock`)
pub static MAP_CODEC: LazyLock<CodecRef<std::collections::HashMap<String, bool>>> = LazyLock::new(|| CodecUtils::object2_boolean_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> commandNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static COMMAND_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));

/// java: public class WarpAutocomplete
#[derive(Clone)]
pub struct WarpAutocomplete {}

impl WarpAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CompletableFuture::<()>::run_async(move || (|| {
            //   java: Object2BooleanMap<String> warps = Object2BooleanMaps.<String>emptyMap();
            let mut warps: std::collections::HashMap<String, bool> = std::collections::HashMap::new();
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                //   java: String warpsString = Http.sendGetRequest("https://hysky.de/api/locations");
                let mut warps_string: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://hysky.de/api/locations");
                warps = (MAP_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&warps_string).expect("JsonSyntaxException")))).get_or_throw().unwrap();
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to download warps list {}", compat::panic_message(&e)));
            }
            if warps.is_empty() {
                warps = WarpAutocomplete::get_warps_from_file();
            }
            WarpAutocomplete::create_command_node(&warps);
            WarpAutocomplete::save_warps_to_file(&warps);
        })(), compat::executor);
    }

    /// java: private static Object2BooleanMap<String> getWarpsFromFile()
    pub fn get_warps_from_file() -> std::collections::HashMap<String, bool> {
        //   java: try (BufferedReader reader = Files.newBufferedReader(FILE)) { object = SkyblockerMod.GSON.fromJson(reader, JsonObject.class); }
        //   java: catch (NoSuchFileException _) { return Object2BooleanMaps.emptyMap(); }
        //   java: catch (Exception e) { LOGGER.error("[Skyblocker] Failed to read warp autocomplete file", e); return Object2BooleanMaps.emptyMap(); }
        let object: serde_json::Value = match std::fs::read_to_string(&*FILE) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return std::collections::HashMap::new(),
            Err(e) => {
                logger().error(&format!("[Skyblocker] Failed to read warp autocomplete file {}", e));
                return std::collections::HashMap::new();
            }
            Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(object) => object,
                Err(e) => {
                    logger().error(&format!("[Skyblocker] Failed to read warp autocomplete file {}", e));
                    return std::collections::HashMap::new();
                }
            },
        };
        //   java: return MAP_CODEC.parse(JsonOps.INSTANCE, object).result().orElse(Object2BooleanMaps.emptyMap());
        return MAP_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&object)).result().unwrap_or_default();
    }

    /// java: private static void saveWarpsToFile(Object2BooleanMap<String> warps)
    pub fn save_warps_to_file(warps: &std::collections::HashMap<String, bool>) {
        let mut result: Option<serde_json::Value> = (MAP_CODEC.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), warps)).result().map(|o| compat::json_from_port(&o));
        if result.is_none() {
            return;
        }
        let mut warps_json: serde_json::Value = result.clone().unwrap();
        //   java: try (BufferedWriter writer = Files.newBufferedWriter(FILE, StandardCharsets.UTF_8)) { SkyblockerMod.GSON.toJson(warpsJson, writer); }
        //   java: catch (Exception e) { LOGGER.error("[Skyblocker] Failed to save warps auto complete", e); }
        let written: Result<(), String> = serde_json::to_string_pretty(&warps_json).map_err(|e| e.to_string()).and_then(|text| std::fs::write(&*FILE, text).map_err(|e| e.to_string()));
        if let Err(e) = written {
            logger().error(&format!("[Skyblocker] Failed to save warps auto complete {}", e));
        }
    }

    /// java: private static void createCommandNode(Object2BooleanMap<String> warps)
    pub fn create_command_node(warps: &std::collections::HashMap<String, bool>) {
        //   java: commandNode = literal("warp")
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("destination", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(getEligibleWarps(warps), builder))
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 				).build();
        let warps: std::collections::HashMap<String, bool> = warps.clone();
        let mut destination = client_command_registration_callback::argument("destination", Arc::new(StringArgumentType::greedy_string()));
        destination.suggests(Some(Arc::new(move |_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
            let eligible: Vec<String> = WarpAutocomplete::get_eligible_warps(&warps).collect();
            Ok(shared_suggestion_provider::suggest(&eligible, builder))
        })));
        destination.executes(no_op());
        let mut node = client_command_registration_callback::literal("warp");
        node.requires(Arc::new(|_: &FabricClientCommandSource| Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&destination);
        compat::set_state(&COMMAND_NODE, Some(node.build()));
    }

    /// java: private static Stream<String> getEligibleWarps(Object2BooleanMap<String> warps)
    pub fn get_eligible_warps(warps: &std::collections::HashMap<String, bool>) -> Box<dyn Iterator<Item = String> + '_> {
        //   java: return warps.object2BooleanEntrySet().stream()
        //   java: 				.filter(WarpAutocomplete::shouldShowWarp)
        //   java: 				.map(Object2BooleanMap.Entry::getKey);
        return Box::new(warps.iter().map(|(key, value)| (key.clone(), *value)).filter(|entry| WarpAutocomplete::should_show_warp(entry)).map(|entry| entry.0));
    }

    /// java: private static boolean shouldShowWarp(Object2BooleanMap.Entry<String> entry)
    pub fn should_show_warp(entry: &(String, bool)) -> bool {
        //   java: return !entry.getBooleanValue() || Utils.isOnBingo() || RankType.compare(Utils.getRank(), PackageRank.MVP_PLUS) >= 0;
        return !entry.1 || Utils::is_on_bingo() || crate::unresolved::RankType::compare(*Utils::get_rank(), crate::unresolved::RankType::Package(crate::unresolved::PackageRank::MvpPlus)) >= 0;
    }

}

/// `LiteralCommandNode<FabricClientCommandSource>`: the port's `ArgumentBuilder.build()` answers the node as a
/// `CommandNode` (its `NodeKind::Literal`), the type `ClientboundCommandsPacketMixin` hands back.
pub type ClientCommandNode = Arc<CommandNode<FabricClientCommandSource>>;

