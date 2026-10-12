//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.CompactorDeletorPreview by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::compactor_preview_tooltip_component::CompactorPreviewTooltipComponent;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component;
use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Map<String, IntIntPair> DIMENSIONS = Map.of( "4000", IntIntPair.of(1, 1), "5000", IntIntPair.of(1, 3), "6000", IntIntPair.of(1, 7), "7000", IntIntPair.of(2, 6) );
pub static DIMENSIONS: compat::MainThread<std::collections::HashMap<String, (i32, i32)>> = compat::MainThread::new(|| std::collections::HashMap::from([("4000".to_string(), (1, 1)), ("5000".to_string(), (1, 3)), ("6000".to_string(), (1, 7)), ("7000".to_string(), (2, 6))]));
/// java: private static final IntIntPair DEFAULT_DIMENSION = IntIntPair.of(1, 6);
pub static DEFAULT_DIMENSION: compat::MainThread<(i32, i32)> = compat::MainThread::new(|| (1, 6));
/// java: public static final Pattern NAME = Pattern.compile("PERSONAL_(?<type>COMPACTOR|DELETOR)_(?<size>\\d+)");
pub static NAME: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("PERSONAL_(?<type>COMPACTOR|DELETOR)_(?<size>\\d+)"));

/// java: public class CompactorDeletorPreview
#[derive(Clone)]
pub struct CompactorDeletorPreview {}

impl CompactorDeletorPreview {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean extractPreview(GuiGraphicsExtractor graphics, ItemStack stack, List<Component> tooltips, String type, String size, int x, int y)
    pub fn extract_preview(graphics: &GuiGraphicsExtractor, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, tooltips: &Vec<MutableComponent>, r#type: &str, size: &str, mut x: i32, mut y: i32) -> bool {
        let mut target_index: i32 = CompactorDeletorPreview::get_target_index(tooltips);
        if target_index == -1 {
            return false;
        }
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        // TODO(fabric2seer): static call StringUtils.substringAfterLast(..)
        //   java: List<IntObjectPair<@Nullable FlexibleItemStack>> slots = customData.keySet()
        //   java: 														.stream()
        //   java: 														.filter(slot -> slot.contains(type.toLowerCase(Locale.ENGLISH).substring(0, 7)))
        //   java: 														.map(slot -> IntObjectPair.of(Integer.parseInt(StringUtils.substringAfterLast(slot, "_")), ItemRepository.getItemStack(customData.getStringOr(slot, "")))).toList();
        let mut slots: Vec<(i32, Option<compat::Shared<FlexibleItemStack>>)> = todo!("fabric2seer: the initialiser of `slots` (TODO above)");
        // TODO(fabric2seer): the port's `create_text` ports this Java method with other parameters ( for Java's char_sequence)
        // TODO(fabric2seer): `create_text` takes 0 argument(s) in the port (``), Java passes 1
        // rust: let mut components: Vec<Box<dyn ClientTooltipComponent>> = tooltips.iter().cloned().map(|__a0| __a0.get_visual_order_text()).map(|__a0| client_tooltip_component::create_text(__a0.clone())).collect::<Vec<_>>();
        let mut components: Vec<Box<dyn ClientTooltipComponent>> = todo!("fabric2seer: the initialiser of `components` (TODO above)");
        let mut dimensions: (i32, i32) = DIMENSIONS.get_mut().get(size).cloned().unwrap_or_else(|| DEFAULT_DIMENSION.get_mut().clone());
        if slots.is_empty() {
            let mut slots_count: i32 = dimensions.0.wrapping_mul(dimensions.1);
            // TODO(fabric2seer): the port's `create_text` ports this Java method with other parameters ( for Java's char_sequence)
            // TODO(fabric2seer): `create_text` takes 0 argument(s) in the port (``), Java passes 1
            // rust: components.insert((target_index) as usize, client_tooltip_component::create_text((component::literal(&format!("{}{}", slots_count, (if slots_count == 1 { " slot" } else { " slots" })))).with_style_format(ChatFormatting::Gray).get_visual_order_text()));
            // TODO(fabric2seer): `tooltip` takes 7 argument(s) in the port (`graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, font: &dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont ..`), Java passes 6
            // rust: gui_graphics_extractor::tooltip(&mut compat::font(), &components, &x, y, mcrust::net::minecraft::client::gui::screens::inventory::tooltip::default_tooltip_positioner::DefaultTooltipPositioner::INSTANCE, &stack.get(mcrust::net::minecraft::core::component::data_components::tooltip_style()).unwrap());
            return true;
        }
        components.insert((target_index) as usize, Box::new(CompactorPreviewTooltipComponent::new(slots.clone(), dimensions.clone())));
        if custom_data.contains("PERSONAL_DELETOR_ACTIVE") {
            // TODO(fabric2seer): the port's `create_text` ports this Java method with other parameters ( for Java's char_sequence)
            // TODO(fabric2seer): `create_text` takes 0 argument(s) in the port (``), Java passes 1
            // rust: components.insert((target_index) as usize, client_tooltip_component::create_text((component::literal("Active: ").append((if custom_data.get_boolean_or("PERSONAL_DELETOR_ACTIVE", false) { component::literal("YES").with_style_format(ChatFormatting::Bold).with_style_format(ChatFormatting::Green) } else { component::literal("NO").with_style_format(ChatFormatting::Bold).with_style_format(ChatFormatting::Red) }))).get_visual_order_text()));
        }
        // TODO(fabric2seer): `tooltip` takes 7 argument(s) in the port (`graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, font: &dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont ..`), Java passes 6
        // rust: gui_graphics_extractor::tooltip(&mut compat::font(), &components, &x, y, mcrust::net::minecraft::client::gui::screens::inventory::tooltip::default_tooltip_positioner::DefaultTooltipPositioner::INSTANCE, &stack.get(mcrust::net::minecraft::core::component::data_components::tooltip_style()).unwrap());
        return true;
    }

    /// java: private static int getTargetIndex(List<Component> tooltips)
    pub fn get_target_index(tooltips: &Vec<MutableComponent>) -> i32 {
        let mut target_index: i32 = -1;
        let mut line_count: i32 = 0;
        for i in 0..(tooltips.len() as i32) {
            if ((tooltips[(i) as usize]).get_string()).is_empty() {
                line_count = line_count.wrapping_add(1);
            }
            if line_count == 2 {
                target_index = i;
                break;
            }
        }
        return target_index;
    }

}
