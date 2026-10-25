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
        let mut item: Option<Arc<crate::io::github::moulberry::repo::data::neu_item::NEUItem>> = NEURepoManager::get_item_by_neu_id(&target_item_id);
        if item.is_none() && self.held_item.is_some() && self.held_item.clone().unwrap() == "PET_ITEM_TIER_BOOST" {
            item = NEURepoManager::get_item_by_neu_id(&format!("{};{}", self.get_name(), self.get_tier()));
        }
        // java: fromNEUItem(item, this.heldItem.map(ItemRepository::getItemStack).orElse(null)) - null without a held item
        return (if item.is_none() { ico::BARRIER.get_mut().clone() } else { self.from_neu_item(item.as_ref().unwrap(), self.held_item.clone().and_then(|__a0| ItemRepository::get_item_stack(&__a0)).as_ref()) });
    }

    /// java: private FlexibleItemStack fromNEUItem(NEUItem item, FlexibleItemStack heldItem)
    pub fn from_neu_item(&self, item: &crate::io::github::moulberry::repo::data::neu_item::NEUItem, held_item: Option<&compat::Shared<FlexibleItemStack>>) -> compat::Shared<FlexibleItemStack> {
        if false /* java: item == null: never null here */ {
            return self.get_error_stack();
        }
        //   java: FlexibleItemStack petStack = ItemRepository.getItemStack(item.getSkyblockItemId());
        //   java: if (petStack == null || petStack.isEmpty()) return getErrorStack();
        let Some(mut pet_stack) = ItemRepository::get_item_stack(item.get_skyblock_item_id()) else {
            return self.get_error_stack();
        };
        if FlexibleItemStack::is_empty(&pet_stack) {
            return self.get_error_stack();
        }
        pet_stack = FlexibleItemStack::copy(&pet_stack);
        //   java: List<Component> formattedLore = !(name.equals("GOLDEN_DRAGON") && level < 101) ?  processLore(item.getLore(), heldItem) : buildGoldenDragonEggLore(item.getLore());
        let item_lore: Vec<String> = item.get_lore().to_vec();
        let mut formatted_lore: Vec<MutableComponent> = if !(self.get_name() == "GOLDEN_DRAGON" && self.level < 101) {
            self.process_lore(&item_lore, held_item)
        } else {
            self.build_golden_dragon_egg_lore(&item_lore)
        };
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
            let mut skin_item: Option<Arc<crate::io::github::moulberry::repo::data::neu_item::NEUItem>> = NEURepoManager::get_item_by_neu_id(&format!("PET_SKIN_{}", self.skin.clone().unwrap()));
            if let Some(skin_item) = &skin_item {
                //   java: formattedLore.set(0, Component.nullToEmpty(formattedLore.getFirst().getString() + ", " + ChatFormatting.stripFormatting(skinItem.getDisplayName())));
                let __v = component::null_to_empty(Some(&format!("{}, {}", formatted_lore[0].get_string(), mcrust::net::minecraft::chat_formatting::strip_formatting(skin_item.get_display_name()))));
                formatted_lore[0] = __v;
            }
            FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::profile().clone(), self.skin_texture.clone().unwrap());
        }
        if self.boosted() {
            let __last = ((formatted_lore.len() as i32).wrapping_sub(1)) as usize;
            formatted_lore[__last] = component::literal(&self.get_rarity().next().to_string()).set_style(style.clone()).with_style_format(ChatFormatting::Bold).with_color(self.get_rarity().next().fields().color);
        }
        FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::lore().clone(), ItemLore::new(formatted_lore.iter().cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect()).unwrap());
        //   java: String displayName = ChatFormatting.stripFormatting(item.getDisplayName()).replace("[Lvl {LVL}]", "§7[Lvl " + this.level + "]§r");
        let mut display_name: String = mcrust::net::minecraft::chat_formatting::strip_formatting(item.get_display_name()).replace("[Lvl {LVL}]", &format!("§7[Lvl {}]§r", self.level));
        // (CUSTOM_NAME's value is a Component: the port's is a `FormattedText`)
        FlexibleItemStack::set(&pet_stack, mcrust::net::minecraft::core::component::data_components::custom_name().clone(), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&display_name).set_style(style.clone()).with_color((if self.boosted() { self.get_rarity().next() } else { self.get_rarity() }).fields().color)));
        return pet_stack;
    }

    /// java: private List<Component> processLore(List<String> lore, FlexibleItemStack heldItem)
    pub fn process_lore(&self, lore: &Vec<String>, held_item: Option<&compat::Shared<FlexibleItemStack>>) -> Vec<MutableComponent> {
        //   java: Map<String, Map<Rarity, PetNumbers>> petNums = NEURepoManager.getConstants().getPetNumbers();
        let pet_nums: Arc<crate::io::github::moulberry::repo::neu_constants::PetNumbersMap> = NEURepoManager::get_constants().get_pet_numbers();
        //   java: Rarity rarity = Rarity.values()[getTier()]; - an index past the array throws
        let mut rarity: crate::io::github::moulberry::repo::data::rarity::Rarity = crate::io::github::moulberry::repo::data::rarity::Rarity::values()[self.get_tier() as usize];
        //   java: PetNumbers data = petNums.get(getName()).get(rarity); - a missing entry is a null `data` and the NPE below
        let mut data: crate::io::github::moulberry::repo::constants::pet_numbers::PetNumbers = pet_nums
            .get(&self.get_name())
            .and_then(|by_rarity| by_rarity.get(&rarity))
            .cloned()
            .expect("java.lang.NullPointerException: PetNumbers data");
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
                //   java: String statValue = String.valueOf(fixDecimals(data.interpolatedStatsAtLevel(this.level).getStatNumbers().get(statKey), true));
                let __stat: f64 = *data
                    .interpolated_stats_at_level(self.level)
                    .expect("java.lang.NullPointerException: interpolatedStatsAtLevel")
                    .get_stat_numbers()
                    .get(&stat_key)
                    .expect("java.lang.NullPointerException: stat number");
                let mut stat_value: String = self.fix_decimals(__stat, true);
                formatted_line = formatted_line.replace(&placeholder, &stat_value);
            }
            while other.find() {
                let mut placeholder: String = other.group(0).unwrap();
                let mut number_key: i32 = (compat::substring2(&placeholder, 1, (compat::str_len(&placeholder)).wrapping_sub(1))).parse::<i32>().expect("NumberFormatException");
                //   java: String statValue = String.valueOf(fixDecimals(data.interpolatedStatsAtLevel(this.level).getOtherNumbers().get(numberKey), false));
                let __number: f64 = *data
                    .interpolated_stats_at_level(self.level)
                    .expect("java.lang.NullPointerException: interpolatedStatsAtLevel")
                    .get_other_numbers()
                    .get(number_key as usize)
                    .expect("java.lang.IndexOutOfBoundsException: other number");
                let mut stat_value: String = self.fix_decimals(__number, false);
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
        // java: BigDecimal roundedNum = new BigDecimal(num).setScale(truncate ? 1 : 3, RoundingMode.HALF_UP);
        // java: return roundedNum.stripTrailingZeros().toPlainString();
        // `new BigDecimal(double)` is the double's EXACT binary value, rounded HALF_UP in decimal (see `big_decimal_*`).
        let (negative, digits, scale) = big_decimal_exact(num);
        let (digits, scale) = big_decimal_set_scale_half_up(digits, scale, if truncate { 1 } else { 3 });
        let (digits, scale) = big_decimal_strip_trailing_zeros(digits, scale);
        return big_decimal_to_plain_string(negative, &digits, scale);
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

/// java: `new BigDecimal(double val)` - the exact decimal expansion of the double's binary value, as (sign, unscaled
/// magnitude digits, scale): `val == (-1)^sign * digits * 10^-scale`. Java throws `NumberFormatException` for an infinite
/// or NaN argument.
fn big_decimal_exact(val: f64) -> (bool, Vec<u8>, i32) {
    if !val.is_finite() {
        panic!("NumberFormatException: Infinite or NaN");
    }
    let bits = val.to_bits();
    let negative = (bits >> 63) != 0;
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1u64 << 52) - 1);
    // value = mantissa * 2^exp
    let (mantissa, exp) = if exponent == 0 { (fraction, -1074) } else { (fraction | (1u64 << 52), exponent - 1075) };
    if mantissa == 0 {
        return (negative, vec![0], 0);
    }
    // little-endian base-10^9 limbs
    let mut limbs: Vec<u64> = vec![mantissa % 1_000_000_000, (mantissa / 1_000_000_000) % 1_000_000_000, mantissa / 1_000_000_000_000_000_000];
    let multiply = |limbs: &mut Vec<u64>, by: u64| {
        let mut carry = 0u64;
        for limb in limbs.iter_mut() {
            let v = *limb * by + carry;
            *limb = v % 1_000_000_000;
            carry = v / 1_000_000_000;
        }
        while carry > 0 {
            limbs.push(carry % 1_000_000_000);
            carry /= 1_000_000_000;
        }
    };
    let scale = if exp >= 0 {
        // an integer: mantissa * 2^exp
        for _ in 0..exp {
            multiply(&mut limbs, 2);
        }
        0
    } else {
        // mantissa * 2^-k = mantissa * 5^k / 10^k
        for _ in 0..(-exp) {
            multiply(&mut limbs, 5);
        }
        -exp
    };
    while limbs.len() > 1 && *limbs.last().unwrap() == 0 {
        limbs.pop();
    }
    let mut text = limbs.last().unwrap().to_string();
    for limb in limbs.iter().rev().skip(1) {
        text.push_str(&format!("{limb:09}"));
    }
    (negative, text.bytes().map(|c| c - b'0').collect(), scale)
}

/// java: `BigDecimal.setScale(newScale, RoundingMode.HALF_UP)` on (magnitude digits, scale): round half away from zero.
fn big_decimal_set_scale_half_up(mut digits: Vec<u8>, scale: i32, new_scale: i32) -> (Vec<u8>, i32) {
    if new_scale >= scale {
        digits.extend(std::iter::repeat(0u8).take((new_scale - scale) as usize));
        return (digits, new_scale);
    }
    let drop = (scale - new_scale) as usize;
    let (keep, dropped): (Vec<u8>, u8) = if drop > digits.len() {
        (vec![0], 0)
    } else if drop == digits.len() {
        (vec![0], digits[0])
    } else {
        (digits[..digits.len() - drop].to_vec(), digits[digits.len() - drop])
    };
    let mut keep = keep;
    if dropped >= 5 {
        // add one to the kept magnitude
        let mut i = keep.len();
        loop {
            if i == 0 {
                keep.insert(0, 1);
                break;
            }
            i -= 1;
            if keep[i] == 9 {
                keep[i] = 0;
            } else {
                keep[i] += 1;
                break;
            }
        }
    }
    (keep, new_scale)
}

/// java: `BigDecimal.stripTrailingZeros()` - a zero value is `BigDecimal.ZERO` (scale 0).
fn big_decimal_strip_trailing_zeros(mut digits: Vec<u8>, mut scale: i32) -> (Vec<u8>, i32) {
    while digits.len() > 1 && digits[0] == 0 {
        digits.remove(0);
    }
    if digits == [0] {
        return (digits, 0);
    }
    while digits.len() > 1 && *digits.last().unwrap() == 0 {
        digits.pop();
        scale -= 1;
    }
    (digits, scale)
}

/// java: `BigDecimal.toPlainString()` (no exponent; a zero magnitude has no sign).
fn big_decimal_to_plain_string(negative: bool, digits: &[u8], scale: i32) -> String {
    let unscaled: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    let is_zero = digits.iter().all(|d| *d == 0);
    let mut out = String::new();
    if negative && !is_zero {
        out.push('-');
    }
    if scale <= 0 {
        out.push_str(&unscaled);
        if !is_zero {
            out.extend(std::iter::repeat('0').take((-scale) as usize));
        }
    } else {
        let scale = scale as usize;
        if unscaled.len() > scale {
            out.push_str(&unscaled[..unscaled.len() - scale]);
            out.push('.');
            out.push_str(&unscaled[unscaled.len() - scale..]);
        } else {
            out.push_str("0.");
            out.extend(std::iter::repeat('0').take(scale - unscaled.len()));
            out.push_str(&unscaled);
        }
    }
    out
}
