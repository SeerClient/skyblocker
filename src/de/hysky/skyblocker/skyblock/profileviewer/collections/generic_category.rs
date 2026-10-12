//! Ported from de.hysky.skyblocker.skyblock.profileviewer.collections.GenericCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use mcrust::java::awt::color::Color;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter as _;
use mcrust::net::minecraft::core::component::data_component_holder::DataComponentHolder as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());
/// java: private static final Identifier BUTTON_TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/button_icon_toggled.png");
pub static BUTTON_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/button_icon_toggled.png"));
/// java: private static final int COLUMN_GAP = 26;
pub const COLUMN_GAP: i32 = 26;
/// java: private static final int ROW_GAP = 34;
pub const ROW_GAP: i32 = 34;
/// java: private static final int COLUMNS = 7;
pub const COLUMNS: i32 = 7;

/// java: public class GenericCategory implements ProfileViewerPage
#[derive(Clone)]
pub struct GenericCategory {
    /// java: private final String category;
    pub category: String,
    /// java: private final LinkedList<ItemStack> collections = new LinkedList<>();
    pub collections: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final Map<String, List<Collection>> collectionsData;
    pub collections_data: std::collections::HashMap<String, Vec<Vec<_>>>,
    /// java: private final Map<String, String> ICON_TRANSLATION = Map.ofEntries( Map.entry("MUSHROOM_COLLECTION", "RED_MUSHROOM"));
    pub icon_translation: std::collections::HashMap<String, String>,
}

impl GenericCategory {
    /// java: public GenericCategory(JsonObject hProfile, JsonObject pProfile, String collection)
    pub fn new(mut h_profile: serde_json::Map<String, serde_json::Value>, mut p_profile: serde_json::Map<String, serde_json::Value>, mut collection: String) -> compat::Shared<Self> {
        let __init_collections_data = ProfileViewerScreen::get_collections();
        let __init_category = collection.clone();
        let __this = compat::shared(Self {
            category: __init_category,
            collections: Vec::new(),
            collections_data: __init_collections_data,
            icon_translation: std::collections::HashMap::from([("MUSHROOM_COLLECTION".to_string(), "RED_MUSHROOM".to_string())]),
        });
        GenericCategory::setup_item_stacks(&__this, &h_profile, &p_profile);
        __this
    }

    /// java: private int calculateTier(long achieved, IntList requirements)
    pub fn calculate_tier(__this: &compat::Shared<Self>, mut achieved: i64, requirements: &Vec<i32>) -> i32 {
        // TODO(fabric2seer): List.intStream(..)
        //   java: return (int) requirements.intStream().filter(req -> achieved >= req).count();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void setupItemStacks(JsonObject hProfile, JsonObject pProfile)
    pub fn setup_item_stacks(__this: &compat::Shared<Self>, h_profile: &serde_json::Map<String, serde_json::Value>, p_profile: &serde_json::Map<String, serde_json::Value>) {
        let mut player_collection: Option<serde_json::Map<String, serde_json::Value>> = Some(compat::json::get_as_json_object_of(&p_profile, "collection"));
        for collection in (__this.lock().borrow().collections_data.get(&compat::read(__this, |__t| __t.category.clone())).cloned().unwrap()).iter() {
            // TODO(fabric2seer): List.id(..)
            //   java: FlexibleItemStack flexible = ItemRepository.getItemStack(ICON_TRANSLATION.getOrDefault(collection.id(), collection.id()).replace(':', '-'));
            let mut flexible: compat::Shared<FlexibleItemStack> = todo!("fabric2seer: the initialiser of `flexible` (TODO above)");
            let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = (if false /* java: flexible == null: never null here */ { FlexibleItemStack::get_stack_or_throw(ico::BARRIER.get_mut()).copy() } else { FlexibleItemStack::get_stack_or_throw(&flexible).copy() });
            if compat::component_of(item_stack.get_item().get_name(&item_stack)).get_string() == "Barrier" {
                // TODO(fabric2seer): List.id(..)
                //   java: itemStack.set(DataComponents.CUSTOM_NAME, Component.nullToEmpty(collection.id()));
                println!("{}", collection.to_string());
                println!("{}", compat::read(__this, |__t| __t.category.clone()));
            }
            let mut style: Style = mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(Some(ChatFormatting::White)).with_italic(Some(false));
            // TODO(fabric2seer): `.getString(..)` on a value of unknown type
            //   java: itemStack.set(DataComponents.CUSTOM_NAME, Component.literal(ChatFormatting.stripFormatting(itemStack.getComponents().get(DataComponents.CUSTOM_NAME).getString())).setStyle(style));
            // TODO(fabric2seer): List.id(..)
            //   java: long personalColl = playerCollection != null && playerCollection.has(collection.id()) ? playerCollection.get(collection.id()).getAsLong() : 0;
            let mut personal_coll: i64 = todo!("fabric2seer: the initialiser of `personalColl` (TODO above)");
            let mut total_collection: i64 = ((0) as i64) as i64;
            for member in compat::json::get_as_json_object(&h_profile.get("members").cloned().unwrap()).keys().cloned().collect::<Vec<String>>() {
                if !(compat::json::has(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&h_profile, "members")), &member)), "collection")) {
                    continue;
                }
                let mut member_coll: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&h_profile, "members")), &member)), "collection");
                // TODO(fabric2seer): List.id(..)
                //   java: totalCollection += memberColl.has(collection.id()) ? memberColl.get(collection.id()).getAsLong() : 0;
            }
            // TODO(fabric2seer): List.tiers(..)
            //   java: int collectionTier = calculateTier(totalCollection, collection.tiers());
            let mut collection_tier: i32 = todo!("fabric2seer: the initialiser of `collectionTier` (TODO above)");
            // TODO(fabric2seer): List.tiers(..)
            //   java: IntList tierRequirements = collection.tiers();
            let mut tier_requirements: Vec<i32> = todo!("fabric2seer: the initialiser of `tierRequirements` (TODO above)");
            let mut lore: Vec<MutableComponent> = Vec::new();
            lore.push(component::literal("Collection Item").set_style(style.clone()).with_style_format(ChatFormatting::DarkGray));
            lore.push(component::empty());
            if ((compat::json::get_as_json_object(&h_profile.get("members").cloned().unwrap()).keys().cloned().collect::<Vec<String>>()).len() as i32) > 1 {
                lore.push(((component::literal(&format!("Personal: {}", formatters::INTEGER_NUMBERS.get_mut().format((personal_coll) as f64)))).set_style(style.clone())).with_style_format(ChatFormatting::Gold));
                lore.push(((component::literal(&format!("Co-op: {}", formatters::INTEGER_NUMBERS.get_mut().format((total_collection.wrapping_sub(personal_coll)) as f64)))).set_style(style.clone())).with_style_format(ChatFormatting::Aqua));
            }
            lore.push(((component::literal(&format!("Collection: {}", formatters::INTEGER_NUMBERS.get_mut().format((total_collection) as f64)))).set_style(style.clone())).with_style_format(ChatFormatting::Yellow));
            lore.push(component::empty());
            lore.push(((component::literal(&format!("Collection Tier: {}/{}", collection_tier, (tier_requirements.len() as i32)))).set_style(style.clone())).with_style_format(ChatFormatting::LightPurple));
            if collection_tier == (tier_requirements.len() as i32) {
                item_stack.set(mcrust::net::minecraft::core::component::data_components::enchantment_glint_override().clone(), Some(true));
            }
            item_stack.set(mcrust::net::minecraft::core::component::data_components::lore().clone(), Some(ItemLore::new(lore.clone()).unwrap()));
            item_stack.set(mcrust::net::minecraft::core::component::data_components::custom_data().clone(), Some(mcrust::net::minecraft::world::item::component::custom_data::empty()));
            let __h0 = item_stack.clone();
            __this.lock().borrow_mut().collections.push(__h0);
        }
    }

}

impl ProfileViewerPage for GenericCategory {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32) {
        GenericCategory::extract_render_state_dyn
    }

}

impl GenericCategory {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32, mut root_x: i32, mut root_y: i32) {
        let mut category_title: MutableComponent = (component::literal(&format!("{}{} Collections", compat::char_at(&__this.lock().borrow().category, 0), (compat::substring(&__this.lock().borrow().category, 1)).to_lowercase()))).with_style_format(ChatFormatting::Bold);
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.text(textRenderer, categoryTitle, rootX + 88 - (textRenderer.width(categoryTitle) / 2), rootY, Color.DARK_GRAY.getRGB(), false);
        for i in 0..(__this.lock().borrow().collections.len() as i32) {
            let mut x: i32 = (root_x.wrapping_add(2)).wrapping_add((i.wrapping_rem(COLUMNS)).wrapping_mul(COLUMN_GAP));
            let mut y: i32 = (root_y.wrapping_add(19)).wrapping_add((i.wrapping_div(COLUMNS)).wrapping_mul(ROW_GAP));
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BUTTON_TEXTURE.get_mut(), x.wrapping_sub(3), y.wrapping_sub(3), ((0) as f32) as f32, ((0) as f32) as f32, 22, 22, 22, 22);
            graphics.item(&__this.lock().borrow().collections[(i) as usize], x, y);
            let item_stack = &__this.lock().borrow().collections[(i) as usize];
            // TODO(fabric2seer): `.lines(..)` on a value of unknown type
            //   java: List<Component> lore = itemStack.getOrDefault(DataComponents.LORE, ItemLore.EMPTY).lines();
            let mut lore: Vec<MutableComponent> = todo!("fabric2seer: the initialiser of `lore` (TODO above)");
            for text in lore.iter() {
                if text.get_string().starts_with("Collection Tier: ") {
                    let mut tier_text: String = compat::substring(&text.get_string(), compat::str_len("Collection Tier: "));
                    if tier_text.contains("/") {
                        let mut parts: Vec<String> = compat::regex::split(&tier_text, "/");
                        let mut c_tier: i32 = (compat::trim(&(parts[(0) as usize]))).trim().parse::<i32>().expect("NumberFormatException");
                        let mut colour: Color = (if item_stack.has_foil() { mcrust::java::awt::color::magenta() } else { mcrust::java::awt::color::dark_gray() });
                        // TODO(fabric2seer): `.getRGB(..)` on a Color
                        //   java: graphics.text(textRenderer, Component.literal(RomanNumerals.decimalToRoman(cTier)), x + 9 - (textRenderer.width(RomanNumerals.decimalToRoman(cTier)) / 2), y + 21, colour.getRGB(), false);
                    }
                    break;
                }
            }
            if mouse_x > x && mouse_x < x.wrapping_add(16) && mouse_y > y && mouse_y < y.wrapping_add(16) {
                // TODO(fabric2seer): field `EMPTY` of a value of unknown type
                //   java: List<Component> tooltip = collections.get(i).getTooltipLines(Item.TooltipContext.EMPTY, Minecraft.getInstance().player, TooltipFlag.NORMAL);
                let mut tooltip: Vec<MutableComponent> = todo!("fabric2seer: the initialiser of `tooltip` (TODO above)");
                // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
                // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
                // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(TEXT_RENDERER.get_mut(), tooltip.clone(), mouse_x, mouse_y);
            }
        }
    }

    /// GenericCategory.extractRenderState as the dispatch of ProfileViewerPage.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn ProfileViewerPage>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32, __p4: i32, __p5: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

}
