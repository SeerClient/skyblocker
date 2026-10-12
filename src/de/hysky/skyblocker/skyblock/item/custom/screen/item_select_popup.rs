//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.ItemSelectPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_inventory_screen::SkyblockInventoryScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::core::non_null_list::NonNullList;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier BACKGROUND_TEXTURE = SkyblockerMod.id("textures/gui/inventory_item_selection.png");
pub static BACKGROUND_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/inventory_item_selection.png"));
/// java: private static final int TEXTURE_WIDTH = 176;
pub const TEXTURE_WIDTH: i32 = 176;
/// java: private static final int TEXTURE_HEIGHT = 132;
pub const TEXTURE_HEIGHT: i32 = 132;
/// java: private static final Identifier SLOT_HIGHLIGHT_BACK_TEXTURE = Identifier.withDefaultNamespace("container/slot_highlight_back");
pub static ITEM_WIDGET_SLOT_HIGHLIGHT_BACK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/slot_highlight_back"));
/// java: private static final Identifier SLOT_HIGHLIGHT_FRONT_TEXTURE = Identifier.withDefaultNamespace("container/slot_highlight_front");
pub static ITEM_WIDGET_SLOT_HIGHLIGHT_FRONT_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/slot_highlight_front"));
/// java: private static final ItemStack BARRIER = new ItemStack(Items.BARRIER);
pub static ITEM_WIDGET_BARRIER: compat::MainThread<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::MainThread::new(|| mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("barrier")));

/// java: public class ItemSelectPopup extends AbstractPopupScreen
pub struct ItemSelectPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final Consumer<ItemStack> callback;
    pub callback: Arc<dyn Fn(mcrust::net::minecraft::world::item::item_stack::ItemStack)>,
    /// java: private final GridLayout mainGrid = new GridLayout();
    pub main_grid: GridLayout,
    /// java: private final LinearLayout equipmentLayout = LinearLayout.horizontal();
    pub equipment_layout: LinearLayout,
    /// java: private int x, y;
    pub x: i32,
    /// java: private int x, y;
    pub y: i32,
}

impl std::ops::Deref for ItemSelectPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemSelectPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemSelectPopup {
    /// java: protected ItemSelectPopup(Screen backgroundScreen, Consumer<ItemStack> callback)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, callback: Arc<dyn Fn(mcrust::net::minecraft::world::item::item_stack::ItemStack)>) -> compat::Shared<Self> {
        // java: super(Component.literal("Select Item"), backgroundScreen);
        let (_, __this) = AbstractPopupScreenBase::new_in(component::literal("Select Item"), background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                callback,
                main_grid: GridLayout::new_default(),
                equipment_layout: LinearLayout::horizontal(),
                x: 0,
                y: 0,
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        __this
    }

}

impl AbstractPopupScreen for ItemSelectPopup {
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
        ItemSelectPopup::init_dyn
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        ItemSelectPopup::reposition_elements_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        ItemSelectPopup::extract_background_dyn
    }

}

impl ItemSelectPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        let mut adder: RowHelper = __this.lock().borrow().main_grid.create_row_helper(9);
        // TODO(fabric2seer): the mod keeps the game's `NonNullList` (Java holds a reference to the live object); the port's `NonNullList` is not a value to copy and the port lends the mod no handle to it
        // rust: let mut stacks: NonNullList<mcrust::net::minecraft::world::item::item_stack::ItemStack> = mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items().clone()).unwrap();
        let mut stacks: NonNullList<mcrust::net::minecraft::world::item::item_stack::ItemStack> = todo!("fabric2seer: the initialiser of `stacks` (TODO above)");
        for i in mcrust::net::minecraft::world::entity::player::inventory::SELECTION_SIZE..(stacks.len() as i32).wrapping_add(mcrust::net::minecraft::world::entity::player::inventory::SELECTION_SIZE) {
            let stack = &stacks[(i.wrapping_rem(stacks.len() as i32)) as usize];
            if stack.is_empty() {
                // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
                // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
                // rust: adder.add_child(&mut SpacerElement::of(18, 18)).unwrap();
            } else {
                // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
                // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
                // rust: adder.add_child(&mut ItemWidget::new(stack.clone())).unwrap();
            }
        }
        // TODO(fabric2seer): `getType` on a Object
        //   java: Arrays.stream(EquipmentSlot.values()).filter(e -> e.getType() == EquipmentSlot.Type.HUMANOID_ARMOR).toList().reversed().forEach(slot -> {
        //   java: 			ItemStack stack = player.getItemBySlot(slot);
        //   java: 			if (stack.isEmpty()) equipmentLayout.addChild(new SpacerElement(18, 18));
        //   java: 			else equipmentLayout.addChild(new ItemWidget(stack));
        //   java: 		});
        { let (__q0,) = ((SpacerElement::of(18, 18)).as_layout_element(),); __this.lock().borrow_mut().equipment_layout.add_child(__q0) };
        for stack in SkyblockInventoryScreen::get_current_equipment_set() {
            if stack.is_empty() {
                { let (__q0,) = ((SpacerElement::of(18, 18)).as_layout_element(),); __this.lock().borrow_mut().equipment_layout.add_child(__q0) };
            } else {
                { let (__q0,) = ({ let __w = compat::WidgetAdapter::widget(ItemWidget::new(stack.clone())); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().equipment_layout.add_child(__q0) };
            }
        }
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().main_grid.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        { __this.lock().borrow_mut().main_grid.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().main_grid.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().equipment_layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        { __this.lock().borrow_mut().equipment_layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().equipment_layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        ({ let __f = compat::dispatch(__this, |__t| __t.reposition_elements()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
    }

    /// ItemSelectPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::reposition_elements_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        { let __v = ((compat::read(__this, |__t| __t.width)).wrapping_sub(TEXTURE_WIDTH)).wrapping_div(2); compat::write(__this, |__t| __t.x = __v); }
        { let __v = ((compat::read(__this, |__t| __t.height)).wrapping_sub(TEXTURE_HEIGHT)).wrapping_div(2); compat::write(__this, |__t| __t.y = __v); }
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `GridLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: let __r0 = compat::read(__this, |__t| __t.x);
        // rust: let __r1 = compat::read(__this, |__t| __t.y);
        // rust: { let (__q0, __q1,) = ((__r0).wrapping_add(7), (__r1).wrapping_add(53),); __this.lock().borrow_mut().main_grid.set_position(__q0, __q1) };
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: let __r0 = compat::read(__this, |__t| __t.x);
        // rust: let __r1 = compat::read(__this, |__t| __t.y);
        // rust: { let (__q0, __q1,) = ((__r0).wrapping_add(7), (__r1).wrapping_add(17),); __this.lock().borrow_mut().equipment_layout.set_position(__q0, __q1) };
    }

    /// ItemSelectPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get_mut(), compat::read(__this, |__t| __t.x), compat::read(__this, |__t| __t.y), ((0) as f32) as f32, ((0) as f32) as f32, TEXTURE_WIDTH, TEXTURE_HEIGHT, TEXTURE_WIDTH, TEXTURE_HEIGHT);
    }

    /// ItemSelectPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// ItemSelectPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ItemSelectPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ItemSelectPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ItemSelectPopup> {
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

/// java: private class ItemWidget extends AbstractWidget
#[derive(Clone)]
pub struct ItemWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ItemStack item;
    pub item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final boolean selectable;
    pub selectable: bool,
}

impl std::ops::Deref for ItemWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemWidget {
    /// java: private ItemWidget(ItemStack item)
    pub fn new(mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        let __init_selectable = !(item.get_uuid().is_empty());
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 18, 18, (&compat::component_of(item.get_hover_name()).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            item: None,
            selectable: __init_selectable,
        });
        let mut message: MutableComponent = (if compat::read(&__this, |__t| __t.selectable) { compat::component_of(__this.lock().borrow().get_message_component()) } else { { let (__q0,) = (component::translatable("skyblocker.customization.item.cannotCustomize").with_style_format(ChatFormatting::Red),); (compat::component_of(__this.lock().borrow().get_message_component()).copy().append_str("\n")).append(__q0) } });
        { let (__q0,) = (Tooltip::create(message.get_string()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        { let __v = Some(item.clone()); compat::write(&__this, |__t| __t.item = __v); }
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut x: i32 = (__this.lock().borrow().get_x()).wrapping_add(1);
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_add(1);
        if __this.lock().borrow().is_hovered() {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), ITEM_WIDGET_SLOT_HIGHLIGHT_BACK_TEXTURE.get_mut(), (__this.lock().borrow().get_x()).wrapping_sub(3), (__this.lock().borrow().get_y()).wrapping_sub(3), 24, 24);
        }
        graphics.item(&compat::read(__this, |__t| __t.item.clone()).unwrap(), x, y);
        if __this.lock().borrow().is_hovered() {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), ITEM_WIDGET_SLOT_HIGHLIGHT_FRONT_TEXTURE.get_mut(), (__this.lock().borrow().get_x()).wrapping_sub(3), (__this.lock().borrow().get_y()).wrapping_sub(3), 24, 24);
        }
        if !(compat::read(__this, |__t| __t.selectable)) {
            graphics.item(ITEM_WIDGET_BARRIER.get_mut(), x, y);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        if compat::read(__this, |__t| __t.selectable) {
            // TODO(fabric2seer): `callback` is a field of the enclosing instance (ItemSelectPopup.this) - an inner class has no outer reference in Rust
            //   java: callback.accept(item);
            // TODO(fabric2seer): `onClose(..)` is a method of the enclosing instance (ItemSelectPopup.this) - an inner class has no outer reference in Rust
            //   java: onClose();
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// ItemWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ItemWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ItemWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ItemWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ItemWidget::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ItemWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ItemWidget::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
