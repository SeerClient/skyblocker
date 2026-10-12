//! Ported from de.hysky.skyblocker.skyblock.quicknav.QuickNavButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::screen_utils::ScreenUtils;
use crate::entry::skyblocker_mod;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::component_path::ComponentPath;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::focus_navigation_event::FocusNavigationEvent;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final long TOGGLE_DURATION = 1000;
pub const TOGGLE_DURATION: i64 = ((1000) as i64) as i64;
/// java: @SuppressWarnings("unchecked") private static final @Nullable FallbackedTexture<Identifier>[] TAB_TEXTURES = new FallbackedTexture[14];
pub static TAB_TEXTURES: compat::MainThread<Vec<Option<compat::Shared<dyn FallbackedTexture<Identifier>>>>> = compat::MainThread::new(|| (0..(14) as usize).map(|_| None).collect::<Vec<_>>());
/// java: @SuppressWarnings("unchecked") private static final @Nullable FallbackedTexture<Identifier>[] TAB_TEXTURES_SELECTED = new FallbackedTexture[14];
pub static TAB_TEXTURES_SELECTED: compat::MainThread<Vec<Option<compat::Shared<dyn FallbackedTexture<Identifier>>>>> = compat::MainThread::new(|| (0..(14) as usize).map(|_| None).collect::<Vec<_>>());

/// java: @Environment(value = EnvType.CLIENT) public class QuickNavButton extends AbstractWidget
#[derive(Clone)]
pub struct QuickNavButtonBase {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final int index;
    pub index: i32,
    /// java: private final boolean toggled;
    pub toggled: bool,
    /// java: private final String command;
    pub command: Option<String>,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: protected final Tooltip tooltip;
    pub tooltip: Option<Tooltip>,
    /// java: private boolean temporaryToggled = false;
    pub temporary_toggled: bool,
    /// java: private long toggleTime;
    pub toggle_time: i64,
    /// java: private boolean renderInFront;
    pub render_in_front: bool,
    /// java: private int alpha = 255;
    pub alpha: i32,
}

impl std::ops::Deref for QuickNavButtonBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for QuickNavButtonBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl QuickNavButtonBase {
    /// java: public QuickNavButton(int index, boolean toggled, String command, ItemStack icon, String tooltip) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut index: i32, mut toggled: bool, mut command: String, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tooltip: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn QuickNavButton>, __R)) -> (compat::Shared<dyn QuickNavButton>, __R) {
        let __init_index = index;
        let __init_toggled = toggled;
        let __init_command = Some(command.clone());
        let __init_icon = icon.clone();
        let __init_toggle_time = ((0) as i64) as i64;
        let this = Self {
            base: AbstractWidget::new(0, 0, 26, 32, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            index: __init_index,
            toggled: __init_toggled,
            command: __init_command,
            icon: __init_icon,
            tooltip: None,
            temporary_toggled: false,
            toggle_time: __init_toggle_time,
            render_in_front: false,
            alpha: 255,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        if false /* java: tooltip == null: never null here */ || tooltip.is_empty() {
            { let __v = None; compat::write(&__this, |__t| __t.quick_nav_button_base_mut().tooltip = __v); }
            return (__this, __r);
        }
        // java: try { setTooltip(tip = Tooltip.create(ComponentSerialization.CODEC.decode(JsonOps.INSTANCE, SkyblockerMod.GSON.fromJson(tooltip, JsonElement.class)).getOrThrow().getFirst())); }
        //       catch (Exception _) { setTooltip(tip = Tooltip.create(Component.literal(tooltip))); }
        let __decoded: Option<mcrust::net::minecraft::network::chat::formatted_text::FormattedText> = serde_json::from_str::<serde_json::Value>(&tooltip).ok().and_then(|json| {
            use mcrust::com::mojang::serialization::decoder::Decoder as _;
            mcrust::net::minecraft::network::chat::component_serialization::codec()
                .decode(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json))
                .get_or_throw()
                .ok()
                .map(|(component, _)| component)
        });
        let mut tip: Tooltip = match __decoded {
            Some(component) => Tooltip::create_text(component),
            None => Tooltip::create_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&tooltip))),
        };
        { let (__q0,) = (tip.clone(),); __this.lock().borrow_mut().quick_nav_button_base_mut().set_tooltip(__q0) };
        { let __v = Some(tip.clone()); compat::write(&__this, |__t| __t.quick_nav_button_base_mut().tooltip = __v); }
        // java: setTooltipDelay(Duration.ofMillis(100));
        __this.lock().borrow_mut().quick_nav_button_base_mut().set_tooltip_delay(100);
        (__this, __r)
    }

    /// java: public QuickNavButton(int index, boolean toggled, String command, ItemStack icon, String tooltip)
    pub fn new(mut index: i32, mut toggled: bool, mut command: String, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut tooltip: String) -> compat::Shared<Self> {
        Self::new_in(index, toggled, command, icon, tooltip, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn QuickNavButton>), __h)
        }).1
    }

}

/// java: the virtual methods of QuickNavButton (implemented by QuickNavButtonBase and every subclass)
pub trait QuickNavButton: compat::DynAny {
    fn quick_nav_button_base(&self) -> &QuickNavButtonBase;
    fn quick_nav_button_base_mut(&mut self) -> &mut QuickNavButtonBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: private boolean isTopTab() (dispatch: the body for this class)
    fn is_top_tab(&self) -> fn(&compat::Shared<dyn QuickNavButton>) -> bool {
        <dyn QuickNavButton>::is_top_tab_body
    }

    /// java: public boolean toggled() (dispatch: the body for this class)
    fn toggled(&self) -> fn(&compat::Shared<dyn QuickNavButton>) -> bool {
        <dyn QuickNavButton>::toggled_body
    }

    /// java: public void setRenderInFront(boolean renderInFront) (dispatch: the body for this class)
    fn set_render_in_front(&self) -> fn(&compat::Shared<dyn QuickNavButton>, bool) {
        <dyn QuickNavButton>::set_render_in_front_body
    }

    /// java: public float getAlpha() (dispatch: the body for this class)
    fn get_alpha(&self) -> fn(&compat::Shared<dyn QuickNavButton>) -> f32 {
        <dyn QuickNavButton>::get_alpha_body
    }

    /// java: private void updateCoordinates() (dispatch: the body for this class)
    fn update_coordinates(&self) -> fn(&compat::Shared<dyn QuickNavButton>) {
        <dyn QuickNavButton>::update_coordinates_body
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn on_click(&self) -> fn(&compat::Shared<dyn QuickNavButton>, &MouseButtonEvent, bool) {
        <dyn QuickNavButton>::on_click_body
    }

    /// java: @Override public @Nullable ComponentPath nextFocusPath(FocusNavigationEvent navigation) (dispatch: the body for this class)
    fn next_focus_path(&self) -> fn(&compat::Shared<dyn QuickNavButton>, &FocusNavigationEvent) -> Option<ComponentPath> {
        <dyn QuickNavButton>::next_focus_path_body
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn QuickNavButton>, &GuiGraphicsExtractor, i32, i32, f32) {
        <dyn QuickNavButton>::extract_widget_render_state_body
    }

    /// java: private Identifier getTexture() (dispatch: the body for this class)
    fn get_texture(&self) -> fn(&compat::Shared<dyn QuickNavButton>) -> Identifier {
        <dyn QuickNavButton>::get_texture_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn QuickNavButton>, &dyn NarrationElementOutput) {
        <dyn QuickNavButton>::update_widget_narration_body
    }

}

impl dyn QuickNavButton {
    /// java: private boolean isTopTab()
    pub fn is_top_tab_body(__this: &compat::Shared<dyn QuickNavButton>) -> bool {
        return (compat::read(__this, |__t| __t.quick_nav_button_base().index) < 7);
    }

    /// java: public boolean toggled()
    pub fn toggled_body(__this: &compat::Shared<dyn QuickNavButton>) -> bool {
        return (compat::read(__this, |__t| __t.quick_nav_button_base().toggled) || compat::read(__this, |__t| __t.quick_nav_button_base().temporary_toggled));
    }

    /// java: public void setRenderInFront(boolean renderInFront)
    pub fn set_render_in_front_body(__this: &compat::Shared<dyn QuickNavButton>, mut render_in_front: bool) {
        { let __v = render_in_front; compat::write(__this, |__t| __t.quick_nav_button_base_mut().render_in_front = __v); }
    }

    /// java: public float getAlpha()
    pub fn get_alpha_body(__this: &compat::Shared<dyn QuickNavButton>) -> f32 {
        return ((compat::read(__this, |__t| __t.quick_nav_button_base().alpha) as f32) / 255.0f32);
    }

    /// java: private void updateCoordinates()
    pub fn update_coordinates_body(__this: &compat::Shared<dyn QuickNavButton>) {
        // java: if (ScreenUtils.getUnderlyingScreen() instanceof AbstractContainerScreen<?> handledScreen) - the open container
        // screen's panel (`AbstractContainerScreenAccessor.getX/getY/getImageWidth/getImageHeight`); `ContainerScreen` is
        // the generic chest screen
        let Some((x, y, w, mut h, is_container_screen)) = compat::container_screen_panel() else {
            return;
        };
        if is_container_screen {
            h -= 1; // they messed up the height on these.
        }
        let __r0 = compat::read(__this, |__t| __t.quick_nav_button_base().index);
        { let (__q0,) = (((x.wrapping_add(((__r0).wrapping_rem(7)).wrapping_mul(25))).wrapping_add(w.wrapping_div(2))).wrapping_sub(176i32.wrapping_div(2)),); __this.lock().borrow_mut().quick_nav_button_base_mut().set_x(__q0) };
        let __r0 = compat::read(__this, |__t| __t.quick_nav_button_base().index);
        { let (__q0,) = ((if __r0 < 7 { y.wrapping_sub(28) } else { (y.wrapping_add(h)).wrapping_sub(4) }),); __this.lock().borrow_mut().quick_nav_button_base_mut().set_y(__q0) };
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click_body(__this: &compat::Shared<dyn QuickNavButton>, click: &MouseButtonEvent, mut doubled: bool) {
        if !(compat::read(__this, |__t| __t.quick_nav_button_base().temporary_toggled)) {
            { let __v = true; compat::write(__this, |__t| __t.quick_nav_button_base_mut().temporary_toggled = __v); }
            { let __v = compat::millis(); compat::write(__this, |__t| __t.quick_nav_button_base_mut().toggle_time = __v); }
            if compat::read(__this, |__t| __t.quick_nav_button_base().command.clone()).is_none() || __this.lock().borrow().quick_nav_button_base().command.as_ref().unwrap().is_empty() {
                let __index = compat::read(__this, |__t| __t.quick_nav_button_base().index);
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append((component::literal(&format!("Quick Nav button index {} has no command!", (__index).wrapping_add(1)))).with_style_format(ChatFormatting::Red)).to_string()); });
            } else {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), compat::read(__this, |__t| __t.quick_nav_button_base().command.clone()).as_deref().unwrap(), true);
            }
            { let __v = 0; compat::write(__this, |__t| __t.quick_nav_button_base_mut().alpha = __v); }
        }
    }

    /// java: @Override public @Nullable ComponentPath nextFocusPath(FocusNavigationEvent navigation)
    pub fn next_focus_path_body(__this: &compat::Shared<dyn QuickNavButton>, navigation: &FocusNavigationEvent) -> Option<ComponentPath> {
        return None;
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn QuickNavButton>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.update_coordinates()); __f(__this) });
        if compat::read(__this, |__t| __t.quick_nav_button_base().temporary_toggled) && (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.quick_nav_button_base().toggle_time)) >= TOGGLE_DURATION {
            { let __v = false; compat::write(__this, |__t| __t.quick_nav_button_base_mut().temporary_toggled = __v); }
        }
        if compat::read(__this, |__t| __t.quick_nav_button_base().alpha) < 255 {
            { let __v = ((compat::read(__this, |__t| __t.quick_nav_button_base().alpha)).wrapping_add(10)).min(255); compat::write(__this, |__t| __t.quick_nav_button_base_mut().alpha = __v); }
        }
        let mut tab_texture: Identifier = ({ let __f = compat::dispatch(__this, |__t| __t.get_texture()); __f(__this) });
        gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &tab_texture, __this.lock().borrow().quick_nav_button_base().get_x(), __this.lock().borrow().quick_nav_button_base().get_y(), compat::read(__this, |__t| __t.quick_nav_button_base().width), compat::read(__this, |__t| __t.quick_nav_button_base().height), (if compat::read(__this, |__t| __t.quick_nav_button_base().render_in_front) { argb::color_alpha_rgb(compat::read(__this, |__t| __t.quick_nav_button_base().alpha), -1) } else { -1 }));
        let mut y_offset: i32 = (if compat::read(__this, |__t| __t.quick_nav_button_base().index) < 7 { 1 } else { -1 });
        graphics.item(&compat::read(__this, |__t| __t.quick_nav_button_base().icon.clone()), (__this.lock().borrow().quick_nav_button_base().get_x()).wrapping_add(5), ((__this.lock().borrow().quick_nav_button_base().get_y()).wrapping_add(8)).wrapping_add(y_offset));
        // java: this.handleCursor(graphics); - the port asks it of the widget after the draw (`ModWidget::handle_cursor`, below)
    }

    /// java: private Identifier getTexture()
    pub fn get_texture_body(__this: &compat::Shared<dyn QuickNavButton>) -> Identifier {
        let render_in_front = compat::read(__this, |__t| __t.quick_nav_button_base().render_in_front);
        let index = compat::read(__this, |__t| __t.quick_nav_button_base().index);
        let textures = if render_in_front { &TAB_TEXTURES_SELECTED } else { &TAB_TEXTURES };
        let mut texture: Option<compat::Shared<dyn FallbackedTexture<Identifier>>> = textures.get()[(index) as usize].clone();
        if let Some(texture) = &texture {
            return fallbacked_texture_get(texture);
        }
        // Construct the texture identifier based on the index and toggled state
        let is_top_tab = ({ let __f = compat::dispatch(__this, |__t| __t.is_top_tab()); __f(__this) });
        let __texture = crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::of_gui_sprite(
            &skyblocker_mod::id(&format!("quick_nav/tab_{}_{}_{}", (if is_top_tab { "top" } else { "bottom" }), (if render_in_front { "selected" } else { "unselected" }), (index.wrapping_rem(7)).wrapping_add(1))),
            &Identifier::with_default_namespace(&format!("container/creative_inventory/tab_{}_{}_{}", (if is_top_tab { "top" } else { "bottom" }), (if render_in_front { "selected" } else { "unselected" }), (index.wrapping_rem(7)).wrapping_add(1))),
        );
        textures.get_mut()[(index) as usize] = Some(__texture.clone());
        return fallbacked_texture_get(&__texture);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn QuickNavButton>, builder: &dyn NarrationElementOutput) {
    }

}

impl QuickNavButton for QuickNavButtonBase {
    fn quick_nav_button_base(&self) -> &QuickNavButtonBase {
        &self
    }
    fn quick_nav_button_base_mut(&mut self) -> &mut QuickNavButtonBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// `FallbackedTexture<Identifier>.get()` (`extends Supplier<T>`): the port's `FallbackedTexture` trait has no `get`
/// (utils/render/texture, another batch's file), so it is read off the one implementation of the `Identifier` kind.
fn fallbacked_texture_get(texture: &compat::Shared<dyn FallbackedTexture<Identifier>>) -> Identifier {
    compat::read(texture, |t| {
        t.as_any()
            .downcast_ref::<crate::de::hysky::skyblocker::utils::render::texture::identifier_texture::IdentifierTexture>()
            .map(|t| t.current_texture.clone())
            .expect("FallbackedTexture<Identifier> is an IdentifierTexture")
    })
}

/// QuickNavButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call - for
/// every class of the hierarchy (the mixin hands the game `List<QuickNavButton>`, whose elements are of any class)
impl compat::WidgetHalf for dyn QuickNavButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.quick_nav_button_base_mut().base
    }
}

impl QuickNavButtonBase {
    /// `screen.addWidget(quickNavButton)` / `addRenderableWidget(..)`: the game's widget with the button attached.
    pub fn widget(button: compat::Shared<dyn QuickNavButton>) -> AbstractWidget {
        compat::WidgetAdapter::widget(button)
    }
}

/// QuickNavButton as the game's widget: `QuickNavButtonBase::widget(handle)` where Java hands it on. Each call
/// dispatches to the object's class (QuickNavButton or QuickNavConfirmationButton).
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<dyn QuickNavButton> {
    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_widget_render_state()); __f(__h, &context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.update_widget_narration()); __f(__h, output) }))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_click()); __f(__h, event, double_click) }))
    }

    /// java: `this.handleCursor(graphics)` at the end of `extractWidgetRenderState` - `AbstractWidget.handleCursor`'s body
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }
}
