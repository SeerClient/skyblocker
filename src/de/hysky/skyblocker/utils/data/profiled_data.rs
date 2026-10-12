//! Ported from de.hysky.skyblocker.utils.data.ProfiledData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonDataBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::uuid::Uuid;

/// java: public class ProfiledData<T> extends JsonData<Object2ObjectOpenHashMap<UUID, Object2ObjectOpenHashMap<String, T>>>
pub struct ProfiledData<T> {
    /// java: extends JsonData
    pub base: JsonDataBase<std::collections::HashMap<Uuid, std::collections::HashMap<String, T>>>,
}

impl<T: 'static> std::ops::Deref for ProfiledData<T> {
    type Target = JsonDataBase<std::collections::HashMap<Uuid, std::collections::HashMap<String, T>>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for ProfiledData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> ProfiledData<T> {
    /// java: public ProfiledData(Path file, Codec<T> codec)
    pub fn new(mut file: std::path::PathBuf, mut codec: CodecRef<T>) -> compat::Shared<Self> {
        let __this = Self::new_3(file.clone(), codec.clone(), false);
        __this
    }

    /// java: public ProfiledData(Path file, Codec<T> codec, boolean compressed)
    pub fn new_3(mut file: std::path::PathBuf, mut codec: CodecRef<T>, mut compressed: bool) -> compat::Shared<Self> {
        let __this = Self::new_5(file.clone(), codec.clone(), compressed, true, true);
        __this
    }

    /// java: public ProfiledData(Path file, Codec<T> codec, boolean loadAsync, boolean saveAsync)
    pub fn new_4(mut file: std::path::PathBuf, mut codec: CodecRef<T>, mut load_async: bool, mut save_async: bool) -> compat::Shared<Self> {
        let __this = Self::new_5(file.clone(), codec.clone(), false, load_async, save_async);
        __this
    }

    /// java: public ProfiledData(Path file, Codec<T> codec, boolean compressed, boolean loadAsync, boolean saveAsync)
    pub fn new_5(mut file: std::path::PathBuf, mut codec: CodecRef<T>, mut compressed: bool, mut load_async: bool, mut save_async: bool) -> compat::Shared<Self> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: super(file,
        //   java: 				// Mojang's internal Codec implementation uses ImmutableMaps so we'll just xmap those away and type safety while we're at it :')
        //   java: 				Codec.unboundedMap(UUIDUtil.AUTHLIB_CODEC,
        //   java: 						Codec.unboundedMap(Codec.STRING, codec).xmap(Object2ObjectOpenHashMap::new, Function.identity())
        //   java: 				).xmap(Object2ObjectOpenHashMap::new, Function.identity()),
        //   java: 				new Object2ObjectOpenHashMap<>(),
        //   java: 				compressed,
        //   java: 				loadAsync,
        //   java: 				saveAsync);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public boolean containsKey()
    pub fn contains_key(__this: &compat::Shared<Self>) -> bool {
        return ProfiledData::contains_key_2(__this, &Utils::get_uuid(), &Utils::get_profile_id());
    }

    /// java: public boolean containsKey(UUID uuid, String profileId)
    pub fn contains_key_2(__this: &compat::Shared<Self>, uuid: &Uuid, profile_id: &str) -> bool {
        return (ProfiledData::get_player_data(__this, uuid.clone())).contains_key(profile_id);
    }

    /// java: public @Nullable T get()
    pub fn get(__this: &compat::Shared<Self>) -> Option<T> {
        return ProfiledData::get_2(__this, &Utils::get_uuid(), &Utils::get_profile_id());
    }

    /// java: public @Nullable T get(UUID uuid, String profileId)
    pub fn get_2(__this: &compat::Shared<Self>, uuid: &Uuid, profile_id: &str) -> Option<T> {
        return (ProfiledData::get_player_data(__this, uuid.clone())).get(profile_id).cloned();
    }

    /// java: public @Nullable T put(T value)
    pub fn put(__this: &compat::Shared<Self>, mut value: T) -> Option<T> {
        return ProfiledData::put_3(__this, &Utils::get_uuid(), Utils::get_profile_id(), value.clone());
    }

    /// java: public @Nullable T put(UUID uuid, String profileId, T value)
    pub fn put_3(__this: &compat::Shared<Self>, uuid: &Uuid, mut profile_id: String, mut value: T) -> Option<T> {
        return (ProfiledData::get_player_data(__this, uuid.clone())).insert(profile_id.clone(), value.clone());
    }

    /// java: public @Nullable T putIfAbsent(T value)
    pub fn put_if_absent(__this: &compat::Shared<Self>, mut value: T) -> Option<T> {
        return ProfiledData::put_if_absent_3(__this, &Utils::get_uuid(), Utils::get_profile_id(), value.clone());
    }

    /// java: public @Nullable T putIfAbsent(UUID uuid, String profileId, T value)
    pub fn put_if_absent_3(__this: &compat::Shared<Self>, uuid: &Uuid, mut profile_id: String, mut value: T) -> Option<T> {
        return Some((ProfiledData::get_player_data(__this, uuid.clone())).entry(profile_id.clone()).or_insert(value.clone()).clone());
    }

    /// java: public T computeIfAbsent(Supplier<T> valueSupplier)
    pub fn compute_if_absent(__this: &compat::Shared<Self>, value_supplier: Arc<dyn Fn() -> T>) -> T {
        return ProfiledData::compute_if_absent_3(__this, &Utils::get_uuid(), Utils::get_profile_id(), value_supplier.clone());
    }

    /// java: public T computeIfAbsent(UUID uuid, String profileId, Supplier<T> valueSupplier)
    pub fn compute_if_absent_3(__this: &compat::Shared<Self>, uuid: &Uuid, mut profile_id: String, value_supplier: Arc<dyn Fn() -> T>) -> T {
        return (ProfiledData::get_player_data(__this, uuid.clone())).entry(profile_id.clone()).or_insert_with_key(|k| (|_| (value_supplier)())(k.clone())).clone();
    }

    /// java: public @Nullable T remove()
    pub fn remove(__this: &compat::Shared<Self>) -> Option<T> {
        return ProfiledData::remove_2(__this, &Utils::get_uuid(), &Utils::get_profile_id());
    }

    /// java: public @Nullable T remove(UUID uuid, String profileId)
    pub fn remove_2(__this: &compat::Shared<Self>, uuid: &Uuid, profile_id: &str) -> Option<T> {
        return (ProfiledData::get_player_data(__this, uuid.clone())).remove(profile_id);
    }

    /// java: private Map<String, T> getPlayerData(UUID uuid)
    pub fn get_player_data(__this: &compat::Shared<Self>, mut uuid: Uuid) -> std::collections::HashMap<String, T> {
        // TODO(fabric2seer): `.computeIfAbsent(..)` on a T
        //   java: return getData().computeIfAbsent(uuid, _ -> new Object2ObjectOpenHashMap<>());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl<T: Clone + 'static> JsonData<std::collections::HashMap<mcrust::java::util::uuid::Uuid, std::collections::HashMap<String, T>>> for ProfiledData<T> {
    fn json_data_base(&self) -> &JsonDataBase<std::collections::HashMap<mcrust::java::util::uuid::Uuid, std::collections::HashMap<String, T>>> {
        &self.base
    }
    fn json_data_base_mut(&mut self) -> &mut JsonDataBase<std::collections::HashMap<mcrust::java::util::uuid::Uuid, std::collections::HashMap<String, T>>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
