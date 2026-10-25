//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.ProfileMember by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::accessory_bag_storage::AccessoryBagStorage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::attributes::Attributes;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::bestiary::Bestiary;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::currencies::Currencies;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::dungeons::Dungeons;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::events::Events;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::experimentation::Experimentation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::fairy_souls::FairySouls;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::foraging::Foraging;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::foraging_core::ForagingCore;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::forge::Forge;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::garden_player_data::GardenPlayerData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::glacite_player_data::GlacitePlayerData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::inventories::Inventories;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::item_data::ItemData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::jacobs_contest::JacobsContest;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::levelling::Levelling;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::loadouts::Loadouts;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::mining_core::MiningCore;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::nether_island_player_data::NetherIslandPlayerData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::pets_data::PetsData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::player_data::PlayerData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::player_stats::PlayerStats;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member_profile::ProfileMemberProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::rift::Rift;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::safari::Safari;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::shards::Shards;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::skill_tree::SkillTree;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::slayer_data::SlayerData;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::temples::Temples;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::winter_player_data::WinterPlayerData;
use mcrust::java::util::uuid::Uuid;

/// java: public class ProfileMember
#[derive(Clone)]
pub struct ProfileMember {
    /// java: @SerializedName("player_id") public UUID playerId;
    pub player_id: Option<Uuid>,
    /// java: @SerializedName("accessory_bag_storage") public AccessoryBagStorage accessoryBagStorage = new AccessoryBagStorage();
    pub accessory_bag_storage: AccessoryBagStorage,
    /// java: public Attributes attributes = new Attributes();
    pub attributes: Attributes,
    /// java: public Bestiary bestiary = new Bestiary();
    pub bestiary: Bestiary,
    /// java: public Map<String, Long> collection = Map.of();
    pub collection: std::collections::HashMap<String, i64>,
    /// java: public Events events = new Events();
    pub events: Events,
    /// java: @SerializedName("fairy_soul") public FairySouls fairySouls = new FairySouls();
    pub fairy_souls: FairySouls,
    /// java: public Currencies currencies = new Currencies();
    pub currencies: Currencies,
    /// java: public Dungeons dungeons = new Dungeons();
    pub dungeons: Dungeons,
    /// java: public Experimentation experimentation = new Experimentation();
    pub experimentation: Experimentation,
    /// java: public Foraging foraging = new Foraging();
    pub foraging: Foraging,
    /// java: @SerializedName("foraging_core") public ForagingCore foragingCore = new ForagingCore();
    pub foraging_core: ForagingCore,
    /// java: public Forge forge = new Forge();
    pub forge: Forge,
    /// java: @SerializedName("garden_player_data") public GardenPlayerData gardenPlayerData = new GardenPlayerData();
    pub garden_player_data: GardenPlayerData,
    /// java: @SerializedName("glacite_player_data") public GlacitePlayerData glacitePlayerData = new GlacitePlayerData();
    pub glacite_player_data: GlacitePlayerData,
    /// java: @SerializedName("inventory") public Inventories inventories = new Inventories();
    pub inventories: Inventories,
    /// java: @SerializedName("item_data") public ItemData itemData = new ItemData();
    pub item_data: ItemData,
    /// java: @SerializedName("jacobs_contest") public JacobsContest jacobsContest = new JacobsContest();
    pub jacobs_contest: JacobsContest,
    /// java: @SerializedName("leveling") public Levelling levelling = new Levelling();
    pub levelling: Levelling,
    /// java: @SerializedName("loadout") public Loadouts loadouts = new Loadouts();
    pub loadouts: Loadouts,
    /// java: @SerializedName("mining_core") public MiningCore miningCore = new MiningCore();
    pub mining_core: MiningCore,
    /// java: @SerializedName("nether_island_player_data") public NetherIslandPlayerData netherIslandPlayerData = new NetherIslandPlayerData();
    pub nether_island_player_data: NetherIslandPlayerData,
    /// java: @SerializedName("pets_data") public PetsData petsData = new PetsData();
    pub pets_data: PetsData,
    /// java: @SerializedName("player_data") public PlayerData playerData = new PlayerData();
    pub player_data: PlayerData,
    /// java: @SerializedName("player_stats") public PlayerStats playerStats = new PlayerStats();
    pub player_stats: PlayerStats,
    /// java: public ProfileMemberProfile profile = new ProfileMemberProfile();
    pub profile: ProfileMemberProfile,
    /// java: public Rift rift = new Rift();
    pub rift: Rift,
    /// java: public Safari safari = new Safari();
    pub safari: Safari,
    /// java: public Shards shards = new Shards();
    pub shards: Shards,
    /// java: @SerializedName("skill_tree") public SkillTree skillTree = new SkillTree();
    pub skill_tree: SkillTree,
    /// java: public SlayerData slayer = new SlayerData();
    pub slayer: SlayerData,
    /// java: public Temples temples = new Temples();
    pub temples: Temples,
    /// java: @SerializedName("trophy_fish") public JsonObject trophyFish = new JsonObject();
    pub trophy_fish: serde_json::Map<String, serde_json::Value>,
    /// java: @SerializedName("winter_player_data") public WinterPlayerData winterPlayerData = new WinterPlayerData();
    pub winter_player_data: WinterPlayerData,
}

impl ProfileMember {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            player_id: None,
            accessory_bag_storage: AccessoryBagStorage::new(),
            attributes: Attributes::new(),
            bestiary: Bestiary::new(),
            collection: std::collections::HashMap::from([]),
            events: Events::new(),
            fairy_souls: FairySouls::new(),
            currencies: Currencies::new(),
            dungeons: Dungeons::new(),
            experimentation: Experimentation::new(),
            foraging: Foraging::new(),
            foraging_core: ForagingCore::new(),
            forge: Forge::new(),
            garden_player_data: GardenPlayerData::new(),
            glacite_player_data: GlacitePlayerData::new(),
            inventories: Inventories::new(),
            item_data: ItemData::new(),
            jacobs_contest: JacobsContest::new(),
            levelling: Levelling::new(),
            loadouts: Loadouts::new(),
            mining_core: MiningCore::new(),
            nether_island_player_data: NetherIslandPlayerData::new(),
            pets_data: PetsData::new(),
            player_data: PlayerData::new(),
            player_stats: PlayerStats::new(),
            profile: ProfileMemberProfile::new(),
            rift: Rift::new(),
            safari: Safari::new(),
            shards: Shards::new(),
            skill_tree: SkillTree::new(),
            slayer: SlayerData::new(),
            temples: Temples::new(),
            trophy_fish: serde_json::Map::new(),
            winter_player_data: WinterPlayerData::new(),
        });
        __this
    }

}

impl ProfileMember {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "player_id") {
            if let Some(__x) = (crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v)).map(Some) {
                self.player_id = __x;
            }
        }
        if let Some(__v) = gson::member(json, "accessory_bag_storage") {
            if let Some(__x) = Some(AccessoryBagStorage::from_json(__v)) {
                self.accessory_bag_storage = __x;
            }
        }
        if let Some(__v) = gson::member(json, "attributes") {
            if let Some(__x) = Some(Attributes::from_json(__v)) {
                self.attributes = __x;
            }
        }
        if let Some(__v) = gson::member(json, "bestiary") {
            if let Some(__x) = Some(Bestiary::from_json(__v)) {
                self.bestiary = __x;
            }
        }
        if let Some(__v) = gson::member(json, "collection") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_i64(__e0)) {
                self.collection = __x;
            }
        }
        if let Some(__v) = gson::member(json, "events") {
            if let Some(__x) = Some(Events::from_json(__v)) {
                self.events = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fairy_soul") {
            if let Some(__x) = Some(FairySouls::from_json(__v)) {
                self.fairy_souls = __x;
            }
        }
        if let Some(__v) = gson::member(json, "currencies") {
            if let Some(__x) = Some(Currencies::from_json(__v)) {
                self.currencies = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dungeons") {
            if let Some(__x) = Some(Dungeons::from_json(__v)) {
                self.dungeons = __x;
            }
        }
        if let Some(__v) = gson::member(json, "experimentation") {
            if let Some(__x) = Some(Experimentation::from_json(__v)) {
                self.experimentation = __x;
            }
        }
        if let Some(__v) = gson::member(json, "foraging") {
            if let Some(__x) = Some(Foraging::from_json(__v)) {
                self.foraging = __x;
            }
        }
        if let Some(__v) = gson::member(json, "foraging_core") {
            if let Some(__x) = Some(ForagingCore::from_json(__v)) {
                self.foraging_core = __x;
            }
        }
        if let Some(__v) = gson::member(json, "forge") {
            if let Some(__x) = Some(Forge::from_json(__v)) {
                self.forge = __x;
            }
        }
        if let Some(__v) = gson::member(json, "garden_player_data") {
            if let Some(__x) = Some(GardenPlayerData::from_json(__v)) {
                self.garden_player_data = __x;
            }
        }
        if let Some(__v) = gson::member(json, "glacite_player_data") {
            if let Some(__x) = Some(GlacitePlayerData::from_json(__v)) {
                self.glacite_player_data = __x;
            }
        }
        if let Some(__v) = gson::member(json, "inventory") {
            if let Some(__x) = Some(Inventories::from_json(__v)) {
                self.inventories = __x;
            }
        }
        if let Some(__v) = gson::member(json, "item_data") {
            if let Some(__x) = Some(ItemData::from_json(__v)) {
                self.item_data = __x;
            }
        }
        if let Some(__v) = gson::member(json, "jacobs_contest") {
            if let Some(__x) = Some(JacobsContest::from_json(__v)) {
                self.jacobs_contest = __x;
            }
        }
        if let Some(__v) = gson::member(json, "leveling") {
            if let Some(__x) = Some(Levelling::from_json(__v)) {
                self.levelling = __x;
            }
        }
        if let Some(__v) = gson::member(json, "loadout") {
            if let Some(__x) = Some(Loadouts::from_json(__v)) {
                self.loadouts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mining_core") {
            if let Some(__x) = Some(MiningCore::from_json(__v)) {
                self.mining_core = __x;
            }
        }
        if let Some(__v) = gson::member(json, "nether_island_player_data") {
            if let Some(__x) = Some(NetherIslandPlayerData::from_json(__v)) {
                self.nether_island_player_data = __x;
            }
        }
        if let Some(__v) = gson::member(json, "pets_data") {
            if let Some(__x) = Some(PetsData::from_json(__v)) {
                self.pets_data = __x;
            }
        }
        if let Some(__v) = gson::member(json, "player_data") {
            if let Some(__x) = Some(PlayerData::from_json(__v)) {
                self.player_data = __x;
            }
        }
        if let Some(__v) = gson::member(json, "player_stats") {
            if let Some(__x) = Some(PlayerStats::from_json(__v)) {
                self.player_stats = __x;
            }
        }
        if let Some(__v) = gson::member(json, "profile") {
            if let Some(__x) = Some(ProfileMemberProfile::from_json(__v)) {
                self.profile = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rift") {
            if let Some(__x) = Some(Rift::from_json(__v)) {
                self.rift = __x;
            }
        }
        if let Some(__v) = gson::member(json, "safari") {
            if let Some(__x) = Some(Safari::from_json(__v)) {
                self.safari = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shards") {
            if let Some(__x) = Some(Shards::from_json(__v)) {
                self.shards = __x;
            }
        }
        if let Some(__v) = gson::member(json, "skill_tree") {
            if let Some(__x) = Some(SkillTree::from_json(__v)) {
                self.skill_tree = __x;
            }
        }
        if let Some(__v) = gson::member(json, "slayer") {
            if let Some(__x) = Some(SlayerData::from_json(__v)) {
                self.slayer = __x;
            }
        }
        if let Some(__v) = gson::member(json, "temples") {
            if let Some(__x) = Some(Temples::from_json(__v)) {
                self.temples = __x;
            }
        }
        if let Some(__v) = gson::member(json, "trophy_fish") {
            if let Some(__x) = __v.as_object().cloned() {
                self.trophy_fish = __x;
            }
        }
        if let Some(__v) = gson::member(json, "winter_player_data") {
            if let Some(__x) = Some(WinterPlayerData::from_json(__v)) {
                self.winter_player_data = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ProfileMember.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}
