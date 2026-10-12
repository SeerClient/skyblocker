//! Ported from de.hysky.skyblocker.utils.render.gui.SideTabButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class SideTabButtonWidget extends ImageButton
pub struct SideTabButtonWidgetBase {
    /// java: extends net.minecraft.client.gui.components.ImageButton
    pub base: AbstractWidget,
    /// java: protected ItemStack icon;
    pub icon: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: protected boolean selected = false;
    pub selected: bool,
}

impl std::ops::Deref for SideTabButtonWidgetBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SideTabButtonWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SideTabButtonWidgetBase {
    /// java: public SideTabButtonWidget(int x, int y, boolean toggled, WidgetSprites sprites, ItemStack icon) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut x: i32, mut y: i32, mut toggled: bool, mut sprites: WidgetSprites, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SideTabButtonWidget>, __R)) -> (compat::Shared<dyn SideTabButtonWidget>, __R) {
        let __init_icon = Some(icon.copy());
        // TODO(fabric2seer): `new ImageButton(..)`: the port has no `ImageButton::new`
        //   java: super(x, y, 35, 27, sprites, _ -> {});
        let (__this, __r): (compat::Shared<dyn SideTabButtonWidget>, __R) = todo!("constructor");
        (__this, __r)
    }

    /// java: public SideTabButtonWidget(int x, int y, boolean toggled, WidgetSprites sprites, ItemStack icon)
    pub fn new(mut x: i32, mut y: i32, mut toggled: bool, mut sprites: WidgetSprites, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        Self::new_in(x, y, toggled, sprites, icon, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SideTabButtonWidget>), __h)
        }).1
    }

    /// java: public SideTabButtonWidget(int x, int y, boolean toggled, ItemStack icon) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_4_in<__R>(mut x: i32, mut y: i32, mut toggled: bool, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SideTabButtonWidget>, __R)) -> (compat::Shared<dyn SideTabButtonWidget>, __R) {
        // TODO(fabric2seer): static `RecipeBookTabButton.SPRITES` has no Rust counterpart in the port's RecipeBookTabButton file
        //   java: this(x, y, toggled, RecipeBookTabButton.SPRITES, icon);
        let (__this, __r): (compat::Shared<dyn SideTabButtonWidget>, __R) = todo!("constructor");
        (__this, __r)
    }

    /// java: public SideTabButtonWidget(int x, int y, boolean toggled, ItemStack icon)
    pub fn new_4(mut x: i32, mut y: i32, mut toggled: bool, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        Self::new_4_in(x, y, toggled, icon, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SideTabButtonWidget>), __h)
        }).1
    }

}

/// java: the virtual methods of SideTabButtonWidget (implemented by SideTabButtonWidgetBase and every subclass)
pub trait SideTabButtonWidget: compat::DynAny {
    fn side_tab_button_widget_base(&self) -> &SideTabButtonWidgetBase;
    fn side_tab_button_widget_base_mut(&mut self) -> &mut SideTabButtonWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public void setIcon(ItemStack icon) (dispatch: the body for this class)
    fn set_icon(&self) -> fn(&compat::Shared<dyn SideTabButtonWidget>, &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        <dyn SideTabButtonWidget>::set_icon_body
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_contents(&self) -> fn(&compat::Shared<dyn SideTabButtonWidget>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn SideTabButtonWidget>::extract_contents_body
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn on_click(&self) -> fn(&compat::Shared<dyn SideTabButtonWidget>, &MouseButtonEvent, bool) {
        <dyn SideTabButtonWidget>::on_click_body
    }

    /// java: public void select() (dispatch: the body for this class)
    fn select(&self) -> fn(&compat::Shared<dyn SideTabButtonWidget>) {
        <dyn SideTabButtonWidget>::select_body
    }

    /// java: public void unselect() (dispatch: the body for this class)
    fn unselect(&self) -> fn(&compat::Shared<dyn SideTabButtonWidget>) {
        <dyn SideTabButtonWidget>::unselect_body
    }

}

impl dyn SideTabButtonWidget {
    /// java: public void setIcon(ItemStack icon)
    pub fn set_icon_body(__this: &compat::Shared<dyn SideTabButtonWidget>, icon: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        { let __v = Some(icon.copy()); compat::write(__this, |__t| __t.side_tab_button_widget_base_mut().icon = __v); }
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_contents_body(__this: &compat::Shared<dyn SideTabButtonWidget>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // TODO(fabric2seer): `ImageButton.sprites` has no Rust field in the port (searched net.minecraft.client.gui.components.ImageButton and its supertypes)
        // rust: let identifier = { let (__q0, __q1,) = (true, compat::read(__this, |__t| __t.side_tab_button_widget_base().selected),); __this.lock().borrow().side_tab_button_widget_base().sprites.get(__q0, __q1) };
        let mut identifier: Identifier = todo!("fabric2seer: the initialiser of `identifier` (TODO above)");
        let mut x: i32 = __this.lock().borrow().side_tab_button_widget_base().get_x();
        if compat::read(__this, |__t| __t.side_tab_button_widget_base().selected) {
            x = x.wrapping_sub(2);
        }
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &identifier, x, __this.lock().borrow().side_tab_button_widget_base().get_y(), compat::read(__this, |__t| __t.side_tab_button_widget_base().width), compat::read(__this, |__t| __t.side_tab_button_widget_base().height));
        graphics.item(&compat::read(__this, |__t| __t.side_tab_button_widget_base().icon.clone()).unwrap(), x.wrapping_add(9), (__this.lock().borrow().side_tab_button_widget_base().get_y()).wrapping_add(5));
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click_body(__this: &compat::Shared<dyn SideTabButtonWidget>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `ImageButton.onClick` has no Rust method in the port (looked for `on_click` in ImageButton and its supertypes)
        // rust: __this.lock().borrow_mut().side_tab_button_widget_base_mut().base.on_click(click, doubled);
        if !(compat::read(__this, |__t| __t.side_tab_button_widget_base().selected)) {
            { let __v = true; compat::write(__this, |__t| __t.side_tab_button_widget_base_mut().selected = __v); }
        }
    }

    /// java: public void select()
    pub fn select_body(__this: &compat::Shared<dyn SideTabButtonWidget>) {
        { let __v = true; compat::write(__this, |__t| __t.side_tab_button_widget_base_mut().selected = __v); }
    }

    /// java: public void unselect()
    pub fn unselect_body(__this: &compat::Shared<dyn SideTabButtonWidget>) {
        { let __v = false; compat::write(__this, |__t| __t.side_tab_button_widget_base_mut().selected = __v); }
    }

}

impl SideTabButtonWidget for SideTabButtonWidgetBase {
    fn side_tab_button_widget_base(&self) -> &SideTabButtonWidgetBase {
        &self
    }
    fn side_tab_button_widget_base_mut(&mut self) -> &mut SideTabButtonWidgetBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
