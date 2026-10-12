//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.MinionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::collection_item_widget::CollectionItemWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::collection_item_widget::CollectionItemWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public final class MinionWidget extends CollectionItemWidget
#[derive(Clone)]
pub struct MinionWidget {
    /// java: extends CollectionItemWidget
    pub base: CollectionItemWidgetBase,
}

impl std::ops::Deref for MinionWidget {
    type Target = CollectionItemWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MinionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MinionWidget {
    /// java: private MinionWidget(ItemStack icon, Component tierText, List<Component> tooltip, @Nullable Identifier tooltipStyle)
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tier_text: MutableComponent, mut tooltip: Vec<MutableComponent>, mut tooltip_style: Option<Identifier>) -> compat::Shared<Self> {
        let (_, __this) = CollectionItemWidgetBase::new_in(icon.clone(), tier_text.clone(), tooltip.clone(), tooltip_style.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn CollectionItemWidget>), __h)
        });
        __this
    }

    /// java: public static MinionWidget create(String minionId, int maxTier, List<Integer> tiersCrafted, OptionalInt lowestUncraftedTier)
    pub fn create(minion_id: &str, mut max_tier: i32, tiers_crafted: &Vec<i32>, lowest_uncrafted_tier: &Option<i32>) -> compat::Shared<MinionWidget> {
        let mut displayed_tier: i32 = (if lowest_uncrafted_tier.is_some() { (lowest_uncrafted_tier.clone().unwrap()).wrapping_sub(1) } else { max_tier });
        let mut is_max_tier: bool = ((tiers_crafted.len() as i32) == max_tier);
        let mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&ItemRepository::get_item_stack_2(&format!("{}_GENERATOR_{}", minion_id, displayed_tier.max(1)), ico::BARRIER.get_mut()));
        let mut tier_text: MutableComponent = CollectionItemWidgetBase::build_tier_text(displayed_tier, is_max_tier);
        let mut tooltip: Vec<MutableComponent> = MinionWidget::build_tooltip(max_tier, tiers_crafted, is_max_tier, &compat::component_of(icon.get_hover_name()).get_string());
        let mut tooltip_style: Identifier = CollectionItemWidgetBase::get_tooltip_type(is_max_tier).unwrap();
        return MinionWidget::new(icon.clone(), tier_text.clone(), tooltip.clone(), Some(tooltip_style.clone()));
    }

    /// java: private static List<Component> buildTooltip(int maxTier, List<Integer> tiersCrafted, boolean isMaxTier, String name)
    pub fn build_tooltip(mut max_tier: i32, tiers_crafted: &Vec<i32>, mut is_max_tier: bool, name: &str) -> Vec<MutableComponent> {
        let mut tooltip: Vec<MutableComponent> = Vec::new();
        let mut last_space_index: i32 = (compat::last_index_of(&name, ' ')).clamp(0, compat::str_len(&name));
        let mut name_formatting: ChatFormatting = (if is_max_tier { ChatFormatting::Green } else { (if tiers_crafted.is_empty() { ChatFormatting::Red } else { ChatFormatting::Yellow }) });
        let mut name_text: MutableComponent = (component::literal(&compat::substring2(&name, 0, last_space_index))).with_style_format(name_formatting);
        tooltip.push(name_text.clone());
        for i in 1..=max_tier {
            let mut crafted_tier: bool = tiers_crafted.iter().any(|e| *e == i);
            let mut crafted_text: String = format!("{} Tier {}", (if crafted_tier { "✔" } else { "✖" }), RomanNumerals::decimal_to_roman(i));
            tooltip.push(component::literal(&crafted_text).with_style_format((if crafted_tier { ChatFormatting::Green } else { ChatFormatting::Red })));
        }
        return tooltip.iter().cloned().collect::<Vec<_>>();
    }

}

impl CollectionItemWidget for MinionWidget {
    fn collection_item_widget_base(&self) -> &CollectionItemWidgetBase {
        &self.base
    }
    fn collection_item_widget_base_mut(&mut self) -> &mut CollectionItemWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// MinionWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for MinionWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// MinionWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<MinionWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn CollectionItemWidget>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn CollectionItemWidget>), output) }))
    }

}
