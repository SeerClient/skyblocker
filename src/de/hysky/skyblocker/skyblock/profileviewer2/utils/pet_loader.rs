//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.PetLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::pets_data::Pet;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::Cap;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::Progress;
use crate::de::hysky::skyblocker::utils::flexible_item_stack;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::text_transformer::TextTransformer;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;

/// java: private static final Comparator<PetsData.Pet> PETS_COMPARATOR = Comparator.comparing((PetsData.Pet pet) -> SkyblockItemRarity.valueOf(pet.tier)) .reversed() .thenComparing(pet -> pet.type);
pub static PETS_COMPARATOR: compat::MainThread<Arc<dyn Fn(&compat::Shared<Pet>, &compat::Shared<Pet>) -> std::cmp::Ordering>> = compat::MainThread::new(|| Arc::new(|a: &compat::Shared<Pet>, b: &compat::Shared<Pet>| {
    let rarity = |p: &compat::Shared<Pet>| crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity::value_of(&compat::read(p, |t| t.tier.clone()));
    rarity(b).cmp(&rarity(a)).then_with(|| compat::read(a, |t| t.r#type.clone()).cmp(&compat::read(b, |t| t.r#type.clone())))
}));
/// java: private static final Pattern STATS_NUMBER_PATTERN = Pattern.compile("\\{(?<name>[A-Za-z_]+)\\}");
pub static STATS_NUMBER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\{(?<name>[A-Za-z_]+)\\}"));
/// java: private static final Pattern OTHER_NUMBER_PATTERN = Pattern.compile("\\{(?<index>\\d+)\\}");
pub static OTHER_NUMBER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\{(?<index>\\d+)\\}"));
/// java: private static final Style NO_ITALIC = Style.EMPTY.withItalic(false);
pub static NO_ITALIC: compat::MainThread<Style> = compat::MainThread::new(|| mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_italic(Some(false)));

/// java: public class PetLoader
#[derive(Clone)]
pub struct PetLoader {}

impl PetLoader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static List<ItemStack> parsePets(List<PetsData.Pet> pets)
    pub fn parse_pets(pets: &Vec<compat::Shared<Pet>>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut pets_sorted: Vec<compat::Shared<Pet>> = pets.iter().cloned().collect::<Vec<_>>();
        pets_sorted.sort_by(|a, b| (PETS_COMPARATOR.get_mut())(a, b));
        let mut pet_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        for pet in pets_sorted.iter() {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                pet_items.push(PetLoader::parse_pet(pet));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Profile Viewer Pet Loader] Failed to load pet: {}.", compat::read(pet, |__p| __p.to_string())));
            }
        }
        return pet_items.iter().cloned().collect::<Vec<_>>();
    }

    /// java: private static ItemStack parsePet(PetsData.Pet pet)
    pub fn parse_pet(pet: &compat::Shared<Pet>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        if NEURepoManager::is_loading() || !(ItemRepository::files_imported()) {
            return FlexibleItemStack::get_stack_or_throw(&ItemUtils::get_named_placeholder("NEU Repo / Items not loaded"));
        }
        let mut pet_rarity: SkyblockItemRarity = PetLoader::get_pet_rarity(pet);
        let mut neu_pet_rarity: crate::io::github::moulberry::repo::data::rarity::Rarity = pet_rarity.to_neu_rarity();
        let mut neu_pet_id: String = format!("{};{}", compat::read(pet, |__t| __t.r#type.clone()), PetLoader::get_rarity_index(pet_rarity));
        let mut base_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&neu_pet_id);
        let mut pet_item: Option<Arc<crate::io::github::moulberry::repo::data::neu_item::NEUItem>> = NEURepoManager::get_item_by_neu_id(&neu_pet_id);
        if base_stack.is_none() || pet_item.is_none() {
            logger().warn(&format!("[Skyblocker Profile Viewer Pet Loader] Failed to get pet data for {}.", neu_pet_id));
            return FlexibleItemStack::get_stack_or_throw(&ItemUtils::get_item_id_placeholder(&neu_pet_id));
        }
        //   java: PetLevelingData petLevellingData = NEURepoManager.getConstants().getPetLevelingData();
        let mut pet_levelling_data: Arc<crate::io::github::moulberry::repo::constants::pet_leveling_data::PetLevelingData> = NEURepoManager::get_constants().get_pet_leveling_data();
        //   java: Map<@PetId String, Map<Rarity, PetNumbers>> petNumbersData = NEURepoManager.getConstants().getPetNumbers();
        let mut pet_numbers_data: Arc<crate::io::github::moulberry::repo::neu_constants::PetNumbersMap> = NEURepoManager::get_constants().get_pet_numbers();
        // Ensure the pet has the numbers data for the given rarity
        let pet_type: String = compat::read(pet, |__t| __t.r#type.clone());
        if !(pet_numbers_data.contains_key(&pet_type)) || !(pet_numbers_data.get(&pet_type).unwrap().contains_key(&neu_pet_rarity)) {
            logger().warn(&format!("[Skyblocker Profile Viewer Pet Loader] Mising pet numbers for {}.", pet_type));
            return FlexibleItemStack::get_stack_or_throw(&ItemUtils::get_item_id_placeholder(&pet_type));
        }
        let mut level_info: LevelInfo = PetLoader::calculate_pet_level(pet, &neu_pet_rarity, &pet_levelling_data);
        //   java: PetNumbers petNumbers = petNumbersData.get(pet.type).get(neuPetRarity);
        let mut pet_numbers: crate::io::github::moulberry::repo::constants::pet_numbers::PetNumbers = pet_numbers_data.get(&pet_type).unwrap().get(&neu_pet_rarity).cloned().unwrap();
        let mut name: MutableComponent = PetLoader::format_name(pet_item.as_ref().unwrap(), level_info.level());
        let mut lore: Vec<MutableComponent> = PetLoader::append_lore_extras(pet, &level_info, &mut PetLoader::format_lore(pet_item.as_ref().unwrap(), &pet_numbers, level_info.level()));
        let mut head: Option<ResolvableProfile> = PetLoader::get_head_icon(pet, FlexibleItemStack::get::<ResolvableProfile>(base_stack.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::profile()));
        let mut pet_stack: compat::Shared<FlexibleItemStack> = FlexibleItemStack::copy(base_stack.as_ref().unwrap());
        // (CUSTOM_NAME's value is a Component: the port's is a `FormattedText`)
        FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(name.clone()));
        FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::lore().clone(), ItemLore::new(lore.iter().cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect()).unwrap());
        // java: petStack.set(DataComponents.PROFILE, head); - a null head is the absent component it already is
        if let Some(head) = head {
            FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::profile().clone(), head);
        }
        return FlexibleItemStack::get_stack_or_throw(&pet_stack);
    }

    /// java: private static LevelInfo calculatePetLevel(PetsData.Pet pet, Rarity neuPetRarity, PetLevelingData petLevellingData)
    pub fn calculate_pet_level(pet: &compat::Shared<Pet>, neu_pet_rarity: &crate::io::github::moulberry::repo::data::rarity::Rarity, pet_levelling_data: &crate::io::github::moulberry::repo::constants::pet_leveling_data::PetLevelingData) -> LevelInfo {
        use crate::io::github::moulberry::repo::constants::pet_leveling_behaviour_override::PetExpModifierType;
        let mut max_level: i32 = 100;
        //   java: int xpCostsOffset = petLevellingData.getPetLevelStartOffset().get(neuPetRarity); - unboxing a missing one is an NPE
        let mut xp_costs_offset: i32 = *pet_levelling_data.get_pet_level_start_offset().get(neu_pet_rarity).expect("java.lang.NullPointerException: pet level start offset");
        //   java: List<Integer> allXpCosts = new ArrayList<>(petLevellingData.getPetExpCostForLevel());
        let mut all_xp_costs: Vec<i32> = pet_levelling_data.get_pet_exp_cost_for_level().to_vec();
        let pet_type: String = compat::read(pet, |__t| __t.r#type.clone());
        //   java: if (petLevellingData.getPetLevelingBehaviourOverrides().containsKey(pet.type)) {
        if let Some(r#override) = pet_levelling_data.get_pet_leveling_behaviour_overrides().get(&pet_type) {
            // Adjust max level
            //   java: if (override.getMaxLevel() != null) maxLevel = override.getMaxLevel().intValue();
            if let Some(override_max_level) = r#override.get_max_level() {
                max_level = override_max_level;
            }
            // Apply allXpCosts override
            //   java: if (override.getPetExpCostModifierType() != null && override.getPetExpCostModifier() != null) switch (..) {
            //   java:     case APPEND -> allXpCosts.addAll(override.getPetExpCostModifier());
            //   java:     case REPLACE -> allXpCosts = override.getPetExpCostModifier(); }
            if let (Some(modifier_type), Some(modifier)) = (r#override.get_pet_exp_cost_modifier_type(), r#override.get_pet_exp_cost_modifier()) {
                match modifier_type {
                    PetExpModifierType::Append => all_xp_costs.extend(modifier.iter().copied()),
                    PetExpModifierType::Replace => all_xp_costs = modifier.clone(),
                }
            }
            // Apply xpCostsOffset override
            //   java: if (override.getPetLevelStartOffset() != null) xpCostsOffset = override.getPetLevelStartOffset().get(neuPetRarity);
            if let Some(offsets) = r#override.get_pet_level_start_offset() {
                xp_costs_offset = *offsets.get(neu_pet_rarity).expect("java.lang.NullPointerException: pet level start offset");
            }
        }
        let mut xp_costs: Vec<i32> = all_xp_costs[(xp_costs_offset) as usize..((xp_costs_offset.wrapping_add(max_level)).wrapping_sub(1)) as usize].to_vec();
        let mut level: i32 = 1;
        let mut remaining_xp: i64 = ((compat::read(pet, |__t| __t.exp)) as i64);
        for xp_required in xp_costs.iter().copied() {
            if remaining_xp >= (xp_required as i64) {
                level += 1;
                remaining_xp = remaining_xp.wrapping_sub(xp_required as i64);
            } else {
                break;
            }
        }
        let mut is_max_level: bool = (level == max_level);
        let mut progress: Option<Progress> = None;
        if !is_max_level {
            let mut xp_for_next_level: i64 = (xp_costs[(level.wrapping_sub(1)) as usize]) as i64;
            progress = Some(Progress::new(remaining_xp, xp_for_next_level));
        }
        let mut info: LevelInfo = LevelInfo::new(((compat::read(pet, |__t| __t.exp)) as i64), level, Cap::new(max_level, max_level), progress.clone());
        return info;
    }

    /// java: private static Component formatName(NEUItem petItem, int petLevel)
    pub fn format_name(pet_item: &crate::io::github::moulberry::repo::data::neu_item::NEUItem, mut pet_level: i32) -> MutableComponent {
        //   java: String name = petItem.getDisplayName();
        let mut name: String = pet_item.get_display_name().to_string();
        name = name.replace("{LVL}", &pet_level.to_string());
        return TextTransformer::from_legacy(&name);
    }

    /// java: private static List<Component> formatLore(NEUItem petItem, PetNumbers petNumbers, int petLevel)
    pub fn format_lore(pet_item: &crate::io::github::moulberry::repo::data::neu_item::NEUItem, pet_numbers: &crate::io::github::moulberry::repo::constants::pet_numbers::PetNumbers, mut pet_level: i32) -> Vec<MutableComponent> {
        //   java: List<String> unformattedLore = petItem.getLore();
        let mut unformatted_lore: Vec<String> = pet_item.get_lore().to_vec();
        let mut formatted_lore: Vec<String> = Vec::new();
        //   java: PetNumbers.Stats stats = petNumbers.interpolatedStatsAtLevel(petLevel); - null outside the pet's levels, and
        //   the first read of it the NullPointerException
        let mut stats: Option<crate::io::github::moulberry::repo::constants::pet_numbers::Stats> = pet_numbers.interpolated_stats_at_level(pet_level);
        let mut skip_next: bool = false;
        for line in unformatted_lore.iter() {
            let mut is_rc: bool = (line.contains("Right-click to add this") || line.contains("pet menu!"));
            if is_rc || line.contains("Can be upgraded at") || skip_next {
                skip_next = is_rc;
                continue;
            }
            let mut stats_number_matcher: compat::regex::Matcher = STATS_NUMBER_PATTERN.matcher(line);
            let mut stats_replacements: std::collections::HashSet<String> = std::collections::HashSet::new();
            while stats_number_matcher.find() {
                let mut stat_name: String = stats_number_matcher.group_named("name").unwrap();
                stats_replacements.insert(stat_name.clone());
            }
            let mut other_number_matcher: compat::regex::Matcher = OTHER_NUMBER_PATTERN.matcher(line);
            let mut other_number_replacements: std::collections::HashSet<i32> = std::collections::HashSet::new();
            while other_number_matcher.find() {
                let mut index: i32 = other_number_matcher.group_named("index").unwrap().parse::<i32>().expect("NumberFormatException");
                other_number_replacements.insert(index);
            }
            let mut formatted_line: String = line.to_string();
            for stat_name in stats_replacements.iter() {
                //   java: double replacement = stats.getStatNumbers().get(statName).doubleValue();
                let mut replacement: f64 = *stats
                    .as_ref()
                    .expect("java.lang.NullPointerException: PetNumbers.Stats stats")
                    .get_stat_numbers()
                    .get(stat_name)
                    .expect("java.lang.NullPointerException: stat number");
                formatted_line = formatted_line.replace(&format!("{{{}}}", stat_name), &formatters::TRIPLE_NUMBERS.get_mut().format(replacement));
            }
            for index in other_number_replacements.iter().copied() {
                //   java: double replacement = stats.getOtherNumbers().get(index).doubleValue();
                let mut replacement: f64 = *stats
                    .as_ref()
                    .expect("java.lang.NullPointerException: PetNumbers.Stats stats")
                    .get_other_numbers()
                    .get(index as usize)
                    .expect("java.lang.IndexOutOfBoundsException: other number");
                formatted_line = formatted_line.replace(&format!("{{{}}}", index), &formatters::TRIPLE_NUMBERS.get_mut().format(replacement));
            }
            formatted_lore.push(formatted_line.clone());
        }
        let mut component_lore: Vec<MutableComponent> = Vec::new();
        for line in formatted_lore.iter() {
            component_lore.push(TextTransformer::from_legacy(line));
        }
        return component_lore;
    }

    /// java: private static List<Component> appendLoreExtras(PetsData.Pet pet, LevelInfo levelInfo, List<Component> lore)
    pub fn append_lore_extras(pet: &compat::Shared<Pet>, level_info: &LevelInfo, lore: &mut Vec<MutableComponent>) -> Vec<MutableComponent> {
        if compat::read(pet, |__t| __t.skin.clone()).is_some() {
            let skin_stack = ItemRepository::get_item_stack_2(&format!("PET_SKIN_{}", compat::read(pet, |__t| __t.skin.clone()).unwrap()), crate::de::hysky::skyblocker::utils::flexible_item_stack::EMPTY.get());
            let pet_skin_name: String = FlexibleItemStack::get::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(&skin_stack, mcrust::net::minecraft::core::component::data_components::custom_name()).map(|n| n.get_string()).unwrap_or_default();
            let mut new_first_line: MutableComponent = component::empty().with_style(NO_ITALIC.get_mut()).append(lore[0].clone()).append((component::literal(&format!(", {}", pet_skin_name))).with_style_format(ChatFormatting::DarkGray));
            lore[(0) as usize] = new_first_line.clone();
        }
        if compat::read(pet, |__t| __t.held_item.clone()).is_some() {
            let mut held_item: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(compat::read(pet, |__t| __t.held_item.clone()).as_deref().unwrap());
            // java: heldItem != null && heldItem.get(DataComponents.CUSTOM_NAME) != null
            if let Some(held_item) = &held_item && let Some(held_name) = FlexibleItemStack::get::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(held_item, mcrust::net::minecraft::core::component::data_components::custom_name()) {
                let mut text: MutableComponent = component::empty().with_style(NO_ITALIC.get_mut()).append(component::literal("Held Item: ").with_style_format(ChatFormatting::Gold)).append(compat::component_of(held_name));
                lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, text.clone());
                lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, component::empty());
            }
        }
        if level_info.is_level_absolutely_maxed() {
            lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, component::literal("MAX LEVEL").with_style(NO_ITALIC.get_mut()).with_style_formats(&[ChatFormatting::Aqua, ChatFormatting::Bold]));
            lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, ((component::literal(&format!("▸ {} XP", formatters::INTEGER_NUMBERS.get_mut().format((level_info.xp()) as f64)))).with_style(NO_ITALIC.get_mut())).with_style_format(ChatFormatting::DarkGray));
            lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, component::empty());
        } else if level_info.progress().is_some() {
            let mut progress: Progress = level_info.progress().unwrap();
            let mut next_level: i32 = level_info.level().wrapping_add(1);
            let mut progress_text: MutableComponent = (component::empty().with_style(NO_ITALIC.get_mut()).append((component::literal(&format!("Progress to Level {}: ", next_level))).with_style_format(ChatFormatting::Gray))).append((component::literal(&format!("{}%", formatters::FLOAT_NUMBERS.get_mut().format((progress.percentage_to_next_level() * (100.0f32 as f64)))))).with_style_format(ChatFormatting::Yellow));
            let mut bar_text: MutableComponent = (((((component::empty().with_style(NO_ITALIC.get_mut()).append((component::literal(&" ".repeat((((compat::java_round_f64((progress.percentage_to_next_level() * (30.0f32 as f64)))) as i32)) as usize))).with_style_formats(&[ChatFormatting::DarkGreen, ChatFormatting::Strikethrough]))).append((component::literal(&" ".repeat((30i32.wrapping_sub((compat::java_round_f64((progress.percentage_to_next_level() * (30.0f32 as f64)))) as i32)) as usize))).with_style_formats(&[ChatFormatting::White, ChatFormatting::Strikethrough]))).append_str(" ")).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format((progress.xp_progress()) as f64))).with_style_format(ChatFormatting::Yellow))).append((component::literal("/")).with_style_format(ChatFormatting::Gold))).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format((progress.xp_needed()) as f64))).with_style_format(ChatFormatting::Yellow));
            lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, progress_text.clone());
            lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, bar_text.clone());
            lore.insert(((lore.len() as i32).wrapping_sub(1)) as usize, component::empty());
        }
        return lore.clone();
    }

    /// java: private static ResolvableProfile getHeadIcon(PetsData.Pet pet, ResolvableProfile original)
    /// (`original` is the base stack's PROFILE component: `@Nullable`)
    pub fn get_head_icon(pet: &compat::Shared<Pet>, original: Option<ResolvableProfile>) -> Option<ResolvableProfile> {
        if compat::read(pet, |__t| __t.skin.clone()).is_some() {
            let mut pet_skin: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&format!("PET_SKIN_{}", compat::opt_str(&compat::read(pet, |__t| __t.skin.clone()))));
            // java: petSkin != null && petSkin.get(DataComponents.PROFILE) != null
            if let Some(pet_skin) = &pet_skin && let Some(profile) = FlexibleItemStack::get::<ResolvableProfile>(pet_skin, mcrust::net::minecraft::core::component::data_components::profile()) {
                return Some(profile);
            }
        }
        return original;
    }

    /// java: private static SkyblockItemRarity getPetRarity(PetsData.Pet pet)
    pub fn get_pet_rarity(pet: &compat::Shared<Pet>) -> SkyblockItemRarity {
        let mut rarity: SkyblockItemRarity = SkyblockItemRarity::value_of(&compat::read(pet, |__t| __t.tier.clone()));
        if Some(&*("PET_ITEM_TIER_BOOST")) == compat::read(pet, |__t| __t.held_item.clone()).as_deref() && compat::ord_int(rarity.cmp(&SkyblockItemRarity::Legendary)) > 0 {
            rarity = rarity.next();
        }
        return rarity;
    }

    /// java: private static int getRarityIndex(SkyblockItemRarity rarity)
    pub fn get_rarity_index(mut rarity: SkyblockItemRarity) -> i32 {
        return match rarity {
            SkyblockItemRarity::Common => 0,
            SkyblockItemRarity::Uncommon => 1,
            SkyblockItemRarity::Rare => 2,
            SkyblockItemRarity::Epic => 3,
            SkyblockItemRarity::Legendary => 4,
            SkyblockItemRarity::Mythic => 5,
            _ => {
                panic!("IllegalArgumentException: {}", format!("Rarity {} is not supported!", rarity).to_string());
            }
        };
    }

}
