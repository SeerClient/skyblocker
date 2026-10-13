//! Ported from de.hysky.skyblocker.skyblock.item.slottext.SlotTextManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::bazaar::bazaar_helper::BazaarHelper;
use crate::de::hysky::skyblocker::skyblock::chocolatefactory::chocolate_factory_solver;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::same_color_terminal;
use crate::de::hysky::skyblocker::skyblock::foraging::galatea::tuner_solver;
use crate::de::hysky::skyblocker::skyblock::hunting::attribute_level_helper;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::bestiary_level_adder::BestiaryLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::catacombs_level_adder::DungeonClasses;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::catacombs_level_adder::Dungeoneering;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::catacombs_level_adder::ReadyUp;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::chip_level_adder::ChipLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::choose_pet_level_adder::ChoosePetLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::collection_adder::CollectionAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::community_shop_adder::CommunityShopAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::crop_milestones_adder::CropMilestonesAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::enchantment_abbreviation_adder::EnchantmentAbbreviationAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::enchantment_level_adder::EnchantmentLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::essence_shop_adder::EssenceShopAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::evolving_item_adder::EvolvingItemAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::garden_upgrades_adder::GardenUpgradesAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::hotf_perk_level_adder::HotfPerkLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::hotm_perk_level_adder::HotmPerkLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::hunting_toolkit_indicator_adder::HuntingToolkitIndicatorAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::minion_level_adder::MinionLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::new_year_cake_adder::NewYearCakeAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::pet_level_adder::PetLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::potion_level_adder::PotionLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::power_stones_guide_adder::PowerStonesGuideAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::prehistoric_egg_adder::PrehistoricEggAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::rancher_boots_speed_adder::RancherBootsSpeedAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::skill_level_adder::SkillLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::skyblock_guide_adder::SkyblockGuideAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::skyblock_level_adder::SkyblockLevelAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::stats_tuning_adder::StatsTuningAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::adders::your_essence_adder::YourEssenceAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_mode::SlotTextMode;
use crate::de::hysky::skyblocker::skyblock::item::slottext::text_position::TextPosition;
use crate::de::hysky::skyblocker::skyblock::loadout_keybinds;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu_screen::RadialMenuScreen;
use crate::de::hysky::skyblocker::skyblock::wardrobe_keybinds;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;

/// java: private static final List<SlotTextAdder> adders = new ArrayList<>(List.of( new EssenceShopAdder(), new EnchantmentAbbreviationAdder(), new EnchantmentLevelAdder(), new MinionLevelAdder(), new PetLevel
pub static ADDERS: compat::MainThread<Vec<compat::Shared<dyn SlotTextAdder>>> = compat::MainThread::new(|| vec![(EssenceShopAdder::new() as compat::Shared<dyn SlotTextAdder>), (EnchantmentAbbreviationAdder::new() as compat::Shared<dyn SlotTextAdder>), (EnchantmentLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (MinionLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (PetLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (ChoosePetLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (SkyblockLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (HotmPerkLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (HotfPerkLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (SkillLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (Dungeoneering::new() as compat::Shared<dyn SlotTextAdder>), (DungeonClasses::new() as compat::Shared<dyn SlotTextAdder>), (ReadyUp::new() as compat::Shared<dyn SlotTextAdder>), (RancherBootsSpeedAdder::new() as compat::Shared<dyn SlotTextAdder>), (PrehistoricEggAdder::new() as compat::Shared<dyn SlotTextAdder>), (PotionLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (CollectionAdder::new() as compat::Shared<dyn SlotTextAdder>), (CommunityShopAdder::new() as compat::Shared<dyn SlotTextAdder>), (YourEssenceAdder::new() as compat::Shared<dyn SlotTextAdder>), (PowerStonesGuideAdder::new() as compat::Shared<dyn SlotTextAdder>), (BazaarHelper::new() as compat::Shared<dyn SlotTextAdder>), (StatsTuningAdder::new() as compat::Shared<dyn SlotTextAdder>), (tuner_solver::INSTANCE.get_mut().clone() as compat::Shared<dyn SlotTextAdder>), (chocolate_factory_solver::INSTANCE.get_mut().clone() as compat::Shared<dyn SlotTextAdder>), (EvolvingItemAdder::new() as compat::Shared<dyn SlotTextAdder>), (NewYearCakeAdder::new() as compat::Shared<dyn SlotTextAdder>), (wardrobe_keybinds::INSTANCE.get_mut().clone() as compat::Shared<dyn SlotTextAdder>), (loadout_keybinds::INSTANCE.get_mut().clone() as compat::Shared<dyn SlotTextAdder>), (SkyblockGuideAdder::new() as compat::Shared<dyn SlotTextAdder>), (same_color_terminal::INSTANCE.get_mut().clone() as compat::Shared<dyn SlotTextAdder>), (attribute_level_helper::INSTANCE.get_mut().clone() as compat::Shared<dyn SlotTextAdder>), (BestiaryLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (HuntingToolkitIndicatorAdder::new() as compat::Shared<dyn SlotTextAdder>), (ChipLevelAdder::new() as compat::Shared<dyn SlotTextAdder>), (CropMilestonesAdder::new() as compat::Shared<dyn SlotTextAdder>), (GardenUpgradesAdder::new() as compat::Shared<dyn SlotTextAdder>)].into_iter().collect::<Vec<_>>());
/// java: private static final Set<SlotTextAdder> currentScreenAdders = new HashSet<>();
/// A set of OBJECTS (the adders have identity equality): kept as a list, an adder added at most once (`Arc::ptr_eq`).
pub static CURRENT_SCREEN_ADDERS: compat::MainThread<Vec<compat::Shared<dyn SlotTextAdder>>> = compat::MainThread::new(|| Vec::new());
// TODO(fabric2seer): Fabric API `KeyMappingHelper.registerKeyMapping` has no SeerClient equivalent
//   java: public static final KeyMapping KEY_MAPPING = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.slottext", InputConstants.KEY_LALT, SkyblockerMod.KEYBINDING_CATEGORY));
/// java: public static final KeyMapping KEY_MAPPING = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.slottext", InputConstants.KEY_LALT, SkyblockerMod.KEYBINDING_CATEGORY));
pub static KEY_MAPPING: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(mcrust::net::minecraft::client::key_mapping::KeyMapping::new("key.skyblocker.slottext", 342 /* InputConstants.KEY_LALT */, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));
/// java: private static boolean keyHeld = false;
pub static KEY_HELD: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class SlotTextManager
#[derive(Clone)]
pub struct SlotTextManager {}

impl SlotTextManager {
    /// java: private SlotTextManager()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @SuppressWarnings("unused") public static void registerAdder(SlotTextAdder adder)
    pub fn register_adder(mut adder: compat::Shared<dyn SlotTextAdder>) {
        ADDERS.get_mut().push(adder.clone());
    }

    /// java: @Init public static void init()
    ///
    /// Java registers `ScreenKeyboardEvents.afterKeyPress(screen)` from `AFTER_INIT` for EVERY screen it sees; the
    /// port's `AFTER_KEY_PRESS` is one global event over every screen, so ONE listener is registered here (registering
    /// it inside `AFTER_INIT` would add another listener per screen opened, and the toggle would flip once per screen
    /// ever opened).
    pub fn init() {
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, _width: i32, _height: i32| {
            if compat::container_screen_open() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<ProfileViewerScreen>>() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<RadialMenuScreen>>() {
                SlotTextManager::on_screen_change(&screen);
                // java: ScreenEvents.remove(screen).register(_ -> currentScreenAdders.clear());
                // rust: port gap - the port has no `ScreenEvents.remove` (GAPS_batchE.md); the adders of the last
                //   matching screen stay until the next one replaces them
            }
        }));
        screen_keyboard_events::AFTER_KEY_PRESS.register(Arc::new(|_action: i32, key_event: &KeyEvent| {
            if KEY_MAPPING.get_mut().matches(&key_event) {
                // java: SkyblockerConfigManager.get().uiAndVisuals.slotText.slotTextToggled = !SkyblockerConfigManager.get().uiAndVisuals.slotText.slotTextToggled;
                SkyblockerConfigManager::update(Arc::new(|config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
                    config.ui_and_visuals.slot_text.slot_text_toggled = !config.ui_and_visuals.slot_text.slot_text_toggled;
                }));
                compat::set_state(&KEY_HELD, true);
            }
        }));
        // java: ScreenKeyboardEvents.afterKeyRelease(screen).register((_, input) -> { if (KEY_MAPPING.matches(input)) keyHeld = false; });
        // rust: port gap - the port has no `ScreenKeyboardEvents.afterKeyRelease` (GAPS_batchE.md)
    }

    /// java: private static void onScreenChange(Screen screen)
    pub fn on_screen_change(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        for adder in ADDERS.get_mut().iter() {
            if (Utils::is_on_skyblock() || !({ let __f = compat::dispatch(adder, |__t| __t.skyblock_only()); __f(&compat::view(adder, |a| a as compat::Shared<dyn ContainerMatcher>)) })) && { let __f = compat::dispatch(adder, |__t| __t.is_enabled()); __f(&compat::view(adder, |a| a as compat::Shared<dyn ContainerMatcher>)) } && { let __f = compat::dispatch(adder, |__t| __t.test()); __f(&compat::view(adder, |a| a as compat::Shared<dyn ContainerMatcher>), screen) } {
                if !CURRENT_SCREEN_ADDERS.get().iter().any(|a| Arc::ptr_eq(a, adder)) {
                    CURRENT_SCREEN_ADDERS.get_mut().push(adder.clone());
                }
            }
        }
    }

    /// java: public static List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text(mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        let mut text: Vec<SlotText> = Vec::new();
        if CURRENT_SCREEN_ADDERS.get_mut().is_empty() || !(SlotTextManager::is_enabled_0()) {
            return text;
        }
        for adder in CURRENT_SCREEN_ADDERS.get_mut().iter() {
            text.extend(({ let __f = compat::dispatch(adder, |__t| __t.get_text()); __f(adder, slot.clone(), stack, slot_id) }).into_iter());
        }
        return text;
    }

    /// java: public static void extractSlotText(GuiGraphicsExtractor graphics, Font textRenderer, Slot slot)
    pub fn extract_slot_text(graphics: &GuiGraphicsExtractor, text_renderer: &Font, slot: &Slot) {
        // java: extractSlotText(graphics, textRenderer, slot, slot.getItem(), slot.index, slot.x, slot.y);
        // (`slot.getItem()` reads the open menu's containers: `compat::slot_item`; a caller that holds the stack - the
        // container screen's slot pass - calls `extract_slot_text_7` with it)
        let stack = compat::slot_item(slot);
        SlotTextManager::extract_slot_text_7(graphics, text_renderer, Some(slot.clone()), &stack, slot.index, slot.x, slot.y);
    }

    /// java: public static void extractSlotText(GuiGraphicsExtractor graphics, Font textRenderer, @Nullable Slot slot, ItemStack stack, int slotId, int x, int y)
    pub fn extract_slot_text_7(graphics: &GuiGraphicsExtractor, text_renderer: &Font, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32, mut x: i32, mut y: i32) {
        let mut text_list: Vec<SlotText> = SlotTextManager::get_text(slot.clone(), stack, slot_id);
        if text_list.is_empty() {
            return;
        }
        for slot_text in text_list.iter() {
            let __pose_1 = gui_graphics_extractor::push_pose();
            let mut length: i32 = text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(slot_text.text()));
            if length > 16 {
                let mut scale: f32 = (16.0f32 / (length as f32));
                gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
                match slot_text.position() {
                    TextPosition::TopLeft | TextPosition::TopRight => {
                        gui_graphics_extractor::pose_translate(((((x as f32) / scale) - (x as f32))) as f32, ((((y as f32) / scale) - (y as f32))) as f32);
                    }
                    TextPosition::BottomLeft | TextPosition::BottomRight => {
                        gui_graphics_extractor::pose_translate(((((x as f32) / scale) - (x as f32))) as f32, (((((((y as f32) + 16.0f32) / scale) - (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32)) + 2.0f32) - (y as f32))) as f32);
                    }
                    _ => {}
                }
            } else {
                match slot_text.position() {
                    TextPosition::TopLeft => {
                    }
                    TextPosition::TopRight => {
                        gui_graphics_extractor::pose_translate(((16.0f32 - (length as f32))) as f32, (0.0f32) as f32);
                    }
                    TextPosition::BottomLeft => {
                        gui_graphics_extractor::pose_translate((0.0f32) as f32, (((16.0f32 - (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32)) + 2.0f32)) as f32);
                    }
                    TextPosition::BottomRight => {
                        gui_graphics_extractor::pose_translate(((16.0f32 - (length as f32))) as f32, (((16.0f32 - (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32)) + 2.0f32)) as f32);
                    }
                    _ => {}
                }
            }
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(slot_text.text()), x, y, mcrust::net::minecraft::util::common_colors::WHITE, true);
            gui_graphics_extractor::pop_pose(__pose_1);
        }
    }

    /// java: public static Stream<SlotTextAdder> getAdderStream()
    pub fn get_adder_stream() -> Box<dyn Iterator<Item = compat::Shared<dyn SlotTextAdder>>> {
        return Box::new(ADDERS.get_mut().iter().cloned());
    }

    /// java: public static boolean isEnabled(String adderId)
    pub fn is_enabled(adder_id: &str) -> bool {
        return SkyblockerConfigManager::get().ui_and_visuals.slot_text.text_enabled.get(adder_id).cloned().unwrap_or_else(|| true);
    }

    /// java: public static boolean isEnabled()
    pub fn is_enabled_0() -> bool {
        return match SkyblockerConfigManager::get().ui_and_visuals.slot_text.slot_text_mode {
            SlotTextMode::Enabled => true,
            SlotTextMode::Disabled => false,
            SlotTextMode::PressToToggle => SkyblockerConfigManager::get().ui_and_visuals.slot_text.slot_text_toggled,
            SlotTextMode::HoldToHide => !(compat::get_state(&KEY_HELD)),
            SlotTextMode::HoldToShow => compat::get_state(&KEY_HELD),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}
