//! Ported from io.github.moulberry.repo.constants.Islands (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter @ToString public class Islands
#[derive(Clone, Debug, Default)]
pub struct Islands {
    /// java: @SerializedName("area_names") @Unmodifiable Map<@NotNull String, @NotNull String> areaNames = new HashMap<>();
    pub(crate) area_names: HashMap<String, String>,
    /// java: @Unmodifiable @NotNull List<@NotNull Teleporter> teleporters = new ArrayList<>();
    pub(crate) teleporters: Vec<Teleporter>,
    /// java: @SerializedName("island_warps") @Unmodifiable @NotNull List<@NotNull Warp> warps = new ArrayList<>();
    pub(crate) warps: Vec<Warp>,
}

/// java: @Getter @ToString @AllArgsConstructor @NoArgsConstructor public static class Warp
#[derive(Clone, Debug, Default)]
pub struct Warp {
    /// java: @NotNull String warp;
    pub(crate) warp: String,
    /// java: @NotNull @Unmodifiable List<@NotNull String> aliases = new ArrayList<>();
    pub(crate) aliases: Vec<String>,
    /// java: @NotNull String mode;
    pub(crate) mode: String,
    /// java: float x;
    pub(crate) x: f32,
    /// java: float y;
    pub(crate) y: f32,
    /// java: float z;
    pub(crate) z: f32,
}

impl Warp {
    /// java: public Warp(@NotNull final String warp, @NotNull final List<@NotNull String> aliases, @NotNull final String mode, final float x, final float y, final float z)
    pub fn new(warp: String, aliases: Vec<String>, mode: String, x: f32, y: f32, z: f32) -> Self {
        Warp { warp, aliases, mode, x, y, z }
    }

    /// Gson's reflective binding of the class (the no-args constructor's defaults for an absent member).
    pub fn from_json(json: &Value) -> Self {
        Warp {
            warp: gson::string(json, "warp").unwrap_or_default(),
            aliases: gson::string_list(json, "aliases").unwrap_or_default(),
            mode: gson::string(json, "mode").unwrap_or_default(),
            x: gson::member(json, "x").and_then(gson::as_f32).unwrap_or(0.0),
            y: gson::member(json, "y").and_then(gson::as_f32).unwrap_or(0.0),
            z: gson::member(json, "z").and_then(gson::as_f32).unwrap_or(0.0),
        }
    }

    /// java: public @NotNull String getWarp()
    pub fn get_warp(&self) -> &str {
        &self.warp
    }

    /// java: public @NotNull List<@NotNull String> getAliases()
    pub fn get_aliases(&self) -> &[String] {
        &self.aliases
    }

    /// java: public @NotNull String getMode()
    pub fn get_mode(&self) -> &str {
        &self.mode
    }

    /// java: public float getX()
    pub fn get_x(&self) -> f32 {
        self.x
    }

    /// java: public float getY()
    pub fn get_y(&self) -> f32 {
        self.y
    }

    /// java: public float getZ()
    pub fn get_z(&self) -> f32 {
        self.z
    }
}

/// java: @Getter @ToString @AllArgsConstructor @NoArgsConstructor public static class Teleporter
#[derive(Clone, Debug, Default)]
pub struct Teleporter {
    /// java: float x;
    pub(crate) x: f32,
    /// java: float y;
    pub(crate) y: f32,
    /// java: float z;
    pub(crate) z: f32,
    /// java: @NotNull String from;
    pub(crate) from: String,
    /// java: @NotNull String to;
    pub(crate) to: String,
}

impl Teleporter {
    /// java: public Teleporter(final float x, final float y, final float z, @NotNull final String from, @NotNull final String to)
    pub fn new(x: f32, y: f32, z: f32, from: String, to: String) -> Self {
        Teleporter { x, y, z, from, to }
    }

    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        Teleporter {
            x: gson::member(json, "x").and_then(gson::as_f32).unwrap_or(0.0),
            y: gson::member(json, "y").and_then(gson::as_f32).unwrap_or(0.0),
            z: gson::member(json, "z").and_then(gson::as_f32).unwrap_or(0.0),
            from: gson::string(json, "from").unwrap_or_default(),
            to: gson::string(json, "to").unwrap_or_default(),
        }
    }

    /// java: public float getX()
    pub fn get_x(&self) -> f32 {
        self.x
    }

    /// java: public float getY()
    pub fn get_y(&self) -> f32 {
        self.y
    }

    /// java: public float getZ()
    pub fn get_z(&self) -> f32 {
        self.z
    }

    /// java: public @NotNull String getFrom()
    pub fn get_from(&self) -> &str {
        &self.from
    }

    /// java: public @NotNull String getTo()
    pub fn get_to(&self) -> &str {
        &self.to
    }
}

impl Islands {
    /// java: new Islands() - the field initialisers (NEUConstants uses it when `constants/islands.json` is absent).
    pub fn new() -> Self {
        Islands::default()
    }

    /// Gson's reflective binding of the class (a member absent from the JSON keeps its initialiser).
    pub fn from_json(json: &Value) -> Self {
        Islands {
            area_names: gson::member(json, "area_names").and_then(|v| gson::map(v, gson::string_key, gson::as_string)).unwrap_or_default(),
            teleporters: gson::member(json, "teleporters").and_then(|v| gson::list(v, |t| Some(Teleporter::from_json(t)))).unwrap_or_default(),
            warps: gson::member(json, "island_warps").and_then(|v| gson::list(v, |w| Some(Warp::from_json(w)))).unwrap_or_default(),
        }
    }

    /// java: public Map<@NotNull String, @NotNull String> getAreaNames()
    pub fn get_area_names(&self) -> &HashMap<String, String> {
        &self.area_names
    }

    /// java: public @NotNull List<@NotNull Teleporter> getTeleporters()
    pub fn get_teleporters(&self) -> &[Teleporter] {
        &self.teleporters
    }

    /// java: public @NotNull List<@NotNull Warp> getWarps()
    pub fn get_warps(&self) -> &[Warp] {
        &self.warps
    }
}
