//! Ported from de.hysky.skyblocker.skyblock.garden.GardenPlots by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::garden::garden_plots_widget::GardenPlotsWidget;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: private static final Path FOLDER = SkyblockerMod.CONFIG_DIR.resolve("garden_plots");
pub static FOLDER: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("garden_plots"));
/// java: public static final @Nullable GardenPlot[] GARDEN_PLOTS = new GardenPlot[25];
pub static GARDEN_PLOTS: compat::MainThread<Option<Vec<GardenPlot>>> = compat::MainThread::new(|| Some((0..(25) as usize).map(|_| Default::default()).collect::<Vec<_>>()));
/// java: public static @Nullable GardenPlotsWidget widget;
pub static WIDGET: compat::MainThread<Option<GardenPlotsWidget>> = compat::MainThread::new(|| None);
// TODO(fabric2seer): `.value(..)` on a A
//   java: @SuppressWarnings("deprecation")
//   java: 		public static final Codec<GardenPlot> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 								Codec.either(Item.CODEC.xmap(Holder::value, Item::builtInRegistryHolder), Codec.STRING)
//   java: 										.fieldOf("icon").forGetter(GardenPlot::icon),
//   java: 								Codec.STRING.fieldOf("name").forGetter(GardenPlot::name),
//   java: 								Codec.STRING.optionalFieldOf("customIcon").forGetter(GardenPlot::customIcon)
//   java: 						)
//   java: 						.apply(instance, GardenPlot::new)
//   java: 		);
/// java: @SuppressWarnings("deprecation") public static final Codec<GardenPlot> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.either(Item.CODEC.xmap(Holder::value, Item::builtInRegistryHo
pub static GARDEN_PLOT_CODEC: compat::MainThread<CodecRef<GardenPlot>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

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
                CompletableFuture::<()>::run_async(move || (|| {
                    GardenPlots::save(&prev_profile_id);
                    GardenPlots::load(&profile_id);
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
        // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            std::fs::create_dir_all(FOLDER.get_mut()).expect("IOException");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to create folder for garden plots!"));
        }
        let mut resolve: std::path::PathBuf = FOLDER.get_mut().join(&format!("{}.json", profile_id));
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut elements: Vec<serde_json::Value> = Vec::new();
                for __a0 in GARDEN_PLOTS.get_mut().as_ref().unwrap().iter().cloned().map(|garden_plot| {
                    if false /* java: garden_plot == null: never null here */ {
                        return Default::default(); // java: return null
                    }
                    return (GARDEN_PLOT_CODEC.get_mut().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &garden_plot)).result();
                }) {
                    elements.push(compat::json::to_value(__a0.clone()));
                }
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(elements, writer);
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to save Garden Plots data"));
            }
        }
    }

    /// java: private static void load(String profileId)
    pub fn load(profile_id: &str) {
        let mut resolve: std::path::PathBuf = FOLDER.get_mut().join(&format!("{}.json", profile_id));
        // TODO(fabric2seer): field `length` of a value of unknown type
        //   java: CompletableFuture.supplyAsync(() -> {
        //   java: 			try (BufferedReader reader = Files.newBufferedReader(resolve)) {
        //   java: 				return SkyblockerMod.GSON.fromJson(reader, JsonArray.class).asList().stream().map(jsonElement -> {
        //   java: 							if (jsonElement == null || jsonElement.isJsonNull()) return null;
        //   java: 							return GardenPlot.CODEC.decode(JsonOps.INSTANCE, jsonElement).result().map(Pair::getFirst).orElseThrow();
        //   java: 						}
        //   java: 				).toArray(GardenPlot[]::new);
        //   java: 			} catch (NoSuchFileException _) {
        //   java: 			} catch (Exception e) {
        //   java: 				LOGGER.error("[Skyblocker] Failed to load Equipment data", e);
        //   java: 			}
        //   java: 			return new GardenPlot[25];
        //   java: 			// Schedule on main thread to avoid any async weirdness
        //   java: 		}, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).thenAcceptAsync(newPlots -> System.arraycopy(newPlots, 0, GARDEN_PLOTS, 0, Math.min(newPlots.length, 25)), Minecraft.getInstance());
    }

}

/// java: public record GardenPlot(Either<Item, String> icon, String name, Optional<String> customIcon)
#[derive(Clone, Debug, PartialEq)]
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
        let mut this = Self::new(icon, name.clone(), None);
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut icon: Either<Arc<Item>, String>, mut name: String, mut custom_icon: Option<String>) -> Self {
        Self { icon, name, custom_icon }
    }

    /// java: the record accessor `icon()`
    pub fn icon(&self) -> Either<Arc<Item>, String> {
        self.icon
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
        return GardenPlot::new(*icon, name.to_string(), self.custom_icon.clone());
    }

    /// java: public GardenPlot withCustomIcon(@Nullable String customIcon)
    pub fn with_custom_icon(&self, mut custom_icon: Option<String>) -> GardenPlot {
        return GardenPlot::new(self.icon, self.name.clone(), custom_icon.clone());
    }

}
