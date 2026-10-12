//! Ported from de.hysky.skyblocker.skyblock.shortcut.ShortcutsConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts::Shortcuts;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_list_widget::AbstractShortcutEntry;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_list_widget::KeybindShortcutEntry;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_list_widget::ShortcutEntry;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_list_widget::ShortcutsConfigListWidget;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::screens::confirm_screen::ConfirmScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ShortcutsConfigScreen extends Screen
pub struct ShortcutsConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final @Nullable Screen parent;
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
        let __this = Self::new_1(None);
        __this
    }

    /// java: public ShortcutsConfigScreen(@Nullable Screen parent)
    pub fn new_1(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __init_parent = parent.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.shortcuts.config").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent: __init_parent,
            shortcuts_config_list_widget: None,
            button_delete: None,
            button_new: None,
            button_done: None,
            initialized: false,
            scroll_amount: 0.0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        if compat::read(__this, |__t| __t.initialized) {
            let __wa1 = compat::read(__this, |__t| __t.width);
            let __wa2 = (compat::read(__this, |__t| __t.height)).wrapping_sub(96);
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_size(__wa1, __wa2)) });
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::update_positions(__h)) })).expect("a mod list reached from inside its own method")) });
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.refresh_scroll_amount())).expect("a mod list reached from inside its own method")) });
        } else {
            { let __v = (Some(ShortcutsConfigListWidget::new(compat::minecraft(), __this.clone(), compat::read(__this, |__t| __t.width), (compat::read(__this, |__t| __t.height)).wrapping_sub(96), 32))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.shortcuts_config_list_widget = __v; }); }
            { let __v = true; compat::write(__this, |__t| __t.initialized = __v); }
        }
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        let mut grid_widget: GridLayout = GridLayout::new_default();
        grid_widget.default_cell_setting().padding_horizontal(5).padding_vertical(2);
        let mut adder: RowHelper = grid_widget.create_row_helper(2);
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("selectServer.deleteButton").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if let Some(shortcut_entry) = (compat::write((&__this_2), |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.get_selected().and_then(|__i| compat::entry_at::<dyn AbstractShortcutEntry>(__l, __i)))).expect("a mod list reached from inside its own method")) }).unwrap().lock().borrow()).as_any().downcast_ref::<compat::Shared<dyn ShortcutEntry<compat::Object>>>() {
                { let __v = compat::write((&__this_2), |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.scroll_amount())).expect("a mod list reached from inside its own method")) }); compat::write((&__this_2), |__t| __t.scroll_amount = __v); }
                // TODO(fabric2seer): static `Component.translatableEscape` has no Rust counterpart in the port
                // TODO(fabric2seer): `new ConfirmScreen(..)`: the port has no `ConfirmScreen::new`
                // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
                // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade((&__this_2)));
                // rust: gui::set_screen(&mut ConfirmScreen::new(Arc::new(move |confirmed_action| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ShortcutsConfigScreen::delete_entry((&__this_1), confirmed_action.clone(), &(shortcut_entry.clone() as compat::Shared<dyn AbstractShortcutEntry>)) }), component::translatable("skyblocker.shortcuts.deleteQuestion"), Component::translatable_escape("skyblocker.shortcuts.deleteWarning".to_string(), shortcut_entry.clone()), component::translatable("selectServer.deleteButton"), mcrust::net::minecraft::network::chat::common_components::gui_cancel()));
            }
        } }) })))).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.button_delete = __v; }); }
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(compat::write(__this, |__t| { let __s = &mut *__t; match __s.button_delete.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| __w.clone())), None => None } }).as_mut().unwrap()).unwrap();
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("skyblocker.shortcuts.new").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { compat::write((&__this_3), |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::add_shortcut_after_selected(__h)) })).expect("a mod list reached from inside its own method")) }); } }) })))).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.button_new = __v; }); }
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(compat::write(__this, |__t| { let __s = &mut *__t; match __s.button_new.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| __w.clone())), None => None } }).as_mut().unwrap()).unwrap();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: let __this_4 = compat::RenderThread::new(__this.clone());
        // rust: adder.add_child(&mut (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_4.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { ShortcutsConfigScreen::on_close(__this_4.get()); } }) })))).build()).unwrap();
        let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_5) = __this_w5.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_5, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            compat::write((&__this_5), |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::save_shortcuts(__h)) })).expect("a mod list reached from inside its own method")) });
            ShortcutsConfigScreen::on_close((&__this_5));
        } }) })))).tooltip_text(Some(Tooltip::create(component::translatable("skyblocker.shortcuts.commandSuggestionTooltip").get_string()).message.clone())).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.button_done = __v; }); }
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: adder.add_child(compat::write(__this, |__t| { let __s = &mut *__t; match __s.button_done.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| __w.clone())), None => None } }).as_mut().unwrap()).unwrap();
        { grid_widget.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        frame_layout::center_in_rectangle(&mut *compat::LayoutBack::grid(&mut grid_widget), 0, (compat::read(__this, |__t| __t.height)).wrapping_sub(64), compat::read(__this, |__t| __t.width), 64);
        { let mut __vw = Vec::new(); LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        ShortcutsConfigScreen::update_buttons(__this);
    }

    /// java: private void deleteEntry(boolean confirmedAction, ShortcutsConfigListWidget.AbstractShortcutEntry entry)
    pub fn delete_entry(__this: &compat::Shared<Self>, mut confirmed_action: bool, entry: &compat::Shared<dyn AbstractShortcutEntry>) {
        if confirmed_action {
            if let Some(shortcut_entry) = entry.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ShortcutEntry<compat::Object>>>() {
                let __wa1 = shortcut_entry.clone();
                compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::remove_entry(__h, &(__wa1.clone() as compat::Shared<dyn AbstractShortcutEntry>))) })).expect("a mod list reached from inside its own method")) });
            }
            if entry.lock().borrow().as_any().is::<compat::Shared<KeybindShortcutEntry>>() {
                compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::update_keybinds(__h)) })).expect("a mod list reached from inside its own method")) });
            }
        }
        mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new((__this).clone())));
        let __wa2 = compat::read(__this, |__t| __t.scroll_amount);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.set_scroll_amount(__wa2))).expect("a mod list reached from inside its own method")) });
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.title.clone())), (compat::read(__this, |__t| __t.width)).wrapping_div(2), 16, mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if false {
            return true;
        }
        if compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::stop_editing(__h)) })).expect("a mod list reached from inside its own method")) }) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::update_keybinds(__h)) })).expect("a mod list reached from inside its own method")) });
        }
        ShortcutsConfigScreen::check_for_duplicates(__this);
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if input.is_escape() && compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::stop_editing(__h)) })).expect("a mod list reached from inside its own method")) }) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::update_keybinds(__h)) })).expect("a mod list reached from inside its own method")) });
            return true;
        }
        return (if input.is_escape() && true { ShortcutsConfigScreen::on_close(__this); true } else { false });
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::has_changes(__h)) })).expect("a mod list reached from inside its own method")) }) {
            // TODO(fabric2seer): `new ConfirmScreen(..)`: the port has no `ConfirmScreen::new`
            // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
            // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            // rust: gui::set_screen(&mut ConfirmScreen::new(Arc::new(move |confirmed_action| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            // rust: if confirmed_action {
            // rust: mcrust::mcrust::modding::game::set_screen_to_parent();
            // rust: } else {
            // rust: mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(((&__this_1)).clone())));
            // rust: }
            // rust: } }), component::translatable("text.skyblocker.quit_config"), component::translatable("text.skyblocker.quit_config_sure"), component::translatable("text.skyblocker.quit_discard"), mcrust::net::minecraft::network::chat::common_components::gui_cancel()));
        } else {
            mcrust::mcrust::modding::game::set_screen_to_parent();
        }
    }

    /// java: protected void updateButtons()
    pub fn update_buttons(__this: &compat::Shared<Self>) {
        { let __v = (Shortcuts::is_shortcuts_loaded() && (compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.get_selected().and_then(|__i| compat::entry_at::<dyn AbstractShortcutEntry>(__l, __i)))).expect("a mod list reached from inside its own method")) }).unwrap().lock().borrow()).as_any().is::<compat::Shared<dyn ShortcutEntry<compat::Object>>>()); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_delete.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        { let __v = (Shortcuts::is_shortcuts_loaded() && compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::get_category(__h)) })).expect("a mod list reached from inside its own method")) }).is_some()); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_new.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
        { let __v = Shortcuts::is_shortcuts_loaded(); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_done.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
    }

    /// java: protected void checkForDuplicates()
    pub fn check_for_duplicates(__this: &compat::Shared<Self>) {
        let mut has_duplicates: bool = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.shortcuts_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ShortcutsConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ShortcutsConfigListWidget::has_duplicates(__h)) })).expect("a mod list reached from inside its own method")) });
        if has_duplicates {
            { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_done.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            let __wa1 = Tooltip::create(component::translatable("skyblocker.shortcuts.hasDuplicates").get_string());
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_done.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_tooltip(__wa1.clone())) });
        } else {
            { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_done.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.active = __v) }); }
            let __wa2 = Tooltip::create(component::translatable("skyblocker.shortcuts.commandSuggestionTooltip").get_string());
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.button_done.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_tooltip(__wa2.clone())) });
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
        self.lend_ref(screen, |__h| ShortcutsConfigScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| ShortcutsConfigScreen::key_pressed(__h, event))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| ShortcutsConfigScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ShortcutsConfigScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): extractRenderState, init, keyPressed, mouseClicked, onClose override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
