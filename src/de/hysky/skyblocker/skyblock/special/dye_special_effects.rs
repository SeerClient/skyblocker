//! Ported from de.hysky.skyblocker.skyblock.special.DyeSpecialEffects by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::special::special_effects::SpecialEffects;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: @VisibleForTesting // WOW! (?:\[[A-Z+]+] )?(?<player>[A-Za-z0-9_]+) found (?:a|an) (?<dye>[A-Za-z ]+ Dye)(?: #[\d,]+)?! protected static final Pattern DROP_PATTERN = Pattern.compile("WOW! (?:\\[[A-Z+]
pub static DROP_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("WOW! (?:\\[[A-Z+]+] )?(?<player>[A-Za-z0-9_]+) found (?:a|an) (?<dye>[A-Za-z ]+ Dye)(?: #[\\d,]+)?!"));

/// java: public class DyeSpecialEffects
#[derive(Clone)]
pub struct DyeSpecialEffects {}

impl DyeSpecialEffects {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(DyeSpecialEffects::displayDyeDropEffect);
    }

    /// java: private static boolean displayDyeDropEffect(Component message, boolean overlay)
    pub fn display_dye_drop_effect(message: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_on_skyblock() && SkyblockerConfigManager::get().general.special_effects.rare_dye_drop_effects && !overlay {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut string_form: String = message.get_string();
                let mut matcher: compat::regex::Matcher = DROP_PATTERN.matcher(&string_form);
                if matcher.matches() && matcher.group_named("player").as_deref() == Some(&*(compat::minecraft().get_user().get_name())) {
                    let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = DyeSpecialEffects::find_dye_stack(matcher.group_named("dye").as_deref().unwrap());
                    if !stack.is_empty() {
                        SpecialEffects::display_special_effect(&stack, mcrust::net::minecraft::core::particles::particle_types::trial_spawner_detected_player());
                    }
                }
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Special Effects] An unexpected exception was encountered!"));
            }
        }
        return true;
    }

    /// java: private static ItemStack findDyeStack(String dyeName)
    pub fn find_dye_stack(dye_name: &str) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        //   java: Optional<FlexibleItemStack> dye = ItemRepository.getItemsStream()
        //   java: 				.filter(stack -> stack.getOrDefault(DataComponents.CUSTOM_NAME, Component.empty()).getString().equals(dyeName))
        //   java: 				.findFirst();
        // (`getOrDefault(CUSTOM_NAME, Component.empty())`: the erased component read back as the `FormattedText` it is)
        let mut dye: Option<compat::Shared<FlexibleItemStack>> = crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository::get_items_stream()
            .find(|stack| FlexibleItemStack::get::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(stack, mcrust::net::minecraft::core::component::data_components::custom_name()).map(|name| name.get_string()).unwrap_or_default() == dye_name);
        return dye.clone().and_then(|__a0| FlexibleItemStack::get_stack(&__a0)).unwrap_or(mcrust::net::minecraft::world::item::item_stack::empty());
    }

}
