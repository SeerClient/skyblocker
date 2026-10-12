//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonMapTexture by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map::DungeonMap;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils::DungeonMapUtils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::blaze3d::platform::native_image::NativeImage;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::renderer::map_renderer::MapRenderer;
use mcrust::net::minecraft::client::renderer::state::map_render_state::MapRenderState;
use mcrust::net::minecraft::client::renderer::texture::dynamic_texture::DynamicTexture;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::item::map_item;
use mcrust::net::minecraft::world::level::material::map_color;
use mcrust::net::minecraft::world::level::saveddata::maps::map_id::MapId;
use mcrust::net::minecraft::world::level::saveddata::maps::map_item_saved_data::MapItemSavedData;
use mcrust::org::joml::vector2i::Vector2i;

/// java: private static final int MAP_TEXTURE_SIZE = 128;
pub const MAP_TEXTURE_SIZE: i32 = 128;
/// java: private static final Identifier ID = SkyblockerMod.id("dungeon_map_tex");
pub static ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("dungeon_map_tex"));
/// java: private static @Nullable DynamicTexture dungeonMapTexture = null;
pub static DUNGEON_MAP_TEXTURE: compat::MainThread<Option<DynamicTexture>> = compat::MainThread::new(|| None);
/// java: private static boolean requiresUpload = true;
pub static REQUIRES_UPLOAD: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));
//   java: private static final MapRenderState MAP_RENDER_STATE = Util.make(new MapRenderState(), state -> state.texture = ID);
/// java: private static final MapRenderState MAP_RENDER_STATE = Util.make(new MapRenderState(), state -> state.texture = ID);
pub static MAP_RENDER_STATE: compat::MainThread<MapRenderState> = compat::MainThread::new(|| {
    let mut state = MapRenderState::new();
    state.texture = Some(ID.get().clone());
    state
});
/// java: private static final Set<Room.Type> IGNORED_ROOMS_FOR_CHECKMARK_HIDING = Set.of(Room.Type.ENTRANCE, Room.Type.BLOOD, Room.Type.FAIRY, Room.Type.UNKNOWN);
pub static IGNORED_ROOMS_FOR_CHECKMARK_HIDING: LazyLock<std::collections::HashSet<Type>> = LazyLock::new(|| std::collections::HashSet::from([Type::Entrance, Type::Blood, Type::Fairy, Type::Unknown]));
/// java: private static final Vector2ic[] NEIGHBOUR_OFFSETS = new Vector2ic[] { new Vector2i(0, 1), new Vector2i(0, -1), new Vector2i(1, 0), new Vector2i(-1, 0) };
pub static NEIGHBOUR_OFFSETS: compat::MainThread<Vec<Vector2i>> = compat::MainThread::new(|| vec![Vector2i::new(0, 1), Vector2i::new(0, -1), Vector2i::new(1, 0), Vector2i::new(-1, 0)]);

/// java: public class DungeonMapTexture
#[derive(Clone)]
pub struct DungeonMapTexture {}

impl DungeonMapTexture {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(minecraft -> {
        //   java: 			dungeonMapTexture = new DynamicTexture(() -> "Skyblocker Dungeon Map", MAP_TEXTURE_SIZE, MAP_TEXTURE_SIZE, true);
        //   java: 			minecraft.getTextureManager().register(ID, dungeonMapTexture);
        //   java: 		});
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DungeonMapTexture::clear_map_image();
        }));
        dungeon_events::ROOM_MATCHED.register(Arc::new(move |_| { DungeonMapTexture::on_map_item_data_update(&DungeonMap::get_map_id_component(None), true); }));
        world_render_events::START_MAIN.register(Arc::new(|ci: &mut CallbackInfo<'_>| {
            if DUNGEON_MAP_TEXTURE.get_mut().is_some() && compat::get_state(&REQUIRES_UPLOAD) {
                DUNGEON_MAP_TEXTURE.get_mut().as_mut().unwrap().upload();
                compat::set_state(&REQUIRES_UPLOAD, false);
            }
        }));
    }

    /// java: public static void onMapItemDataUpdate(MapId mapId, boolean updateMapTexture)
    pub fn on_map_item_data_update(map_id: &MapId, mut update_map_texture: bool) {
        if Utils::is_in_dungeons() && !(DungeonManager::is_in_boss()) && *map_id == DungeonMap::get_map_id_component(None) {
            let state_handle = map_item::get_saved_data(Some(map_id), &mcrust::mcrust::modding::game::level().unwrap()).unwrap();
            let state = state_handle.lock().unwrap_or_else(|p| p.into_inner());
            if update_map_texture {
                DungeonMapTexture::update_map_image(&state);
                DungeonMapTexture::hide_checkmarks(&state);
            }
            DungeonMapTexture::update_map_decorations(&state);
        }
    }

    /// java: private static void updateMapImage(MapItemSavedData mapData)
    pub fn update_map_image(map_data: &MapItemSavedData) {
        if DUNGEON_MAP_TEXTURE.get_mut().is_none() {
            return;
        }
        // NativeImage pixels = dungeonMapTexture.getPixels(); - the texture's own image
        let mut pixels: Option<&mut NativeImage> = Some(DUNGEON_MAP_TEXTURE.get_mut().as_mut().unwrap().get_pixels_mut());
        if let Some(pixels) = pixels {
            for y in 0..MAP_TEXTURE_SIZE {
                for x in 0..MAP_TEXTURE_SIZE {
                    let mut i: i32 = x.wrapping_add(y.wrapping_mul(MAP_TEXTURE_SIZE));
                    pixels.set_pixel(x, y, map_color::get_color_from_packed_id((map_data.colors[(i) as usize]) as i32));
                }
            }
        }
        compat::set_state(&REQUIRES_UPLOAD, true);
    }

    /// java: private static void updateMapDecorations(MapItemSavedData mapData)
    pub fn update_map_decorations(map_data: &MapItemSavedData) {
        // java: Minecraft.getMapRenderer() - the port's `GameRenderer::map_renderer` (Minecraft's own getter is not ported)
        let mut map_renderer: MapRenderer = compat::minecraft().game_renderer.map_renderer();
        MAP_RENDER_STATE.get_mut().decorations.clear();
        for decoration in map_data.get_decorations() {
            // rust: port gap - `MapRenderer::extract_decoration_render_state` is private in the port (the Java accessor is `MapRendererInvoker`); GAPS_K #1
            // rust: MAP_RENDER_STATE.get_mut().decorations.push(map_renderer.extract_decoration_render_state(&decoration));
        }
    }

    /// java: private static void hideCheckmarks(MapItemSavedData mapData)
    pub fn hide_checkmarks(map_data: &MapItemSavedData) {
        let map_config = &SkyblockerConfigManager::get().dungeons.dungeon_map;
        if !map_config.show_room_labels || !map_config.hide_checkmarks {
            return;
        }
        for room in DungeonManager::get_rooms_stream().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.is_matched()); __f(__a0) })).filter(|room| !(IGNORED_ROOMS_FOR_CHECKMARK_HIDING.contains(&({ let __f = compat::dispatch(room, |__t| __t.get_type()); __f(room) })))) {
            DungeonManager::get_room_checkmark_colour(map_data, &room, Some(&|checkmark_pixel: Vector2i| {
                let mut checkmark_pixels: std::collections::HashSet<Vector2i> = std::collections::HashSet::new();
                let mut visited: std::collections::HashSet<Vector2i> = std::collections::HashSet::new();
                let mut queue: std::collections::VecDeque<Vector2i> = std::collections::VecDeque::new();
                queue.push_back(checkmark_pixel.clone());
                while !queue.is_empty() {
                    // java: Vector2ic current = queue.remove();
                    let mut current: Vector2i = queue.pop_front().expect("NoSuchElementException");
                    checkmark_pixels.insert(current.clone());
                    for offset in NEIGHBOUR_OFFSETS.get_mut().iter() {
                        let mut neighbour = Vector2i::new_from(&current);
                        neighbour.add_vector(offset);
                        if visited.contains(&neighbour) || queue.iter().any(|e| *e == neighbour) {
                            continue;
                        } else if neighbour.x() >= 0 && neighbour.x() < MAP_TEXTURE_SIZE && neighbour.y() >= 0 && neighbour.y() < MAP_TEXTURE_SIZE && DungeonManager::matches_checkmark_colour((DungeonMapUtils::get_color(map_data, Some(neighbour.clone()))) as i32) {
                            visited.insert(neighbour.clone());
                            queue.push_back(neighbour.clone());
                        }
                    }
                }
                DungeonMapTexture::set_pixels(&checkmark_pixels, map_color::get_color_from_packed_id((({ let __f = compat::dispatch(&room, |__t| __t.get_type()); __f(&room) }).fields().color) as i32));
            }));
        }
    }

    /// java: private static void setPixels(Set<Vector2ic> checkmarkPixels, int colour)
    pub fn set_pixels(checkmark_pixels: &std::collections::HashSet<Vector2i>, mut colour: i32) {
        if DUNGEON_MAP_TEXTURE.get_mut().is_none() {
            return;
        }
        // NativeImage pixels = dungeonMapTexture.getPixels(); - the texture's own image
        let mut pixels: Option<&mut NativeImage> = Some(DUNGEON_MAP_TEXTURE.get_mut().as_mut().unwrap().get_pixels_mut());
        if let Some(pixels) = pixels {
            for pixel in checkmark_pixels.iter() {
                pixels.set_pixel(pixel.x(), pixel.y(), colour);
            }
        }
        compat::set_state(&REQUIRES_UPLOAD, true);
    }

    /// java: private static void clearMapImage()
    pub fn clear_map_image() {
        if DUNGEON_MAP_TEXTURE.get_mut().is_none() {
            return;
        }
        // NativeImage pixels = dungeonMapTexture.getPixels(); - the texture's own image
        let mut pixels: Option<&mut NativeImage> = Some(DUNGEON_MAP_TEXTURE.get_mut().as_mut().unwrap().get_pixels_mut());
        if let Some(pixels) = pixels {
            for y in 0..MAP_TEXTURE_SIZE {
                for x in 0..MAP_TEXTURE_SIZE {
                    pixels.set_pixel(x, y, argb::transparent(mcrust::net::minecraft::util::common_colors::BLACK));
                }
            }
        }
        compat::set_state(&REQUIRES_UPLOAD, true);
    }

    /// java: private static void uploadMapTexture(LevelTerrainRenderContext context)
    pub fn upload_map_texture(context: &crate::unresolved::LevelTerrainRenderContext) {
        if DUNGEON_MAP_TEXTURE.get_mut().is_some() && compat::get_state(&REQUIRES_UPLOAD) {
            DUNGEON_MAP_TEXTURE.get_mut().as_mut().unwrap().upload();
            compat::set_state(&REQUIRES_UPLOAD, false);
        }
    }

    /// java: protected static void blitMap(GuiGraphicsExtractor graphics)
    pub fn blit_map(graphics: &GuiGraphicsExtractor) {
        gui_graphics_extractor::map(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), MAP_RENDER_STATE.get_mut());
    }

    /// java: public static void close()
    pub fn close() {
        if DUNGEON_MAP_TEXTURE.get_mut().is_some() {
            DUNGEON_MAP_TEXTURE.get_mut().as_mut().unwrap().close();
        }
    }

}
