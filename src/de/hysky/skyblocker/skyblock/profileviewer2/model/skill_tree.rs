//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.SkillTree by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_calculator::LevelCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::tree::skill_tree_builder;

/// java: public class SkillTree
#[derive(Clone)]
pub struct SkillTree {
    /// java: public Nodes nodes = new Nodes();
    pub nodes: Nodes,
    /// java: @SerializedName("tokens_spent") public TokensSpent tokensSpent = new TokensSpent();
    pub tokens_spent: TokensSpent,
    /// java: @SerializedName("selected_ability") public SelectedAbility selectedAbility = new SelectedAbility();
    pub selected_ability: SelectedAbility,
    /// java: public Experience experience = new Experience();
    pub experience: Experience,
    /// java: @SerializedName("selected_skill_tree_slot") public SelectedSkillTreeSlot selectedSkillTreeSlot = new SelectedSkillTreeSlot();
    pub selected_skill_tree_slot: SelectedSkillTreeSlot,
    /// java: @SerializedName("mining") public SkillTreeProperties mining1Properties = new SkillTreeProperties();
    pub mining1_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("mining_2") public SkillTreeProperties mining2Properties = new SkillTreeProperties();
    pub mining2_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("mining_3") public SkillTreeProperties mining3Properties = new SkillTreeProperties();
    pub mining3_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("mining_4") public SkillTreeProperties mining4Properties = new SkillTreeProperties();
    pub mining4_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("mining_5") public SkillTreeProperties mining5Properties = new SkillTreeProperties();
    pub mining5_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("foraging") public SkillTreeProperties foraging1Properties = new SkillTreeProperties();
    pub foraging1_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("foraging_2") public SkillTreeProperties foraging2Properties = new SkillTreeProperties();
    pub foraging2_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("foraging_3") public SkillTreeProperties foraging3Properties = new SkillTreeProperties();
    pub foraging3_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("foraging_4") public SkillTreeProperties foraging4Properties = new SkillTreeProperties();
    pub foraging4_properties: compat::Shared<SkillTreeProperties>,
    /// java: @SerializedName("foraging_5") public SkillTreeProperties foraging5Properties = new SkillTreeProperties();
    pub foraging5_properties: compat::Shared<SkillTreeProperties>,
}

impl SkillTree {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            nodes: Nodes::new(),
            tokens_spent: TokensSpent::new(),
            selected_ability: SelectedAbility::new(),
            experience: Experience::new(),
            selected_skill_tree_slot: SelectedSkillTreeSlot::new(),
            mining1_properties: SkillTreeProperties::new(),
            mining2_properties: SkillTreeProperties::new(),
            mining3_properties: SkillTreeProperties::new(),
            mining4_properties: SkillTreeProperties::new(),
            mining5_properties: SkillTreeProperties::new(),
            foraging1_properties: SkillTreeProperties::new(),
            foraging2_properties: SkillTreeProperties::new(),
            foraging3_properties: SkillTreeProperties::new(),
            foraging4_properties: SkillTreeProperties::new(),
            foraging5_properties: SkillTreeProperties::new(),
        };
        this
    }

    /// java: public boolean unlockedMiningTree(int slot)
    pub fn unlocked_mining_tree(&self, mut slot: i32) -> bool {
        return (slot <= skill_tree_builder::FREE_SLOTS || self.tokens_spent.get_mountain_tokens_spent(slot) > 0);
    }

    /// java: public boolean unlockedForagingTree(int slot)
    pub fn unlocked_foraging_tree(&self, mut slot: i32) -> bool {
        return (slot <= skill_tree_builder::FREE_SLOTS || self.tokens_spent.get_forest_tokens_spent(slot) > 0);
    }

    /// java: public SkillTreeProperties getMiningTreeProperties(int slot)
    pub fn get_mining_tree_properties(&self, mut slot: i32) -> compat::Shared<SkillTreeProperties> {
        return match slot {
            1 => self.mining1_properties.clone(),
            2 => self.mining2_properties.clone(),
            3 => self.mining3_properties.clone(),
            4 => self.mining4_properties.clone(),
            5 => self.mining5_properties.clone(),
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

    /// java: public SkillTreeProperties getForagingTreeProperties(int slot)
    pub fn get_foraging_tree_properties(&self, mut slot: i32) -> compat::Shared<SkillTreeProperties> {
        return match slot {
            1 => self.foraging1_properties.clone(),
            2 => self.foraging2_properties.clone(),
            3 => self.foraging3_properties.clone(),
            4 => self.foraging4_properties.clone(),
            5 => self.foraging5_properties.clone(),
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

}

/// java: public static class Nodes
#[derive(Clone)]
pub struct Nodes {
    /// java: @SerializedName("mining") public Map<String, Object> mining1 = Map.of();
    pub mining1: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("mining_2") public Map<String, Object> mining2 = Map.of();
    pub mining2: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("mining_3") public Map<String, Object> mining3 = Map.of();
    pub mining3: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("mining_4") public Map<String, Object> mining4 = Map.of();
    pub mining4: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("mining_5") public Map<String, Object> mining5 = Map.of();
    pub mining5: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("foraging") public Map<String, Object> foraging1 = Map.of();
    pub foraging1: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("foraging_2") public Map<String, Object> foraging2 = Map.of();
    pub foraging2: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("foraging_3") public Map<String, Object> foraging3 = Map.of();
    pub foraging3: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("foraging_4") public Map<String, Object> foraging4 = Map.of();
    pub foraging4: std::collections::HashMap<String, compat::Object>,
    /// java: @SerializedName("foraging_5") public Map<String, Object> foraging5 = Map.of();
    pub foraging5: std::collections::HashMap<String, compat::Object>,
}

impl Nodes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mining1: std::collections::HashMap::from([]),
            mining2: std::collections::HashMap::from([]),
            mining3: std::collections::HashMap::from([]),
            mining4: std::collections::HashMap::from([]),
            mining5: std::collections::HashMap::from([]),
            foraging1: std::collections::HashMap::from([]),
            foraging2: std::collections::HashMap::from([]),
            foraging3: std::collections::HashMap::from([]),
            foraging4: std::collections::HashMap::from([]),
            foraging5: std::collections::HashMap::from([]),
        };
        this
    }

    /// java: public Map<String, Object> getMiningNode(int slot)
    pub fn get_mining_node(&self, mut slot: i32) -> std::collections::HashMap<String, compat::Object> {
        return match slot {
            1 => self.mining1.clone(),
            2 => self.mining2.clone(),
            3 => self.mining3.clone(),
            4 => self.mining4.clone(),
            5 => self.mining5.clone(),
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

    /// java: public Map<String, Object> getForagingNode(int slot)
    pub fn get_foraging_node(&self, mut slot: i32) -> std::collections::HashMap<String, compat::Object> {
        return match slot {
            1 => self.foraging1.clone(),
            2 => self.foraging2.clone(),
            3 => self.foraging3.clone(),
            4 => self.foraging4.clone(),
            5 => self.foraging5.clone(),
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

}

/// java: public static class TokensSpent
#[derive(Clone)]
pub struct TokensSpent {
    /// java: @SerializedName("mountain") public int mountain1;
    pub mountain1: i32,
    /// java: @SerializedName("mountain_2") public int mountain2;
    pub mountain2: i32,
    /// java: @SerializedName("mountain_3") public int mountain3;
    pub mountain3: i32,
    /// java: @SerializedName("mountain_4") public int mountain4;
    pub mountain4: i32,
    /// java: @SerializedName("mountain_5") public int mountain5;
    pub mountain5: i32,
    /// java: @SerializedName("forest") public int forest1;
    pub forest1: i32,
    /// java: @SerializedName("forest_2") public int forest2;
    pub forest2: i32,
    /// java: @SerializedName("forest_3") public int forest3;
    pub forest3: i32,
    /// java: @SerializedName("forest_4") public int forest4;
    pub forest4: i32,
    /// java: @SerializedName("forest_5") public int forest5;
    pub forest5: i32,
}

impl TokensSpent {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mountain1: 0,
            mountain2: 0,
            mountain3: 0,
            mountain4: 0,
            mountain5: 0,
            forest1: 0,
            forest2: 0,
            forest3: 0,
            forest4: 0,
            forest5: 0,
        };
        this
    }

    /// java: public int getMountainTokensSpent(int slot)
    pub fn get_mountain_tokens_spent(&self, mut slot: i32) -> i32 {
        return match slot {
            1 => self.mountain1,
            2 => self.mountain2,
            3 => self.mountain3,
            4 => self.mountain4,
            5 => self.mountain5,
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

    /// java: public int getForestTokensSpent(int slot)
    pub fn get_forest_tokens_spent(&self, mut slot: i32) -> i32 {
        return match slot {
            1 => self.forest1,
            2 => self.forest2,
            3 => self.forest3,
            4 => self.forest4,
            5 => self.forest5,
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

}

/// java: public static class SelectedAbility
#[derive(Clone)]
pub struct SelectedAbility {
    /// java: @SerializedName("mining") public String mining1 = "";
    pub mining1: String,
    /// java: @SerializedName("mining_2") public String mining2 = "";
    pub mining2: String,
    /// java: @SerializedName("mining_3") public String mining3 = "";
    pub mining3: String,
    /// java: @SerializedName("mining_4") public String mining4 = "";
    pub mining4: String,
    /// java: @SerializedName("mining_5") public String mining5 = "";
    pub mining5: String,
    /// java: @SerializedName("foraging") public String foraging1 = "";
    pub foraging1: String,
    /// java: @SerializedName("foraging_2") public String foraging2 = "";
    pub foraging2: String,
    /// java: @SerializedName("foraging_3") public String foraging3 = "";
    pub foraging3: String,
    /// java: @SerializedName("foraging_4") public String foraging4 = "";
    pub foraging4: String,
    /// java: @SerializedName("foraging_5") public String foraging5 = "";
    pub foraging5: String,
}

impl SelectedAbility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mining1: "".to_string(),
            mining2: "".to_string(),
            mining3: "".to_string(),
            mining4: "".to_string(),
            mining5: "".to_string(),
            foraging1: "".to_string(),
            foraging2: "".to_string(),
            foraging3: "".to_string(),
            foraging4: "".to_string(),
            foraging5: "".to_string(),
        };
        this
    }

    /// java: public String getSelectedMiningAbility(int slot)
    pub fn get_selected_mining_ability(&self, mut slot: i32) -> String {
        return match slot {
            1 => self.mining1.clone(),
            2 => self.mining2.clone(),
            3 => self.mining3.clone(),
            4 => self.mining4.clone(),
            5 => self.mining5.clone(),
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

    /// java: public String getSelectedForagingAbility(int slot)
    pub fn get_selected_foraging_ability(&self, mut slot: i32) -> String {
        return match slot {
            1 => self.foraging1.clone(),
            2 => self.foraging2.clone(),
            3 => self.foraging3.clone(),
            4 => self.foraging4.clone(),
            5 => self.foraging5.clone(),
            _ => {
                panic!("IllegalArgumentException: {}", "Slot must be between 1-5.");
            }
        };
    }

}

/// java: public static class Experience
#[derive(Clone)]
pub struct Experience {
    /// java: public double mining;
    pub mining: f64,
    /// java: public double foraging;
    pub foraging: f64,
}

impl Experience {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mining: 0.0,
            foraging: 0.0,
        };
        this
    }

    /// java: public LevelInfo getHotmLevel()
    pub fn get_hotm_level(&self) -> LevelInfo {
        return LevelCalculator::get_hotm_level(((self.mining) as i64));
    }

    /// java: public LevelInfo getHotfLevel()
    pub fn get_hotf_level(&self) -> LevelInfo {
        return LevelCalculator::get_hotf_level(((self.foraging) as i64));
    }

}

/// java: public static class SelectedSkillTreeSlot
#[derive(Clone)]
pub struct SelectedSkillTreeSlot {
    /// java: public int mining = 1;
    pub mining: i32,
    /// java: public int foraging = 1;
    pub foraging: i32,
}

impl SelectedSkillTreeSlot {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mining: 1,
            foraging: 1,
        };
        this
    }

}

/// java: public static class SkillTreeProperties
#[derive(Clone)]
pub struct SkillTreeProperties {
    /// java: @SerializedName("custom_name") public @Nullable String customName;
    pub custom_name: Option<String>,
}

impl SkillTreeProperties {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            custom_name: None,
        });
        __this
    }

}
