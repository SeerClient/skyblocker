//! Ported from io.github.moulberry.repo.NEUConstants (neurepoparser 1.12.0).

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde_json::Value;

use crate::io::github::moulberry::repo::constants::abiphone_contact::AbiphoneContact;
use crate::io::github::moulberry::repo::constants::bazaar_stocks::{BazaarStocks, InternalRepresentation};
use crate::io::github::moulberry::repo::constants::bonuses::Bonuses;
use crate::io::github::moulberry::repo::constants::enchants::Enchants;
use crate::io::github::moulberry::repo::constants::essence_costs::EssenceCosts;
use crate::io::github::moulberry::repo::constants::fairy_souls::FairySouls;
use crate::io::github::moulberry::repo::constants::islands::Islands;
use crate::io::github::moulberry::repo::constants::leveling::Leveling;
use crate::io::github::moulberry::repo::constants::misc::Misc;
use crate::io::github::moulberry::repo::constants::parents::Parents;
use crate::io::github::moulberry::repo::constants::pet_leveling_data::PetLevelingData;
use crate::io::github::moulberry::repo::constants::pet_numbers::PetNumbers;
use crate::io::github::moulberry::repo::constants::resource_pack::ResourcePack;
use crate::io::github::moulberry::repo::data::rarity::Rarity;
use crate::io::github::moulberry::repo::i_reloadable::IReloadable;
use crate::io::github::moulberry::repo::neu_repo_file::NEURepoFile;
use crate::io::github::moulberry::repo::neu_repository::{NEURepository, gson};
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;

/// `Map<@PetId String, Map<Rarity, PetNumbers>>`
pub type PetNumbersMap = HashMap<String, HashMap<Rarity, PetNumbers>>;

/// One constant field: Java reassigns the field on each reload while other threads read it; here it is swapped
/// under a lock and a getter hands out the current object (`Arc`), so a reader keeps a consistent snapshot as Java's
/// local copy of the reference does. Before the first reload Java's field is `null`; the port answers the class's
/// empty value there (every Skyblocker reader asks after the load, behind `NEURepoManager.isLoading()`).
type Field<T> = RwLock<Arc<T>>;

fn get<T>(field: &Field<T>) -> Arc<T> {
    field.read().unwrap_or_else(|p| p.into_inner()).clone()
}

fn set<T>(field: &Field<T>, value: T) {
    *field.write().unwrap_or_else(|p| p.into_inner()) = Arc::new(value);
}

/// java: @Getter public class NEUConstants implements IReloadable
#[derive(Default)]
pub struct NEUConstants {
    /// java: Map<String, AbiphoneContact> abiphoneContacts;
    abiphone_contacts: Field<HashMap<String, AbiphoneContact>>,
    /// java: Bonuses bonuses;
    bonuses: Field<Bonuses>,
    /// java: Parents parents;
    parents: Field<Parents>,
    /// java: Enchants enchants;
    enchants: Field<Enchants>,
    /// java: EssenceCosts essenceCost;
    essence_cost: Field<EssenceCosts>,
    /// java: FairySouls fairySouls;
    fairy_souls: Field<FairySouls>,
    /// java: Misc misc;
    misc: Field<Misc>,
    /// java: Leveling leveling;
    leveling: Field<Leveling>,
    /// java: PetLevelingData petLevelingData;
    pet_leveling_data: Field<PetLevelingData>,
    /// java: Map<@PetId String, Map<Rarity, PetNumbers>> petNumbers;
    pet_numbers: Field<PetNumbersMap>,
    /// java: Islands islands;
    islands: Field<Islands>,
    /// java: BazaarStocks bazaarStocks;
    bazaar_stocks: Field<BazaarStocks>,
    /// java: ResourcePack resourcePack;
    resource_pack: Field<ResourcePack>,
}

impl NEUConstants {
    pub(crate) fn new() -> Self {
        NEUConstants::default()
    }

    /// java: public Map<String, AbiphoneContact> getAbiphoneContacts()
    pub fn get_abiphone_contacts(&self) -> Arc<HashMap<String, AbiphoneContact>> {
        get(&self.abiphone_contacts)
    }

    /// java: public Bonuses getBonuses()
    pub fn get_bonuses(&self) -> Arc<Bonuses> {
        get(&self.bonuses)
    }

    /// java: public Parents getParents()
    pub fn get_parents(&self) -> Arc<Parents> {
        get(&self.parents)
    }

    /// java: public Enchants getEnchants()
    pub fn get_enchants(&self) -> Arc<Enchants> {
        get(&self.enchants)
    }

    /// java: public EssenceCosts getEssenceCost()
    pub fn get_essence_cost(&self) -> Arc<EssenceCosts> {
        get(&self.essence_cost)
    }

    /// java: public FairySouls getFairySouls()
    pub fn get_fairy_souls(&self) -> Arc<FairySouls> {
        get(&self.fairy_souls)
    }

    /// java: public Misc getMisc()
    pub fn get_misc(&self) -> Arc<Misc> {
        get(&self.misc)
    }

    /// java: public Leveling getLeveling()
    pub fn get_leveling(&self) -> Arc<Leveling> {
        get(&self.leveling)
    }

    /// java: public PetLevelingData getPetLevelingData()
    pub fn get_pet_leveling_data(&self) -> Arc<PetLevelingData> {
        get(&self.pet_leveling_data)
    }

    /// java: public Map<@PetId String, Map<Rarity, PetNumbers>> getPetNumbers()
    pub fn get_pet_numbers(&self) -> Arc<PetNumbersMap> {
        get(&self.pet_numbers)
    }

    /// java: public Islands getIslands()
    pub fn get_islands(&self) -> Arc<Islands> {
        get(&self.islands)
    }

    /// java: public BazaarStocks getBazaarStocks()
    pub fn get_bazaar_stocks(&self) -> Arc<BazaarStocks> {
        get(&self.bazaar_stocks)
    }

    /// java: public ResourcePack getResourcePack()
    pub fn get_resource_pack(&self) -> Arc<ResourcePack> {
        get(&self.resource_pack)
    }
}

impl IReloadable for NEUConstants {
    /// java: public void reload(NEURepository repository) throws NEURepositoryException
    fn reload(&self, repository: &Arc<NEURepository>) -> Result<(), NEURepositoryException> {
        // java: abiphoneContacts = repository.requireFile("constants/abiphone.json").json(new TypeToken<Map<String, AbiphoneContact>>() {});
        set(
            &self.abiphone_contacts,
            repository
                .require_file("constants/abiphone.json")?
                .json_as(|json| gson::map(json, gson::string_key, |c| Some(AbiphoneContact::from_json(c))).unwrap_or_default())?,
        );
        // java: bonuses = repository.requireFile("constants/bonuses.json").json(Bonuses.class);
        set(&self.bonuses, repository.require_file("constants/bonuses.json")?.json_as(Bonuses::from_json)?);
        // java: parents = new Parents(repository.requireFile("constants/parents.json").json(new TypeToken<Map<String, List<String>>>() {}));
        set(
            &self.parents,
            Parents::new(
                repository
                    .require_file("constants/parents.json")?
                    .json_as(|json| gson::map(json, gson::string_key, |l| gson::list(l, gson::as_string)))?,
            ),
        );
        // java: enchants = repository.requireFile("constants/enchants.json").json(Enchants.class);
        set(&self.enchants, repository.require_file("constants/enchants.json")?.json_as(Enchants::from_json)?);
        // java: essenceCost = new EssenceCosts(repository.requireFile("constants/essencecosts.json").json(JsonObject.class));
        set(&self.essence_cost, EssenceCosts::new(&repository.require_file("constants/essencecosts.json")?.json()?)?);
        // java: fairySouls = new FairySouls(repository.gson, repository.requireFile("constants/fairy_souls.json").json(new TypeToken<Map<String, JsonElement>>() {}));
        let fairy_souls_json: Value = repository.require_file("constants/fairy_souls.json")?.json()?;
        set(&self.fairy_souls, FairySouls::new(fairy_souls_json.as_object().unwrap_or(&serde_json::Map::new())));
        // java: misc = repository.requireFile("constants/misc.json").json(Misc.class);
        set(&self.misc, repository.require_file("constants/misc.json")?.json_as(Misc::from_json)?);
        // java: leveling = repository.requireFile("constants/leveling.json").json(Leveling.class);
        set(&self.leveling, repository.require_file("constants/leveling.json")?.json_as(Leveling::from_json)?);
        // java: petLevelingData = repository.requireFile("constants/pets.json").json(PetLevelingData.class);
        set(&self.pet_leveling_data, repository.require_file("constants/pets.json")?.json_as(PetLevelingData::from_json)?);
        // java: petNumbers = repository.requireFile("constants/petnums.json").json(new TypeToken<Map<@PetId String, Map<Rarity, PetNumbers>>>() {});
        set(
            &self.pet_numbers,
            repository.require_file("constants/petnums.json")?.json_as(|json| {
                gson::map(json, gson::string_key, |by_rarity| gson::map(by_rarity, Rarity::value_of, |n| Some(PetNumbers::from_json(n))))
                    .unwrap_or_default()
            })?,
        );
        // java: NEURepoFile islandsFile = repository.file("constants/islands.json");
        //       islands = islandsFile != null ? islandsFile.json(Islands.class) : new Islands();
        let islands_file: Option<NEURepoFile> = repository.file("constants/islands.json");
        set(
            &self.islands,
            match islands_file {
                Some(islands_file) => islands_file.json_as(Islands::from_json)?,
                None => Islands::new(),
            },
        );
        // java: NEURepoFile bazaarFile = repository.file("constants/bazaarstocks.json");
        //       bazaarStocks = new BazaarStocks(bazaarFile != null ? (bazaarFile.json(new TypeToken<List<BazaarStocks.InternalRepresentation>>() {})
        //           .stream().collect(Collectors.toMap(InternalRepresentation::getId, InternalRepresentation::getStock))) : (Collections.emptyMap()));
        let bazaar_file: Option<NEURepoFile> = repository.file("constants/bazaarstocks.json");
        let stocks: HashMap<String, String> = match bazaar_file {
            Some(bazaar_file) => {
                let list: Vec<InternalRepresentation> =
                    bazaar_file.json_as(|json| gson::list(json, |e| Some(InternalRepresentation::from_json(e))).unwrap_or_default())?;
                let mut map: HashMap<String, String> = HashMap::new();
                for representation in list {
                    let id = representation.id.unwrap_or_else(|| panic!("java.lang.NullPointerException: bazaar stock without an id"));
                    // java: Collectors.toMap throws IllegalStateException on a duplicate key and NullPointerException on a null value
                    let stock = representation.stock.unwrap_or_else(|| panic!("java.lang.NullPointerException: bazaar stock {} has no stock", id));
                    if map.contains_key(&id) {
                        panic!("java.lang.IllegalStateException: Duplicate key {}", id);
                    }
                    map.insert(id, stock);
                }
                map
            }
            None => HashMap::new(),
        };
        set(&self.bazaar_stocks, BazaarStocks::new(stocks));
        // java: resourcePack = repository.requireFile("constants/resource_pack.json").json(ResourcePack.class);
        set(&self.resource_pack, repository.require_file("constants/resource_pack.json")?.json_as(ResourcePack::from_json)?);
        Ok(())
    }
}
