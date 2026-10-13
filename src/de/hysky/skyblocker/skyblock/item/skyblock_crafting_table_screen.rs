//! Ported from de.hysky.skyblocker.skyblock.item.SkyblockCraftingTableScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::{OnPress, PressLambda};
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
//   java: protected static final WidgetSprites MORE_CRAFTS_TEXTURES = new WidgetSprites(
//   java: 			SkyblockerMod.id("quick_craft/more_button"),
//   java: 			SkyblockerMod.id("quick_craft/more_button_disabled"),
//   java: 			SkyblockerMod.id("quick_craft/more_button_highlighted")
//   java: 	);
/// java: protected static final WidgetSprites MORE_CRAFTS_TEXTURES = new WidgetSprites( SkyblockerMod.id("quick_craft/more_button"), SkyblockerMod.id("quick_craft/more_button_disabled"), SkyblockerMod.id("quic
pub static MORE_CRAFTS_TEXTURES: compat::MainThread<WidgetSprites> = compat::MainThread::new(|| WidgetSprites::with_disabled(crate::entry::skyblocker_mod::id("quick_craft/more_button"), crate::entry::skyblocker_mod::id("quick_craft/more_button_disabled"), crate::entry::skyblocker_mod::id("quick_craft/more_button_highlighted")));
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
    /// (its index in the screen's widgets: the screen holds the button)
    pub more_crafts_button: Option<usize>,
    /// the recipe book button `init` adds (its index in the screen's widgets)
    pub recipe_book_button: Option<usize>,
    /// the recipe book button's press, run in the screen's next own call (the press runs inside the game's widget
    /// pass, where the screen's fields - `leftPos` - are the game's)
    pub recipe_book_toggle_requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// java: `AbstractContainerScreen.menu` - the screen's handler (the port's container screen holds no menu of its
    /// own: the mod's object is kept here)
    pub menu: compat::Shared<SkyblockCraftingTableScreenHandler>,
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
            recipe_book_button: None,
            recipe_book_toggle_requested: Default::default(),
            menu: handler.clone(),
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
        // java: addRenderableWidget(new ImageButton(this.leftPos + 5, this.height / 2 - 49, 20, 18, RecipeBookComponent.RECIPE_BUTTON_SPRITES, button -> {
        //   this.recipeBook.toggleVisibility(); this.leftPos = this.recipeBook.updateScreenPosition(this.width, this.imageWidth);
        //   button.setPosition(this.leftPos + 5, this.height / 2 - 49);
        //   if (moreCraftsButton != null) moreCraftsButton.setPosition(this.leftPos + 152, this.topPos + 63); }));
        // (the press is run by `toggle_recipe_book` in the screen's next own call)
        let (__bx, __by) = compat::read(__this, |__t| (__t.left_pos.wrapping_add(5), (__t.screen.height.wrapping_div(2)).wrapping_sub(49)));
        let __requested = compat::read(__this, |__t| __t.recipe_book_toggle_requested.clone());
        let __recipe_button = AbstractWidget::new(__bx, __by, 20, 18, String::new(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::ImageButton(mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites::with_focused(mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::recipe_button_sprite(), mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::recipe_button_highlighted_sprite())), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            __requested.store(true, std::sync::atomic::Ordering::Relaxed);
        })));
        let __recipe_index = compat::write(__this, |__t| __t.screen.add_renderable_widget(__recipe_button));
        compat::write(__this, |__t| __t.recipe_book_button = Some(__recipe_index));
        // java: if (!menu.mirrorverse)
        let __menu = compat::read(__this, |__t| __t.menu.clone());
        let (__mirrorverse, __container_id) = compat::read(&__menu, |__m| (__m.mirrorverse, __m.base.container_id));
        if !__mirrorverse {
            // java: moreCraftsButton = new ImageButton(this.leftPos + 152, topPos + 63, 16, 16, MORE_CRAFTS_TEXTURES,
            //         _ -> this.slotClicked(menu.slots.get(26), menu.slots.get(26).index, 0, ContainerInput.PICKUP));
            // (`slotClicked`'s base is `gameMode.handleContainerInput(menu.containerId, slot.index, ..)`; the recipe
            // book's half of this override is a port gap, see `slot_clicked`)
            let (__x, __y) = compat::read(__this, |__t| (__t.left_pos.wrapping_add(152), __t.top_pos.wrapping_add(63)));
            let __button = AbstractWidget::new(__x, __y, 16, 16, mcrust::net::minecraft::client::gui::components::image_button::DEFAULT_MESSAGE.to_string(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::ImageButton(MORE_CRAFTS_TEXTURES.get().clone()), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
                compat::handle_container_input(__container_id, 26, 0, mcrust::net::minecraft::world::inventory::container_input::ContainerInput::Pickup);
            })));
            let mut __button = __button;
            // java: moreCraftsButton.setTooltipDelay(Duration.ofMillis(250));
            __button.set_tooltip_delay(250);
            // java: moreCraftsButton.setTooltip(Tooltip.create(Component.literal("More Crafts")));
            __button.set_tooltip(Tooltip::create(component::literal("More Crafts").get_string()));
            // java: addRenderableWidget(moreCraftsButton);
            let __index = compat::write(__this, |__t| __t.screen.add_renderable_widget(__button));
            compat::write(__this, |__t| __t.more_crafts_button = Some(__index));
        }
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        // java: this.addWidget(this.recipeBook); this.setInitialFocus(this.recipeBook);
        // rust: port gap - the port's `RecipeBookComponent` is not a widget of the screen's tree (it is drawn and clicked
        //   by its screen's own bodies), so a mod's recipe book has no widget to add or focus (GAPS_batchE.md)
        { let __v = 29; compat::write(__this, |__t| __t.title_label_x = __v); }
    }

    /// The recipe book button's press (`init`): `recipeBook.toggleVisibility(); leftPos = recipeBook
    /// .updateScreenPosition(width, imageWidth); button.setPosition(leftPos + 5, height / 2 - 49); if (moreCraftsButton
    /// != null) moreCraftsButton.setPosition(leftPos + 152, topPos + 63);`
    pub fn toggle_recipe_book(__this: &compat::Shared<Self>) {
        let __requested = compat::read(__this, |__t| __t.recipe_book_toggle_requested.clone());
        if !__requested.swap(false, std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let __book = compat::read(__this, |__t| __t.recipe_book.clone());
        compat::write(&__book, |__t| __t.toggle_visibility(&*compat::minecraft()));
        let (__width, __image_width) = compat::read(__this, |__t| (__t.screen.width, __t.image_width));
        let __left = compat::write(&__book, |__t| __t.update_screen_position(__width, __image_width));
        compat::write(__this, |__t| __t.left_pos = __left);
        let (__left, __top, __height, __recipe, __more) = compat::read(__this, |__t| (__t.left_pos, __t.top_pos, __t.screen.height, __t.recipe_book_button, __t.more_crafts_button));
        compat::write(__this, |__t| {
            if let Some(__i) = __recipe {
                __t.screen.widget_mut(__i).set_position(__left.wrapping_add(5), (__height.wrapping_div(2)).wrapping_sub(49));
            }
            if let Some(__i) = __more {
                __t.screen.widget_mut(__i).set_position(__left.wrapping_add(152), __top.wrapping_add(63));
            }
        });
    }

    /// java: @Override public void containerTick()
    pub fn container_tick(__this: &compat::Shared<Self>) {
        SkyblockCraftingTableScreen::toggle_recipe_book(__this);
        ();
        // TODO(fabric2seer): `tick` takes 2 argument(s) in the port (`times_inventory_changed: i32, gfx: &Minecraft`), Java passes 0
        // rust: let __o1 = compat::read(__this, |__t| __t.recipe_book.clone());
        // rust: compat::write(&__o1, |__t| __t.tick());
        if compat::read(__this, |__t| __t.more_crafts_button.clone()).is_none() {
            return;
        }
        // java: ItemStack stack = menu.slots.get(26).getItem(); - the open menu's slot
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::with_open_menu(|menu, containers| menu.slots[26].get_item(containers).duplicate()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
        let __active = stack.is_empty() || stack.is_item(&items::items().get("player_head"));
        if let Some(__index) = compat::read(__this, |__t| __t.more_crafts_button) {
            compat::write(__this, |__t| __t.screen.widget_mut(__index).active = __active);
        }
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
        // java: ItemStack stack = slot.getItem();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
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
        // java: graphics.blit(RenderPipelines.GUI_TEXTURED, menu.mirrorverse ? MIRRORVERSE_TEXTURE.get() : TEXTURE.get(), i, j, 0, 0, this.imageWidth, this.imageHeight, 256, 256);
        let __mirrorverse: bool = { let __menu = compat::read(__this, |__t| __t.menu.clone()); compat::read(&__menu, |__m| __m.mirrorverse) };
        let __texture: Identifier = compat::read(if __mirrorverse { MIRRORVERSE_TEXTURE.get() } else { TEXTURE.get() }, |__t| __t.get());
        let (__iw, __ih) = compat::read(__this, |__t| (__t.image_width, __t.image_height));
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &__texture, i, j, 0.0f32, 0.0f32, __iw, __ih, 256, 256);
        // java: TEXTURE.isUsingFallback() && !menu.mirrorverse
        if compat::read(TEXTURE.get(), |__t| __t.is_using_fallback()) && !__mirrorverse {
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
