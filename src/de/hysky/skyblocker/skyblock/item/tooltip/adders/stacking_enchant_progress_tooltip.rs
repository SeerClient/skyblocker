//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.StackingEnchantProgressTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: private static final Set<String> STACKING_ENCHANT_IDS = Set.of("expertise", "compact", "cultivating", "champion", "hecatomb", "absorb", "toxophilite");
pub static STACKING_ENCHANT_IDS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["expertise".to_string(), "compact".to_string(), "cultivating".to_string(), "champion".to_string(), "hecatomb".to_string(), "absorb".to_string(), "toxophilite".to_string()]));
/// java: private static final StackingEnchantInfo EXPERTISE_INFO = new StackingEnchantInfo("Expertise", "expertise_kills", "kills", 0, 50, 100, 250, 500, 1000, 2500, 5500, 10_000, 15_000);
pub static EXPERTISE_INFO: LazyLock<StackingEnchantInfo> = LazyLock::new(|| StackingEnchantInfo::new("Expertise".to_string(), "expertise_kills".to_string(), "kills".to_string(), vec![0, 50, 100, 250, 500, 1000, 2500, 5500, 10000, 15000]));
/// java: private static final StackingEnchantInfo COMPACT_INFO = new StackingEnchantInfo("Compact", "compact_blocks", "blocks", 0, 100, 500, 1500, 5000, 15_000, 50_000, 150_000, 500_000, 1_000_000);
pub static COMPACT_INFO: LazyLock<StackingEnchantInfo> = LazyLock::new(|| StackingEnchantInfo::new("Compact".to_string(), "compact_blocks".to_string(), "blocks".to_string(), vec![0, 100, 500, 1500, 5000, 15000, 50000, 150000, 500000, 1000000]));
/// java: private static final StackingEnchantInfo CULTIVATING_INFO = new StackingEnchantInfo("Cultivating", "farmed_cultivating", "crops", 0, 1000, 5000, 25_000, 100_000, 250_000, 1_000_000, 2_500_000, 10_000_
pub static CULTIVATING_INFO: LazyLock<StackingEnchantInfo> = LazyLock::new(|| StackingEnchantInfo::new("Cultivating".to_string(), "farmed_cultivating".to_string(), "crops".to_string(), vec![0, 1000, 5000, 25000, 100000, 250000, 1000000, 2500000, 10000000, 25000000]));
/// java: private static final StackingEnchantInfo CHAMPION_INFO = new StackingEnchantInfo("Champion", "champion_combat_xp", "Combat XP", 0, 50_000, 100_000, 250_000, 500_000, 1_000_000, 1_500_000, 2_000_000, 2
pub static CHAMPION_INFO: LazyLock<StackingEnchantInfo> = LazyLock::new(|| StackingEnchantInfo::new("Champion".to_string(), "champion_combat_xp".to_string(), "Combat XP".to_string(), vec![0, 50000, 100000, 250000, 500000, 1000000, 1500000, 2000000, 2500000, 3000000]));
/// java: private static final StackingEnchantInfo HECATOMB_INFO = new StackingEnchantInfo("Hecatomb", "hecatomb_s_runs", "S runs", 0, 2, 5, 10, 20, 30, 40, 60, 80, 100);
pub static HECATOMB_INFO: LazyLock<StackingEnchantInfo> = LazyLock::new(|| StackingEnchantInfo::new("Hecatomb".to_string(), "hecatomb_s_runs".to_string(), "S runs".to_string(), vec![0, 2, 5, 10, 20, 30, 40, 60, 80, 100]));
/// java: private static final StackingEnchantInfo ABSORB_INFO = new StackingEnchantInfo("Absorb", "absorb_logs_chopped", "logs", 0, 1000, 5000, 25_000, 100_000, 250_000, 1_000_000, 2_500_000, 10_000_000, 25_00
pub static ABSORB_INFO: LazyLock<StackingEnchantInfo> = LazyLock::new(|| StackingEnchantInfo::new("Absorb".to_string(), "absorb_logs_chopped".to_string(), "logs".to_string(), vec![0, 1000, 5000, 25000, 100000, 250000, 1000000, 2500000, 10000000, 25000000]));
/// java: private static final StackingEnchantInfo TOXOPHILITE_INFO = new StackingEnchantInfo("Toxophilite", "toxophilite_combat_xp", "Combat XP", 0, 50_000, 100_000, 250_000, 500_000, 1_000_000, 1_500_000, 2_0
pub static TOXOPHILITE_INFO: LazyLock<StackingEnchantInfo> = LazyLock::new(|| StackingEnchantInfo::new("Toxophilite".to_string(), "toxophilite_combat_xp".to_string(), "Combat XP".to_string(), vec![0, 50000, 100000, 250000, 500000, 1000000, 1500000, 2000000, 2500000, 3000000]));

/// java: public class StackingEnchantProgressTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct StackingEnchantProgressTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for StackingEnchantProgressTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StackingEnchantProgressTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StackingEnchantProgressTooltip {
    /// java: public StackingEnchantProgressTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

}

impl SimpleTooltipAdder for StackingEnchantProgressTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for StackingEnchantProgressTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for StackingEnchantProgressTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        StackingEnchantProgressTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for StackingEnchantProgressTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        StackingEnchantProgressTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl StackingEnchantProgressTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.enable_stacking_enchant_progress);
    }

    /// StackingEnchantProgressTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        if custom_data.contains("enchantments") {
            let mut enchantments: Tag = custom_data.get_compound_or_empty("enchantments");
            let mut stacking_enchant_info: Option<StackingEnchantInfo> = None;
            let mut stacking_enchant_level: i32 = 0;
            for enchantment in enchantments.key_set() {
                if STACKING_ENCHANT_IDS.contains(&enchantment) {
                    stacking_enchant_info = Some(match enchantment.as_str() {
                        "expertise" => EXPERTISE_INFO.clone(),
                        "compact" => COMPACT_INFO.clone(),
                        "cultivating" => CULTIVATING_INFO.clone(),
                        "champion" => CHAMPION_INFO.clone(),
                        "hecatomb" => HECATOMB_INFO.clone(),
                        "absorb" => ABSORB_INFO.clone(),
                        "toxophilite" => TOXOPHILITE_INFO.clone(),
                        _ => {
                            panic!("IllegalStateException: {}", format!("Unexpected stacking enchant: {}", enchantment).to_string());
                        }
                    });
                    stacking_enchant_level = enchantments.get_int_or(&enchantment, 0);
                    break;
                }
            }
            if stacking_enchant_info.is_some() && stacking_enchant_level > 0 && stacking_enchant_level < 10 {
                let mut progress: i32 = custom_data.get_int_or(&stacking_enchant_info.as_ref().unwrap().field(), 0);
                let mut needed: i32 = stacking_enchant_info.as_ref().unwrap().ladder()[(stacking_enchant_level) as usize];
                let mut text: MutableComponent = ((((component::empty().append((component::literal(&format!("{} ", stacking_enchant_info.as_ref().unwrap().name()))).with_style_format(ChatFormatting::Gray))).append((component::translatable(&format!("enchantment.level.{}", stacking_enchant_level.wrapping_add(1)))).with_style_format(ChatFormatting::Gray)).append(component::literal(": ").with_style_format(ChatFormatting::Gray)).append((component::literal(&formatters::INTEGER_NUMBERS.get_mut().format((progress) as f64))).with_style_format(ChatFormatting::Red))).append((component::literal("/")).with_style_format(ChatFormatting::Gray))).append((component::literal(&formatters::INTEGER_NUMBERS.get_mut().format((needed) as f64))).with_style_format(ChatFormatting::Red))).append((component::literal(&format!(" {}", stacking_enchant_info.as_ref().unwrap().unit()))).with_style_format(ChatFormatting::Gray));
                lines.push(text.clone());
            }
        }
    }

    /// StackingEnchantProgressTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: private record StackingEnchantInfo(String name, String field, String unit, int... ladder)
#[derive(Clone, Debug, PartialEq)]
pub struct StackingEnchantInfo {
    /// java: String name
    pub name: String,
    /// java: String field
    pub field: String,
    /// java: String unit
    pub unit: String,
    /// java: int[] ladder
    pub ladder: Vec<i32>,
}

impl StackingEnchantInfo {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut field: String, mut unit: String, mut ladder: Vec<i32>) -> Self {
        if (ladder.len() as i32) != 10 {
            panic!("IllegalStateException: {}", format!("Ladder must have 10 entries but had only {}!", (ladder.len() as i32)).to_string());
        }
        Self { name, field, unit, ladder }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `field()`
    pub fn field(&self) -> String {
        self.field.clone()
    }

    /// java: the record accessor `unit()`
    pub fn unit(&self) -> String {
        self.unit.clone()
    }

    /// java: the record accessor `ladder()`
    pub fn ladder(&self) -> Vec<i32> {
        self.ladder.clone()
    }

}
