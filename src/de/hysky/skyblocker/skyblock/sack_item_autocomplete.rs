//! Ported from de.hysky.skyblocker.skyblock.SackItemAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;

/// java: private static final Pattern BAD_CHARACTERS = Pattern.compile(String.format("[%s%s%s%s%s%s%s%s%s%s%s%s]", SkyBlockIcons.MINING_FORTUNE, SkyBlockIcons.MINING_SPEED, SkyBlockIcons.PRISTINE, SkyBlockIcon
pub static BAD_CHARACTERS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("[{}{}{}{}{}{}{}{}{}{}{}{}]", sky_block_icons::MINING_FORTUNE, sky_block_icons::MINING_SPEED, sky_block_icons::PRISTINE, sky_block_icons::INTELLIGENCE, sky_block_icons::DEFENSE, sky_block_icons::STRENGTH, sky_block_icons::HEALTH, sky_block_icons::TRUE_DEFENSE, sky_block_icons::CRIT_DAMAGE, sky_block_icons::FISHING_SPEED, sky_block_icons::FORAGING_FORTUNE, sky_block_icons::FARMING_FORTUNE)));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> longCommandNode;
pub static LONG_COMMAND_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> shortCommandNode;
pub static SHORT_COMMAND_NODE: compat::MainThread<Option<LiteralCommandNode>> = compat::MainThread::new(|| None);

/// java: public class SackItemAutocomplete
#[derive(Clone)]
pub struct SackItemAutocomplete {}

impl SackItemAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || { SackItemAutocomplete::load_sack_items(); }));
    }

    /// java: private static void loadSackItems()
    pub fn load_sack_items() {
        {
            // TODO(fabric2seer): `.stream(..)` on a NEURepoFile
            //   java: InputStream stream = NEURepoManager.file(..).stream(..);
            let mut stream: crate::unresolved::InputStream = todo!("fabric2seer: the initialiser of `stream` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.readAllBytes(..)` on a InputStream
                //   java: JsonObject sacks = JsonParser.parseString(new String(stream.readAllBytes())).getAsJsonObject().getAsJsonObject("sacks");
                let mut sacks: serde_json::Map<String, serde_json::Value> = todo!("fabric2seer: the initialiser of `sacks` (TODO above)");
                // TODO(fabric2seer): `getAsString` on a Object
                //   java: Set<String> sackItemIds = sacks.entrySet().stream()
                //   java: 					.map(entry -> entry.getValue().getAsJsonObject())
                //   java: 					.map(sack -> sack.getAsJsonArray("contents"))
                //   java: 					.map(JsonArray::asList)
                //   java: 					.flatMap(List::stream)
                //   java: 					.map(JsonElement::getAsString)
                //   java: 					.collect(Collectors.toUnmodifiableSet());
                let mut sack_item_ids: std::collections::HashSet<String> = todo!("fabric2seer: the initialiser of `sackItemIds` (TODO above)");
                let mut sack_items: std::collections::HashSet<String> = sack_item_ids.iter().cloned().map(|neu_id| {
                    let mut stack: Option<crate::unresolved::NEUItem> = NEURepoManager::get_item_by_neu_id(&neu_id);
                    // java: return stack != null ? ChatFormatting.stripFormatting(stack.getDisplayName()) : neuId;
                    // rust: port gap - `NEUItem.getDisplayName` (the NEU repo parser is a `crate::unresolved` placeholder,
                    // GAPS_batchB L1): a found item's display name cannot be read, so its id stands in
                    let _ = stack;
                    return neu_id.clone();
                }).map(|name| compat::trim(&BAD_CHARACTERS.matcher(&compat::jstring(&name)).replace_all(""))).collect::<std::collections::HashSet<_>>();
                *LONG_COMMAND_NODE.get_mut() = Some(SackItemAutocomplete::create_command_node("getfromsacks", &sack_items));
                *SHORT_COMMAND_NODE.get_mut() = Some(SackItemAutocomplete::create_command_node("gfs", &sack_items));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Sack Item Autocomplete] Failed to load sacks data from the NEU Repo."));
            }
        }
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createCommandNode(String command, Set<String> sackItems)
    pub fn create_command_node(command: &str, sack_items: &std::collections::HashSet<String>) -> LiteralCommandNode {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("item", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(sackItems, builder))
        //   java: 						.then(argument("amount", IntegerArgumentType.integer(0))) // Adds a nice <amount> text to the suggestion when any number is entered after the item string
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 				)
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
