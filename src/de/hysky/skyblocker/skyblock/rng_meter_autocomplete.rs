//! Ported from de.hysky.skyblocker.skyblock.RngMeterAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: private static final Codec<Map<String, List<String>>> CODEC = Codec.unboundedMap(Codec.STRING, Codec.STRING.listOf());
/// java: private static final Codec<Map<String, List<String>>> CODEC = Codec.unboundedMap(Codec.STRING, Codec.STRING.listOf());
pub static CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, Vec<String>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: private static Map<String, List<String>> rngMeters = new HashMap<>();
pub static RNG_METERS: LazyLock<Mutex<std::collections::HashMap<String, Vec<String>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> longCommand;
pub static LONG_COMMAND: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> shortCommand;
pub static SHORT_COMMAND: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);

/// java: public class RngMeterAutocomplete
#[derive(Clone)]
pub struct RngMeterAutocomplete {}

impl RngMeterAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CompletableFuture::<()>::run_async(move || (|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                //   java: String json = Http.sendGetRequest("https://hysky.de/api/rngmeters");
                let mut json: String = todo!("fabric2seer: the initialiser of `json` (TODO above)");
                compat::set_state(&RNG_METERS, (CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&json).expect("JsonSyntaxException")))).get_or_throw().unwrap());
                *LONG_COMMAND.get_mut() = Some(RngMeterAutocomplete::create_command_node("rngmeter"));
                *SHORT_COMMAND.get_mut() = Some(RngMeterAutocomplete::create_command_node("rng"));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker RNG Meter Autocomplete] Failed to load RNG Meter data."));
            }
        })(), compat::executor);
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createCommandNode(String command)
    pub fn create_command_node(command: &str) -> LiteralCommandNode {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("type", StringArgumentType.string())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(rngMeters.keySet(), builder))
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 						.then(argument("subtype", StringArgumentType.string())
        //   java: 								.suggests((context, builder) -> SharedSuggestionProvider.suggest(rngMeters.getOrDefault(StringArgumentType.getString(context, "type"), List.of()), builder))
        //   java: 								.executes(CommandUtils.noOp)
        //   java: 						)
        //   java: 				).build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
