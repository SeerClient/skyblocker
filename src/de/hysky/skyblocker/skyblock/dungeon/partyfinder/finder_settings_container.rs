//! Ported from de.hysky.skyblocker.skyblock.dungeon.partyfinder.FinderSettingsContainer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::option_dropdown_widget::OptionDropdownWidget;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::ranged_value_widget::RangedValueWidget;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::ranged_value_widget::State;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::level::block::entity::sign_block_entity::SignBlockEntity;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class FinderSettingsContainer extends AbstractContainerWidget
#[derive(Clone)]
pub struct FinderSettingsContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private boolean isInitialized = false;
    pub is_initialized: bool,
    /// java: private @Nullable OptionDropdownWidget floorSelector;
    pub floor_selector: Option<compat::Shared<OptionDropdownWidget>>,
    /// java: private @Nullable OptionDropdownWidget dungeonTypeSelector;
    pub dungeon_type_selector: Option<compat::Shared<OptionDropdownWidget>>,
    /// java: private @Nullable OptionDropdownWidget sortGroupsSelector;
    pub sort_groups_selector: Option<compat::Shared<OptionDropdownWidget>>,
    /// java: private @Nullable RangedValueWidget classLevelRange;
    pub class_level_range: Option<compat::Shared<RangedValueWidget>>,
    /// java: private @Nullable RangedValueWidget dungeonLevelRange;
    pub dungeon_level_range: Option<compat::Shared<RangedValueWidget>>,
    /// java: private @Nullable AbstractContainerWidget currentlyOpenedOption = null;
    pub currently_opened_option: Option<OptionWidget>,
    /// java: private final List<AbstractContainerWidget> initializedWidgets = new ArrayList<>();
    pub initialized_widgets: Vec<OptionWidget>,
}

/// java: an `AbstractContainerWidget` reference this container holds - always one of its option widgets
/// (`OptionDropdownWidget extends AbstractSelectionList extends AbstractContainerWidget`, `RangedValueWidget extends
/// AbstractContainerWidget`). The handle of the object, so `==` is identity and a call dispatches to the override.
#[derive(Clone)]
pub enum OptionWidget {
    Dropdown(compat::Shared<OptionDropdownWidget>),
    Ranged(compat::Shared<RangedValueWidget>),
}

impl OptionWidget {
    /// java: `a == b` on two `AbstractContainerWidget` references
    pub fn ptr_eq(&self, other: &OptionWidget) -> bool {
        match (self, other) {
            (OptionWidget::Dropdown(a), OptionWidget::Dropdown(b)) => Arc::ptr_eq(a, b),
            (OptionWidget::Ranged(a), OptionWidget::Ranged(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }

    /// java: `widget.mouseClicked(click, doubled)` - both classes override it
    pub fn mouse_clicked(&self, click: &MouseButtonEvent, doubled: bool) -> bool {
        match self {
            OptionWidget::Dropdown(w) => OptionDropdownWidget::mouse_clicked(w, click, doubled),
            OptionWidget::Ranged(w) => RangedValueWidget::mouse_clicked(w, click, doubled),
        }
    }

    /// java: `widget.isMouseOver(x, y)` (neither class overrides it)
    pub fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        match self {
            OptionWidget::Dropdown(w) => compat::read(w, |__t| __t.base.list.is_mouse_over(x, y)),
            OptionWidget::Ranged(w) => compat::read(w, |__t| __t.base.is_mouse_over(x, y)),
        }
    }
}

impl From<compat::Shared<OptionDropdownWidget>> for OptionWidget {
    fn from(w: compat::Shared<OptionDropdownWidget>) -> OptionWidget {
        OptionWidget::Dropdown(w)
    }
}

impl From<compat::Shared<RangedValueWidget>> for OptionWidget {
    fn from(w: compat::Shared<RangedValueWidget>) -> OptionWidget {
        OptionWidget::Ranged(w)
    }
}

impl std::ops::Deref for FinderSettingsContainer {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FinderSettingsContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FinderSettingsContainer {
    /// java: public FinderSettingsContainer(int x, int y, int height)
    pub fn new(mut x: i32, mut y: i32, mut height: i32) -> compat::Shared<Self> {
        // java: super(x, y, 336, height, Component.empty(), AbstractScrollArea.defaultSettings(4));
        let __this: compat::Shared<Self> = compat::shared(Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(x, y, 336, height, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::empty()), mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings::default_settings(4.0)),
            is_initialized: false,
            floor_selector: None,
            dungeon_type_selector: None,
            sort_groups_selector: None,
            class_level_range: None,
            dungeon_level_range: None,
            currently_opened_option: None,
            initialized_widgets: Vec::new(),
        });
        __this
    }

    /// java: @Override public void setRectangle(int width, int height, int x, int y)
    pub fn set_rectangle(__this: &compat::Shared<Self>, mut width: i32, mut height: i32, mut x: i32, mut y: i32) {
        __this.lock().borrow_mut().base.set_rectangle(width, height, x, y);
        if compat::read(__this, |__t| __t.floor_selector.clone()).is_some() {
            let __wa1 = (x.wrapping_add(width.wrapping_div(4))).wrapping_sub(70);
            let __wa2 = y.wrapping_add(20);
            let __o1 = compat::read(__this, |__t| __t.floor_selector.clone()).unwrap();
            compat::write(&__o1, |__t| __t.set_position(__wa1, __wa2));
        }
        if compat::read(__this, |__t| __t.dungeon_type_selector.clone()).is_some() {
            let __wa3 = (x.wrapping_add((3i32.wrapping_mul(width)).wrapping_div(4))).wrapping_sub(70);
            let __wa4 = y.wrapping_add(20);
            let __o2 = compat::read(__this, |__t| __t.dungeon_type_selector.clone()).unwrap();
            compat::write(&__o2, |__t| __t.set_position(__wa3, __wa4));
        }
        if compat::read(__this, |__t| __t.sort_groups_selector.clone()).is_some() {
            let __wa5 = (x.wrapping_add(width.wrapping_div(2))).wrapping_sub(70);
            let __wa6 = y.wrapping_add(120);
            let __o3 = compat::read(__this, |__t| __t.sort_groups_selector.clone()).unwrap();
            compat::write(&__o3, |__t| __t.set_position(__wa5, __wa6));
        }
        if compat::read(__this, |__t| __t.class_level_range.clone()).is_some() {
            let __wa7 = (x.wrapping_add(width.wrapping_div(4))).wrapping_sub(50);
            let __wa8 = y.wrapping_add(70);
            let __o4 = compat::read(__this, |__t| __t.class_level_range.clone()).unwrap();
            // java: classLevelRange.setPosition(..) - `setX(x); setY(y)`, both overridden (the children move along)
            RangedValueWidget::set_x(&__o4, __wa7);
            RangedValueWidget::set_y(&__o4, __wa8);
        }
        if compat::read(__this, |__t| __t.dungeon_level_range.clone()).is_some() {
            let __wa9 = (x.wrapping_add((3i32.wrapping_mul(width)).wrapping_div(4))).wrapping_sub(50);
            let __wa10 = y.wrapping_add(70);
            let __o5 = compat::read(__this, |__t| __t.dungeon_level_range.clone()).unwrap();
            // java: dungeonLevelRange.setPosition(..) - `setX(x); setY(y)`, both overridden (the children move along)
            RangedValueWidget::set_x(&__o5, __wa9);
            RangedValueWidget::set_y(&__o5, __wa10);
        }
    }

    /// java: public boolean handle(PartyFinderScreen screen, String inventoryName)
    pub fn handle(__this: &compat::Shared<Self>, screen: &compat::Shared<PartyFinderScreen>, inventory_name: &str) -> bool {
        let mut name_lower_case: String = inventory_name.to_lowercase();
        let mut handler: compat::LiveChestMenu = PartyFinderScreen::get_handler(screen);
        if !(compat::read(__this, |__t| __t.is_initialized)) {
            if !(name_lower_case.contains("search settings")) {
                return false;
            }
            { let __v = true; compat::write(__this, |__t| __t.is_initialized = __v); }
            // handler.slots and what each slot's getItem() answers: (slot.index, slot.getItem())
            let __slots: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = handler.with_slots(|menu, containers| menu.slots.iter().map(|slot| (slot.index, slot.get_item(containers).duplicate())).collect()).unwrap_or_default();
            let __row_count: i32 = handler.with(|__chest| __chest.get_row_count()).unwrap_or(0);
            for (slot_index, slot_item) in __slots.iter() {
                if *slot_index > (__row_count.wrapping_mul(9)).wrapping_sub(1) {
                    break;
                }
                if slot_item.is_empty() {
                    continue;
                }
                let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = slot_item.clone();
                let mut name: String = compat::component_of(stack.get_hover_name()).get_string().to_lowercase();
                if name.contains("floor") {
                    { let __v = Some(OptionDropdownWidget::new(screen.clone(), compat::component_of(stack.get_hover_name()), ((__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(4))).wrapping_sub(70), (__this.lock().borrow().get_y()).wrapping_add(20), 140, 170, *slot_index)); compat::write(__this, |__t| __t.floor_selector = __v); }
                    if !(FinderSettingsContainer::set_selected_element_from_tooltip(__this, *slot_index, &stack, &compat::read(__this, |__t| __t.floor_selector.clone()).unwrap())) {
                        return false;
                    }
                    let __h0 = compat::read(__this, |__t| __t.floor_selector.clone()).unwrap();
                    __this.lock().borrow_mut().initialized_widgets.push(__h0.into());
                } else if name.contains("dungeon type") {
                    { let __v = Some(OptionDropdownWidget::new(screen.clone(), compat::component_of(stack.get_hover_name()), ((__this.lock().borrow().get_x()).wrapping_add((3i32.wrapping_mul(__this.lock().borrow().get_width())).wrapping_div(4))).wrapping_sub(70), (__this.lock().borrow().get_y()).wrapping_add(20), 140, 100, *slot_index)); compat::write(__this, |__t| __t.dungeon_type_selector = __v); }
                    if !(FinderSettingsContainer::set_selected_element_from_tooltip(__this, *slot_index, &stack, &compat::read(__this, |__t| __t.dungeon_type_selector.clone()).unwrap())) {
                        return false;
                    }
                    let __h0 = compat::read(__this, |__t| __t.dungeon_type_selector.clone()).unwrap();
                    __this.lock().borrow_mut().initialized_widgets.push(__h0.into());
                } else if name.contains("groups") {
                    { let __v = Some(OptionDropdownWidget::new(screen.clone(), compat::component_of(stack.get_hover_name()), ((__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2))).wrapping_sub(70), (__this.lock().borrow().get_y()).wrapping_add(120), 140, 100, *slot_index)); compat::write(__this, |__t| __t.sort_groups_selector = __v); }
                    if !(FinderSettingsContainer::set_selected_element_from_tooltip(__this, *slot_index, &stack, &compat::read(__this, |__t| __t.sort_groups_selector.clone()).unwrap())) {
                        return false;
                    }
                    let __h0 = compat::read(__this, |__t| __t.sort_groups_selector.clone()).unwrap();
                    __this.lock().borrow_mut().initialized_widgets.push(__h0.into());
                } else if name.contains("class level") {
                    { let __v = Some(RangedValueWidget::new(screen.clone(), compat::component_of(stack.get_hover_name()), ((__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(4))).wrapping_sub(50), (__this.lock().borrow().get_y()).wrapping_add(70), 100, *slot_index)); compat::write(__this, |__t| __t.class_level_range = __v); }
                    if !(FinderSettingsContainer::set_range_from_tooltip(__this, &stack, &compat::read(__this, |__t| __t.class_level_range.clone()).unwrap())) {
                        return false;
                    }
                    let __h0 = compat::read(__this, |__t| __t.class_level_range.clone()).unwrap();
                    __this.lock().borrow_mut().initialized_widgets.push(__h0.into());
                } else if name.contains("dungeon level") {
                    { let __v = Some(RangedValueWidget::new(screen.clone(), compat::component_of(stack.get_hover_name()), ((__this.lock().borrow().get_x()).wrapping_add((3i32.wrapping_mul(__this.lock().borrow().get_width())).wrapping_div(4))).wrapping_sub(50), (__this.lock().borrow().get_y()).wrapping_add(70), 100, *slot_index)); compat::write(__this, |__t| __t.dungeon_level_range = __v); }
                    if !(FinderSettingsContainer::set_range_from_tooltip(__this, &stack, &compat::read(__this, |__t| __t.dungeon_level_range.clone()).unwrap())) {
                        return false;
                    }
                    let __h0 = compat::read(__this, |__t| __t.dungeon_level_range.clone()).unwrap();
                    __this.lock().borrow_mut().initialized_widgets.push(__h0.into());
                }
            }
        }
        if name_lower_case.contains("search settings") {
            if compat::read(__this, |__t| __t.floor_selector.clone()).is_some() {
                OptionDropdownWidget::close(&compat::read(__this, |__t| __t.floor_selector.clone()).unwrap());
            }
            if compat::read(__this, |__t| __t.dungeon_type_selector.clone()).is_some() {
                OptionDropdownWidget::close(&compat::read(__this, |__t| __t.dungeon_type_selector.clone()).unwrap());
            }
            if compat::read(__this, |__t| __t.sort_groups_selector.clone()).is_some() {
                OptionDropdownWidget::close(&compat::read(__this, |__t| __t.sort_groups_selector.clone()).unwrap());
            }
            if compat::read(__this, |__t| __t.class_level_range.clone()).is_some() {
                RangedValueWidget::set_state(&compat::read(__this, |__t| __t.class_level_range.clone()).unwrap(), State::Closed);
            }
            if compat::read(__this, |__t| __t.dungeon_level_range.clone()).is_some() {
                RangedValueWidget::set_state(&compat::read(__this, |__t| __t.dungeon_level_range.clone()).unwrap(), State::Closed);
            }
            // java: screen.partyFinderButton.active = true;
            compat::write(screen, |__t| { let __s = &mut *__t; if let Some(__r) = __s.party_finder_button.as_mut() { __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = true) } });
            { let __v = None; compat::write(__this, |__t| __t.currently_opened_option = __v); }
            let __slots: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = handler.with_slots(|menu, containers| menu.slots.iter().map(|slot| (slot.index, slot.get_item(containers).duplicate())).collect()).unwrap_or_default();
            let __row_count: i32 = handler.with(|__chest| __chest.get_row_count()).unwrap_or(0);
            for i in (__row_count.wrapping_sub(1)).wrapping_mul(9)..__row_count.wrapping_mul(9) {
                let (slot_index, slot_item) = &__slots[(i) as usize];
                if !slot_item.is_empty() && slot_item.is_item(&items::items().get("arrow")) {
                    { let __v = *slot_index; compat::write(screen, |__t| __t.party_button_slot_id = __v); }
                }
            }
            return true;
        } else {
            // java: screen.partyFinderButton.active = false;
            compat::write(screen, |__t| { let __s = &mut *__t; if let Some(__r) = __s.party_finder_button.as_mut() { __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = false) } });
            if name_lower_case.contains("floor") {
                FinderSettingsContainer::update_dropdown_option_widget(__this, &handler, compat::read(__this, |__t| __t.floor_selector.clone()));
                return true;
            } else if name_lower_case.contains("select type") {
                FinderSettingsContainer::update_dropdown_option_widget(__this, &handler, compat::read(__this, |__t| __t.dungeon_type_selector.clone()));
                return true;
            } else if name_lower_case.contains("class level range") {
                FinderSettingsContainer::update_ranged_value(__this, &handler, compat::read(__this, |__t| __t.class_level_range.clone()));
                return true;
            } else if name_lower_case.contains("dungeon level range") {
                FinderSettingsContainer::update_ranged_value(__this, &handler, compat::read(__this, |__t| __t.dungeon_level_range.clone()));
                return true;
            } else if name_lower_case.contains("sort") {
                FinderSettingsContainer::update_dropdown_option_widget(__this, &handler, compat::read(__this, |__t| __t.sort_groups_selector.clone()));
                return true;
            }
        }
        return false;
    }

    /// java: private int findBackSlotId(ChestMenu handler)
    /// (`handler` is the menu the mod keeps: `compat::LiveChestMenu`)
    pub fn find_back_slot_id(__this: &compat::Shared<Self>, handler: &compat::LiveChestMenu) -> i32 {
        let mut back_id: i32 = -1;
        // handler.slots and what each slot's getItem() answers: (slot.index, slot.getItem())
        let __slots: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = handler.with_slots(|menu, containers| menu.slots.iter().map(|slot| (slot.index, slot.get_item(containers).duplicate())).collect()).unwrap_or_default();
        let __row_count: i32 = handler.with(|__chest| __chest.get_row_count()).unwrap_or(0);
        for i in (__row_count.wrapping_sub(1)).wrapping_mul(9)..__row_count.wrapping_mul(9) {
            let (slot_index, slot_item) = &__slots[(i) as usize];
            if !slot_item.is_empty() && slot_item.is_item(&items::items().get("arrow")) {
                back_id = *slot_index;
                break;
            }
        }
        return back_id;
    }

    /// java: private boolean setRangeFromTooltip(ItemStack stack, RangedValueWidget widget)
    pub fn set_range_from_tooltip(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, widget: &compat::Shared<RangedValueWidget>) -> bool {
        for text in stack.skyblocker_get_lore_strings() {
            let mut text_lower_case: String = text.to_lowercase();
            if text_lower_case.contains("selected:") {
                let mut split: Vec<String> = compat::regex::split(&text, ":");
                if (split.len() as i32) < 2 {
                    return false;
                }
                let mut min_and_max: Vec<String> = compat::regex::split(&(split[(1) as usize]), "-");
                if (min_and_max.len() as i32) < 2 {
                    return false;
                }
                let mut le_min: i32 = -1;
                let mut le_max: i32 = -1;
                // java: try { leMin = Integer.parseInt(minAndMax[0].trim()); } catch (NumberFormatException _) {}
                if let Ok(v) = compat::trim(&(min_and_max[(0) as usize])).parse::<i32>() {
                    le_min = v;
                }
                // java: try { leMax = Integer.parseInt(minAndMax[1].trim()); } catch (NumberFormatException _) {}
                if let Ok(v) = compat::trim(&(min_and_max[(1) as usize])).parse::<i32>() {
                    le_max = v;
                }
                RangedValueWidget::set_min_and_max(widget, le_min, le_max);
                return true;
            }
        }
        return false;
    }

    /// java: private boolean setSelectedElementFromTooltip(Slot slot, ItemStack stack, OptionDropdownWidget dropdownWidget)
    /// `slot_index` is `slot.index`, all the method reads of the slot.
    pub fn set_selected_element_from_tooltip(__this: &compat::Shared<Self>, slot_index: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, dropdown_widget: &compat::Shared<OptionDropdownWidget>) -> bool {
        for text in stack.skyblocker_get_lore_strings() {
            let mut text_lower_case: String = text.to_lowercase();
            if text_lower_case.contains("selected:") {
                let mut split: Vec<String> = compat::regex::split(&text, ":");
                if (split.len() as i32) < 2 {
                    return false;
                }
                let mut floor_name: String = compat::trim(&(split[(1) as usize]));
                // java: dropdownWidget.setSelectedOption(dropdownWidget.new Option(floorName, stack, slot.index));
                let __option = crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::option_dropdown_widget::Option::new(dropdown_widget, floor_name.clone(), Some(stack.clone()), slot_index);
                OptionDropdownWidget::set_selected_option(dropdown_widget, __option);
                return true;
            }
        }
        return false;
    }

    /// java: public boolean handleSign(SignBlockEntity sign, boolean front)
    pub fn handle_sign(__this: &compat::Shared<Self>, sign: &SignBlockEntity, mut front: bool) -> bool {
        if !(compat::read(__this, |__t| __t.is_initialized)) {
            return false;
        }
        // java: currentlyOpenedOption == classLevelRange (reference identity; two nulls are equal)
        let __same = |opened: &Option<OptionWidget>, range: &Option<compat::Shared<RangedValueWidget>>| match (opened, range) {
            (None, None) => true,
            (Some(o), Some(r)) => o.ptr_eq(&OptionWidget::Ranged(r.clone())),
            _ => false,
        };
        let __opened = compat::read(__this, |__t| __t.currently_opened_option.clone());
        if __same(&__opened, &compat::read(__this, |__t| __t.class_level_range.clone())) {
            return FinderSettingsContainer::update_values(__this, sign, front, &compat::read(__this, |__t| __t.class_level_range.clone()).expect("NullPointerException"));
        } else if __same(&__opened, &compat::read(__this, |__t| __t.dungeon_level_range.clone())) {
            return FinderSettingsContainer::update_values(__this, sign, front, &compat::read(__this, |__t| __t.dungeon_level_range.clone()).expect("NullPointerException"));
        }
        return false;
    }

    /// java: private boolean updateValues(SignBlockEntity sign, boolean front, RangedValueWidget valueWidget)
    pub fn update_values(__this: &compat::Shared<Self>, sign: &SignBlockEntity, mut front: bool, value_widget: &compat::Shared<RangedValueWidget>) -> bool {
        let mut state: State;
        let mut lower_case: String = ((compat::component_of(sign.get_text(compat::sign_slot(front)).get_messages(false)[(3) as usize].clone())).get_string()).to_lowercase();
        if lower_case.contains("max") {
            state = State::ModifyingMax;
        } else if lower_case.contains("min") {
            state = State::ModifyingMin;
        } else {
            return false;
        }
        RangedValueWidget::set_state(value_widget, state);
        // java: this.setFocused(valueWidget); - the child's place in `children()` (`initializedWidgets`)
        let __index: Option<usize> = compat::read(__this, |__t| __t.initialized_widgets.iter().position(|w| w.ptr_eq(&OptionWidget::Ranged(value_widget.clone()))));
        compat::write(__this, |__t| { __t.base.set_focused(__index); });
        return true;
    }

    /// java: private void updateDropdownOptionWidget(ChestMenu handler, @Nullable OptionDropdownWidget dropdownWidget)
    /// (`handler` is the menu the mod keeps: `compat::LiveChestMenu`)
    pub fn update_dropdown_option_widget(__this: &compat::Shared<Self>, handler: &compat::LiveChestMenu, mut dropdown_widget: Option<compat::Shared<OptionDropdownWidget>>) {
        let Some(mut dropdown_widget) = dropdown_widget.clone() else {
            return;
        };
        { let __v = Some(OptionWidget::Dropdown(dropdown_widget.clone())); compat::write(__this, |__t| __t.currently_opened_option = __v); }
        let mut entries: Vec<compat::Shared<crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::option_dropdown_widget::Option>> = Vec::new();
        // handler.slots and what each slot's getItem() answers: (slot.index, slot.getItem())
        let __slots: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = handler.with_slots(|menu, containers| menu.slots.iter().map(|slot| (slot.index, slot.get_item(containers).duplicate())).collect()).unwrap_or_default();
        let __row_count: i32 = handler.with(|__chest| __chest.get_row_count()).unwrap_or(0);
        for (slot_index, slot_item) in __slots.iter() {
            if *slot_index > ((__row_count.wrapping_sub(1)).wrapping_mul(9)).wrapping_sub(1) {
                break;
            }
            if !slot_item.is_empty() && !(slot_item.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black())) {
                // java: entries.add(dropdownWidget.new Option(slot.getItem().getHoverName().getString(), slot.getItem(), slot.index));
                entries.push(crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::option_dropdown_widget::Option::new(&dropdown_widget, compat::component_of(slot_item.get_hover_name()).get_string(), Some(slot_item.clone()), *slot_index));
            }
        }
        let mut back_id: i32 = FinderSettingsContainer::find_back_slot_id(__this, handler);
        OptionDropdownWidget::open(&dropdown_widget, &entries, back_id);
    }

    /// java: private void updateRangedValue(ChestMenu handler, @Nullable RangedValueWidget valueWidget)
    /// (`handler` is the menu the mod keeps: `compat::LiveChestMenu`)
    pub fn update_ranged_value(__this: &compat::Shared<Self>, handler: &compat::LiveChestMenu, mut value_widget: Option<compat::Shared<RangedValueWidget>>) {
        let Some(mut value_widget) = value_widget.clone() else {
            return;
        };
        { let __v = Some(OptionWidget::Ranged(value_widget.clone())); compat::write(__this, |__t| __t.currently_opened_option = __v); }
        let mut min: i32 = -1;
        let mut max: i32 = -1;
        // handler.slots and what each slot's getItem() answers: (slot.index, slot.getItem())
        let __slots: Vec<(i32, mcrust::net::minecraft::world::item::item_stack::ItemStack)> = handler.with_slots(|menu, containers| menu.slots.iter().map(|slot| (slot.index, slot.get_item(containers).duplicate())).collect()).unwrap_or_default();
        let __row_count: i32 = handler.with(|__chest| __chest.get_row_count()).unwrap_or(0);
        for (slot_index, slot_item) in __slots.iter() {
            if *slot_index > ((__row_count.wrapping_sub(1)).wrapping_mul(9)).wrapping_sub(1) {
                break;
            }
            if !slot_item.is_empty() && compat::component_of(slot_item.get_hover_name()).get_string().to_lowercase().contains("min") {
                min = *slot_index;
            } else if !slot_item.is_empty() && compat::component_of(slot_item.get_hover_name()).get_string().to_lowercase().contains("max") {
                max = *slot_index;
            }
        }
        let mut back_id: i32 = FinderSettingsContainer::find_back_slot_id(__this, handler);
        RangedValueWidget::set_state_and_slots(&value_widget, State::Open, min, max, back_id);
    }

    /// java: public void setVisible(boolean visible)
    pub fn set_visible(__this: &compat::Shared<Self>, mut visible: bool) {
        { let __v = visible; compat::write(__this, |__t| __t.visible = __v); }
        if compat::read(__this, |__t| __t.floor_selector.clone()).is_some() {
            // java: this.floor_selector.visible = visible; - the list's own field (`ModSelectionList.visible`)
            { let __v = visible; compat::write(&compat::read(__this, |__t| __t.floor_selector.clone()).unwrap(), |__t| __t.base.visible = __v); }
        }
        if compat::read(__this, |__t| __t.dungeon_type_selector.clone()).is_some() {
            // java: this.dungeon_type_selector.visible = visible; - the list's own field (`ModSelectionList.visible`)
            { let __v = visible; compat::write(&compat::read(__this, |__t| __t.dungeon_type_selector.clone()).unwrap(), |__t| __t.base.visible = __v); }
        }
        if compat::read(__this, |__t| __t.class_level_range.clone()).is_some() {
            { let __v = visible; compat::write(&compat::read(__this, |__t| __t.class_level_range.clone()).unwrap(), |__t| __t.visible = __v); }
        }
        if compat::read(__this, |__t| __t.dungeon_level_range.clone()).is_some() {
            { let __v = visible; compat::write(&compat::read(__this, |__t| __t.dungeon_level_range.clone()).unwrap(), |__t| __t.visible = __v); }
        }
        if compat::read(__this, |__t| __t.sort_groups_selector.clone()).is_some() {
            // java: this.sort_groups_selector.visible = visible; - the list's own field (`ModSelectionList.visible`)
            { let __v = visible; compat::write(&compat::read(__this, |__t| __t.sort_groups_selector.clone()).unwrap(), |__t| __t.base.visible = __v); }
        }
    }

    /// java: public boolean canInteract(AbstractContainerWidget widget)
    /// (`widget` is Java's nullable reference: `None` is `null`)
    pub fn can_interact(__this: &compat::Shared<Self>, widget: Option<&OptionWidget>) -> bool {
        // java: return currentlyOpenedOption == null || currentlyOpenedOption == widget;
        let __opened = compat::read(__this, |__t| __t.currently_opened_option.clone());
        return (__opened.is_none() || widget.is_some_and(|w| __opened.as_ref().unwrap().ptr_eq(w)));
    }

    /// java: public boolean hasOpenOption()
    pub fn has_open_option(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.currently_opened_option.clone()).is_some();
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if FinderSettingsContainer::has_open_option(__this) {
            let __opened = compat::read(__this, |__t| __t.currently_opened_option.clone()).unwrap();
            return __opened.mouse_clicked(click, doubled);
        }
        // java: return super.mouseClicked(click, doubled); - AbstractContainerWidget.mouseClicked:
        // `boolean scrolling = this.updateScrolling(event); return ContainerEventHandler.super.mouseClicked(event, doubleClick) || scrolling;`
        // updateScrolling answers false: the scrollbar is never visible (contentHeight() is 0)
        let scrolling: bool = false;
        // ContainerEventHandler.mouseClicked: the first child under the mouse (`getChildAt`) takes the click
        let __children = FinderSettingsContainer::children(__this);
        let __child = __children.iter().enumerate().find(|(_, c)| c.is_mouse_over(click.x(), click.y())).map(|(i, c)| (i, c.clone()));
        let consumed: bool = match __child {
            None => false,
            Some((__index, widget)) => {
                // `&& widget.shouldTakeFocusAfterInteraction()` - the default `true`
                if widget.mouse_clicked(click, doubled) {
                    compat::write(__this, |__t| { __t.base.set_focused(Some(__index)); });
                    if click.button() == 1 {
                        compat::write(__this, |__t| __t.base.set_dragging(true));
                    }
                }
                true
            }
        };
        return consumed || scrolling;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    /// (`context` is the frame's render context, which the children's draws take)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if !(compat::read(__this, |__t| __t.visible)) || !(compat::read(__this, |__t| __t.is_initialized)) {
            return;
        }
        // java: this.classLevelRange.extractRenderState(graphics, mouseX, mouseY, a); (and dungeonLevelRange) -
        // `AbstractWidget.extractRenderState`: drawn only while visible
        for __range in [compat::read(__this, |__t| __t.class_level_range.clone()), compat::read(__this, |__t| __t.dungeon_level_range.clone())].into_iter().flatten() {
            if compat::read(&__range, |__t| __t.visible) {
                RangedValueWidget::extract_widget_render_state(&__range, context, mouse_x, mouse_y, a);
            }
        }
        // Render the dropdowns last to fix overlap issue.
        // java: this.sortGroupsSelector / floorSelector / dungeonTypeSelector .extractRenderState(graphics, mouseX, mouseY, a)
        for __dropdown in [compat::read(__this, |__t| __t.sort_groups_selector.clone()), compat::read(__this, |__t| __t.floor_selector.clone()), compat::read(__this, |__t| __t.dungeon_type_selector.clone())].into_iter().flatten() {
            if compat::read(&__dropdown, |__t| __t.base.visible) {
                OptionDropdownWidget::extract_widget_render_state(&__dropdown, context, mouse_x, mouse_y, a);
            }
        }
    }

    /// java: `ContainerEventHandler.keyPressed(event)` - `getFocused() != null && getFocused().keyPressed(event)`
    /// (a dropdown's is the list's own, which moves a selection its `entriesCanBeSelected() == false` never shows)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let __focused: Option<OptionWidget> = compat::read(__this, |__t| __t.base.get_focused().and_then(|i| __t.initialized_widgets.get(i).cloned()));
        match __focused {
            Some(OptionWidget::Ranged(w)) => RangedValueWidget::key_pressed(&w, event),
            _ => false,
        }
    }

    /// java: `ContainerEventHandler.charTyped(event)` - the focused child's
    pub fn char_typed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        let __focused: Option<OptionWidget> = compat::read(__this, |__t| __t.base.get_focused().and_then(|i| __t.initialized_widgets.get(i).cloned()));
        match __focused {
            Some(OptionWidget::Ranged(w)) => RangedValueWidget::char_typed(&w, event),
            _ => false,
        }
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<OptionWidget> {
        return compat::read(__this, |__t| __t.initialized_widgets.clone());
    }


    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(__this: &compat::Shared<Self>) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(__this: &compat::Shared<Self>) -> f64 {
        return ((0) as f64) as f64;
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, mouseClicked, scrollRate, setRectangle, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// FinderSettingsContainer's container half (`super`'s fields)
impl ContainerWidgetHalf for FinderSettingsContainer {
    fn container_half(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }
}

/// FinderSettingsContainer as the game's widget: `ContainerWidgetAdapter::widget(handle)` where Java hands it on
impl mcrust::mcrust::modding::widgets::ModWidget for ContainerWidgetAdapter<FinderSettingsContainer> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend(|__h| FinderSettingsContainer::extract_widget_render_state(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, _widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend(|__h| FinderSettingsContainer::update_widget_narration(__h, &*output))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(|__h| FinderSettingsContainer::mouse_clicked(__h, event, double_click))
    }

    /// java: `ContainerEventHandler.keyPressed(event)` - the focused child's
    fn key_pressed(&mut self, _widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(|__h| FinderSettingsContainer::key_pressed(__h, event))
    }

    /// java: `ContainerEventHandler.charTyped(event)` - the focused child's
    fn char_typed(&mut self, _widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        self.lend(|__h| FinderSettingsContainer::char_typed(__h, event))
    }

    fn container_widget_mut(&mut self) -> Option<&mut AbstractContainerWidget> {
        Some(self.part_mut())
    }
}

/// A mod class that `extends AbstractContainerWidget` (this container, `RangedValueWidget`, `ChatRuleConfigScreen
/// .ContentContainer`, ..): its container half (`base`), which holds `super`'s fields.
///
/// (A helper for the batch's container widgets; it belongs beside `compat::WidgetHalf` - see PATTERNS_batchF.md.)
pub(crate) trait ContainerWidgetHalf {
    fn container_half(&mut self) -> &mut AbstractContainerWidget;
}

/// The fields of a container widget's box the game keeps on its own `AbstractWidget` and syncs into the part
/// (`AbstractContainerWidget::sync_from_widget` / `sync_to_widget`).
#[derive(Clone, Copy, PartialEq)]
struct ContainerBox {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    active: bool,
    visible: bool,
}

impl ContainerBox {
    fn of(w: &AbstractContainerWidget) -> ContainerBox {
        ContainerBox { x: w.get_x(), y: w.get_y(), width: w.width, height: w.height, active: w.active, visible: w.visible }
    }

    fn write(&self, w: &mut AbstractContainerWidget) {
        w.set_x(self.x);
        w.set_y(self.y);
        w.width = self.width;
        w.height = self.height;
        w.active = self.active;
        w.visible = self.visible;
    }
}

/// A mod object that `extends AbstractContainerWidget`, as the game's `ModWidget`: Java's ONE object is the mod's
/// object (its fields, its `base`) plus the game's `AbstractWidget` in the screen's list. The game syncs that
/// widget into `part` before each call (`ModWidget::container_widget_mut`); `lend` brings it into the object's
/// `base` - keeping a box field the mod's own code changed since the last call (`setRectangle` from the screen),
/// which then reaches the game's widget through `part` - and copies the object's box back into `part` after the
/// call, which the game writes back onto its widget for a call that holds it (`sync_to_widget`).
pub(crate) struct ContainerWidgetAdapter<C: 'static> {
    object: compat::RenderThread<compat::Shared<C>>,
    part: AbstractContainerWidget,
    /// the game's box as of the last call: a field of the object's `base` that differs from it was set by the mod
    synced: ContainerBox,
}

impl<C: ContainerWidgetHalf + 'static> ContainerWidgetAdapter<C> {
    /// `new MyContainer(..)` handed to the game (`addRenderableWidget`, a layout): the game's widget, built from
    /// the object's own box, with the object attached.
    pub(crate) fn widget(object: compat::Shared<C>) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget
    where
        ContainerWidgetAdapter<C>: mcrust::mcrust::modding::widgets::ModWidget,
    {
        let part: AbstractContainerWidget = compat::write(&object, |__t| __t.container_half().clone());
        let synced = ContainerBox::of(&part);
        let message: String = part.get_message().get_string();
        let mut widget = mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(synced.x, synced.y, synced.width, synced.height, message, Box::new(ContainerWidgetAdapter { object: compat::RenderThread::new(object), part, synced }));
        widget.active = synced.active;
        widget.visible = synced.visible;
        widget
    }

    /// The mod object.
    pub(crate) fn object(&self) -> &compat::Shared<C> {
        self.object.get()
    }

    /// The part the game syncs (`container_widget_mut`).
    pub(crate) fn part_mut(&mut self) -> &mut AbstractContainerWidget {
        &mut self.part
    }

    /// One of the object's methods, with its `base` brought up to the live widget first and its box handed back
    /// after.
    pub(crate) fn lend<R>(&mut self, f: impl FnOnce(&compat::Shared<C>) -> R) -> R {
        let object: compat::Shared<C> = self.object.get().clone();
        let live = ContainerBox::of(&self.part);
        let synced = self.synced;
        let hovered = self.part.is_hovered;
        compat::write(&object, |__t| {
            let half = __t.container_half();
            let mine = ContainerBox::of(half);
            let merged = ContainerBox {
                x: if mine.x != synced.x { mine.x } else { live.x },
                y: if mine.y != synced.y { mine.y } else { live.y },
                width: if mine.width != synced.width { mine.width } else { live.width },
                height: if mine.height != synced.height { mine.height } else { live.height },
                active: if mine.active != synced.active { mine.active } else { live.active },
                visible: if mine.visible != synced.visible { mine.visible } else { live.visible },
            };
            merged.write(half);
            half.is_hovered = hovered;
        });
        self.synced = live;
        let out = f(&object);
        let after = compat::write(&object, |__t| ContainerBox::of(__t.container_half()));
        after.write(&mut self.part);
        out
    }
}
