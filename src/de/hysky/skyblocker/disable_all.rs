//! Ported from de.hysky.skyblocker.DisableAll by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final long CONFIRM_TIMEOUT = 30_000L;
pub const CONFIRM_TIMEOUT: i64 = 30000;
/// java: private static long confirmAllowedUntil;
pub static CONFIRM_ALLOWED_UNTIL: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));

/// java: public class DisableAll
#[derive(Clone)]
pub struct DisableAll {}

impl DisableAll {
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
                // java: ClientCommandRegistrationCallback.EVENT.register(DisableAll::registerCommand) - `registerCommand`'s tree:
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(ClientCommands.literal("disableAll")
                //   java: 						.executes(DisableAll::confirmMessage)
                //   java: 						.then(ClientCommands.literal("confirm")
                //   java: 								.executes(DisableAll::disableAll)))
                //   java: 		);
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut confirm = client_command_registration_callback::literal("confirm");
                confirm.executes(Arc::new(|context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| Ok(DisableAll::disable_all(context))));
                let mut disable_all = client_command_registration_callback::literal("disableAll");
                disable_all.executes(Arc::new(|context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| Ok(DisableAll::confirm_message(context))));
                disable_all.then(&confirm);
                let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                root.then(&disable_all);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: private static void registerCommand(CommandDispatcher<FabricClientCommandSource> dispatcher, net.minecraft.commands.CommandBuildContext registryAccess)
    pub fn register_command(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // (the tree is built and registered by `init`'s client command listener - the dispatcher this signature names is
        // the event's)
        let _ = (dispatcher, registry_access);
    }

    /// java: private static int confirmMessage(CommandContext<FabricClientCommandSource> context)
    pub fn confirm_message(context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>) -> i32 {
        compat::set_state(&CONFIRM_ALLOWED_UNTIL, (compat::millis()).wrapping_add(CONFIRM_TIMEOUT));
        let mut confirm: MutableComponent = component::translatable("skyblocker.disableAll.confirmYes").with_style_format(ChatFormatting::Red).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command(format!("/{} disableAll confirm", skyblocker_mod::NAMESPACE)))));
        // java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.confirm", confirm)));
        compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable_args("skyblocker.disableAll.confirm", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(confirm.clone())])));
        return 1;
    }

    /// java: @SuppressWarnings("SameReturnValue") private static int disableAll(CommandContext<FabricClientCommandSource> context)
    pub fn disable_all(context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>) -> i32 {
        if compat::millis() > compat::get_state(&CONFIRM_ALLOWED_UNTIL) {
            // java: context.getSource().sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.notPending").withStyle(ChatFormatting.RED)));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable("skyblocker.disableAll.notPending").with_style_format(ChatFormatting::Red)));
            return 1;
        }
        compat::set_state(&CONFIRM_ALLOWED_UNTIL, ((0) as i64) as i64);
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            DisableAll::disable_all_0();
            // java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.success").withStyle(ChatFormatting.RED)));
            compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable("skyblocker.disableAll.success").with_style_format(ChatFormatting::Red)));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker DisableAll] Failed to disable all features"));
            // java: context.getSource().sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.failed").withStyle(ChatFormatting.RED)));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable("skyblocker.disableAll.failed").with_style_format(ChatFormatting::Red)));
        }
        return 1;
    }

    /// java: protected static void disableAll()
    pub fn disable_all_0() {
        SkyblockerConfigManager::update(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                DisableAll::disable_entries(&mut *config);
            }));
            if let Err(e) = __result {
                panic!("RuntimeException: {}", compat::panic_message(&e));
            }
        }));
        WidgetManager::remove_all();
        WidgetManager::save_config();
    }

    /// java: protected static void disableEntries(Object target) throws IllegalAccessException
    ///
    /// Java walks `target.getClass().getDeclaredFields()` by reflection. The config classes carry their reflective
    /// view here (`__gson_write` - the fields in declaration order - and `__gson_patch`, Dandelion's
    /// `field.set` by a field path, which answers false where a path is not a field of a config class), so the walk
    /// runs over the same fields:
    /// * a `boolean` / `Boolean` field: set to false (`patch`);
    /// * a `Map` whose values are `Boolean`: every entry set to `FALSE` - a field whose entries are not fields (their
    ///   `patch` fails) and are all booleans; written into the config's own tree and read back;
    /// * an enum field whose enum has an `@EnumDisabledValue` constant: set to that constant;
    /// * a config class: walked (a nested object that is not one - a map's values, a `Component`, a `Color` - has
    ///   no field a `patch` reaches, so nothing in it changes).
    pub fn disable_entries(target: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig) {
        use dandelion::compat::{GsonValue, PatchValue};
        /// the enum fields of the config classes whose type carries `@EnumDisabledValue`, by (declaring object key,
        /// field) - `Criterion.NONE`, `Craft.OFF`, `FishingHookDisplay.OFF`, `CommissionWaypointMode.OFF`,
        /// `HighlightSlayerEntities.OFF` (two fields), `FirePillar.OFF`
        fn enum_disabled_value(parent: Option<&str>, field: &str) -> Option<&'static str> {
            match (parent, field) {
                (Some("eventNotifications"), "criterion") => Some("NONE"),
                (_, "enableCraftingCost") => Some("OFF"),
                (_, "fishingHookDisplay") => Some("OFF"),
                (Some("commissionWaypoints"), "mode") => Some("OFF"),
                (Some("slayers"), "highlightMinis") | (Some("slayers"), "highlightBosses") => Some("OFF"),
                (_, "firePillarCountdown") => Some("OFF"),
                _ => None,
            }
        }
        fn walk(config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig, node: &GsonValue, path: &mut Vec<String>, boolean_maps: &mut Vec<Vec<String>>) {
            let GsonValue::Obj(fields) = node else { return };
            for (key, value) in fields {
                path.push(key.clone());
                let refs: Vec<&str> = path.iter().map(String::as_str).collect();
                match value {
                    GsonValue::Bool(_) => {
                        config.__gson_patch(&refs, &PatchValue::Bool(false));
                    }
                    GsonValue::Str(_) => {
                        let parent: Option<&str> = refs.len().checked_sub(2).map(|i| refs[i]);
                        if let Some(disabled) = enum_disabled_value(parent, key) {
                            config.__gson_patch(&refs, &PatchValue::Enum(disabled.to_string()));
                        }
                    }
                    GsonValue::Obj(entries) => {
                        let all_booleans: bool = !entries.is_empty() && entries.iter().all(|(_, v)| matches!(v, GsonValue::Bool(_)));
                        let first_is_field: bool = entries.first().is_some_and(|(k, _)| {
                            let mut probe: Vec<&str> = refs.clone();
                            probe.push(k.as_str());
                            config.__gson_patch(&probe, &PatchValue::Bool(false))
                        });
                        if all_booleans && !first_is_field {
                            // `value instanceof Map<?, ?>` with Boolean values
                            boolean_maps.push(path.clone());
                        } else {
                            walk(config, value, path, boolean_maps);
                        }
                    }
                    _ => {}
                }
                path.pop();
            }
        }
        let tree: GsonValue = target.__gson_write();
        let mut boolean_maps: Vec<Vec<String>> = Vec::new();
        walk(target, &tree, &mut Vec::new(), &mut boolean_maps);
        if boolean_maps.is_empty() {
            return;
        }
        // `m.put(entry.getKey(), Boolean.FALSE)` for every Boolean entry: on the config's own tree, read back
        let mut json: serde_json::Value = serde_json::from_str(&dandelion::compat::gson_text(&target.__gson_write(), false)).expect("the config's Gson form is JSON");
        for map_path in &boolean_maps {
            let mut node: &mut serde_json::Value = &mut json;
            for segment in map_path {
                node = &mut node[segment.as_str()];
            }
            if let Some(map) = node.as_object_mut() {
                for (_, entry) in map.iter_mut() {
                    if entry.is_boolean() {
                        *entry = serde_json::Value::Bool(false);
                    }
                }
            }
        }
        if let Some(read) = crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig::__gson_read(&json) {
            *target = read;
        }
    }

}
