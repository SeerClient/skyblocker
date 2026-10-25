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

/// `NumberFormat.getInstance()`: the default locale's NUMBER pattern `#,##0.###`. The port models no default locale
/// (`java::text` has ROOT and US, whose CLDR symbols are the same); US is used.
pub(crate) fn number_format_instance() -> mcrust::java::text::decimal_format::DecimalFormat {
    mcrust::java::text::decimal_format::DecimalFormat::new("#,##0.###", mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us))
}

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
        let mut rewards: Vec<compat::Shared<Reward>> = Vec::new();
        for entry in loot_list.iter().flat_map(|loot| CorpseLoot::rewards(loot)) {
            if let Some(reward) = rewards.iter().find(|reward| Reward::item_id(reward) == Reward::item_id(&entry)) {
                Reward::amount_1(reward, Reward::amount(reward).wrapping_add(Reward::amount(&entry)));
            } else {
                rewards.push(Reward::new(Reward::amount(&entry), Reward::item_id(&entry), Reward::price_per_unit(&entry))); // Add a clone of the entry so we don't modify the original
            }
        }
        // Sorts in-place: `comparingInt(RewardList::comparePriority).thenComparing(Reward::itemId)` (a stable sort, as `List.sort`)
        rewards.sort_by(|a, b| RewardList::compare_priority(a).cmp(&RewardList::compare_priority(b)).then_with(|| compat::compare_to(&Reward::item_id(a), &Reward::item_id(b)).cmp(&0)));
        // `Reference2IntArrayMap`: insertion-ordered, keyed by identity (an enum constant).
        let mut key_amounts: Vec<(CorpseType, i32)> = Vec::new();
        for loot in loot_list.iter() {
            let corpse_type = CorpseLoot::corpse_type(loot);
            match key_amounts.iter_mut().find(|(key, _)| *key == corpse_type) {
                Some((_, value)) => *value = value.wrapping_add(1),
                None => key_amounts.push((corpse_type, 1)),
            }
        }
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
        for entry in key_amounts.iter() {
            let __entry = (MultiEntry::new4(entry.0, entry.1) as compat::Shared<dyn AbstractEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        }
        let __entry = (EmptyEntry::new() as compat::Shared<dyn AbstractEntry>);
        compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        let __entry = (MultiEntry::new_1(profit) as compat::Shared<dyn AbstractEntry>);
        compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        __this
    }

    /// java: private static int comparePriority(Reward reward)
    pub fn compare_priority(reward: &compat::Shared<Reward>) -> i32 {
        // Lists according to the order in which they appear in the NAME2ID map
        let item_id = Reward::item_id(reward);
        let mut ids = CorpseProfitTracker::get_name2_id_map().values();
        let mut i: i32 = 0;
        while let Some(id) = ids.next() {
            if *id == item_id {
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
        compat::write(__this, |__t| __t.base.list.refresh_scroll_amount());
        // `entry.getX()` / `entry.getY()`: the row's box in the list (lent to this call), not the row object's copy,
        // which is only brought up to date when the row is drawn.
        let (__entries, __w, __h) = compat::write(__this, |__t| {
            let __l = &mut __t.base;
            let __entries = (0..__l.entry_count()).filter_map(|__i| compat::entry_at::<dyn AbstractEntry>(__l, __i).map(|__e| (__e, __l.list.entry(__i).get_x(), __l.list.entry(__i).get_y()))).collect::<Vec<_>>();
            (__entries, __l.list.width, __l.list.height)
        });
        for (entry, __x, __y) in __entries {
            ({ let __f = compat::dispatch(&entry, |__t| __t.reposition_elements()); __f(&entry, __x, __y, __w, __h) });
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
    /// The widgets are the row's (`__row`, which the game draws and clicks); the list holds their references.
    pub children: Vec<compat::WidgetRef>,
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

/// `children = List.of(a, b, ..)`: each widget joins the row's widget list (`__row`, which the game draws and clicks
/// through `ModListEntry::children`), and `children` holds the references. `List.of` throws on a null element, as
/// the `unwrap` at each call does.
pub fn set_children(base: &mut AbstractEntryBase, widgets: Vec<&mut compat::WidgetRef>) {
    let mut children = Vec::with_capacity(widgets.len());
    for widget in widgets {
        let index = base.__widgets.add(&mut base.__row, widget, true);
        children.push(compat::WidgetRef::Screen(index));
    }
    base.children = children;
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
    /// The port's widgets are neither `NarratableEntry` nor `GuiEventListener` objects; the game narrates and clicks
    /// the row's widgets through `ModListEntry::children`, and these answer the references (`children`).
    fn narratables(&self) -> fn(&compat::Shared<dyn AbstractEntry>) -> Vec<compat::WidgetRef> {
        <dyn AbstractEntry>::narratables_body
    }

    /// java: @Override public List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn AbstractEntry>) -> Vec<compat::WidgetRef> {
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
    pub fn narratables_body(__this: &compat::Shared<dyn AbstractEntry>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.abstract_entry_base().children.clone());
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn AbstractEntry>) -> Vec<compat::WidgetRef> {
        return compat::read(__this, |__t| __t.abstract_entry_base().children.clone());
    }

}

/// AbstractEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn AbstractEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.abstract_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self.abstract_entry_base_mut();
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
        let mut __text_widget = compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, text.clone()));
        compat::write(&__this, |__t| set_children(&mut __t.base, vec![&mut __text_widget]));
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
        let __y = compat::read(__this, |__t| __t.get_y()).wrapping_add(INNER_MARGIN);
        for mut child in compat::read(__this, |__t| __t.children.clone()) {
            compat::write(__this, |__t| { let __s = &mut __t.base; __s.__widgets.with(&mut __s.__row, &mut child, |__w| __w.set_y(__y)) });
            // `child.extractRenderState(..)`: the game draws the row's widgets after this body (`context.extract_widgets`).
        }
    }

    /// SingleEntry.extractContent as the dispatch of AbstractEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int height)
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        let (__width, __y) = compat::read(__this, |__t| (__t.get_width(), __t.get_y()));
        let mut child = compat::read(__this, |__t| __t.children[0].clone());
        compat::write(__this, |__t| { let __s = &mut __t.base; __s.__widgets.with(&mut __s.__row, &mut child, |child| {
            child.set_x((x.wrapping_add(INNER_MARGIN)).wrapping_add((__width.wrapping_sub(child.get_width())).wrapping_div(2)));
            child.set_y(__y.wrapping_add(INNER_MARGIN));
        }) });
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
        let __init_total_price = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} Coins", number_format_instance().format((amount as f64) * price_per_unit))).with_style_format(ChatFormatting::Gold))));
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: __init_amount,
                total_price: __init_total_price,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        if amount > 1 {
            { let __v = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} each", number_format_instance().format(price_per_unit))).with_style_format(ChatFormatting::Gray)))); compat::write(&__this, |__t| __t.price_per_unit = __v); }
            compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap(), __s.total_price.as_mut().unwrap(), __s.price_per_unit.as_mut().unwrap()]) });
        } else {
            compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap(), __s.total_price.as_mut().unwrap()]) });
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
        compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap()]) });
        __this
    }

    /// java: protected MultiEntry(double profit)
    pub fn new_1(mut profit: f64) -> compat::Shared<Self> {
        let __init_item_name = (Some(compat::string_widget(0, 0, -1, 9, component::literal("Total Profit").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Gold])))).map(compat::WidgetRef::value);
        let __init_total_price = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} Coins", number_format_instance().format(profit))).with_style_format(if profit > 0.0 { ChatFormatting::Green } else { ChatFormatting::Red }))));
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: None,
                total_price: __init_total_price,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.total_price.as_mut().unwrap()]) });
        __this
    }

    /// java: protected MultiEntry(CorpseType corpseType, int amount)
    pub fn new4(mut corpse_type: CorpseType, mut amount: i32) -> compat::Shared<Self> {
        let __init_item_name = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} Corpse Key Cost", compat::capitalize_fully(corpse_type.name()))).with_style_format(corpse_type.fields().color))));
        let __init_amount = (Some(compat::string_widget(0, 0, -1, 9, (component::literal(&format!("x{}", formatters::INTEGER_NUMBERS.get_mut().format((amount) as f64)))).with_style_format(ChatFormatting::Aqua)))).map(compat::WidgetRef::value);
        let mut price_per_key: f64 = corpse_type.get_key_price();
        // `(pricePerKey > 0 ? "-" + format(pricePerKey * amount) : 0) + " Coins"`: the else arm is the int 0.
        let mut price_string: String = format!("{} Coins", if price_per_key > (0 as f64) { format!("-{}", number_format_instance().format(price_per_key * (amount as f64))) } else { "0".to_string() });
        let mut price_text: MutableComponent = component::literal(&price_string).with_style_format((if price_per_key > (0 as f64) { ChatFormatting::Red } else { ChatFormatting::Gold }));
        let __init_total_price = (Some(compat::string_widget(0, 0, -1, 9, price_text.clone()))).map(compat::WidgetRef::value);
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: __init_amount,
                total_price: __init_total_price,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        if amount > 1 {
            { let __v = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} each", number_format_instance().format(price_per_key))).with_style_format(ChatFormatting::Gray)))); compat::write(&__this, |__t| __t.price_per_unit = __v); }
            compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap(), __s.total_price.as_mut().unwrap(), __s.price_per_unit.as_mut().unwrap()]) });
        } else {
            compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap(), __s.total_price.as_mut().unwrap()]) });
        }
        __this
    }

    /// java: private static void position(StringWidget widget, int start, int width, int y)
    pub fn position(widget: &mut AbstractWidget, mut start: i32, mut width: i32, mut y: i32) {
        widget.set_max_width(width, string_widget::TextOverflow::Scrolling);
        // `widget.getWidth()` is `StringWidget.getWidth`: `min(maxWidth, font.width(message))` once a max width is set.
        let text_width = compat::font().width_of(&widget.get_message_component());
        let widget_width = string_widget::width(widget.max_width, text_width);
        widget.set_x(start.wrapping_add((width.wrapping_sub(widget_width)).wrapping_div(2)));
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
        if compat::read(__this, |__t| __t.item_name.is_some()) {
            let __wa1 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa1)) });
            let __wa2 = (entry_width.wrapping_div(3)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN));
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_max_width(__wa2, string_widget::TextOverflow::Scrolling)) });
            // `itemName.extractRenderState(..)` (and each one below): the game draws the row's widgets after this body.
            // java: itemName.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
        if compat::read(__this, |__t| __t.amount.is_some()) {
            let __wa7 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.amount.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa7)) });
            // java: amount.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
        if compat::read(__this, |__t| __t.total_price.is_some()) {
            let __wa12 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.total_price.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa12)) });
            // java: totalPrice.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
        if compat::read(__this, |__t| __t.price_per_unit.is_some()) {
            let __wa17 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.price_per_unit.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa17)) });
            // java: pricePerUnit.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
    }

    /// MultiEntry.extractContent as the dispatch of AbstractEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int entryHeight)
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut width: i32, mut entry_height: i32) {
        if compat::read(__this, |__t| __t.item_name.is_some()) {
            let __wa1 = x.wrapping_add(INNER_MARGIN);
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_x(__wa1)) });
        }
        let mut entry_width: i32 = __this.lock().borrow().get_width();
        if compat::read(__this, |__t| __t.amount.is_some()) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.amount.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| MultiEntry::position(__w, (x.wrapping_add(entry_width.wrapping_div(3))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(6)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y)) });
        }
        if compat::read(__this, |__t| __t.total_price.is_some()) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.total_price.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| MultiEntry::position(__w, (x.wrapping_add(entry_width.wrapping_div(2))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(4)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y)) });
        }
        if compat::read(__this, |__t| __t.price_per_unit.is_some()) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.price_per_unit.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| MultiEntry::position(__w, (x.wrapping_add((3i32.wrapping_mul(entry_width)).wrapping_div(4))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(4)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y)) });
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
