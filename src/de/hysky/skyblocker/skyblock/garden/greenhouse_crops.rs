//! Ported from de.hysky.skyblocker.skyblock.garden.GreenhouseCrops by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::head_textures;
use crate::de::hysky::skyblocker::utils::flexible_item_stack;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;

/// java: public static final Map<String, Crop> CROP_ID_MAP;
pub static CROP_ID_MAP: compat::MainThread<std::collections::HashMap<String, Crop>> = compat::MainThread::new(|| Default::default());
/// java: public static final Map<Integer, Crop> CROP_BY_INT;
pub static CROP_BY_INT: compat::MainThread<std::collections::HashMap<i32, Crop>> = compat::MainThread::new(|| Default::default());
/// java: public static final Map<String, Crop> CROP_BY_HEAD_TEXTURE_HASH;
pub static CROP_BY_HEAD_TEXTURE_HASH: compat::MainThread<std::collections::HashMap<String, Crop>> = compat::MainThread::new(|| Default::default());

/// java: @SuppressWarnings("unchecked") public final class GreenhouseCrops
#[derive(Clone)]
pub struct GreenhouseCrops {}

impl GreenhouseCrops {
    /// java: static { .. } - Java runs it when the class is first used; every static method and
    /// constructor calls this first.
    fn clinit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let mut crops: Vec<Crop> = vec![Crop::new("Ashwreath".to_string(), "ashwreath".to_string(), 1, head_textures::ASHWREATH.to_string()), Crop::new("Choconut".to_string(), "Choconut".to_string(), 2, head_textures::CHOCONUT.to_string()), Crop::new("Dustgrain".to_string(), "Dustgrain".to_string(), 3, head_textures::DUSTGRAIN.to_string()), Crop::new("Gloomgourd".to_string(), "Gloomgourd".to_string(), 4, head_textures::GLOOMGOURD.to_string()), Crop::new("Lonelily".to_string(), "Lonelilly".to_string(), 5, head_textures::LONELILY.to_string()), Crop::new("Scourroot".to_string(), "Scourroot".to_string(), 6, head_textures::SCOURROOT.to_string()), Crop::new("Shadevine".to_string(), "shadevine".to_string(), 7, head_textures::SHADEVINE.to_string()), Crop::new("Veilshroom".to_string(), "Veilshroom".to_string(), 8, head_textures::VEILSHROOM.to_string()), Crop::new("Witherbloom".to_string(), "Witherbloom".to_string(), 9, head_textures::WITHERBLOOM.to_string()), Crop::new("Chocoberry".to_string(), "Chocoberry".to_string(), 10, head_textures::CHOCOBERRY.to_string()), Crop::new("Cindershade".to_string(), "Cindershade".to_string(), 11, head_textures::CINDERSHADE.to_string()), Crop::new("Coalroot".to_string(), "Coalroot".to_string(), 12, head_textures::COALROOT.to_string()), Crop::new("Creambloom".to_string(), "Creambloom".to_string(), 13, head_textures::CREAMBLOOM.to_string()), Crop::new("Duskbloom".to_string(), "duskbloom".to_string(), 14, head_textures::DUSKBLOOM.to_string()), Crop::new("Thornshade".to_string(), "thornshade".to_string(), 15, head_textures::THORNSHADE.to_string()), Crop::new("Blastberry".to_string(), "blastberry".to_string(), 16, head_textures::BLASTBERRY.to_string()), Crop::new("Cheesebite".to_string(), "cheesebite".to_string(), 17, head_textures::CHEESEBITE.to_string()), Crop::new("Chloronite".to_string(), "chloronite".to_string(), 18, head_textures::CHLORONITE.to_string()), Crop::new("Do Not Eat Shroom".to_string(), "donoteatshroom".to_string(), 19, head_textures::DO_NOT_EAT_SHROOM.to_string()), Crop::new("Fleshtrap".to_string(), "fleshtrap".to_string(), 20, head_textures::FLESHTRAP.to_string()), Crop::new("Magic Jellybean".to_string(), "magicjellybean".to_string(), 21, head_textures::MAGIC_JELLYBEAN.to_string()), Crop::new("Noctilume".to_string(), "noctilume".to_string(), 22, head_textures::NOCTILUME.to_string()), Crop::new("Snoozling".to_string(), "snoozlingFlower".to_string(), 23, head_textures::SNOOZLING.to_string()), Crop::new("Soggybud".to_string(), "soggybud".to_string(), 24, head_textures::SOGGYBUD.to_string()), Crop::new("Chorus Fruit".to_string(), "chorusFruit".to_string(), 25, head_textures::CHORUS_FRUIT.to_string()), Crop::new("PlantBoy Advance".to_string(), "Plantboy".to_string(), 26, head_textures::PLANTBOY_ADVANCE.to_string()), Crop::new("Puffercloud".to_string(), "puffercloud".to_string(), 27, head_textures::PUFFERCLOUD.to_string()), Crop::new("Shellfruit".to_string(), "shellfruit".to_string(), 28, head_textures::SHELLFRUIT.to_string()), Crop::new("Startlevine".to_string(), "startlevine".to_string(), 29, head_textures::STARTLEVINE.to_string()), Crop::new("Stoplight Petal".to_string(), "stoplightpetal".to_string(), 30, head_textures::STOPLIGHT_PETAL.to_string()), Crop::new("Thunderling".to_string(), "thunderling".to_string(), 31, head_textures::THUNDERLING.to_string()), Crop::new("Turtlellini".to_string(), "turtlellini".to_string(), 32, head_textures::TURTLELLINI.to_string()), Crop::new("Zombud".to_string(), "zombud".to_string(), 33, head_textures::ZOMBUD.to_string()), Crop::new("All In Aloe".to_string(), "allinaloe".to_string(), 34, head_textures::ALL_IN_ALOE.to_string()), Crop::new("Devourer".to_string(), "devourer".to_string(), 35, head_textures::DEVOURER.to_string()), Crop::new("Glasscorn".to_string(), "glasscorn".to_string(), 36, head_textures::GLASSCORN.to_string()), Crop::new("Godseed".to_string(), "godseed".to_string(), 37, head_textures::GODSEED.to_string()), Crop::new("Jerryflower".to_string(), "jerryseed".to_string(), 38, head_textures::JERRYFLOWER.to_string()), Crop::new("Phantomleaf".to_string(), "phantomleaf".to_string(), 39, head_textures::PHANTOMLEAF.to_string()), Crop::new("Timestalk".to_string(), "timestalk".to_string(), 40, head_textures::TIMESTALK.to_string()), Crop::new("Cocoa Beans".to_string(), "coco".to_string(), 41, head_textures::COCOA_BEANS.to_string()), Crop::new("Wild Rose".to_string(), "wildrose".to_string(), 42, head_textures::WILD_ROSE.to_string()), Crop::new("Moonflower".to_string(), "moonflower".to_string(), 43, head_textures::MOONFLOWER.to_string()), Crop::new("Sunflower".to_string(), "sunflower".to_string(), 44, head_textures::SUNFLOWER.to_string()), Crop::new("Cactus".to_string(), "cactus".to_string(), 45, head_textures::CACTUS.to_string()), Crop::new("Melon Seeds".to_string(), "melon".to_string(), 46, head_textures::MELON.to_string()), Crop::new("Pumpkin Seeds".to_string(), "pumpkin".to_string(), 47, head_textures::PUMPKIN.to_string()), Crop::new("Brown Mushroom".to_string(), "brownmushroom".to_string(), 48, head_textures::BROWN_MUSHROOM.to_string()), Crop::new("Red Mushroom".to_string(), "redmushroom".to_string(), 49, head_textures::RED_MUSHROOM.to_string()), Crop::new2("Wheat Seeds".to_string(), "wheat".to_string(), 50, blocks::blocks().get("WHEAT")), Crop::new2("Carrot".to_string(), "carrot".to_string(), 51, blocks::blocks().get("CARROTS")), Crop::new2("Potato".to_string(), "potato".to_string(), 52, blocks::blocks().get("POTATOES")), Crop::new2("Nether Wart".to_string(), "netherwart".to_string(), 53, blocks::blocks().get("NETHER_WART")), Crop::new2("Sugar Cane".to_string(), "sugarcane".to_string(), 54, blocks::blocks().get("SUGAR_CANE")), Crop::new2("Dead Plants".to_string(), "deadplant".to_string(), 55, blocks::blocks().get("DEAD_BUSH")), Crop::new2("Fire".to_string(), "fire".to_string(), 56, blocks::blocks().get("FIRE")), Crop::new("Helianthus".to_string(), "helianthus".to_string(), 57, head_textures::HELIANTHUS.to_string()), Crop::new("Fermento".to_string(), "fermento".to_string(), 58, head_textures::FERMENTO.to_string()), Crop::new("Squash".to_string(), "squash".to_string(), 59, head_textures::SQUASH.to_string()), Crop::new("Cropie".to_string(), "cropie".to_string(), 60, head_textures::CROPIE.to_string())];
            // TODO(fabric2seer): not translated (unexpected ']' (offset 4904))
            //   java: CROP_ID_MAP = Map.ofEntries(Stream.of(crops).map(c -> Map.entry(c.name(), c)).toArray(Map.Entry[]::new));
            // TODO(fabric2seer): not translated (unexpected ']' (offset 5010))
            //   java: CROP_BY_INT = Map.ofEntries(Stream.of(crops).map(c -> Map.entry(c.id(), c)).toArray(Map.Entry[]::new));
            // TODO(fabric2seer): not translated (unexpected ']' (offset 5157))
            //   java: CROP_BY_HEAD_TEXTURE_HASH = Map.ofEntries(Stream.of(crops).filter(Crop::isHead).map(c -> Map.entry(c.headSkin(), c)).toArray(Map.Entry[]::new));
        });
    }

    /// java: private GreenhouseCrops()
    pub fn new() -> Self {
        Self::clinit();
        let mut this = Self {
        };
        this
    }

}

/// java: public record Crop( String name, String armorStandName, int id, boolean isHead, Block cropBlock, @Nullable String headSkin, FlexibleItemStack displayStack )
#[derive(Clone)]
pub struct Crop {
    /// java: String name
    pub name: String,
    /// java: String armorStandName
    pub armor_stand_name: String,
    /// java: int id
    pub id: i32,
    /// java: boolean isHead
    pub is_head: bool,
    /// java: Block cropBlock
    pub crop_block: Arc<Block>,
    /// java: String headSkin
    pub head_skin: Option<String>,
    /// java: FlexibleItemStack displayStack
    pub display_stack: compat::Shared<FlexibleItemStack>,
}

impl Crop {
    /// java: public Crop(String name, String armorStandName, int id, String headSkin)
    pub fn new(mut name: String, mut armor_stand_name: String, mut id: i32, mut head_skin: String) -> Self {
        let mut this = Self::new_7(name.clone(), armor_stand_name.clone(), id, true, blocks::blocks().get("AIR"), Some(head_skin.clone()), ItemUtils::create_skull(&head_skin));
        this
    }

    /// java: public Crop(String name, String armorStandName, int id, Block cropBlock)
    pub fn new2(mut name: String, mut armor_stand_name: String, mut id: i32, mut crop_block: Arc<Block>) -> Self {
        let mut this = Self::new_7(name.clone(), armor_stand_name.clone(), id, false, crop_block.clone(), None, flexible_item_stack::EMPTY.get_mut().clone());
        this
    }

    /// java: the canonical record constructor
    pub fn new_7(mut name: String, mut armor_stand_name: String, mut id: i32, mut is_head: bool, mut crop_block: Arc<Block>, mut head_skin: Option<String>, mut display_stack: compat::Shared<FlexibleItemStack>) -> Self {
        Self { name, armor_stand_name, id, is_head, crop_block, head_skin, display_stack }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `armorStandName()`
    pub fn armor_stand_name(&self) -> String {
        self.armor_stand_name.clone()
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> i32 {
        self.id
    }

    /// java: the record accessor `isHead()`
    pub fn is_head(&self) -> bool {
        self.is_head
    }

    /// java: the record accessor `cropBlock()`
    pub fn crop_block(&self) -> Arc<Block> {
        self.crop_block.clone()
    }

    /// java: the record accessor `headSkin()`
    pub fn head_skin(&self) -> Option<String> {
        self.head_skin.clone()
    }

    /// java: the record accessor `displayStack()`
    pub fn display_stack(&self) -> compat::Shared<FlexibleItemStack> {
        self.display_stack.clone()
    }

}
