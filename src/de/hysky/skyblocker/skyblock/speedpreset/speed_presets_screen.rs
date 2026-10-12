//! Ported from de.hysky.skyblocker.skyblock.speedpreset.SpeedPresetsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::speedpreset::speed_preset_list_widget::SpeedPresetListWidget;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::screens::confirm_screen::ConfirmScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class SpeedPresetsScreen extends Screen
pub struct SpeedPresetsScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: protected final Screen parent;
    pub parent: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: protected @Nullable HeaderAndFooterLayout layout;
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: protected @Nullable SpeedPresetListWidget list;
    pub list: Option<compat::WidgetRef>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for SpeedPresetsScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SpeedPresetsScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SpeedPresetsScreen {
    /// java: public SpeedPresetsScreen(Screen parent)
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.parent = parent;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.general.speedPresets.config").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent: todo!("fabric2seer: the field value (TODO above)"),
            layout: None,
            list: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.list.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| compat::list_object::<SpeedPresetListWidget>(__w).expect("a mod list reached from inside its own method"))), None => None } }).is_none() {
            { let __v = (Some(SpeedPresetListWidget::new(0, 0, 24))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.list = __v; }); }
        }
        let mut layout: HeaderAndFooterLayout = HeaderAndFooterLayout::new(0, 0);
        // TODO(fabric2seer): `HeaderAndFooterLayout.addTitleHeader` has no Rust method in the port (looked for `add_title_header` in HeaderAndFooterLayout and its supertypes)
        // rust: layout.add_title_header(&compat::read(__this, |__t| __t.title.clone()), &compat::font());
        let __wa1 = layout.get_width();
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_width(__wa1)) });
        let __wa2 = layout.get_content_height();
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_height(__wa2)) });
        layout.add_to_contents({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) });
        let mut footer_layout: LinearLayout = LinearLayout::horizontal();
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        footer_layout.add_child_adjusted({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            compat::write((&__this_1), |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<SpeedPresetListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || SpeedPresetListWidget::save(__h)) })).expect("a mod list reached from inside its own method")) });
            mcrust::mcrust::modding::game::set_screen_to_parent();
        } }) })))).width(((compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(mcrust::net::minecraft::network::chat::common_components::gui_done()))).wrapping_add(8)).max(100)).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, &mut |s| { s.padding_right(2); });
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        footer_layout.add_child_adjusted({ let __w = (button::builder((component::literal("+")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { compat::write((&__this_2), |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<SpeedPresetListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || SpeedPresetListWidget::new_entry(__h)) })).expect("a mod list reached from inside its own method")) }); } }) })))).width(20).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, &mut |s| { s.padding_left(2); });
        layout.add_to_footer(LayoutElement::Grid(footer_layout.clone().wrapped));
        { let __vw = layout.visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        { let (__sw, __sh) = compat::read(__this, |__t| (__t.width, __t.height)); layout.screen_width = __sw; layout.screen_height = __sh; layout.arrange_elements(); let __vw = layout.visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.refresh_scroll_amount())).expect("a mod list reached from inside its own method")) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<SpeedPresetListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || SpeedPresetListWidget::update_position(__h)) })).expect("a mod list reached from inside its own method")) });
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.list.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| compat::list_object::<SpeedPresetListWidget>(__w).expect("a mod list reached from inside its own method"))), None => None } }).is_some() && compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<SpeedPresetListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || SpeedPresetListWidget::has_been_changed(__h)) })).expect("a mod list reached from inside its own method")) }) {
            // TODO(fabric2seer): `new ConfirmScreen(..)`: the port has no `ConfirmScreen::new`
            // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
            // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            // rust: gui::set_screen(&mut ConfirmScreen::new(Arc::new(move |confirmed_action| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            // rust: if confirmed_action {
            // rust: mcrust::mcrust::modding::game::set_screen_to_parent();
            // rust: } else {
            // rust: mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(((&__this_1)).clone())));
            // rust: }
            // rust: } }), component::translatable("text.skyblocker.quit_config"), component::translatable("text.skyblocker.quit_config_sure"), component::translatable("text.skyblocker.quit_discard").with_style_format(ChatFormatting::Red), mcrust::net::minecraft::network::chat::common_components::gui_cancel()));
            return;
        }
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}

/// SpeedPresetsScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for SpeedPresetsScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// SpeedPresetsScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<SpeedPresetsScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| SpeedPresetsScreen::init(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| SpeedPresetsScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
