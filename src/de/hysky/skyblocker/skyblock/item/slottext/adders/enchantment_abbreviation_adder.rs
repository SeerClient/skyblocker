//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.EnchantmentAbbreviationAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::ConfigInformation;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::nbt::compound_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final ConfigInformation CONFIG_INFORMATION = new ConfigInformation( "enchantment_abbreviation", "skyblocker.config.uiAndVisuals.slotText.enchantmentAbbreviation" );
pub static CONFIG_INFORMATION: compat::MainThread<ConfigInformation> = compat::MainThread::new(|| ConfigInformation::new2("enchantment_abbreviation".to_string(), "skyblocker.config.uiAndVisuals.slotText.enchantmentAbbreviation".to_string()));
/// java: @VisibleForTesting static final Object2ObjectMap<String, String> ENCHANTMENT_ABBREVIATIONS = new Object2ObjectOpenHashMap<>();
pub static ENCHANTMENT_ABBREVIATIONS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| {
    let mut value = std::collections::HashMap::new();
    value.insert("absorb".to_string(), "AB".to_string());
    value.insert("angler".to_string(), "AN".to_string());
    value.insert("aqua_affinity".to_string(), "AA".to_string());
    value.insert("arcane".to_string(), "WS".to_string());
    value.insert("aiming".to_string(), "DT".to_string());
    value.insert("bane_of_arthropods".to_string(), "BOA".to_string());
    value.insert("big_brain".to_string(), "BB".to_string());
    value.insert("blast_protection".to_string(), "BP".to_string());
    value.insert("blessing".to_string(), "BL".to_string());
    value.insert("bug_blender".to_string(), "BUG".to_string());
    value.insert("caster".to_string(), "CAT".to_string());
    value.insert("cayenne".to_string(), "CAY".to_string());
    value.insert("champion".to_string(), "CHM".to_string());
    value.insert("chance".to_string(), "CHN".to_string());
    value.insert("charm".to_string(), "CHR".to_string());
    value.insert("cleave".to_string(), "CL".to_string());
    value.insert("compact".to_string(), "COM".to_string());
    value.insert("corruption".to_string(), "COR".to_string());
    value.insert("counter_strike".to_string(), "CS".to_string());
    value.insert("critical".to_string(), "CR".to_string());
    value.insert("cubism".to_string(), "CUB".to_string());
    value.insert("cultivating".to_string(), "CUL".to_string());
    value.insert("dedication".to_string(), "DED".to_string());
    value.insert("delicate".to_string(), "DEL".to_string());
    value.insert("depth_strider".to_string(), "DS".to_string());
    value.insert("divine_gift".to_string(), "DG".to_string());
    value.insert("dragon_hunter".to_string(), "GV".to_string());
    value.insert("efficiency".to_string(), "EF".to_string());
    value.insert("ender_slayer".to_string(), "ES".to_string());
    value.insert("execute".to_string(), "EXE".to_string());
    value.insert("experience".to_string(), "EXP".to_string());
    value.insert("expertise".to_string(), "EPR".to_string());
    value.insert("feast".to_string(), "FE".to_string());
    value.insert("feather_falling".to_string(), "FF".to_string());
    value.insert("ferocious_mana".to_string(), "VIV".to_string());
    value.insert("fire_aspect".to_string(), "FA".to_string());
    value.insert("fire_protection".to_string(), "FPR".to_string());
    value.insert("first_strike".to_string(), "FS".to_string());
    value.insert("flame".to_string(), "FL".to_string());
    value.insert("forest_pledge".to_string(), "FPL".to_string());
    value.insert("fortune".to_string(), "FO".to_string());
    value.insert("frail".to_string(), "FR".to_string());
    value.insert("giant_killer".to_string(), "GK".to_string());
    value.insert("great_spook".to_string(), "GS".to_string());
    value.insert("green_thumb".to_string(), "GT".to_string());
    value.insert("growth".to_string(), "GR".to_string());
    value.insert("hardened_mana".to_string(), "HV".to_string());
    value.insert("harvesting".to_string(), "HRV".to_string());
    value.insert("hecatomb".to_string(), "HEC".to_string());
    value.insert("ice_cold".to_string(), "IC".to_string());
    value.insert("impaling".to_string(), "IMP".to_string());
    value.insert("infinite_quiver".to_string(), "IQ".to_string());
    value.insert("karma".to_string(), "KA".to_string());
    value.insert("knockback".to_string(), "KB".to_string());
    value.insert("lapidary".to_string(), "LAP".to_string());
    value.insert("lethality".to_string(), "LE".to_string());
    value.insert("life_steal".to_string(), "LS".to_string());
    value.insert("looting".to_string(), "LO".to_string());
    value.insert("luck".to_string(), "LU".to_string());
    value.insert("luck_of_the_sea".to_string(), "LTS".to_string());
    value.insert("lure".to_string(), "LR".to_string());
    value.insert("magmarizer".to_string(), "PY".to_string());
    value.insert("magnet".to_string(), "MAG".to_string());
    value.insert("mana_steal".to_string(), "MS".to_string());
    value.insert("mana_vampire".to_string(), "VV".to_string());
    value.insert("overload".to_string(), "OV".to_string());
    value.insert("paleontologist".to_string(), "PAL".to_string());
    value.insert("pesterminator".to_string(), "PS".to_string());
    value.insert("petalfall".to_string(), "PET".to_string());
    value.insert("piercing".to_string(), "PR".to_string());
    value.insert("piscary".to_string(), "PSC".to_string());
    value.insert("power".to_string(), "POW".to_string());
    value.insert("pristine".to_string(), "PRI".to_string());
    value.insert("projectile_protection".to_string(), "PP".to_string());
    value.insert("PROSECUTE".to_string(), "PRS".to_string());
    value.insert("prosperity".to_string(), "PSP".to_string());
    value.insert("protection".to_string(), "PRO".to_string());
    value.insert("punch".to_string(), "PU".to_string());
    value.insert("quantum".to_string(), "QUA".to_string());
    value.insert("quick_bite".to_string(), "QB".to_string());
    value.insert("rainbow".to_string(), "RA".to_string());
    value.insert("reflection".to_string(), "REF".to_string());
    value.insert("rejuvenate".to_string(), "RJ".to_string());
    value.insert("replenish".to_string(), "REP".to_string());
    value.insert("respiration".to_string(), "RES".to_string());
    value.insert("respite".to_string(), "RSP".to_string());
    value.insert("scavenger".to_string(), "SCV".to_string());
    value.insert("scuba".to_string(), "SCU".to_string());
    value.insert("sharpness".to_string(), "SH".to_string());
    value.insert("silk_touch".to_string(), "ST".to_string());
    value.insert("small_brain".to_string(), "SB".to_string());
    value.insert("smarty_pants".to_string(), "SP".to_string());
    value.insert("smelting_touch".to_string(), "SMT".to_string());
    value.insert("smite".to_string(), "SMI".to_string());
    value.insert("smoldering".to_string(), "SML".to_string());
    value.insert("snipe".to_string(), "SN".to_string());
    value.insert("spiked_hook".to_string(), "SPH".to_string());
    value.insert("stealth".to_string(), "STL".to_string());
    value.insert("strong_mana".to_string(), "SV".to_string());
    value.insert("sugar_rush".to_string(), "SR".to_string());
    value.insert("syphon".to_string(), "DR".to_string());
    value.insert("tabasco".to_string(), "TAB".to_string());
    value.insert("thorns".to_string(), "TH".to_string());
    value.insert("thunderbolt".to_string(), "TB".to_string());
    value.insert("thunderlord".to_string(), "TL".to_string());
    value.insert("tidal".to_string(), "TD".to_string());
    value.insert("titan_killer".to_string(), "TK".to_string());
    value.insert("toxophilite".to_string(), "TX".to_string());
    value.insert("transylvanian".to_string(), "TRN".to_string());
    value.insert("triple_strike".to_string(), "TS".to_string());
    value.insert("true_protection".to_string(), "TP".to_string());
    value.insert("turbo_cactus".to_string(), "TCC".to_string());
    value.insert("turbo_cane".to_string(), "TCN".to_string());
    value.insert("turbo_carrot".to_string(), "TCR".to_string());
    value.insert("turbo_coco".to_string(), "TCO".to_string());
    value.insert("turbo_melon".to_string(), "TME".to_string());
    value.insert("turbo_moonflower".to_string(), "TMO".to_string());
    value.insert("turbo_mushrooms".to_string(), "TMU".to_string());
    value.insert("turbo_potato".to_string(), "TPO".to_string());
    value.insert("turbo_pumpkin".to_string(), "TPU".to_string());
    value.insert("turbo_rose".to_string(), "TRO".to_string());
    value.insert("turbo_sunflower".to_string(), "TSU".to_string());
    value.insert("turbo_warts".to_string(), "TWA".to_string());
    value.insert("turbo_wheat".to_string(), "TWH".to_string());
    value.insert("vampirism".to_string(), "VMP".to_string());
    value.insert("venomous".to_string(), "VEN".to_string());
    value.insert("vicious".to_string(), "VIC".to_string());
    value
});
/// java: @VisibleForTesting static final Map<String, String> ULTIMATE_ENCHANTMENT_ABBREVIATIONS = new Object2ObjectOpenHashMap<>();
pub static ULTIMATE_ENCHANTMENT_ABBREVIATIONS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| {
    let mut value = std::collections::HashMap::new();
    value.insert("ultimate_bank".to_string(), "B".to_string());
    value.insert("ultimate_bobbin_time".to_string(), "BT".to_string());
    value.insert("ultimate_chimera".to_string(), "CH".to_string());
    value.insert("ultimate_combo".to_string(), "CO".to_string());
    value.insert("ultimate_crop_fever".to_string(), "CF".to_string());
    value.insert("ultimate_reiterate".to_string(), "DU".to_string());
    value.insert("ultimate_fatal_tempo".to_string(), "FT".to_string());
    value.insert("ultimate_first_impression".to_string(), "FI".to_string());
    value.insert("ultimate_flash".to_string(), "FL".to_string());
    value.insert("ultimate_flowstate".to_string(), "FLW".to_string());
    value.insert("ultimate_habanero_tactics".to_string(), "HT".to_string());
    value.insert("ultimate_inferno".to_string(), "IN".to_string());
    value.insert("ultimate_last_stand".to_string(), "LS".to_string());
    value.insert("ultimate_legion".to_string(), "L".to_string());
    value.insert("ultimate_missile".to_string(), "M".to_string());
    value.insert("ultimate_no_pain_no_gain".to_string(), "NP".to_string());
    value.insert("ultimate_one_for_all".to_string(), "OFA".to_string());
    value.insert("ultimate_refrigerate".to_string(), "RF".to_string());
    value.insert("ultimate_rend".to_string(), "RN".to_string());
    value.insert("ultimate_soul_eater".to_string(), "SE".to_string());
    value.insert("ultimate_sunset".to_string(), "SU".to_string());
    value.insert("ultimate_swarm".to_string(), "SW".to_string());
    value.insert("ultimate_the_one".to_string(), "TO".to_string());
    value.insert("ultimate_jerry".to_string(), "UJ".to_string());
    value.insert("ultimate_wise".to_string(), "UW".to_string());
    value.insert("ultimate_wisdom".to_string(), "W".to_string());
    value
});

/// java: public class EnchantmentAbbreviationAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct EnchantmentAbbreviationAdder {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for EnchantmentAbbreviationAdder {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EnchantmentAbbreviationAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EnchantmentAbbreviationAdder {
    /// java: public EnchantmentAbbreviationAdder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleSlotTextAdderBase::new6_in(Some(CONFIG_INFORMATION.get_mut().clone()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), __h)
        });
        __this
    }

    /// java: public static @Nullable CompoundTag getEnchantments(ItemStack stack)
    pub fn get_enchantments(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<Tag> {
        let mut nbt: Tag = ItemUtils::get_custom_data(stack);
        // TODO(fabric2seer): `is_empty` takes 1 argument(s) in the port (`tags: &BTreeMap<String, Tag>`), Java passes 0
        if compound_tag::is_empty() || !(nbt.contains("enchantments")) {
            return None;
        }
        let mut enchantments: Tag = nbt.get_compound_or_empty("enchantments");
        // TODO(fabric2seer): `size` takes 1 argument(s) in the port (`tags: &BTreeMap<String, Tag>`), Java passes 0
        if compound_tag::size() != 1 {
            return None;
        }
        return Some(enchantments);
    }

    /// java: private Optional<Component> getAbbreviation(String enchantmentId)
    pub fn get_abbreviation(__this: &compat::Shared<Self>, enchantment_id: &str) -> Option<MutableComponent> {
        if ENCHANTMENT_ABBREVIATIONS.contains_key(enchantment_id) {
            return Some(component::literal(ENCHANTMENT_ABBREVIATIONS.get(enchantment_id).cloned().as_deref().unwrap()).with_style_format(ChatFormatting::Blue));
        } else if ULTIMATE_ENCHANTMENT_ABBREVIATIONS.contains_key(enchantment_id) {
            return Some(component::literal(ULTIMATE_ENCHANTMENT_ABBREVIATIONS.get(enchantment_id).cloned().as_deref().unwrap()).with_style_format(ChatFormatting::LightPurple));
        }
        return None;
    }

}

impl SimpleSlotTextAdder for EnchantmentAbbreviationAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase {
        &self.base
    }
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for EnchantmentAbbreviationAdder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for EnchantmentAbbreviationAdder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

    /// java: @Override default boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn SlotTextAdder>::is_enabled_from::<Self>
    }

}

impl SlotTextAdder for EnchantmentAbbreviationAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        EnchantmentAbbreviationAdder::get_text_dyn
    }

    /// java: @Override public @Nullable ConfigInformation getConfigInformation() (dispatch: the body for this class)
    fn get_config_information(&self) -> fn(&compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_from::<Self>
    }

}

impl EnchantmentAbbreviationAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if !(stack.is_item(&items::items().get("enchanted_book"))) {
            return vec![];
        }
        let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
        if !(name == "Enchanted Book") {
            return vec![];
        }
        let Some(mut enchantments) = EnchantmentAbbreviationAdder::get_enchantments(stack) else {
            return vec![];
        };
        let mut enchantment_id: String = enchantments.key_set().into_iter().next().unwrap();
        return EnchantmentAbbreviationAdder::get_abbreviation(__this, &enchantment_id).map(|text| vec![SlotText::top_right(&text)]).unwrap_or_else(|| vec![]);
    }

    /// EnchantmentAbbreviationAdder.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
