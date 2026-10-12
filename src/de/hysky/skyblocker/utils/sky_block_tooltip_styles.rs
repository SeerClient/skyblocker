//! Ported from de.hysky.skyblocker.utils.SkyBlockTooltipStyles by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::utils;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final Identifier COMMON = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "common");
pub static COMMON: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "common"));
/// java: public static final Identifier UNCOMMON = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "uncommon");
pub static UNCOMMON: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "uncommon"));
/// java: public static final Identifier RARE = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "rare");
pub static RARE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "rare"));
/// java: public static final Identifier EPIC = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "epic");
pub static EPIC: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "epic"));
/// java: public static final Identifier LEGENDARY = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "legendary");
pub static LEGENDARY: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "legendary"));
/// java: public static final Identifier MYTHIC = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "mythic");
pub static MYTHIC: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "mythic"));
/// java: public static final Identifier DIVINE = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "supreme");
pub static DIVINE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "supreme"));
/// java: public static final Identifier SPECIAL = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "special");
pub static SPECIAL: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "special"));
/// java: public static final Identifier VERY_SPECIAL = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "very_special");
pub static VERY_SPECIAL: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "very_special"));
/// java: public static final Identifier ULTIMATE = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "ultimate");
pub static ULTIMATE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "ultimate"));
/// java: public static final Identifier ADMIN = Identifier.fromNamespaceAndPath(Utils.HYPIXEL_SKYBLOCK_NAMESPACE, "admin");
pub static ADMIN: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::from_namespace_and_path(utils::HYPIXEL_SKYBLOCK_NAMESPACE, "admin"));

/// java: public final class SkyBlockTooltipStyles
#[derive(Clone)]
pub struct SkyBlockTooltipStyles {}

impl SkyBlockTooltipStyles {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
