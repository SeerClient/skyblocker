//! Ported from de.hysky.skyblocker.utils.ItemAbility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack;

/// java: private static final Pattern ABILITY_NAME_PATTERN = Pattern.compile("(?:⦾ )?Ability: (.+)" + " {2}(" + String.join("|", Arrays.stream(Activation.values()).map(Activation::toString).toArray(String[]::new)) + ")");
pub static ABILITY_NAME_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| {
    compat::regex::Pattern::compile(&format!(
        "(?:⦾ )?Ability: (.+){}({})",
        " {2}",
        Activation::values().iter().map(|activation| activation.to_string()).collect::<Vec<String>>().join("|")
    ))
});
/// java: private static final Pattern MANA_COST_PATTERN = Pattern.compile("Mana Cost: (\\d+)");
pub static MANA_COST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Mana Cost: (\\d+)"));
/// java: private static final Pattern SOULFLOW_COST_PATTERN = Pattern.compile("Soulflow Cost: (\\d+)");
pub static SOULFLOW_COST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Soulflow Cost: (\\d+)"));
/// java: private static final Pattern COOLDOWN_PATTERN = Pattern.compile("Cooldown: ([0-9]+\\.?[0-9]*)s");
pub static COOLDOWN_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Cooldown: ([0-9]+\\.?[0-9]*)s"));

/// java: public record ItemAbility(String name, Activation activation, OptionalInt manaCost, OptionalInt soulflowCost, OptionalInt cooldown)
#[derive(Clone, Debug, PartialEq)]
pub struct ItemAbility {
    /// java: String name
    pub name: String,
    /// java: OptionalInt manaCost
    pub mana_cost: Option<i32>,
    /// java: OptionalInt soulflowCost
    pub soulflow_cost: Option<i32>,
    /// java: OptionalInt cooldown
    pub cooldown: Option<i32>,
    /// java: Activation activation
    pub activation: Activation,
}

impl ItemAbility {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut activation: Activation, mut mana_cost: Option<i32>, mut soulflow_cost: Option<i32>, mut cooldown: Option<i32>) -> Self {
        Self { name, activation, mana_cost, soulflow_cost, cooldown }
    }

    /// java: @Deprecated public static <T extends ItemInstance & SkyblockerStack> List<ItemAbility> getAbilities(T stack)
    pub fn get_abilities<T: SkyblockerStack + ?Sized>(stack: &T) -> Vec<ItemAbility> {
        let strings: Vec<String> = stack.skyblocker_get_lore_strings();
        let mut abilities: Vec<ItemAbility> = Vec::with_capacity(2); // items rarely have more than 2
        let mut name: Option<String> = None;
        let mut mana_cost: i32 = -1;
        let mut soulflow_cost: i32 = -1;
        let mut cooldown: i32 = -1;
        let mut activation: Option<Activation> = None;
        for string in strings.iter() {
            let string: String = compat::trim(&mcrust::net::minecraft::chat_formatting::strip_formatting(string));
            let mut matcher: compat::regex::Matcher = ABILITY_NAME_PATTERN.matcher(&string);
            if matcher.matches() {
                // add previous ability to list
                if let Some(name) = &name {
                    abilities.push(ItemAbility::new(name.clone(), activation.expect("the ability's activation"), ItemAbility::positive_only(mana_cost), ItemAbility::positive_only(soulflow_cost), ItemAbility::positive_only(cooldown)));
                }
                // reset values
                name = matcher.group(1);
                mana_cost = -1;
                soulflow_cost = -1;
                cooldown = -1;
                activation = Some(Activation::of(&matcher.group(2).unwrap_or_default()));
            }
            if name.is_none() {
                continue;
            }
            // Mana
            if mana_cost < 0 {
                matcher = MANA_COST_PATTERN.matcher(&string);
                if matcher.matches() {
                    // java: NumberUtils.toInt(matcher.group(1), -1)
                    mana_cost = matcher.group(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(-1);
                    continue;
                }
            }
            // Soulflow
            if soulflow_cost < 0 {
                matcher = SOULFLOW_COST_PATTERN.matcher(&string);
                if matcher.matches() {
                    soulflow_cost = matcher.group(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(-1);
                    continue;
                }
            }
            // Cooldown
            if cooldown < 0 {
                matcher = COOLDOWN_PATTERN.matcher(&string);
                if matcher.matches() {
                    // java: (int) (NumberUtils.toFloat(matcher.group(1), -1) * 20) - multiply by 20 to convert to ticks.
                    cooldown = (matcher.group(1).and_then(|s| s.parse::<f32>().ok()).unwrap_or(-1.0) * 20.0) as i32;
                    continue;
                }
            }
        }
        if let Some(name) = &name {
            abilities.push(ItemAbility::new(name.clone(), activation.expect("the ability's activation"), ItemAbility::positive_only(mana_cost), ItemAbility::positive_only(soulflow_cost), ItemAbility::positive_only(cooldown)));
        }
        return abilities;
    }

    /// java: public static boolean hasAbility(ItemStack stack, String ability)
    pub fn has_ability(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, ability: &str) -> bool {
        let abilities: Vec<ItemAbility> = stack.skyblocker_get_abilities();
        for item_ability in abilities.iter() {
            if item_ability.name() == ability {
                return true;
            }
        }
        return false;
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `activation()`
    pub fn activation(&self) -> Activation {
        self.activation
    }

    /// java: the record accessor `manaCost()`
    pub fn mana_cost(&self) -> Option<i32> {
        self.mana_cost.clone()
    }

    /// java: the record accessor `soulflowCost()`
    pub fn soulflow_cost(&self) -> Option<i32> {
        self.soulflow_cost.clone()
    }

    /// java: the record accessor `cooldown()`
    pub fn cooldown(&self) -> Option<i32> {
        self.cooldown.clone()
    }

    /// java: private static OptionalInt positiveOnly(int value)
    pub fn positive_only(mut value: i32) -> Option<i32> {
        return (if value < 0 { None } else { Some(value) });
    }

}

/// java: public enum Activation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Activation {
    RightClick,
    LeftClick,
    SneakRightClick,
    SneakLeftClick,
}

impl Activation {
    pub const VALUES: [Activation; 4] = [Activation::RightClick, Activation::LeftClick, Activation::SneakRightClick, Activation::SneakLeftClick];

    /// java: values()
    pub fn values() -> Vec<Activation> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Activation::RightClick => "RIGHT_CLICK",
            Activation::LeftClick => "LEFT_CLICK",
            Activation::SneakRightClick => "SNEAK_RIGHT_CLICK",
            Activation::SneakLeftClick => "SNEAK_LEFT_CLICK",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Activation {
        match name {
            "RIGHT_CLICK" => Activation::RightClick,
            "LEFT_CLICK" => Activation::LeftClick,
            "SNEAK_RIGHT_CLICK" => Activation::SneakRightClick,
            "SNEAK_LEFT_CLICK" => Activation::SneakLeftClick,
            _ => panic!("IllegalArgumentException: No enum constant Activation.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // java: return name().replace('_', ' '); - the enum constant's own `name()`
        return self.name().replace('_', " ");
    }

    /// java: public static Activation of(String name)
    pub fn of(name: &str) -> Activation {
        for value in Activation::values() {
            if value.to_string() == name {
                return value;
            }
        }
        return Activation::RightClick;
    }

    /// java: public boolean isRightClick()
    pub fn is_right_click(&self) -> bool {
        return (*self == Activation::SneakRightClick || *self == Activation::RightClick);
    }

    /// java: public boolean isLeftClick()
    pub fn is_left_click(&self) -> bool {
        return (*self == Activation::SneakLeftClick || *self == Activation::LeftClick);
    }

}

impl std::fmt::Display for Activation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Activation::to_string(self))
    }
}
