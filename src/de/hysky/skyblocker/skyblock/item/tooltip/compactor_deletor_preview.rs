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
        // java: List<IntObjectPair<@Nullable FlexibleItemStack>> slots = customData.keySet().stream()
        //           .filter(slot -> slot.contains(type.toLowerCase(Locale.ENGLISH).substring(0, 7)))
        //           .map(slot -> IntObjectPair.of(Integer.parseInt(StringUtils.substringAfterLast(slot, "_")), ItemRepository.getItemStack(customData.getStringOr(slot, "")))).toList();
        let type_prefix: String = compat::substring2(&r#type.to_lowercase(), 0, 7);
        let mut slots: Vec<(i32, Option<compat::Shared<FlexibleItemStack>>)> = custom_data
            .key_set()
            .into_iter()
            .filter(|slot| slot.contains(type_prefix.as_str()))
            .map(|slot| {
                // `StringUtils.substringAfterLast(slot, "_")`: after the last `_`, or "" when there is none
                let after_last: &str = slot.rfind('_').map(|i| &slot[i + 1..]).unwrap_or("");
                // `Integer.parseInt` throws on a malformed number
                let index: i32 = after_last.parse::<i32>().unwrap_or_else(|e| panic!("NumberFormatException: For input string: \"{}\" ({})", after_last, e));
                (index, crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository::get_item_stack(&custom_data.get_string_or(slot, "")))
            })
            .collect();
        // java: List<ClientTooltipComponent> components = tooltips.stream().map(Component::getVisualOrderText).map(ClientTooltipComponent::create).collect(Collectors.toList());
        let mut components: Vec<Box<dyn ClientTooltipComponent>> = tooltips.iter().map(|line| create_text_component(line.clone())).collect();
        let mut dimensions: (i32, i32) = DIMENSIONS.get_mut().get(size).cloned().unwrap_or_else(|| DEFAULT_DIMENSION.get_mut().clone());
        // If there are no items in compactor or deletor
        if slots.is_empty() {
            let mut slots_count: i32 = dimensions.0.wrapping_mul(dimensions.1);
            // java: components.add(targetIndex, ClientTooltipComponent.create(Component.literal(slotsCount + (slotsCount == 1 ? " slot" : " slots")).withStyle(ChatFormatting.GRAY).getVisualOrderText()));
            components.insert((target_index) as usize, create_text_component(component::literal(&format!("{}{}", slots_count, (if slots_count == 1 { " slot" } else { " slots" }))).with_style_format(ChatFormatting::Gray)));
            // java: graphics.tooltip(client.font, components, x, y, DefaultTooltipPositioner.INSTANCE, stack.get(DataComponents.TOOLTIP_STYLE));
            draw_tooltip(graphics, stack, components, x, y);
            return true;
        }
        // Add the preview tooltip component
        components.insert((target_index) as usize, Box::new(CompactorPreviewTooltipComponent::new(slots.clone(), dimensions.clone())));
        if custom_data.contains("PERSONAL_DELETOR_ACTIVE") {
            // java: components.add(targetIndex, ClientTooltipComponent.create(Component.literal("Active: ").append(customData.getBooleanOr("PERSONAL_DELETOR_ACTIVE", false)
            //           ? Component.literal("YES").withStyle(ChatFormatting.BOLD).withStyle(ChatFormatting.GREEN) : Component.literal("NO").withStyle(ChatFormatting.BOLD).withStyle(ChatFormatting.RED)).getVisualOrderText()));
            let active: MutableComponent = if custom_data.get_boolean_or("PERSONAL_DELETOR_ACTIVE", false) { component::literal("YES").with_style_format(ChatFormatting::Bold).with_style_format(ChatFormatting::Green) } else { component::literal("NO").with_style_format(ChatFormatting::Bold).with_style_format(ChatFormatting::Red) };
            components.insert((target_index) as usize, create_text_component(component::literal("Active: ").append(active)));
        }
        // java: graphics.tooltip(client.font, components, x, y, DefaultTooltipPositioner.INSTANCE, stack.get(DataComponents.TOOLTIP_STYLE));
        draw_tooltip(graphics, stack, components, x, y);
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

/// `ClientTooltipComponent.create(component.getVisualOrderText())` - a text line.
fn create_text_component(line: MutableComponent) -> Box<dyn ClientTooltipComponent> {
    Box::new(mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_text_tooltip::ClientTextTooltip::new(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line)))
}

/// `graphics.tooltip(client.font, components, x, y, DefaultTooltipPositioner.INSTANCE, stack.get(DataComponents.TOOLTIP_STYLE))`.
/// Drawn now, at `x, y`, through the game's `widgets::draw_tooltip` (the `GuiGraphicsExtractor.tooltip` call for a mod's
/// draw), with the item's tooltip style.
fn draw_tooltip(graphics: &GuiGraphicsExtractor, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, components: Vec<Box<dyn ClientTooltipComponent>>, x: i32, y: i32) {
    let _ = graphics;
    let style = mcrust::mcrust::modding::widgets::item_tooltip_style(stack);
    mcrust::mcrust::modding::widgets::draw_tooltip(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &components, x, y, &mcrust::net::minecraft::client::gui::screens::inventory::tooltip::default_tooltip_positioner::DefaultTooltipPositioner, style.as_ref());
}
