//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.ColorTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::obtained_date_tooltip::ObtainedDateTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::core::component::data_component_map::DataComponentGetter as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final long WITHER_GLITCHED_AFTER_DATE = 1605830400000L;
pub const WITHER_GLITCHED_AFTER_DATE: i64 = 1605830400000;

/// java: public class ColorTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct ColorTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for ColorTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorTooltip {
    /// java: public ColorTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: private static void addExoticTooltip(List<Component> lines, ItemStack stack, String internalID, String colorHex, String expectedHex, String existingTooltip)
    pub fn add_exotic_tooltip(lines: &mut Vec<MutableComponent>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, internal_id: &str, color_hex: &str, expected_hex: &str, existing_tooltip: &str) {
        if true /* java: expected_hex != null: never null here */ && !(compat::equals_ignore_case(&color_hex, expected_hex)) && !(ColorTooltip::is_exception(internal_id, color_hex)) && !(ColorTooltip::intended_dyed(&ItemUtils::get_custom_data(stack))) {
            let mut r#type: DyeType = ColorTooltip::check_dye_type(stack, color_hex);
            lines.insert((1) as usize, (component::literal(&format!("{}{}(", existing_tooltip, compat::jstring(&ChatFormatting::DarkGray)))).append((r#type).get_translated_text()).append(component::literal(" - ")).append((component::literal(&format!("#{}", color_hex))).with_color(compat::integer_decode(&format!("0x{}", color_hex)))).append(component::literal(")").with_style_format(ChatFormatting::DarkGray)));
        }
    }

    /// java: private static String getExpectedHex(String id)
    pub fn get_expected_hex(id: &str) -> Option<String> {
        // TODO(fabric2seer): `.get(..)` on a T
        //   java: String color = TooltipInfoType.COLOR.getData().get(id);
        let mut color: String = todo!("fabric2seer: the initialiser of `color` (TODO above)");
        if true /* java: color != null: never null here */ {
            let mut rgbvalues: Vec<String> = compat::regex::split(&color, ",");
            return Some(format!("{:02X}{:02X}{:02X}", (rgbvalues[(0) as usize]).trim().parse::<i32>().expect("NumberFormatException"), (rgbvalues[(1) as usize]).trim().parse::<i32>().expect("NumberFormatException"), (rgbvalues[(2) as usize]).trim().parse::<i32>().expect("NumberFormatException")));
        } else {
            return None;
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static boolean isException(String id, String hex)
    pub fn is_exception(id: &str, hex: &str) -> bool {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (id) {
        //   java: 			case String it when it.startsWith("LEATHER") || it.equals("GHOST_BOOTS") || Constants.SEYMOUR_IDS.contains(it) -> true;
        //   java: 			case String it when it.startsWith("RANCHER") -> Constants.RANCHERS.contains(hex);
        //   java: 			case String it when it.contains("ADAPTIVE_CHESTPLATE") -> Constants.ADAPTIVE_CHEST.contains(hex);
        //   java: 			case String it when it.contains("ADAPTIVE") -> Constants.ADAPTIVE.contains(hex);
        //   java: 			case String it when it.contains("REAPER") -> Constants.REAPER.contains(hex);
        //   java: 			case String it when it.contains("FAIRY") -> Constants.FAIRY_HEXES.contains(hex);
        //   java: 			case String it when it.contains("CRYSTAL") -> Constants.CRYSTAL_HEXES.contains(hex);
        //   java: 			case String it when it.contains("SPOOK") -> Constants.SPOOK.contains(hex);
        //   java: 			default -> false;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static DyeType checkDyeType(ItemStack stack, String hex)
    pub fn check_dye_type(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, hex: &str) -> DyeType {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (hex) {
        //   java: 			case String it when Constants.CRYSTAL_HEXES.contains(it) -> DyeType.CRYSTAL;
        //   java: 			case String it when Constants.FAIRY_HEXES.contains(it) -> DyeType.FAIRY;
        //   java: 			case String it when Constants.OG_FAIRY_HEXES.contains(it) -> DyeType.OG_FAIRY;
        //   java: 			case String it when Constants.SPOOK.contains(it) && stack.getSkyblockId().startsWith("FAIRY_") -> DyeType.SPOOK;
        //   java: 			case String it when Constants.GLITCHED.contains(it) && isGlitched(stack, hex) -> DyeType.GLITCHED;
        //   java: 			default -> DyeType.EXOTIC;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static boolean intendedDyed(CompoundTag customData)
    pub fn intended_dyed(custom_data: &Tag) -> bool {
        return custom_data.contains("dye_item");
    }

    /// java: private static boolean isGlitched(ItemStack stack, String hex)
    pub fn is_glitched(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, hex: &str) -> bool {
        let mut id: String = stack.get_skyblock_id();
        if id.contains("WITHER") {
            return ColorTooltip::is_wither_glitched(&id, hex, ObtainedDateTooltip::get_long_timestamp(stack));
        }
        let mut misc_glitched_id: Option<String> = constants::MISC_GLITCHED_HEXES.get(hex).cloned();
        return (misc_glitched_id.is_some() && id.starts_with(misc_glitched_id.as_deref().unwrap()));
    }

    /// java: private static boolean isWitherGlitched(String id, String hex, long obtained)
    pub fn is_wither_glitched(id: &str, hex: &str, mut obtained: i64) -> bool {
        if hex == "000000" && obtained < WITHER_GLITCHED_AFTER_DATE {
            return false;
        }
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (id) {
        //   java: 			case String it when it.contains("CHESTPLATE") -> Constants.WITHER_CHESTPLATE_HEXES.containsKey(hex) && Constants.WITHER_CHESTPLATE_HEXES.containsValue(it) && !Constants.WITHER_CHESTPLATE_HEXES.get(hex).equals(it);
        //   java: 			case String it when it.contains("LEGGINGS") -> Constants.WITHER_LEGGINGS_HEXES.containsKey(hex) && Constants.WITHER_LEGGINGS_HEXES.containsValue(it) && !Constants.WITHER_LEGGINGS_HEXES.get(hex).equals(it);
        //   java: 			case String it when it.contains("BOOTS") -> Constants.WITHER_BOOTS_HEXES.containsKey(hex) && Constants.WITHER_BOOTS_HEXES.containsValue(it) && !Constants.WITHER_BOOTS_HEXES.get(hex).equals(it);
        //   java:
        //   java: 			default -> false;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl SimpleTooltipAdder for ColorTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for ColorTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for ColorTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        ColorTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for ColorTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        ColorTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl ColorTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return ({ let __r = tooltip_info_type::COLOR.get_mut(); let __f = compat::dispatch(__r, |__t| __t.is_tooltip_enabled()); __f(&compat::view(__r, |a| a as compat::Shared<dyn TooltipInfoType>)) });
    }

    /// ColorTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut internal_id: String = stack.get_skyblock_id();
        if tooltip_info_type::COLOR.get_mut().lock().borrow().has_or_null_warning(&internal_id) && stack.has(mcrust::net::minecraft::core::component::data_components::dyed_color()) {
            // TODO(fabric2seer): `.rgb(..)` on a value of unknown type
            //   java: int dyeColor = stack.get(DataComponents.DYED_COLOR).rgb() & 0x00FFFFFF;
            let mut dye_color: i32 = todo!("fabric2seer: the initialiser of `dyeColor` (TODO above)");
            let mut color_hex: String = format!("{:06X}", dye_color);
            let mut expected_hex: String = ColorTooltip::get_expected_hex(&internal_id).unwrap();
            let mut correct_line: bool = false;
            for text in lines.iter() {
                let mut existing_tooltip: String = format!("{} ", text.get_string());
                if existing_tooltip.starts_with("Color: ") {
                    correct_line = true;
                    ColorTooltip::add_exotic_tooltip(lines, stack, &internal_id, &color_hex, &expected_hex, &existing_tooltip);
                    break;
                }
            }
            if !correct_line {
                ColorTooltip::add_exotic_tooltip(lines, stack, &internal_id, &color_hex, &expected_hex, "");
            }
        }
    }

    /// ColorTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: private enum DyeType implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DyeType {
    Crystal,
    Fairy,
    OgFairy,
    Spook,
    Glitched,
    Exotic,
}

impl DyeType {
    pub const VALUES: [DyeType; 6] = [DyeType::Crystal, DyeType::Fairy, DyeType::OgFairy, DyeType::Spook, DyeType::Glitched, DyeType::Exotic];

    /// java: values()
    pub fn values() -> Vec<DyeType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DyeType::Crystal => "CRYSTAL",
            DyeType::Fairy => "FAIRY",
            DyeType::OgFairy => "OG_FAIRY",
            DyeType::Spook => "SPOOK",
            DyeType::Glitched => "GLITCHED",
            DyeType::Exotic => "EXOTIC",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DyeType {
        match name {
            "CRYSTAL" => DyeType::Crystal,
            "FAIRY" => DyeType::Fairy,
            "OG_FAIRY" => DyeType::OgFairy,
            "SPOOK" => DyeType::Spook,
            "GLITCHED" => DyeType::Glitched,
            "EXOTIC" => DyeType::Exotic,
            _ => panic!("IllegalArgumentException: No enum constant DyeType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DyeTypeFields {
        static FIELDS: LazyLock<Vec<DyeTypeFields>> = LazyLock::new(|| vec![
            DyeTypeFields::new("crystal".to_string(), ChatFormatting::Aqua),
            DyeTypeFields::new("fairy".to_string(), ChatFormatting::LightPurple),
            DyeTypeFields::new("og_fairy".to_string(), ChatFormatting::DarkPurple),
            DyeTypeFields::new("spook".to_string(), ChatFormatting::Red),
            DyeTypeFields::new("glitched".to_string(), ChatFormatting::Blue),
            DyeTypeFields::new("exotic".to_string(), ChatFormatting::Gold),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: public MutableComponent getTranslatedText()
    pub fn get_translated_text(&self) -> MutableComponent {
        return (component::translatable(&format!("skyblocker.exotic.{}", self.fields().name))).with_style_format(self.fields().formatting);
    }

}

impl std::fmt::Display for DyeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each DyeType constant
pub struct DyeTypeFields {
    pub name: String,
    pub formatting: ChatFormatting,
}

impl DyeTypeFields {
    fn new(mut name: String, mut formatting: ChatFormatting) -> Self {
        // this.name = name; this.formatting = formatting;
        Self {
            name: name.clone(),
            formatting: formatting,
        }
    }

}
