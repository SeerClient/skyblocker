//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.ArmorTab by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::animated_dye_timeline_widget::AnimatedDyeTimelineWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::color_selection_widget::ColorSelectionWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::customize_screen::CustomizeScreen;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::head_selection_widget::HeadSelectionWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::identifier_text_field::IdentifierTextField;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::player_widget::PlayerWidget;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::trim_selection_widget::TrimSelectionWidget;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidget;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::active_text_collector;
use mcrust::net::minecraft::client::gui::active_text_collector::ActiveTextCollector;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tabs::grid_layout_tab::GridLayoutTab;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::player::remote_player::RemotePlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Identifier INNER_SPACE_TEXTURE = SkyblockerMod.id("menu_inner_space");
pub static INNER_SPACE_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_inner_space"));
/// java: private static final int PLAYER_WIDGET_WIDTH = 84;
pub const PLAYER_WIDGET_WIDTH: i32 = 84;
/// java: private static final int PADDING = 10;
pub const PADDING: i32 = 10;
// TODO(fabric2seer): static `EquipmentSlot.Type` has no Rust counterpart in the port's EquipmentSlot file
//   java: private static final EquipmentSlot[] ARMOR_SLOTS = EquipmentSlot.VALUES.stream().filter(slot -> slot.getType() == EquipmentSlot.Type.HUMANOID_ARMOR).toArray(EquipmentSlot[]::new);
/// java: private static final EquipmentSlot[] ARMOR_SLOTS = EquipmentSlot.VALUES.stream().filter(slot -> slot.getType() == EquipmentSlot.Type.HUMANOID_ARMOR).toArray(EquipmentSlot[]::new);
pub static ARMOR_SLOTS: compat::MainThread<Vec<EquipmentSlot>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ARMOR_SLOTS` - see the TODO above"));
/// java: private static final ItemStack BARRIER = new ItemStack(Items.BARRIER);
pub static BARRIER: compat::MainThread<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::MainThread::new(|| mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("barrier")));
/// java: private static final Identifier HOTBAR_TEXTURE = SkyblockerMod.id("armor_customization_screen/mini_hotbar");
pub static PIECE_SELECTION_WIDGET_HOTBAR_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("armor_customization_screen/mini_hotbar"));
/// java: private static final Identifier HOTBAR_SELECTION_TEXTURE = SkyblockerMod.id("hotbar_selection_full");
pub static PIECE_SELECTION_WIDGET_HOTBAR_SELECTION_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("hotbar_selection_full"));

/// java: public class ArmorTab extends GridLayoutTab implements Closeable
pub struct ArmorTab {
    /// java: extends net.minecraft.client.gui.components.tabs.GridLayoutTab
    pub base: GridLayoutTab,
    /// java: private final ItemStack[] armor = new ItemStack[4];
    pub armor: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final CustomizeScreen parent;
    pub parent: compat::Shared<CustomizeScreen>,
    /// java: private int selectedSlot = 0;
    pub selected_slot: i32,
    /// java: private final TrimSelectionWidget trimSelectionWidget;
    pub trim_selection_widget: Option<TrimSelectionWidget>,
    /// java: private final ColorSelectionWidget colorSelectionWidget;
    pub color_selection_widget: Option<ColorSelectionWidget>,
    /// java: private final HeadSelectionWidget headSelectionWidget;
    pub head_selection_widget: Option<compat::Shared<HeadSelectionWidget>>,
    /// java: private final ModelFieldContainer modelFieldContainer;
    pub model_field_container: Option<ModelFieldContainer>,
    /// java: private final boolean nothingCustomizable;
    pub nothing_customizable: bool,
    /// java: private final RemotePlayer player = new RemotePlayer(CLIENT.level, CLIENT.getGameProfile()) { @Override public boolean isInvisibleTo(Player player) { return tru
    pub player: RemotePlayer,
}

impl std::ops::Deref for ArmorTab {
    type Target = GridLayoutTab;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ArmorTab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ArmorTab {
    /// java: public ArmorTab(CustomizeScreen parent)
    pub fn new(mut parent: compat::Shared<CustomizeScreen>) -> Self {
        let __init_parent = parent.clone();
        // TODO(fabric2seer): the initialiser of `player`: `new RemotePlayer(..) {..}` (an anonymous class with state or several methods)
        //   java: private final RemotePlayer player = new RemotePlayer(CLIENT.level, CLIENT.getGameProfile()) {
        //   java: 		@Override
        //   java: 		public boolean isInvisibleTo(Player player) {
        //   java: 			return true;
        //   java: 		}
        //   java:
        //   java: 		@Override
        //   java: 		public void onEquipItem(EquipmentSlot slot, ItemStack oldStack, ItemStack newStack) {}
        //   java:
        //   java: 		@Override
        //   java: 		public boolean isModelPartShown(PlayerModelPart modelPart) {
        //   java: 			return modelPart != PlayerModelPart.CAPE && CLIENT.options.isModelPartEnabled(modelPart);
        //   java: 		}
        //   java:
        //   java: 		@Override
        //   java: 		public int getId() {
        //   java: 			return EntityUtils.PLACEHOLDER_ID;
        //   java: 		}
        //   java: 	};
        let mut this = Self {
            base: GridLayoutTab::new(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.customization.armor"))),
            armor: (0..(4) as usize).map(|_| Default::default()).collect::<Vec<_>>(),
            parent: __init_parent,
            selected_slot: 0,
            trim_selection_widget: None,
            color_selection_widget: None,
            head_selection_widget: None,
            model_field_container: None,
            nothing_customizable: false,
            player: Default::default(),
        };
        { this.layout.row_spacing(PADDING.wrapping_div(2)); this.layout.column_spacing(PADDING); &mut this.layout };
        let mut list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = mcrust::mcrust::modding::game::with_player(|player| ItemUtils::get_armor(player)).unwrap();
        for i in 0..(list.len() as i32) {
            let mut copy: mcrust::net::minecraft::world::item::item_stack::ItemStack = (list[(i) as usize]).copy();
            this.armor[(3i32.wrapping_sub(i)) as usize] = copy.clone();
            this.player.set_item_slot(ARMOR_SLOTS.get_mut()[(i) as usize], copy.clone());
        }
        while this.selected_slot < (this.armor.len() as i32).wrapping_sub(1) && !(ArmorTab::can_edit(&this.armor[(this.selected_slot) as usize])) {
            this.selected_slot += 1;
        }
        this.nothing_customizable = !(ArmorTab::can_edit(&this.armor[(this.selected_slot) as usize]));
        let mut vertical: LinearLayout = LinearLayout::vertical().spacing(1);
        // TODO(fabric2seer): the mod keeps the game's `RemotePlayer` (Java holds a reference to the live object); the port's `RemotePlayer` is not a value to copy and the port lends the mod no handle to it
        // rust: let mut player_widget: compat::Shared<PlayerWidget> = PlayerWidget::new(0, 0, 84, 165, this.player.clone());
        let mut player_widget: compat::Shared<PlayerWidget> = todo!("fabric2seer: the initialiser of `playerWidget` (TODO above)");
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: vertical.add_child(player_widget.clone());
        let mut piece_selection_widget: compat::Shared<PieceSelectionWidget> = PieceSelectionWidget::new(0, 0);
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: vertical.add_child(piece_selection_widget.clone());
        this.layout.add_child_span_adjusted(LayoutElement::Grid(vertical.clone().wrapped), 0, 0, 2, 1, &mut |__a0| { __a0.align_vertically_middle(); }).unwrap();
        let mut width: i32 = ((444i32.min(compat::read(&parent, |__t| __t.width))).wrapping_sub(PLAYER_WIDGET_WIDTH)).wrapping_sub(PADDING.wrapping_mul(3));
        this.head_selection_widget = Some(HeadSelectionWidget::new(0, 0, width, 165));
        this.layout.add_child_span_adjusted(this.head_selection_widget.clone().unwrap(), 0, 1, 2, 1, &mut |__a0| { __a0.align_vertically_middle(); }).unwrap();
        let mut layout_widget: LinearLayout = LinearLayout::horizontal().spacing(PADDING.wrapping_div(2));
        let mut container_width: i32 = ((((width as f32) * (1.0f32 / 3.0f32))) as i32);
        this.trim_selection_widget = Some(TrimSelectionWidget::new(0, 0, (width.wrapping_sub(container_width)).wrapping_sub(PADDING.wrapping_div(2)), 80));
        this.model_field_container = Some(layout_widget.add_child(ModelFieldContainer::new(container_width, 80)));
        layout_widget.add_child(this.trim_selection_widget.clone().unwrap());
        layout_widget.arrange_elements();
        this.layout.add_child_at(LayoutElement::Grid(layout_widget.clone().wrapped), 0, 1).unwrap();
        let mut text_renderer: Font = compat::font();
        this.color_selection_widget = Some(ColorSelectionWidget::new(0, 0, width, 100, compat::font()));
        this.layout.add_child_at(this.color_selection_widget.clone().unwrap(), 1, 1).unwrap();
        if this.nothing_customizable {
            // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
            // rust: this.layout.add_child_span_adjusted(compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customization.nothingCustomizable")), 0, 1, 2, 1, &mut |p| { p.align_vertically_middle().align_horizontally_center(); }).unwrap();
        }
        this.update_widgets();
        this
    }

    /// java: private static boolean canEdit(ItemStack stack)
    pub fn can_edit(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut has_uuid: bool = !(stack.get_uuid().is_empty());
        if stack.is_item(&items::items().get("player_head")) {
            return has_uuid;
        }
        return (stack.is_in_tag(&mcrust::net::minecraft::tags::item_tags::trimmable_armor()) && has_uuid);
    }

    /// java: private void updateWidgets()
    pub fn update_widgets(&mut self) {
        if self.nothing_customizable {
            { let __v = false; compat::write(self.head_selection_widget.as_ref().unwrap(), |__t| __t.visible = __v); }
            self.trim_selection_widget.as_mut().unwrap().visible = false;
            self.color_selection_widget.as_mut().unwrap().visible = false;
            self.model_field_container.as_mut().unwrap().visible = false;
            return;
        }
        let item = &self.armor[(self.selected_slot) as usize];
        CustomizeScreen::backup_configs(&self.parent, item);
        let mut is_player_head: bool = item.is_item(&items::items().get("player_head"));
        HeadSelectionWidget::set_current_item(self.head_selection_widget.as_ref().unwrap(), item.clone());
        self.trim_selection_widget.as_mut().unwrap().set_current_item(item.clone());
        self.color_selection_widget.as_mut().unwrap().set_current_item(item.clone());
        { let __v = is_player_head; compat::write(self.head_selection_widget.as_ref().unwrap(), |__t| __t.visible = __v); }
        self.trim_selection_widget.as_mut().unwrap().visible = !is_player_head;
        self.color_selection_widget.as_mut().unwrap().visible = !is_player_head;
        self.model_field_container.as_mut().unwrap().visible = !is_player_head;
        let mut uuid: String = item.get_uuid();
        if SkyblockerConfigManager::get().general.custom_armor_model.contains_key(&uuid) {
            let mut identifier: Identifier = SkyblockerConfigManager::get().general.custom_armor_model.get(&uuid).cloned().unwrap();
            // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
            // rust: let mut string: String = identifier.fmt();
            let mut string: String = todo!("fabric2seer: the initialiser of `string` (TODO above)");
            // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
            // rust: let __wa1 = string.clone();
            // rust: let __o1 = self.model_field_container.as_ref().unwrap().field.clone().unwrap();
            // rust: compat::write(&__o1, |__t| __t.set_value(&__wa1));
        } else {
            // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
            // rust: let __o2 = self.model_field_container.as_ref().unwrap().field.clone().unwrap();
            // rust: compat::write(&__o2, |__t| __t.set_value(""));
        }
    }

    /// java: void tick()
    pub fn tick(&self) {
        // TODO(fabric2seer): `RemotePlayer.tickCount` has no Rust field in the port (searched net.minecraft.client.player.RemotePlayer and its supertypes)
        // rust: self.player.tick_count += 1;
    }

    /// java: @Override public void doLayout(ScreenRectangle tabArea)
    pub fn do_layout(&mut self, tab_area: &ScreenRectangle) {
        let mut width: i32 = ((444i32.min(tab_area.width())).wrapping_sub(PLAYER_WIDGET_WIDTH)).wrapping_sub(PADDING.wrapping_mul(3));
        ({ let __r = self.head_selection_widget.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_width()); __f(&compat::view(__r, |a| a as compat::Shared<dyn SearchableGridWidget>), width) });
        let mut model_field_width: i32 = ((((width as f32) * ((1 as f32) / 3.0f32))) as i32);
        self.trim_selection_widget.as_mut().unwrap().set_width((width.wrapping_sub(model_field_width)).wrapping_sub(PADDING.wrapping_div(2)));
        self.model_field_container.as_mut().unwrap().set_width(model_field_width);
        self.color_selection_widget.as_mut().unwrap().set_width(width);
        // TODO(fabric2seer): `do_layout` takes 2 argument(s) in the port (`screen: &mut crate::net::minecraft::client::gui::screens::screen::Screen, screen_rectangle: crate::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle`), Java passes 1
        // rust: self.base.do_layout(&mut tab_area);
    }

    /// java: public void recreate()
    pub fn recreate(&self) {
        AnimatedDyeTimelineWidget::recreate_image(&self.color_selection_widget.as_ref().unwrap().get_timeline_widget());
    }

    /// java: @Override public void close()
    pub fn close(&mut self) {
        self.color_selection_widget.as_mut().unwrap().close();
    }

}

// NOTE(fabric2seer): close, doLayout override(s) of GridLayoutTab - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class PieceSelectionWidget extends AbstractWidget
#[derive(Clone)]
pub struct PieceSelectionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final boolean[] selectable;
    pub selectable: Vec<bool>,
}

impl std::ops::Deref for PieceSelectionWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PieceSelectionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PieceSelectionWidget {
    /// java: private PieceSelectionWidget(int x, int y)
    pub fn new(mut x: i32, mut y: i32) -> compat::Shared<Self> {
        // TODO(fabric2seer): `armor` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
        //   java: selectable = new boolean[armor.length];
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, 84, 24, (&component::null_to_empty(Some("")).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            selectable: todo!("fabric2seer: the field value (TODO above)"),
        });
        // TODO(fabric2seer): `armor` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
        //   java: for (int i = 0; i < armor.length; i++) {
        //   java: 				selectable[i] = canEdit(armor[i]);
        //   java: 			}
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), PIECE_SELECTION_WIDGET_HOTBAR_TEXTURE.get_mut(), (__this.lock().borrow().get_x()).wrapping_add(1), (__this.lock().borrow().get_y()).wrapping_add(1), 82, 22);
        let mut hovered_slot: i32 = -1;
        let mut local_x: i32 = (mouse_x.wrapping_sub(__this.lock().borrow().get_x())).wrapping_sub(2);
        let mut local_y: i32 = (mouse_y.wrapping_sub(__this.lock().borrow().get_y())).wrapping_sub(2);
        if local_y >= 0 && local_y < 20 {
            // TODO(fabric2seer): `armor` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
            //   java: hoveredSlot = localX / 20 >= armor.length ? -1 : localX / 20;
        }
        if hovered_slot >= 0 && __this.lock().borrow().selectable[(hovered_slot) as usize] {
            let mut i: i32 = ((__this.lock().borrow().get_x()).wrapping_add(2)).wrapping_add(hovered_slot.wrapping_mul(20));
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), i, (__this.lock().borrow().get_y()).wrapping_add(2), i.wrapping_add(20), (__this.lock().borrow().get_y()).wrapping_add(22), 0x20FFFFFF);
        }
        // TODO(fabric2seer): `armor` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
        //   java: for (int i = 0; i < armor.length; i++) {
        //   java: 				graphics.item(armor[i], getX() + 4 + i * 20, getY() + 4);
        //   java: 				if (!selectable[i] && !armor[i].isEmpty()) {
        //   java: 					graphics.item(BARRIER, getX() + 4 + i * 20, getY() + 4);
        //   java: 				}
        //   java: 			}
        // TODO(fabric2seer): `selectedSlot` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, HOTBAR_SELECTION_TEXTURE, getX() + selectedSlot * 20, getY(), 24, 24);
        __this.lock().borrow().handle_cursor(graphics);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        let mut local_x: f64 = ((click.x() - (__this.lock().borrow().get_x() as f64)) - (2 as f64));
        let mut local_y: f64 = ((click.y() - (__this.lock().borrow().get_y() as f64)) - (2 as f64));
        if local_y < (0 as f64) || local_y >= (20 as f64) {
            return;
        }
        let mut i: i32 = (((local_x / (20 as f64))) as i32);
        // TODO(fabric2seer): the condition: `armor` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
        //   java: i < 0 || i >= armor.length || !selectable[i]
        if todo!("fabric2seer: the condition (TODO above)") {
            return;
        }
        // TODO(fabric2seer): the condition: `selectedSlot` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
        //   java: i != selectedSlot
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `selectedSlot` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
            //   java: selectedSlot = i;
            // TODO(fabric2seer): `updateWidgets(..)` is a method of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
            //   java: updateWidgets();
        }
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (compat::read(__this, |__t| __t.active) && compat::read(__this, |__t| __t.visible) && mouse_x >= ((__this.lock().borrow().get_x()).wrapping_add(2) as f64) && mouse_y >= ((__this.lock().borrow().get_y()).wrapping_add(2) as f64) && mouse_x < ((__this.lock().borrow().get_right()).wrapping_sub(2) as f64) && mouse_y < ((__this.lock().borrow().get_bottom()).wrapping_sub(2) as f64));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// PieceSelectionWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for PieceSelectionWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// PieceSelectionWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<PieceSelectionWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| PieceSelectionWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| PieceSelectionWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| PieceSelectionWidget::on_click(__h, event, double_click))
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, widget: &AbstractWidget, mouse_x: f64, mouse_y: f64) -> bool {
        self.lend_ref(widget, |__h| PieceSelectionWidget::is_mouse_over(__h, mouse_x, mouse_y))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, isMouseOver, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class ModelFieldContainer extends AbstractContainerWidget
#[derive(Clone)]
pub struct ModelFieldContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final Component text = Component.translatable("skyblocker.customization.armor.modelOverride").withStyle(ChatFormatting.ITALIC).withStyle(ChatFormatting.
    pub text: MutableComponent,
    /// java: private final FrameLayout containerLayout;
    pub container_layout: FrameLayout,
    /// java: private final IdentifierTextField field;
    pub field: Option<compat::Shared<IdentifierTextField>>,
}

impl std::ops::Deref for ModelFieldContainer {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ModelFieldContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ModelFieldContainer {
    /// java: private ModelFieldContainer(int width, int height)
    pub fn new(mut width: i32, mut height: i32) -> Self {
        let __init_container_layout = FrameLayout::new();
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(0, 0, width, height, Component.empty(), AbstractScrollArea.defaultSettings(4));
        let mut this: Self = todo!("constructor");
        this.field = Some(this.container_layout.add_child(IdentifierTextField::new(width.wrapping_sub(10), 20, Arc::new(move |identifier| {
            // TODO(fabric2seer): `armor` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
            //   java: String uuid = armor[selectedSlot].getUuid();
            let mut uuid: String = todo!("fabric2seer: the initialiser of `uuid` (TODO above)");
            if uuid.is_empty() {
                return;
            }
            SkyblockerConfigManager::update_only(Arc::new(move |config| {
                if identifier.is_none() {
                    config.general.custom_armor_model.remove(&uuid);
                } else {
                    config.general.custom_armor_model.insert(uuid.clone(), identifier.clone().unwrap());
                }
            }));
            // TODO(fabric2seer): `colorSelectionWidget` is a field of the enclosing instance (ArmorTab.this) - an inner class has no outer reference in Rust
            //   java: colorSelectionWidget.refresh();
        }))));
        this.container_layout.arrange_elements();
        // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: let __wa1 = Tooltip::create(component::translatable("skyblocker.customization.armor.modelOverride.tooltip").get_string());
        // rust: let __o1 = this.field.clone().unwrap();
        // rust: compat::write(&__o1, |__t| __t.set_tooltip(__wa1.clone()));
        // TODO(fabric2seer): `EditBox.setTooltipDelay` has no Rust method in the port (looked for `set_tooltip_delay` in EditBox and its supertypes)
        // rust: let __wa2 = std::time::Duration::from_millis((400) as u64);
        // rust: let __o2 = this.field.clone().unwrap();
        // rust: compat::write(&__o2, |__t| __t.set_tooltip_delay(&__wa2));
        this
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&self, mut x: i32) {
        self.base.set_x(x);
        // TODO(fabric2seer): `align_in_dimension` takes 4 argument(s) in the port (`pos: i32, length: i32, widget_length: i32, align: f32`), Java passes 5
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<Integer>`)
        // rust: frame_layout::align_in_dimension(self.get_x(), self.get_width(), self.container_layout.get_width(), |__a0| { self.container_layout.set_x(__a0); }, 0.5f32);
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        { self.container_layout.set_min_width(width); &mut self.container_layout };
        self.container_layout.arrange_elements();
        let __wa1 = width.wrapping_sub(10);
        let __o1 = self.field.clone().unwrap();
        compat::write(&__o1, |__t| __t.set_width(__wa1));
        self.set_x(self.get_x());
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&self, mut y: i32) {
        self.base.set_y(y);
        // TODO(fabric2seer): `align_in_dimension` takes 4 argument(s) in the port (`pos: i32, length: i32, widget_length: i32, align: f32`), Java passes 5
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<Integer>`)
        // rust: frame_layout::align_in_dimension(self.get_y(), self.get_height(), self.container_layout.get_height(), |__a0| { self.container_layout.set_y_translating(__a0); }, 0.5f32);
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return (if self.visible { vec![self.field.clone().unwrap()] } else { vec![] });
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((0) as f64) as f64;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if !self.visible {
            return;
        }
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), INNER_SPACE_TEXTURE.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `EditBox` does not implement
        // rust: let __wa1 = graphics.clone();
        // rust: let __wa2 = mouse_x;
        // rust: let __wa3 = mouse_y;
        // rust: let __wa4 = a;
        // rust: let __o1 = self.field.clone().unwrap();
        // rust: compat::write(&__o1, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4));
        // TODO(fabric2seer): field `NONE` of a value of unknown type
        //   java: this.extractLabel(graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.NONE));
    }

    /// java: private void extractLabel(ActiveTextCollector drawer)
    pub fn extract_label(&self, drawer: &dyn ActiveTextCollector) {
        let mut padding: i32 = 5;
        let mut start_y: i32 = self.get_y().wrapping_add(padding);
        // TODO(fabric2seer): `accept_scrolling_with_default_center` takes 6 argument(s) in the port (`left: i32, right: i32, top: i32, bottom: i32, line_width: i32, time_millis: i64`), Java passes 5
        // rust: active_text_collector::accept_scrolling_with_default_center(self.text.clone(), self.get_x().wrapping_add(padding), self.get_right().wrapping_sub(padding), start_y, start_y.wrapping_add(9));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, scrollRate, setWidth, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
