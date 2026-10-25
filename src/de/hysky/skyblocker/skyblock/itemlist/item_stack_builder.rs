//! Ported from de.hysky.skyblocker.skyblock.itemlist.ItemStackBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::text_transformer::TextTransformer;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use crate::de::hysky::skyblocker::utils::datafixer::legacy_item_stack_fixer::LegacyItemStackFixer;
use crate::de::hysky::skyblocker::utils::datafixer::legacy_string_nbt_reader::LegacyStringNbtReader;
use crate::io::github::moulberry::repo::constants::pet_numbers::PetNumbers;
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::data::rarity::Rarity;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;


/// java: private static Map<String, Map<Rarity, PetNumbers>> petNums = Map.of();
pub static PET_NUMS: compat::MainThread<std::collections::HashMap<String, std::collections::HashMap<Rarity, PetNumbers>>> = compat::MainThread::new(|| std::collections::HashMap::from([]));

/// java: public class ItemStackBuilder
#[derive(Clone)]
pub struct ItemStackBuilder {}

impl ItemStackBuilder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void loadPetNums()
    pub fn load_pet_nums() {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // java: petNums = NEURepoManager.getConstants().getPetNumbers();
            let pet_numbers: std::collections::HashMap<String, std::collections::HashMap<Rarity, PetNumbers>> = (*NEURepoManager::get_constants().get_pet_numbers()).clone();
            *PET_NUMS.get_mut() = pet_numbers;
        }));
        if let Err(_) = __result {
            logger().error("[Skyblocker ItemStackBuilder] Failed to load petnums.json");
        }
    }

    /// java: protected static FlexibleItemStack fromNEUItem(NEUItem item)
    pub fn from_neu_item(item: &NEUItem) -> compat::Shared<FlexibleItemStack> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut nbt: Tag = Tag::compound();
            // java: CompoundTag tag = LegacyStringNbtReader.parse(item.getNbttag());
            let mut tag: Tag = LegacyStringNbtReader::parse(&item.get_nbttag().to_string());
            nbt.put("tag", tag.clone());
            // java: nbt.putString("id", item.getMinecraftItemId());
            nbt.put_string("id", &item.get_minecraft_item_id().to_string());
            // java: nbt.putShort("Damage", (short) item.getDamage());
            nbt.put_short("Damage", item.get_damage() as i16);
            nbt.put_int("Count", 1);
            // java: FlexibleItemStack stack = LegacyItemStackFixer.fixLegacyStack(nbt, FlexibleItemStack.CODEC, FlexibleItemStack.EMPTY, FlexibleItemStack::set);
            let mut stack: compat::Shared<FlexibleItemStack> = LegacyItemStackFixer::fix_legacy_stack(&nbt, flexible_item_stack::CODEC.get(), flexible_item_stack::EMPTY.get(), &|s: &mut compat::Shared<FlexibleItemStack>, r#type, value| FlexibleItemStack::set_obj(s, r#type, value));
            // java: if (stack.is(Items.AIR)) {
            if Arc::ptr_eq(&flexible_item_stack::item_of(&FlexibleItemStack::type_holder(&stack)), &items::items().get("air")) {
                // java: LOGGER.error("[Skyblocker ItemStackBuilder] Failed to build item with skyblock id: {}!", item.getSkyblockItemId());
                logger().error(&format!("[Skyblocker ItemStackBuilder] Failed to build item with skyblock id: {}!", item.get_skyblock_item_id()));
                // java: return createErrorStack(item.getSkyblockItemId());
                return ItemStackBuilder::create_error_stack(&item.get_skyblock_item_id().to_string());
            }
            // java: List<Pair<String, String>> injectors = new ArrayList<>(petData(item.getSkyblockItemId()));
            let mut injectors: Vec<(String, String)> = ItemStackBuilder::pet_data(&item.get_skyblock_item_id().to_string());
            // java: String name = injectData(item.getDisplayName(), injectors);
            let mut name: String = ItemStackBuilder::inject_data(item.get_display_name().to_string(), &injectors);
            // java: stack.set(DataComponents.CUSTOM_NAME, TextTransformer.fromLegacy(name));
            // (the port's CUSTOM_NAME value is a `FormattedText` - `ItemStack.getCustomName` casts to it)
            FlexibleItemStack::set(&stack, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), FormattedText::from(TextTransformer::from_legacy(&name)));
            //   java: stack.set(DataComponents.LORE, new ItemLore(item.getLore().stream()
            //   java: 					.map(line -> TextTransformer.fromLegacy(injectData(line, injectors)))
            //   java: 					.map(Component.class::cast)
            //   java: 					.toList()));
            // (the port's LORE lines are `FormattedText`, as CUSTOM_NAME's; `new ItemLore` throws past 256 lines - the
            // catch below)
            let lore_lines: Vec<FormattedText> = item.get_lore().iter().map(|line| FormattedText::from(TextTransformer::from_legacy(&ItemStackBuilder::inject_data(line.to_string(), &injectors)))).collect();
            FlexibleItemStack::set(&stack, mcrust::net::minecraft::core::component::data_components::lore().clone(), ItemLore::new(lore_lines).unwrap_or_else(|e| panic!("IllegalArgumentException: {e:?}")));
            return stack;
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                // java: LOGGER.error("[Skyblocker ItemStackBuilder] Failed to build item with skyblock id: {}!", item.getSkyblockItemId(), e);
                logger().error(&format!("[Skyblocker ItemStackBuilder] Failed to build item with skyblock id: {}! {}", item.get_skyblock_item_id(), compat::panic_message(&e)));
            }
        }
        // java: return createErrorStack(item.getSkyblockItemId());
        return ItemStackBuilder::create_error_stack(&item.get_skyblock_item_id().to_string());
    }

    /// java: private static FlexibleItemStack createErrorStack(String skyblockItemId)
    pub fn create_error_stack(skyblock_item_id: &str) -> compat::Shared<FlexibleItemStack> {
        let mut error_stack: compat::Shared<FlexibleItemStack> = FlexibleItemStack::new2(items::items().get("barrier"));
        // (the port's CUSTOM_NAME value is a `FormattedText`)
        FlexibleItemStack::set(&error_stack, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), FormattedText::from(component::null_to_empty(Some(skyblock_item_id))));
        return error_stack;
    }

    /// java: private static List<Pair<String, String>> petData(String internalName)
    pub fn pet_data(internal_name: &str) -> Vec<(String, String)> {
        let mut list: Vec<(String, String)> = Vec::new();
        let mut pet_name: String = (compat::regex::split(&internal_name, ";"))[(0) as usize].clone();
        if !(internal_name.contains(";")) || !(PET_NUMS.get_mut().contains_key(&pet_name)) {
            return list;
        }
        //   java: final Rarity[] rarities = {
        //   java: 				Rarity.COMMON,
        //   java: 				Rarity.UNCOMMON,
        //   java: 				Rarity.RARE,
        //   java: 				Rarity.EPIC,
        //   java: 				Rarity.LEGENDARY,
        //   java: 				Rarity.MYTHIC,
        //   java: 				Rarity.DIVINE, // unused
        //   java: 				Rarity.SPECIAL, // phoenix
        //   java: 				Rarity.VERY_SPECIAL // phoenix
        //   java: 		};
        let mut rarities: Vec<Rarity> = vec![Rarity::Common, Rarity::Uncommon, Rarity::Rare, Rarity::Epic, Rarity::Legendary, Rarity::Mythic, Rarity::Divine, Rarity::Special, Rarity::VerySpecial];
        // java: Rarity rarity = rarities[Integer.parseInt(internalName.split(";")[1])]; - an index past the array throws
        let rarity = &rarities[(((compat::regex::split(&internal_name, ";"))[(1) as usize]).parse::<i32>().expect("NumberFormatException")) as usize];
        // java: PetNumbers data = petNums.get(petName).get(rarity); - a rarity the pet has no numbers for is a null
        // `data`, and `data.getLowLevel()` the NullPointerException the caller's catch logs
        let mut data: PetNumbers = PET_NUMS.get().get(&pet_name).and_then(|by_rarity| by_rarity.get(rarity).cloned()).expect("NullPointerException");
        //   java: int minLevel = data.getLowLevel();
        let mut min_level: i32 = data.get_low_level();
        //   java: int maxLevel = data.getHighLevel();
        let mut max_level: i32 = data.get_high_level();
        list.push(("\\{LVL\\}".to_string(), format!("{} ➡ {}", min_level, max_level)));
        //   java: Map<String, Double> statNumsMin = data.getStatsAtLowLevel().getStatNumbers();
        let mut stat_nums_min: std::collections::HashMap<String, f64> = data.get_stats_at_low_level().get_stat_numbers().clone();
        //   java: Map<String, Double> statNumsMax = data.getStatsAtHighLevel().getStatNumbers();
        let mut stat_nums_max: std::collections::HashMap<String, f64> = data.get_stats_at_high_level().get_stat_numbers().clone();
        // java: Set<Map.Entry<String, Double>> entrySet = statNumsMin.entrySet(); - the entries, iterated
        let mut entry_set: Vec<(String, f64)> = stat_nums_min.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>();
        for entry in entry_set.iter() {
            let mut key: String = entry.0.clone();
            let mut left: String = format!("\\{{{}\\}}", key);
            let mut right: String = format!("{} ➡ {}", compat::opt_str(&stat_nums_min.get(&key).cloned()), compat::opt_str(&stat_nums_max.get(&key).cloned()));
            list.push((left.clone(), right.clone()));
        }
        //   java: List<Double> otherNumsMin = data.getStatsAtLowLevel().getOtherNumbers();
        let mut other_nums_min: Vec<f64> = data.get_stats_at_low_level().get_other_numbers().clone();
        //   java: List<Double> otherNumsMax = data.getStatsAtHighLevel().getOtherNumbers();
        let mut other_nums_max: Vec<f64> = data.get_stats_at_high_level().get_other_numbers().clone();
        for i in 0..(other_nums_min.len() as i32) {
            let mut left: String = format!("\\{{{}\\}}", i);
            let mut right: String = format!("{} ➡ {}", compat::fstr(other_nums_min[(i) as usize] as f64), compat::fstr(other_nums_max[(i) as usize] as f64));
            list.push((left.clone(), right.clone()));
        }
        return list;
    }

    /// java: private static String injectData(String string, List<Pair<String, String>> injectors)
    pub fn inject_data(mut string: String, injectors: &Vec<(String, String)>) -> String {
        for injector in injectors.iter() {
            string = compat::regex::replace_all(&string, &injector.0, &injector.1);
        }
        return string;
    }

}
