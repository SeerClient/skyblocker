//! Ported from de.hysky.skyblocker.skyblock.museum.DonationButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::item_price;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache;
use crate::de::hysky::skyblocker::skyblock::museum::museum_utils::MuseumUtils;
use crate::de::hysky::skyblocker::skyblock::museum::price_data::PriceData;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int SIZE = 33;
pub const SIZE: i32 = 33;
/// java: private static final int ITEM_OFFSET = 8;
pub const ITEM_OFFSET: i32 = 8;
/// java: private static final Font TEXT_RENDERER = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());

/// java: public class DonationButton extends AbstractWidget
#[derive(Clone)]
pub struct DonationButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private @Nullable Donation donation = null;
    pub donation: Option<compat::Shared<Donation>>,
    /// java: private @Nullable ItemStack itemStack = null;
    pub item_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private @Nullable String textToRender;
    pub text_to_render: Option<String>,
    /// java: private @Nullable List<Component> tooltip;
    pub tooltip: Option<Vec<MutableComponent>>,
}

impl std::ops::Deref for DonationButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DonationButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DonationButton {
    /// java: protected DonationButton(int x, int y)
    pub fn new(mut x: i32, mut y: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, SIZE, SIZE.wrapping_add(2), (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            donation: None,
            item_stack: None,
            text_to_render: None,
            tooltip: None,
        });
        __this
    }

    /// java: protected @Nullable ItemStack getDisplayStack()
    pub fn get_display_stack(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return compat::read(__this, |__t| __t.item_stack.clone());
    }

    /// java: public void init(Donation donation)
    pub fn init(__this: &compat::Shared<Self>, mut donation: compat::Shared<Donation>) {
        { let __v = true; compat::write(__this, |__t| __t.visible = __v); }
        { let __v = Some(donation.clone()); compat::write(__this, |__t| __t.donation = __v); }
        { let __v = Some(MuseumUtils::format_price(PriceData::get_effective_price(&Donation::get_price_data(&donation)))); compat::write(__this, |__t| __t.text_to_render = __v); }
        let mut flexible: Option<compat::Shared<FlexibleItemStack>> = Some((if !(Donation::is_set(&donation)) { ItemRepository::get_item_stack(&Donation::get_id(&donation)).unwrap() } else { ItemRepository::get_item_stack(compat::lock(&museum_item_cache::ARMOR_TO_ID).get(&Donation::get_id(&donation)).cloned().as_deref().unwrap()).unwrap() }));
        if flexible.is_none() {
            flexible = Some(ItemUtils::get_item_id_placeholder(&Donation::get_id(&donation)));
        }
        { let __v = Some(FlexibleItemStack::get_stack_or_empty(flexible.as_ref().unwrap())); compat::write(__this, |__t| __t.item_stack = __v); }
        DonationButton::build_tooltip(__this);
    }

    /// java: protected void resetButton()
    pub fn reset_button(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.visible = __v); }
        { let __v = None; compat::write(__this, |__t| __t.donation = __v); }
        { let __v = None; compat::write(__this, |__t| __t.item_stack = __v); }
        { let __v = None; compat::write(__this, |__t| __t.tooltip = __v); }
        { let __v = None; compat::write(__this, |__t| __t.text_to_render = __v); }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        debug_assert!(compat::read(__this, |__t| __t.donation.clone()).is_some() && compat::read(__this, |__t| __t.text_to_render.clone()).is_some());
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::recipebook::recipe_button::slot_craftable_sprite(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        let mut has_price: bool = Donation::has_price(&compat::read(__this, |__t| __t.donation.clone()).unwrap());
        if compat::read(__this, |__t| __t.item_stack.clone()).is_some() && !(__this.lock().borrow().item_stack.as_ref().unwrap().is_empty()) {
            graphics.fake_item(&compat::read(__this, |__t| __t.item_stack.clone()).unwrap(), (__this.lock().borrow().get_x()).wrapping_add(ITEM_OFFSET), (__this.lock().borrow().get_y()).wrapping_add(if has_price { 4 } else { 8 }));
        }
        if has_price {
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), compat::read(__this, |__t| __t.text_to_render.clone()).as_deref().unwrap(), (__this.lock().borrow().get_x()).wrapping_add((compat::read(__this, |__t| __t.width)).wrapping_div(2)), ((__this.lock().borrow().get_y()).wrapping_add(ITEM_OFFSET)).wrapping_add(13), 0xFF00FF00u32 as i32);
        }
    }

    /// java: private void buildTooltip()
    pub fn build_tooltip(__this: &compat::Shared<Self>) {
        debug_assert!(compat::read(__this, |__t| __t.donation.clone()).is_some());
        let mut tooltip: Vec<MutableComponent> = Vec::new();
        let mut soulbound: bool = (compat::read(__this, |__t| __t.item_stack.clone()).is_some() && !(__this.lock().borrow().item_stack.as_ref().unwrap().is_empty()) && ItemUtils::is_soulbound(&compat::read(__this, |__t| __t.item_stack.clone()).unwrap()));
        let mut discount: (String, f64) = Donation::get_discount(&compat::read(__this, |__t| __t.donation.clone()).unwrap()).unwrap();
        let mut counts_towards: Vec<(String, i32)> = Donation::get_counts_towards(&compat::read(__this, |__t| __t.donation.clone()).unwrap());
        tooltip.push(MuseumUtils::get_display_name(&Donation::get_id(&compat::read(__this, |__t| __t.donation.clone()).unwrap()), Donation::is_set(&compat::read(__this, |__t| __t.donation.clone()).unwrap())));
        if Donation::is_set(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) {
            for piece in Donation::get_set(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) {
                let mut flexible: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&piece.0);
                if let Some(flexible) = &flexible {
                    let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(flexible);
                    let mut item_name: MutableComponent = compat::component_of(stack.get_hover_name()).copy();
                    if soulbound {
                        tooltip.push(component::literal("  ").append(item_name.clone()));
                    } else if PriceData::get_effective_price(piece.1.as_ref().expect("java.lang.NullPointerException: the set piece's PriceData")) > (0 as f64) {
                        tooltip.push(component::literal("  ").append(item_name.clone()).append(component::literal(" (").with_style_format(ChatFormatting::DarkGray)).append(component::literal(&MuseumUtils::format_price(PriceData::get_effective_price(piece.1.as_ref().expect("java.lang.NullPointerException: the set piece's PriceData")))).with_style_format(ChatFormatting::Gold)).append(component::literal(")").with_style_format(ChatFormatting::DarkGray)));
                    } else {
                        tooltip.push(component::literal("  ").append(item_name.clone()).append(component::literal(" (").with_style_format(ChatFormatting::DarkGray)).append(component::translatable("skyblocker.museum.hud.unknownPrice").with_style_format(ChatFormatting::Red)).append(component::literal(")").with_style_format(ChatFormatting::DarkGray)));
                    }
                }
            }
        }
        tooltip.push(component::empty());
        let mut xp_text: MutableComponent = (component::literal(&Donation::get_total_xp(&compat::read(__this, |__t| __t.donation.clone()).unwrap()).to_string())).append_str(" ").append(component::translatable("skyblocker.museum.hud.skyblockXp")).with_style_format(ChatFormatting::Aqua);
        tooltip.push(component::translatable("skyblocker.museum.hud.xpReward").append_str(": ").with_style_format(ChatFormatting::Gray).append(xp_text.clone()));
        if !soulbound {
            let mut price_data: compat::Shared<PriceData> = Donation::get_price_data(&compat::read(__this, |__t| __t.donation.clone()).unwrap());
            let mut l_bin_text: MutableComponent = (if Donation::has_l_bin_price(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) { component::literal(&MuseumUtils::format_price(PriceData::get_l_bin_price(&price_data))).append_str(" Coins").with_style_format(ChatFormatting::Gold) } else { component::translatable("skyblocker.museum.hud.unknownPrice").with_style_format(ChatFormatting::Red) });
            let mut craft_cost_text: MutableComponent = (if Donation::is_craftable(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) { (component::literal(&MuseumUtils::format_price((if Donation::has_discount(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) { (PriceData::get_craft_cost(&price_data) - discount.1) } else { PriceData::get_craft_cost(&price_data) })))).append_str(" ").append(component::translatable("skyblocker.museum.hud.coin")).with_style_format(ChatFormatting::Gold) } else { component::translatable("skyblocker.museum.hud.unknownPrice").with_style_format(ChatFormatting::Red) });
            let mut discount_text: MutableComponent = (if Donation::has_discount(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) && Donation::is_craftable(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) { (component::literal(" (").with_style_format(ChatFormatting::DarkGray).append(component::literal(&MuseumUtils::format_price(PriceData::get_craft_cost(&price_data))).with_style_format(ChatFormatting::Gold)).append_str(" - ")).append(component::literal(&MuseumUtils::format_price(discount.1)).with_style_format(ChatFormatting::Gold)).append(component::literal(")").with_style_format(ChatFormatting::DarkGray)) } else { component::empty() });
            let mut xp_coins_ratio: MutableComponent = (component::literal(&MuseumUtils::format_price(Donation::get_xp_coins_ratio(&compat::read(__this, |__t| __t.donation.clone()).unwrap())))).append_str(" ").append(component::translatable("skyblocker.museum.hud.ratioText")).with_style_format(ChatFormatting::Aqua);
            tooltip.push(component::translatable("skyblocker.museum.hud.sorter.lBin").append_str(": ").with_style_format(ChatFormatting::Gray).append(l_bin_text.clone()));
            tooltip.push(component::translatable("skyblocker.museum.hud.sorter.craftCost").append_str(": ").with_style_format(ChatFormatting::Gray).append(craft_cost_text.clone()).append(discount_text.clone()));
            tooltip.push(component::translatable("skyblocker.museum.hud.sorter.ratio").append_str(": ").with_style_format(ChatFormatting::Gray).append(xp_coins_ratio.clone()));
        }
        if (counts_towards.len() as i32) > 1 {
            tooltip.push(component::empty());
            tooltip.push(component::translatable("skyblocker.museum.hud.countsFor").append_str(":").with_style_format(ChatFormatting::Gray));
            for credit in counts_towards.iter() {
                tooltip.push((component::literal(" ● ")).with_style_format(ChatFormatting::Gray).append(MuseumUtils::get_display_name(&credit.0, Donation::is_set(&compat::read(__this, |__t| __t.donation.clone()).unwrap()))).append((component::literal(&format!(" ({} XP)", credit.1))).with_style_format(ChatFormatting::Aqua)));
            }
        }
        if Donation::is_craftable(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) && Donation::has_discount(&compat::read(__this, |__t| __t.donation.clone()).unwrap()) {
            tooltip.push(component::empty());
            tooltip.push(component::translatable("skyblocker.museum.hud.craftIngredient").append_str(": ").with_style_format(ChatFormatting::Gray).append(component::literal("(").append(component::translatable("skyblocker.museum.hud.alreadyDonatedItem").append_str(")")).with_style_format(ChatFormatting::DarkGray)));
            tooltip.push((component::literal(" - ")).with_style_format(ChatFormatting::Gray).append(MuseumUtils::get_display_name(&discount.0, Donation::is_set(&compat::read(__this, |__t| __t.donation.clone()).unwrap()))).append((component::literal(" ✔")).with_style_format(ChatFormatting::Green)).append(component::literal(" (").with_style_format(ChatFormatting::DarkGray).append(component::literal(&MuseumUtils::format_price(discount.1)).with_style_format(ChatFormatting::Gold)).append_str(")").with_style_format(ChatFormatting::DarkGray)));
        }
        let mut wiki_lookup_key_string: String = WikiLookupManager::get_keys_text();
        // TODO(fabric2seer): `get_translated_key_message` takes 2 argument(s) in the port (`name: &str, key_name: &dyn Fn(i32, i32) -> Option<String>`), Java passes 0
        // rust: let mut item_price_lookup_key_string: String = compat::component_of(key_mapping::get_translated_key_message()).get_string();
        let mut item_price_lookup_key_string: String = todo!("fabric2seer: the initialiser of `itemPriceLookupKeyString` (TODO above)");
        if soulbound || !wiki_lookup_key_string.is_empty() || !item_price_lookup_key_string.is_empty() {
            tooltip.push(component::empty());
        }
        if soulbound {
            tooltip.push(component::literal("* Soulbound *").with_style_format(ChatFormatting::DarkGray));
        }
        if !wiki_lookup_key_string.is_empty() {
            tooltip.push((component::translatable_args("skyblocker.museum.hud.wikiLookup", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(wiki_lookup_key_string.clone())])).with_style_format(ChatFormatting::Yellow));
        }
        if !soulbound && !item_price_lookup_key_string.is_empty() {
            tooltip.push((component::translatable_args("skyblocker.museum.hud.itemPriceLookup", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(item_price_lookup_key_string.clone())])).with_style_format(ChatFormatting::Yellow));
        }
        { let __v = Some(tooltip.clone()); compat::write(__this, |__t| __t.tooltip = __v); }
    }

    /// java: protected List<Component> getItemTooltip()
    pub fn get_item_tooltip(__this: &compat::Shared<Self>) -> Vec<MutableComponent> {
        return compat::read(__this, |__t| __t.tooltip.clone()).unwrap();
    }

    /// java: protected ItemStack getItem()
    pub fn get_item(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return compat::read(__this, |__t| __t.item_stack.clone()).unwrap();
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// DonationButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for DonationButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// DonationButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<DonationButton> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| DonationButton::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| DonationButton::update_widget_narration(__h, output))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
