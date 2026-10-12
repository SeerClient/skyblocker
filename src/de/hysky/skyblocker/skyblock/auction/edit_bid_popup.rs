//! Ported from de.hysky.skyblocker.skyblock.auction.EditBidPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::auction::auction_view_screen::AuctionViewScreen;
use crate::de::hysky::skyblocker::skyblock::calculators::sign_calculator::SignCalculator;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::EnterConfirmTextFieldWidget;
use crate::de::hysky::skyblocker::utils::render::gui::filtered_edit_box::FilteredEditBox;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::protocol::game::serverbound_sign_update_packet::ServerboundSignUpdatePacket;
use mcrust::net::minecraft::world::level::block::entity::sign_block_entity::SignBlockEntity;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class EditBidPopup extends AbstractPopupScreen
pub struct EditBidPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private LinearLayout layout = LinearLayout.vertical();
    pub layout: LinearLayout,
    /// java: private final String minimumBid;
    pub minimum_bid: String,
    /// java: private final SignBlockEntity signBlockEntity;
    pub sign_block_entity: SignBlockEntity,
    /// java: private final boolean signFront;
    pub sign_front: bool,
    /// java: private FilteredEditBox textFieldWidget;
    pub text_field_widget: Option<compat::Shared<dyn FilteredEditBox>>,
    /// java: private boolean packetSent = false;
    pub packet_sent: bool,
}

impl std::ops::Deref for EditBidPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EditBidPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EditBidPopup {
    /// java: public EditBidPopup(AuctionViewScreen auctionViewScreen, SignBlockEntity signBlockEntity, boolean signFront, String minimumBid)
    pub fn new(mut auction_view_screen: compat::Shared<AuctionViewScreen>, mut sign_block_entity: SignBlockEntity, mut sign_front: bool, mut minimum_bid: String) -> compat::Shared<Self> {
        let __init_minimum_bid = minimum_bid.clone();
        // TODO(fabric2seer): the mod keeps the game's `SignBlockEntity` (Java holds a reference to the live object); the port's `SignBlockEntity` is not a value to copy and the port lends the mod no handle to it
        //   java: this.signBlockEntity = signBlockEntity;
        let __init_sign_front = sign_front;
        // java: super(Component.literal("Edit Bid"), auctionViewScreen);
        // rust: port gap (GAPS_batchA #19) - the popup base keeps its background as the game's `Screen` value; a mod screen
        // (AuctionViewScreen) is not one, so an empty screen stands in (the base's own background field is still a TODO)
        let _ = &auction_view_screen;
        let (_, __this) = AbstractPopupScreenBase::new_in(component::literal("Edit Bid"), mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                layout: LinearLayout::vertical(),
                minimum_bid: __init_minimum_bid,
                sign_block_entity: todo!("fabric2seer: the field value (TODO above)"),
                sign_front: __init_sign_front,
                text_field_widget: None,
                packet_sent: false,
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        __this
    }

    /// java: private boolean isStringGood(String s)
    pub fn is_string_good(__this: &compat::Shared<Self>, s: &str) -> bool {
        return (compat::font().width(&compat::read(__this, |__t| __t.minimum_bid.clone())) <= __this.lock().borrow().sign_block_entity.get_max_text_line_width());
    }

    /// java: private void buttonMinimumBid(Button widget)
    pub fn button_minimum_bid(__this: &compat::Shared<Self>, widget: &AbstractWidget) {
        if !(EditBidPopup::is_string_good(__this, &compat::read(__this, |__t| __t.minimum_bid.clone()))) {
            return;
        }
        EditBidPopup::send_packet(__this, &compat::read(__this, |__t| __t.minimum_bid.clone()));
        ({ let __f = compat::dispatch(__this, |__t| __t.on_close()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
    }

    /// java: private void done(Button widget)
    /// (`widget` is Java's nullable `Button`: the text field's enter callback passes `null`)
    pub fn done<'a>(__this: &compat::Shared<Self>, widget: impl Into<Option<&'a AbstractWidget>>) {
        let _widget: Option<&AbstractWidget> = widget.into();
        if SkyblockerConfigManager::get().ui_and_visuals.input_calculator.enabled {
            if !(EditBidPopup::is_string_good(__this, &SignCalculator::get_new_value(false))) {
                return;
            }
            EditBidPopup::send_packet(__this, &SignCalculator::get_new_value(false));
        } else {
            let __o1 = compat::read(__this, |__t| __t.text_field_widget.clone()).unwrap();
            let __o2 = compat::read(__this, |__t| __t.text_field_widget.clone()).unwrap();
            if !(EditBidPopup::is_string_good(__this, &compat::trim(&(compat::write(&__o2, |__t| __t.filtered_edit_box_base_mut().get_value()))))) {
                return;
            }
            let __o3 = compat::read(__this, |__t| __t.text_field_widget.clone()).unwrap();
            EditBidPopup::send_packet(__this, &compat::trim(&(compat::write(&__o3, |__t| __t.filtered_edit_box_base_mut().get_value()))));
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.on_close()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
    }

    /// java: private void sendPacket(String string)
    pub fn send_packet(__this: &compat::Shared<Self>, string: &str) {
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        // TODO(fabric2seer): `ClientPacketListener.send` has no Rust method in the port (looked for `send` in ClientPacketListener and its supertypes)
        // rust: mcrust::mcrust::modding::game::with_player(|player| { player.connection.as_ref().unwrap().send(&ServerboundSignUpdatePacket::new(__this.lock().borrow().sign_block_entity.get_block_pos(), compat::read(__this, |__t| __t.sign_front), string.replace("coins", ""), "".to_string(), "".to_string(), "".to_string())); });
        { let __v = true; compat::write(__this, |__t| __t.packet_sent = __v); }
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        if !(compat::read(__this, |__t| __t.packet_sent)) {
            EditBidPopup::send_packet(__this, "");
        }
        ();
    }

}

impl AbstractPopupScreen for EditBidPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        EditBidPopup::init_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        EditBidPopup::extract_background_dyn
    }

    /// java: @Override public void onClose() (dispatch: the body for this class)
    fn on_close(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        EditBidPopup::on_close_dyn
    }

}

impl EditBidPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        { let __v = LinearLayout::vertical(); compat::write(__this, |__t| __t.layout = __v); }
        // java: layout.spacing(8).defaultCellSetting().alignHorizontallyCenter(); - `spacing` answers `this`
        compat::write(__this, |__t| {
            let layout = std::mem::replace(&mut __t.layout, LinearLayout::vertical());
            __t.layout = layout.spacing(8);
            __t.layout.default_cell_setting().align_horizontally_center();
        });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some((EnterConfirmTextFieldWidget::new(compat::font(), 120, 15, component::empty(), Arc::new(move || { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { EditBidPopup::done((&__this_1), None::<&AbstractWidget> /* java: null */); } })) as compat::Shared<dyn FilteredEditBox>)); compat::write(__this, |__t| __t.text_field_widget = __v); }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        ({ let __r = &compat::read(__this, |__t| __t.text_field_widget.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_filter()); __f(__r, Arc::new(move |__a0| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; EditBidPopup::is_string_good((&__this_2), &__a0) })) });
        { let (__q0,) = ({ let __w = compat::string_widget(0, 0, -1, 9, (component::literal("- Set Bid -")).with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_bold(Some(true)))); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.add_child(__q0) };
        // java: layout.addChild(textFieldWidget);
        // rust: port gap (GAPS_batchA #3) - the text field is a mod subclass of EditBox (FilteredEditBox); the port has no
        // widget adapter for an EditBox subclass, so it cannot join the screen's widgets or this layout
        let __r0 = compat::read(__this, |__t| __t.minimum_bid.clone());
        { let (__q0,) = ({ let __w = compat::string_widget(0, 0, -1, 9, component::literal(&format!("Minimum Bid: {}", __r0))); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.add_child(__q0) };
        let mut horizontal: LinearLayout = LinearLayout::horizontal();
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut button_widget: AbstractWidget = (button::builder(component::literal("Set Minimum Bid").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { EditBidPopup::button_minimum_bid((&__this_3), &compat::write((&__this_3), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.clone()) })); } }) })))).width(80).build();
        button_widget.active = EditBidPopup::is_string_good(__this, &compat::read(__this, |__t| __t.minimum_bid.clone()));
        horizontal.add_child({ let __w = button_widget.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        horizontal.add_child({ let __w = (button::builder(component::literal("Done").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { EditBidPopup::done((&__this_4), &compat::write((&__this_4), |__t| { let __s = &mut *__t; let __r = &mut __pidx; __s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.clone()) })); } }) })))).width(80).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        { let (__q0,) = (LayoutElement::Grid(horizontal.clone().wrapped),); __this.lock().borrow_mut().layout.add_child(__q0) };
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        // java: layout.arrangeElements(); FrameLayout.centerInRectangle(layout, getRectangle()); - the widgets take their
        // positions from the centred layout
        {
            let mut layout: LinearLayout = compat::read(__this, |__t| __t.layout.clone());
            layout.arrange_elements();
            let rectangle = compat::read(__this, |__t| __t.get_rectangle());
            frame_layout::center_in_rectangle_of(&mut *compat::LayoutBack::grid(&mut layout.wrapped), rectangle);
            let mut __vw = Vec::new();
            LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw);
            for (__i, __x, __y) in __vw {
                compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y));
            }
            compat::write(__this, |__t| __t.layout = layout);
        }
        // java: setInitialFocus(textFieldWidget);
        // rust: port gap (GAPS_batchA #3) - the text field is not one of the screen's widgets (see above), so it has no index to focus
    }

    /// EditBidPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        AbstractPopupScreenBase::extract_popup_background(graphics, __this.lock().borrow().layout.get_x(), __this.lock().borrow().layout.get_y(), __this.lock().borrow().layout.get_width(), __this.lock().borrow().layout.get_height());
        if SkyblockerConfigManager::get().ui_and_visuals.input_calculator.enabled {
            let __o1 = compat::read(__this, |__t| __t.text_field_widget.clone()).unwrap();
            let __o2 = compat::read(__this, |__t| __t.text_field_widget.clone()).unwrap();
            SignCalculator::extract_calculator(graphics, compat::write(&__o1, |__t| __t.filtered_edit_box_base_mut().get_value()), (gui_graphics_extractor::gui_width(&*compat::minecraft())).wrapping_div(2), (compat::write(&__o2, |__t| __t.filtered_edit_box_base_mut().get_y())).wrapping_sub(8));
        }
    }

    /// EditBidPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void onClose()
    pub fn on_close_impl(__this: &compat::Shared<Self>) {
        if !(compat::read(__this, |__t| __t.packet_sent)) {
            EditBidPopup::send_packet(__this, "");
        }
        mcrust::mcrust::modding::game::set_screen_null();
    }

    /// EditBidPopup.onClose as the dispatch of AbstractPopupScreen.onClose
    pub fn on_close_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::on_close_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// EditBidPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for EditBidPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// EditBidPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<EditBidPopup> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.reposition_elements()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| EditBidPopup::removed(__h))
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.added()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
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
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

}
