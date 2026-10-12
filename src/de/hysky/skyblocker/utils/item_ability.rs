//! Ported from de.hysky.skyblocker.utils.ItemAbility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): expression not parsed
//   java: private static final Pattern ABILITY_NAME_PATTERN = Pattern.compile("(?:⦾ )?Ability: (.+)" + " {2}(" + String.join("|", Arrays.stream(Activation.values()).map(Activation::toString).toArray(String[]::new)) + ")");
//   java: 	private static final Pattern MANA_COST_PATTERN = Pattern.compile("Mana Cost: (\\d+)");
//   java: 	private static final Pattern SOULFLOW_COST_PATTERN = Pattern.compile("Soulflow Cost: (\\d+)");
//   java: 	private static final Pattern COOLDOWN_PATTERN = Pattern.compile("Cooldown: ([0-9]+\\.?[0-9]*)s");
//   java:
//   java: 	/**
//   java: 	 * Use {@link ItemStack#skyblocker$getAbilities()}
//   java: 	 */
//   java: 	@Deprecated
//   java: 	public static <T extends ItemInstance & SkyblockerStack> List<ItemAbility> getAbilities(T stack) {
//   java: 		List<String> strings = stack.skyblocker$getLoreStrings();
/// java: private static final Pattern ABILITY_NAME_PATTERN = Pattern.compile("(?:⦾ )?Ability: (.+)" + " {2}(" + String.join("|", Arrays.stream(Activation.values()).map(Activation::toString).toArray(String[]::n
pub static ABILITY_NAME_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `ABILITY_NAME_PATTERN` - see the TODO above"));

/// java: public record ItemAbility(String name, Activation activation, OptionalInt manaCost, OptionalInt soulflowCost, OptionalInt cooldown)
#[derive(Clone, Debug)]
pub struct ItemAbility {
    /// java: List<ItemAbility> abilities = new ArrayList<>(2);
    pub abilities: Vec<ItemAbility>,
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
        // TODO(fabric2seer): `name(..)` is a method of the enclosing instance (ItemAbility.this) - an inner class has no outer reference in Rust
        //   java: return name().replace('_', ' ');
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static Activation of(String name)
    pub fn of(name: &str) -> Activation {
        for value in Activation::values() {
            if value.to_string() == name {
                return value;
            }
        }
        return RIGHT_CLICK;
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
        write!(f, "{}", Activation::to_string(*self))
    }
}
