//! package io.github.moulberry.repo - the NEU repository parser (`moe.nea:neurepoparser` 1.12.0, the version
//! Skyblocker's `gradle.properties` pins as `repoparser_version`), ported from its published sources jar
//! (`repo.nea.moe/releases/moe/nea/neurepoparser/1.12.0/neurepoparser-1.12.0-sources.jar`) alongside the mod, as
//! Fabric loads it beside it (`include implementation`).
//!
//! Gson's reflective binding has no Rust equivalent; each class that Java binds with `gson.fromJson(.., X.class)`
//! has a `from_json(&serde_json::Value)` that reads the same JSON member names (`@SerializedName` or the field
//! name), and each registered `JsonDeserializer` is ported as that class's `Serializer::deserialize`.
pub mod constants;
pub mod data;
pub mod i_reloadable;
pub mod neu_constants;
pub mod neu_items;
pub mod neu_recipe_cache;
pub mod neu_repo_file;
pub mod neu_repository;
pub mod neu_repository_exception;
pub mod neu_repository_version;
pub mod util;
pub mod vendored;
