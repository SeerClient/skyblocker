//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.SkyblockRecipeResultButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int SIZE = 25;
pub const SIZE: i32 = 25;
/// java: private static final int ITEM_OFFSET = 4;
pub const ITEM_OFFSET: i32 = 4;

/// java: public class SkyblockRecipeResultButton extends AbstractWidget
#[derive(Clone)]
pub struct SkyblockRecipeResultButtonBase {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private ItemStack itemStack = ItemStack.EMPTY;
    pub item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack,
}

impl std::ops::Deref for SkyblockRecipeResultButtonBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockRecipeResultButtonBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockRecipeResultButtonBase {
    /// java: protected SkyblockRecipeResultButton() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn SkyblockRecipeResultButton>, __R)) -> (compat::Shared<dyn SkyblockRecipeResultButton>, __R) {
        let this = Self {
            base: AbstractWidget::new(0, 0, SIZE, SIZE, (&mcrust::net::minecraft::network::chat::common_components::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            item_stack: mcrust::net::minecraft::world::item::item_stack::empty(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected SkyblockRecipeResultButton()
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SkyblockRecipeResultButton>), __h)
        }).1
    }

    /// java: protected SkyblockRecipeResultButton(int x, int y) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_2_in<__R>(mut x: i32, mut y: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SkyblockRecipeResultButton>, __R)) -> (compat::Shared<dyn SkyblockRecipeResultButton>, __R) {
        let (__this, __r) = Self::new_in(__wrap);
        __this.lock().borrow_mut().skyblock_recipe_result_button_base_mut().set_position(x, y);
        (__this, __r)
    }

    /// java: protected SkyblockRecipeResultButton(int x, int y)
    pub fn new_2(mut x: i32, mut y: i32) -> compat::Shared<Self> {
        Self::new_2_in(x, y, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SkyblockRecipeResultButton>), __h)
        }).1
    }

}

/// java: the virtual methods of SkyblockRecipeResultButton (implemented by SkyblockRecipeResultButtonBase and every subclass)
pub trait SkyblockRecipeResultButton: compat::DynAny {
    fn skyblock_recipe_result_button_base(&self) -> &SkyblockRecipeResultButtonBase;
    fn skyblock_recipe_result_button_base_mut(&mut self) -> &mut SkyblockRecipeResultButtonBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected @Nullable ItemStack getDisplayStack() (dispatch: the body for this class)
    fn get_display_stack(&self) -> fn(&compat::Shared<dyn SkyblockRecipeResultButton>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        <dyn SkyblockRecipeResultButton>::get_display_stack_body
    }

    /// java: protected SkyblockRecipeResultButton setDisplayStack(ItemStack stack) (dispatch: the body for this class)
    fn set_display_stack(&self) -> fn(&compat::Shared<dyn SkyblockRecipeResultButton>, mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<dyn SkyblockRecipeResultButton> {
        <dyn SkyblockRecipeResultButton>::set_display_stack_body
    }

    /// java: protected void clearDisplayStack() (dispatch: the body for this class)
    fn clear_display_stack(&self) -> fn(&compat::Shared<dyn SkyblockRecipeResultButton>) {
        <dyn SkyblockRecipeResultButton>::clear_display_stack_body
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn SkyblockRecipeResultButton>, &GuiGraphicsExtractor, i32, i32, f32) {
        <dyn SkyblockRecipeResultButton>::extract_widget_render_state_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn SkyblockRecipeResultButton>, &dyn NarrationElementOutput) {
        <dyn SkyblockRecipeResultButton>::update_widget_narration_body
    }

}

impl dyn SkyblockRecipeResultButton {
    /// java: protected @Nullable ItemStack getDisplayStack()
    pub fn get_display_stack_body(__this: &compat::Shared<dyn SkyblockRecipeResultButton>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return Some(compat::read(__this, |__t| __t.skyblock_recipe_result_button_base().item_stack.clone()));
    }

    /// java: protected SkyblockRecipeResultButton setDisplayStack(ItemStack stack)
    pub fn set_display_stack_body(__this: &compat::Shared<dyn SkyblockRecipeResultButton>, mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<dyn SkyblockRecipeResultButton> {
        { let __v = !stack.is_empty(); compat::write(__this, |__t| __t.skyblock_recipe_result_button_base_mut().active = __v); }
        { let __v = true; compat::write(__this, |__t| __t.skyblock_recipe_result_button_base_mut().visible = __v); }
        { let __v = stack.clone(); compat::write(__this, |__t| __t.skyblock_recipe_result_button_base_mut().item_stack = __v); }
        return __this.clone();
    }

    /// java: protected void clearDisplayStack()
    pub fn clear_display_stack_body(__this: &compat::Shared<dyn SkyblockRecipeResultButton>) {
        { let __v = false; compat::write(__this, |__t| __t.skyblock_recipe_result_button_base_mut().visible = __v); }
        { let __v = mcrust::net::minecraft::world::item::item_stack::empty(); compat::write(__this, |__t| __t.skyblock_recipe_result_button_base_mut().item_stack = __v); }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn SkyblockRecipeResultButton>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let client = compat::minecraft();
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::recipebook::recipe_button::slot_craftable_sprite(), __this.lock().borrow().skyblock_recipe_result_button_base().get_x(), __this.lock().borrow().skyblock_recipe_result_button_base().get_y(), compat::read(__this, |__t| __t.skyblock_recipe_result_button_base().width), compat::read(__this, |__t| __t.skyblock_recipe_result_button_base().height));
        graphics.fake_item(&compat::read(__this, |__t| __t.skyblock_recipe_result_button_base().item_stack.clone()), (__this.lock().borrow().skyblock_recipe_result_button_base().get_x()).wrapping_add(ITEM_OFFSET), (__this.lock().borrow().skyblock_recipe_result_button_base().get_y()).wrapping_add(ITEM_OFFSET));
        graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &compat::read(__this, |__t| __t.skyblock_recipe_result_button_base().item_stack.clone()), (__this.lock().borrow().skyblock_recipe_result_button_base().get_x()).wrapping_add(ITEM_OFFSET), (__this.lock().borrow().skyblock_recipe_result_button_base().get_y()).wrapping_add(ITEM_OFFSET));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn SkyblockRecipeResultButton>, builder: &dyn NarrationElementOutput) {
    }

}

impl SkyblockRecipeResultButton for SkyblockRecipeResultButtonBase {
    fn skyblock_recipe_result_button_base(&self) -> &SkyblockRecipeResultButtonBase {
        self
    }
    fn skyblock_recipe_result_button_base_mut(&mut self) -> &mut SkyblockRecipeResultButtonBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
