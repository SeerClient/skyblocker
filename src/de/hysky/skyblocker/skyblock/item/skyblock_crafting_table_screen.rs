//! Ported from de.hysky.skyblocker.skyblock.item.SkyblockCraftingTableScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_crafting_table_screen_handler::SkyblockCraftingTableScreenHandler;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_book_component::SkyblockRecipeBookComponent;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use mcrust::mcrust::modding::container_screens::ModContainerScreen;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::RecipeBookComponent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final FallbackedTexture<Identifier> TEXTURE = FallbackedTexture.ofTexture( SkyblockerMod.id("textures/gui/container/skyblock_crafting_table.png"), Identifier.withDefaultNamespace("textu
pub static TEXTURE: compat::MainThread<compat::Shared<dyn FallbackedTexture<Identifier>>> = compat::MainThread::new(|| fallbacked_texture::of_texture(&crate::entry::skyblocker_mod::id("textures/gui/container/skyblock_crafting_table.png"), &Identifier::with_default_namespace("textures/gui/container/crafting_table.png")));
/// java: private static final FallbackedTexture<Identifier> MIRRORVERSE_TEXTURE = FallbackedTexture.ofTexture( SkyblockerMod.id("textures/gui/container/skyblock_crafting_table_mirroverse.png"), Identifier.with
pub static MIRRORVERSE_TEXTURE: compat::MainThread<compat::Shared<dyn FallbackedTexture<Identifier>>> = compat::MainThread::new(|| fallbacked_texture::of_texture(&crate::entry::skyblocker_mod::id("textures/gui/container/skyblock_crafting_table_mirroverse.png"), &Identifier::with_default_namespace("textures/gui/container/crafting_table.png")));
// TODO(fabric2seer): the port's `new` ports this Java method with other parameters (enabled, disabled, enabled_focused, disabled_focused for Java's enabled, disabled, focused)
//   java: protected static final WidgetSprites MORE_CRAFTS_TEXTURES = new WidgetSprites(
//   java: 			SkyblockerMod.id("quick_craft/more_button"),
//   java: 			SkyblockerMod.id("quick_craft/more_button_disabled"),
//   java: 			SkyblockerMod.id("quick_craft/more_button_highlighted")
//   java: 	);
/// java: protected static final WidgetSprites MORE_CRAFTS_TEXTURES = new WidgetSprites( SkyblockerMod.id("quick_craft/more_button"), SkyblockerMod.id("quick_craft/more_button_disabled"), SkyblockerMod.id("quic
pub static MORE_CRAFTS_TEXTURES: compat::MainThread<WidgetSprites> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MORE_CRAFTS_TEXTURES` - see the TODO above"));
/// java: protected static final Identifier QUICK_CRAFT = SkyblockerMod.id("textures/gui/sprites/quick_craft/quick_craft_overlay.png");
pub static QUICK_CRAFT: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/sprites/quick_craft/quick_craft_overlay.png"));

/// java: public class SkyblockCraftingTableScreen extends AbstractContainerScreen<SkyblockCraftingTableScreenHandler>
pub struct SkyblockCraftingTableScreen {
    /// java: extends net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
    pub base: AbstractContainerScreen,
    /// java: private final SkyblockRecipeBookComponent recipeBook = new SkyblockRecipeBookComponent(menu);
    pub recipe_book: compat::Shared<SkyblockRecipeBookComponent>,
    /// java: private boolean narrow;
    pub narrow: bool,
    /// java: private ImageButton moreCraftsButton;
    pub more_crafts_button: Option<AbstractWidget>,
}

impl std::ops::Deref for SkyblockCraftingTableScreen {
    type Target = AbstractContainerScreen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockCraftingTableScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockCraftingTableScreen {
    /// java: public SkyblockCraftingTableScreen(SkyblockCraftingTableScreenHandler handler, Inventory inventory, Component title)
    pub fn new(mut handler: compat::Shared<SkyblockCraftingTableScreenHandler>, inventory: compat::PlayerInventory, mut title: MutableComponent) -> compat::Shared<Self> {
        // java: private final SkyblockRecipeBookComponent recipeBook = new SkyblockRecipeBookComponent(menu);
        // (the component reads only the menu's `containerId` - its `super(new NoopRecipeBookMenu(screenHandler.containerId), ..)`;
        // the port's constructor takes the menu by value, so it is handed a menu carrying that id)
        let __container_id = compat::read(&handler, |__h| __h.base.container_id);
        let __this = compat::shared(Self {
            base: { let mut __s = AbstractContainerScreen::new((&title.get_string()).to_string(), String::new());  __s },
            recipe_book: SkyblockRecipeBookComponent::new(mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu::new(__container_id)),
            narrow: false,
            more_crafts_button: None,
        });
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        { let __v = (compat::read(__this, |__t| __t.screen.width) < 379); compat::write(__this, |__t| __t.narrow = __v); }
        SkyblockRecipeBookComponent::init(&compat::read(__this, |__t| __t.recipe_book.clone()), compat::read(__this, |__t| __t.screen.width), compat::read(__this, |__t| __t.screen.height), compat::minecraft(), compat::read(__this, |__t| __t.narrow));
        let __wa1 = compat::read(__this, |__t| __t.screen.width);
        let __wa2 = compat::read(__this, |__t| __t.image_width);
        let __o1 = compat::read(__this, |__t| __t.recipe_book.clone());
        { let __v = compat::write(&__o1, |__t| __t.update_screen_position(__wa1, __wa2)); compat::write(__this, |__t| __t.left_pos = __v); }
        // TODO(fabric2seer): static `RecipeBookComponent.RECIPE_BUTTON_SPRITES` has no Rust counterpart in the port's RecipeBookComponent file
        // TODO(fabric2seer): `new ImageButton(..)`: the port has no `ImageButton::new`
        // rust: let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: { let (__q0,) = (AbstractWidget::new((compat::read(__this, |__t| __t.left_pos)).wrapping_add(5), ((compat::read(__this, |__t| __t.screen.height)).wrapping_div(2)).wrapping_sub(49), 20, 18, RecipeBookComponent::RECIPE_BUTTON_SPRITES.clone(), Arc::new(move |button| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; {
        // rust: let __o5 = compat::read((&__this_4), |__t| __t.recipe_book.clone());
        // rust: compat::write(&__o5, |__t| __t.toggle_visibility(&*compat::minecraft()));
        // rust: let __wa5 = compat::read((&__this_4), |__t| __t.screen.width);
        // rust: let __wa6 = compat::read((&__this_4), |__t| __t.image_width);
        // rust: let __o6 = compat::read((&__this_4), |__t| __t.recipe_book.clone());
        // rust: { let __v = compat::write(&__o6, |__t| __t.update_screen_position(__wa5, __wa6)); compat::write((&__this_4), |__t| __t.left_pos = __v); }
        // rust: // TODO(fabric2seer): `.setPosition(..)` on a value of unknown type
        // rust: //   java: button.setPosition(this.leftPos + 5, this.height / 2 - 49);
        // rust: if compat::read((&__this_4), |__t| __t.more_crafts_button.clone()).is_some() {
        // rust: let __r0 = compat::read((&__this_4), |__t| __t.left_pos);
        // rust: let __r1 = compat::read((&__this_4), |__t| __t.top_pos);
        // rust: { let (__q0, __q1,) = ((__r0).wrapping_add(152), (__r1).wrapping_add(63),); (&__this_4).lock().borrow_mut().more_crafts_button.as_mut().unwrap().set_position(__q0, __q1) };
        // rust: }
        // rust: } })),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        // TODO(fabric2seer): `AbstractContainerScreen.menu` has no Rust field in the port (searched net.minecraft.client.gui.screens.inventory.AbstractContainerScreen and its supertypes)
        // TODO(fabric2seer): the condition: field `mirrorverse` of a T
        //   java: !menu.mirrorverse
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): field `slots` of a T
            //   java: moreCraftsButton = new ImageButton(this.leftPos + 152, topPos + 63, 16, 16, MORE_CRAFTS_TEXTURES,
            //   java: 					_ -> this.slotClicked(menu.slots.get(26), menu.slots.get(26).index, 0, ContainerInput.PICKUP));
            // TODO(fabric2seer): `ImageButton.setTooltipDelay` has no Rust method in the port (looked for `set_tooltip_delay` in ImageButton and its supertypes)
            // rust: let __h0 = &std::time::Duration::from_millis((250) as u64);
            // rust: __this.lock().borrow().more_crafts_button.as_ref().unwrap().set_tooltip_delay(__h0);
            { let (__q0,) = (Tooltip::create(component::literal("More Crafts").get_string()),); __this.lock().borrow_mut().more_crafts_button.as_mut().unwrap().set_tooltip(__q0) };
            let __r0 = compat::read(__this, |__t| __t.more_crafts_button.clone());
            { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().screen.add_renderable_widget(__q0) };
        }
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        // java: this.addWidget(this.recipeBook); this.setInitialFocus(this.recipeBook);
        // rust: port gap - the port's `RecipeBookComponent` is not a widget of the screen's tree (it is drawn and clicked
        //   by its screen's own bodies), so a mod's recipe book has no widget to add or focus (GAPS_batchE.md)
        { let __v = 29; compat::write(__this, |__t| __t.title_label_x = __v); }
    }

    /// java: @Override public void containerTick()
    pub fn container_tick(__this: &compat::Shared<Self>) {
        ();
        // TODO(fabric2seer): `tick` takes 2 argument(s) in the port (`times_inventory_changed: i32, gfx: &Minecraft`), Java passes 0
        // rust: let __o1 = compat::read(__this, |__t| __t.recipe_book.clone());
        // rust: compat::write(&__o1, |__t| __t.tick());
        if compat::read(__this, |__t| __t.more_crafts_button.clone()).is_none() {
            return;
        }
        // TODO(fabric2seer): field `slots` of a T
        //   java: ItemStack stack = menu.slots.get(26).getItem();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
        __this.lock().borrow_mut().more_crafts_button.as_mut().unwrap().active = (stack.is_empty() || stack.is_item(&items::items().get("player_head")));
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let __o1 = compat::read(__this, |__t| __t.recipe_book.clone());
        let __o2 = compat::read(__this, |__t| __t.recipe_book.clone());
        if compat::write(&__o2, |__t| __t.is_visible()) && compat::read(__this, |__t| __t.narrow) {
            SkyblockCraftingTableScreen::extract_background(__this, graphics, mouse_x, mouse_y, a);
            SkyblockRecipeBookComponent::extract_render_state(&compat::read(__this, |__t| __t.recipe_book.clone()), graphics, mouse_x, mouse_y, a);
        } else {
            ();
            SkyblockRecipeBookComponent::extract_render_state(&compat::read(__this, |__t| __t.recipe_book.clone()), graphics, mouse_x, mouse_y, a);
            // TODO(fabric2seer): `extract_ghost_recipe` takes 6 argument(s) in the port (`gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, is_result_slot_big: bool, slot_position: &dyn Fn(usize) -> (i32, i32), resolver: Option< &crate::net::minecraft::client::renderer::item::item_model_re ..`), Java passes 2
            // rust: let __wa1 = graphics.clone();
            // rust: let __o3 = compat::read(__this, |__t| __t.recipe_book.clone());
            // rust: compat::write(&__o3, |__t| __t.extract_ghost_recipe(&__wa1, &true));
        }
        // TODO(fabric2seer): `AbstractContainerScreen.extractTooltip` has no Rust method in the port (looked for `extract_tooltip` in AbstractContainerScreen and its supertypes)
        // rust: __this.lock().borrow().extract_tooltip(graphics, mouse_x, mouse_y);
        SkyblockRecipeBookComponent::extract_tooltip(&compat::read(__this, |__t| __t.recipe_book.clone()), graphics, mouse_x, mouse_y, None);
    }

    /// java: @Override protected void extractSlot(GuiGraphicsExtractor graphics, Slot slot, int mouseX, int mouseY)
    pub fn extract_slot(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, slot: &Slot, mut mouse_x: i32, mut mouse_y: i32) {
        // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
        // rust: let stack = slot.get_item();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
        if slot.index == 23 && stack.is_item(&items::items().get("barrier")) {
            return;
        }
        if stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().gray()) && stack.get_skyblock_id().is_empty() {
            return;
        }
        compat::call_super();
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        compat::call_super();
        let mut i: i32 = compat::read(__this, |__t| __t.left_pos);
        let mut j: i32 = ((compat::read(__this, |__t| __t.screen.height)).wrapping_sub(compat::read(__this, |__t| __t.image_height))).wrapping_div(2);
        // TODO(fabric2seer): field `mirrorverse` of a T
        //   java: graphics.blit(RenderPipelines.GUI_TEXTURED, menu.mirrorverse ? MIRRORVERSE_TEXTURE.get() : TEXTURE.get(), i, j, 0, 0, this.imageWidth, this.imageHeight, 256, 256);
        // TODO(fabric2seer): `AbstractContainerScreen.menu` has no Rust field in the port (searched net.minecraft.client.gui.screens.inventory.AbstractContainerScreen and its supertypes)
        // TODO(fabric2seer): the condition: field `mirrorverse` of a T
        //   java: TEXTURE.isUsingFallback() && !menu.mirrorverse
        if todo!("fabric2seer: the condition (TODO above)") {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), QUICK_CRAFT.get_mut(), i.wrapping_add(143), j.wrapping_sub(3), ((0) as f32) as f32, ((0) as f32) as f32, 37, 90, 37, 90);
        }
    }

    /// java: @Override protected boolean isHovering(int x, int y, int width, int height, double pointX, double pointY)
    pub fn is_hovering(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut point_x: f64, mut point_y: f64) -> bool {
        let __o1 = compat::read(__this, |__t| __t.recipe_book.clone());
        return ((!(compat::read(__this, |__t| __t.narrow)) || !(compat::write(&__o1, |__t| __t.is_visible()))) && __this.lock().borrow_mut().base.is_hovering_box(x, y, width, height, point_x, point_y));
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if SkyblockRecipeBookComponent::mouse_clicked(&compat::read(__this, |__t| __t.recipe_book.clone()), click, doubled) {
            // TODO(fabric2seer): `AbstractContainerScreen.setFocused` has no Rust method in the port (looked for `set_focused` in AbstractContainerScreen and its supertypes)
            // rust: let __h0 = &compat::read(__this, |__t| __t.recipe_book.clone());
            // rust: __this.lock().borrow().set_focused(__h0);
            return true;
        }
        let __o1 = compat::read(__this, |__t| __t.recipe_book.clone());
        let __o2 = compat::read(__this, |__t| __t.recipe_book.clone());
        if compat::read(__this, |__t| __t.narrow) && compat::write(&__o2, |__t| __t.is_visible()) {
            return true;
        }
        return false;
    }

    /// java: @Override protected boolean hasClickedOutside(double mouseX, double mouseY, int left, int top)
    pub fn has_clicked_outside(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut left: i32, mut top: i32) -> bool {
        let mut bl: bool = (mouse_x < ((left) as f64) || mouse_y < ((top) as f64) || mouse_x >= ((left.wrapping_add(compat::read(__this, |__t| __t.image_width))) as f64) || mouse_y >= ((top.wrapping_add(compat::read(__this, |__t| __t.image_height))) as f64));
        return (SkyblockRecipeBookComponent::has_clicked_outside(&compat::read(__this, |__t| __t.recipe_book.clone()), mouse_x, mouse_y, compat::read(__this, |__t| __t.left_pos), compat::read(__this, |__t| __t.top_pos), compat::read(__this, |__t| __t.image_width), compat::read(__this, |__t| __t.image_height)) && bl);
    }

    /// java: @Override protected void slotClicked(Slot slot, int slotId, int button, ContainerInput actionType)
    pub fn slot_clicked(__this: &compat::Shared<Self>, slot: &Slot, mut slot_id: i32, mut button: i32, action_type: &ContainerInput) {
        compat::call_super();
        // TODO(fabric2seer): `RecipeBookComponent.slotClicked` has no Rust method in the port (looked for `slot_clicked` in RecipeBookComponent and its supertypes)
        // rust: let __wa1 = slot.clone();
        // rust: let __o1 = compat::read(__this, |__t| __t.recipe_book.clone());
        // rust: compat::write(&__o1, |__t| __t.slot_clicked(&__wa1));
    }

}

/// SkyblockCraftingTableScreen's screen half (`super`'s fields), lent by the game for each call
impl compat::ContainerHalf for SkyblockCraftingTableScreen {
    fn container_half(&mut self) -> &mut AbstractContainerScreen {
        &mut self.base
    }
}

/// SkyblockCraftingTableScreen as the game's container screen (`container_screens::register`'s answer)
impl ModContainerScreen for compat::ContainerScreenAdapter<SkyblockCraftingTableScreen> {
    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut AbstractContainerScreen) {
        self.lend(screen, |__h| SkyblockCraftingTableScreen::init(__h))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_background: &mut dyn FnMut()) {
        compat::with_super(super_extract_background, || self.lend_ref(screen, |__h| SkyblockCraftingTableScreen::extract_background(__h, &context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| SkyblockCraftingTableScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut AbstractContainerScreen, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(screen, |__h| SkyblockCraftingTableScreen::mouse_clicked(__h, event, double_click))
    }

    /// java: @Override public void containerTick()
    fn container_tick(&mut self, screen: &mut AbstractContainerScreen) {
        self.lend(screen, |__h| SkyblockCraftingTableScreen::container_tick(__h))
    }

    /// java: @Override protected void slotClicked(Slot slot, int slotId, int button, ContainerInput actionType)
    fn slot_clicked(&mut self, screen: &mut AbstractContainerScreen, slot: Option<&Slot>, slot_id: i32, button_num: i32, container_input: ContainerInput, super_slot_clicked: &mut dyn FnMut(&mut AbstractContainerScreen, Option<i32>, i32, i32, ContainerInput)) {
        let __slot_index = slot.map(|s| s.index);
        // java's `null` slot (a click outside): the override's body takes a slot - the base runs
        let Some(slot) = slot else {
            super_slot_clicked(screen, None, slot_id, button_num, container_input);
            return;
        };
        let __object = self.object().clone();
        let mut __super = || compat::write(&__object, |__t| super_slot_clicked(compat::ContainerHalf::container_half(__t), __slot_index, slot_id, button_num, container_input));
        compat::with_super(&mut __super, || self.lend(screen, |__h| SkyblockCraftingTableScreen::slot_clicked(__h, slot, slot_id, button_num, &container_input)))
    }

    /// java: @Override protected void extractSlot(GuiGraphicsExtractor graphics, Slot slot, int mouseX, int mouseY)
    fn extract_slot(&mut self, screen: &mut AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, slot: &Slot, mouse_x: i32, mouse_y: i32, super_extract_slot: &mut dyn FnMut(&mut AbstractContainerScreen)) {
        let __object = self.object().clone();
        let mut __super = || compat::write(&__object, |__t| super_extract_slot(compat::ContainerHalf::container_half(__t)));
        compat::with_super(&mut __super, || self.lend(screen, |__h| SkyblockCraftingTableScreen::extract_slot(__h, &context.graphics, slot, mouse_x, mouse_y)))
    }

}

// NOTE(fabric2seer): containerTick, extractBackground, extractRenderState, extractSlot, hasClickedOutside, init, isHovering, mouseClicked, slotClicked override(s) of AbstractContainerScreen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
