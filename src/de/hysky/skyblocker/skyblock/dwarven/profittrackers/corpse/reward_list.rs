//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.corpse.RewardList by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::corpse_type::CorpseType;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_list::CorpseList;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_loot::CorpseLoot;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_tracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_tracker::CorpseProfitTracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward::Reward;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: private static final int BORDER_COLOR = 0xFF6C7086;
pub const BORDER_COLOR: i32 = 0xFF6C7086u32 as i32;
/// java: private static final int INNER_MARGIN = 2;
pub const INNER_MARGIN: i32 = 2;

/// java: public class RewardList extends ContainerObjectSelectionList<RewardList.AbstractEntry>
pub struct RewardList {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for RewardList {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for RewardList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl RewardList {
    /// java: public RewardList(Minecraft client, int width, int height, int y, int entryHeight, List<CorpseLoot> lootList)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32, mut entry_height: i32, mut loot_list: Vec<compat::Shared<CorpseLoot>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, entry_height),
        });
        if loot_list.is_empty() {
            RewardList::add_empty_entry(&__this);
            RewardList::add_empty_entry(&__this);
            RewardList::add_empty_entry(&__this);
            let __entry = (SingleEntry::new_2(component::translatable("skyblocker.corpseTracker.emptyHistory").with_style_format(ChatFormatting::Red), false) as compat::Shared<dyn AbstractEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            return __this.clone();
        }
        // TODO(fabric2seer): Stream.collect
        //   java: List<Reward> rewards = lootList.stream()
        //   java: 				.flatMap(loot -> loot.rewards().stream())
        //   java: 				.collect(ObjectArrayList::new,
        //   java: 						(list, entry) -> {
        //   java: 							if (list.stream().anyMatch(reward -> reward.itemId().equals(entry.itemId()))) {
        //   java: 								list.stream().filter(reward -> reward.itemId().equals(entry.itemId())).findFirst().ifPresent(reward -> reward.amount(reward.amount() + entry.amount()));
        //   java: 							} else {
        //   java: 								list.add(new Reward(entry.amount(), entry.itemId(), entry.pricePerUnit())); // Add a clone of the entry so we don't modify the original
        //   java: 							}
        //   java: 						}, ObjectArrayList::addAll);
        let mut rewards: Vec<compat::Shared<Reward>> = todo!("fabric2seer: the initialiser of `rewards` (TODO above)");
        rewards.sort_by(|a, b| (|a, b| { let key = |__a0| RewardList::compare_priority(__a0); key(a).cmp(&key(b)) })(a, b).then_with(|| (|a, b| { let key = |__a0| Reward::item_id(__a0); key(a).cmp(&key(b)) })(a, b)));
        // TODO(fabric2seer): Stream.collect
        //   java: Reference2IntArrayMap<CorpseType> keyAmounts = lootList.stream()
        //   java: 				.collect(Reference2IntArrayMap::new,
        //   java: 						(map, loot) -> map.mergeInt(loot.corpseType(), 1, Integer::sum),
        //   java: 						Reference2IntArrayMap::putAll);
        let mut key_amounts: std::collections::HashMap<CorpseType, i32> = todo!("fabric2seer: the initialiser of `keyAmounts` (TODO above)");
        let mut profit: f64 = loot_list.iter().cloned().map(|__a0| CorpseLoot::profit(&__a0)).sum::<f64>();
        for reward in rewards.iter() {
            let mut item_name: MutableComponent = CorpseList::get_item_name(&Reward::item_id(reward));
            if corpse_profit_tracker::PRICELESS_ITEMS.iter().any(|e| *e == Reward::item_id(reward)) {
                let __entry = (MultiEntry::new2(item_name.clone(), Reward::amount(reward)) as compat::Shared<dyn AbstractEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            } else {
                let __entry = (MultiEntry::new(item_name.clone(), Reward::amount(reward), Reward::price_per_unit(reward)) as compat::Shared<dyn AbstractEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
        }
        let __entry = (EmptyEntry::new() as compat::Shared<dyn AbstractEntry>);
        compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        // TODO(fabric2seer): Map.reference2IntEntrySet(..)
        //   java: for (var entry : keyAmounts.reference2IntEntrySet()) {
        //   java: 			addEntry(new MultiEntry(entry.getKey(), entry.getIntValue()));
        //   java: 		}
        let __entry = (EmptyEntry::new() as compat::Shared<dyn AbstractEntry>);
        compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        let __entry = (MultiEntry::new_1(profit) as compat::Shared<dyn AbstractEntry>);
        compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        __this
    }

    /// java: private static int comparePriority(Reward reward)
    pub fn compare_priority(reward: &compat::Shared<Reward>) -> i32 {
        let mut ids: crate::unresolved::ObjectIterator = CorpseProfitTracker::get_name2_id_map().values().cloned();
        let mut i: i32 = 0;
        // TODO(fabric2seer): the condition: `.hasNext(..)` on a ObjectIterator
        //   java: ids.hasNext(..)
        while todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): the condition: `.next(..)` on a ObjectIterator
            //   java: ids.next(..).equals(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                return i;
            }
            i += 1;
        }
        logger().warn(&format!("Item ID `{}` not found in NAME2ID map", Reward::item_id(reward)));
        return i32::MAX;
    }

    /// java: private void addEmptyEntry()
    pub fn add_empty_entry(__this: &compat::Shared<Self>) {
        let __entry = (EmptyEntry::new() as compat::Shared<dyn AbstractEntry>);
        compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 500i32.min((__this.lock().borrow().get_width()).wrapping_sub(24));
    }

    /// java: @Override public void refreshScrollAmount()
    pub fn refresh_scroll_amount(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().base.list.refresh_scroll_amount();
        for entry in compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractEntry>(__l) }) {
            let __o1 = entry.clone();
            let __o2 = entry.clone();
            ({ let __f = compat::dispatch(&entry, |__t| __t.reposition_elements()); __f(&entry, compat::write(&__o1, |__t| __t.abstract_entry_base_mut().get_x()), compat::write(&__o2, |__t| __t.abstract_entry_base_mut().get_y()), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height)) });
        }
    }

}

/// RewardList's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for RewardList {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// RewardList's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<RewardList> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| RewardList::get_row_width(__h))
    }

}

// NOTE(fabric2seer): getRowWidth, refreshScrollAmount override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: abstract static class AbstractEntry extends ContainerObjectSelectionList.Entry<AbstractEntry>
pub struct AbstractEntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: protected List<AbstractWidget> children;
    pub children: Vec<AbstractWidget>,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractEntryBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractEntry>, __R)) -> (compat::Shared<dyn AbstractEntry>, __R) {
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            children: Default::default(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractEntry (implemented by AbstractEntryBase and every subclass)
pub trait AbstractEntry: compat::DynAny {
    fn abstract_entry_base(&self) -> &AbstractEntryBase;
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn AbstractEntry>, &mut GuiGraphicsExtractor, i32, i32, bool, f32) {
        <dyn AbstractEntry>::extract_content_body
    }

    /// java: public void repositionElements(int x, int y, int width, int height) (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractEntry>, i32, i32, i32, i32) {
        <dyn AbstractEntry>::reposition_elements_body
    }

    /// java: @Override public List<? extends NarratableEntry> narratables() (dispatch: the body for this class)
    fn narratables(&self) -> fn(&compat::Shared<dyn AbstractEntry>) -> Vec<Box<dyn NarratableEntry>> {
        <dyn AbstractEntry>::narratables_body
    }

    /// java: @Override public List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn AbstractEntry>) -> Vec<Box<dyn GuiEventListener>> {
        <dyn AbstractEntry>::children_body
    }

}

impl dyn AbstractEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_body(__this: &compat::Shared<dyn AbstractEntry>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
    }

    /// java: public void repositionElements(int x, int y, int width, int height)
    pub fn reposition_elements_body(__this: &compat::Shared<dyn AbstractEntry>, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables_body(__this: &compat::Shared<dyn AbstractEntry>) -> Vec<Box<dyn NarratableEntry>> {
        return compat::read(__this, |__t| __t.abstract_entry_base().children.clone());
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn AbstractEntry>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.abstract_entry_base().children.clone());
    }

}

/// AbstractEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn AbstractEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.abstract_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.abstract_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// AbstractEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn AbstractEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_list::MultiEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }); } if let Some(__c) = compat::try_view::<EmptyEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }); } if let Some(__c) = compat::try_view::<MultiEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }); } if let Some(__c) = compat::try_view::<SingleEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: protected static class EmptyEntry extends AbstractEntry
pub struct EmptyEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
}

impl std::ops::Deref for EmptyEntry {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EmptyEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EmptyEntry {
    /// java: public EmptyEntry()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        { let __v = vec![]; compat::write(&__this, |__t| __t.children = __v); }
        __this
    }

}

impl AbstractEntry for EmptyEntry {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// EmptyEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for EmptyEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// EmptyEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<EmptyEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: protected static class SingleEntry extends AbstractEntry
pub struct SingleEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: private boolean drawBorder = true;
    pub draw_border: bool,
}

impl std::ops::Deref for SingleEntry {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SingleEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SingleEntry {
    /// java: protected SingleEntry(Component text)
    pub fn new(mut text: MutableComponent) -> compat::Shared<Self> {
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                draw_border: true,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        { let __v = vec![compat::string_widget(0, 0, -1, 9, text.clone())]; compat::write(&__this, |__t| __t.children = __v); }
        __this
    }

    /// java: protected SingleEntry(Component text, boolean drawBorder)
    pub fn new_2(mut text: MutableComponent, mut draw_border: bool) -> compat::Shared<Self> {
        let __this = Self::new(text.clone());
        { let __v = draw_border; compat::write(&__this, |__t| __t.draw_border = __v); }
        __this
    }

}

impl AbstractEntry for SingleEntry {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn AbstractEntry>, &mut GuiGraphicsExtractor, i32, i32, bool, f32) {
        SingleEntry::extract_content_dyn
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int height) (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractEntry>, i32, i32, i32, i32) {
        SingleEntry::reposition_elements_dyn
    }

}

impl SingleEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        if compat::read(__this, |__t| __t.draw_border) {
            GuiHelper::border(graphics, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), (__this.lock().borrow().get_height()).wrapping_add(1), BORDER_COLOR);
        }
        for child in compat::read(__this, |__t| __t.children.clone()) {
            child.set_y((__this.lock().borrow().get_y()).wrapping_add(INNER_MARGIN));
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: child.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        }
    }

    /// SingleEntry.extractContent as the dispatch of AbstractEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int height)
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        let child = &mut __this.lock().borrow().children[0];
        child.set_x((x.wrapping_add(INNER_MARGIN)).wrapping_add(((__this.lock().borrow().get_width()).wrapping_sub(child.get_width())).wrapping_div(2)));
        child.set_y((__this.lock().borrow().get_y()).wrapping_add(INNER_MARGIN));
    }

    /// SingleEntry.repositionElements as the dispatch of AbstractEntry.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: i32, __p1: i32, __p2: i32, __p3: i32) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// SingleEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for SingleEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// SingleEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<SingleEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: protected static class MultiEntry extends AbstractEntry
pub struct MultiEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: protected @Nullable StringWidget itemName;
    pub item_name: Option<compat::WidgetRef>,
    /// java: protected @Nullable StringWidget amount;
    pub amount: Option<compat::WidgetRef>,
    /// java: protected @Nullable StringWidget totalPrice;
    pub total_price: Option<compat::WidgetRef>,
    /// java: protected @Nullable StringWidget pricePerUnit = null;
    pub price_per_unit: Option<compat::WidgetRef>,
}

impl std::ops::Deref for MultiEntry {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MultiEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MultiEntry {
    /// java: protected MultiEntry(Component itemName, int amount, double pricePerUnit)
    pub fn new(mut item_name: MutableComponent, mut amount: i32, mut price_per_unit: f64) -> compat::Shared<Self> {
        let __init_item_name = (Some(compat::string_widget(0, 0, -1, 9, item_name.clone()))).map(compat::WidgetRef::value);
        let __init_amount = (Some(compat::string_widget(0, 0, -1, 9, (component::literal(&format!("x{}", amount))).with_style_format(ChatFormatting::Aqua)))).map(compat::WidgetRef::value);
        // TODO(fabric2seer): static `NumberFormat.getInstance` has no Rust counterpart in the port
        //   java: this.totalPrice = new StringWidget(Component.literal(NumberFormat.getInstance().format(amount * pricePerUnit) + " Coins").withStyle(ChatFormatting.GOLD), Minecraft.getInstance().font);
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: __init_amount,
                total_price: todo!("fabric2seer: the field value (TODO above)"),
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        if amount > 1 {
            // TODO(fabric2seer): `.format(..)` on a value of unknown type
            //   java: this.pricePerUnit = new StringWidget(Component.literal(NumberFormat.getInstance().format(pricePerUnit) + " each").withStyle(ChatFormatting.GRAY), Minecraft.getInstance().font);
            { let __v = vec![compat::write(&__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.price_per_unit.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()]; compat::write(&__this, |__t| __t.children = __v); }
        } else {
            { let __v = vec![compat::write(&__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()]; compat::write(&__this, |__t| __t.children = __v); }
        }
        __this
    }

    /// java: protected MultiEntry(Component itemName, int amount)
    pub fn new2(mut item_name: MutableComponent, mut amount: i32) -> compat::Shared<Self> {
        let __init_item_name = (Some(compat::string_widget(0, 0, -1, 9, item_name.clone()))).map(compat::WidgetRef::value);
        let __init_amount = (Some(compat::string_widget(0, 0, -1, 9, (component::literal(&format!("x{}", formatters::INTEGER_NUMBERS.get_mut().format((amount) as f64)))).with_style_format(ChatFormatting::Aqua)))).map(compat::WidgetRef::value);
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: __init_amount,
                total_price: None,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        { let __v = vec![compat::write(&__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()]; compat::write(&__this, |__t| __t.children = __v); }
        __this
    }

    /// java: protected MultiEntry(double profit)
    pub fn new_1(mut profit: f64) -> compat::Shared<Self> {
        let __init_item_name = (Some(compat::string_widget(0, 0, -1, 9, component::literal("Total Profit").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Gold])))).map(compat::WidgetRef::value);
        // TODO(fabric2seer): static `NumberFormat.getInstance` has no Rust counterpart in the port
        //   java: this.totalPrice = new StringWidget(Component.literal(NumberFormat.getInstance().format(profit) + " Coins").withStyle(profit > 0 ? ChatFormatting.GREEN : ChatFormatting.RED), Minecraft.getInstance().font);
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: None,
                total_price: todo!("fabric2seer: the field value (TODO above)"),
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        { let __v = vec![compat::write(&__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()]; compat::write(&__this, |__t| __t.children = __v); }
        __this
    }

    /// java: protected MultiEntry(CorpseType corpseType, int amount)
    pub fn new4(mut corpse_type: CorpseType, mut amount: i32) -> compat::Shared<Self> {
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: this.itemName = new StringWidget(Component.literal(WordUtils.capitalizeFully(corpseType.name()) + " Corpse Key Cost").withStyle(corpseType.color), Minecraft.getInstance().font);
        let __init_amount = (Some(compat::string_widget(0, 0, -1, 9, (component::literal(&format!("x{}", formatters::INTEGER_NUMBERS.get_mut().format((amount) as f64)))).with_style_format(ChatFormatting::Aqua)))).map(compat::WidgetRef::value);
        let mut price_per_key: f64 = corpse_type.get_key_price();
        // TODO(fabric2seer): `.format(..)` on a value of unknown type
        //   java: String priceString = (pricePerKey > 0 ? "-" + NumberFormat.getInstance().format(pricePerKey * amount) : 0) + " Coins";
        let mut price_string: String = todo!("fabric2seer: the initialiser of `priceString` (TODO above)");
        let mut price_text: MutableComponent = component::literal(&price_string).with_style_format((if price_per_key > (0 as f64) { ChatFormatting::Red } else { ChatFormatting::Gold }));
        let __init_total_price = (Some(compat::string_widget(0, 0, -1, 9, price_text.clone()))).map(compat::WidgetRef::value);
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: todo!("fabric2seer: the field value (TODO above)"),
                amount: __init_amount,
                total_price: __init_total_price,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        if amount > 1 {
            // TODO(fabric2seer): `.format(..)` on a value of unknown type
            //   java: this.pricePerUnit = new StringWidget(Component.literal(NumberFormat.getInstance().format(pricePerKey) + " each").withStyle(ChatFormatting.GRAY), Minecraft.getInstance().font);
            { let __v = vec![compat::write(&__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.price_per_unit.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()]; compat::write(&__this, |__t| __t.children = __v); }
        } else {
            { let __v = vec![compat::write(&__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap(), compat::write(&__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).unwrap()]; compat::write(&__this, |__t| __t.children = __v); }
        }
        __this
    }

    /// java: private static void position(StringWidget widget, int start, int width, int y)
    pub fn position(widget: &mut AbstractWidget, mut start: i32, mut width: i32, mut y: i32) {
        // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
        //   java: widget.setMaxWidth(width, StringWidget.TextOverflow.SCROLLING);
        // TODO(fabric2seer): `width` takes 2 argument(s) in the port (`max_width: i32, text_width: i32`), Java passes 0
        // rust: widget.set_x(start.wrapping_add((width.wrapping_sub(string_widget::width())).wrapping_div(2)));
        widget.set_y(y);
    }

}

impl AbstractEntry for MultiEntry {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn AbstractEntry>, &mut GuiGraphicsExtractor, i32, i32, bool, f32) {
        MultiEntry::extract_content_dyn
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int entryHeight) (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractEntry>, i32, i32, i32, i32) {
        MultiEntry::reposition_elements_dyn
    }

}

impl MultiEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        let mut x: i32 = __this.lock().borrow().get_x();
        let mut y: i32 = __this.lock().borrow().get_y();
        let mut entry_width: i32 = __this.lock().borrow().get_width();
        let mut entry_height: i32 = __this.lock().borrow().get_height();
        GuiHelper::border(graphics, x, y, entry_width, entry_height.wrapping_add(1), BORDER_COLOR);
        GuiHelper::border(graphics, x.wrapping_add(entry_width.wrapping_div(3)), y, (entry_width.wrapping_div(6)).wrapping_add(2), entry_height.wrapping_add(1), BORDER_COLOR);
        GuiHelper::border(graphics, x.wrapping_add(entry_width.wrapping_div(2)), y, entry_width.wrapping_div(4), entry_height.wrapping_add(1), BORDER_COLOR);
        let mut entry_y: i32 = y.wrapping_add(INNER_MARGIN);
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            let __wa1 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa1)) });
            // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
            //   java: itemName.setMaxWidth(entryWidth / 3 - 2 * INNER_MARGIN, StringWidget.TextOverflow.SCROLLING);
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa3 = graphics.clone();
            // rust: let __wa4 = mouse_x;
            // rust: let __wa5 = mouse_y;
            // rust: let __wa6 = a;
            // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa4, __wa5, __wa6)) });
        }
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            let __wa7 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.amount.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa7)) });
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa8 = graphics.clone();
            // rust: let __wa9 = mouse_x;
            // rust: let __wa10 = mouse_y;
            // rust: let __wa11 = a;
            // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.amount.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa9, __wa10, __wa11)) });
        }
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            let __wa12 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.total_price.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa12)) });
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa13 = graphics.clone();
            // rust: let __wa14 = mouse_x;
            // rust: let __wa15 = mouse_y;
            // rust: let __wa16 = a;
            // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.total_price.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa14, __wa15, __wa16)) });
        }
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.price_per_unit.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            let __wa17 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.price_per_unit.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa17)) });
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa18 = graphics.clone();
            // rust: let __wa19 = mouse_x;
            // rust: let __wa20 = mouse_y;
            // rust: let __wa21 = a;
            // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.price_per_unit.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa19, __wa20, __wa21)) });
        }
    }

    /// MultiEntry.extractContent as the dispatch of AbstractEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int entryHeight)
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut width: i32, mut entry_height: i32) {
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.item_name.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            let __wa1 = x.wrapping_add(INNER_MARGIN);
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_x(__wa1)) });
        }
        let mut entry_width: i32 = __this.lock().borrow().get_width();
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            MultiEntry::position(compat::write(__this, |__t| { let __s = &mut *__t; match __s.amount.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).as_mut().unwrap(), (x.wrapping_add(entry_width.wrapping_div(3))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(6)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y);
        }
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            MultiEntry::position(compat::write(__this, |__t| { let __s = &mut *__t; match __s.total_price.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).as_mut().unwrap(), (x.wrapping_add(entry_width.wrapping_div(2))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(4)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y);
        }
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.price_per_unit.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).is_some() {
            MultiEntry::position(compat::write(__this, |__t| { let __s = &mut *__t; match __s.price_per_unit.as_mut() { Some(__r) => Some(__s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.clone())), None => None } }).as_mut().unwrap(), (x.wrapping_add((3i32.wrapping_mul(entry_width)).wrapping_div(4))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(4)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y);
        }
    }

    /// MultiEntry.repositionElements as the dispatch of AbstractEntry.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: i32, __p1: i32, __p2: i32, __p3: i32) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// MultiEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for MultiEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// MultiEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<MultiEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}
