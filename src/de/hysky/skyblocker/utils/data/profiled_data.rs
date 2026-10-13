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
        //   java: super(file,
        //   java: 		// Mojang's internal Codec implementation uses ImmutableMaps so we'll just xmap those away and type safety while we're at it :')
        //   java: 		Codec.unboundedMap(UUIDUtil.AUTHLIB_CODEC,
        //   java: 				Codec.unboundedMap(Codec.STRING, codec).xmap(Object2ObjectOpenHashMap::new, Function.identity())
        //   java: 		).xmap(Object2ObjectOpenHashMap::new, Function.identity()),
        //   java: 		new Object2ObjectOpenHashMap<>(), compressed, loadAsync, saveAsync);
        // (the port's `unboundedMap` answers the entries in order; the `xmap`s make the hash maps Java's do)
        let profile_codec: CodecRef<std::collections::HashMap<String, T>> = codec::xmap(codec::unbounded_map(codec::string_codec(), codec.clone()), |entries: Vec<(String, T)>| entries.into_iter().collect::<std::collections::HashMap<String, T>>(), |map: &std::collections::HashMap<String, T>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<(String, T)>>());
        let data_codec: CodecRef<std::collections::HashMap<Uuid, std::collections::HashMap<String, T>>> = codec::xmap(codec::unbounded_map(mcrust::net::minecraft::core::uuid_util::authlib_codec(), profile_codec), |entries: Vec<(Uuid, std::collections::HashMap<String, T>)>| entries.into_iter().collect::<std::collections::HashMap<Uuid, std::collections::HashMap<String, T>>>(), |map: &std::collections::HashMap<Uuid, std::collections::HashMap<String, T>>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<(Uuid, std::collections::HashMap<String, T>)>>());
        let __this: compat::Shared<Self> = compat::shared(Self {
            base: JsonDataBase::new_6(file, data_codec, std::collections::HashMap::new(), compressed, load_async, save_async),
        });
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
        // (the live map - Java's `getPlayerData` answers the stored one)
        return ProfiledData::with_player_data(__this, uuid.clone(), |__map| __map.insert(profile_id.clone(), value.clone()));
    }

    /// java: public @Nullable T putIfAbsent(T value)
    pub fn put_if_absent(__this: &compat::Shared<Self>, mut value: T) -> Option<T> {
        return ProfiledData::put_if_absent_3(__this, &Utils::get_uuid(), Utils::get_profile_id(), value.clone());
    }

    /// java: public @Nullable T putIfAbsent(UUID uuid, String profileId, T value)
    pub fn put_if_absent_3(__this: &compat::Shared<Self>, uuid: &Uuid, mut profile_id: String, mut value: T) -> Option<T> {
        // `Map.putIfAbsent`: the value already there (no change), else `null` after the put
        return ProfiledData::with_player_data(__this, uuid.clone(), |__map| match __map.get(&profile_id) {
            Some(__existing) => Some(__existing.clone()),
            None => {
                __map.insert(profile_id.clone(), value.clone());
                None
            }
        });
    }

    /// java: public T computeIfAbsent(Supplier<T> valueSupplier)
    pub fn compute_if_absent(__this: &compat::Shared<Self>, value_supplier: Arc<dyn Fn() -> T>) -> T {
        return ProfiledData::compute_if_absent_3(__this, &Utils::get_uuid(), Utils::get_profile_id(), value_supplier.clone());
    }

    /// java: public T computeIfAbsent(UUID uuid, String profileId, Supplier<T> valueSupplier)
    pub fn compute_if_absent_3(__this: &compat::Shared<Self>, uuid: &Uuid, mut profile_id: String, value_supplier: Arc<dyn Fn() -> T>) -> T {
        // `Map.computeIfAbsent` on the live map; the supplier runs with no borrow of this object held
        if let Some(__existing) = ProfiledData::with_player_data(__this, uuid.clone(), |__map| __map.get(&profile_id).cloned()) {
            return __existing;
        }
        let __value: T = (value_supplier)();
        return ProfiledData::with_player_data(__this, uuid.clone(), |__map| __map.entry(profile_id.clone()).or_insert(__value).clone());
    }

    /// java: public @Nullable T remove()
    pub fn remove(__this: &compat::Shared<Self>) -> Option<T> {
        return ProfiledData::remove_2(__this, &Utils::get_uuid(), &Utils::get_profile_id());
    }

    /// java: public @Nullable T remove(UUID uuid, String profileId)
    pub fn remove_2(__this: &compat::Shared<Self>, uuid: &Uuid, profile_id: &str) -> Option<T> {
        return ProfiledData::with_player_data(__this, uuid.clone(), |__map| __map.remove(profile_id));
    }

    /// java: private Map<String, T> getPlayerData(UUID uuid)
    pub fn get_player_data(__this: &compat::Shared<Self>, mut uuid: Uuid) -> std::collections::HashMap<String, T> {
        //   java: return getData().computeIfAbsent(uuid, _ -> new Object2ObjectOpenHashMap<>());
        // (a copy of the live map, which is created here as Java's `computeIfAbsent` creates it; the mutating callers
        // write through `with_player_data`)
        return ProfiledData::with_player_data(__this, uuid, |__map| __map.clone());
    }

    /// `getPlayerData(uuid)` as the live map, lent to `f` - what every Java caller that mutates the answer reaches.
    fn with_player_data<R>(__this: &compat::Shared<Self>, uuid: Uuid, f: impl FnOnce(&mut std::collections::HashMap<String, T>) -> R) -> R {
        compat::write(__this, |__t| {
            let __data = __t.base.data.get_or_insert_with(std::collections::HashMap::new);
            f(__data.entry(uuid).or_insert_with(std::collections::HashMap::new))
        })
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
