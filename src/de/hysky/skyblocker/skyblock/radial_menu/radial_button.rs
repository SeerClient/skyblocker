//! Ported from de.hysky.skyblocker.skyblock.radialMenu.RadialButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::backpack_preview::BackpackPreview;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::org::joml::vector2f::Vector2f;
use mcrust::org::joml::vector2i::Vector2i;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class RadialButton implements Renderable, GuiEventListener, LayoutElement, NarratableEntry
#[derive(Clone)]
pub struct RadialButton {
    /// java: private final float startAngle;
    pub start_angle: f32,
    /// java: private final float arcLength;
    pub arc_length: f32,
    /// java: private final float internalRadius;
    pub internal_radius: f32,
    /// java: private final float externalRadius;
    pub external_radius: f32,
    /// java: private final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final BooleanSupplier getHovered;
    pub get_hovered: Arc<dyn Fn() -> bool>,
    /// java: private final int linkedSlot;
    pub linked_slot: i32,
}

impl RadialButton {
    /// java: public RadialButton(float startAngle, float arcLength, float internalRadius, float externalRadius, ItemStack icon, BooleanSupplier getHovered, int linkedSlot)
    pub fn new(mut start_angle: f32, mut arc_length: f32, mut internal_radius: f32, mut external_radius: f32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, get_hovered: Arc<dyn Fn() -> bool>, mut linked_slot: i32) -> Self {
        let __init_start_angle = ((((start_angle as f64) - (std::f64::consts::PI / (2 as f64)))) as f32);
        let __init_arc_length = arc_length;
        let __init_internal_radius = internal_radius;
        let __init_external_radius = external_radius;
        let __init_icon = icon.clone();
        let __init_get_hovered = get_hovered.clone();
        let __init_linked_slot = linked_slot;
        let mut this = Self {
            start_angle: __init_start_angle,
            arc_length: __init_arc_length,
            internal_radius: __init_internal_radius,
            external_radius: __init_external_radius,
            icon: __init_icon,
            get_hovered: __init_get_hovered,
            linked_slot: __init_linked_slot,
        };
        this
    }

    /// java: private static Vector2f getPos(Vector2i center, float angle, float radius)
    pub fn get_pos(center: &Vector2i, mut angle: f32, mut radius: f32) -> Vector2f {
        return Vector2f::new_xy(((((center.x as f64) + ((radius as f64) * (angle as f64).cos()))) as f32), ((((center.y as f64) + ((radius as f64) * (angle as f64).sin()))) as f32));
    }

    /// java: public String getName()
    pub fn get_name(&self) -> String {
        let Some(mut custom_name) = self.icon.get_custom_name() else {
            return "null".to_string();
        };
        return self.icon.get_custom_name().unwrap().get_string();
    }

    /// java: protected int getLinkedSlot()
    pub fn get_linked_slot(&self) -> i32 {
        return self.linked_slot;
    }

    /// java: @Override public ScreenRectangle getRectangle()
    pub fn get_rectangle(&self) -> ScreenRectangle {
        // TODO(fabric2seer): `.getRectangle(..)` on a value of unknown type
        //   java: return GuiEventListener.super.getRectangle();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_render_state(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        let mut hovered: bool = (self.get_hovered)();
        let mut color: i32 = (if hovered { 0xFE000000u32 as i32 } else { 0x77000000 });
        let mut internal: f32 = self.internal_radius;
        let mut external: f32 = (if hovered { (self.external_radius + (5 as f32)) } else { self.external_radius });
        let mut center: Vector2i = Vector2i::new((gui_graphics_extractor::gui_width(&*compat::minecraft())).wrapping_div(2), (gui_graphics_extractor::gui_height(&*compat::minecraft())).wrapping_div(2));
        let mut vertices: Vec<Vector2f> = Vec::new();
        vertices.push(RadialButton::get_pos(&center, self.start_angle, internal));
        vertices.push(RadialButton::get_pos(&center, (self.start_angle + (self.arc_length / (2 as f32))), internal));
        vertices.push(RadialButton::get_pos(&center, (self.start_angle + (self.arc_length / (2 as f32))), external));
        vertices.push(RadialButton::get_pos(&center, self.start_angle, external));
        vertices.push(RadialButton::get_pos(&center, (self.start_angle + (self.arc_length / (2 as f32))), internal));
        vertices.push(RadialButton::get_pos(&center, (self.start_angle + self.arc_length), internal));
        vertices.push(RadialButton::get_pos(&center, (self.start_angle + self.arc_length), external));
        vertices.push(RadialButton::get_pos(&center, (self.start_angle + (self.arc_length / (2 as f32))), external));
        GuiHelper::custom_shape(graphics, &vertices, color);
        let mut icon_angle: f32 = (self.start_angle + (self.arc_length / (2 as f32)));
        let mut icon_pos: Vector2f = RadialButton::get_pos(&center, icon_angle, ((internal + external) / (2 as f32)));
        icon_pos.sub((8) as f32, (8) as f32);
        graphics.item(&self.icon, ((icon_pos.x) as i32), ((icon_pos.y) as i32));
        graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &self.icon, ((icon_pos.x) as i32), ((icon_pos.y) as i32));
        SlotTextManager::extract_slot_text_7(graphics, &compat::font(), None, &self.icon, self.linked_slot, ((icon_pos.x) as i32), ((icon_pos.y) as i32));
        if hovered && GuiHelper::has_shift_down() || SkyblockerConfigManager::get().ui_and_visuals.radial_menu.tooltips_without_shift {
            if compat::minecraft().gui.screen().is_some() && compat::minecraft().gui.screen().unwrap().get_title().to_string() == "Storage" {
                BackpackPreview::extract_preview(graphics, &compat::minecraft().gui.screen().unwrap(), self.linked_slot, mouse_x, mouse_y);
            } else {
                // TODO(fabric2seer): `GuiGraphicsExtractor.setTooltipForNextFrame` has no Rust method in the port (looked for `set_tooltip_for_next_frame` in GuiGraphicsExtractor and its supertypes)
                // rust: graphics.set_tooltip_for_next_frame(&compat::font(), &self.icon, mouse_x, mouse_y);
            }
        }
    }

    /// java: @Override public NarrationPriority narrationPriority()
    pub fn narration_priority(&self) -> crate::unresolved::NarrationPriority {
        if self.is_focused() {
            // TODO(fabric2seer): field `FOCUSED` of a value of unknown type
            //   java: return NarratableEntry.NarrationPriority.FOCUSED;
        } else {
            // TODO(fabric2seer): field `HOVERED` of a value of unknown type
            //   java: return this.getHovered.getAsBoolean() ? NarratableEntry.NarrationPriority.HOVERED : NarratableEntry.NarrationPriority.NONE;
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean isActive()
    pub fn is_active(&self) -> bool {
        // TODO(fabric2seer): `.isActive(..)` on a value of unknown type
        //   java: return NarratableEntry.super.isActive();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Collection<? extends NarratableEntry> getNarratables()
    pub fn get_narratables(&self) -> Vec<Box<dyn NarratableEntry>> {
        // TODO(fabric2seer): `.getNarratables(..)` on a value of unknown type
        //   java: return NarratableEntry.super.getNarratables();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    pub fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
        builder.add_text(NarratedElementType::Title, &self.get_name());
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&self, mut x: i32) {
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&self, mut y: i32) {
    }

    /// java: @Override public int getX()
    pub fn get_x(&self) -> i32 {
        return 0;
    }

    /// java: @Override public int getY()
    pub fn get_y(&self) -> i32 {
        return 0;
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        return 0;
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override public void visitWidgets(Consumer<AbstractWidget> consumer)
    pub fn visit_widgets(&self, consumer: Arc<dyn Fn(AbstractWidget)>) {
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused(&self, mut focused: bool) {
    }

    /// java: @Override public boolean isFocused()
    pub fn is_focused(&self) -> bool {
        return false;
    }

}
