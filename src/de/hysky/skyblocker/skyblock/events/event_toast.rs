//! Ported from de.hysky.skyblocker.skyblock.events.EventToast by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::toasts::toast_manager::ToastManager;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;

/// java: protected static final Identifier TEXTURE = SkyblockerMod.id("notification");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("notification"));

/// java: public class EventToast implements Toast
pub struct EventToastBase {
    /// java: private long toastTime = 0;
    pub toast_time: i64,
    /// java: private final long eventStartTime;
    pub event_start_time: i64,
    /// java: private final long eventEndTime;
    pub event_end_time: i64,
    /// java: protected final List<FormattedCharSequence> message;
    ///
    /// The lines as the port's `Font.split` answers them and its sequence draw takes them (`FormattedText`).
    pub message: Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>,
    /// java: protected final List<FormattedCharSequence> messageNow;
    pub message_now: Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>,
    /// java: protected int messageWidth;
    pub message_width: i32,
    /// java: protected int messageNowWidth;
    pub message_now_width: i32,
    /// java: protected final FlexibleItemStack icon;
    pub icon: Option<compat::Shared<FlexibleItemStack>>,
    /// java: protected boolean started;
    pub started: bool,
}

impl EventToastBase {
    /// java: public EventToast(long eventStartTime, long eventEndTime, String name, FlexibleItemStack icon) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut event_start_time: i64, mut event_end_time: i64, mut name: String, mut icon: compat::Shared<FlexibleItemStack>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn EventToast>, __R)) -> (compat::Shared<dyn EventToast>, __R) {
        let __init_event_start_time = event_start_time;
        let __init_event_end_time = event_end_time;
        let mut formatted: MutableComponent = (component::translatable_args("skyblocker.events.startsSoon", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&name).with_style_format(ChatFormatting::Yellow))])).with_style_format(ChatFormatting::White);
        let mut renderer: Font = compat::font();
        let __init_message = renderer.split(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(formatted.clone()), 150);
        let this = Self {
            toast_time: ((0) as i64) as i64,
            event_start_time: __init_event_start_time,
            event_end_time: __init_event_end_time,
            message: __init_message,
            message_now: Vec::new(),
            message_width: 0,
            message_now_width: 0,
            icon: None,
            started: false,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __v = compat::read(&__this, |__t| __t.event_toast_base().message.clone()).into_iter().map(|__a0| renderer.width_of_cached(&__a0)).max().unwrap_or(150); compat::write(&__this, |__t| __t.event_toast_base_mut().message_width = __v); }
        let mut formatted_now: MutableComponent = (component::translatable_args("skyblocker.events.startsNow", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&name).with_style_format(ChatFormatting::Yellow))])).with_style_format(ChatFormatting::White);
        { let __v = renderer.split(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(formatted_now.clone()), 150); compat::write(&__this, |__t| __t.event_toast_base_mut().message_now = __v); }
        { let __v = compat::read(&__this, |__t| __t.event_toast_base().message_now.clone()).into_iter().map(|__a0| renderer.width_of_cached(&__a0)).max().unwrap_or(150); compat::write(&__this, |__t| __t.event_toast_base_mut().message_now_width = __v); }
        { let __v = Some(icon.clone()); compat::write(&__this, |__t| __t.event_toast_base_mut().icon = __v); }
        { let __v = (event_start_time.wrapping_sub((compat::millis()).wrapping_div(1000 as i64)) < (0 as i64)); compat::write(&__this, |__t| __t.event_toast_base_mut().started = __v); }
        (__this, __r)
    }

    /// java: public EventToast(long eventStartTime, long eventEndTime, String name, FlexibleItemStack icon)
    pub fn new(mut event_start_time: i64, mut event_end_time: i64, mut name: String, mut icon: compat::Shared<FlexibleItemStack>) -> compat::Shared<Self> {
        Self::new_in(event_start_time, event_end_time, name, icon, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn EventToast>), __h)
        }).1
    }

}

/// java: the virtual methods of EventToast (implemented by EventToastBase and every subclass)
pub trait EventToast: compat::DynAny {
    fn event_toast_base(&self) -> &EventToastBase;
    fn event_toast_base_mut(&mut self) -> &mut EventToastBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, Font textRenderer, long startTime) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn EventToast>, &GuiGraphicsExtractor, &Font, i64) {
        <dyn EventToast>::extract_render_state_body
    }

    /// java: protected int extractMessage(GuiGraphicsExtractor graphics, int x, int y, int color) (dispatch: the body for this class)
    fn extract_message(&self) -> fn(&compat::Shared<dyn EventToast>, &GuiGraphicsExtractor, i32, i32, i32) -> i32 {
        <dyn EventToast>::extract_message_body
    }

    /// java: protected void extractTimer(GuiGraphicsExtractor graphics, int x, int y) (dispatch: the body for this class)
    fn extract_timer(&self) -> fn(&compat::Shared<dyn EventToast>, &GuiGraphicsExtractor, i32, i32) {
        <dyn EventToast>::extract_timer_body
    }

    /// java: @Override public int width() (dispatch: the body for this class)
    fn width(&self) -> fn(&compat::Shared<dyn EventToast>) -> i32 {
        <dyn EventToast>::width_body
    }

    /// java: protected int getInnerContentsHeight() (dispatch: the body for this class)
    fn get_inner_contents_height(&self) -> fn(&compat::Shared<dyn EventToast>) -> i32 {
        <dyn EventToast>::get_inner_contents_height_body
    }

    /// java: @Override public int height() (dispatch: the body for this class)
    fn height(&self) -> fn(&compat::Shared<dyn EventToast>) -> i32 {
        <dyn EventToast>::height_body
    }

    /// java: @Override public Visibility getWantedVisibility() (dispatch: the body for this class)
    fn get_wanted_visibility(&self) -> fn(&compat::Shared<dyn EventToast>) -> mcrust::net::minecraft::client::gui::components::toasts::toast::Visibility {
        <dyn EventToast>::get_wanted_visibility_body
    }

    /// java: @Override public void update(ToastManager manager, long time) (dispatch: the body for this class)
    fn update(&self) -> fn(&compat::Shared<dyn EventToast>, &ToastManager, i64) {
        <dyn EventToast>::update_body
    }

}

impl dyn EventToast {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, Font textRenderer, long startTime)
    pub fn extract_render_state_body(__this: &compat::Shared<dyn EventToast>, graphics: &GuiGraphicsExtractor, text_renderer: &Font, mut start_time: i64) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), 0, 0, ({ let __f = compat::dispatch(__this, |__t| __t.width()); __f(__this) }), ({ let __f = compat::dispatch(__this, |__t| __t.height()); __f(__this) }));
        let mut y: i32 = (({ let __f = compat::dispatch(__this, |__t| __t.height()); __f(__this) }).wrapping_sub({ let __f = compat::dispatch(__this, |__t| __t.get_inner_contents_height()); __f(__this) })).wrapping_div(2);
        y = 2i32.wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.extract_message()); __f(__this, graphics, 30, y, mcrust::net::minecraft::util::common_colors::WHITE) });
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_timer()); __f(__this, graphics, 30, y) });
        graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(&compat::read(__this, |__t| __t.event_toast_base().icon.clone()).unwrap()), 8, (({ let __f = compat::dispatch(__this, |__t| __t.height()); __f(__this) }).wrapping_div(2)).wrapping_sub(8));
    }

    /// java: protected int extractMessage(GuiGraphicsExtractor graphics, int x, int y, int color)
    pub fn extract_message_body(__this: &compat::Shared<dyn EventToast>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut color: i32) -> i32 {
        let mut text_renderer: Font = compat::font();
        for ordered_text in (if compat::read(__this, |__t| __t.event_toast_base().started) { compat::read(__this, |__t| __t.event_toast_base().message_now.clone()) } else { compat::read(__this, |__t| __t.event_toast_base().message.clone()) }) {
            gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &ordered_text, x, y, color, false);
            y = y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT);
        }
        return y;
    }

    /// java: protected void extractTimer(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_timer_body(__this: &compat::Shared<dyn EventToast>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut current_time: i64 = (compat::millis()).wrapping_div(1000 as i64);
        let mut time_till_event: i32 = (((compat::read(__this, |__t| __t.event_toast_base().event_start_time)).wrapping_sub(current_time)) as i32);
        { let __v = (time_till_event < 0); compat::write(__this, |__t| __t.event_toast_base_mut().started = __v); }
        if compat::read(__this, |__t| __t.event_toast_base().started) {
            return;
        }
        let mut time: MutableComponent = SkyblockTime::format_time((time_till_event) as f64);
        let mut text_renderer: Font = compat::font();
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(time.clone()), x, y, mcrust::net::minecraft::util::common_colors::SOFT_YELLOW, false);
    }

    /// java: @Override public int width()
    pub fn width_body(__this: &compat::Shared<dyn EventToast>) -> i32 {
        return ((if compat::read(__this, |__t| __t.event_toast_base().started) { compat::read(__this, |__t| __t.event_toast_base().message_now_width) } else { compat::read(__this, |__t| __t.event_toast_base().message_width) }).wrapping_add(30)).wrapping_add(6);
    }

    /// java: protected int getInnerContentsHeight()
    pub fn get_inner_contents_height_body(__this: &compat::Shared<dyn EventToast>) -> i32 {
        return ((__this.lock().borrow().event_toast_base().message.len() as i32).wrapping_mul(9)).wrapping_add(if compat::read(__this, |__t| __t.event_toast_base().started) { 0 } else { 9 });
    }

    /// java: @Override public int height()
    pub fn height_body(__this: &compat::Shared<dyn EventToast>) -> i32 {
        return ((({ let __f = compat::dispatch(__this, |__t| __t.get_inner_contents_height()); __f(__this) }).wrapping_add(12)).wrapping_add(2)).max(32);
    }

    /// java: @Override public Visibility getWantedVisibility()
    pub fn get_wanted_visibility_body(__this: &compat::Shared<dyn EventToast>) -> mcrust::net::minecraft::client::gui::components::toasts::toast::Visibility {
        let mut current_time: i64 = (compat::millis()).wrapping_div(1000 as i64);
        if compat::read(__this, |__t| __t.event_toast_base().toast_time) > (5000 as i64) || current_time > compat::read(__this, |__t| __t.event_toast_base().event_end_time) {
            return mcrust::net::minecraft::client::gui::components::toasts::toast::Visibility::Hide;
        }
        return mcrust::net::minecraft::client::gui::components::toasts::toast::Visibility::Show;
    }

    /// java: @Override public void update(ToastManager manager, long time)
    pub fn update_body(__this: &compat::Shared<dyn EventToast>, manager: &ToastManager, mut time: i64) {
        { let __v = time; compat::write(__this, |__t| __t.event_toast_base_mut().toast_time = __v); }
    }

}

impl EventToast for EventToastBase {
    fn event_toast_base(&self) -> &EventToastBase {
        self
    }
    fn event_toast_base_mut(&mut self) -> &mut EventToastBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// `EventToast implements Toast`: the object as the game's `Toast`, for `ToastManager.addToast` - each of the
/// interface's methods dispatches to the class's own (`compat::dispatch`, so a subclass's overrides run).
pub struct EventToastAdapter(pub compat::Shared<dyn EventToast>);

impl mcrust::net::minecraft::client::gui::components::toasts::toast::Toast for EventToastAdapter {
    fn get_wanted_visibility(&self) -> mcrust::net::minecraft::client::gui::components::toasts::toast::Visibility {
        let __f = compat::dispatch(&self.0, |__t| __t.get_wanted_visibility());
        __f(&self.0)
    }

    fn update(&mut self, manager: &ToastManager, fully_visible_for_ms: i64) {
        let __f = compat::dispatch(&self.0, |__t| __t.update());
        __f(&self.0, manager, fully_visible_for_ms)
    }

    fn extract_render_state(
        &self,
        _graphics: &mut mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>,
        _font: &mcrust::net::minecraft::client::gui::font::font_set::FontSet,
        fully_visible_for_ms: i64,
    ) {
        // the class's body draws through the `gui_graphics_extractor` functions (the frame's own state), as the HUD
        // callbacks do; its `graphics` is the extractor those functions stand for
        let __f = compat::dispatch(&self.0, |__t| __t.extract_render_state());
        __f(&self.0, &GuiGraphicsExtractor::new(), &compat::font(), fully_visible_for_ms)
    }

    fn width(&self) -> i32 {
        let __f = compat::dispatch(&self.0, |__t| __t.width());
        __f(&self.0)
    }

    fn height(&self) -> i32 {
        let __f = compat::dispatch(&self.0, |__t| __t.height());
        __f(&self.0)
    }
}
