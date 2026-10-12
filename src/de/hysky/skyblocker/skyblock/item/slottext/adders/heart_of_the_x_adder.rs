//! Ported from de.hysky.skyblocker.skyblock.item.slottext.adders.HeartOfTheXAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdder;
use crate::de::hysky::skyblocker::skyblock::item::slottext::simple_slot_text_adder::SimpleSlotTextAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::ConfigInformation;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final Pattern LEVEL = Pattern.compile("Level (?<level>\\d+)/?(?<max>\\d+)?");
pub static LEVEL: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Level (?<level>\\d+)/?(?<max>\\d+)?"));

/// java: public abstract class HeartOfTheXAdder extends SimpleSlotTextAdder
#[derive(Clone)]
pub struct HeartOfTheXAdderBase {
    /// java: extends SimpleSlotTextAdder
    pub base: SimpleSlotTextAdderBase,
}

impl std::ops::Deref for HeartOfTheXAdderBase {
    type Target = SimpleSlotTextAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HeartOfTheXAdderBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HeartOfTheXAdderBase {
    /// java: protected HeartOfTheXAdder(@Language("RegExp") String titlePattern, @Nullable ConfigInformation configInformation) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title_pattern: String, mut config_information: Option<ConfigInformation>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn HeartOfTheXAdder>, __R)) -> (compat::Shared<dyn HeartOfTheXAdder>, __R) {
        let (_, (__this, __r)) = SimpleSlotTextAdderBase::new4_in(title_pattern.clone(), config_information.clone(), move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn SimpleSlotTextAdder>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of HeartOfTheXAdder (implemented by HeartOfTheXAdderBase and every subclass)
pub trait HeartOfTheXAdder: SimpleSlotTextAdder {
    fn heart_of_the_x_adder_base(&self) -> &HeartOfTheXAdderBase;
    fn heart_of_the_x_adder_base_mut(&mut self) -> &mut HeartOfTheXAdderBase;

    /// java: protected abstract boolean isNonLeveledItem(ItemStack stack); (dispatch: the body for this class)
    fn is_non_leveled_item(&self) -> fn(&compat::Shared<dyn HeartOfTheXAdder>, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool;

}

impl dyn HeartOfTheXAdder {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_body(__this: &compat::Shared<dyn HeartOfTheXAdder>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if slot_id < 0 || slot_id > 44 || { let __f = compat::dispatch(__this, |__t| __t.is_non_leveled_item()); __f(__this, stack) } {
            return vec![];
        }
        let mut lore: Vec<String> = stack.skyblocker_get_lore_strings();
        if lore.is_empty() {
            return vec![];
        }
        let mut level_line: String = lore[0].clone();
        let mut matcher: compat::regex::Matcher = LEVEL.matcher(&level_line);
        if !matcher.matches() {
            return vec![];
        }
        let mut level: String = matcher.group_named("level").unwrap();
        let mut is_maxed: bool = matcher.group_named("max").is_none();
        return SlotText::bottom_right_list(&component::literal(&level).with_color((if is_maxed { slot_text::GOLD } else { slot_text::CREAM })));
    }

    /// HeartOfTheXAdder.getText as the dispatch of SlotTextAdder.getText for the concrete class `__C`
    pub fn get_text_from<__C: HeartOfTheXAdder + 'static>(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        <dyn HeartOfTheXAdder>::get_text_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn HeartOfTheXAdder>), __p0, __p1, __p2)
    }

}

// HeartOfTheXAdder.getText implements SlotTextAdder.getText: it is emitted in each concrete subclass's `impl SlotTextAdder` (Rust has no inherited impls)
