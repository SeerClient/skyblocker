//! Ported from de.hysky.skyblocker.skyblock.garden.GardenPlots by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::garden::garden_plots_widget::GardenPlotsWidget;
use crate::de::hysky::skyblocker::skyblock::garden::garden_plots_widget::GardenPlotsWidgetRef;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::mixins::inventory_screen_mixin;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::InventoryScreen;
use mcrust::net::minecraft::world::item::items;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final Path FOLDER = SkyblockerMod.CONFIG_DIR.resolve("garden_plots");
pub static FOLDER: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("garden_plots"));
/// java: public static final @Nullable GardenPlot[] GARDEN_PLOTS = new GardenPlot[25];
///
/// The ELEMENTS are nullable (`new GardenPlot[25]` starts as 25 nulls).
pub static GARDEN_PLOTS: compat::MainThread<Vec<Option<GardenPlot>>> = compat::MainThread::new(|| vec![None; 25]);
/// java: public static @Nullable GardenPlotsWidget widget;
pub static WIDGET: compat::MainThread<Option<GardenPlotsWidgetRef>> = compat::MainThread::new(|| None);

thread_local! {
    /// The open "Configure Plots" screen registered its `ScreenEvents.remove` listener.
    static CONFIGURE_PLOTS_REMOVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// The open inventory screen registered `ScreenEvents.remove(screen).register(_ -> widget = null)`.
    static WIDGET_REMOVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// The inventory screen being initialised gets the garden plots widget (see `attach_widget`).
    static WIDGET_ATTACH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// The recipe book was toggled since the inventory screen last drew (`registerRecipeBookToggleCallback`).
    static RECIPE_BOOK_TOGGLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}
/// java: @SuppressWarnings("deprecation") public static final Codec<GardenPlot> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.either(Item.CODEC.xmap(Holder::value, Item::builtInRegistryHo
///
/// `Item.CODEC.xmap(Holder::value, Item::builtInRegistryHolder)` is the port's `item::codec()` itself (its holder IS the
/// `Arc<Item>`). A `LazyLock`: the save and the load run on the executor.
pub static GARDEN_PLOT_CODEC: LazyLock<CodecRef<GardenPlot>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &GardenPlot| o.icon(), codec::field_of(codec::either(mcrust::net::minecraft::world::item::item::codec(), codec::string_codec()), "icon")), record_codec_builder::of(|o: &GardenPlot| o.name(), codec::field_of(codec::string_codec(), "name")), record_codec_builder::of(|o: &GardenPlot| o.custom_icon(), codec::optional_field_of_opt(codec::string_codec(), "customIcon")), move |__a0, __a1, __a2| GardenPlot::new_3(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public final class GardenPlots
#[derive(Clone)]
pub struct GardenPlots {}

impl GardenPlots {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ScreenEvents.remove(screen).register(..) - the per-screen listeners below, fired as the screen they
        // were registered for is removed (`compat::batch_s_on_container_screen_remove`: the screen's `onClose`)
        compat::batch_s_on_container_screen_remove(|stacks| {
            if CONFIGURE_PLOTS_REMOVE.with(|r| r.replace(false)) {
                GardenPlots::take_plot_icons_and_names(stacks);
            }
            if WIDGET_REMOVE.with(|r| r.replace(false)) {
                // java: ScreenEvents.remove(screen).register(_ -> widget = null);
                *WIDGET.get_mut() = None;
            }
        });
        // java: inventoryScreen.registerRecipeBookToggleCallback(() -> widget.setInventoryRectangle(new ScreenRectangle(
        // java: 		accessor.getX(), accessor.getY(), accessor.getImageWidth(), accessor.getImageHeight())));
        // (the callback runs in the recipe book button's press, where the screen is not lent; the rectangle is read off
        // the screen at its next lent call, before it draws)
        let rectangle = mcrust::mcrust::modding::hook::register_injector(
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractContents",
            mcrust::mcrust::modding::hook::InjectOptions::at(mcrust::mcrust::modding::hook::At::Head).priority(900),
            Arc::new(|ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                if !RECIPE_BOOK_TOGGLED.with(|t| t.replace(false)) {
                    return;
                }
                let Some(accessor) = ci.find_arg::<AbstractContainerScreen>() else {
                    return;
                };
                let rectangle = ScreenRectangle::new_at(accessor.left_pos, accessor.top_pos, accessor.image_width, accessor.image_height);
                let widget = WIDGET.get().clone();
                if let Some(widget) = widget {
                    widget.set_inventory_rectangle(rectangle);
                }
            }),
        );
        if let Err(e) = rectangle {
            logger().error(&format!("[Skyblocker] Failed to hook the inventory screen's draw: {:?}", e));
        }
        // The garden plots widget joins the inventory screen as its `init` returns (where Java's AFTER_INIT listener,
        // below, runs on the whole screen: `leftPos` / `topPos` are final only after the recipe book's placement).
        let attach = mcrust::mcrust::modding::hook::inject("net.minecraft.client.gui.screens.inventory.InventoryScreen#init", mcrust::mcrust::modding::hook::At::Return, |ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            if !WIDGET_ATTACH.with(|a| a.replace(false)) {
                return;
            }
            let Some(inventory_screen) = ci.this_mut::<InventoryScreen>() else {
                return;
            };
            GardenPlots::attach_widget(inventory_screen);
        });
        if let Err(e) = attach {
            logger().error(&format!("[Skyblocker] Failed to hook the inventory screen's init: {:?}", e));
        }
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // a new screen (or a re-init) has registered no per-screen listener yet
            CONFIGURE_PLOTS_REMOVE.with(|r| r.set(false));
            WIDGET_REMOVE.with(|r| r.set(false));
            WIDGET_ATTACH.with(|a| a.set(false));
            // java: if (screen instanceof ContainerScreen containerScreen && screen.getTitle().getString().trim().equals("Configure Plots")) {
            let is_container_screen: bool = compat::is_chest_screen_initialising();
            let is_inventory_screen: bool = compat::is_inventory_screen_initialising();
            if is_container_screen && screen.get_title().to_string().trim_matches(|c: char| c <= ' ') == "Configure Plots" {
                // java: ScreenEvents.remove(screen).register(_ -> { .. take plot icons and names .. });
                CONFIGURE_PLOTS_REMOVE.with(|r| r.set(true));
            } else if is_inventory_screen && Utils::get_location() == Location::Garden && SkyblockerConfigManager::get().farming.plots_widget.enabled && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:gardenPlots")) {
                // java: ScreenEvents.remove(screen).register(_ -> widget = null);
                WIDGET_REMOVE.with(|r| r.set(true));
                // java: widget = new GardenPlotsWidget(..); Screens.getWidgets(inventoryScreen).add(widget);
                // java: inventoryScreen.registerRecipeBookToggleCallback(..);
                // (`attach_widget`, as the inventory screen's `init` returns)
                WIDGET_ATTACH.with(|a| a.set(true));
            }
        }));
        skyblock_events::PROFILE_CHANGE.register(Arc::new(move |prev_profile_id, profile_id| {
            if !prev_profile_id.is_empty() {
                // `save` reads GARDEN_PLOTS: the snapshot is taken here, on the render thread, and written on the executor
                let plots: Vec<Option<GardenPlot>> = GARDEN_PLOTS.get().clone();
                let folder: std::path::PathBuf = FOLDER.get().clone();
                CompletableFuture::<()>::run_async(move || (|| {
                    GardenPlots::save_plots(&folder, &prev_profile_id, &plots);
                    GardenPlots::load_in(&folder, &profile_id);
                })(), compat::executor);
            } else {
                GardenPlots::load(&profile_id);
            }
        }));
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> {
        // java: 	String profileId = Utils.getProfileId();
        // java: 	if (!profileId.isBlank()) {
        // java: 		CompletableFuture.runAsync(() -> save(profileId), SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
        // java: 	}
        // java: });
        // (`CLIENT_STOPPING` is Fabric's HEAD injector on `Minecraft#stop`; `save` reads GARDEN_PLOTS and FOLDER,
        // render-thread statics, so the snapshot is taken here and written on the executor)
        let stopping = mcrust::mcrust::modding::hook::inject("net.minecraft.client.Minecraft#stop", mcrust::mcrust::modding::hook::At::Head, |_: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            let profile_id: String = Utils::get_profile_id();
            if !compat::is_blank(&profile_id) {
                let plots: Vec<Option<GardenPlot>> = GARDEN_PLOTS.get().clone();
                let folder: std::path::PathBuf = FOLDER.get().clone();
                CompletableFuture::<()>::run_async(move || GardenPlots::save_plots(&folder, &profile_id, &plots), compat::executor);
            }
        });
        if let Err(e) = stopping {
            logger().error(&format!("[Skyblocker] Failed to hook the client stop: {:?}", e));
        }
    }

    /// java: `ScreenEvents.remove(screen).register(_ -> { .. })` of the "Configure Plots" screen: its body, over the
    /// stacks of `containerScreen.getMenu().slots`
    fn take_plot_icons_and_names(stacks: &[mcrust::net::minecraft::world::item::item_stack::ItemStack]) {
        // Take plot icons and names
        for row in 0..5 {
            for i in (row * 9 + 2)..(row * 9 + 7) {
                if i == 22 {
                    continue; // Barn icon
                }
                // java: Slot slot = screenHandler.slots.get(i); ItemStack stack = slot.getItem();
                let stack = &stacks[i as usize];
                let held = stack.get_item();
                if stack.is_empty() || Arc::ptr_eq(&held, items::stained_glass_pane().red()) || Arc::ptr_eq(&held, &items::items().get("oak_button")) || Arc::ptr_eq(&held, items::stained_glass_pane().black()) {
                    continue;
                }
                // SkyHanni adds formatting codes to the plot names when using their custom plot icons.
                let name: String = mcrust::net::minecraft::chat_formatting::strip_formatting(&compat::component_of(stack.get_hover_name()).get_string());
                // java: String[] parts = name.split("-", 2);
                let parts: Vec<&str> = name.splitn(2, '-').collect();
                if parts.len() < 2 {
                    logger().warn(&format!("Invalid plot name: {}", name));
                    continue;
                }
                let index: i32 = (i / 9) * 5 + (i % 9 - 2);
                // java: Either<Item, String> icon = stack.getNeuName().isBlank() ? Either.left(stack.getItem()) : Either.right(stack.getNeuName());
                let neu_name: String = stack.get_neu_name();
                let icon: Either<Arc<Item>, String> = if compat::is_blank(&neu_name) { Either::Left(held.clone()) } else { Either::Right(neu_name) };
                // java: String plotName = parts[1].trim();
                let plot_name: String = parts[1].trim_matches(|c: char| c <= ' ').to_string();
                let plot: Option<GardenPlot> = GARDEN_PLOTS.get()[index as usize].clone();
                match plot {
                    None => GARDEN_PLOTS.get_mut()[index as usize] = Some(GardenPlot::new(icon, plot_name)),
                    Some(plot) => GARDEN_PLOTS.get_mut()[index as usize] = Some(plot.with_icon_name(&icon, &plot_name)),
                }
            }
        }
    }

    /// The inventory screen half of the AFTER_INIT listener, on the screen whose `init` is returning:
    /// java: AbstractContainerScreenAccessor accessor = (AbstractContainerScreenAccessor) inventoryScreen;
    /// java: widget = new GardenPlotsWidget(new ScreenRectangle(accessor.getX(), accessor.getY(), accessor.getImageWidth(), accessor.getImageHeight()));
    /// java: Screens.getWidgets(inventoryScreen).add(widget);
    /// java: inventoryScreen.registerRecipeBookToggleCallback(() -> widget.setInventoryRectangle(..));
    fn attach_widget(inventory_screen: &mut InventoryScreen) {
        let accessor = &mut inventory_screen.container;
        let widget: compat::Shared<GardenPlotsWidget> = compat::shared(GardenPlotsWidget::new(ScreenRectangle::new_at(accessor.left_pos, accessor.top_pos, accessor.image_width, accessor.image_height)));
        *WIDGET.get_mut() = Some(GardenPlotsWidgetRef(widget.clone()));
        accessor.screen.add_renderable_widget(compat::object_widget(widget, ""));
        // (`InventoryScreenMixin`'s `registerRecipeBookToggleCallback`, the list its `onRecipeBookButtonClick` TAIL runs)
        inventory_screen_mixin::RECIPE_BOOK_TOGGLE_CALLBACKS.get_mut().push(Arc::new(|| RECIPE_BOOK_TOGGLED.with(|t| t.set(true))));
    }

    /// java: private static void save(String profileId)
    pub fn save(profile_id: &str) {
        let plots: Vec<Option<GardenPlot>> = GARDEN_PLOTS.get().clone();
        GardenPlots::save_plots(FOLDER.get(), profile_id, &plots);
    }

    /// `save`'s body over a snapshot of `GARDEN_PLOTS` and `FOLDER` (both render-thread statics), so it can run on
    /// the executor as Java's does.
    fn save_plots(folder: &std::path::Path, profile_id: &str, plots: &[Option<GardenPlot>]) {
        // java: try { Files.createDirectories(FOLDER); } catch (IOException e) {..}
        if let Err(e) = std::fs::create_dir_all(folder) {
            logger().error(&format!("[Skyblocker] Failed to create folder for garden plots! {}", e));
        }
        let mut resolve: std::path::PathBuf = folder.join(&format!("{}.json", profile_id));
        // java: try (BufferedWriter writer = Files.newBufferedWriter(resolve)) {..} catch (Exception e) {..}
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            use std::io::Write as _;
            let mut writer = std::io::BufWriter::new(std::fs::File::create(&resolve).unwrap_or_else(|e| panic!("IOException: {e}")));
            let mut elements: Vec<serde_json::Value> = Vec::new();
            for garden_plot in plots {
                let element: serde_json::Value = match garden_plot {
                    None => serde_json::Value::Null,
                    Some(garden_plot) => GARDEN_PLOT_CODEC.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), garden_plot).result().map(|o| compat::json_from_port(&o)).unwrap_or(serde_json::Value::Null),
                };
                elements.push(element);
            }
            // SkyblockerMod.GSON.toJson(elements, writer) (GSON is `setPrettyPrinting()`)
            let text: String = serde_json::to_string_pretty(&serde_json::Value::Array(elements)).unwrap_or_else(|e| panic!("JsonIOException: {e}"));
            writer.write_all(text.as_bytes()).unwrap_or_else(|e| panic!("IOException: {e}"));
            writer.flush().unwrap_or_else(|e| panic!("IOException: {e}"));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to save Garden Plots data {}", compat::panic_message(&e)));
        }
    }

    /// java: private static void load(String profileId)
    pub fn load(profile_id: &str) {
        GardenPlots::load_in(FOLDER.get(), profile_id);
    }

    /// `load`'s body over `FOLDER` read by the caller (a render-thread static).
    fn load_in(folder: &std::path::Path, profile_id: &str) {
        let mut resolve: std::path::PathBuf = folder.join(&format!("{}.json", profile_id));
        // CompletableFuture.supplyAsync(.., VIRTUAL_THREAD_EXECUTOR).thenAcceptAsync(newPlots -> arraycopy, Minecraft.getInstance())
        compat::executor(Box::new(move || {
            let new_plots: Vec<Option<GardenPlot>> = GardenPlots::read_plots(&resolve);
            crate::compat::execute(move || {
                let length = new_plots.len().min(25);
                GARDEN_PLOTS.get_mut()[..length].clone_from_slice(&new_plots[..length]);
            });
        }));
    }

    /// `load`'s `supplyAsync` body.
    fn read_plots(resolve: &std::path::Path) -> Vec<Option<GardenPlot>> {
        match std::fs::read_to_string(resolve) {
            // catch (NoSuchFileException _) {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                logger().error(&format!("[Skyblocker] Failed to load Equipment data {}", e));
            }
            Ok(reader) => {
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let json: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    let serde_json::Value::Array(elements) = json else {
                        panic!("JsonSyntaxException: not a JsonArray");
                    };
                    elements
                        .iter()
                        .map(|json_element| {
                            if json_element.is_null() {
                                return None;
                            }
                            Some(GARDEN_PLOT_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(json_element)).result().expect("NoSuchElementException"))
                        })
                        .collect::<Vec<Option<GardenPlot>>>()
                }));
                match __result {
                    Ok(plots) => return plots,
                    Err(e) => logger().error(&format!("[Skyblocker] Failed to load Equipment data {}", compat::panic_message(&e))),
                }
            }
        }
        return vec![None; 25];
    }

}

/// java: public record GardenPlot(Either<Item, String> icon, String name, Optional<String> customIcon)
#[derive(Clone, Debug)]
pub struct GardenPlot {
    /// java: Either<Item, String> icon
    pub icon: Either<Arc<Item>, String>,
    /// java: String name
    pub name: String,
    /// java: Optional<String> customIcon
    pub custom_icon: Option<String>,
}

impl GardenPlot {
    /// java: public GardenPlot(Either<Item, String> icon, String name)
    pub fn new(mut icon: Either<Arc<Item>, String>, mut name: String) -> Self {
        let mut this = Self::new_3(icon, name.clone(), None);
        this
    }

    /// java: the canonical record constructor
    pub fn new_3(mut icon: Either<Arc<Item>, String>, mut name: String, mut custom_icon: Option<String>) -> Self {
        Self { icon, name, custom_icon }
    }

    /// java: the record accessor `icon()`
    pub fn icon(&self) -> Either<Arc<Item>, String> {
        self.icon.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `customIcon()`
    pub fn custom_icon(&self) -> Option<String> {
        self.custom_icon.clone()
    }

    /// java: public GardenPlot withIconName(Either<Item, String> icon, String name)
    pub fn with_icon_name(&self, icon: &Either<Arc<Item>, String>, name: &str) -> GardenPlot {
        return GardenPlot::new_3(icon.clone(), name.to_string(), self.custom_icon.clone());
    }

    /// java: public GardenPlot withCustomIcon(@Nullable String customIcon)
    pub fn with_custom_icon(&self, mut custom_icon: Option<String>) -> GardenPlot {
        return GardenPlot::new_3(self.icon.clone(), self.name.clone(), custom_icon.clone());
    }

}

/// The record's `equals`: component-wise, `Item` by identity (it keeps `Object.equals`).
impl PartialEq for GardenPlot {
    fn eq(&self, other: &Self) -> bool {
        let icon_eq = match (&self.icon, &other.icon) {
            (Either::Left(a), Either::Left(b)) => Arc::ptr_eq(a, b),
            (Either::Right(a), Either::Right(b)) => a == b,
            _ => false,
        };
        icon_eq && self.name == other.name && self.custom_icon == other.custom_icon
    }
}
