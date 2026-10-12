//! Ported from de.hysky.skyblocker.skyblock.quicknav.QuickNavConfirmationButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton;
use crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButtonBase;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::resources::sounds::simple_sound_instance::SimpleSoundInstance;
use mcrust::net::minecraft::client::sounds::sound_manager::SoundManager;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Tooltip CONFIRM_TOOLTIP = Tooltip.create(Component.translatable("skyblocker.quickNav.confirm"));
pub static CONFIRM_TOOLTIP: compat::MainThread<Tooltip> = compat::MainThread::new(|| Tooltip::create(component::translatable("skyblocker.quickNav.confirm").get_string()));
/// java: private static final long DOUBLE_CLICK_TIME = 1000;
pub const DOUBLE_CLICK_TIME: i64 = ((1000) as i64) as i64;

/// java: public class QuickNavConfirmationButton extends QuickNavButton
#[derive(Clone)]
pub struct QuickNavConfirmationButton {
    /// java: extends QuickNavButton
    pub base: QuickNavButtonBase,
    /// java: private long lastClicked = 0;
    pub last_clicked: i64,
    /// java: private boolean showingConfirmTooltip;
    pub showing_confirm_tooltip: bool,
    /// java: protected final int index;
    pub index: i32,
}

impl std::ops::Deref for QuickNavConfirmationButton {
    type Target = QuickNavButtonBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for QuickNavConfirmationButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl QuickNavConfirmationButton {
    /// java: public QuickNavConfirmationButton(int index, boolean toggled, String command, ItemStack icon, String tooltip)
    pub fn new(mut index: i32, mut toggled: bool, mut command: String, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tooltip: String) -> compat::Shared<Self> {
        let __init_index = index;
        let (_, __this) = QuickNavButtonBase::new_in(index, toggled, command.clone(), icon.clone(), tooltip.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                last_clicked: ((0) as i64) as i64,
                showing_confirm_tooltip: false,
                index: __init_index,
            });
            ((__h.clone() as compat::Shared<dyn QuickNavButton>), __h)
        });
        __this
    }

    /// java: @Override public void playDownSound(SoundManager soundManager)
    pub fn play_down_sound(__this: &compat::Shared<Self>, sound_manager: &mut SoundManager) {
        if QuickNavConfirmationButton::is_double_click(__this) {
            // TODO(fabric2seer): `AbstractWidget.playDownSound` has no Rust method in the port (looked for `play_down_sound` in AbstractWidget and its supertypes)
            // rust: __this.lock().borrow_mut().base.play_down_sound(sound_manager);
            return;
        }
        // TODO(fabric2seer): `play` takes 4 argument(s) in the port (`_al: &OpenAl, library: &mut Library, options: &OptionsData, instance: Box<dyn SoundInstance>`), Java passes 1
        // rust: sound_manager.play(&SimpleSoundInstance::for_ui(mcrust::net::minecraft::sounds::sound_events::note_block_chime(), 1.0f32));
    }

    /// java: private boolean isDoubleClick()
    pub fn is_double_click(__this: &compat::Shared<Self>) -> bool {
        let mut now: i64 = compat::millis();
        return (now.wrapping_sub(compat::read(__this, |__t| __t.last_clicked)) < DOUBLE_CLICK_TIME);
    }

}

impl QuickNavButton for QuickNavConfirmationButton {
    fn quick_nav_button_base(&self) -> &QuickNavButtonBase {
        &self.base
    }
    fn quick_nav_button_base_mut(&mut self) -> &mut QuickNavButtonBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn QuickNavButton>, &GuiGraphicsExtractor, i32, i32, f32) {
        QuickNavConfirmationButton::extract_widget_render_state_dyn
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn on_click(&self) -> fn(&compat::Shared<dyn QuickNavButton>, &MouseButtonEvent, bool) {
        QuickNavConfirmationButton::on_click_dyn
    }

}

impl QuickNavConfirmationButton {
    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn QuickNavButton>::extract_widget_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn QuickNavButton>), graphics, mouse_x, mouse_y, a);
        if { let __f = compat::dispatch(__this, |__t| __t.toggled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn QuickNavButton>)) } {
            return;
        }
        if QuickNavConfirmationButton::is_double_click(__this) == compat::read(__this, |__t| __t.showing_confirm_tooltip) {
            return;
        }
        { let __v = !(compat::read(__this, |__t| __t.showing_confirm_tooltip)); compat::write(__this, |__t| __t.showing_confirm_tooltip = __v); }
        let __r0 = compat::read(__this, |__t| __t.showing_confirm_tooltip);
        let __r1 = compat::read(__this, |__t| __t.tooltip.clone());
        { let (__q0,) = ((if __r0 { Some(CONFIRM_TOOLTIP.get_mut().clone()) } else { __r1 }).unwrap(),); __this.lock().borrow_mut().set_tooltip(__q0) };
    }

    /// QuickNavConfirmationButton.extractWidgetRenderState as the dispatch of QuickNavButton.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn QuickNavButton>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click_impl(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        if !(QuickNavConfirmationButton::is_double_click(__this)) {
            { let __v = compat::millis(); compat::write(__this, |__t| __t.last_clicked = __v); }
            return;
        }
        <dyn QuickNavButton>::on_click_body(&compat::view(__this, |a| a as compat::Shared<dyn QuickNavButton>), click, doubled);
    }

    /// QuickNavConfirmationButton.onClick as the dispatch of QuickNavButton.onClick
    pub fn on_click_dyn(__this: &compat::Shared<dyn QuickNavButton>, __p0: &MouseButtonEvent, __p1: bool) {
        Self::on_click_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

/// QuickNavConfirmationButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for QuickNavConfirmationButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// QuickNavConfirmationButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<QuickNavConfirmationButton> {
    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn QuickNavButton>), &context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(&compat::view(__h, |a| a as compat::Shared<dyn QuickNavButton>), output) }))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_click()); __f(&compat::view(__h, |a| a as compat::Shared<dyn QuickNavButton>), event, double_click) }))
    }

}
