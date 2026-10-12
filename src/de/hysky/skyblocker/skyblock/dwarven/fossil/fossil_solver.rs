//! Ported from de.hysky.skyblocker.skyblock.dwarven.fossil.FossilSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::fossil_calculations;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::fossil_calculations::FossilCalculations;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::structures::TileGrid;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::line_smoothener::LineSmoothener;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::item::Item;

/// java: private static final Pattern PERCENTAGE_PATTERN = Pattern.compile("Fossil Excavation Progress: (\\d{1,2}.\\d)%");
pub static PERCENTAGE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Fossil Excavation Progress: (\\d{1,2}.\\d)%"));
/// java: private static final Pattern CHARGES_PATTERN = Pattern.compile("Chisel Charges Remaining: (\\d{1,2})");
pub static CHARGES_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Chisel Charges Remaining: (\\d{1,2})"));
/// java: private static String percentage;
pub static PERCENTAGE: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static double[] probability;
pub static PROBABILITY: LazyLock<Mutex<Option<Vec<f64>>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static int chiselLeft = -1;
pub static CHISEL_LEFT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));

/// java: public class FossilSolver extends SimpleContainerSolver implements TooltipAdder
#[derive(Clone)]
pub struct FossilSolver {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for FossilSolver {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FossilSolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FossilSolver {
    /// java: public FossilSolver()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("Fossil Excavator".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: private static List<ColorHighlight> convertChanceToColor(double[] chances)
    pub fn convert_chance_to_color(chances: &Vec<f64>) -> Vec<ColorHighlight> {
        let mut output_colors: Vec<ColorHighlight> = Vec::new();
        let mut gradient_color: Color = mcrust::java::awt::color::blue();
        let mut high_probability: Option<f64> = chances.iter().copied().max_by(|a, b| a.total_cmp(b));
        for i in 0..(chances.len() as i32) {
            let mut chance: f64 = chances[(i) as usize];
            if (chances[(i) as usize]).is_nan() || chances[(i) as usize] == (0 as f64) {
                continue;
            }
            if chances[(i) as usize] == high_probability.clone().unwrap() {
                output_colors.push(ColorHighlight::green(i));
                continue;
            }
            // TODO(fabric2seer): `.getRed(..)` on a Color
            //   java: outputColors.add(new ColorHighlight(i, (int) (chance * 255) << 24 | gradientColor.getRed() << 16 | gradientColor.getGreen() << 8 | gradientColor.getBlue()));
        }
        return output_colors;
    }

    /// java: private int getChiselLeft(ItemStack itemStack)
    pub fn get_chisel_left(__this: &compat::Shared<Self>, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> i32 {
        // TODO(fabric2seer): field `EMPTY` of a value of unknown type
        //   java: for (Component line : itemStack.getTooltipLines(Item.TooltipContext.EMPTY, CLIENT.player, TooltipFlag.NORMAL)) {
        //   java: 			Matcher matcher = CHARGES_PATTERN.matcher(line.getString());
        //   java: 			if (matcher.find()) {
        //   java: 				return Integer.parseInt(matcher.group(1));
        //   java: 			}
        //   java: 		}
        return -1;
    }

    /// java: private String getFossilPercentage(Int2ObjectMap<ItemStack> slots)
    pub fn get_fossil_percentage(__this: &compat::Shared<Self>, slots: &std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Option<String> {
        for item in slots.values().cloned() {
            // TODO(fabric2seer): field `EMPTY` of a value of unknown type
            //   java: for (Component line : item.getTooltipLines(Item.TooltipContext.EMPTY, CLIENT.player, TooltipFlag.NORMAL)) {
            //   java: 				Matcher matcher = PERCENTAGE_PATTERN.matcher(line.getString());
            //   java: 				if (matcher.matches()) {
            //   java: 					return matcher.group(1);
            //   java: 				}
            //   java: 			}
        }
        return None;
    }

}

impl SimpleContainerSolver for FossilSolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for FossilSolver {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

    /// java: @Override public Pattern getTitlePattern() (dispatch: the body for this class)
    fn get_title_pattern(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        <dyn SimpleContainerSolver>::get_title_pattern_from::<Self>
    }

}

impl ContainerMatcher for FossilSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        FossilSolver::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for FossilSolver {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        FossilSolver::get_colors_dyn
    }

}

impl Resettable for FossilSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        FossilSolver::reset_dyn
    }

}

impl TooltipAdder for FossilSolver {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        FossilSolver::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        FossilSolver::get_priority_dyn
    }

}

impl FossilSolver {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().mining.glacite.fossil_solver;
    }

    /// FossilSolver.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut main_tile_grid: TileGrid = FossilCalculations::convert_items_to_tiles(slots);
        if !slots.is_empty() {
            compat::set_state(&CHISEL_LEFT, FossilSolver::get_chisel_left(__this, &slots.values().cloned().next().unwrap()));
        } else {
            compat::set_state(&CHISEL_LEFT, -1);
        }
        if compat::get_state(&PERCENTAGE).is_none() {
            compat::set_state(&PERCENTAGE, FossilSolver::get_fossil_percentage(__this, slots));
        }
        compat::set_state(&PROBABILITY, Some(FossilCalculations::get_fossil_chance(&main_tile_grid, compat::get_state(&PERCENTAGE).as_deref())));
        return FossilSolver::convert_chance_to_color(&compat::get_state(&PROBABILITY).unwrap());
    }

    /// FossilSolver.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        compat::set_state(&CHISEL_LEFT, -1);
        compat::set_state(&PERCENTAGE, None);
    }

    /// FossilSolver.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        if !std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().gray())) && !std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().brown())) {
            return;
        }
        lines.push(LineSmoothener::create_smooth_line());
        if compat::get_state(&fossil_calculations::PERMUTATIONS) == 0 {
            lines.push(component::translatable("skyblocker.config.mining.glacite.fossilSolver.toolTip.noFossilFound").with_style_format(ChatFormatting::Gold));
            return;
        }
        lines.push(component::translatable("skyblocker.config.mining.glacite.fossilSolver.toolTip.possiblePatterns").append(component::literal(&compat::get_state(&fossil_calculations::PERMUTATIONS).to_string()).with_style_format(ChatFormatting::Yellow)));
        lines.push(component::translatable("skyblocker.config.mining.glacite.fossilSolver.toolTip.minimumTilesLeft").append(component::literal(&compat::get_state(&fossil_calculations::MINIMUM_TILES).to_string()).with_style_format((if compat::get_state(&CHISEL_LEFT) >= compat::get_state(&fossil_calculations::MINIMUM_TILES) { ChatFormatting::Yellow } else { ChatFormatting::Red }))));
        if focused_slot.is_some() && compat::get_state(&PROBABILITY).is_some() && (compat::lock(&PROBABILITY).as_ref().unwrap().len() as i32) > focused_slot.as_ref().unwrap().get_container_slot() && std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().brown())) {
            lines.push(component::translatable("skyblocker.config.mining.glacite.fossilSolver.toolTip.probability").append((component::literal(&format!("{}%", compat::java_round_f64((compat::lock(&PROBABILITY).as_ref().unwrap()[(focused_slot.as_ref().unwrap().get_container_slot()) as usize] * (100 as f64)))))).with_style_format(ChatFormatting::Yellow)));
        }
        if compat::get_state(&fossil_calculations::FOSSIL_NAME).is_some() && compat::get_state(&PERCENTAGE).is_some() {
            lines.push(component::translatable("skyblocker.config.mining.glacite.fossilSolver.toolTip.foundFossil").append(component::literal(compat::get_state(&fossil_calculations::FOSSIL_NAME).as_deref().unwrap()).with_style_format(ChatFormatting::Yellow)));
        }
    }

    /// FossilSolver.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public int getPriority()
    pub fn get_priority_impl(__this: &compat::Shared<Self>) -> i32 {
        return 0;
    }

    /// FossilSolver.getPriority as the dispatch of TooltipAdder.getPriority
    pub fn get_priority_dyn(__this: &compat::Shared<dyn TooltipAdder>) -> i32 {
        Self::get_priority_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
