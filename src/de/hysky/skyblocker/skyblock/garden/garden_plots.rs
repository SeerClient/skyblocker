//! Ported from de.hysky.skyblocker.skyblock.garden.GardenPlots by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::garden::garden_plots_widget::GardenPlotsWidget;
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
pub static WIDGET: compat::MainThread<Option<GardenPlotsWidget>> = compat::MainThread::new(|| None);
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
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // TODO(fabric2seer): `.getTitle(..)` on a value of unknown type
            //   java: if (screen instanceof ContainerScreen containerScreen && screen.getTitle().getString().trim().equals("Configure Plots")) {
            //   java: 				ScreenEvents.remove(screen).register(_ -> {
            //   java: 					ChestMenu screenHandler = containerScreen.getMenu();
            //   java: 					// Take plot icons and names
            //   java: 					for (int row = 0; row < 5; row++)
            //   java: 						for (int i = row * 9 + 2; i < row * 9 + 7; i++) {
            //   java: 							if (i == 22) continue; // Barn icon
            //   java: 							Slot slot = screenHandler.slots.get(i);
            //   java: 							ItemStack stack = slot.getItem();
            //   java: 							if (stack.isEmpty() || stack.is(Items.STAINED_GLASS_PANE.red()) || stack.is(Items.OAK_BUTTON) || stack.is(Items.STAINED_GLASS_PANE.black()))
            //   java: 								continue;
            //   java: 							// SkyHanni adds formatting codes to the plot names when using their custom plot icons.
            //   java: 							String name = ChatFormatting.stripFormatting(stack.getHoverName().getString());
            //   java: 							String[] parts = name.split("-", 2);
            //   java: 							if (parts.length < 2) {
            //   java: 								LOGGER.warn("Invalid plot name: {}", name);
            //   java: 								continue;
            //   java: 							}
            //   java: 							int index = (i / 9) * 5 + (i % 9 - 2);
            //   java: 							Either<Item, String> icon = stack.getNeuName().isBlank() ? Either.left(stack.getItem()) : Either.right(stack.getNeuName());
            //   java: 							String plotName = parts[1].trim();
            //   java: 							GardenPlot plot = GARDEN_PLOTS[index];
            //   java: 							if (plot == null) {
            //   java: 								GARDEN_PLOTS[index] = new GardenPlot(icon, plotName);
            //   java: 							} else {
            //   java: 								GARDEN_PLOTS[index] = plot.withIconName(icon, plotName);
            //   java: 							}
            //   java: 						}
            //   java:
            //   java: 				});
            //   java: 			} else if (screen instanceof InventoryScreen inventoryScreen && Utils.getLocation().equals(Location.GARDEN) && SkyblockerConfigManager.get().farming.plotsWidget.enabled && !CatharsisCompatibility.isGuiElementHidden("skyblocker:gardenPlots")) {
            //   java: 				ScreenEvents.remove(screen).register(_ -> widget = null);
            //   java: 				AbstractContainerScreenAccessor accessor = (AbstractContainerScreenAccessor) inventoryScreen;
            //   java: 				widget = new GardenPlotsWidget(new ScreenRectangle(
            //   java: 						accessor.getX(),
            //   java: 						accessor.getY(),
            //   java: 						accessor.getImageWidth(),
            //   java: 						accessor.getImageHeight()
            //   java: 				));
            //   java: 				Screens.getWidgets(inventoryScreen).add(widget);
            //   java:
            //   java: 				inventoryScreen.registerRecipeBookToggleCallback(() -> widget.setInventoryRectangle(new ScreenRectangle(
            //   java: 								accessor.getX(),
            //   java: 								accessor.getY(),
            //   java: 								accessor.getImageWidth(),
            //   java: 								accessor.getImageHeight()
            //   java: 						)));
            //   java: 			}
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
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> {
        //   java: 			String profileId = Utils.getProfileId();
        //   java: 			if (!profileId.isBlank()) {
        //   java: 				CompletableFuture.runAsync(() -> save(profileId), SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
        //   java: 			}
        //   java: 		});
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
            mcrust::mcrust::modding::game::execute(move || {
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
