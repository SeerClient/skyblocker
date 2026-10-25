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
/// (`None` is an absent storage - the array's null element)
pub static STORAGES: compat::MainThread<Vec<Option<compat::Shared<Storage>>>> = compat::MainThread::new(|| vec![None; STORAGE_SIZE as usize]);
/// java: private static String loaded;
pub static LOADED: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(Default::default()));
/// java: private static volatile Path saveDir;
/// (volatile: the workers read it, so it is a lock rather than a render-thread static)
pub static SAVE_DIR: LazyLock<Mutex<Option<std::path::PathBuf>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static final Codec<Storage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(Storage::name), ItemUtils.EMPTY_ALLOWING_ITEMSTACK_CODEC.listOf
pub static STORAGE_CODEC: LazyLock<CodecRef<compat::Shared<Storage>>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &compat::Shared<Storage>| Storage::name(o), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &compat::Shared<Storage>| Storage::get_item_list(o), codec::field_of(codec::list(item_utils::EMPTY_ALLOWING_ITEMSTACK_CODEC.get_mut().clone()), "items")), move |__a0, __a1| Storage::create(&compat::jstring(&__a0), &__a1))));

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
            // screen instanceof AbstractContainerScreen<?> handledScreen - the screen being initialised is the one the
            // game names now
            if compat::container_screen_open() {
                // `handledScreen.getMenu()` - the menu open while the screen initialises is the screen's own. The port's
                // storage cannot hold that menu's container object (Java's `slots.getFirst().container`, read live), so
                // it is followed by its container id: while it is still the open menu its items are re-read each tick
                // (the live container's contents), and at `removed` the last of them become the cached storage - by
                // then the game may already have made the NEXT menu the open one (`MenuScreens.create` sets
                // `containerMenu` before `setScreen` removes this screen), whose items are not this storage's.
                let menu_id: Option<i32> = compat::with_open_menu(|menu, _| menu.container_id);
                if let Some(menu_id) = menu_id {
                    screen_events::after_tick(screen, Arc::new(move |s: &mut Screen| {
                        let title: String = s.get_title().to_string();
                        BackpackPreview::follow_storage(&title, menu_id, false);
                        // (`initializeStorage` too, while the menu is still there to read - it only adds and drops blanks)
                        if STRORAGE_PATTERN.matcher(&title).matches() {
                            BackpackPreview::initialize_storage(s, menu_id);
                        }
                    }));
                    // java: ScreenEvents.remove(screen).register(_ -> updateStorage(handledScreen));
                    screen_events::remove(screen, Arc::new(move |removed: &Screen| BackpackPreview::update_storage(removed, menu_id)));
                }
            }
        }));
    }

    /// The storage of the screen whose menu is `menu_id` refreshed from that menu while it is still the open one
    /// (`slots.getFirst().container`'s items); `dirty` as Java's `new Storage(.., title, true)` at `removed`, `false`
    /// for the per-tick refresh so that only the removal is saved.
    pub fn follow_storage(title: &str, menu_id: i32, dirty: bool) {
        let index: i32 = BackpackPreview::get_storage_index_from_title(title);
        if index == -1 {
            return;
        }
        let __items: Option<Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>> = compat::with_open_menu(|menu, containers| {
            if menu.container_id != menu_id || menu.slots.is_empty() {
                return None;
            }
            let container = containers.get(menu.slots[0].container);
            Some((0..container.get_container_size()).map(|i| container.get_item(i).duplicate()).collect())
        }).flatten();
        match __items {
            Some(items) => STORAGES.get_mut()[(index) as usize] = Some(Storage::new(SimpleContainer::of(items), title.to_string(), dirty)),
            // the menu is gone: the last refresh is the storage, marked for saving
            None if dirty => {
                if let Some(storage) = STORAGES.get()[(index) as usize].clone() {
                    compat::write(&storage, |__t| { __t.dirty = true; __t.name = title.to_string(); });
                }
            }
            None => {}
        }
    }

    /// java: public static void tick()
    pub fn tick() {
        if Utils::is_on_skyblock() {
            BackpackPreview::save_storages();
            let mut id: String = format!("{}/{}", compat::regex::replace_all(&compat::minecraft().get_user().get_profile_id(), "-", ""), Utils::get_profile_id());
            if !(id == compat::get_state(&LOADED)) {
                *compat::lock(&SAVE_DIR) = Some(skyblocker_mod::CONFIG_DIR.get_mut().join(&format!("backpack-preview/{}", id)));
                CompletableFuture::<()>::run_async(move || (|| {
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let __dir = compat::lock(&SAVE_DIR).clone().unwrap();
                        std::fs::create_dir_all(&__dir).expect("IOException");
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("[Skyblocker] Failed to create the backpack preview save directory! Path: {}", compat::opt_str(&compat::lock(&SAVE_DIR).as_ref().map(|p| p.display().to_string()))));
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
            STORAGES.get_mut()[(index) as usize] = None;
            let mut index2: i32 = index;
            CompletableFuture::supply_async(move || {
                let mut storage_file: std::path::PathBuf = compat::lock(&SAVE_DIR).clone().unwrap().join(&format!("{}.nbt", index2));
                if storage_file.is_file() {
                    // try { return Storage.CODEC.parse(getOps(), NbtIo.read(storageFile)).getOrThrow(); }
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<compat::Shared<Storage>, String> {
                        let bytes = std::fs::read(&storage_file).map_err(|e| e.to_string())?;
                        let tag = nbt_io::read(&bytes)?.ok_or_else(|| "empty file".to_string())?;
                        STORAGE_CODEC.parse(&BackpackPreview::get_ops(), &mcrust::com::mojang::serialization::obj(tag)).get_or_throw().map_err(|e| e.to_string())
                    }));
                    match outcome {
                        Ok(Ok(storage)) => return Ok(Some(storage)),
                        // catch (Exception e)
                        Ok(Err(e)) => logger().error(&format!("[Skyblocker] Failed to load backpack preview file: {} {}", std::path::PathBuf::from(storage_file.file_name().unwrap_or_default()).display(), e)),
                        Err(e) => logger().error(&format!("[Skyblocker] Failed to load backpack preview file: {} {}", std::path::PathBuf::from(storage_file.file_name().unwrap_or_default()).display(), compat::panic_message(&e))),
                    }
                }
                return Ok(None);
            }, compat::executor).then_accept_async(move |storage: &Option<compat::Shared<Storage>>| {
                // storages[index2] = storage
                STORAGES.get_mut()[(index2) as usize] = storage.clone();
            }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)));
        }
    }

    /// java: private static RegistryOps<Tag> getOps()
    pub fn get_ops() -> mcrust::com::mojang::serialization::dynamic_ops::Ops {
        return holder_lookup::create_serialization_context(RegistryUtils::get_registry_wrapper_lookup(), mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance());
    }

    /// java: private static void saveStorages()
    pub fn save_storages() {
        for index in 0..STORAGE_SIZE {
            if let Some(storage) = STORAGES.get()[(index) as usize].clone() {
                if compat::read(&storage, |__t| __t.dirty) {
                    BackpackPreview::save_storage(index);
                }
            }
        }
    }

    /// java: private static void saveStorage(int index)
    pub fn save_storage(mut index: i32) {
        let mut storage: compat::Shared<Storage> = STORAGES.get()[(index) as usize].clone().expect("NullPointerException");
        // the worker encodes what the storage holds now (its name and items: `Storage.CODEC`'s two getters)
        let __name: String = Storage::name(&storage);
        let __items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Storage::get_item_list(&storage);
        let __storage = compat::RenderThread::new(storage.clone());
        (CompletableFuture::<()>::run_async(move || (|| {
            let mut storage_file: std::path::PathBuf = compat::lock(&SAVE_DIR).clone().unwrap().join(&format!("{}.nbt", index));
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<(), String> {
                // NbtIo.write((CompoundTag) Storage.CODEC.encodeStart(getOps(), storage).getOrThrow(), storageFile);
                let __snapshot = Storage::new(SimpleContainer::of(__items.clone()), __name.clone(), false);
                let encoded = STORAGE_CODEC.encode_start(&BackpackPreview::get_ops(), &__snapshot).get_or_throw().map_err(|e| e.to_string())?;
                let tag: &Tag = mcrust::com::mojang::serialization::cast::<Tag>(&encoded).ok_or_else(|| "ClassCastException: CompoundTag".to_string())?;
                std::fs::write(&storage_file, nbt_io::write(tag)).map_err(|e| e.to_string())
            }));
            match __result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => logger().error(&format!("[Skyblocker] Failed to save backpack preview file: {} {}", std::path::PathBuf::from(storage_file.file_name().unwrap_or_default()).display(), e)),
                Err(e) => logger().error(&format!("[Skyblocker] Failed to save backpack preview file: {} {}", std::path::PathBuf::from(storage_file.file_name().unwrap_or_default()).display(), compat::panic_message(&e))),
            }
        })(), compat::executor)).then_run_async(move || Storage::mark_clean(__storage.get()), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)));
    }

    /// java: private static void updateStorage(AbstractContainerScreen<?> handledScreen)
    /// (`handledScreen` is the container screen's `Screen` half - its title; its menu is the player's open menu)
    pub fn update_storage(handled_screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, menu_id: i32) {
        let mut title: String = handled_screen.get_title().to_string();
        // storages[index] = new Storage(handledScreen.getMenu().slots.getFirst().container, title, true); - see `init`
        BackpackPreview::follow_storage(&title, menu_id, true);
        if STRORAGE_PATTERN.matcher(&title).matches() {
            BackpackPreview::initialize_storage(handled_screen, menu_id);
        }
    }

    /// java: private static void initializeStorage(AbstractContainerScreen<?> handledScreen)
    /// (`menu_id`: the screen's menu, read only while it is still the open one - see `init`)
    pub fn initialize_storage(handled_screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, menu_id: i32) {
        let _ = handled_screen;
        // NonNullList<Slot> slots = handledScreen.getMenu().slots; - what the loops read of them, `slot.getItem()`
        let mut slots: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::with_open_menu(|menu, containers| {
            if menu.container_id != menu_id {
                return Vec::new();
            }
            menu.slots.iter().map(|slot| slot.get_item(containers).duplicate()).collect()
        }).unwrap_or_default();
        if slots.len() < 45 {
            return;
        }
        //echests
        for i in 9..18i32 {
            let slot_item = &slots[(i) as usize];
            let mut index: i32 = i.wrapping_sub(9);
            //ignore non-existent ender chest or if they are already created
            if slot_item.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().red()) || STORAGES.get()[(index) as usize].is_some() {
                continue;
            }
            // new SimpleContainer(Stream.generate(() -> ItemStack.EMPTY).limit(18).toArray(ItemStack[]::new))
            STORAGES.get_mut()[(index) as usize] = Some(Storage::new(SimpleContainer::of((0..18).map(|_| mcrust::net::minecraft::world::item::item_stack::empty()).collect()), "".to_string(), true));
        }
        //backpacks
        for i in 27..45i32 {
            let slot_item = &slots[(i) as usize];
            let mut index: i32 = i.wrapping_sub(18);
            //remove backpacks if they are no longer there
            if slot_item.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().brown()) {
                STORAGES.get_mut()[(index) as usize] = None;
            }
            //add new backpacks
            if STORAGES.get()[(index) as usize].is_some() {
                continue;
            }
            let mut size: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_contains_match(slot_item, &BACKPACK_SIZE_PATTERN);
            if let Some(size) = &size {
                // .limit(NumberUtils.toInt(size.group(1)) + 9)
                let count: i32 = size.group(1).and_then(|group| group.parse::<i32>().ok()).unwrap_or(0).wrapping_add(9);
                STORAGES.get_mut()[(index) as usize] = Some(Storage::new(SimpleContainer::of((0..count.max(0)).map(|_| mcrust::net::minecraft::world::item::item_stack::empty()).collect()), "".to_string(), true));
            }
        }
    }

    /// java: public static @Nullable Storage[] getStorages()
    pub fn get_storages() -> Vec<Option<compat::Shared<Storage>>> {
        return STORAGES.get().clone();
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
        if STORAGES.get()[(index) as usize].is_none() {
            return false;
        }
        let __storage: compat::Shared<Storage> = STORAGES.get()[(index) as usize].clone().unwrap();
        let mut rows: i32 = ((Storage::size(&__storage)).wrapping_sub(9)).wrapping_div(9);
        let mut x: i32 = (if mouse_x.wrapping_add(184) >= screen.width { mouse_x.wrapping_sub(188) } else { mouse_x.wrapping_add(8) });
        let mut y: i32 = 0i32.max(mouse_y.wrapping_sub(16));
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 176, (rows.wrapping_mul(18)).wrapping_add(17), 256, 256);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, (y.wrapping_add(rows.wrapping_mul(18))).wrapping_add(17), ((0) as f32) as f32, ((215) as f32) as f32, 176, 7, 256, 256);
        let mut text_renderer: Font = compat::font();
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&Storage::name(&__storage)), x.wrapping_add(8), y.wrapping_add(6), 0xFF404040u32 as i32, false);
        for i in 9..Storage::size(&__storage) {
            let mut current_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = Storage::get_stack(&__storage, i);
            let mut item_x: i32 = (x.wrapping_add(((i.wrapping_sub(9)).wrapping_rem(9)).wrapping_mul(18))).wrapping_add(8);
            let mut item_y: i32 = (y.wrapping_add(((i.wrapping_sub(9)).wrapping_div(9)).wrapping_mul(18))).wrapping_add(18);
            ItemBackgroundManager::draw_backgrounds(&current_stack, graphics, item_x, item_y);
            if ItemProtection::is_item_protected_ref(Some(&current_stack)) {
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
            return (echest.group(1).unwrap().parse::<i32>().expect("NumberFormatException")).wrapping_sub(1);
        }
        let mut backpack: compat::regex::Matcher = BACKPACK_PATTERN.matcher(title);
        if backpack.find() {
            return (backpack.group(1).unwrap().parse::<i32>().expect("NumberFormatException")).wrapping_add(8);
        }
        return -1;
    }

}

/// java: public static class Storage
pub struct Storage {
    /// java: private final Container inventory;
    /// (the port's mod owns its container: a `SimpleContainer` - see `updateStorage`)
    pub inventory: SimpleContainer,
    /// java: private final String name;
    pub name: String,
    /// java: private boolean dirty;
    pub dirty: bool,
}

impl Storage {
    /// java: private Storage(Container inventory, String name, boolean dirty)
    pub fn new(mut inventory: SimpleContainer, mut name: String, mut dirty: bool) -> compat::Shared<Self> {
        let __init_inventory = inventory;
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
        return Storage::new(inventory, name.to_string(), false);
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
