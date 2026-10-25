//! Ported from de.hysky.skyblocker.skyblock.shortcut.ShortcutsConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts::Shortcuts;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_list_widget;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_list_widget::KeybindShortcutEntry;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_list_widget::ShortcutsConfigListWidget;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::ModWidgetSlot;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::common_components;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;

/// java: public class ShortcutsConfigScreen extends Screen
pub struct ShortcutsConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final @Nullable Screen parent; (the game keeps the parent: `set_screen_keeping_parent`)
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private ShortcutsConfigListWidget shortcutsConfigListWidget;
    pub shortcuts_config_list_widget: Option<compat::WidgetRef>,
    /// java: private Button buttonDelete;
    pub button_delete: Option<compat::WidgetRef>,
    /// java: private Button buttonNew;
    pub button_new: Option<compat::WidgetRef>,
    /// java: private Button buttonDone;
    pub button_done: Option<compat::WidgetRef>,
    /// java: private boolean initialized;
    pub initialized: bool,
    /// java: private double scrollAmount;
    pub scroll_amount: f64,
    /// the list OBJECT (Java's `shortcutsConfigListWidget` field holds the one list across re-inits): the game's
    /// list widget's own slot, reached without the screen's widget tree
    pub list_slot: Option<ModWidgetSlot>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for ShortcutsConfigScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ShortcutsConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ShortcutsConfigScreen {
    /// java: public ShortcutsConfigScreen()
    pub fn new() -> compat::Shared<Self> {
        Self::new_1(None)
    }

    /// java: public ShortcutsConfigScreen(@Nullable Screen parent)
    pub fn new_1(parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.shortcuts.config").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent,
            shortcuts_config_list_widget: None,
            button_delete: None,
            button_new: None,
            button_done: None,
            initialized: false,
            scroll_amount: 0.0,
            list_slot: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// `shortcutsConfigListWidget.x(..)`: the list object, lent for `f` (no borrow of this screen is held, so the
    /// list's own overrides can reach back to it). Not from inside the list's own methods - those hand the list in.
    pub fn with_list<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut AbstractSelectionList) -> R) -> R {
        let slot = compat::read(__this, |__t| __t.list_slot.clone()).expect("ShortcutsConfigScreen.shortcutsConfigListWidget before init()");
        slot.with::<ModSelectionList, _>(|l| f(&mut l.list)).expect("the shortcuts list reached from inside its own method")
    }

    /// One access to a button through its reference.
    fn with_button<R>(__this: &compat::Shared<Self>, button: fn(&mut Self) -> &mut Option<compat::WidgetRef>, f: impl FnOnce(&mut AbstractWidget) -> R) -> R {
        compat::write(__this, |__t| {
            let __s = &mut *__t;
            // (the reference leaves its field for the call: `button(__s)` borrows all of `__s`)
            let mut __r = button(__s).take().expect("a button before init()");
            let __out = __s.__widgets.with(&mut __s.base, &mut __r, f);
            *button(__s) = Some(__r);
            __out
        })
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let (width, height) = compat::read(__this, |__t| (__t.width, __t.height));
        if compat::read(__this, |__t| __t.initialized) {
            // java: shortcutsConfigListWidget.setSize(width, height - 96); updatePositions(); refreshScrollAmount();
            ShortcutsConfigScreen::with_list(__this, |list| {
                list.set_size(width, height - 96);
                ShortcutsConfigListWidget::update_positions(list);
                list.refresh_scroll_amount();
            });
            // the same list object joins the new widget tree
            let slot = compat::read(__this, |__t| __t.list_slot.clone()).unwrap();
            let (x, y) = ShortcutsConfigScreen::with_list(__this, |list| (list.get_x(), list.get_y()));
            let widget = AbstractWidget::new(x, y, width, height - 96, String::new(), WidgetKind::Mod(slot), OnPress::Inert);
            compat::write(__this, |__t| __t.shortcuts_config_list_widget = Some(compat::WidgetRef::value(widget)));
        } else {
            // java: shortcutsConfigListWidget = new ShortcutsConfigListWidget(minecraft, this, width, height - 96, 32);
            let widget = ShortcutsConfigListWidget::new(__this, width, height - 96, 32);
            let slot = widget.mod_widget();
            compat::write(__this, |__t| {
                __t.list_slot = slot;
                __t.shortcuts_config_list_widget = Some(compat::WidgetRef::value(widget));
                __t.initialized = true;
            });
        }
        // java: addRenderableWidget(shortcutsConfigListWidget);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        let mut grid_widget: GridLayout = GridLayout::new_default();
        // java: gridWidget.defaultCellSetting().paddingHorizontal(5).paddingVertical(2); - on the layout's own settings
        let __settings = grid_widget.default_cell_setting();
        *__settings = __settings.clone().padding_horizontal(5).padding_vertical(2);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);

        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let delete = button::builder(component::translatable("selectServer.deleteButton").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            // java: if (shortcutsConfigListWidget.getSelected() instanceof ShortcutsConfigListWidget.ShortcutEntry<?> shortcutEntry)
            let selected = ShortcutsConfigScreen::with_list(&__this, |list| {
                let index = list.get_selected()?;
                let slot = list.entry_object(index)?;
                let text = shortcuts_config_list_widget::with_shortcut_entry(slot, |e| e.to_display_string())?;
                Some((index, text, slot.is::<KeybindShortcutEntry>(), list.scroll_amount()))
            });
            if let Some((index, shortcut_entry, is_keybind, scroll_amount)) = selected {
                compat::write(&__this, |__t| __t.scroll_amount = scroll_amount);
                let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ConfirmScreen::new(
                    move |confirmed_action| {
                        let Some(__this) = __this_w.get().upgrade() else { return; };
                        ShortcutsConfigScreen::delete_entry(&__this, confirmed_action, index, is_keybind);
                    },
                    component::translatable("skyblocker.shortcuts.deleteQuestion"),
                    component::translatable_escape("skyblocker.shortcuts.deleteWarning", vec![FormattedText::of(shortcut_entry)]),
                    component::translatable("selectServer.deleteButton"),
                    common_components::gui_cancel(),
                )));
            }
        })));
        compat::write(__this, |__t| __t.button_delete = Some(compat::WidgetRef::value(delete.build())));
        ShortcutsConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, |s| &mut s.button_delete);

        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let new = button::builder(component::translatable("skyblocker.shortcuts.new").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            ShortcutsConfigScreen::with_list(&__this, |list| ShortcutsConfigListWidget::add_shortcut_after_selected(list));
        })));
        compat::write(__this, |__t| __t.button_new = Some(compat::WidgetRef::value(new.build())));
        ShortcutsConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, |s| &mut s.button_new);

        // java: adder.addChild(Button.builder(CommonComponents.GUI_CANCEL, _ -> onClose()).build());
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let cancel = button::builder(common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            ShortcutsConfigScreen::on_close(&__this);
        })))
        .build();
        let mut cancel_ref = compat::WidgetRef::value(cancel);
        let (__i, __w, __h) = compat::write(__this, |__t| { let __s = &mut *__t; __s.__widgets.add_to_layout(&mut __s.base, &mut cancel_ref) });
        adder.add_child(&mut grid_widget, LayoutElement::widget(__i, __w, __h)).expect("GridLayout.addChild");

        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let done = button::builder(common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            ShortcutsConfigScreen::with_list(&__this, |list| ShortcutsConfigListWidget::save_shortcuts(list));
            ShortcutsConfigScreen::on_close(&__this);
        })))
        .tooltip_text(Some(FormattedText::from(component::translatable("skyblocker.shortcuts.commandSuggestionTooltip"))));
        compat::write(__this, |__t| __t.button_done = Some(compat::WidgetRef::value(done.build())));
        ShortcutsConfigScreen::add_to_grid(__this, &mut grid_widget, &mut adder, |s| &mut s.button_done);

        // java: gridWidget.arrangeElements(); FrameLayout.centerInRectangle(gridWidget, 0, this.height - 64, this.width, 64);
        grid_widget.arrange_elements();
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut grid_widget), 0, height - 64, width, 64);
        // java: gridWidget.visitWidgets(this::addRenderableWidget);
        let mut __vw = Vec::new();
        LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw);
        compat::write(__this, |__t| {
            for (__i, __x, __y) in __vw {
                __t.widget_mut(__i).set_position(__x, __y);
                __t.mark_renderable(__i);
            }
        });
        ShortcutsConfigScreen::update_buttons(__this);
    }

    /// `adder.addChild(button)`: the button joins the screen's widgets (by index) and the grid.
    fn add_to_grid(__this: &compat::Shared<Self>, grid: &mut GridLayout, adder: &mut RowHelper, button: fn(&mut Self) -> &mut Option<compat::WidgetRef>) {
        let (__i, __w, __h) = compat::write(__this, |__t| {
            let __s = &mut *__t;
            let mut __r = button(__s).take().unwrap();
            let __out = __s.__widgets.add_to_layout(&mut __s.base, &mut __r);
            *button(__s) = Some(__r);
            __out
        });
        adder.add_child(grid, LayoutElement::widget(__i, __w, __h)).expect("GridLayout.addChild");
    }

    /// java: private void deleteEntry(boolean confirmedAction, ShortcutsConfigListWidget.AbstractShortcutEntry entry)
    /// (the entry is the list row at `index`, which the modal confirm screen kept in place; `is_keybind` is its
    /// `instanceof KeybindShortcutEntry`)
    pub fn delete_entry(__this: &compat::Shared<Self>, confirmed_action: bool, index: usize, is_keybind: bool) {
        if confirmed_action {
            // java: if (entry instanceof ShortcutEntry<?> shortcutEntry) shortcutsConfigListWidget.removeEntry(shortcutEntry);
            ShortcutsConfigScreen::with_list(__this, |list| ShortcutsConfigListWidget::remove_entry(list, index));
            if is_keybind {
                ShortcutsConfigScreen::with_list(__this, |list| ShortcutsConfigListWidget::update_keybinds(list, None));
            }
        }
        // Re-inits the screen and keeps the old instance of ShortcutsConfigListWidget
        // (java: minecraft.gui.setScreen(this) - the confirm screen holds this one as its parent: it comes back)
        mcrust::mcrust::modding::game::set_screen_to_parent();
        let scroll_amount = compat::read(__this, |__t| __t.scroll_amount);
        ShortcutsConfigScreen::with_list(__this, |list| list.set_scroll_amount(scroll_amount));
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &ScreenRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        // java: super.extractRenderState(..) - the game draws the widgets first
        let (title, width) = compat::read(__this, |__t| (__t.title.clone(), __t.width));
        gui_graphics_extractor::centered_text(context.gl, context.minecraft, &FormattedText::from(component::translatable("skyblocker.shortcuts.config")), width / 2, 16, mcrust::net::minecraft::util::common_colors::WHITE);
        let _ = title;
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    /// - reached when no child is under the cursor (`super.mouseClicked` - `ContainerEventHandler.mouseClicked`, whose
    /// `getChildAt` finds none - answered `false`); a click on a child is the game's sweep, and the list's own
    /// `mouseClicked` override runs these same lines when the click falls through it
    /// (`ShortcutsConfigListWidget::mouse_clicked_body`)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) -> bool {
        // java: if (super.mouseClicked(click, doubled)) return true; - false here (see above)
        // java: if (shortcutsConfigListWidget.stopEditing()) shortcutsConfigListWidget.updateKeybinds();
        if ShortcutsConfigScreen::with_list(__this, |list| ShortcutsConfigListWidget::stop_editing(list)) {
            ShortcutsConfigScreen::with_list(__this, |list| ShortcutsConfigListWidget::update_keybinds(list, None));
        }
        // java: checkForDuplicates();
        ShortcutsConfigScreen::check_for_duplicates(__this);
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // Process ESC before super to prevent closing the screen if we were editing a keybind
        if input.is_escape() && ShortcutsConfigScreen::with_list(__this, |list| ShortcutsConfigListWidget::stop_editing(list)) {
            ShortcutsConfigScreen::with_list(__this, |list| ShortcutsConfigListWidget::update_keybinds(list, None));
            return true;
        }
        // java: return super.keyPressed(input); - Escape closes through `onClose`
        if input.is_escape() {
            ShortcutsConfigScreen::on_close(__this);
            return true;
        }
        return false;
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if ShortcutsConfigScreen::with_list(__this, |list| ShortcutsConfigListWidget::has_changes(list)) {
            mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ConfirmScreen::new(
                move |confirmed_action| {
                    if confirmed_action {
                        // java: this.minecraft.gui.setScreen(parent); - the confirm screen holds this screen, which
                        // holds `parent`: back past both
                        mcrust::mcrust::modding::game::set_screen_to_parent();
                        mcrust::mcrust::modding::game::set_screen_to_parent();
                    } else {
                        // java: minecraft.gui.setScreen(this);
                        mcrust::mcrust::modding::game::set_screen_to_parent();
                    }
                },
                component::translatable("text.skyblocker.quit_config"),
                component::translatable("text.skyblocker.quit_config_sure"),
                component::translatable("text.skyblocker.quit_discard"),
                common_components::gui_cancel(),
            )));
        } else {
            mcrust::mcrust::modding::game::set_screen_to_parent();
        }
    }

    /// java: protected void updateButtons()
    pub fn update_buttons(__this: &compat::Shared<Self>) {
        let slot = compat::read(__this, |__t| __t.list_slot.clone());
        if let Some(slot) = slot {
            slot.with::<ModSelectionList, _>(|l| ShortcutsConfigScreen::update_buttons_with(__this, &l.list));
        }
    }

    /// java: protected void updateButtons() - over the list handed in (the list's `setSelected` override calls it
    /// from inside the list)
    pub fn update_buttons_with(__this: &compat::Shared<Self>, list: &AbstractSelectionList) {
        if compat::read(__this, |__t| __t.button_delete.is_none()) {
            // `setSelected` before `init()` built the buttons (Java's fields are null there too, and Java's list is
            // not selected before init)
            return;
        }
        let loaded = Shortcuts::is_shortcuts_loaded();
        // java: buttonDelete.active = isShortcutsLoaded() && getSelected() instanceof ShortcutEntry;
        let delete_active = loaded && list.get_selected().and_then(|i| list.entry_object(i)).and_then(|slot| shortcuts_config_list_widget::with_shortcut_entry(slot, |_| ())).is_some();
        ShortcutsConfigScreen::with_button(__this, |s| &mut s.button_delete, |w| w.active = delete_active);
        // java: buttonNew.active = isShortcutsLoaded() && getCategory().isPresent();
        let new_active = loaded && ShortcutsConfigListWidget::get_category(list).is_some();
        ShortcutsConfigScreen::with_button(__this, |s| &mut s.button_new, |w| w.active = new_active);
        ShortcutsConfigScreen::with_button(__this, |s| &mut s.button_done, |w| w.active = loaded);
    }

    /// java: protected void checkForDuplicates()
    pub fn check_for_duplicates(__this: &compat::Shared<Self>) {
        let slot = compat::read(__this, |__t| __t.list_slot.clone());
        if let Some(slot) = slot {
            slot.with::<ModSelectionList, _>(|l| ShortcutsConfigScreen::check_for_duplicates_with(__this, &l.list));
        }
    }

    /// java: protected void checkForDuplicates() - over the list handed in
    pub fn check_for_duplicates_with(__this: &compat::Shared<Self>, list: &AbstractSelectionList) {
        let has_duplicates: bool = ShortcutsConfigListWidget::has_duplicates(list);
        if has_duplicates {
            ShortcutsConfigScreen::with_button(__this, |s| &mut s.button_done, |w| {
                w.active = false;
                w.set_tooltip(Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.shortcuts.hasDuplicates"))));
            });
        } else {
            ShortcutsConfigScreen::with_button(__this, |s| &mut s.button_done, |w| {
                w.active = true;
                w.set_tooltip(Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.shortcuts.commandSuggestionTooltip"))));
            });
        }
    }
}

/// ShortcutsConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ShortcutsConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ShortcutsConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ShortcutsConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ShortcutsConfigScreen::init(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| ShortcutsConfigScreen::extract_render_state(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| ShortcutsConfigScreen::key_pressed(__h, event))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        // `super.mouseClicked` first: a child under the cursor takes the click through the game's sweep (this answers
        // `false` to let it run); with none, Java's own lines run here
        if screen.get_child_at(x, y).is_some() {
            return false;
        }
        self.lend(screen, |__h| ShortcutsConfigScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ShortcutsConfigScreen::on_close(__h))
    }

    /// `Screen.shouldCloseOnEsc` - Escape reaches `keyPressed` above, which closes through `onClose`
    fn should_close_on_esc(&self) -> bool {
        false
    }
}
