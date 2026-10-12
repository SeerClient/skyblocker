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
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(ClientCommands.literal("disableAll")
                //   java: 						.executes(DisableAll::confirmMessage)
                //   java: 						.then(ClientCommands.literal("confirm")
                //   java: 								.executes(DisableAll::disableAll)))
                //   java: 		);
            },
        ));
    }

    /// java: private static void registerCommand(CommandDispatcher<FabricClientCommandSource> dispatcher, net.minecraft.commands.CommandBuildContext registryAccess)
    pub fn register_command(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(ClientCommands.literal("disableAll")
        //   java: 						.executes(DisableAll::confirmMessage)
        //   java: 						.then(ClientCommands.literal("confirm")
        //   java: 								.executes(DisableAll::disableAll)))
        //   java: 		);
    }

    /// java: private static int confirmMessage(CommandContext<FabricClientCommandSource> context)
    pub fn confirm_message(context: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        compat::set_state(&CONFIRM_ALLOWED_UNTIL, (compat::millis()).wrapping_add(CONFIRM_TIMEOUT));
        let mut confirm: MutableComponent = component::translatable("skyblocker.disableAll.confirmYes").with_style_format(ChatFormatting::Red).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command(format!("/{} disableAll confirm", skyblocker_mod::NAMESPACE)))));
        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
        //   java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.confirm", confirm)));
        return 1;
    }

    /// java: @SuppressWarnings("SameReturnValue") private static int disableAll(CommandContext<FabricClientCommandSource> context)
    pub fn disable_all(context: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        if compat::millis() > compat::get_state(&CONFIRM_ALLOWED_UNTIL) {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: context.getSource().sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.notPending").withStyle(ChatFormatting.RED)));
            return 1;
        }
        compat::set_state(&CONFIRM_ALLOWED_UNTIL, ((0) as i64) as i64);
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            DisableAll::disable_all_0();
            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
            //   java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.success").withStyle(ChatFormatting.RED)));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker DisableAll] Failed to disable all features"));
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: context.getSource().sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.disableAll.failed").withStyle(ChatFormatting.RED)));
        }
        return 1;
    }

    /// java: protected static void disableAll()
    pub fn disable_all_0() {
        SkyblockerConfigManager::update(Arc::new(move |config| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                DisableAll::disable_entries(&config);
            }));
            if let Err(e) = __result {
                panic!("RuntimeException: {}", compat::panic_message(&e));
            }
        }));
        WidgetManager::remove_all();
        WidgetManager::save_config();
    }

    /// java: protected static void disableEntries(Object target) throws IllegalAccessException
    pub fn disable_entries(target: &compat::Object) {
        // TODO(fabric2seer): `.getDeclaredFields(..)` on a Class
        //   java: for (Field field : target.getClass().getDeclaredFields()) {
        //   java: 			if (Modifier.isStatic(field.getModifiers())) continue;
        //   java: 			field.setAccessible(true);
        //   java: 			Object value = field.get(target);
        //   java: 			Class<?> type = field.getType();
        //   java: 			if (type == boolean.class) {
        //   java: 				field.setBoolean(target, false);
        //   java: 			} else if (type == Boolean.class) {
        //   java: 				field.set(target, false);
        //   java: 			} else if (value instanceof Map<?, ?>) {
        //   java: 				@SuppressWarnings("unchecked")
        //   java: 				Map<Object, Object> m = (Map<Object, Object>) value;
        //   java: 				for (Map.Entry<Object, Object> entry : m.entrySet()) {
        //   java: 					if (entry.getValue() instanceof Boolean) {
        //   java: 						m.put(entry.getKey(), Boolean.FALSE);
        //   java: 					}
        //   java: 				}
        //   java: 			} else if (type.isEnum()) {
        //   java: 				Field[] declaredFields = type.getDeclaredFields();
        //   java: 				if (declaredFields.length == 0) continue;
        //   java: 				Optional<Field> option = Arrays.stream(declaredFields).filter(f -> f.getAnnotation(EnumDisabledValue.class) != null).findFirst();
        //   java: 				if (option.isEmpty()) continue;
        //   java: 				field.set(target, option.get().get(type));
        //   java: 			} else if (value != null && SkyblockerConfigManager.isConfigClass(type)) {
        //   java: 				disableEntries(value);
        //   java: 			}
        //   java: 		}
    }

}
