//! Ported from de.hysky.skyblocker.skyblock.item.SkyblockInventoryScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::utils::hovered_item::hovered_item_stack_provider::HoveredItemStackProvider;
use crate::de::hysky::skyblocker::utils::item_utils;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::mcrust::modding::container_menus::ModSlot;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_recipe_book_screen::AbstractRecipeBookScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::InventoryScreen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::mouse_handler;
use mcrust::net::minecraft::nbt::nbt_io;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::container::Container;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::simple_container::SimpleContainer;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: private static final Supplier<ItemStack[]> EMPTY_EQUIPMENT = () -> new ItemStack[]{ItemStack.EMPTY, ItemStack.EMPTY, ItemStack.EMPTY, ItemStack.EMPTY};
pub static EMPTY_EQUIPMENT: compat::MainThread<Arc<dyn Fn() -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>> = compat::MainThread::new(|| Arc::new(move || vec![mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty()]));
/// java: public static final ItemStack[] EQUIPMENT = EMPTY_EQUIPMENT.get();
pub static EQUIPMENT: compat::MainThread<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = compat::MainThread::new(|| (EMPTY_EQUIPMENT.get_mut())());
/// java: private static final ItemStack[] EQUIPMENT_RIFT = EMPTY_EQUIPMENT.get();
pub static EQUIPMENT_RIFT: compat::MainThread<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = compat::MainThread::new(|| (EMPTY_EQUIPMENT.get_mut())());
/// java: private static final ItemStack[] EQUIPMENT_SAFARI = EMPTY_EQUIPMENT.get();
pub static EQUIPMENT_SAFARI: compat::MainThread<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = compat::MainThread::new(|| (EMPTY_EQUIPMENT.get_mut())());
/// java: private static final Codec<ItemStack[]> CODEC = ItemUtils.EMPTY_ALLOWING_ITEMSTACK_CODEC.listOf(4, 12) // min size at 4 for backwards compat
///   .xmap(itemStacks -> itemStacks.toArray(ItemStack[]::new), List::of).fieldOf("items").codec();
/// (the array and the list are the same `Vec` here, so the xmap is the identity both ways; a `LazyLock` because
/// `load` parses on a worker)
pub static CODEC: std::sync::LazyLock<CodecRef<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>> = std::sync::LazyLock::new(|| {
    mcrust::com::mojang::serialization::map_codec::codec(codec::field_of(
        codec::xmap(
            codec::list_sized(crate::de::hysky::skyblocker::utils::item_utils::EMPTY_ALLOWING_ITEMSTACK_CODEC.get().clone(), 4, 12),
            |item_stacks: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>| item_stacks,
            |item_stacks: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>| item_stacks.clone(),
        ),
        "items",
    ))
});
/// java: private static final Identifier SLOT_TEXTURE = Identifier.withDefaultNamespace("container/slot");
pub static SLOT_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("container/slot"));
/// java: private static final Identifier EMPTY_NECKLACE = SkyblockerMod.id("equipment/empty_necklace");
pub static EMPTY_NECKLACE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("equipment/empty_necklace"));
/// java: private static final Identifier EMPTY_CLOAK = SkyblockerMod.id("equipment/empty_cloak");
pub static EMPTY_CLOAK: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("equipment/empty_cloak"));
/// java: private static final Identifier EMPTY_BELT = SkyblockerMod.id("equipment/empty_belt");
pub static EMPTY_BELT: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("equipment/empty_belt"));
/// java: private static final Identifier EMPTY_HAND = SkyblockerMod.id("equipment/empty_hand");
pub static EMPTY_HAND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("equipment/empty_hand"));
/// java: private static final Path FOLDER = SkyblockerMod.CONFIG_DIR.resolve("equipment");
pub static FOLDER: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("equipment"));
/// java: public static final FallbackedTexture<Identifier> BACKGROUND = FallbackedTexture.ofTexture( SkyblockerMod.id("textures/gui/container/skyblock_inventory.png"), INVENTORY_LOCATION);
pub static BACKGROUND: compat::MainThread<compat::Shared<dyn FallbackedTexture<Identifier>>> = compat::MainThread::new(|| fallbacked_texture::of_texture(&crate::entry::skyblocker_mod::id("textures/gui/container/skyblock_inventory.png"), &abstract_container_screen::inventory_location()));

/// java: @SuppressWarnings("JavadocReference") public class SkyblockInventoryScreen extends InventoryScreen implements HoveredItemStackProvider
pub struct SkyblockInventoryScreen {
    /// java: extends net.minecraft.client.gui.screens.inventory.InventoryScreen
    pub base: InventoryScreen,
    /// java: private final Slot[] equipmentSlots = new Slot[4];
    /// (`None` is the array's null before the constructor's loop fills it)
    pub equipment_slots: Vec<Option<Slot>>,
    /// java: private @Nullable ItemStack hoveredItem;
    pub hovered_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
}

impl std::ops::Deref for SkyblockInventoryScreen {
    type Target = InventoryScreen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockInventoryScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockInventoryScreen {
    /// java: public SkyblockInventoryScreen(Player player)
    pub fn new(mut player: Box<dyn Player>) -> compat::Shared<Self> {
        let mut inventory: SimpleContainer = SimpleContainer::of(SkyblockInventoryScreen::get_current_equipment_set());
        let mut textures: Vec<Identifier> = vec![EMPTY_NECKLACE.get_mut().clone(), EMPTY_CLOAK.get_mut().clone(), EMPTY_BELT.get_mut().clone(), EMPTY_HAND.get_mut().clone()];
        let __this = compat::shared(Self {
            // super(player): the port's InventoryScreen takes the two translated captions; the menu it reads is the
            // player's `inventoryMenu`, which the game lends per call
            base: InventoryScreen::new(&|key| compat::minecraft().label(key)),
            equipment_slots: vec![None, None, None, None],
            hovered_item: None,
        });
        for i in 0..4 {
            // rust: port gap - `new Slot(inventory, ..)` over a mod-owned `SimpleContainer`: the port's `Slot.container`
            //   is a `SlotContainer` tag naming one of the game's own containers, and it has no tag for a container a mod
            //   builds (GAPS_batchB.md). The slots stay null until it has one.
            // rust: __this.lock().borrow_mut().equipment_slots[(i) as usize] = compat::slot_with_class(EquipmentSlot::new(inventory.clone(), i, 77, 8i32.wrapping_add(i.wrapping_mul(18)), textures[(i) as usize].clone()));
        }
        __this
    }

    /// java: private static void save(String profileId)
    pub fn save(profile_id: &str) {
        // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            std::fs::create_dir_all(FOLDER.get_mut()).expect("IOException");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to create folder for equipment!"));
        }
        let mut resolve: std::path::PathBuf = FOLDER.get_mut().join(&format!("{}.nbt", profile_id));
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut combined: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = vec![EQUIPMENT.get_mut().clone(), EQUIPMENT_RIFT.get_mut().clone(), EQUIPMENT_SAFARI.get_mut().clone()].into_iter().flatten().collect::<Vec<_>>();
            // TODO(fabric2seer): `new DataOutputStream(..)`
            //   java: NbtIo.writeUnnamedTagWithFallback(CODEC.encodeStart(NbtOps.INSTANCE, combined).getOrThrow(), new DataOutputStream(Files.newOutputStream(resolve)));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to save Equipment data"));
        }
    }

    /// java: private static void load(String profileId)
    pub fn load(profile_id: &str) {
        let mut resolve: std::path::PathBuf = FOLDER.get_mut().join(&format!("{}.nbt", profile_id));
        CompletableFuture::supply_async(move || {
            if !resolve.exists() {
                return Ok(vec![mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty()]);
            }
            // try { return CODEC.parse(NbtOps.INSTANCE, NbtIo.read(resolve)).getOrThrow(); }
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, Option<String>> {
                // catch (NoSuchFileException _) {} - the file went away after the check: nothing logged
                let bytes = std::fs::read(&resolve).map_err(|e| if e.kind() == std::io::ErrorKind::NotFound { None } else { Some(e.to_string()) })?;
                let tag = nbt_io::read(&bytes).map_err(Some)?.ok_or_else(|| Some("empty file".to_string()))?;
                CODEC.parse(&mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance(), &mcrust::com::mojang::serialization::obj(tag)).get_or_throw().map_err(|e| Some(e.to_string()))
            }));
            match outcome {
                Ok(Ok(item_stacks)) => return Ok(item_stacks),
                Ok(Err(None)) => {}
                // catch (Exception e)
                Ok(Err(Some(e))) => logger().error(&format!("[Skyblocker] Failed to load Equipment data {}", e)),
                Err(e) => logger().error(&format!("[Skyblocker] Failed to load Equipment data {}", compat::panic_message(&e))),
            }
            return Ok(vec![mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty(), mcrust::net::minecraft::world::item::item_stack::empty()]);
            // Schedule on main thread to avoid any async weirdness
        }, compat::executor).then_accept_async(|item_stacks: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>| {
            // System.arraycopy(itemStacks, 0, EQUIPMENT, 0, Math.min(itemStacks.length, 4));
            let n = item_stacks.len().min(4);
            EQUIPMENT.get_mut()[..n].clone_from_slice(&item_stacks[..n]);
            if item_stacks.len() <= 4 {
                return;
            }
            // System.arraycopy(itemStacks, 4, EQUIPMENT_RIFT, 0, Math.clamp(itemStacks.length - 4, 0, 4));
            let n = (item_stacks.len() - 4).clamp(0, 4);
            EQUIPMENT_RIFT.get_mut()[..n].clone_from_slice(&item_stacks[4..4 + n]);
            if item_stacks.len() <= 8 {
                return;
            }
            // System.arraycopy(itemStacks, 8, EQUIPMENT_SAFARI, 0, Math.clamp(itemStacks.length - 8, 0, 4));
            let n = (item_stacks.len() - 8).clamp(0, 4);
            EQUIPMENT_SAFARI.get_mut()[..n].clone_from_slice(&item_stacks[8..8 + n]);
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)));
    }

    /// java: @Init public static void initEquipment()
    pub fn init_equipment() {
        skyblock_events::PROFILE_CHANGE.register(Arc::new(move |prev_profile_id, profile_id| {
            if !prev_profile_id.is_empty() {
                CompletableFuture::<()>::run_async(move || (|| {
                    SkyblockInventoryScreen::save(&prev_profile_id);
                    SkyblockInventoryScreen::load(&profile_id);
                })(), compat::executor);
            } else {
                SkyblockInventoryScreen::load(&profile_id);
            }
        }));
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> {
        //   java: 			String profileId = Utils.getProfileId();
        //   java: 			if (!profileId.isBlank()) save(profileId);
        //   java: 		});
    }

    /// java: public static ItemStack[] getCurrentEquipmentSet()
    pub fn get_current_equipment_set() -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        if Utils::is_in_the_rift() {
            return EQUIPMENT_RIFT.get_mut().clone();
        } else if Utils::is_in_safari() {
            return EQUIPMENT_SAFARI.get_mut().clone();
        } else {
            return EQUIPMENT.get_mut().clone();
        }
    }

    /// java: @Override public void added()
    pub fn added(__this: &compat::Shared<Self>) {
        // java: Slot slot = menu.slots.get(45); slot.x += 21; - the screen's menu is the player's open (inventory) menu, and
        // the write is to that menu's slot object
        mcrust::mcrust::modding::game::with_abstract_container_menu(|menu| { let slot = &mut menu.slots[45]; slot.x = slot.x.wrapping_add(21); });
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        for equipment_slot in compat::read(__this, |__t| __t.equipment_slots.clone()).into_iter().flatten() {
            if { let (__q0, __q1, __q2, __q3, __q4, __q5,) = (equipment_slot.x, equipment_slot.y, 16, 16, click.x(), click.y(),); compat::read(__this, |__t| __t.container.is_hovering_box(__q0, __q1, __q2, __q3, __q4, __q5)) } {
                let mut command: String = (if SkyblockerConfigManager::get().ui_and_visuals.skyblock_inventory_screen.open_equipment_to_stats_page || Utils::is_in_the_rift() || Utils::is_in_safari() { "/stats" } else { "/equipment" }).to_string();
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &command, true);
                return true;
            }
        }
        // java: return super.mouseClicked(click, doubled); - a container screen override answering false hands the event to
        // the base body (`ModContainerScreen.mouse_clicked`'s convention)
        // TODO(fabric2seer): `mouse_clicked` takes 10 argument(s) in the port (`host: &mut acs::PlayerContainerHost<'_>, button: i32, x: f64, y: f64, double_click: bool, shift_down: bool, cloning: bool, is_spectator: bool, gfx: &crate::net::minecraft::client::minecraft::Minecraft, handle_place_recipe: &mut dyn FnMut(i3 ..`), Java passes 2
        // rust: return __this.lock().borrow_mut().base.mouse_clicked(&mut click, doubled);
        return false;
    }

    /// java: @Override protected void extractSlots(GuiGraphicsExtractor graphics, int mouseX, int mouseY)
    pub fn extract_slots(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32) {
        // TODO(fabric2seer): `extract_slots` takes 7 argument(s) in the port (`recipe_book_component: &RecipeBookComponent, is_bigger_result_slot: bool, gl: &crate::lwjgl::opengl::Gl, gfx: &Minecraft, slot_position: &dyn Fn(usize) -> (i32, i32), resolver: Option< &crate::net::minecraft::client::renderer::item::item_mo ..`), Java passes 3
        // rust: AbstractRecipeBookScreen::extract_slots(graphics, mouse_x, &mouse_y);
        for equipment_slot in compat::read(__this, |__t| __t.equipment_slots.clone()).into_iter().flatten() {
            // TODO(fabric2seer): `is_hovering` takes 1 argument(s) in the port (`super_says_hovering: bool`), Java passes 6
            // rust: let mut hovered: bool = (compat::read(__this, |__t| __t.container.hovered_slot.clone()).is_none() && { let (__q0, __q1, __q2, __q3, __q4, __q5,) = (equipment_slot.x, equipment_slot.y, 16, 16, (mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_hovering(__q0, __q1, __q2, __q3, __q4, __q5) });
            // java: boolean hovered = this.hoveredSlot == null && this.isHovering(equipmentSlot.x, equipmentSlot.y, 16, 16, mouseX, mouseY);
            let mut hovered: bool = compat::read(__this, |__t| __t.container.hovered_slot.is_none() && __t.container.is_hovering_box(equipment_slot.x, equipment_slot.y, 16, 16, mouse_x as f64, mouse_y as f64));
            if hovered {
                // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, AbstractContainerScreenAccessor.getSLOT_HIGHLIGHT_BACK_SPRITE(), equipmentSlot.x - 4, equipmentSlot.y - 4, 24, 24);
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::slot_highlight_back_sprite(), equipment_slot.x - 4, equipment_slot.y - 4, 24, 24);
            }
            // TODO(fabric2seer): `extract_slot` takes 11 argument(s) in the port (`gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, state: &mut AbstractContainerScreen, menu: &AbstractContainerMenu, containers: &mut Containers<'_>, index: i32, resolver: Option< &crate::net::minecra ..`), Java passes 4
            // rust: abstract_container_screen::extract_slot(graphics, &equipment_slot, &mut mouse_x, &mouse_y);
            if hovered {
                // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, AbstractContainerScreenAccessor.getSLOT_HIGHLIGHT_FRONT_SPRITE(), equipmentSlot.x - 4, equipmentSlot.y - 4, 24, 24);
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::slot_highlight_front_sprite(), equipment_slot.x - 4, equipment_slot.y - 4, 24, 24);
            }
        }
    }

    /// java: @Override protected void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_tooltip(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        // TODO(fabric2seer): `extract_tooltip` takes 4 argument(s) in the port (`state: &AbstractContainerScreen, menu: &AbstractContainerMenu, containers: &Containers<'_>, carried: &crate::net::minecraft::world::item::item_stack::ItemStack`), Java passes 3
        // rust: abstract_container_screen::extract_tooltip(graphics, &x, &y);
        { let __v = None; compat::write(__this, |__t| __t.hovered_item = __v); }
        // java: !menu.getCarried().isEmpty() - the screen's menu is the player's open one
        if compat::with_open_menu(|menu, _| !menu.get_carried().is_empty()).unwrap_or(false) {
            return;
        }
        for equipment_slot in compat::read(__this, |__t| __t.equipment_slots.clone()).into_iter().flatten() {
            if { let (__q0, __q1, __q2, __q3, __q4, __q5,) = (equipment_slot.x, equipment_slot.y, 16, 16, (x) as f64, (y) as f64,); compat::read(__this, |__t| __t.container.is_hovering_box(__q0, __q1, __q2, __q3, __q4, __q5)) } && compat::slot_has_item(&equipment_slot) {
                let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(&equipment_slot);
                // TODO(fabric2seer): `GuiGraphicsExtractor.setTooltipForNextFrame` has no Rust method in the port (looked for `set_tooltip_for_next_frame` in GuiGraphicsExtractor and its supertypes)
                // rust: graphics.set_tooltip_for_next_frame(&compat::font(), &item_stack, x, y);
                { let __v = Some(item_stack.clone()); compat::write(__this, |__t| __t.hovered_item = __v); }
            }
        }
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `removed` takes 1 argument(s) in the port (`menu: &mut AbstractContainerMenu`), Java passes 0
        // rust: __this.lock().borrow_mut().base.container.removed();
        // java: Slot slot = menu.slots.get(45); slot.x -= 21;
        mcrust::mcrust::modding::game::with_abstract_container_menu(|menu| { let slot = &mut menu.slots[45]; slot.x = slot.x.wrapping_sub(21); });
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `InventoryScreen.extractBackground` has no Rust method in the port (looked for `extract_background` in InventoryScreen and its supertypes)
        // rust: __this.lock().borrow_mut().base.extract_background(graphics, mouse_x, mouse_y, a);
        if !(BACKGROUND.get_mut().lock().borrow().is_using_fallback()) {
            return;
        }
        for i in 0..3i32 {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_TEXTURE.get_mut(), (compat::read(__this, |__t| __t.container.left_pos)).wrapping_add(76), ((compat::read(__this, |__t| __t.container.top_pos)).wrapping_add(7)).wrapping_add(i.wrapping_mul(18)), 18, 18);
        }
        // java: Slot slot = menu.slots.get(45); - only its position is read
        let (__slot_x, __slot_y): (i32, i32) = compat::with_open_menu(|menu, _| (menu.slots[45].x, menu.slots[45].y)).unwrap_or((0, 0));
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SLOT_TEXTURE.get_mut(), ((compat::read(__this, |__t| __t.container.left_pos)).wrapping_add(__slot_x)).wrapping_sub(1), ((compat::read(__this, |__t| __t.container.top_pos)).wrapping_add(__slot_y)).wrapping_sub(1), 18, 18);
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let client = compat::minecraft();
        if client.is_window_active() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            let mouse = &client.mouse_handler;
            let window = client.get_window();
            let mut mouse_x = mouse_handler::get_scaled_x_pos(window, mouse.xpos());
            let mut mouse_y = mouse_handler::get_scaled_y_pos(window, mouse.ypos());
            for equipment_slot in compat::read(__this, |__t| __t.equipment_slots.clone()).into_iter().flatten() {
                if compat::read(__this, |__t| __t.container.is_hovering_box(equipment_slot.x, equipment_slot.y, 16, 16, mouse_x, mouse_y)) {
                    if mcrust::mcrust::modding::game::with_player(|player| WikiLookupManager::handle_wiki_lookup(&Either::left(equipment_slot.clone()), player, input)).unwrap_or(false) {
                        return true;
                    }
                }
            }
        }
        // TODO(fabric2seer): `key_pressed` takes 6 argument(s) in the port (`event: crate::net::minecraft::client::input::key_event::KeyEvent, matches_chat_key: bool, is_spectator: bool, gfx: &crate::net::minecraft::client::minecraft::Minecraft, clipboard: &mut dyn crate::net::minecraft::client::gui::components::edi ..`), Java passes 1
        // rust: return { let (__q0,) = (input.clone(),); __this.lock().borrow_mut().base.key_pressed(__q0) };
        // java: return super.keyPressed(input); - answering false hands the key to the base body (the
        // `ModContainerScreen.key_pressed` convention)
        return false;
    }

}

impl HoveredItemStackProvider for SkyblockInventoryScreen {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable ItemStack getFocusedItem() (dispatch: the body for this class)
    fn get_focused_item(&self) -> fn(&compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        SkyblockInventoryScreen::get_focused_item_dyn
    }

}

impl SkyblockInventoryScreen {
    /// java: @Override public @Nullable ItemStack getFocusedItem()
    pub fn get_focused_item_impl(__this: &compat::Shared<Self>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return compat::read(__this, |__t| __t.hovered_item.clone());
    }

    /// SkyblockInventoryScreen.getFocusedItem as the dispatch of HoveredItemStackProvider.getFocusedItem
    pub fn get_focused_item_dyn(__this: &compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::get_focused_item_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

// NOTE(fabric2seer): added, extractBackground, extractSlots, extractTooltip, keyPressed, mouseClicked, removed override(s) of InventoryScreen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private static class EquipmentSlot extends Slot
#[derive(Clone)]
pub struct EquipmentSlot {
    /// java: extends net.minecraft.world.inventory.Slot
    pub base: Slot,
    /// java: private final Identifier noItemIcon;
    pub no_item_icon: Identifier,
}

impl std::ops::Deref for EquipmentSlot {
    type Target = Slot;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EquipmentSlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EquipmentSlot {
    /// java: private EquipmentSlot(Container inventory, int index, int x, int y, Identifier noItemIcon)
    /// `inventory` is the port's `Slot.container` (a `SlotContainer` tag; see the gap in the outer constructor).
    pub fn new(mut inventory: mcrust::net::minecraft::world::inventory::slot::SlotContainer, mut index: i32, mut x: i32, mut y: i32, mut no_item_icon: Identifier) -> compat::Shared<Self> {
        let __init_no_item_icon = no_item_icon.clone();
        let __this = compat::shared(Self {
            base: Slot::new(inventory, index, x, y),
            no_item_icon: __init_no_item_icon,
        });
        __this
    }

    /// java: @Override public boolean mayPickup(Player playerEntity)
    pub fn may_pickup(__this: &compat::Shared<Self>, player_entity: &dyn Player) -> bool {
        return false;
    }

    /// java: @Override public boolean mayPlace(ItemStack stack)
    pub fn may_place(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return false;
    }

    /// java: @Override public Identifier getNoItemIcon()
    pub fn get_no_item_icon(__this: &compat::Shared<Self>) -> Identifier {
        return compat::read(__this, |__t| __t.no_item_icon.clone());
    }

}

/// EquipmentSlot's slot half (`super`'s fields), the slot's state when the port asks the class
impl compat::SlotHalf for EquipmentSlot {
    fn slot_half(&mut self) -> &mut Slot {
        &mut self.base
    }
}

/// EquipmentSlot's overrides, asked by the port's slot (`compat::slot_with_class` attaches them)
impl ModSlot for compat::SlotClass<EquipmentSlot> {
    /// java: @Override public boolean mayPlace(ItemStack stack)
    fn may_place(&self, slot: &Slot, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, super_answer: bool) -> bool {
        self.ask(slot, super_answer, |__h| EquipmentSlot::may_place(__h, item_stack))
    }

}

// NOTE(fabric2seer): getNoItemIcon, mayPickup, mayPlace override(s) of Slot - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
