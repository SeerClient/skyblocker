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
pub static TAB_TEXTURES: compat::MainThread<Option<Vec<compat::Shared<dyn FallbackedTexture<Identifier>>>>> = compat::MainThread::new(|| Some((0..(14) as usize).map(|_| Default::default()).collect::<Vec<_>>()));
/// java: @SuppressWarnings("unchecked") private static final @Nullable FallbackedTexture<Identifier>[] TAB_TEXTURES_SELECTED = new FallbackedTexture[14];
pub static TAB_TEXTURES_SELECTED: compat::MainThread<Option<Vec<compat::Shared<dyn FallbackedTexture<Identifier>>>>> = compat::MainThread::new(|| Some((0..(14) as usize).map(|_| Default::default()).collect::<Vec<_>>()));

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
        let mut tip: Tooltip;
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.fromJson(..)` on a Gson
            //   java: setTooltip(tip = Tooltip.create(ComponentSerialization.CODEC.decode(JsonOps.INSTANCE, SkyblockerMod.GSON.fromJson(tooltip, JsonElement.class)).getOrThrow().getFirst()));
        }));
        if let Err(_) = __result {
            { let (__q0,) = ({ tip = Tooltip::create(component::literal(&tooltip).get_string()); tip.clone() },); __this.lock().borrow_mut().quick_nav_button_base_mut().set_tooltip(__q0) };
        }
        { let __v = Some(tip.clone()); compat::write(&__this, |__t| __t.quick_nav_button_base_mut().tooltip = __v); }
        // TODO(fabric2seer): `AbstractWidget.setTooltipDelay` has no Rust method in the port (looked for `set_tooltip_delay` in AbstractWidget and its supertypes)
        // rust: let __h0 = &std::time::Duration::from_millis((100) as u64);
        // rust: __this.lock().borrow().quick_nav_button_base().set_tooltip_delay(__h0);
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
        if let Some(handled_screen) = ScreenUtils::get_underlying_screen().unwrap().as_any().downcast_ref::<AbstractContainerScreen>() {
            let mut accessible_screen = handled_screen;
            let mut x: i32 = accessible_screen.left_pos;
            let mut y: i32 = accessible_screen.top_pos;
            let mut h: i32 = accessible_screen.image_height;
            if handled_screen.as_any().is::<ContainerScreen>() {
                h -= 1;
            }
            let mut w: i32 = accessible_screen.image_width;
            let __r0 = compat::read(__this, |__t| __t.quick_nav_button_base().index);
            { let (__q0,) = (((x.wrapping_add(((__r0).wrapping_rem(7)).wrapping_mul(25))).wrapping_add(w.wrapping_div(2))).wrapping_sub(176i32.wrapping_div(2)),); __this.lock().borrow_mut().quick_nav_button_base_mut().set_x(__q0) };
            let __r0 = compat::read(__this, |__t| __t.quick_nav_button_base().index);
            { let (__q0,) = ((if __r0 < 7 { y.wrapping_sub(28) } else { (y.wrapping_add(h)).wrapping_sub(4) }),); __this.lock().borrow_mut().quick_nav_button_base_mut().set_y(__q0) };
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click_body(__this: &compat::Shared<dyn QuickNavButton>, click: &MouseButtonEvent, mut doubled: bool) {
        if !(compat::read(__this, |__t| __t.quick_nav_button_base().temporary_toggled)) {
            { let __v = true; compat::write(__this, |__t| __t.quick_nav_button_base_mut().temporary_toggled = __v); }
            { let __v = compat::millis(); compat::write(__this, |__t| __t.quick_nav_button_base_mut().toggle_time = __v); }
            if compat::read(__this, |__t| __t.quick_nav_button_base().command.clone()).is_none() || __this.lock().borrow().quick_nav_button_base().command.as_ref().unwrap().is_empty() {
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append((component::literal(&format!("Quick Nav button index {} has no command!", (compat::read(__this, |__t| __t.quick_nav_button_base().index)).wrapping_add(1)))).with_style_format(ChatFormatting::Red)).to_string()); });
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
        gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &tab_texture, __this.lock().borrow().quick_nav_button_base().get_x(), __this.lock().borrow().quick_nav_button_base().get_y(), compat::read(__this, |__t| __t.quick_nav_button_base().width), compat::read(__this, |__t| __t.quick_nav_button_base().height), (((if compat::read(__this, |__t| __t.quick_nav_button_base().render_in_front) { (argb::color_alpha_rgb(compat::read(__this, |__t| __t.quick_nav_button_base().alpha), -1)) as f32 } else { (-1) as f32 })) as f32) as f32);
        let mut y_offset: i32 = (if compat::read(__this, |__t| __t.quick_nav_button_base().index) < 7 { 1 } else { -1 });
        graphics.item(&compat::read(__this, |__t| __t.quick_nav_button_base().icon.clone()), (__this.lock().borrow().quick_nav_button_base().get_x()).wrapping_add(5), ((__this.lock().borrow().quick_nav_button_base().get_y()).wrapping_add(8)).wrapping_add(y_offset));
        __this.lock().borrow().quick_nav_button_base().handle_cursor(graphics);
    }

    /// java: private Identifier getTexture()
    pub fn get_texture_body(__this: &compat::Shared<dyn QuickNavButton>) -> Identifier {
        let mut textures = (if compat::read(__this, |__t| __t.quick_nav_button_base().render_in_front) { TAB_TEXTURES_SELECTED.get_mut().clone() } else { TAB_TEXTURES.get_mut().clone() });
        let mut texture: Option<compat::Shared<dyn FallbackedTexture<Identifier>>> = Some(textures[(compat::read(__this, |__t| __t.quick_nav_button_base().index)) as usize].clone());
        if let Some(texture) = &texture {
            // TODO(fabric2seer): `get` is not a method of FallbackedTexture
            //   java: return texture.get();
        }
        // TODO(fabric2seer): an assignment used as a value
        //   java: return (textures[index] = FallbackedTexture.ofGuiSprite(
        //   java: 				SkyblockerMod.id("quick_nav/tab_" + (isTopTab() ? "top" : "bottom") + "_" + (renderInFront ? "selected" : "unselected") + "_" + (index % 7 + 1)),
        //   java: 				Identifier.withDefaultNamespace("container/creative_inventory/tab_" + (isTopTab() ? "top" : "bottom") + "_" + (renderInFront ? "selected" : "unselected") + "_" + (index % 7 + 1))
        //   java: 		)).get();
        todo!("fabric2seer: the value of this function is in a TODO above")
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
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
