//! Ported from de.hysky.skyblocker.skyblock.SackItemAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::command_utils::no_op;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::brigadier::tree::command_node::CommandNode;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::commands::shared_suggestion_provider;

/// java: private static final Pattern BAD_CHARACTERS = Pattern.compile(String.format("[%s%s%s%s%s%s%s%s%s%s%s%s]", SkyBlockIcons.MINING_FORTUNE, SkyBlockIcons.MINING_SPEED, SkyBlockIcons.PRISTINE, SkyBlockIcon
pub static BAD_CHARACTERS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("[{}{}{}{}{}{}{}{}{}{}{}{}]", sky_block_icons::MINING_FORTUNE, sky_block_icons::MINING_SPEED, sky_block_icons::PRISTINE, sky_block_icons::INTELLIGENCE, sky_block_icons::DEFENSE, sky_block_icons::STRENGTH, sky_block_icons::HEALTH, sky_block_icons::TRUE_DEFENSE, sky_block_icons::CRIT_DAMAGE, sky_block_icons::FISHING_SPEED, sky_block_icons::FORAGING_FORTUNE, sky_block_icons::FARMING_FORTUNE)));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> longCommandNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static LONG_COMMAND_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));
/// java: public static @Nullable LiteralCommandNode<FabricClientCommandSource> shortCommandNode;
/// (written by the async loader, read by `ClientboundCommandsPacketMixin`: a thread-safe cell)
pub static SHORT_COMMAND_NODE: LazyLock<Mutex<Option<ClientCommandNode>>> = LazyLock::new(|| Mutex::new(None));

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
            // java: try (InputStream stream = NEURepoManager.file("constants/sacks.json").stream()) {..} catch (Exception e) - an
            // exception here (a missing file is a null `file(..)`) is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let file: crate::io::github::moulberry::repo::neu_repo_file::NEURepoFile = NEURepoManager::file("constants/sacks.json").expect("NullPointerException: NEURepoManager.file(\"constants/sacks.json\")");
                //   java: JsonObject sacks = JsonParser.parseString(new String(stream.readAllBytes())).getAsJsonObject().getAsJsonObject("sacks");
                let content: String = compat::neu_repo_file_read_all(&file);
                let mut sacks: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&serde_json::from_str::<serde_json::Value>(&content).expect("JsonSyntaxException"), "sacks");
                //   java: Set<String> sackItemIds = sacks.entrySet().stream()
                //   java: 					.map(entry -> entry.getValue().getAsJsonObject())
                //   java: 					.map(sack -> sack.getAsJsonArray("contents"))
                //   java: 					.map(JsonArray::asList)
                //   java: 					.flatMap(List::stream)
                //   java: 					.map(JsonElement::getAsString)
                //   java: 					.collect(Collectors.toUnmodifiableSet());
                let mut sack_item_ids: std::collections::HashSet<String> = sacks.iter()
                    .map(|(_, sack)| compat::json::get_as_json_object(sack))
                    .map(|sack| compat::json::get_as_json_array_of(&sack, "contents"))
                    .flat_map(|contents| contents.into_iter())
                    .map(|element| compat::json::get_as_string(&element))
                    .collect();
                let mut sack_items: std::collections::HashSet<String> = sack_item_ids.iter().cloned().map(|neu_id| {
                    let mut stack: Option<Arc<crate::io::github::moulberry::repo::data::neu_item::NEUItem>> = NEURepoManager::get_item_by_neu_id(&neu_id);
                    // java: return stack != null ? ChatFormatting.stripFormatting(stack.getDisplayName()) : neuId;
                    return match stack {
                        Some(stack) => mcrust::net::minecraft::chat_formatting::strip_formatting(stack.get_display_name()),
                        None => neu_id.clone(),
                    };
                }).map(|name| compat::trim(&BAD_CHARACTERS.matcher(&compat::jstring(&name)).replace_all(""))).collect::<std::collections::HashSet<_>>();
                compat::set_state(&LONG_COMMAND_NODE, Some(SackItemAutocomplete::create_command_node("getfromsacks", &sack_items)));
                compat::set_state(&SHORT_COMMAND_NODE, Some(SackItemAutocomplete::create_command_node("gfs", &sack_items)));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Sack Item Autocomplete] Failed to load sacks data from the NEU Repo."));
            }
        }
    }

    /// java: private static LiteralCommandNode<FabricClientCommandSource> createCommandNode(String command, Set<String> sackItems)
    pub fn create_command_node(command: &str, sack_items: &std::collections::HashSet<String>) -> ClientCommandNode {
        //   java: return literal(command)
        //   java: 				.requires(_ -> Utils.isOnSkyblock())
        //   java: 				.executes(CommandUtils.noOp)
        //   java: 				.then(argument("item", StringArgumentType.greedyString())
        //   java: 						.suggests((_, builder) -> SharedSuggestionProvider.suggest(sackItems, builder))
        //   java: 						.then(argument("amount", IntegerArgumentType.integer(0))) // Adds a nice <amount> text to the suggestion when any number is entered after the item string
        //   java: 						.executes(CommandUtils.noOp)
        //   java: 				)
        //   java: 				.build();
        let sack_items: Vec<String> = sack_items.iter().cloned().collect();
        let amount = client_command_registration_callback::argument("amount", Arc::new(mcrust::com::mojang::brigadier::arguments::integer_argument_type::IntegerArgumentType::integer_min(0)));
        let mut item = client_command_registration_callback::argument("item", Arc::new(StringArgumentType::greedy_string()));
        item.suggests(Some(Arc::new(move |_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| Ok(shared_suggestion_provider::suggest(&sack_items, builder)))));
        item.then(&amount);
        item.executes(no_op());
        let mut node = client_command_registration_callback::literal(command);
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&item);
        return node.build();
    }

}

/// `LiteralCommandNode<FabricClientCommandSource>`: the port's `ArgumentBuilder.build()` answers the node as a
/// `CommandNode` (its `NodeKind::Literal`), the type `ClientboundCommandsPacketMixin` hands back.
pub type ClientCommandNode = Arc<CommandNode<FabricClientCommandSource>>;

