//! Ported from de.hysky.skyblocker.skyblock.auction.widgets.CategoryTabWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::slot_click_handler::SlotClickHandler;
use crate::de::hysky::skyblocker::utils::render::gui::side_tab_button_widget::SideTabButtonWidget;
use crate::de::hysky::skyblocker::utils::render::gui::side_tab_button_widget::SideTabButtonWidgetBase;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;

/// java: @SuppressWarnings("unchecked") private static final @Nullable FallbackedTexture<WidgetSprites>[] SPRITES = new FallbackedTexture[6];
pub static SPRITES: compat::MainThread<Option<Vec<compat::Shared<dyn FallbackedTexture<WidgetSprites>>>>> = compat::MainThread::new(|| Some((0..(6) as usize).map(|_| Default::default()).collect::<Vec<_>>()));

/// java: public class CategoryTabWidget extends SideTabButtonWidget
pub struct CategoryTabWidget {
    /// java: extends SideTabButtonWidget
    pub base: SideTabButtonWidgetBase,
    /// java: private final SlotClickHandler slotClick;
    pub slot_click: compat::Shared<dyn SlotClickHandler>,
    /// java: private int slotId = -1;
    pub slot_id: i32,
}

impl std::ops::Deref for CategoryTabWidget {
    type Target = SideTabButtonWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CategoryTabWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CategoryTabWidget {
    /// java: public CategoryTabWidget(ItemStack icon, SlotClickHandler slotClick, int id)
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_click: compat::Shared<dyn SlotClickHandler>, mut id: i32) -> compat::Shared<Self> {
        let __init_slot_click = slot_click.clone();
        let (_, __this) = SideTabButtonWidgetBase::new_in(0, 0, false, CategoryTabWidget::get_sprites(id), icon.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                slot_click: __init_slot_click,
                slot_id: -1,
            });
            ((__h.clone() as compat::Shared<dyn SideTabButtonWidget>), __h)
        });
        __this
    }

    /// java: private static WidgetSprites getSprites(int index)
    pub fn get_sprites(mut index: i32) -> WidgetSprites {
        let mut sprite: Option<compat::Shared<dyn FallbackedTexture<WidgetSprites>>> = Some(SPRITES.get_mut().as_ref().unwrap()[(index) as usize].clone());
        let Some(mut sprite) = sprite.clone() else {
            // TODO(fabric2seer): an assignment used as a value
            //   java: return (SPRITES[index] = FallbackedTexture.ofWidgetSprites(
            //   java: 				new WidgetSprites(
            //   java: 						SkyblockerMod.id("auctions_gui/category_tab_" + (index + 1)),
            //   java: 						SkyblockerMod.id("auctions_gui/category_tab_selected_" + (index + 1))
            //   java: 				),
            //   java: 				RecipeBookTabButton.SPRITES)).get();
        };
        // TODO(fabric2seer): `get` is not a method of FallbackedTexture
        //   java: return sprite.get();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public void setSlotId(int slotId)
    pub fn set_slot_id(__this: &compat::Shared<Self>, mut slot_id: i32) {
        { let __v = slot_id; compat::write(__this, |__t| __t.slot_id = __v); }
    }

}

impl SideTabButtonWidget for CategoryTabWidget {
    fn side_tab_button_widget_base(&self) -> &SideTabButtonWidgetBase {
        &self.base
    }
    fn side_tab_button_widget_base_mut(&mut self) -> &mut SideTabButtonWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_contents(&self) -> fn(&compat::Shared<dyn SideTabButtonWidget>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        CategoryTabWidget::extract_contents_dyn
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn on_click(&self) -> fn(&compat::Shared<dyn SideTabButtonWidget>, &MouseButtonEvent, bool) {
        CategoryTabWidget::on_click_dyn
    }

}

impl CategoryTabWidget {
    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn SideTabButtonWidget>::extract_contents_body(&compat::view(__this, |a| a as compat::Shared<dyn SideTabButtonWidget>), graphics, mouse_x, mouse_y, a);
        if { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } {
            // TODO(fabric2seer): the mod keeps the game's `LocalPlayer` (Java holds a reference to the live object); the port's `LocalPlayer` is not a value to copy and the port lends the mod no handle to it
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut compat::font(), mcrust::mcrust::modding::game::with_player(|player| { let (__q0, __q1, __q2,) = (&mcrust::net::minecraft::world::item::item::TooltipContext::empty(), Some(player.clone()), mcrust::net::minecraft::world::item::tooltip_flag::NORMAL,); __this.lock().borrow().icon.as_ref().unwrap().get_tooltip_lines(__q0, __q1, __q2) }).unwrap(), mouse_x, mouse_y);
        }
    }

    /// CategoryTabWidget.extractContents as the dispatch of SideTabButtonWidget.extractContents
    pub fn extract_contents_dyn(__this: &compat::Shared<dyn SideTabButtonWidget>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_contents_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click_impl(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        if compat::read(__this, |__t| __t.selected) || compat::read(__this, |__t| __t.slot_id) == -1 {
            return;
        }
        <dyn SideTabButtonWidget>::on_click_body(&compat::view(__this, |a| a as compat::Shared<dyn SideTabButtonWidget>), click, doubled);
        ({ let __r = &compat::read(__this, |__t| __t.slot_click.clone()); let __f = compat::dispatch(__r, |__t| __t.click_1()); __f(__r, compat::read(__this, |__t| __t.slot_id)) });
    }

    /// CategoryTabWidget.onClick as the dispatch of SideTabButtonWidget.onClick
    pub fn on_click_dyn(__this: &compat::Shared<dyn SideTabButtonWidget>, __p0: &MouseButtonEvent, __p1: bool) {
        Self::on_click_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
