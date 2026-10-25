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
use mcrust::mcrust::modding::container_screens::ModContainerScreen;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_recipe_book_screen::AbstractRecipeBookScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::InventoryScreen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::mouse_handler;
use mcrust::net::minecraft::nbt::nbt_io;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;
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
    pub equipment_slots: Vec<Option<EquipmentSlot>>,
    /// java: private @Nullable ItemStack hoveredItem;
    pub hovered_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// Whether `added()` has run for this screen object (see `ModContainerScreen::init` below).
    pub added_ran: bool,
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
    /// `super(player)`: the `InventoryScreen` half is the game's own (`container_screens::register_inventory_screen`
    /// attaches this object to the `new InventoryScreen(player)` the client builds), so the player is not taken here;
    /// `base` is the copy of that half each call is lent (`compat::ContainerScreenAdapter::lend`).
    pub fn new() -> compat::Shared<Self> {
        // java: new SimpleContainer(getCurrentEquipmentSet()) - `NonNullList.of(EMPTY, items)` is `Arrays.asList(items)`,
        // a view OF the array: the slots read the static array live (an update arriving while the screen is open, or
        // the profile's file loading after it was built, shows), so the container half is the array itself here.
        let inventory: &'static compat::MainThread<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = SkyblockInventoryScreen::current_equipment_set();
        let textures: Vec<Identifier> = vec![EMPTY_NECKLACE.get_mut().clone(), EMPTY_CLOAK.get_mut().clone(), EMPTY_BELT.get_mut().clone(), EMPTY_HAND.get_mut().clone()];
        let __this = compat::shared(Self {
            base: InventoryScreen::new(&|key| compat::minecraft().label(key)),
            equipment_slots: vec![None, None, None, None],
            hovered_item: None,
            added_ran: false,
        });
        for i in 0..4usize {
            // java: equipmentSlots[i] = new EquipmentSlot(inventory, i, 77, 8 + i * 18, textures[i]);
            let __slot = EquipmentSlot::new(inventory, i as i32, 77, 8 + (i as i32) * 18, textures[i].clone());
            compat::write(&__this, |__t| __t.equipment_slots[i] = Some(__slot));
        }
        __this
    }

    /// `MinecraftMixin.skyblocker$skyblockInventoryScreen`'s answer: `Utils.isOnSkyblock() &&
    /// SkyblockerConfigManager.get().uiAndVisuals.showEquipmentInInventory ? new SkyblockInventoryScreen(player) :
    /// original.call(player)` - the factory `container_screens::register_inventory_screen` takes (`None` is
    /// `original.call(player)`).
    pub fn inventory_screen_factory() -> Option<Box<dyn ModContainerScreen>> {
        if Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.show_equipment_in_inventory {
            Some(Box::new(compat::ContainerScreenAdapter::new(SkyblockInventoryScreen::new())))
        } else {
            None
        }
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
            //   java: NbtIo.writeUnnamedTagWithFallback(CODEC.encodeStart(NbtOps.INSTANCE, combined).getOrThrow(), new DataOutputStream(Files.newOutputStream(resolve)));
            let encoded = CODEC.encode_start(&mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance(), &combined).get_or_throw().unwrap_or_else(|e| panic!("java.lang.IllegalStateException: {e}"));
            let tag: &mcrust::net::minecraft::nbt::tag::Tag = mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::nbt::tag::Tag>(&encoded).expect("NbtOps encodes a Tag");
            // `new DataOutputStream(Files.newOutputStream(resolve))`: the bytes are written to the file as the stream is
            let mut out: Vec<u8> = Vec::new();
            nbt_io::write_unnamed_tag_with_fallback(tag, &mut out).unwrap_or_else(|e| panic!("IOException: {e:?}"));
            std::fs::write(&resolve, &out).unwrap_or_else(|e| panic!("IOException: {e}"));
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
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)));
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
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> {
        //   java: 			String profileId = Utils.getProfileId();
        //   java: 			if (!profileId.isBlank()) save(profileId);
        //   java: 		});
        // (`CLIENT_STOPPING` is Fabric's HEAD injector on `Minecraft#stop`)
        let _ = mcrust::mcrust::modding::hook::inject("net.minecraft.client.Minecraft#stop", mcrust::mcrust::modding::hook::At::Head, |_: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            let profile_id: String = Utils::get_profile_id();
            if !compat::is_blank(&profile_id) {
                SkyblockInventoryScreen::save(&profile_id);
            }
        })
        .map_err(|e| logger().error(&format!("[Skyblocker] Failed to hook the client stop: {:?}", e)));
    }

    /// java: public static ItemStack[] getCurrentEquipmentSet() - the ARRAY itself (a reference: `SkyBlockEquipmentUpdater`
    /// `System.arraycopy`s into it and the screen's `SimpleContainer` is backed by it).
    pub fn current_equipment_set() -> &'static compat::MainThread<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> {
        if Utils::is_in_the_rift() {
            &EQUIPMENT_RIFT
        } else if Utils::is_in_safari() {
            &EQUIPMENT_SAFARI
        } else {
            &EQUIPMENT
        }
    }

    /// java: `getCurrentEquipmentSet()` for a caller that only READS the array (a copy of its elements). A caller
    /// that writes into it uses `current_equipment_set` - writing into this copy was lost, which is why the
    /// equipment slots never showed what `/equipment` held.
    pub fn get_current_equipment_set() -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        SkyblockInventoryScreen::current_equipment_set().get().clone()
    }

    /// java: @Override public void added()
    pub fn added(__this: &compat::Shared<Self>) {
        // java: Slot slot = menu.slots.get(45); slot.x += 21; - the screen's menu is the player's open (inventory) menu, and
        // the write is to that menu's slot object. When the opening call holds the player, the write runs as the client
        // thread's next task (before the screen's first frame).
        // (Java's `menu` is `InventoryScreen`'s - always `player.inventoryMenu`, never `player.containerMenu`, which may
        // already be another menu when the screen is added or removed)
        let shifted = with_inventory_menu_slot_45(|slot| slot.x = slot.x.wrapping_add(21));
        if shifted.is_none() {
            compat::execute(|| {
                let _ = with_inventory_menu_slot_45(|slot| slot.x = slot.x.wrapping_add(21));
            });
        }
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
        // java: return super.mouseClicked(click, doubled); - answering false hands the event to the game's
        // `InventoryScreen` body (`ModContainerScreen.mouse_clicked`'s convention)
        return false;
    }

    /// java: @Override protected void extractSlots(GuiGraphicsExtractor graphics, int mouseX, int mouseY)
    /// `super.extractSlots(..)` has run when this is called: the game's slot pass hands its last slot (45, the
    /// offhand) to `ModContainerScreen::extract_slot`, which runs the base body and then this.
    pub fn extract_slots(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32) {
        for equipment_slot in compat::read(__this, |__t| __t.equipment_slots.clone()).into_iter().flatten() {
            // java: boolean hovered = this.hoveredSlot == null && this.isHovering(equipmentSlot.x, equipmentSlot.y, 16, 16, mouseX, mouseY);
            let mut hovered: bool = compat::read(__this, |__t| __t.container.hovered_slot.is_none() && __t.container.is_hovering_box(equipment_slot.x, equipment_slot.y, 16, 16, mouse_x as f64, mouse_y as f64));
            if hovered {
                // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, AbstractContainerScreenAccessor.getSLOT_HIGHLIGHT_BACK_SPRITE(), equipmentSlot.x - 4, equipmentSlot.y - 4, 24, 24);
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::slot_highlight_back_sprite(), equipment_slot.x - 4, equipment_slot.y - 4, 24, 24);
            }
            // java: this.extractSlot(graphics, equipmentSlot, mouseX, mouseY);
            SkyblockInventoryScreen::extract_slot(__this, graphics, &equipment_slot);
            if hovered {
                // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, AbstractContainerScreenAccessor.getSLOT_HIGHLIGHT_FRONT_SPRITE(), equipmentSlot.x - 4, equipmentSlot.y - 4, 24, 24);
                gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::slot_highlight_front_sprite(), equipment_slot.x - 4, equipment_slot.y - 4, 24, 24);
            }
        }
    }

    /// `AbstractContainerScreen.extractSlot(graphics, slot, mouseX, mouseY):197-249` for an `EquipmentSlot`: the slot
    /// is not in the menu (no quick-craft drag can include it, it is never fake), so the body is `:227-233`'s empty-slot
    /// icon, or `:240-246`'s item at the position seed with its decorations.
    pub fn extract_slot(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, slot: &EquipmentSlot) {
        let x = slot.x;
        let y = slot.y;
        let item_stack = slot.get_item();
        if item_stack.is_empty() {
            // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, icon, x, y, 16, 16);
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &slot.get_no_item_icon(), x, y, 16, 16);
            return;
        }
        // java: int seed = slot.x + slot.y * this.imageWidth; graphics.item(itemStack, x, y, seed);
        // `AbstractContainerScreenMixin.skyblocker$drawOnItem` (@At INVOKE `graphics.item`): this IS the mixed-in
        // `extractSlot` in Java, so the rarity background, the protection icon and the slot text apply here too.
        crate::mixins::abstract_container_screen_mixin::draw_on_item(&item_stack, x, y, None);
        let seed = abstract_container_screen::item_render_seed(x, y, compat::read(__this, |__t| __t.container.image_width));
        graphics.item_with_seed(&item_stack, x, y, seed);
        // `skyblocker$drawSlotText` (@At INVOKE `itemDecorations`) - `slot.index` of a slot no menu added is 0
        if Utils::is_on_skyblock() {
            crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager::extract_slot_text_7(graphics, &compat::font(), None, &item_stack, 0, x, y);
        }
        // java: graphics.itemDecorations(this.font, itemStack, x, y, itemCount); - `itemCount` is null outside a drag
        graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &item_stack, x, y);
    }

    /// java: @Override protected void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    /// `super.extractTooltip(..)` has run when this is called (`ModContainerScreen::extract_render_state` runs after
    /// the base's `extractRenderState`, whose last statement is `extractTooltip`).
    pub fn extract_tooltip(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        { let __v = None; compat::write(__this, |__t| __t.hovered_item = __v); }
        // java: !menu.getCarried().isEmpty() - the screen's menu is the player's open one
        if compat::with_open_menu(|menu, _| !menu.get_carried().is_empty()).unwrap_or(false) {
            return;
        }
        for equipment_slot in compat::read(__this, |__t| __t.equipment_slots.clone()).into_iter().flatten() {
            if { let (__q0, __q1, __q2, __q3, __q4, __q5,) = (equipment_slot.x, equipment_slot.y, 16, 16, (x) as f64, (y) as f64,); compat::read(__this, |__t| __t.container.is_hovering_box(__q0, __q1, __q2, __q3, __q4, __q5)) } && equipment_slot.has_item() {
                let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = equipment_slot.get_item();
                // java: graphics.setTooltipForNextFrame(this.font, itemStack, x, y);
                // (the frame's deferred slot: `compat::batch_s_set_tooltip_for_next_frame`, lines only, default style)
                {
                    let level = mcrust::mcrust::modding::game::level();
                    let flag = mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::tooltip_flag_from_options();
                    let lines = mcrust::mcrust::modding::game::with_player(|player| {
                        mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::get_tooltip_from_item(&item_stack, level.as_deref(), Some(&*player as &dyn mcrust::net::minecraft::world::entity::player::player::Player), flag)
                    })
                    .unwrap_or_else(|| mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::get_tooltip_from_item(&item_stack, level.as_deref(), None, flag));
                    // `setTooltipForNextFrame(font, itemStack, x, y):1107-1111` passes `itemStack.get(TOOLTIP_STYLE)`
                    compat::batch_s_set_tooltip_for_next_frame_styled(lines, mcrust::net::minecraft::client::gui::gui_graphics_extractor::item_tooltip_style(&item_stack));
                }
                { let __v = Some(item_stack.clone()); compat::write(__this, |__t| __t.hovered_item = __v); }
            }
        }
    }

    /// java: @Override public void removed()
    /// `super.removed()` has run when this is called (`ModContainerScreen::removed` runs after the base's).
    pub fn removed(__this: &compat::Shared<Self>) {
        // java: Slot slot = menu.slots.get(45); slot.x -= 21;
        let _ = with_inventory_menu_slot_45(|slot| slot.x = slot.x.wrapping_sub(21));
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // java: super.extractBackground(graphics, mouseX, mouseY, a); - the game's `InventoryScreen.extractBackground`
        compat::call_super();
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
                    if mcrust::mcrust::modding::game::with_player(|player| WikiLookupManager::handle_wiki_lookup(&Either::right(equipment_slot.get_item()), player, input)).unwrap_or(false) {
                        return true;
                    }
                }
            }
        }
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

/// SkyblockInventoryScreen's screen half (`super`'s fields), lent by the game for each call
impl compat::ContainerHalf for SkyblockInventoryScreen {
    fn container_half(&mut self) -> &mut abstract_container_screen::AbstractContainerScreen {
        &mut self.base.container
    }
}

/// SkyblockInventoryScreen as the game's `InventoryScreen` subclass (`container_screens::register_inventory_screen`'s
/// answer, `SkyblockInventoryScreen::inventory_screen_factory`)
impl ModContainerScreen for compat::ContainerScreenAdapter<SkyblockInventoryScreen> {
    /// java: @Override public void added()
    fn added(&mut self, screen: &mut abstract_container_screen::AbstractContainerScreen) {
        self.lend(screen, |__h| SkyblockInventoryScreen::added(__h));
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, screen: &abstract_container_screen::AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_background: &mut dyn FnMut()) {
        compat::with_super(super_extract_background, || self.lend_ref(screen, |__h| SkyblockInventoryScreen::extract_background(__h, &context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: @Override protected void extractTooltip(GuiGraphicsExtractor graphics, int x, int y) - the last statement
    /// of the base's `extractRenderState`, after which this runs
    fn extract_render_state(&mut self, screen: &abstract_container_screen::AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = a;
        self.lend_ref(screen, |__h| SkyblockInventoryScreen::extract_tooltip(__h, &mut context.graphics, mouse_x, mouse_y))
    }

    /// java: @Override protected void extractSlots(GuiGraphicsExtractor graphics, int mouseX, int mouseY) - the base pass
    /// draws each menu slot; after its last one (45) the override's own loop runs
    fn extract_slot(&mut self, screen: &mut abstract_container_screen::AbstractContainerScreen, context: &mut WidgetRenderContext<'_>, slot: &Slot, mouse_x: i32, mouse_y: i32, super_extract_slot: &mut dyn FnMut(&mut abstract_container_screen::AbstractContainerScreen)) {
        super_extract_slot(screen);
        if slot.index == 45 {
            self.lend(screen, |__h| SkyblockInventoryScreen::extract_slots(__h, &context.graphics, mouse_x, mouse_y));
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut abstract_container_screen::AbstractContainerScreen, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(screen, |__h| SkyblockInventoryScreen::mouse_clicked(__h, event, double_click))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, screen: &mut abstract_container_screen::AbstractContainerScreen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| SkyblockInventoryScreen::key_pressed(__h, event))
    }

    /// java: @Override public void removed()
    fn removed(&mut self, screen: &mut abstract_container_screen::AbstractContainerScreen) {
        self.lend(screen, |__h| SkyblockInventoryScreen::removed(__h))
    }
}

/// java: private static class EquipmentSlot extends Slot
/// A slot over the screen's own `SimpleContainer` that Java never adds to the menu (it lives only in
/// `equipmentSlots`): `Slot`'s `container`, `slot` (its index), `x` and `y` are held here and the screen reads them.
#[derive(Clone)]
pub struct EquipmentSlot {
    /// java: Slot.container - the `SimpleContainer` over the equipment array (`Arrays.asList`, a view of it)
    pub container: &'static compat::MainThread<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>,
    /// java: Slot.slot
    pub slot: i32,
    /// java: Slot.x
    pub x: i32,
    /// java: Slot.y
    pub y: i32,
    /// java: private final Identifier noItemIcon;
    pub no_item_icon: Identifier,
}

impl EquipmentSlot {
    /// java: private EquipmentSlot(Container inventory, int index, int x, int y, Identifier noItemIcon)
    pub fn new(inventory: &'static compat::MainThread<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>>, index: i32, x: i32, y: i32, no_item_icon: Identifier) -> Self {
        // java: super(inventory, index, x, y);
        Self { container: inventory, slot: index, x, y, no_item_icon }
    }

    /// `Slot.getItem()`: `this.container.getItem(this.slot)` (`SimpleContainer.getItem:63-65` - `EMPTY` out of range)
    pub fn get_item(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        self.container.get().get(self.slot as usize).cloned().unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty)
    }

    /// `Slot.hasItem()`: `!this.getItem().isEmpty()`
    pub fn has_item(&self) -> bool {
        !self.get_item().is_empty()
    }

    /// java: @Override public boolean mayPickup(Player playerEntity)
    pub fn may_pickup(&self, player_entity: &dyn Player) -> bool {
        return false;
    }

    /// java: @Override public boolean mayPlace(ItemStack stack)
    pub fn may_place(&self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return false;
    }

    /// java: @Override public Identifier getNoItemIcon()
    pub fn get_no_item_icon(&self) -> Identifier {
        return self.no_item_icon.clone();
    }
}

/// `menu.slots.get(45)` of `InventoryScreen`'s menu: the player's `inventoryMenu` (Java's `InventoryScreen(player)`
/// passes `player.inventoryMenu`), whatever `containerMenu` is at the time.
fn with_inventory_menu_slot_45<R>(f: impl FnOnce(&mut mcrust::net::minecraft::world::inventory::slot::Slot) -> R) -> Option<R> {
    use mcrust::net::minecraft::world::entity::player::player::Player as _;
    mcrust::mcrust::modding::game::with_player(|player| {
        let (_, inventory_menu, _) = player.player_mut().get_container_menu_and_inventory();
        inventory_menu.menu.slots.get_mut(45).map(f)
    })
    .flatten()
}
