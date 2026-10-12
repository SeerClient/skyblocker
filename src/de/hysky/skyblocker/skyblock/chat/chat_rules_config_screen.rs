//! Ported from de.hysky.skyblocker.skyblock.chat.ChatRulesConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rules_config_list_widget::ChatRulesConfigListWidget;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
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

/// java: public class ChatRulesConfigScreen extends Screen
pub struct ChatRulesConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private ChatRulesConfigListWidget chatRulesConfigListWidget;
    pub chat_rules_config_list_widget: Option<compat::WidgetRef>,
    /// java: private HeaderAndFooterLayout layout;
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for ChatRulesConfigScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChatRulesConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChatRulesConfigScreen {
    /// java: public ChatRulesConfigScreen(Screen parent)
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __init_parent = parent;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            chat_rules_config_list_widget: None,
            layout: None,
            parent: __init_parent,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(__this, |__t| __t.layout = __v); }
        // TODO(fabric2seer): `HeaderAndFooterLayout.addTitleHeader` has no Rust method in the port (looked for `add_title_header` in HeaderAndFooterLayout and its supertypes)
        // rust: let __h0 = &compat::read(__this, |__t| __t.title.clone());
        // rust: let __h1 = &compat::font();
        // rust: __this.lock().borrow().layout.as_ref().unwrap().add_title_header(__h0, __h1);
        { let __v = (Some(ChatRulesConfigListWidget::new(compat::minecraft(), __this.clone(), compat::read(__this, |__t| __t.width), __this.lock().borrow().layout.as_ref().unwrap().get_content_height(), __this.lock().borrow().layout.as_ref().unwrap().get_header_height()))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.chat_rules_config_list_widget = __v; }); }
        { let (__q0,) = ({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.chat_rules_config_list_widget.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        // TODO(fabric2seer): field `HORIZONTAL` of a value of unknown type
        //   java: LinearLayout footerLayout = layout.addToFooter(new LinearLayout(0, 0, LinearLayout.Orientation.HORIZONTAL));
        let mut footer_layout: LinearLayout = todo!("fabric2seer: the initialiser of `footerLayout` (TODO above)");
        footer_layout.default_cell_setting().padding_horizontal(5).padding_vertical(2);
        let __this_1 = compat::RenderThread::new(__this.clone());
        footer_layout.add_child({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_1.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            ChatRulesConfigScreen::on_close(__this_1.get());
        } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __this_2 = compat::RenderThread::new(__this.clone());
        footer_layout.add_child({ let __w = (button::builder(component::translatable("skyblocker.config.chat.chatRules.screen.new").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_2.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { compat::write(__this_2.get(), |__t| { let __s = &mut *__t; let __r = __s.chat_rules_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ChatRulesConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ChatRulesConfigListWidget::add_rule_after_selected(__h)) })).expect("a mod list reached from inside its own method")) }); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __this_3 = compat::RenderThread::new(__this.clone());
        footer_layout.add_child({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_3.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            compat::write(__this_3.get(), |__t| { let __s = &mut *__t; let __r = __s.chat_rules_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ChatRulesConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ChatRulesConfigListWidget::save_rules(__h)) })).expect("a mod list reached from inside its own method")) });
            ChatRulesConfigScreen::on_close(__this_3.get());
        } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        let __wa1 = compat::read(__this, |__t| __t.width);
        let __wa2 = __this.lock().borrow().layout.as_ref().unwrap().get_content_height();
        let __wa3 = __this.lock().borrow().layout.as_ref().unwrap().get_header_height();
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.chat_rules_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_rectangle(__wa1, __wa2, 0, __wa3)) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.chat_rules_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.refresh_scroll_amount())).expect("a mod list reached from inside its own method")) });
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if !(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.chat_rules_config_list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<ChatRulesConfigListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || ChatRulesConfigListWidget::has_changes(__h)) })).expect("a mod list reached from inside its own method")) })) {
            mcrust::mcrust::modding::game::set_screen_to_parent();
            return;
        }
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
    }

}

/// ChatRulesConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ChatRulesConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ChatRulesConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ChatRulesConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ChatRulesConfigScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ChatRulesConfigScreen::reposition_elements(__h))
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
        self.call(|__h| ChatRulesConfigScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
