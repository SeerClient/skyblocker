//! Ported from de.hysky.skyblocker.skyblock.profileviewer.utils.Collection by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::decoder::Decoder;

// TODO(fabric2seer): static call IntLists.unmodifiable(..)
//   java: private static final Decoder<IntList> TIERS_DECODER = Codec.PASSTHROUGH.map(tiersDynamic -> IntLists.unmodifiable(tiersDynamic.asStream()
//   java: 			.<Number>mapMulti((tierDynamic, consumer) -> tierDynamic.get("amountRequired").asNumber().ifSuccess(consumer))
//   java: 			.mapToInt(Number::intValue)
//   java: 			.collect(IntArrayList::of, IntList::add, IntList::addAll)));
/// java: private static final Decoder<IntList> TIERS_DECODER = Codec.PASSTHROUGH.map(tiersDynamic -> IntLists.unmodifiable(tiersDynamic.asStream() .<Number>mapMulti((tierDynamic, consumer) -> tierDynamic.get("
pub static TIERS_DECODER: compat::MainThread<Arc<dyn Decoder<Vec<i32>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `TIERS_DECODER` - see the TODO above"));

/// java: public record Collection(String id, String name, IntList tiers)
#[derive(Clone, Debug, PartialEq)]
pub struct Collection {
    /// java: String id
    pub id: String,
    /// java: String name
    pub name: String,
    /// java: IntList tiers
    pub tiers: Vec<i32>,
}

impl Collection {
    /// java: the canonical record constructor
    pub fn new(mut id: String, mut name: String, mut tiers: Vec<i32>) -> Self {
        Self { id, name, tiers }
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `tiers()`
    pub fn tiers(&self) -> Vec<i32> {
        self.tiers.clone()
    }

    /// java: public static Map<String, List<Collection>> parse(JsonObject object)
    pub fn parse(object: &serde_json::Map<String, serde_json::Value>) -> std::collections::HashMap<String, Vec<Collection>> {
        if !(compat::json::get_as_boolean(&object.get("success").cloned().unwrap())) {
            return std::collections::HashMap::from([]);
        }
        let mut skill_collections: std::collections::HashMap<String, Vec<Collection>> = std::collections::HashMap::new();
        // TODO(fabric2seer): JsonObject.asMap
        //   java: for (Map.Entry<String, JsonElement> skillEntry : object.getAsJsonObject("collections").asMap().entrySet()) {
        //   java: 			List<Collection> collections = new ArrayList<>();
        //   java:
        //   java: 			for (Map.Entry<String, JsonElement> collectionEntry : skillEntry.getValue().getAsJsonObject().getAsJsonObject("items").asMap().entrySet()) {
        //   java: 				String id = collectionEntry.getKey();
        //   java: 				String name = collectionEntry.getValue().getAsJsonObject().get("name").getAsString();
        //   java: 				IntList tiers = TIERS_DECODER.parse(JsonOps.INSTANCE,  collectionEntry.getValue().getAsJsonObject().get("tiers")).getOrThrow();
        //   java: 				Collection collection = new Collection(id, name, tiers);
        //   java:
        //   java: 				collections.add(collection);
        //   java: 			}
        //   java:
        //   java: 			// Sort collections by name
        //   java: 			collections.sort(Comparator.comparing(Collection::name, String::compareToIgnoreCase));
        //   java: 			skillCollections.put(skillEntry.getKey(), List.copyOf(collections));
        //   java: 		}
        return skill_collections.clone();
    }

}
