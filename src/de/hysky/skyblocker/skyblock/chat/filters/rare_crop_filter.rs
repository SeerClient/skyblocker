//! Ported from de.hysky.skyblocker.skyblock.chat.filters.RareCropFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::render::gui::basic_toast::BasicToast;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final Map<String, String> IDS = Map.ofEntries( entry("Cropie", "CROPIE"), entry("Squash", "SQUASH"), entry("Fermento", "FERMENTO"), entry("Helianthus", "HELIANTHUS"), entry("Warty", "WA
pub static IDS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("Cropie".to_string(), "CROPIE".to_string()), ("Squash".to_string(), "SQUASH".to_string()), ("Fermento".to_string(), "FERMENTO".to_string()), ("Helianthus".to_string(), "HELIANTHUS".to_string()), ("Warty".to_string(), "WARTY".to_string()), ("Burrowing Spores".to_string(), "BURROWING_SPORES".to_string()), ("Overclocker 3000".to_string(), "OVERCLOCKER_3000".to_string()), ("Ethereal Vine".to_string(), "ETHEREAL_VINE".to_string()), ("Rarefinder Chip".to_string(), "RAREFINDER_GARDEN_CHIP".to_string()), ("Cornucopia".to_string(), "CORNUCOPIA".to_string()), ("Carrot Zest".to_string(), "CARROT_ZEST".to_string()), ("Deepfries".to_string(), "DEEPFRIES".to_string()), ("Aggourdian".to_string(), "AGGOURDIAN".to_string()), ("Cane Knot".to_string(), "CANE_KNOT".to_string()), ("Melon Juice".to_string(), "MELON_JUICE".to_string()), ("Cactus Flower".to_string(), "CACTUS_FLOWER".to_string()), ("Designer Coffee Beans".to_string(), "DESIGNER_COFFEE_BEANS".to_string()), ("Feastfungus".to_string(), "FEASTFUNGUS".to_string()), ("Botroot".to_string(), "BOTROOT".to_string()), ("Salted Sunflower Seeds".to_string(), "SALTED_SUNFLOWER_SEEDS".to_string()), ("Crystalized Moonlight".to_string(), "CRYSTALIZED_MOONLIGHT".to_string()), ("Floral Gelatin".to_string(), "FLORAL_GELATIN".to_string()), ("Wild Strawberry Dye".to_string(), "DYE_WILD_STRAWBERRY".to_string()), ("Ray of Helios".to_string(), "RAY_OF_HELIOS".to_string())]));
/// java: private static final Map<String, FlexibleItemStack> ICONS = new HashMap<>();
pub static ICONS: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class RareCropFilter extends ChatPatternListener
#[derive(Clone)]
pub struct RareCropFilter {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for RareCropFilter {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RareCropFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RareCropFilter {
    /// java: public RareCropFilter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in(format!("^RARE CROP!\\s+(?<crop>[\\w\\s]+)\\s+\\(\\+\\d+(?:\\.\\d+)?{}\\).*", sky_block_icons::OVERBLOOM), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

    /// java: private @Nullable ItemStack getCropIcon(Matcher matcher)
    pub fn get_crop_icon(__this: &compat::Shared<Self>, matcher: &compat::regex::Matcher) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let Some(mut skyblock_id) = IDS.get(matcher.group_named("crop").as_ref().unwrap()).cloned() else {
            return None;
        };
        if NEURepoManager::is_loading() || !(ItemRepository::files_imported()) {
            return FlexibleItemStack::get_stack(&ItemUtils::get_item_id_placeholder(&skyblock_id));
        }
        return FlexibleItemStack::get_stack(ICONS.get_mut().entry(skyblock_id.clone()).or_insert_with_key(|id: &String| ItemRepository::get_item_stack_2(id, &ItemUtils::get_item_id_placeholder(id))));
    }

}

impl ChatPatternListener for RareCropFilter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        RareCropFilter::on_match_dyn
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        RareCropFilter::state_dyn
    }

}

impl ChatMessageListener for RareCropFilter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl RareCropFilter {
    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        if SkyblockerConfigManager::get().chat.hide_rare_crops == ChatFilterResult::Toast {
            compat::minecraft().gui.toast_manager().borrow_mut().add_toast(Box::new(BasicToast::new_3(message.clone(), (((SkyblockerConfigManager::get().chat.toast_display_duration * (1000 as f32))) as i64), RareCropFilter::get_crop_icon(__this, matcher))));
        }
        return true;
    }

    /// RareCropFilter.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        if SkyblockerConfigManager::get().chat.hide_rare_crops == ChatFilterResult::Toast {
            return ChatFilterResult::Filter;
        } else {
            return SkyblockerConfigManager::get().chat.hide_rare_crops;
        }
    }

    /// RareCropFilter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

}
