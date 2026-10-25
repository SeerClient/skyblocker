//! Ported from de.hysky.skyblocker.skyblock.auction.widgets.CategoryTabWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::slot_click_handler;
use crate::de::hysky::skyblocker::skyblock::auction::slot_click_handler::SlotClickHandler;
use crate::de::hysky::skyblocker::utils::render::gui::side_tab_button_widget::SideTabButtonWidget;
use crate::de::hysky::skyblocker::utils::render::gui::side_tab_button_widget::SideTabButtonWidgetBase;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;

/// java: @SuppressWarnings("unchecked") private static final @Nullable FallbackedTexture<WidgetSprites>[] SPRITES = new FallbackedTexture[6];
pub static SPRITES: compat::MainThread<Vec<Option<compat::Shared<dyn FallbackedTexture<WidgetSprites>>>>> = compat::MainThread::new(|| vec![None; 6]);

/// java: public class CategoryTabWidget extends SideTabButtonWidget
pub struct CategoryTabWidget {
    /// java: extends SideTabButtonWidget
    pub base: SideTabButtonWidgetBase,
    /// java: private final SlotClickHandler slotClick;
    pub slot_click: SlotClickHandler,
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
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_click: SlotClickHandler, mut id: i32) -> compat::Shared<Self> {
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
        let sprite: Option<compat::Shared<dyn FallbackedTexture<WidgetSprites>>> = SPRITES.get()[index as usize].clone();
        let Some(sprite) = sprite else {
            // java: return (SPRITES[index] = FallbackedTexture.ofWidgetSprites(new WidgetSprites(..), RecipeBookTabButton.SPRITES)).get();
            let created = fallbacked_texture::of_widget_sprites(
                &WidgetSprites::with_focused(
                    crate::entry::skyblocker_mod::id(&format!("auctions_gui/category_tab_{}", index.wrapping_add(1))),
                    crate::entry::skyblocker_mod::id(&format!("auctions_gui/category_tab_selected_{}", index.wrapping_add(1))),
                ),
                &recipe_book_tab_button_sprites(),
            );
            SPRITES.get_mut()[index as usize] = Some(created.clone());
            return compat::read(&created, |__t| __t.get());
        };
        // java: return sprite.get();
        return compat::read(&sprite, |__t| __t.get())
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
            // java: graphics.setComponentTooltipForNextFrame(Minecraft.getInstance().font, icon.getTooltipLines(TooltipContext.EMPTY,
            //   Minecraft.getInstance().player, TooltipFlag.NORMAL), mouseX, mouseY);
            let icon = compat::read(__this, |__t| __t.icon.clone()).expect("NullPointerException: icon");
            let lines = mcrust::mcrust::modding::game::with_player(|player| icon.get_tooltip_lines(&mcrust::net::minecraft::world::item::item::TooltipContext::empty(), Some(&*player as &dyn mcrust::net::minecraft::world::entity::player::player::Player), mcrust::net::minecraft::world::item::tooltip_flag::NORMAL))
                .unwrap_or_else(|| icon.get_tooltip_lines(&mcrust::net::minecraft::world::item::item::TooltipContext::empty(), None, mcrust::net::minecraft::world::item::tooltip_flag::NORMAL));
            compat::batch_s_set_tooltip_for_next_frame(lines);
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
        slot_click_handler::click_1(&compat::read(__this, |__t| __t.slot_click.clone()), compat::read(__this, |__t| __t.slot_id));
    }

    /// CategoryTabWidget.onClick as the dispatch of SideTabButtonWidget.onClick
    pub fn on_click_dyn(__this: &compat::Shared<dyn SideTabButtonWidget>, __p0: &MouseButtonEvent, __p1: bool) {
        Self::on_click_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

/// CategoryTabWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for CategoryTabWidget {
    fn widget_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
        &mut self.base.base
    }
}

/// CategoryTabWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
/// (`AuctionBrowserScreen.init`'s `addWidget`)
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<CategoryTabWidget> {
    /// java: `AbstractButton.extractWidgetRenderState:32-35` - `this.extractContents(graphics, mouseX, mouseY, a)`, which
    /// this class overrides
    fn extract_widget_render_state(&mut self, widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| {
            let __v = compat::view(__h, |a| a as compat::Shared<dyn SideTabButtonWidget>);
            let __f = compat::dispatch(&*__v, |__t| __t.extract_contents());
            __f(&__v, &mut context.graphics, mouse_x, mouse_y, a)
        })
    }

    /// java: `Button.updateWidgetNarration` - `defaultButtonNarrationText(output)`
    fn update_widget_narration(&mut self, widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {
        widget.default_button_narration_text(output);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| {
            let __v = compat::view(__h, |a| a as compat::Shared<dyn SideTabButtonWidget>);
            let __f = compat::dispatch(&*__v, |__t| __t.on_click());
            __f(&__v, event, double_click)
        })
    }
}

/// java: `RecipeBookTabButton.SPRITES` (private static final, widened by the mod) = `new WidgetSprites(
/// Identifier.withDefaultNamespace("recipe_book/tab"), Identifier.withDefaultNamespace("recipe_book/tab_selected"))` -
/// the port keeps its two names as functions
fn recipe_book_tab_button_sprites() -> WidgetSprites {
    use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_tab_button;
    WidgetSprites::with_focused(recipe_book_tab_button::tab_sprite(), recipe_book_tab_button::tab_selected_sprite())
}
