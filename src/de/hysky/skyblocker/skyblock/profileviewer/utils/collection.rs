//! Ported from de.hysky.skyblocker.skyblock.profileviewer.utils.Collection by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::decoder::Decoder;

/// java: private static final Decoder<IntList> TIERS_DECODER = Codec.PASSTHROUGH.map(tiersDynamic -> IntLists.unmodifiable(tiersDynamic.asStream()
///     .<Number>mapMulti((tierDynamic, consumer) -> tierDynamic.get("amountRequired").asNumber().ifSuccess(consumer))
///     .mapToInt(Number::intValue).collect(IntArrayList::of, IntList::add, IntList::addAll)));
pub static TIERS_DECODER: compat::MainThread<Arc<dyn Decoder<Vec<i32>>>> = compat::MainThread::new(|| Arc::new(TiersDecoder { passthrough: mcrust::com::mojang::serialization::codec::passthrough() }) as Arc<dyn Decoder<Vec<i32>>>);

/// `Codec.PASSTHROUGH.map(..)` - `Decoder.map:55-67`: the passthrough's decode, its value mapped and the remainder
/// carried through.
struct TiersDecoder {
    passthrough: mcrust::com::mojang::serialization::codec::CodecRef<mcrust::com::mojang::serialization::dynamic::Dynamic>,
}

impl Decoder<Vec<i32>> for TiersDecoder {
    fn decode(&self, ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops, input: &mcrust::com::mojang::serialization::Obj) -> mcrust::com::mojang::serialization::data_result::DataResult<(Vec<i32>, mcrust::com::mojang::serialization::Obj)> {
        use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
        self.passthrough.decode(ops, input).map(|(tiers_dynamic, rest)| {
            let mut tiers: Vec<i32> = Vec::new();
            for tier_dynamic in tiers_dynamic.as_stream() {
                let _ = tier_dynamic.get("amountRequired").as_number().if_success(|number| tiers.push(number.int_value()));
            }
            (tiers, rest)
        })
    }
}

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
        // java: for (Map.Entry<String, JsonElement> skillEntry : object.getAsJsonObject("collections").asMap().entrySet()) {
        let collections_json = compat::json::get_as_json_object(object.get("collections").expect("NullPointerException: collections"));
        for (skill_key, skill_value) in collections_json {
            let mut collections: Vec<Collection> = Vec::new();
            // java: for (Map.Entry<String, JsonElement> collectionEntry : skillEntry.getValue().getAsJsonObject().getAsJsonObject("items").asMap().entrySet()) {
            let items = compat::json::get_as_json_object(compat::json::get_as_json_object(&skill_value).get("items").expect("NullPointerException: items"));
            for (id, collection_value) in items {
                let collection_object = compat::json::get_as_json_object(&collection_value);
                // java: String name = collectionEntry.getValue().getAsJsonObject().get("name").getAsString();
                let name: String = compat::json::get_as_string(collection_object.get("name").expect("NullPointerException: name"));
                // java: IntList tiers = TIERS_DECODER.parse(JsonOps.INSTANCE, collectionEntry.getValue().getAsJsonObject().get("tiers")).getOrThrow();
                let tiers_json = collection_object.get("tiers").cloned().unwrap_or(serde_json::Value::Null);
                let tiers: Vec<i32> = TIERS_DECODER.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&tiers_json)).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
                collections.push(Collection::new(id, name, tiers));
            }
            // java: collections.sort(Comparator.comparing(Collection::name, String::compareToIgnoreCase)); (a stable sort)
            collections.sort_by(|a, b| mcrust::javacompat::java_string_compare_ignore_case(&a.name, &b.name).cmp(&0));
            skill_collections.insert(skill_key, collections);
        }
        return skill_collections.clone();
    }

}
