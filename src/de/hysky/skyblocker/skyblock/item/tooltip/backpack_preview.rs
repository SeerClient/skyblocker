//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.BackpackPreview by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::background::item_background_manager::ItemBackgroundManager;
use crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::utils::item_utils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::core::holder_lookup;
use mcrust::net::minecraft::core::non_null_list::NonNullList;
use mcrust::net::minecraft::nbt::nbt_io;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::resources::registry_ops::RegistryOps;
use mcrust::net::minecraft::world::container::Container;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::simple_container::SimpleContainer;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;

/// java: private static final Identifier TEXTURE = Identifier.withDefaultNamespace("textures/gui/container/generic_54.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/gui/container/generic_54.png"));
/// java: private static final Pattern ECHEST_PATTERN = Pattern.compile("Ender Chest.*\\((\\d+)/\\d+\\)", Pattern.CASE_INSENSITIVE);
pub static ECHEST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("Ender Chest.*\\((\\d+)/\\d+\\)", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern BACKPACK_PATTERN = Pattern.compile("Backpack.*\\(Slot #(\\d+)\\)", Pattern.CASE_INSENSITIVE);
pub static BACKPACK_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("Backpack.*\\(Slot #(\\d+)\\)", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern STRORAGE_PATTERN = Pattern.compile("Storage", Pattern.CASE_INSENSITIVE);
pub static STRORAGE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("Storage", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final Pattern BACKPACK_SIZE_PATTERN = Pattern.compile("has (\\d+) slots", Pattern.CASE_INSENSITIVE);
pub static BACKPACK_SIZE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile_with_flags("has (\\d+) slots", mcrust::java::util::regex::pattern::CASE_INSENSITIVE));
/// java: private static final int STORAGE_SIZE = 27;
pub const STORAGE_SIZE: i32 = 27;
/// java: private static final @Nullable Storage[] storages = new Storage[STORAGE_SIZE];
pub static STORAGES: compat::MainThread<Option<Vec<compat::Shared<Storage>>>> = compat::MainThread::new(|| Some((0..(STORAGE_SIZE) as usize).map(|_| Default::default()).collect::<Vec<_>>()));
/// java: private static String loaded;
pub static LOADED: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(Default::default()));
/// java: private static volatile Path saveDir;
pub static SAVE_DIR: compat::MainThread<Option<std::path::PathBuf>> = compat::MainThread::new(|| None);
/// java: private static final Codec<Storage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(Storage::name), ItemUtils.EMPTY_ALLOWING_ITEMSTACK_CODEC.listOf
pub static STORAGE_CODEC: compat::MainThread<CodecRef<compat::Shared<Storage>>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &compat::Shared<Storage>| Storage::name(o), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &compat::Shared<Storage>| Storage::get_item_list(o), codec::field_of(codec::list(item_utils::EMPTY_ALLOWING_ITEMSTACK_CODEC.get_mut().clone()), "items")), move |__a0, __a1| Storage::create(&compat::jstring(&__a0), &__a1))));

/// java: public class BackpackPreview
#[derive(Clone)]
pub struct BackpackPreview {}

impl BackpackPreview {
    /// java: private BackpackPreview()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            if let Some(handled_screen) = screen.as_any().downcast_ref::<AbstractContainerScreen>() {
                // TODO(fabric2seer): ScreenEvents.remove(..).register: no SeerClient equivalent yet
                //   java: ScreenEvents.remove(screen).register(_ -> updateStorage(handledScreen));
            }
        }));
    }

    /// java: public static void tick()
    pub fn tick() {
        if Utils::is_on_skyblock() {
            BackpackPreview::save_storages();
            let mut id: String = format!("{}/{}", compat::regex::replace_all(&compat::minecraft().get_user().get_profile_id(), "-", ""), Utils::get_profile_id());
            if !(id == compat::get_state(&LOADED)) {
                *SAVE_DIR.get_mut() = Some(skyblocker_mod::CONFIG_DIR.get_mut().join(&format!("backpack-preview/{}", id)));
                CompletableFuture::<()>::run_async(move || (|| {
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        std::fs::create_dir_all(SAVE_DIR.get_mut().as_ref().unwrap()).expect("IOException");
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("[Skyblocker] Failed to create the backpack preview save directory! Path: {}", compat::opt_str(&SAVE_DIR.get_mut().as_ref().map(|p| p.display().to_string()))));
                    }
                })(), compat::executor);
                compat::set_state(&LOADED, id.clone());
                BackpackPreview::load_storages();
            }
        }
    }

    /// java: private static void loadStorages()
    pub fn load_storages() {
        for index in 0..STORAGE_SIZE {
            STORAGES.get_mut().as_mut().unwrap()[(index) as usize] = None;
            let mut index2: i32 = index;
            // TODO(fabric2seer): an assignment used as a value
            //   java: CompletableFuture.supplyAsync(() -> {
            //   java: 				Path storageFile = saveDir.resolve(index2 + ".nbt");
            //   java:
            //   java: 				if (Files.isRegularFile(storageFile)) {
            //   java: 					try {
            //   java: 						return Storage.CODEC.parse(getOps(), NbtIo.read(storageFile)).getOrThrow();
            //   java: 					} catch (Exception e) {
            //   java: 						LOGGER.error("[Skyblocker] Failed to load backpack preview file: {}", storageFile.getFileName().toString(), e);
            //   java: 					}
            //   java: 				}
            //   java:
            //   java: 				return null;
            //   java: 			}, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).thenAcceptAsync(storage -> storages[index2] = storage, Minecraft.getInstance());
        }
    }

    /// java: private static RegistryOps<Tag> getOps()
    pub fn get_ops() -> RegistryOps {
        // TODO(fabric2seer): `create_serialization_context` takes 2 argument(s) in the port (`provider: Arc<dyn Provider>, parent: crate::com::mojang::serialization::dynamic_ops::Ops`), Java passes 1
        // rust: return holder_lookup::create_serialization_context(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private static void saveStorages()
    pub fn save_storages() {
        for index in 0..STORAGE_SIZE {
            if true /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] != null: never null here */ && compat::read(&STORAGES.get_mut().as_ref().unwrap()[(index) as usize], |__t| __t.dirty) {
                BackpackPreview::save_storage(index);
            }
        }
    }

    /// java: private static void saveStorage(int index)
    pub fn save_storage(mut index: i32) {
        let mut storage: compat::Shared<Storage> = STORAGES.get_mut().as_ref().unwrap()[(index) as usize].clone();
        (CompletableFuture::<()>::run_async(move || (|| {
            let mut storage_file: std::path::PathBuf = SAVE_DIR.get_mut().as_ref().unwrap().join(&format!("{}.nbt", index));
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): static `NbtIo.write` has no Rust counterpart in the port
                // rust: NbtIo::write((STORAGE_CODEC.get_mut().encode_start(&BackpackPreview::get_ops(), &storage)).get_or_throw().unwrap(), storage_file.clone());
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to save backpack preview file: {}", std::path::PathBuf::from(storage_file.file_name().unwrap_or_default()).display()));
            }
        })(), compat::executor)).then_run_async(|| Storage::mark_clean(&storage), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)));
    }

    /// java: private static void updateStorage(AbstractContainerScreen<?> handledScreen)
    pub fn update_storage(handled_screen: &AbstractContainerScreen) {
        let mut title: String = handled_screen.screen.get_title().to_string();
        let mut index: i32 = BackpackPreview::get_storage_index_from_title(&title);
        if index != -1 {
            // TODO(fabric2seer): field `slots` of a Object
            //   java: storages[index] = new Storage(handledScreen.getMenu().slots.getFirst().container, title, true);
        }
        if STRORAGE_PATTERN.matcher(&title).matches() {
            BackpackPreview::initialize_storage(handled_screen);
        }
    }

    /// java: private static void initializeStorage(AbstractContainerScreen<?> handledScreen)
    pub fn initialize_storage(handled_screen: &AbstractContainerScreen) {
        // TODO(fabric2seer): field `slots` of a Object
        //   java: NonNullList<Slot> slots = handledScreen.getMenu().slots;
        let mut slots: NonNullList<Slot> = todo!("fabric2seer: the initialiser of `slots` (TODO above)");
        for i in 9..18 {
            let slot = &slots[(i) as usize];
            let mut index: i32 = i.wrapping_sub(9);
            // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
            if slot.get_item().is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().red()) || true /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] != null: never null here */ {
                continue;
            }
            // TODO(fabric2seer): static call Stream.generate(..)
            //   java: storages[index] = new Storage(
            //   java: 					new SimpleContainer(Stream.generate(() -> ItemStack.EMPTY)
            //   java: 							.limit(18)
            //   java: 							.toArray(ItemStack[]::new)),
            //   java: 					"", true
            //   java: 			);
        }
        for i in 27..45 {
            let slot = &slots[(i) as usize];
            let mut index: i32 = i.wrapping_sub(18);
            // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
            if slot.get_item().is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().brown()) {
                STORAGES.get_mut().as_mut().unwrap()[(index) as usize] = None;
            }
            if true /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] != null: never null here */ {
                continue;
            }
            // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
            // rust: let mut size: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_contains_match(slot.get_item(), &BACKPACK_SIZE_PATTERN);
            let mut size: compat::regex::Matcher = todo!("fabric2seer: the initialiser of `size` (TODO above)");
            if true /* java: size != null: never null here */ {
                // TODO(fabric2seer): static call Stream.generate(..)
                //   java: storages[index] = new Storage(
                //   java: 						new SimpleContainer(Stream.generate(() -> ItemStack.EMPTY)
                //   java: 								.limit(NumberUtils.toInt(size.group(1)) + 9)
                //   java: 								.toArray(ItemStack[]::new)),
                //   java: 						"", true
                //   java: 				);
            }
        }
    }

    /// java: public static @Nullable Storage[] getStorages()
    pub fn get_storages() -> Option<Vec<compat::Shared<Storage>>> {
        return STORAGES.get_mut().clone();
    }

    /// java: public static boolean extractPreview(GuiGraphicsExtractor graphics, Screen screen, int index, int mouseX, int mouseY)
    pub fn extract_preview(graphics: &GuiGraphicsExtractor, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, mut index: i32, mut mouse_x: i32, mut mouse_y: i32) -> bool {
        if index >= 9 && index < 18 {
            index = index.wrapping_sub(9);
        } else if index >= 27 && index < 45 {
            index = index.wrapping_sub(18);
        } else {
            return false;
        }
        if false /* java: STORAGES.get_mut().as_ref().unwrap()[(index) as usize] == null: never null here */ {
            return false;
        }
        let mut rows: i32 = ((Storage::size(&STORAGES.get_mut().as_ref().unwrap()[(index) as usize])).wrapping_sub(9)).wrapping_div(9);
        let mut x: i32 = (if mouse_x.wrapping_add(184) >= screen.width { mouse_x.wrapping_sub(188) } else { mouse_x.wrapping_add(8) });
        let mut y: i32 = 0i32.max(mouse_y.wrapping_sub(16));
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 176, (rows.wrapping_mul(18)).wrapping_add(17), 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, (y.wrapping_add(rows.wrapping_mul(18))).wrapping_add(17), ((0) as f32) as f32, ((215) as f32) as f32, 176, 7, 256, 256);
        let mut text_renderer: Font = compat::font();
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&Storage::name(&STORAGES.get_mut().as_ref().unwrap()[(index) as usize])), x.wrapping_add(8), y.wrapping_add(6), 0xFF404040u32 as i32, false);
        for i in 9..Storage::size(&STORAGES.get_mut().as_ref().unwrap()[(index) as usize]) {
            let mut current_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = Storage::get_stack(&STORAGES.get_mut().as_ref().unwrap()[(index) as usize], i);
            let mut item_x: i32 = (x.wrapping_add(((i.wrapping_sub(9)).wrapping_rem(9)).wrapping_mul(18))).wrapping_add(8);
            let mut item_y: i32 = (y.wrapping_add(((i.wrapping_sub(9)).wrapping_div(9)).wrapping_mul(18))).wrapping_add(18);
            ItemBackgroundManager::draw_backgrounds(&current_stack, graphics, item_x, item_y);
            if ItemProtection::is_item_protected(Some(current_stack.clone())) {
                ItemProtection::draw_slot_icon(graphics, item_x, item_y);
            }
            graphics.item(&current_stack, item_x, item_y);
            graphics.item_decorations(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &current_stack, item_x, item_y);
            SlotTextManager::extract_slot_text_7(graphics, &text_renderer, None, &current_stack, i, item_x, item_y);
        }
        return true;
    }

    /// java: public static int getStorageIndexFromTitle(String title)
    pub fn get_storage_index_from_title(title: &str) -> i32 {
        let mut echest: compat::regex::Matcher = ECHEST_PATTERN.matcher(title);
        if echest.find() {
            return (echest.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_sub(1);
        }
        let mut backpack: compat::regex::Matcher = BACKPACK_PATTERN.matcher(title);
        if backpack.find() {
            return (backpack.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException")).wrapping_add(8);
        }
        return -1;
    }

}

/// java: public static class Storage
pub struct Storage {
    /// java: private final Container inventory;
    pub inventory: Box<dyn Container>,
    /// java: private final String name;
    pub name: String,
    /// java: private boolean dirty;
    pub dirty: bool,
}

impl Storage {
    /// java: private Storage(Container inventory, String name, boolean dirty)
    pub fn new(mut inventory: Box<dyn Container>, mut name: String, mut dirty: bool) -> compat::Shared<Self> {
        let __init_inventory = inventory.clone();
        let __init_name = name.clone();
        let __init_dirty = dirty;
        let __this = compat::shared(Self {
            inventory: __init_inventory,
            name: __init_name,
            dirty: __init_dirty,
        });
        __this
    }

    /// java: private static Storage create(String name, List<ItemStack> items)
    pub fn create(name: &str, items: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> compat::Shared<Storage> {
        let mut inventory: SimpleContainer = SimpleContainer::of(items.clone());
        // TODO(fabric2seer): the mod keeps the game's `SimpleContainer` (Java holds a reference to the live object); the port's `SimpleContainer` is not a value to copy and the port lends the mod no handle to it
        // rust: return Storage::new(inventory.clone(), name.to_string(), false);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private String name()
    pub fn name(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// java: public int size()
    pub fn size(__this: &compat::Shared<Self>) -> i32 {
        return __this.lock().borrow().inventory.get_container_size();
    }

    /// java: public ItemStack getStack(int index)
    pub fn get_stack(__this: &compat::Shared<Self>, mut index: i32) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return __this.lock().borrow().inventory.get_item(index).clone();
    }

    /// java: private void markClean()
    pub fn mark_clean(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.dirty = __v); }
    }

    /// java: private List<ItemStack> getItemList()
    pub fn get_item_list(__this: &compat::Shared<Self>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        for i in 0..Storage::size(__this) {
            items.push(Storage::get_stack(__this, i));
        }
        return items;
    }

}
