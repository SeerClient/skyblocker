//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.Pet by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelFinder;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::profile_viewer_utils::ProfileViewerUtils;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern statsMatcher = Pattern.compile("\\{[A-Za-z_]+}");
pub static STATS_MATCHER: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\{[A-Za-z_]+}"));
/// java: private static final Pattern numberMatcher = Pattern.compile("\\{\\d+}");
pub static NUMBER_MATCHER: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\{\\d+}"));

/// java: public class Pet
#[derive(Clone)]
pub struct Pet {
    /// java: private final String name;
    pub name: String,
    /// java: private final double xp;
    pub xp: f64,
    /// java: private final SkyblockItemRarity tier;
    pub tier: Option<SkyblockItemRarity>,
    /// java: private final Optional<String> heldItem;
    pub held_item: Option<String>,
    /// java: private final Optional<String> skin;
    pub skin: Option<String>,
    /// java: private final Optional<ResolvableProfile> skinTexture;
    pub skin_texture: Option<ResolvableProfile>,
    /// java: private final int level;
    pub level: i32,
    /// java: private final double perecentageToLevel;
    pub perecentage_to_level: f64,
    /// java: private final long levelXP;
    pub level_xp: i64,
    /// java: private final long nextLevelXP;
    pub next_level_xp: i64,
    /// java: private final FlexibleItemStack icon;
    pub icon: Option<compat::Shared<FlexibleItemStack>>,
}

impl Pet {
    /// java: public Pet(PetInfo petData)
    pub fn new(mut pet_data: PetInfo) -> Self {
        let mut info: compat::Shared<LevelInfo> = LevelFinder::get_level_info(&(if pet_data.r#type() == "GOLDEN_DRAGON" { "PET_GREG".to_string() } else { format!("PET_{}", compat::jstring(&pet_data.tier())) }), ((pet_data.exp()) as i64));
        let __init_name = pet_data.r#type();
        let __init_xp = pet_data.exp();
        let __init_held_item = pet_data.item();
        let __init_skin = pet_data.skin();
        let mut this = Self {
            name: __init_name,
            xp: __init_xp,
            tier: None,
            held_item: __init_held_item,
            skin: __init_skin,
            skin_texture: Default::default(),
            level: 0,
            perecentage_to_level: 0.0,
            level_xp: 0,
            next_level_xp: 0,
            icon: None,
        };
        this.skin_texture = this.calculate_skin_texture();
        this.tier = Some(pet_data.tier());
        this.level = compat::read(&info, |__t| __t.level);
        this.perecentage_to_level = compat::read(&info, |__t| __t.fill);
        this.level_xp = compat::read(&info, |__t| __t.level_xp);
        this.next_level_xp = compat::read(&info, |__t| __t.next_level_xp);
        this.icon = Some(this.create_icon());
        this
    }

    /// java: private String getName()
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    /// java: public long getXP()
    pub fn get_xp(&self) -> i64 {
        return ((self.xp) as i64);
    }

    /// java: public SkyblockItemRarity getRarity()
    pub fn get_rarity(&self) -> SkyblockItemRarity {
        return self.tier.clone().unwrap();
    }

    /// java: public int getTier()
    pub fn get_tier(&self) -> i32 {
        return self.tier.as_ref().unwrap().ordinal();
    }

    /// java: private Optional<ResolvableProfile> calculateSkinTexture()
    pub fn calculate_skin_texture(&self) -> Option<ResolvableProfile> {
        if self.skin.is_some() {
            let mut item: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&format!("PET_SKIN_{}", self.skin.clone().unwrap()));
            if item.is_none() || FlexibleItemStack::is_empty(item.as_ref().unwrap()) {
                return None;
            }
            let mut profile: Option<ResolvableProfile> = FlexibleItemStack::get::<ResolvableProfile>(item.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::profile());
            return (if profile.is_some() { profile.clone() } else { None });
        }
        return None;
    }

    /// java: public int getLevel()
    pub fn get_level(&self) -> i32 {
        return self.level;
    }

    /// java: public FlexibleItemStack getIcon()
    pub fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return self.icon.clone().unwrap();
    }

    /// java: private FlexibleItemStack createIcon()
    pub fn create_icon(&self) -> compat::Shared<FlexibleItemStack> {
        if NEURepoManager::is_loading() || !(ItemRepository::files_imported()) {
            return ico::BARRIER.get_mut().clone();
        }
        let mut target_item_id: String = format!("{};{}", self.get_name(), self.get_tier().wrapping_add(if self.held_item.is_some() && self.held_item.clone().unwrap() == "PET_ITEM_TIER_BOOST" { 1 } else { 0 }));
        let mut item: Option<crate::unresolved::NEUItem> = NEURepoManager::get_item_by_neu_id(&target_item_id);
        if item.is_none() && self.held_item.is_some() && self.held_item.clone().unwrap() == "PET_ITEM_TIER_BOOST" {
            item = NEURepoManager::get_item_by_neu_id(&format!("{};{}", self.get_name(), self.get_tier()));
        }
        // java: fromNEUItem(item, this.heldItem.map(ItemRepository::getItemStack).orElse(null)) - null without a held item
        return (if item.is_none() { ico::BARRIER.get_mut().clone() } else { self.from_neu_item(item.as_ref().unwrap(), self.held_item.clone().and_then(|__a0| ItemRepository::get_item_stack(&__a0)).as_ref()) });
    }

    /// java: private FlexibleItemStack fromNEUItem(NEUItem item, FlexibleItemStack heldItem)
    pub fn from_neu_item(&self, item: &crate::unresolved::NEUItem, held_item: Option<&compat::Shared<FlexibleItemStack>>) -> compat::Shared<FlexibleItemStack> {
        if false /* java: item == null: never null here */ {
            return self.get_error_stack();
        }
        // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
        //   java: FlexibleItemStack petStack = ItemRepository.getItemStack(item.getSkyblockItemId());
        let mut pet_stack: compat::Shared<FlexibleItemStack> = todo!("fabric2seer: the initialiser of `petStack` (TODO above)");
        if false /* java: pet_stack == null: never null here */ || FlexibleItemStack::is_empty(&pet_stack) {
            return self.get_error_stack();
        }
        pet_stack = FlexibleItemStack::copy(&pet_stack);
        // TODO(fabric2seer): `.getLore(..)` on a NEUItem
        //   java: List<Component> formattedLore = !(name.equals("GOLDEN_DRAGON") && level < 101) ?  processLore(item.getLore(), heldItem) : buildGoldenDragonEggLore(item.getLore());
        let mut formatted_lore: Vec<MutableComponent> = todo!("fabric2seer: the initialiser of `formattedLore` (TODO above)");
        let mut style: Style = mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_italic(Some(false));
        if self.level != 100 && self.level != 200 {
            let mut progress: String = format!("Progress to Level {}: §e{}%", self.level, self.fix_decimals((self.perecentage_to_level * (100 as f64)), true));
            formatted_lore.insert(((formatted_lore.len() as i32).wrapping_sub(1)) as usize, component::literal(&progress).set_style(style.clone()).with_style_format(ChatFormatting::Gray));
            let mut string: String = format!("{}{}", "§2§m ".repeat((((compat::java_round_f64((self.perecentage_to_level * (30 as f64)))) as i32)) as usize), "§f§m ".repeat((30i32.wrapping_sub((compat::java_round_f64((self.perecentage_to_level * (30 as f64)))) as i32)) as usize));
            formatted_lore.insert(((formatted_lore.len() as i32).wrapping_sub(1)) as usize, (component::literal(&format!("{}§r§e {}§6/§e{}", string, ProfileViewerUtils::num_letter_format((self.level_xp) as f64), ProfileViewerUtils::num_letter_format((self.next_level_xp) as f64)))).set_style(style.clone()));
            formatted_lore.insert(((formatted_lore.len() as i32).wrapping_sub(1)) as usize, component::empty());
        } else {
            formatted_lore.insert(((formatted_lore.len() as i32).wrapping_sub(1)) as usize, component::literal("MAX LEVEL").set_style(style.clone()).with_style_formats(&[ChatFormatting::Aqua, ChatFormatting::Bold]));
            formatted_lore.insert(((formatted_lore.len() as i32).wrapping_sub(1)) as usize, ((component::literal(&format!("▸ {} XP", formatters::INTEGER_NUMBERS.get_mut().format((((self.xp) as i64)) as f64)))).set_style(style.clone())).with_style_format(ChatFormatting::DarkGray));
            formatted_lore.insert(((formatted_lore.len() as i32).wrapping_sub(1)) as usize, component::empty());
        }
        if self.skin_texture.is_some() && self.skin.is_some() {
            let mut skin_item: Option<crate::unresolved::NEUItem> = NEURepoManager::get_item_by_neu_id(&format!("PET_SKIN_{}", self.skin.clone().unwrap()));
            if let Some(skin_item) = &skin_item {
                // TODO(fabric2seer): `.getDisplayName(..)` on a NEUItem
                //   java: formattedLore.set(0, Component.nullToEmpty(formattedLore.getFirst().getString() + ", " + ChatFormatting.stripFormatting(skinItem.getDisplayName())));
            }
            FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::profile().clone(), self.skin_texture.clone().unwrap());
        }
        if self.boosted() {
            formatted_lore[((formatted_lore.len() as i32).wrapping_sub(1)) as usize] = component::literal(&self.get_rarity().next().to_string()).set_style(style.clone()).with_style_format(ChatFormatting::Bold).with_color(self.get_rarity().next().fields().color);
        }
        FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::lore().clone(), ItemLore::new(formatted_lore.iter().cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect()).unwrap());
        // TODO(fabric2seer): `.getDisplayName(..)` on a NEUItem
        //   java: String displayName = ChatFormatting.stripFormatting(item.getDisplayName()).replace("[Lvl {LVL}]", "§7[Lvl " + this.level + "]§r");
        let mut display_name: String = todo!("fabric2seer: the initialiser of `displayName` (TODO above)");
        // (CUSTOM_NAME's value is a Component: the port's is a `FormattedText`)
        FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&display_name).set_style(style.clone()).with_color((if self.boosted() { self.get_rarity().next() } else { self.get_rarity() }).fields().color)));
        return pet_stack;
    }

    /// java: private List<Component> processLore(List<String> lore, FlexibleItemStack heldItem)
    pub fn process_lore(&self, lore: &Vec<String>, held_item: Option<&compat::Shared<FlexibleItemStack>>) -> Vec<MutableComponent> {
        // TODO(fabric2seer): `.getPetNumbers(..)` on a NEUConstants
        //   java: Map<String, Map<Rarity, PetNumbers>> petNums = NEURepoManager.getConstants().getPetNumbers();
        let mut pet_nums: std::collections::HashMap<String, std::collections::HashMap<crate::unresolved::Rarity, crate::unresolved::PetNumbers>> = todo!("fabric2seer: the initialiser of `petNums` (TODO above)");
        // TODO(fabric2seer): static call Rarity.values(..)
        //   java: Rarity rarity = Rarity.values()[getTier()];
        let mut rarity: crate::unresolved::Rarity = todo!("fabric2seer: the initialiser of `rarity` (TODO above)");
        // rust: port gap (GAPS_batchB L1, the NEU repo library) - `Rarity` / `PetNumbers` are the library's types, of which
        // the port has placeholders with no equality or hash: the map cannot be looked up by them
        // rust: let mut data: crate::unresolved::PetNumbers = pet_nums.get(&self.get_name()).cloned().unwrap().get(&rarity).cloned().unwrap();
        let _ = (&pet_nums, &rarity);
        let mut data: crate::unresolved::PetNumbers = crate::unresolved::PetNumbers;
        let mut formatted_lore: Vec<MutableComponent> = Vec::new();
        for line in lore.iter() {
            if line.contains("Right-click to add this") || line.contains("pet menu!") {
                continue;
            }
            let mut formatted_line: String = line.to_string();
            let mut stats: compat::regex::Matcher = STATS_MATCHER.matcher(&formatted_line);
            let mut other: compat::regex::Matcher = NUMBER_MATCHER.matcher(&formatted_line);
            while stats.find() {
                let mut placeholder: String = stats.group(0).unwrap();
                let mut stat_key: String = compat::substring2(&placeholder, 1, (compat::str_len(&placeholder)).wrapping_sub(1));
                // TODO(fabric2seer): `.interpolatedStatsAtLevel(..)` on a PetNumbers
                //   java: String statValue = String.valueOf(fixDecimals(data.interpolatedStatsAtLevel(this.level).getStatNumbers().get(statKey), true));
                let mut stat_value: String = todo!("fabric2seer: the initialiser of `statValue` (TODO above)");
                formatted_line = formatted_line.replace(&placeholder, &stat_value);
            }
            while other.find() {
                let mut placeholder: String = other.group(0).unwrap();
                let mut number_key: i32 = (compat::substring2(&placeholder, 1, (compat::str_len(&placeholder)).wrapping_sub(1))).parse::<i32>().expect("NumberFormatException");
                // TODO(fabric2seer): `.interpolatedStatsAtLevel(..)` on a PetNumbers
                //   java: String statValue = String.valueOf(fixDecimals(data.interpolatedStatsAtLevel(this.level).getOtherNumbers().get(numberKey), false));
                let mut stat_value: String = todo!("fabric2seer: the initialiser of `statValue` (TODO above)");
                formatted_line = formatted_line.replace(&placeholder, &stat_value);
            }
            formatted_lore.push(component::null_to_empty(Some(&formatted_line)));
        }
        if let Some(held_item) = held_item {
            // java: formattedLore.set(formattedLore.size() - 2, Component.nullToEmpty("§r§6Held Item: " + heldItem.get(DataComponents.CUSTOM_NAME).getString()));
            let held_name: mcrust::net::minecraft::network::chat::formatted_text::FormattedText = FlexibleItemStack::get(held_item, mcrust::net::minecraft::core::component::data_components::custom_name()).expect("NullPointerException");
            let __index = ((formatted_lore.len() as i32).wrapping_sub(2)) as usize;
            formatted_lore[__index] = component::null_to_empty(Some(&format!("§r§6Held Item: {}", compat::component_of(held_name).get_string())));
            formatted_lore.insert(((formatted_lore.len() as i32).wrapping_sub(1)) as usize, component::empty());
        }
        return formatted_lore;
    }

    /// java: private List<Component> buildGoldenDragonEggLore(List<String> lore)
    pub fn build_golden_dragon_egg_lore(&self, lore: &Vec<String>) -> Vec<MutableComponent> {
        let mut formatted_lore: Vec<MutableComponent> = Vec::new();
        let mut style: Style = mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_italic(Some(false));
        formatted_lore.push(component::null_to_empty(Some(&lore[0])));
        formatted_lore.push(component::empty());
        formatted_lore.push(component::literal("Perks:").set_style(style.clone()).with_style_format(ChatFormatting::Gray));
        formatted_lore.push((component::literal("???")).set_style(style.clone()).with_style_formats(&[ChatFormatting::Red, ChatFormatting::Bold]));
        formatted_lore.push(component::empty());
        formatted_lore.push((component::literal("Hatches at level §b100")).set_style(style.clone()).with_style_format(ChatFormatting::Gray));
        formatted_lore.push(component::empty());
        formatted_lore.push(component::null_to_empty(Some(&lore[lore.len() - 1])));
        return formatted_lore;
    }

    /// java: private String fixDecimals(double num, boolean truncate)
    pub fn fix_decimals(&self, mut num: f64, mut truncate: bool) -> String {
        if (num % (1 as f64)) == (0 as f64) {
            return ((num) as i32).to_string();
        }
        // TODO(fabric2seer): `new BigDecimal(..)`
        //   java: BigDecimal roundedNum = new BigDecimal(num).setScale(truncate ? 1 : 3, RoundingMode.HALF_UP);
        let mut rounded_num: crate::unresolved::BigDecimal = todo!("fabric2seer: the initialiser of `roundedNum` (TODO above)");
        // TODO(fabric2seer): `.stripTrailingZeros(..)` on a BigDecimal
        //   java: return roundedNum.stripTrailingZeros().toPlainString();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private boolean boosted()
    pub fn boosted(&self) -> bool {
        return (self.held_item.is_some() && self.held_item.clone().unwrap() == "PET_ITEM_TIER_BOOST");
    }

    /// java: private FlexibleItemStack getErrorStack()
    pub fn get_error_stack(&self) -> compat::Shared<FlexibleItemStack> {
        let mut err_icon: compat::Shared<FlexibleItemStack> = FlexibleItemStack::new2(items::items().get("barrier"));
        FlexibleItemStack::set(&err_icon, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), component::null_to_empty(Some(&self.get_name())));
        return err_icon;
    }

}
