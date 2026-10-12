//! Ported from de.hysky.skyblocker.utils.SkyBlockBiomes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::utils;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final Identifier CAVERN = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_NAMESPACE, "cavern");
pub static CAVERN: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_NAMESPACE, "cavern"));
/// java: public static final Identifier FOREST = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_NAMESPACE, "forest");
pub static FOREST: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_NAMESPACE, "forest"));
/// java: public static final Identifier HAUNTED = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_NAMESPACE, "haunted");
pub static HAUNTED: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_NAMESPACE, "haunted"));
/// java: public static final Identifier ICY = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_NAMESPACE, "icy");
pub static ICY: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_NAMESPACE, "icy"));
/// java: public static final Identifier ICY_CAVES = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_NAMESPACE, "icy_caves");
pub static ICY_CAVES: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_NAMESPACE, "icy_caves"));

/// java: public final class SkyBlockBiomes
#[derive(Clone)]
pub struct SkyBlockBiomes {}

impl SkyBlockBiomes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
