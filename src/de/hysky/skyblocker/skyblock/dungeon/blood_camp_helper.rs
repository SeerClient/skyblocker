//! Ported from de.hysky.skyblocker.skyblock.dungeon.BloodCampHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Set<String> WATCHER_SKINS = Set.of( "ewogICJ0aW1lc3RhbXAiIDogMTcxOTYwNjM1MjMyMiwKICAicHJvZmlsZUlkIiA6ICI3MmY5MTdjNWQyNDU0OTk0YjlmYzQ1YjVhM2YyMjIzMCIsCiAgInByb2ZpbGVOYW1lIiA6ICJUaG
pub static WATCHER_SKINS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["ewogICJ0aW1lc3RhbXAiIDogMTcxOTYwNjM1MjMyMiwKICAicHJvZmlsZUlkIiA6ICI3MmY5MTdjNWQyNDU0OTk0YjlmYzQ1YjVhM2YyMjIzMCIsCiAgInByb2ZpbGVOYW1lIiA6ICJUaGF0X0d1eV9Jc19NZSIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS8yNzM5ZDdmNGU2NmE3ZGIyZWE2Y2Q0MTRlNGM0YmE0MWRmN2E5MjQ1NWM5ZmM0MmNhYWIwMTQ2NjVjMzY3YWQ1IiwKICAgICAgIm1ldGFkYXRhIiA6IHsKICAgICAgICAibW9kZWwiIDogInNsaW0iCiAgICAgIH0KICAgIH0KICB9Cn0=".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTcxOTYwNjI5MjgzNiwKICAicHJvZmlsZUlkIiA6ICIzZDIxZTYyMTk2NzQ0Y2QwYjM3NjNkNTU3MWNlNGJlZSIsCiAgInByb2ZpbGVOYW1lIiA6ICJTcl83MUJsYWNrYmlyZCIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS9iZjZlMWU3ZWQzNjU4NmMyZDk4MDU3MDAyYmMxYWRjOTgxZTI4ODlmN2JkN2I1YjM4NTJiYzU1Y2M3ODAyMjA0IiwKICAgICAgIm1ldGFkYXRhIiA6IHsKICAgICAgICAibW9kZWwiIDogInNsaW0iCiAgICAgIH0KICAgIH0KICB9Cn0=".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTcxOTYwNjIzOTU4NiwKICAicHJvZmlsZUlkIiA6ICJhYWZmMDUwYTExOTk0NzM1YjEyNDVlNDk0MGFlZjY4NCIsCiAgInByb2ZpbGVOYW1lIiA6ICJMYXN0SW1tb3J0YWwiLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZTVjMWRjNDdhMDRjZTU3MDAxYThiNzI2ZjAxOGNkZWY0MGI3ZWE5ZDdiZDZkODM1Y2E0OTVhMGVmMTY5Zjg5MyIsCiAgICAgICJtZXRhZGF0YSIgOiB7CiAgICAgICAgIm1vZGVsIiA6ICJzbGltIgogICAgICB9CiAgICB9CiAgfQp9".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTY5NzMwOTQxNzI1NiwKICAicHJvZmlsZUlkIiA6ICJjYjYxY2U5ODc4ZWI0NDljODA5MzliNWYxNTkwMzE1MiIsCiAgInByb2ZpbGVOYW1lIiA6ICJWb2lkZWRUcmFzaDUxODUiLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvNTY2MmI2ZmI0YjhiNTg2ZGM0Y2RmODAzYjA0NDRkOWI0MWQyNDVjZGY2NjhkYWIzOGZhNmMwNjRhZmU4ZTQ2MSIsCiAgICAgICJtZXRhZGF0YSIgOiB7CiAgICAgICAgIm1vZGVsIiA6ICJzbGltIgogICAgICB9CiAgICB9CiAgfQp9".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTY5NzIzODQ0NjgxMiwKICAicHJvZmlsZUlkIiA6ICJmMjc0YzRkNjI1MDQ0ZTQxOGVmYmYwNmM3NWIyMDIxMyIsCiAgInByb2ZpbGVOYW1lIiA6ICJIeXBpZ3NlbCIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS80Y2VjNDAwMDhlMWMzMWMxOTg0ZjRkNjUwYWJiMzQxMGYyMDM3MTE5ZmQ2MjRhZmM5NTM1NjNiNzM1MTVhMDc3IiwKICAgICAgIm1ldGFkYXRhIiA6IHsKICAgICAgICAibW9kZWwiIDogInNsaW0iCiAgICAgIH0KICAgIH0KICB9Cn0=".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTcxOTYwNjIxMjc1NSwKICAicHJvZmlsZUlkIiA6ICI2NGRiNmMwNTliOTk0OTM2YTY0M2QwODEwODE0ZmJkMyIsCiAgInByb2ZpbGVOYW1lIiA6ICJUaGVTaWx2ZXJEcmVhbXMiLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvOWZkNjFlODA1NWY2ZWU5N2FiNWI2MTk2YThkN2VjOTgwNzhhYzM3ZTAwMzc2MTU3YjZiNTIwZWFhYTJmOTNhZiIsCiAgICAgICJtZXRhZGF0YSIgOiB7CiAgICAgICAgIm1vZGVsIiA6ICJzbGltIgogICAgICB9CiAgICB9CiAgfQp9".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTcxOTYwNjAwOTg2NywKICAicHJvZmlsZUlkIiA6ICJiMGQ0YjI4YmMxZDc0ODg5YWYwZTg2NjFjZWU5NmFhYiIsCiAgInByb2ZpbGVOYW1lIiA6ICJNaW5lU2tpbl9vcmciLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYjM3ZGQxOGI1OTgzYTc2N2U1NTZkYzY0NDI0YWY0YjlhYmRiNzVkNGM5ZThiMDk3ODE4YWZiYzQzMWJmMGUwOSIsCiAgICAgICJtZXRhZGF0YSIgOiB7CiAgICAgICAgIm1vZGVsIiA6ICJzbGltIgogICAgICB9CiAgICB9CiAgfQp9".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTcxOTYwNTkyNDIwNSwKICAicHJvZmlsZUlkIiA6ICIzZDIxZTYyMTk2NzQ0Y2QwYjM3NjNkNTU3MWNlNGJlZSIsCiAgInByb2ZpbGVOYW1lIiA6ICJTcl83MUJsYWNrYmlyZCIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS9mNWYwZDc4ZmUzOGQxZDdmNzVmMDhjZGNmMmExODU1ZDZkYTAzMzdlMTE0YTNjNjNlM2JmM2M2MThiYzczMmIwIiwKICAgICAgIm1ldGFkYXRhIiA6IHsKICAgICAgICAibW9kZWwiIDogInNsaW0iCiAgICAgIH0KICAgIH0KICB9Cn0=".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU4OTU1MDkyNjM2MSwKICAicHJvZmlsZUlkIiA6ICI0ZDcwNDg2ZjUwOTI0ZDMzODZiYmZjOWMxMmJhYjRhZSIsCiAgInByb2ZpbGVOYW1lIiA6ICJzaXJGYWJpb3pzY2hlIiwKICAic2lnbmF0dXJlUmVxdWlyZWQiIDogdHJ1ZSwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzUxOTY3ZGI1ZTMxOTk5MTYyNTIwMjE5MDNjZjRlOTk1MmVmN2NlYzIyMGZhYWNhMWJhNzliYWZlNTkzOGJkODAiCiAgICB9CiAgfQp9".to_string()]));
/// java: private static final Set<String> BLOOD_MOB_SKINS = Set.of( "ewogICJ0aW1lc3RhbXAiIDogMTc1MTMyNzE1NzA5NCwKICAicHJvZmlsZUlkIiA6ICIyNjAyZmI2N2E5MWY0MDg0OWFmOGIyMmNiMzJlNTc5ZSIsCiAgInByb2ZpbGVOYW1lIiA6ICJJ
pub static BLOOD_MOB_SKINS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["ewogICJ0aW1lc3RhbXAiIDogMTc1MTMyNzE1NzA5NCwKICAicHJvZmlsZUlkIiA6ICIyNjAyZmI2N2E5MWY0MDg0OWFmOGIyMmNiMzJlNTc5ZSIsCiAgInByb2ZpbGVOYW1lIiA6ICJJbElJbGxJbGxJbGxJSWxJIiwKICAic2lnbmF0dXJlUmVxdWlyZWQiIDogdHJ1ZSwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2ZiMTU2Y2VlMzcwNzA2NDA4YmIwNjcyNjFmNTkzODZmMjgxZWFmMGJjMjRkMTY4ZDlkMDFiMTMwMTI5NDZkMDQiCiAgICB9CiAgfQp9".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTcyMDAyMjEyMzM0NiwKICAicHJvZmlsZUlkIiA6ICJkMDlkOTg1NzRiNjM0ZmNiOWNiN2I5MWZhYmE4ZGIzYSIsCiAgInByb2ZpbGVOYW1lIiA6ICJEdWNrVGVsbHoiLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYWM5MWY5YWZkODRmMjM2NWNlZThhNTNiNjFiOTQ0MmIyOGU0ZjBlMjViYzZiNmIxYmFkYmNkYWZjM2UzMGM0OSIKICAgIH0KICB9Cn0= ".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU4OTc5MzA2ODgzOSwKICAicHJvZmlsZUlkIiA6ICIyYzEwNjRmY2Q5MTc0MjgyODRlM2JmN2ZhYTdlM2UxYSIsCiAgInByb2ZpbGVOYW1lIiA6ICJOYWVtZSIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS83ZGU3YmJiZGYyMmJmZTE3OTgwZDRlMjA2ODdlMzg2ZjExZDU5ZWUxZGI2ZjhiNDc2MjM5MWI3OWE1YWM1MzJkIgogICAgfQogIH0KfQ==".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU4OTkyMzAyODAxNSwKICAicHJvZmlsZUlkIiA6ICJhMmY4MzQ1OTVjODk0YTI3YWRkMzA0OTcxNmNhOTEwYyIsCiAgInByb2ZpbGVOYW1lIiA6ICJiUHVuY2giLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvMzI2MDMyNTE3MWE3YmE4NDYwODMwYzBlZWE1MTVjNzU3YTY2NWU1YjE2YTE0MjA3YmExYTMxODI3NTJiZWU4NyIKICAgIH0KICB9Cn0=".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU4OTkyMzE2OTIxMSwKICAicHJvZmlsZUlkIiA6ICJhMmY4MzQ1OTVjODk0YTI3YWRkMzA0OTcxNmNhOTEwYyIsCiAgInByb2ZpbGVOYW1lIiA6ICJiUHVuY2giLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvODQyMWJhNWI4ZTM1NzNlZjk3YmViNWI0MGUxNWQxNWIyMGYzMDYzMWM0YzUzMzBjM2RlZGEzMDQ3ZGYwZTkyIgogICAgfQogIH0KfQ==".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU4OTkyMzExMjUwMCwKICAicHJvZmlsZUlkIiA6ICJhMmY4MzQ1OTVjODk0YTI3YWRkMzA0OTcxNmNhOTEwYyIsCiAgInByb2ZpbGVOYW1lIiA6ICJiUHVuY2giLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYWQyMjc3MmY3NjkwNDVmZGM1YmU4MTlhZDY4YjAxYTk3YWMwNGM2MDg4NmQyY2E3YWZlZTM5YjI4MmY3YTM4MyIKICAgIH0KICB9Cn0=".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU4OTkyMzIxNTkwNSwKICAicHJvZmlsZUlkIiA6ICJhMmY4MzQ1OTVjODk0YTI3YWRkMzA0OTcxNmNhOTEwYyIsCiAgInByb2ZpbGVOYW1lIiA6ICJiUHVuY2giLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZmIzOTczYTc1MmIyNGEyZjNhYmIwMDM0MjdmNmRiZTZjYTNhNjFkYjBhMWJjZjM1MWM2ZWFiMjdlYzI3ZTUwIgogICAgfQogIH0KfQ==".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU4OTkyMzM4Njc5NCwKICAicHJvZmlsZUlkIiA6ICJhMmY4MzQ1OTVjODk0YTI3YWRkMzA0OTcxNmNhOTEwYyIsCiAgInByb2ZpbGVOYW1lIiA6ICJiUHVuY2giLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYWQ2N2Y5N2Q3ZjgyMTcyOWJlYjM0YTgyYzNmMTM1OTJiNDA0MzlmZTUyNDhlNzI1NzZmZGU3YWExODBiZjc3IgogICAgfQogIH0KfQ==".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU5NTQyODIyMDAyMCwKICAicHJvZmlsZUlkIiA6ICJkYTQ5OGFjNGU5Mzc0ZTVjYjYxMjdiMzgwODU1Nzk4MyIsCiAgInByb2ZpbGVOYW1lIiA6ICJOaXRyb2hvbGljXzIiLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvNjJkOGZkM2FhNTYxN2IxZGFjMGFhZTljODFmNmRkNzBhZDkzYTU5OTQyZjQ2MGQyN2U0ZDU1YTVjYjg5MThlOCIKICAgIH0KICB9Cn0=".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTU5ODk3NzI1OTM1NywKICAicHJvZmlsZUlkIiA6ICJlNzkzYjJjYTdhMmY0MTI2YTA5ODA5MmQ3Yzk5NDE3YiIsCiAgInByb2ZpbGVOYW1lIiA6ICJUaGVfSG9zdGVyX01hbiIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS9jMTAwN2M1YjcxMTRhYmVjNzM0MjA2ZDRmYzYxM2RhNGYzYTBlOTlmNzFmZjk0OWNlZGFkYzk5MDc5MTM1YTBiIgogICAgfQogIH0KfQ==".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTYxODg5MzgxMDEwOCwKICAicHJvZmlsZUlkIiA6ICIxYWZhZjc2NWI1ZGY0NjA3YmY3ZjY1ZGYzYWIwODhhOCIsCiAgInByb2ZpbGVOYW1lIiA6ICJMb3lfQmxvb2RBbmdlbCIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS82OTE5OGY0MTBhMTBmOTkzMTRhYTBmYmU5YTNkYjEwNjk3YmJjMWMwMTFmMDE5NTA3ZDk2NjczYzY0MjE3ZjVhIgogICAgfQogIH0KfQ==".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTYyNjI2MTAwOTg1MSwKICAicHJvZmlsZUlkIiA6ICJiN2ZkYmU2N2NkMDA0NjgzYjlmYTllM2UxNzczODI1NCIsCiAgInByb2ZpbGVOYW1lIiA6ICJDVUNGTDE0IiwKICAic2lnbmF0dXJlUmVxdWlyZWQiIDogdHJ1ZSwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzQ5ZjdjZWMwMGFmZTlmN2M2MjRhZThkZjVjMDMzY2I0MTlmNmVhNDEwMTcwMjFiOWJlZmQ5MTk3MGI4MzNhNWMiCiAgICB9CiAgfQp9".to_string(), "ewogICJ0aW1lc3RhbXAiIDogMTYzNTQ5MTU0OTU4NCwKICAicHJvZmlsZUlkIiA6ICI2MTZiODhkNDMwNzM0ZTM3OWM3NDc1ODdlZTJkNzlmZCIsCiAgInByb2ZpbGVOYW1lIiA6ICJKZWxseUZpbiIsCiAgInNpZ25hdHVyZVJlcXVpcmVkIiA6IHRydWUsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS8zYjQ4ZWM5YzNlMjNhMDllOGFhMmUxZWZiZmY5YWZiMjVlNzMxNWY5MzkwOTg0ZDAxNjcxZGQwYWUzYzQ2OWFiIgogICAgfQogIH0KfQ==".to_string(), "eyJ0aW1lc3RhbXAiOjE1NzQ0MTkzMTAxNjQsInByb2ZpbGVJZCI6Ijc1MTQ0NDgxOTFlNjQ1NDY4Yzk3MzlhNmUzOTU3YmViIiwicHJvZmlsZU5hbWUiOiJUaGFua3NNb2phbmciLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzEyNzE2ZWNiZjViOGRhMDBiMDVmMzE2ZWM2YWY2MWU4YmQwMjgwNWIyMWViOGU0NDAxNTE0NjhkYzY1NjU0OWMifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA0OTUwMjgsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2ZmMTg0YzE5ZTcyNTYyM2QzMjgyOGEwYTRlNzQxZTg2ZjEzNWFjewogICJ0aW1lc3RhbXAiIDogMTU4OTkyMzAyODAxNSwKICAicHJvZmlsZUlkIiA6ICJhMmY4MzQ1OTVjODk0YTI3YWRkMzA0OTcxNmNhOTEwYyIsCiAgInByb2ZpbGVOYW1lIiA6ICJiUHVuY2giLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvMzI2MDMyNTE3MWE3YmE4NDYwODMwYzBlZWE1MTVjNzU3YTY2NWU1YjE2YTE0MjA3YmExYTMxODI3NTJiZWU4NyIKICAgIH0KICB9Cn0=".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA0OTUwMjgsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2ZmMTg0YzE5ZTcyNTYyM2QzMjgyOGEwYTRlNzQxZTg2ZjEzNWFjNjNkYmM4MjhmZjNjODQ2ODMzOGYzNjgzYiJ9fX0=".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA1MzgzODIsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2E4OWY2MzAzYWY4NTg3NzYxMDkxMmRjMDRiOGIxZTg5NzI0NzUyZjBhN2VlYTA1YWI2NTQ3ZTIyODE3OWMwNmYiLCJtZXRhZGF0YSI6eyJtb2RlbCI6InNsaW0ifX19fQ==".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA3Njk2MTQsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2FhMjNjOGNkZTI5NDNjODQyNDlkZTgzNTFiYzM1NDBiZTVmOGFmYWFiYThiMmNiMDMyZmM1YWNhZDc4YTI2OWIifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA4MTg4MDMsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzkxNzFmMzViOGY1MDgxNDJiZDhjNjU0MTdkMGYzMjQxNTNhYjkxNDc3MzllZTRkMTBkZWE3MzNjYzgwZWFhMjAifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA5MTc4NzYsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2I1YmE3NmUwMmNhYjcyZmE3ZDhhYzU0Y2VlYzg0OTk3NmFiMGIwMGEwMTA2OGQ2OGMyNjY3NjZiZjcwYzM5OTcifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA5NTY0MjIsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzdkMTJiMmFkZTQxM2E2Y2Q3Y2NhM2M5NWU5NjFiYTlmMGFlNzE2NWZhNDFmYzdiNWQ1ZjA5NGEwMTI0MGM2MDkifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDA5ODk1NTgsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzY3MjM3ZWRkYWViZGJiZGFhY2ZhOTEyODg1NTYwY2NkYzY1ZGE5M2I0YzNkNTEzNTMyODY4ZWMyM2JiNWI0NDgifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDAyMDM1NzMsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2Y0NjI0YTlhOGM2OWNhMjA0NTA0YWJiMDQzZDQ3NDU2Y2Q5YjA5NzQ5YTM2MzU3NDYyMzAzZjI3NmEyMjlkNCJ9fX0=".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDEwMzA3NjUsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzVjY2NkNTNmNTE5MWMyOWE5ZGM4ZjAxNzBmYmRjNGU1OWU2NjQ3NmFhZTMzZGUyN2I0NjhmMWRlMWI3Y2YzYjIifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDEwNjQwNTAsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzVhNzk4NjBhY2E3OTk0MDdjMGZhYTEwYjFiYmNmNDI5OThmYWQ0ZWJjZjMxZDdhMjE0MTgwODI2YjRhYzk0ZTEifX19".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDExNDUyMjIsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2M5MTllNWI4ZDU2ZjA2MmEyMWQyMjRkZTE0YWY3NzFlMmY1NWQwOWI1OWU3YjA5OWQwOWRhYTU3NTQwYjc5Y2YiLCJtZXRhZGF0YSI6eyJtb2RlbCI6InNsaW0ifX19fQ==".to_string(), "eyJ0aW1lc3RhbXAiOjE1ODYwNDExODY2MzYsInByb2ZpbGVJZCI6ImRhNDk4YWM0ZTkzNzRlNWNiNjEyN2IzODA4NTU3OTgzIiwicHJvZmlsZU5hbWUiOiJOaXRyb2hvbGljXzIiLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzQ3NzQ4NzExOTBjODc4YzlhMmM0NDk2YzFlMTAyNTdjNmM0ZWExMzgwN2Q3MmMxNWQ3YWM2YWIzYTdhOWE4ZGMifX19".to_string(), "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvNTZmYzg1NGJiODRjZjRiNzY5NzI5Nzk3M2UwMmI3OWJjMTA2OTg0NjBiNTFhNjM5YzYwZTVlNDE3NzM0ZTExIn19fQ==".to_string()]));
/// java: private static final float[] BOX_COLOR = {1f, 0f, 0f};
pub static BOX_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 0.0f32, 0.0f32]);
/// java: private static final float[] LINE_COLOR = {1f, 1f, 0f};
pub static LINE_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 1.0f32, 0.0f32]);
/// java: private static final Set<Zombie> WATCHERS = new HashSet<>();
/// Java holds the live `Zombie`; the port's entities live in the level, so the set holds each one's id
/// (identity in the level) and every read goes through `game::with_entity`.
pub static WATCHERS: compat::MainThread<std::collections::HashSet<i32>> = compat::MainThread::new(|| std::collections::HashSet::new());
/// java: private static final Map<ArmorStand, TrackedMob> MOBS = new HashMap<>();
/// Keyed by the armor stand's entity id (see `WATCHERS`).
pub static MOBS: compat::MainThread<std::collections::HashMap<i32, TrackedMob>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2IntMap<Zombie> PENDING_WATCHERS = new Object2IntOpenHashMap<>();
/// Keyed by the zombie's entity id (see `WATCHERS`).
pub static PENDING_WATCHERS: compat::MainThread<std::collections::HashMap<i32, i32>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static int mobsPredicted = 0;
pub static MOBS_PREDICTED: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: static final int DELTA_SAMPLES = 5;
pub const TRACKED_MOB_DELTA_SAMPLES: i32 = 5;

/// java: public class BloodCampHelper
#[derive(Clone)]
pub struct BloodCampHelper {}

impl BloodCampHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientEntityEvents.ENTITY_LOAD.register(BloodCampHelper::onEntityLoad);
        mcrust::mcrust::modding::events::client_entity_events::register_entity_load(|entity, world| BloodCampHelper::on_entity_load(entity, world));
        // java: ClientEntityEvents.ENTITY_UNLOAD.register(BloodCampHelper::onEntityUnload);
        mcrust::mcrust::modding::events::client_entity_events::register_entity_unload(|entity, world| BloodCampHelper::on_entity_unload(entity, world));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { BloodCampHelper::extract_rendering(&__a0); }));
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            BloodCampHelper::tick();
        }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            BloodCampHelper::reset();
        }));
        client_play_connection_events::DISCONNECT.register(Arc::new(|listener: &mut ClientPacketListener| {
            BloodCampHelper::reset();
        }));
    }

    /// java: private static void onEntityLoad(Entity entity, ClientLevel world)
    pub fn on_entity_load(entity: &dyn Entity, world: &ClientLevel) {
        if !(Utils::is_in_dungeons()) || !(SkyblockerConfigManager::get().dungeons.blood_camp_helper) {
            return;
        }
        if entity.as_any().is_some_and(|a| a.is::<Zombie>()) {
            PENDING_WATCHERS.get_mut().insert(entity.base().get_id(), 1);
        }
    }

    /// java: private static void onEntityUnload(Entity entity, ClientLevel world)
    pub fn on_entity_unload(entity: &dyn Entity, world: &ClientLevel) {
        if !(Utils::is_in_dungeons()) || !(SkyblockerConfigManager::get().dungeons.blood_camp_helper) {
            return;
        }
        let id = entity.base().get_id();
        if entity.as_any().is_some_and(|a| a.is::<Zombie>()) {
            WATCHERS.get_mut().remove(&id);
            PENDING_WATCHERS.get_mut().remove(&id);
        } else if entity.as_any().is_some_and(|a| a.is::<ArmorStand>()) {
            MOBS.get_mut().remove(&id);
        }
    }

    /// `entity.isAlive()` of an entity the mod holds by id: an entity no longer in the level was removed
    /// (`isRemoved()`), so it is not alive.
    fn is_alive(id: i32) -> bool {
        mcrust::mcrust::modding::game::with_entity(id, |e| mcrust::net::minecraft::world::entity::entity_selector::is_alive(&*e)).unwrap_or(false)
    }

    /// `ItemUtils.getHeadTexture(entity.getItemBySlot(EquipmentSlot.HEAD))` when `entity.hasItemInSlot(HEAD)`.
    fn head_texture(e: &mcrust::net::minecraft::client::multiplayer::client_level::StoredEntity) -> Option<String> {
        let living = e.as_living_entity()?;
        if !living.has_item_in_slot(EquipmentSlot::Head) {
            return None;
        }
        Some(ItemUtils::get_head_texture(living.get_item_by_slot(EquipmentSlot::Head)))
    }

    /// java: private static void tick()
    pub fn tick() {
        if !(Utils::is_in_dungeons()) || !(SkyblockerConfigManager::get().dungeons.blood_camp_helper) {
            return;
        }
        let mut now: i64 = compat::millis();
        PENDING_WATCHERS.get_mut().retain(|id, _| BloodCampHelper::is_alive(*id));
        // PENDING_WATCHERS.replaceAll((_, ticks) -> ticks - 1);
        for ticks in PENDING_WATCHERS.get_mut().values_mut() {
            *ticks -= 1;
        }
        let mut new_watchers: Vec<i32> = Vec::new();
        PENDING_WATCHERS.get_mut().retain(|id, ticks| {
            if *ticks <= 0 {
                let texture = mcrust::mcrust::modding::game::with_entity(*id, |zombie| BloodCampHelper::head_texture(zombie)).flatten();
                if let Some(texture) = texture {
                    if WATCHER_SKINS.contains(&texture) {
                        new_watchers.push(*id);
                    }
                }
                return false;
            }
            true
        });
        WATCHERS.get_mut().extend(new_watchers);
        WATCHERS.get_mut().retain(|watcher| BloodCampHelper::is_alive(*watcher));
        let watchers: Vec<i32> = WATCHERS.get().iter().copied().collect();
        for watcher in watchers {
            let Some(bounds) = mcrust::mcrust::modding::game::with_entity(watcher, |w| {
                mcrust::net::minecraft::world::entity::entity_selector::is_alive(&*w).then(|| w.base().get_bounding_box().inflate(2.0))
            })
            .flatten() else {
                continue;
            };
            let Some(level) = mcrust::mcrust::modding::game::level() else {
                continue;
            };
            use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;
            // getEntitiesOfClass(ArmorStand.class, box, stand -> stand.hasItemInSlot(HEAD) && !MOBS.containsKey(stand))
            let stands = level.get_entities(None, &bounds, &|stand| {
                stand.as_any().is_some_and(|a| a.is::<ArmorStand>())
                    && stand.as_living_entity().is_some_and(|l| l.has_item_in_slot(EquipmentSlot::Head))
                    && !MOBS.get().contains_key(&stand.base().get_id())
            });
            for stand in stands.iter() {
                let (id, texture, position) = {
                    let guard = mcrust::mcrust::modding::game::lock_entity(&**stand, "blood_camp_helper.rs:BloodCampHelper.tick");
                    (guard.base().get_id(), BloodCampHelper::head_texture(&*guard).unwrap_or_default(), guard.base().position())
                };
                if BLOOD_MOB_SKINS.contains(&texture) {
                    MOBS.get_mut().insert(id, TrackedMob::new(id, position));
                }
            }
        }
        MOBS.get_mut().retain(|id, _| BloodCampHelper::is_alive(*id));
        for (id, mob) in MOBS.get_mut().iter_mut() {
            if let Some(position) = mcrust::mcrust::modding::game::with_entity(*id, |e| e.base().position()) {
                mob.update(position, now);
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(Utils::is_in_dungeons()) || !(SkyblockerConfigManager::get().dungeons.blood_camp_helper) {
            return;
        }
        let mobs: Vec<(i32, Option<Vec3>)> = MOBS.get().iter().map(|(id, mob)| (*id, mob.predicted_pos.clone())).collect();
        for (id, predicted_pos) in mobs {
            if let Some(predicted_pos) = predicted_pos {
                let Some((bounding_box, position)) = mcrust::mcrust::modding::game::with_entity(id, |e| (e.base().get_bounding_box(), e.base().position())) else {
                    continue;
                };
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &bounding_box.move_by((0.0f32) as f64, (2.0f32) as f64, (0.0f32) as f64), &LINE_COLOR, ((2) as f32) as f32, true) });
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![position.add((0.0f32) as f64, (2.0f32) as f64, (0.0f32) as f64), predicted_pos.add((0.0f32) as f64, (2.0f32) as f64, (0.0f32) as f64)], &LINE_COLOR, 0.5f32, 2.0f32, true) });
                let mut r#box: AABB = AABB::new((predicted_pos.x - 0.5), (predicted_pos.y + (2 as f64)), (predicted_pos.z - 0.5), (predicted_pos.x + 0.5), (predicted_pos.y + (4 as f64)), (predicted_pos.z + 0.5));
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &r#box, &BOX_COLOR, ((5) as f32) as f32, true) });
            }
        }
    }

    /// java: private static void reset()
    pub fn reset() {
        WATCHERS.get_mut().clear();
        MOBS.get_mut().clear();
        PENDING_WATCHERS.get_mut().clear();
        compat::set_state(&MOBS_PREDICTED, 0);
    }

}

/// java: private static class TrackedMob
pub struct TrackedMob {
    /// java: final ArmorStand entity;
    /// The armor stand's entity id (Java's reference to the live object; read through `game::with_entity`).
    pub entity: i32,
    /// java: final Vec3 startPos;
    pub start_pos: Vec3,
    /// java: long startTime = -1;
    pub start_time: i64,
    /// java: Vec3 lastPos;
    pub last_pos: Option<Vec3>,
    /// java: long lastTime;
    pub last_time: i64,
    /// java: boolean firstWave;
    pub first_wave: bool,
    /// java: final Deque<Vec3> deltas = new ArrayDeque<>();
    pub deltas: std::collections::VecDeque<Vec3>,
    /// java: boolean inMotion = false;
    pub in_motion: bool,
    /// java: @Nullable Vec3 predictedPos;
    pub predicted_pos: Option<Vec3>,
}

impl TrackedMob {
    /// java: TrackedMob(ArmorStand entity)
    /// `entity` is the stand's id and `position` its `entity.position()`, read by the caller under the
    /// entity's lock.
    pub fn new(entity: i32, position: Vec3) -> Self {
        let __init_start_pos = position;
        let mut this = Self {
            entity,
            start_pos: __init_start_pos,
            start_time: ((-1) as i64) as i64,
            last_pos: None,
            last_time: 0,
            first_wave: false,
            deltas: std::collections::VecDeque::new(),
            in_motion: false,
            predicted_pos: None,
        };
        this.last_pos = Some(this.start_pos.clone());
        this.last_time = compat::millis();
        this
    }

    /// java: void update(Vec3 currentPos, long now)
    pub fn update(&mut self, mut current_pos: Vec3, mut now: i64) {
        let mut dt: i64 = now.wrapping_sub(self.last_time);
        self.last_time = now;
        let mut delta: Vec3 = current_pos.subtract_vec(self.last_pos.as_ref().unwrap());
        self.last_pos = Some(current_pos.clone());
        self.in_motion = (delta.length_sqr() > (0 as f64) && dt > (0 as i64));
        if self.in_motion {
            if self.start_time < (0 as i64) {
                self.start_time = now.wrapping_sub(dt);
            }
            if (self.deltas.len() as i32) == TRACKED_MOB_DELTA_SAMPLES {
                self.deltas.pop_front().expect("NoSuchElementException");
            }
            self.deltas.push_back(delta.clone());
        }
        if (self.deltas.len() as i32) == TRACKED_MOB_DELTA_SAMPLES && self.predicted_pos.is_none() {
            let mut total_delta: Vec3 = mcrust::net::minecraft::world::phys::vec3::ZERO;
            for d in self.deltas.iter() {
                total_delta = total_delta.add_vec(d);
            }
            let mut dir: Vec3 = total_delta.normalize();
            self.first_wave = (compat::get_state(&MOBS_PREDICTED) < 4);
            compat::set_state(&MOBS_PREDICTED, compat::get_state(&MOBS_PREDICTED) + 1);
            let mut distance: f64 = (if self.first_wave { (15.6 + 0.5) } else { (11.4 + 0.5) });
            self.predicted_pos = Some(self.start_pos.add_vec(&dir.scale(distance)));
        }
    }

}
